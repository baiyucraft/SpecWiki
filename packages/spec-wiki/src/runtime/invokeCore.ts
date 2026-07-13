/**
 * 这个文件负责 Node 侧对 `wiki-runtime` 的 JSON IPC。
 * 它给宿主薄封装和测试使用，不负责命令行 passthrough。
 */
import { spawn } from "node:child_process";

import {
  parseEventLine,
  parseResult,
  responseFromTerminalEvent,
  type CoreKnownData,
  type CoreLlmRequest,
  type CoreLlmRequestEvent,
  type CoreProgressEvent,
  type CoreResponse,
  type CoreResultEvent,
  type CoreErrorEvent,
} from "./parseResult.js";
import { resolveBinary } from "./resolveBinary.js";
import { buildCoreEnv } from "./runtimeEnv.js";

export type CoreCommand = {
  /** 要执行的 core 工作流名称，例如 `init`、`query`。 */
  action: string;
  /** 待分析的本地代码目录。 */
  repoRoot?: string;
  /** `query` 命令使用的检索词。 */
  term?: string;
  changeId?: string;
  /** `archive` 的显式执行模式；CLI 默认发送 `dry_run`。 */
  archiveMode?: "dry_run" | "apply" | "resume";
  /** `archiveMode=resume` 时要恢复的 durable operation。 */
  archiveOperationId?: string;
  developmentMode?: boolean;
  bootstrap?: unknown;
  /** 长流程协议提示；当前由 invokeCore 内部自动打开。 */
  streamProgress?: boolean;
  /** 宿主若支持双向会话桥接，会在这里显式协商协议。 */
  llmBridge?: {
    protocol: "ndjson_session_v1";
  };
};

export type ToolInvoker<TData = unknown> = (command: CoreCommand) => Promise<CoreResponse<TData>>;

/** 单次 LLM 请求的宿主处理结果。 */
export type LlmBridgeResponse = {
  /** 宿主返回给 core 的结构化输出。 */
  output: unknown;
  /** 宿主实际使用的模型标识。 */
  model?: string | null;
};

/** `invokeCore` 的附加运行选项。 */
export type InvokeCoreOptions = {
  /** 宿主若需要进度感知，可在这里消费阶段事件。 */
  onProgress?: (event: CoreProgressEvent) => void;
  /** 需要覆盖默认子进程环境时使用。 */
  env?: NodeJS.ProcessEnv;
  /** 宿主若具备 LLM provider，可在这里桥接 `llm_request`。 */
  llmBridge?: {
    request: (request: CoreLlmRequest) => Promise<LlmBridgeResponse | null | undefined>;
  };
};

export type CoreInvoker = <TData = CoreKnownData>(
  command: CoreCommand,
  options?: InvokeCoreOptions,
) => Promise<CoreResponse<TData>>;

/** 这些 action 在 CLI/宿主里都必须保留 NDJSON 长流程事件。 */
export const STREAMING_ACTIONS = new Set(["init", "update", "rebuild"]);

/**
 * 创建一个面向指定 binary resolver 的 `invokeCore` 实现。
 *
 * @param binaryResolver 用于定位 `wiki-runtime` 的函数。
 * @returns 返回可直接执行 JSON IPC 的调用函数。
 */
export function createCoreInvoker(
  binaryResolver: () => string = () => resolveBinary(),
): CoreInvoker {
  return async function coreInvoker<TData = CoreKnownData>(
    command: CoreCommand,
    options: InvokeCoreOptions = {},
  ): Promise<CoreResponse<TData>> {
    const binary = binaryResolver();
    const streamOutput = STREAMING_ACTIONS.has(command.action);
    const llmBridgeEnabled = streamOutput && Boolean(options.llmBridge);
    const payload = streamOutput
      ? {
          ...command,
          streamProgress: true,
          ...(llmBridgeEnabled
            ? { llmBridge: { protocol: "ndjson_session_v1" as const } }
            : {}),
        }
      : command;

    return new Promise((resolve, reject) => {
      // `--json` 会让 runtime 走 stdin/stdout 协议模式。
      const child = spawn(binary, ["--json"], {
        env: buildCoreEnv(options.env ?? process.env, command.action),
        stdio: ["pipe", "pipe", "pipe"],
      });

      let stdout = "";
      let stderr = "";
      let stdoutBuffer = "";
      let terminal: CoreResponse<TData> | null = null;
      let streamError: Error | null = null;
      let pendingEventWork = Promise.resolve();

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
        reject(new Error(`failed to launch wiki-runtime at ${binary}: ${error.message}`));
      });
      child.on("close", (code) => {
        if (streamOutput) {
          drainEventBuffer(true);
        }

        void pendingEventWork
          .then(() => {
            if (code !== 0) {
              throw new Error(stderr || `wiki-runtime exited with code ${code}`);
            }

            if (streamError) {
              throw streamError;
            }

            if (streamOutput) {
              if (!terminal) {
                throw new Error("wiki-runtime stream ended without terminal event");
              }
              resolve(terminal);
              return;
            }

            resolve(parseResult(stdout.trim()) as CoreResponse<TData>);
          })
          .catch((error) => {
            reject(error instanceof Error ? error : new Error(String(error)));
          });
      });

      child.stdin.write(streamOutput ? `${JSON.stringify(payload)}\n` : JSON.stringify(payload));
      if (!llmBridgeEnabled) {
        child.stdin.end();
      }

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
          pendingEventWork = pendingEventWork.then(() => consumeEventLine(line));
        }

        if (flushRemainder && stdoutBuffer.trim().length > 0) {
          const finalLine = stdoutBuffer.trim();
          pendingEventWork = pendingEventWork.then(() => consumeEventLine(finalLine));
          stdoutBuffer = "";
        }
      }

      async function consumeEventLine(line: string) {
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

          if (event.type === "llm_request") {
            if (terminal) {
              throw new Error("received llm_request after terminal event");
            }
            await handleLlmRequest(event);
            return;
          }

          if (terminal) {
            throw new Error("received multiple terminal events from wiki-runtime");
          }

          terminal = responseFromTerminalEvent(event as CoreResultEvent | CoreErrorEvent) as CoreResponse<TData>;
        } catch (error) {
          streamError = error instanceof Error ? error : new Error(String(error));
        }
      }

      async function handleLlmRequest(event: CoreLlmRequestEvent) {
        const requestId = event.request.request_id;

        if (!options.llmBridge) {
          child.stdin.write(
            `${JSON.stringify({
              type: "llm_unavailable",
              requestId,
              reason: "agent_llm_bridge_unavailable",
            })}\n`,
          );
          return;
        }

        try {
          const response = await options.llmBridge.request(event.request);
          if (!response) {
            child.stdin.write(
              `${JSON.stringify({
                type: "llm_unavailable",
                requestId,
                reason: "agent_llm_bridge_returned_empty",
              })}\n`,
            );
            return;
          }

          child.stdin.write(
            `${JSON.stringify({
              type: "llm_response",
              requestId,
              response: {
                output: response.output,
                model: response.model ?? null,
              },
            })}\n`,
          );
        } catch (error) {
          child.stdin.write(
            `${JSON.stringify({
              type: "llm_unavailable",
              requestId,
              reason: error instanceof Error ? error.message : String(error),
            })}\n`,
          );
        }
      }
    });
  };
}

/** 默认给宿主直接使用的 `wiki-runtime` 调用函数。 */
export const invokeCore = createCoreInvoker();
