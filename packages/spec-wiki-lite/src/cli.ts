import { mkdirSync } from "node:fs";

import { syncProjectAssets } from "./core/assets/sync.js";
import { archiveChange, ChangeNotReadyError } from "./core/change/archive.js";
import { isArtifactId } from "./core/change/artifacts.js";
import { showChange } from "./core/change/show.js";
import { validateChange } from "./core/change/validate.js";
import { isWikiLanguage, type WikiLanguage, WIKI_LANGUAGES } from "./core/config.js";
import { resolveSafePath } from "./core/path.js";
import { getProjectStatus } from "./core/status.js";
import { runBootstrapInit } from "./orchestration/init/runInit.js";

export const EXIT_CODES = {
  success: 0,
  failure: 1,
  notReady: 2,
  usage: 64,
} as const;

const COMMANDS = ["init", "status", "show", "validate", "update", "archive"] as const;
type CommandName = typeof COMMANDS[number];

export type CliIo = {
  cwd: string;
  env: NodeJS.ProcessEnv;
  stdin?: NodeJS.ReadStream;
  interactive?: boolean;
  stdout: (text: string) => void;
  stderr: (text: string) => void;
};

type ParsedCommand = {
  command: CommandName;
  json: boolean;
  force: boolean;
  strict: boolean;
  host: "codex";
  language?: WikiLanguage;
  path?: string;
  changeId?: string;
  artifact?: string;
  help: boolean;
};

class CliUsageError extends Error {}

function defaultHelp(): string {
  return [
    "Usage:",
    "  spec-wiki-lite init [path] [--host codex] [--language zh|en] [--force]",
    "  spec-wiki-lite status [--json]",
    "  spec-wiki-lite show <change-id> [--artifact <artifact>] [--json]",
    "  spec-wiki-lite validate <change-id> [--strict] [--json]",
    "  spec-wiki-lite update [--force] [--json]",
    "  spec-wiki-lite archive <change-id>",
    "",
  ].join("\n");
}

function commandHelp(command: CommandName): string {
  return `${defaultHelp().split("\n").filter(line => line === "Usage:" || line.includes(`spec-wiki-lite ${command}`)).join("\n")}\n`;
}

function optionValue(args: string[], index: number, option: string): string {
  const value = args[index + 1];
  if (!value || value.startsWith("--")) {
    throw new CliUsageError(`missing value for ${option}`);
  }
  return value;
}

function parseArgs(args: string[]): ParsedCommand | undefined {
  if (args.length === 0 || args[0] === "--help" || args[0] === "-h") {
    if (args.length > 1) {
      throw new CliUsageError("help cannot be combined with other arguments");
    }
    return undefined;
  }
  if (!COMMANDS.includes(args[0] as CommandName)) {
    throw new CliUsageError(`unknown command: ${args[0]}`);
  }
  const parsed: ParsedCommand = {
    command: args[0] as CommandName,
    json: false,
    force: false,
    strict: false,
    host: "codex",
    help: false,
  };
  const positionals: string[] = [];

  for (let index = 1; index < args.length; index += 1) {
    const argument = args[index];
    if (argument === "--help" || argument === "-h") {
      parsed.help = true;
      continue;
    }
    if (argument === "--json") {
      if (!(["status", "show", "validate", "update"] as CommandName[]).includes(parsed.command)) {
        throw new CliUsageError(`--json is not supported for ${parsed.command}`);
      }
      parsed.json = true;
      continue;
    }
    if (argument === "--force") {
      if (parsed.command !== "init" && parsed.command !== "update") {
        throw new CliUsageError(`--force is not supported for ${parsed.command}`);
      }
      parsed.force = true;
      continue;
    }
    if (argument === "--strict") {
      if (parsed.command !== "validate") {
        throw new CliUsageError(`--strict is not supported for ${parsed.command}`);
      }
      parsed.strict = true;
      continue;
    }
    if (argument === "--host") {
      if (parsed.command !== "init") {
        throw new CliUsageError("--host is only supported for init");
      }
      const host = optionValue(args, index, argument).toLowerCase();
      if (host !== "codex") {
        throw new CliUsageError(`unsupported host "${host}". Supported hosts: codex`);
      }
      index += 1;
      continue;
    }
    if (argument === "--language") {
      if (parsed.command !== "init") {
        throw new CliUsageError("--language is only supported for init");
      }
      const language = optionValue(args, index, argument).toLowerCase();
      if (!isWikiLanguage(language)) {
        throw new CliUsageError(`unsupported language "${language}". Supported languages: ${WIKI_LANGUAGES.join(", ")}`);
      }
      parsed.language = language;
      index += 1;
      continue;
    }
    if (argument === "--artifact") {
      if (parsed.command !== "show") {
        throw new CliUsageError("--artifact is only supported for show");
      }
      parsed.artifact = optionValue(args, index, argument);
      if (!isArtifactId(parsed.artifact)) {
        throw new CliUsageError(`unknown artifact: ${parsed.artifact}`);
      }
      index += 1;
      continue;
    }
    if (argument.startsWith("--")) {
      throw new CliUsageError(`unsupported option: ${argument}`);
    }
    positionals.push(argument);
  }

  if (parsed.help && (positionals.length > 0 || parsed.json || parsed.force || parsed.strict || parsed.artifact || parsed.language)) {
    throw new CliUsageError("help cannot be combined with other arguments");
  }
  if (parsed.command === "init") {
    if (positionals.length > 1) {
      throw new CliUsageError("init accepts at most one path");
    }
    parsed.path = positionals[0];
  } else if (parsed.command === "show" || parsed.command === "validate" || parsed.command === "archive") {
    if (positionals.length !== 1) {
      throw new CliUsageError(`${parsed.command} requires <change-id>`);
    }
    parsed.changeId = positionals[0];
  } else if (positionals.length > 0) {
    throw new CliUsageError(`${parsed.command} does not accept positional arguments`);
  }
  return parsed;
}

