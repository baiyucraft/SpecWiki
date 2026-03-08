import { invokeCore, type CoreCommand } from "../runtime/invokeCore.js";

export type ToolInvoker = (command: CoreCommand) => Promise<unknown>;

/**
 * `wikiStatus` 请求 core 判断当前 runtime 是否 fresh/stale/missing。
 *
 * @param invoke 用于调用 core 的执行函数。
 * @returns 返回一个只接收 `repoRoot` 的状态工具函数。
 */
export function createWikiStatus(invoke: ToolInvoker = invokeCore) {
  return async ({ repoRoot }: { repoRoot: string }) =>
    invoke({ action: "status", repoRoot });
}
