import { invokeCore, type CoreCommand } from "../runtime/invokeCore.js";

export type ToolInvoker = (command: CoreCommand) => Promise<unknown>;

/**
 * `wikiRebuild` 显式触发全量重建。
 *
 * @param invoke 用于调用 core 的执行函数。
 * @returns 返回一个只接收 `repoRoot` 的重建工具函数。
 */
export function createWikiRebuild(invoke: ToolInvoker = invokeCore) {
  return async ({ repoRoot }: { repoRoot: string }) =>
    invoke({ action: "rebuild", repoRoot });
}
