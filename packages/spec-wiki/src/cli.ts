/**
 * 这个文件实现 `spec-wiki` 的命令行入口。
 * 顶层 `init` 只做参数解析后进入 orchestration，`wiki <action>` 只走 runtime forwarding。
 */
import { runBootstrapInit } from "./orchestration/init/runInit.js";
import { selectHostsForInit } from "./orchestration/init/selectHosts.js";
import { forwardCoreCommand } from "./runtime/forwardCore.js";
import { STREAMING_ACTIONS } from "./runtime/invokeCore.js";
import { isPublicWikiAction, PUBLIC_WIKI_ACTIONS } from "./wikiActions.js";

export type CliIo = {
  /** 当前 CLI 的工作目录。 */
  cwd: string;
  /** 当前 CLI 的环境变量。 */
  env: NodeJS.ProcessEnv;
  /** 需要 bridge 时复用的标准输入。 */
  stdin?: NodeJS.ReadStream;
  /** 显式覆盖交互能力判断；主要给测试使用。 */
  interactive?: boolean;
  /** 标准输出写入函数。 */
  stdout: (text: string) => void;
  /** 标准错误写入函数。 */
  stderr: (text: string) => void;
};

function renderUsage(): string {
  return [
    "Usage:",
    "  spec-wiki init [--tool <host> | --tools <host1,host2>] [--repo-root <path>] [--no-interactive]",
    "  spec-wiki wiki <action> [--repo-root <path>] [--term <text>] [--bridge-stdio]",
    "",
    "Supported hosts: codex, claude, codebuddy",
    `Supported actions: ${PUBLIC_WIKI_ACTIONS.join(", ")}`,
    "  --bridge-stdio only applies to long-running wiki actions such as init, update, and rebuild",
    "",
  ].join("\n");
}

function expectOptionValue(args: string[], index: number, optionName: string): string {
  const optionValue = args[index + 1];
  if (!optionValue || optionValue.startsWith("--")) {
    throw new Error(`missing value for ${optionName}`);
  }
  return optionValue;
}

/**
 * 运行 `spec-wiki` CLI。
 *
 * @param args 去掉可执行文件名后的命令行参数。
 * @param io CLI 的输入输出与环境。
 * @returns 返回最终退出码。
 */
export async function runCli(args: string[], io: CliIo): Promise<number> {
  try {
    if (args.length === 0 || args.includes("--help")) {
      io.stdout(renderUsage());
      return args.includes("--help") ? 0 : 1;
    }

    if (args[0] === "init") {
      let repoRoot = io.cwd;
      let tools: string | undefined;
      let interactive = io.interactive;

      for (let index = 1; index < args.length; index += 1) {
        const currentArg = args[index];

        if (currentArg === "--repo-root") {
          repoRoot = expectOptionValue(args, index, currentArg);
          index += 1;
          continue;
        }

        if (currentArg === "--tool" || currentArg === "--tools") {
          tools = expectOptionValue(args, index, currentArg);
          index += 1;
          continue;
        }

        if (currentArg === "--no-interactive") {
          interactive = false;
          continue;
        }

        throw new Error(`unsupported init argument: ${currentArg}`);
      }

      const selectedHosts = await selectHostsForInit({
        repoRoot,
        rawTools: tools,
        interactive,
        stdin: io.stdin,
      });

      const result = await runBootstrapInit({
        repoRoot,
        tools: selectedHosts.join(","),
        env: io.env,
      });

      for (const host of result.hosts) {
        const created = host.files.filter((file) => file.status === "created").length;
        const updated = host.files.filter((file) => file.status === "updated").length;
        const unchanged = host.files.filter((file) => file.status === "unchanged").length;
        io.stdout(
          `bootstrapped ${host.host}: created ${created}, updated ${updated}, unchanged ${unchanged}\n`,
        );
      }

      return 0;
    }

    if (args[0] === "wiki") {
      const action = args[1];
      if (!action || !isPublicWikiAction(action)) {
        throw new Error(
          `unsupported wiki action: ${action ?? "<missing>"}; current version only exposes ${PUBLIC_WIKI_ACTIONS.join(", ")}`,
        );
      }

      let repoRoot = io.cwd;
      let term: string | undefined;
      let bridgeStdio = false;
      const positionalArgs: string[] = [];

      for (let index = 2; index < args.length; index += 1) {
        const currentArg = args[index];

        if (currentArg === "--repo-root") {
          repoRoot = expectOptionValue(args, index, currentArg);
          index += 1;
          continue;
        }

        if (currentArg === "--term") {
          term = expectOptionValue(args, index, currentArg);
          index += 1;
          continue;
        }

        if (currentArg === "--bridge-stdio") {
          bridgeStdio = true;
          continue;
        }

        if (currentArg.startsWith("--")) {
          throw new Error(`unsupported wiki argument: ${currentArg}`);
        }

        positionalArgs.push(currentArg);
      }

      if (!term && action === "query" && positionalArgs.length > 0) {
        term = positionalArgs.join(" ");
      }

      if (action !== "query" && positionalArgs.length > 0) {
        throw new Error(`unexpected positional arguments for wiki ${action}`);
      }

      if (action === "query" && !term) {
        throw new Error("wiki query requires --term or positional query text");
      }
      if (bridgeStdio && !STREAMING_ACTIONS.has(action)) {
        throw new Error(
          `--bridge-stdio is only supported for long-running wiki actions; ${action} does not stream`,
        );
      }

      return forwardCoreCommand(
        {
          action,
          repoRoot,
          ...(term ? { term } : {}),
        },
        {
          bridgeStdio,
          cwd: io.cwd,
          env: io.env,
          stdin: io.stdin,
          stdout: io.stdout,
          stderr: io.stderr,
        },
      );
    }

    throw new Error(`unknown command: ${args[0]}`);
  } catch (error) {
    io.stderr(`${error instanceof Error ? error.message : String(error)}\n`);
    return 1;
  }
}



