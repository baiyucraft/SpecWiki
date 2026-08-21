import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { readFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

export type CodeGraphCommandResult = {
  code: number;
  stdout: string;
  stderr: string;
};

export type CodeGraphCommandRunner = (
  command: string,
  args: string[],
  options: { cwd: string; env: NodeJS.ProcessEnv },
) => Promise<CodeGraphCommandResult>;

export type CodeGraphWarning = {
  stage: "cli" | "cli-install" | "codex-mcp" | "project";
  command: string;
  message: string;
  recovery: string;
};

export type CodeGraphResult = {
  requested: boolean;
  cli: { available: boolean; version?: string; installed: boolean };
  codexMcp: { configured: boolean };
  project: { initialized: boolean; path: ".codegraph" };
  warnings: CodeGraphWarning[];
};

export type CodeGraphIntegrationOptions = {
  projectRoot: string;
  env: NodeJS.ProcessEnv;
  mode?: "auto" | "force" | "skip";
  enabled?: boolean;
  runner?: CodeGraphCommandRunner;
};

const missingCliRecovery = "install CodeGraph manually with npm install -g @colbymchenry/codegraph@latest and rerun init";
const mcpRecovery = "run codegraph install --target=codex --location=global --yes --no-permissions manually";
const projectRecovery = "run codegraph init from the project root after fixing the reported error";

function defaultRunner(command: string, args: string[], options: { cwd: string; env: NodeJS.ProcessEnv }): Promise<CodeGraphCommandResult> {
  return new Promise((resolve) => {
    let executable = command;
    let commandArgs = args;
    if (process.platform === "win32" && command === "codegraph") {
      executable = process.execPath;
      commandArgs = [path.join(path.dirname(process.execPath), "node_modules", "@colbymchenry", "codegraph", "npm-shim.js"), ...args];
    } else if (process.platform === "win32" && command === "npm") {
      executable = process.execPath;
      commandArgs = [path.join(path.dirname(process.execPath), "node_modules", "npm", "bin", "npm-cli.js"), ...args];
    }
    const child = spawn(executable, commandArgs, { cwd: options.cwd, env: options.env, shell: false, windowsHide: true });
    let stdout = "";
    let stderr = "";
    child.stdout?.on("data", chunk => {
      stdout += String(chunk);
    });
    child.stderr?.on("data", chunk => {
      stderr += String(chunk);
    });
    child.on("error", error => resolve({ code: 1, stdout, stderr: `${stderr}${error.message}` }));
    child.on("close", code => resolve({ code: code ?? 1, stdout, stderr }));
  });
}

function commandText(command: string, args: string[]): string {
  return [command, ...args].join(" ");
}

function warning(
  stage: CodeGraphWarning["stage"],
  command: string,
  result: CodeGraphCommandResult,
  recovery: string,
): CodeGraphWarning {
  const detail = result.stderr.trim() || result.stdout.trim() || `exit code ${result.code}`;
  return { stage, command, message: detail, recovery };
}

function detectCodexMcp(env: NodeJS.ProcessEnv): boolean {
  const codexHome = env.CODEX_HOME || path.join(os.homedir(), ".codex");
  const configPath = path.join(codexHome, "config.toml");
  if (!existsSync(configPath)) {
    return false;
  }
  try {
    return /codegraph/iu.test(readFileSync(configPath, "utf8"));
  } catch {
    return false;
  }
}

async function invoke(
  runner: CodeGraphCommandRunner,
  command: string,
  args: string[],
  options: { cwd: string; env: NodeJS.ProcessEnv },
): Promise<CodeGraphCommandResult> {
  try {
    return await runner(command, args, options);
  } catch (error) {
    return {
      code: 1,
      stdout: "",
      stderr: error instanceof Error ? error.message : String(error),
    };
  }
}

export async function runCodeGraphIntegration(options: CodeGraphIntegrationOptions): Promise<CodeGraphResult> {
  const mode = options.mode ?? (options.enabled === false ? "skip" : options.enabled === true ? "force" : "auto");
  const requested = mode !== "skip";
  const empty: CodeGraphResult = {
    requested,
    cli: { available: false, installed: false },
    codexMcp: { configured: false },
    project: { initialized: false, path: ".codegraph" },
    warnings: [],
  };
  if (mode === "skip") {
    return empty;
  }

  const runner = options.runner ?? defaultRunner;
  const warnings: CodeGraphWarning[] = [];
  const existingIndex = existsSync(path.join(options.projectRoot, ".codegraph"));
  empty.codexMcp.configured = detectCodexMcp(options.env);
  let cli: CodeGraphResult["cli"] = { available: false, installed: false };
  const versionCommand = ["codegraph", ["--version"]] as const;
  let versionResult = await invoke(runner, versionCommand[0], [...versionCommand[1]], { cwd: options.projectRoot, env: options.env });
  let version: string | undefined;
  if (versionResult.code === 0) {
    version = versionResult.stdout.trim().split(/\s+/u)[0]?.replace(/^v/u, "") || undefined;
    cli = { available: true, installed: false };
  } else {
    if (mode === "auto") {
      warnings.push(warning("cli", commandText(versionCommand[0], [...versionCommand[1]]), versionResult, missingCliRecovery));
      empty.warnings = warnings;
      empty.project.initialized = existsSync(path.join(options.projectRoot, ".codegraph"));
      return empty;
    }
    if (existingIndex) {
      warnings.push(warning("cli", commandText(versionCommand[0], [...versionCommand[1]]), versionResult, missingCliRecovery));
      empty.cli = cli;
      empty.project.initialized = true;
      empty.warnings = warnings;
      return empty;
    }
    const installArgs = ["install", "-g", "@colbymchenry/codegraph@latest"];
    const installResult = await invoke(runner, "npm", installArgs, { cwd: options.projectRoot, env: options.env });
    if (installResult.code === 0) {
      cli = { available: true, installed: true };
      versionResult = await invoke(runner, "codegraph", ["--version"], { cwd: options.projectRoot, env: options.env });
      if (versionResult.code === 0) {
        version = versionResult.stdout.trim().split(/\s+/u)[0]?.replace(/^v/u, "") || undefined;
      }
    } else {
      warnings.push(warning("cli-install", commandText("npm", installArgs), installResult, missingCliRecovery));
    }
  }
  if (version) {
    cli = { ...cli, available: true, version };
  }
  empty.cli = cli;

  if (mode === "auto") {
    empty.project.initialized = existingIndex;
    if (!existingIndex) {
      warnings.push({
        stage: "project",
        command: commandText("codegraph", ["init", options.projectRoot]),
        message: "project CodeGraph index is not initialized",
        recovery: "rerun spec-wiki-lite init --codegraph to initialize the project index",
      });
    }
    empty.warnings = warnings;
    return empty;
  }
  if (existingIndex) {
    empty.project.initialized = true;
    empty.warnings = warnings;
    return empty;
  }

  const mcpArgs = ["install", "--target=codex", "--location=global", "--yes", "--no-permissions"];
  const mcpResult = await invoke(runner, "codegraph", mcpArgs, { cwd: options.projectRoot, env: options.env });
  if (mcpResult.code === 0) {
    empty.codexMcp.configured = true;
  } else {
    warnings.push(warning("codex-mcp", commandText("codegraph", mcpArgs), mcpResult, mcpRecovery));
  }

  const projectArgs = ["init", options.projectRoot];
  const projectResult = await invoke(runner, "codegraph", projectArgs, { cwd: options.projectRoot, env: options.env });
  if (projectResult.code === 0) {
    empty.project.initialized = true;
  } else {
    warnings.push(warning("project", commandText("codegraph", projectArgs), projectResult, projectRecovery));
  }
  empty.warnings = warnings;
  return empty;
}
