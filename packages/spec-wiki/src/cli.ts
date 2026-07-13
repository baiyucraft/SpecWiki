import { ADVANCED_COMMANDS, STREAMING_COMMANDS, isCommand, type CommandName } from "./cli/commandSpec.js";
import { runBootstrapInit } from "./orchestration/init/runInit.js";
import { selectHostsForInit } from "./orchestration/init/selectHosts.js";
import { EXIT_CODES } from "./runtime/exitPolicy.js";
import { forwardCoreCommand } from "./runtime/forwardCore.js";

export type CliIo = {
  cwd: string;
  env: NodeJS.ProcessEnv;
  stdin?: NodeJS.ReadStream;
  interactive?: boolean;
  stdout: (text: string) => void;
  stderr: (text: string) => void;
};

type Parsed = {
  command: CommandName;
  repoRoot: string;
  term?: string;
  changeId?: string;
  archiveMode?: "dry_run" | "apply" | "resume";
  archiveOperationId?: string;
  hosts?: string;
  noInteractive: boolean;
  bridgeStdio: boolean;
  json: boolean;
  developmentMode: boolean;
  help?: "default" | "all" | "command";
};

class CliUsageError extends Error {}

function defaultHelp(): string {
  return [
    "Usage:",
    "  spec-wiki init [options]",
    "  spec-wiki status [options]",
    "  spec-wiki query <term...> [options]",
    "  spec-wiki update [options]",
    "",
    "Run `spec-wiki --help-all` for advanced commands.",
    "Options: --repo-root <path> --json --development-mode",
    "Init options: --host <id> (repeatable) | --hosts <id,id,...> --no-interactive",
    "",
  ].join("\n");
}

function allHelp(): string {
  const commands = ADVANCED_COMMANDS.map((command) => {
    const operand = command === "change" || command === "validate" || command === "archive" ? " <change-id>" : "";
    return `  spec-wiki ${command}${operand} [options]`;
  });
  return `${defaultHelp()}Advanced:\n${commands.join("\n")}\n`;
}

function commandHelp(command: CommandName): string {
  const operand = command === "query"
    ? " <term...>"
    : command === "change" || command === "validate" || command === "archive"
      ? " <change-id>"
      : "";
  const bridgeOption = STREAMING_COMMANDS.has(command) ? " --bridge-stdio" : "";
  const archiveOptions = command === "archive"
    ? "\nArchive mode: --dry-run | --apply | --resume <operation-id>"
    : "";
  return `Usage:\n  spec-wiki ${command}${operand} [options]\n\nOptions: --repo-root <path> --json${bridgeOption}${archiveOptions}\n`;
}

function optionValue(args: string[], index: number, name: string): string {
  const value = args[index + 1];
  if (!value || value.startsWith("--")) {
    throw new CliUsageError(`missing value for ${name}`);
  }
  return value;
}

function emptyParsed(command: CommandName, cwd: string): Parsed {
  return {
    command,
    repoRoot: cwd,
    noInteractive: false,
    bridgeStdio: false,
    json: false,
    developmentMode: false,
  };
}

function parseHelp(args: string[], cwd: string): Parsed | undefined {
  if (args.length === 0 || args[0] === "--help" || args[0] === "-h") {
    if (args.length > 1) {
      throw new CliUsageError("help cannot be combined with other arguments");
    }
    return { ...emptyParsed("status", cwd), help: "default" };
  }
  if (args[0] === "--help-all") {
    if (args.length > 1) {
      throw new CliUsageError("help cannot be combined with other arguments");
    }
    return { ...emptyParsed("status", cwd), help: "all" };
  }
  return undefined;
}

