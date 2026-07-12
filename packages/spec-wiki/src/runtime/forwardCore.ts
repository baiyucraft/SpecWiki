/**
 * 这个文件负责 CLI 形态的 runtime passthrough。
 * 它只做最小协议感知来决定退出码，stdout/stderr 仍保持原样透传。
 */
import type { Buffer } from "node:buffer";
import { spawn } from "node:child_process";

import type { CoreCommand } from "./invokeCore.js";
import { STREAMING_ACTIONS } from "./invokeCore.js";
import { parseResult } from "./parseResult.js";
import { CoreEventStream } from "./coreEventStream.js";
import { renderHumanEvent, renderHumanResponse } from "./humanRenderer.js";
import { exitCodeForResponse } from "./exitPolicy.js";
import { resolveBinary } from "./resolveBinary.js";
import { buildCoreEnv } from "./runtimeEnv.js";

export type ForwardCoreOptions = {
  /** CLI 当前工作目录。 */
  cwd?: string;
  /** 透传给子进程的环境变量。 */
  env?: NodeJS.ProcessEnv;
  /** 需要桥接会话时使用的标准输入。 */
  stdin?: NodeJS.ReadStream;
  /** 标准输出写入函数。 */
  stdout?: (text: string) => void;
  /** 标准错误写入函数。 */
  stderr?: (text: string) => void;
  /** 是否开启 stdin/stdout 会话桥接。 */
  bridgeStdio?: boolean;
  /** 测试时可替换二进制解析函数。 */
  binaryResolver?: () => string;
  /** 输出模式；默认 machine 以保持宿主 API 的原始协议。 */
  outputMode?: "human" | "machine";
  /** 用于 human renderer 的公开命令名。 */
  action?: string;
};

/**
 * 以原样 passthrough 的方式调用 `wiki-runtime`。
 *
 * @param command 发送给 `wiki-runtime` 的命令对象。
 * @param options CLI 侧的流与环境选项。
 * @returns 返回子进程的退出码。
 */
export async function forwardCoreCommand(
  command: CoreCommand,
  options: ForwardCoreOptions = {},
): Promise<number> {
  const binary = (options.binaryResolver ?? (() => resolveBinary()))();
  const streamOutput = command.action === "cli_init" || STREAMING_ACTIONS.has(command.action);
  const bridgeStdio = streamOutput && Boolean(options.bridgeStdio);
  const outputMode = options.outputMode ?? "machine";
  const payload = streamOutput
    ? {
        ...command,
        streamProgress: true,
        ...(bridgeStdio
          ? { llmBridge: { protocol: "ndjson_session_v1" as const } }
          : {}),
      }
    : command;

  return new Promise((resolve, reject) => {
    const child = spawn(binary, ["--json"], {
      cwd: options.cwd,
      env: buildCoreEnv(options.env ?? process.env, command.action),
      stdio: ["pipe", "pipe", "pipe"],
    });
    let stdout = "";
    let protocolError: Error | undefined;
    const eventStream = streamOutput
      ? new CoreEventStream({
          onEvent: (event) => {
            if (outputMode === "machine")
return;
            if (event.type === "progress" || event.type === "llm_request") {
              options.stdout?.(renderHumanEvent(event));
            }
          },
        })
      : undefined;

    const handleStdout = (chunk: Buffer | string) => {
      const text = chunk.toString();
      stdout += text;
      if (streamOutput) {
        if (outputMode === "machine")
options.stdout?.(text);
        try {
          eventStream?.push(text);
        } catch (error) {
          protocolError = error instanceof Error ? error : new Error(String(error));
          options.stderr?.(`${protocolError.message}\n`);
        }
      } else if (outputMode === "machine") {
        options.stdout?.(text);
      }
    };
    const handleStderr = (chunk: Buffer | string) => {
      options.stderr?.(chunk.toString());
    };
    const handleStdinData = (chunk: Buffer | string) => {
      child.stdin.write(chunk);
    };
    const handleStdinEnd = () => {
      child.stdin.end();
    };

    child.stdout.on("data", handleStdout);
    child.stderr.on("data", handleStderr);

    child.on("error", (error) => {
      reject(new Error(`failed to launch wiki-runtime at ${binary}: ${error.message}`));
    });
    child.on("close", (code) => {
      cleanup();
      if (code !== 0) {
        resolve(1);
        return;
      }

      try {
        if (streamOutput) {
          if (protocolError)
throw protocolError;
          const response = eventStream?.finish();
          if (outputMode === "human" && response)
options.stdout?.(renderHumanResponse(response, { action: options.action }));
          resolve(response ? exitCodeForResponse(response) : 1);
        } else {
          const response = parseResult(stdout.trim());
          if (outputMode === "human")
options.stdout?.(renderHumanResponse(response, { action: options.action }));
          resolve(exitCodeForResponse(response));
        }
      } catch {
        resolve(1);
      }
    });

    child.stdin.write(streamOutput ? `${JSON.stringify(payload)}\n` : JSON.stringify(payload));

    if (bridgeStdio && options.stdin) {
      options.stdin.on("data", handleStdinData);
      options.stdin.on("end", handleStdinEnd);
      options.stdin.resume();
      return;
    }

    child.stdin.end();

    function cleanup() {
      if (!options.stdin) {
        return;
      }

      options.stdin.off("data", handleStdinData);
      options.stdin.off("end", handleStdinEnd);
    }
  });
}
