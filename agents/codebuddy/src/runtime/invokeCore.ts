/**
 * 这个文件负责 Agent 与 `wiki-core` 的 JSON IPC。
 * 这里不解释业务语义，只处理进程启动、协议收发和错误透传。
 */
import { spawn } from "node:child_process";

import { parseResult, type CoreResponse } from "./parseResult.js";
import { resolveBinary } from "./resolveBinary.js";

export type CoreCommand = {
  /** 要执行的 core 工作流名称，例如 `init`、`query`。 */
  action: string;
  /** 待分析的本地代码目录。 */
  repoRoot?: string;
  /** `query` 之类命令使用的检索词。 */
  term?: string;
};

/**
 * 通过子进程调用 `wiki-core`。
 * Agent 层只负责 IPC、路径定位和错误透传，不解释返回的数据结构。
 *
 * @param command 要发送给 `wiki-core` 的命令对象。
 * @returns 返回 `wiki-core` 的统一响应对象。
 */
export async function invokeCore(command: CoreCommand): Promise<CoreResponse> {
  const binary = resolveBinary();

  return new Promise((resolve, reject) => {
    // `--json` 会让 core 走 stdin/stdout 协议模式。
    const child = spawn(binary, ["--json"], {
      stdio: ["pipe", "pipe", "pipe"],
    });

    let stdout = "";
    let stderr = "";

    child.stdout.on("data", (chunk) => {
      stdout += chunk.toString();
    });

    child.stderr.on("data", (chunk) => {
      stderr += chunk.toString();
    });

    child.on("error", (error) => {
      reject(new Error(`failed to launch wiki-core at ${binary}: ${error.message}`));
    });
    child.on("close", (code) => {
      if (code !== 0) {
        reject(new Error(stderr || `wiki-core exited with code ${code}`));
        return;
      }

      try {
        // 返回结果的协议校验单独放在 `parseResult`，让这里保持单一职责。
        resolve(parseResult(stdout.trim()));
      } catch (error) {
        reject(error);
      }
    });

    child.stdin.write(JSON.stringify(command));
    child.stdin.end();
  });
}
