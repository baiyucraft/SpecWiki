/**
 * 这个文件只暴露 `init` 工具，不承载额外业务逻辑。
 * 每个工具文件都保持“参数收敛后直通 core”的薄封装边界。
 */
import { invokeCore, type CoreCommand } from "../runtime/invokeCore.js";

export type ToolInvoker = (command: CoreCommand) => Promise<unknown>;

/**
 * `wikiInit` 是对 core `init` 命令的薄封装。
 *
 * @param invoke 用于调用 core 的执行函数。
 * @returns 返回一个只接收 `repoRoot` 的初始化工具函数。
 */
export function createWikiInit(invoke: ToolInvoker = invokeCore) {
  return async ({ repoRoot }: { repoRoot: string }) =>
    invoke({ action: "init", repoRoot });
}