function writeJson(io: CliIo, ok: boolean, data?: unknown, error?: string): void {
  io.stdout(`${JSON.stringify({ ok, ...(data === undefined ? {} : { data }), ...(error ? { error } : {}) })}\n`);
}

function writeHuman(io: CliIo, heading: string, data: unknown): void {
  io.stdout(`${heading}\n${JSON.stringify(data, null, 2)}\n`);
}

async function execute(parsed: ParsedCommand, io: CliIo): Promise<number> {
  if (parsed.help) {
    io.stdout(commandHelp(parsed.command));
    return EXIT_CODES.success;
  }
  if (parsed.command === "init") {
    const projectRoot = resolveSafePath(io.cwd, parsed.path ?? ".");
    mkdirSync(projectRoot, { recursive: true });
    const result = await runBootstrapInit({
      repoRoot: projectRoot,
      hosts: parsed.host,
      env: io.env,
      force: parsed.force,
      language: parsed.language,
    });
    if (result.outcome === "failed") {
      io.stderr(`${result.error ?? "project initialization failed"}\n`);
      return EXIT_CODES.failure;
    }
    writeHuman(io, "SpecWiki Lite initialized", {
      assets: result.assets,
      status: await getProjectStatus(projectRoot),
    });
    return EXIT_CODES.success;
  }
  if (parsed.command === "update") {
    const result = await syncProjectAssets(io.cwd, { force: parsed.force });
    parsed.json ? writeJson(io, true, result) : writeHuman(io, "SpecWiki Lite assets updated", result);
    return EXIT_CODES.success;
  }
  if (parsed.command === "status") {
    const result = await getProjectStatus(io.cwd);
    parsed.json ? writeJson(io, true, result) : writeHuman(io, "SpecWiki Lite status", result);
    return EXIT_CODES.success;
  }
  if (parsed.command === "show") {
    const result = await showChange(io.cwd, parsed.changeId!, parsed.artifact);
    parsed.json
      ? writeJson(io, result.change.valid, result)
      : writeHuman(io, `Change ${parsed.changeId} is ${result.change.valid ? "valid" : "not ready"}`, result);
    return result.change.valid ? EXIT_CODES.success : EXIT_CODES.notReady;
  }
  if (parsed.command === "validate") {
    const result = await validateChange(io.cwd, parsed.changeId!, { strict: parsed.strict });
    parsed.json
      ? writeJson(io, result.valid, result)
      : writeHuman(io, `Change ${parsed.changeId} is ${result.valid ? "valid" : "not ready"}`, result);
    return result.valid ? EXIT_CODES.success : EXIT_CODES.notReady;
  }
  const result = await archiveChange(io.cwd, parsed.changeId!);
  writeHuman(io, `Archived ${parsed.changeId}`, result);
  return EXIT_CODES.success;
}

export async function runCli(args: string[], io: CliIo): Promise<number> {
  try {
    const parsed = parseArgs(args);
    if (!parsed) {
      io.stdout(defaultHelp());
      return EXIT_CODES.success;
    }
    return await execute(parsed, io);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    const json = args.includes("--json");
    if (json) {
      writeJson(io, false, undefined, message);
    } else {
      io.stderr(`${message}\n`);
    }
    return error instanceof CliUsageError
      ? EXIT_CODES.usage
      : error instanceof ChangeNotReadyError
        ? EXIT_CODES.notReady
        : EXIT_CODES.failure;
  }
}
