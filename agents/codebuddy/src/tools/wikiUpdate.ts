import { invokeCore, type CoreCommand } from "../runtime/invokeCore.js";

export type ToolInvoker = (command: CoreCommand) => Promise<unknown>;

/**
 * `wikiUpdate` 触发 core 更新当前 Repo Wiki。
 *
 * @param invoke 用于调用 core 的执行函数。
 * @returns 返回一个只接收 `repoRoot` 的更新工具函数。
 */
export function createWikiUpdate(invoke: ToolInvoker = invokeCore) {
  return async ({ repoRoot }: { repoRoot: string }) =>
    invoke({ action: "update", repoRoot });
}
