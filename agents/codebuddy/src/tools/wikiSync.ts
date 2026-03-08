import { invokeCore, type CoreCommand } from "../runtime/invokeCore.js";

export type ToolInvoker = (command: CoreCommand) => Promise<unknown>;

/**
 * `wikiSync` 用于同步人工修改后的 Markdown 状态。
 *
 * @param invoke 用于调用 core 的执行函数。
 * @returns 返回一个只接收 `repoRoot` 的同步工具函数。
 */
export function createWikiSync(invoke: ToolInvoker = invokeCore) {
  return async ({ repoRoot }: { repoRoot: string }) =>
    invoke({ action: "sync", repoRoot });
}
