/**
 * 这个文件负责 Agent 与 `wiki-core` 的 JSON IPC。
 * 这里不解释业务语义，只处理进程启动、协议收发和错误透传。
 */
import { spawn } from "node:child_process";

import {
  parseEventLine,
  parseResult,
  responseFromTerminalEvent,
  type CoreProgressEvent,
  type CoreResponse,
  type CoreResultEvent,
  type CoreErrorEvent,
} from "./parseResult.js";
import { resolveBinary } from "./resolveBinary.js";

export type CoreCommand = {
  /** 要执行的 core 工作流名称，例如 `init`、`query`。 */
  action: string;
  /** 待分析的本地代码目录。 */
  repoRoot?: string;
  /** `query` 之类命令使用的检索词。 */
  term?: string;
  /** 长流程协议提示；当前由 invokeCore 内部自动打开。 */
  streamProgress?: boolean;
};

/** `invokeCore` 的附加运行选项。 */
export type InvokeCoreOptions = {
  /** 宿主若需要进度感知，可在这里消费阶段事件。 */
  onProgress?: (event: CoreProgressEvent) => void;
};

const STREAMING_ACTIONS = new Set(["init", "update", "rebuild"]);

/**
 * 通过子进程调用 `wiki-core`。
 * Agent 层只负责 IPC、路径定位和错误透传，不解释返回的数据结构。
 *
 * @param command 要发送给 `wiki-core` 的命令对象。
 * @returns 返回 `wiki-core` 的统一响应对象。
 */
export async function invokeCore(
  command: CoreCommand,
  options: InvokeCoreOptions = {},
): Promise<CoreResponse> {
  const binary = resolveBinary();
  const streamOutput = STREAMING_ACTIONS.has(command.action);
  const payload = streamOutput
    ? { ...command, streamProgress: true }
    : command;

  return new Promise((resolve, reject) => {
    // `--json` 会让 core 走 stdin/stdout 协议模式。
    const child = spawn(binary, ["--json"], {
      stdio: ["pipe", "pipe", "pipe"],
    });

    let stdout = "";
    let stderr = "";
    let stdoutBuffer = "";
    let terminal: CoreResponse | null = null;
    let streamError: Error | null = null;

    child.stdout.on("data", (chunk) => {
      const text = chunk.toString();
      stdout += text;

      if (!streamOutput) {
        return;
      }

      stdoutBuffer += text;
      drainEventBuffer(false);
    });

    child.stderr.on("data", (chunk) => {
      stderr += chunk.toString();
    });

    child.on("error", (error) => {
      reject(new Error(`failed to launch wiki-core at ${binary}: ${error.message}`));
    });
    child.on("close", (code) => {
      if (streamOutput) {
        drainEventBuffer(true);
      }

      if (code !== 0) {
        reject(new Error(stderr || `wiki-core exited with code ${code}`));
        return;
      }

      if (streamError) {
        reject(streamError);
        return;
      }

      try {
        if (streamOutput) {
          if (!terminal) {
            throw new Error("wiki-core stream ended without terminal event");
          }
          resolve(terminal);
          return;
        }

        // 返回结果的协议校验单独放在 `parseResult`，让这里保持单一职责。
        resolve(parseResult(stdout.trim()));
      } catch (error) {
        reject(error);
      }
    });

    child.stdin.write(JSON.stringify(payload));
    child.stdin.end();

    function drainEventBuffer(flushRemainder: boolean) {
      while (true) {
        const newlineIndex = stdoutBuffer.indexOf("\n");
        if (newlineIndex < 0) {
          break;
        }

        const line = stdoutBuffer.slice(0, newlineIndex).trim();
        stdoutBuffer = stdoutBuffer.slice(newlineIndex + 1);
        if (line.length === 0) {
          continue;
        }
        consumeEventLine(line);
      }

      if (flushRemainder && stdoutBuffer.trim().length > 0) {
        consumeEventLine(stdoutBuffer.trim());
        stdoutBuffer = "";
      }
    }

    function consumeEventLine(line: string) {
      if (streamError) {
        return;
      }

      try {
        const event = parseEventLine(line);
        if (event.type === "progress") {
          if (terminal) {
            throw new Error("received progress event after terminal event");
          }
          options.onProgress?.(event);
          return;
        }

        if (terminal) {
          throw new Error("received multiple terminal events from wiki-core");
        }

        terminal = responseFromTerminalEvent(event as CoreResultEvent | CoreErrorEvent);
      } catch (error) {
        streamError =
          error instanceof Error ? error : new Error(String(error));
      }
    }
  });
}
