import { invokeCore, type CoreCommand } from "../runtime/invokeCore.js";

export type ToolInvoker = (command: CoreCommand) => Promise<unknown>;

/**
 * `wikiQuery` 透传查询词给 core，匹配逻辑全部由 core 负责。
 *
 * @param invoke 用于调用 core 的执行函数。
 * @returns 返回一个接收 `repoRoot` 和 `term` 的查询工具函数。
 */
export function createWikiQuery(invoke: ToolInvoker = invokeCore) {
  return async ({ repoRoot, term }: { repoRoot: string; term: string }) =>
    invoke({ action: "query", repoRoot, term });
}