function parseArgs(args: string[], cwd: string): Parsed {
  const help = parseHelp(args, cwd);
  if (help) {
    return help;
  }
  if (!isCommand(args[0])) {
    throw new CliUsageError(`unknown command: ${args[0]}`);
  }

  const parsed = emptyParsed(args[0], cwd);
  const terms: string[] = [];
  const hosts: string[] = [];
  let usedHostsOption = false;
  let archiveModeExplicit = false;

  for (let index = 1; index < args.length; index += 1) {
    const arg = args[index];
    if (arg === "--help" || arg === "-h") {
      parsed.help = "command";
      continue;
    }
    if (arg === "--repo-root") {
      parsed.repoRoot = optionValue(args, index, arg);
      index += 1;
      continue;
    }
    if (arg === "--json") {
      parsed.json = true;
      continue;
    }
    if (arg === "--development-mode") {
      parsed.developmentMode = true;
      continue;
    }
    if (arg === "--dry-run" || arg === "--apply" || arg === "--resume") {
      if (parsed.command !== "archive") {
        throw new CliUsageError(`${arg} is only supported for archive`);
      }
      if (archiveModeExplicit) {
        throw new CliUsageError("--dry-run, --apply, and --resume are mutually exclusive");
      }
      archiveModeExplicit = true;
      if (arg === "--resume") {
        parsed.archiveMode = "resume";
        parsed.archiveOperationId = optionValue(args, index, arg);
        index += 1;
      } else {
        parsed.archiveMode = arg === "--apply" ? "apply" : "dry_run";
      }
      continue;
    }
    if (arg === "--bridge-stdio") {
      parsed.bridgeStdio = true;
      continue;
    }
    if (arg === "--no-interactive") {
      if (parsed.command !== "init") {
        throw new CliUsageError("--no-interactive is only supported for init");
      }
      parsed.noInteractive = true;
      continue;
    }
    if (arg === "--host") {
      if (parsed.command !== "init" || usedHostsOption) {
        throw new CliUsageError("--host and --hosts are mutually exclusive");
      }
      hosts.push(optionValue(args, index, arg));
      index += 1;
      continue;
    }
    if (arg === "--hosts") {
      if (parsed.command !== "init" || hosts.length > 0 || usedHostsOption) {
        throw new CliUsageError("--host and --hosts are mutually exclusive");
      }
      usedHostsOption = true;
      parsed.hosts = optionValue(args, index, arg);
      index += 1;
      continue;
    }
    if (arg.startsWith("--")) {
      throw new CliUsageError(`unsupported option: ${arg}`);
    }
    if (parsed.command === "query") {
      terms.push(arg);
      continue;
    }
    if (parsed.command === "change" || parsed.command === "validate" || parsed.command === "archive") {
      if (parsed.changeId) {
        throw new CliUsageError(`unexpected positional argument: ${arg}`);
      }
      parsed.changeId = arg;
      continue;
    }
    throw new CliUsageError(`unexpected positional argument: ${arg}`);
  }

  if (parsed.command === "query") {
    if (terms.length === 0) {
      throw new CliUsageError("query requires at least one term");
    }
    parsed.term = terms.join(" ");
  }
  if (!parsed.help && (parsed.command === "change" || parsed.command === "validate" || parsed.command === "archive") && !parsed.changeId) {
    throw new CliUsageError(`${parsed.command} requires <change-id>`);
  }
  if (parsed.command === "archive" && !parsed.archiveMode) {
    parsed.archiveMode = "dry_run";
  }
  if (parsed.bridgeStdio && !STREAMING_COMMANDS.has(parsed.command)) {
    throw new CliUsageError("--bridge-stdio is only supported for init, update, and rebuild");
  }
  if (parsed.help && (parsed.json || parsed.bridgeStdio)) {
    throw new CliUsageError("help cannot be combined with --json or --bridge-stdio");
  }
  if (hosts.length > 0) {
    parsed.hosts = [...new Set(hosts)].join(",");
  }
  return parsed;
}

function renderHelp(parsed: Parsed): string {
  if (parsed.help === "all") {
    return allHelp();
  }
  if (parsed.help === "command") {
    return commandHelp(parsed.command);
  }
  return defaultHelp();
}

async function runInit(parsed: Parsed, io: CliIo, machine: boolean): Promise<number> {
  const selectedHosts = await selectHostsForInit({
    repoRoot: parsed.repoRoot,
    rawHosts: parsed.hosts,
    interactive: machine ? false : parsed.noInteractive ? false : io.interactive,
    stdin: io.stdin,
  });
  const bootstrap = await runBootstrapInit({
    repoRoot: parsed.repoRoot,
    hosts: selectedHosts.join(","),
    env: io.env,
  });
  if (bootstrap.outcome === "failed") {
    const message = "host bootstrap failed";
    if (machine) {
      io.stdout(`${JSON.stringify({ ok: false, error: message, errorKind: "workflow_failed", data: bootstrap })}\n`);
    } else {
      io.stderr(`${message}${bootstrap.recoveryHint ? `: ${bootstrap.recoveryHint}` : ""}\n`);
    }
    return EXIT_CODES.failure;
  }

  const exitCode = await forwardCoreCommand(
    {
      action: "cli_init",
      repoRoot: parsed.repoRoot,
      developmentMode: parsed.developmentMode,
      bootstrap,
    },
    {
      outputMode: machine ? "machine" : "human",
      bridgeStdio: parsed.bridgeStdio,
      action: "init",
      cwd: io.cwd,
      env: io.env,
      stdin: io.stdin,
      stdout: io.stdout,
      stderr: io.stderr,
    },
  );
  return typeof exitCode === "number" ? exitCode : EXIT_CODES.success;
}

export async function runCli(args: string[], io: CliIo): Promise<number> {
  try {
    const parsed = parseArgs(args, io.cwd);
    if (parsed.help) {
      io.stdout(renderHelp(parsed));
      return EXIT_CODES.success;
    }

    const machine = parsed.json || parsed.bridgeStdio;
    if (parsed.command === "init") {
      return runInit(parsed, io, machine);
    }
    return forwardCoreCommand(
      {
        action: parsed.command,
        repoRoot: parsed.repoRoot,
        ...(parsed.term ? { term: parsed.term } : {}),
        ...(parsed.changeId ? { changeId: parsed.changeId } : {}),
        ...(parsed.archiveMode ? { archiveMode: parsed.archiveMode } : {}),
        ...(parsed.archiveOperationId ? { archiveOperationId: parsed.archiveOperationId } : {}),
        ...(parsed.developmentMode ? { developmentMode: true } : {}),
      },
      {
        outputMode: machine ? "machine" : "human",
        bridgeStdio: parsed.bridgeStdio,
        action: parsed.command,
        cwd: io.cwd,
        env: io.env,
        stdin: io.stdin,
        stdout: io.stdout,
        stderr: io.stderr,
      },
    );
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    if (args.includes("--json") || args.includes("--bridge-stdio")) {
      io.stdout(`${JSON.stringify({ ok: false, error: message, errorKind: "invalid_argument" })}\n`);
    } else {
      io.stderr(`${message}\n`);
    }
    return error instanceof CliUsageError ? EXIT_CODES.usage : EXIT_CODES.failure;
  }
}
