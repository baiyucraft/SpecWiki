import { existsSync, readFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { invokeTool, runToolCommand, type ToolCommandResult, type ToolCommandRunner } from "../tools/command.js";
import { CODEGRAPH_RELEASE } from "../tools/manifest.js";

export type CodeGraphCommandResult = ToolCommandResult;
export type CodeGraphCommandRunner = ToolCommandRunner;

export type CodeGraphWarning = {
  stage: "cli" | "cli-install" | "codex-mcp" | "project" | "status";
  command: string;
  message: string;
  recovery: string;
};

export type CodeGraphStatusJson = {
  initialized?: boolean;
  version?: string;
  pendingChanges?: { added?: number; modified?: number; removed?: number };
  worktreeMismatch?: unknown;
  index?: { state?: string; reindexRecommended?: boolean; pendingRefs?: number };
  [key: string]: unknown;
};

export type CodeGraphResult = {
  requested: boolean;
  deferred: boolean;
  cli: { available: boolean; expectedVersion: string; version?: string; installed: boolean; compatible: boolean };
  codexMcp: { configured: boolean };
  project: { initialized: boolean; path: ".codegraph"; healthy: boolean; stale: boolean };
  warnings: CodeGraphWarning[];
  nextActions: string[];
};

export type CodeGraphIntegrationOptions = {
  projectRoot: string;
  env: NodeJS.ProcessEnv;
  mode?: "required" | "skip";
  runner?: CodeGraphCommandRunner;
};

function commandText(command: string, args: string[]): string {
  return [command, ...args].join(" ");
}

function message(result: ToolCommandResult): string {
  return (result.stderr || result.stdout || `exit code ${result.code}`).trim().slice(0, 1000);
}

function warning(
  stage: CodeGraphWarning["stage"],
  command: string,
  result: ToolCommandResult,
  recovery: string,
): CodeGraphWarning {
  return { stage, command, message: message(result), recovery };
}

function detectCodexMcp(env: NodeJS.ProcessEnv): boolean {
  const codexHome = env.CODEX_HOME || path.join(os.homedir(), ".codex");
  const configPath = path.join(codexHome, "config.toml");
  if (!existsSync(configPath)) return false;
  try { return /codegraph/iu.test(readFileSync(configPath, "utf8")); } catch { return false; }
}

function executable(command: "codegraph" | "npm", args: string[]): [string, string[]] {
  if (process.platform !== "win32") return [command, args];
  if (command === "codegraph") {
    return [process.execPath, [path.join(path.dirname(process.execPath), "node_modules", "@colbymchenry", "codegraph", "npm-shim.js"), ...args]];
  }
  return [process.execPath, [path.join(path.dirname(process.execPath), "node_modules", "npm", "bin", "npm-cli.js"), ...args]];
}

const defaultRunner: CodeGraphCommandRunner = (command, args, options) => {
  if (command === "codegraph" && options.env.SPEC_WIKI_LITE_CODEGRAPH_NODE_SCRIPT) {
    return runToolCommand(process.execPath, [options.env.SPEC_WIKI_LITE_CODEGRAPH_NODE_SCRIPT, ...args], options);
  }
  if (command === "codegraph" && options.env.SPEC_WIKI_LITE_CODEGRAPH_BIN) {
    return runToolCommand(options.env.SPEC_WIKI_LITE_CODEGRAPH_BIN, args, options);
  }
  if (command === "npm" && options.env.SPEC_WIKI_LITE_NPM_BIN) {
    return runToolCommand(options.env.SPEC_WIKI_LITE_NPM_BIN, args, options);
  }
  const [resolved, resolvedArgs] = command === "codegraph" || command === "npm"
    ? executable(command, args)
    : [command, args];
  return runToolCommand(resolved, resolvedArgs, options);
};

function versionFrom(output: string): string | undefined {
  return output.match(/(\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?)/u)?.[1];
}

function empty(requested: boolean): CodeGraphResult {
  return {
    requested,
    deferred: !requested,
    cli: {
      available: false,
      expectedVersion: CODEGRAPH_RELEASE.version,
      installed: false,
      compatible: false,
    },
    codexMcp: { configured: false },
    project: { initialized: false, path: ".codegraph", healthy: false, stale: true },
    warnings: [],
    nextActions: [],
  };
}

function parseStatus(result: ToolCommandResult): CodeGraphStatusJson | undefined {
  if (result.code !== 0) return undefined;
  try {
    const parsed = JSON.parse(result.stdout) as unknown;
    return parsed && typeof parsed === "object" ? parsed as CodeGraphStatusJson : undefined;
  } catch {
    return undefined;
  }
}

function applyStatus(target: CodeGraphResult, status: CodeGraphStatusJson): void {
  const pending = status.pendingChanges;
  const pendingCount = (pending?.added ?? 0) + (pending?.modified ?? 0) + (pending?.removed ?? 0);
  const initialized = status.initialized === true;
  const stale = !initialized
    || pendingCount > 0
    || status.worktreeMismatch != null
    || status.index?.state !== "complete"
    || status.index?.reindexRecommended === true
    || (status.index?.pendingRefs ?? 0) > 0;
  target.project = { initialized, path: ".codegraph", healthy: initialized && !stale, stale };
}

export async function inspectCodeGraph(options: Omit<CodeGraphIntegrationOptions, "mode">): Promise<CodeGraphResult> {
  const state = empty(true);
  const runner = options.runner ?? defaultRunner;
  state.codexMcp.configured = detectCodexMcp(options.env);
  const versionArgs = ["--version"];
  const versionResult = await invokeTool(runner, "codegraph", versionArgs, {
    cwd: options.projectRoot,
    env: options.env,
    timeoutMs: 15_000,
  });
  const version = versionFrom(versionResult.stdout);
  state.cli = {
    available: versionResult.code === 0,
    expectedVersion: CODEGRAPH_RELEASE.version,
    version,
    installed: false,
    compatible: version === CODEGRAPH_RELEASE.version,
  };
  if (!state.cli.compatible) {
    state.warnings.push(warning("cli", commandText("codegraph", versionArgs), versionResult, "run spec-wiki-lite update --tools"));
    state.nextActions.push("run spec-wiki-lite update --tools");
    return state;
  }
  const statusArgs = ["status", "--json"];
  const statusResult = await invokeTool(runner, "codegraph", statusArgs, {
    cwd: options.projectRoot,
    env: options.env,
    timeoutMs: 30_000,
  });
  const status = parseStatus(statusResult);
  if (!status) {
    state.warnings.push(warning("status", commandText("codegraph", statusArgs), statusResult, "run codegraph status --json and repair the project index"));
    state.nextActions.push("run codegraph sync from the project root");
    return state;
  }
  applyStatus(state, status);
  if (!state.codexMcp.configured) state.nextActions.push("run spec-wiki-lite update --tools to configure the Codex CodeGraph MCP");
  if (!state.project.healthy) state.nextActions.push("run codegraph sync from the project root");
  return state;
}

export async function runCodeGraphIntegration(options: CodeGraphIntegrationOptions): Promise<CodeGraphResult> {
  if (options.mode === "skip") {
    const skipped = empty(false);
    skipped.nextActions.push("rerun spec-wiki-lite init without --no-codegraph");
    return skipped;
  }
  const runner = options.runner ?? defaultRunner;
  let state = await inspectCodeGraph({ projectRoot: options.projectRoot, env: options.env, runner });
  if (!state.cli.compatible) {
    const installArgs = ["install", "-g", `${CODEGRAPH_RELEASE.package}@${CODEGRAPH_RELEASE.version}`];
    const install = await invokeTool(runner, "npm", installArgs, {
      cwd: options.projectRoot,
      env: options.env,
      timeoutMs: 120_000,
    });
    if (install.code !== 0) {
      state.warnings.push(warning("cli-install", commandText("npm", installArgs), install, "install the pinned CodeGraph package and rerun init"));
      return state;
    }
    state = await inspectCodeGraph({ projectRoot: options.projectRoot, env: options.env, runner });
    state.cli.installed = true;
    if (!state.cli.compatible) return state;
  }

  if (!state.codexMcp.configured) {
    const mcpArgs = ["install", "--target=codex", "--location=global", "--yes", "--no-permissions"];
    const mcp = await invokeTool(runner, "codegraph", mcpArgs, {
      cwd: options.projectRoot,
      env: options.env,
      timeoutMs: 60_000,
    });
    if (mcp.code === 0) state.codexMcp.configured = true;
    else state.warnings.push(warning("codex-mcp", commandText("codegraph", mcpArgs), mcp, "configure the CodeGraph Codex MCP manually"));
  }

  const indexArgs = state.project.initialized ? ["sync"] : ["init", options.projectRoot];
  if (!state.project.healthy) {
    const index = await invokeTool(runner, "codegraph", indexArgs, {
      cwd: options.projectRoot,
      env: options.env,
      timeoutMs: 120_000,
    });
    if (index.code !== 0) {
      state.warnings.push(warning("project", commandText("codegraph", indexArgs), index, "repair CodeGraph and rerun codegraph sync"));
    }
  }
  const final = await inspectCodeGraph({ projectRoot: options.projectRoot, env: options.env, runner });
  final.cli.installed = state.cli.installed;
  final.codexMcp.configured ||= state.codexMcp.configured;
  final.warnings = [...state.warnings, ...final.warnings];
  final.nextActions = [...new Set(final.nextActions.filter(action => {
    if (action.includes("CodeGraph MCP")) return !final.codexMcp.configured;
    if (action.includes("codegraph sync")) return !final.project.healthy;
    if (action.includes("update --tools")) return !final.cli.compatible;
    return true;
  }))];
  return final;
}
