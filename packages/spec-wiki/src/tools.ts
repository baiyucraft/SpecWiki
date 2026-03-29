/**
 * 这个文件暴露共享的 Wiki 工具集合。
 * 它给测试和宿主薄封装复用，不承担宿主特有的业务判断。
 */
import { invokeCore, type ToolInvoker } from "./runtime/invokeCore.js";
import type {
  QueryProfileData,
  StatusPreflightData,
  WorkflowTerminalData,
} from "./runtime/parseResult.js";

/**
 * 组装 Repo Wiki 的公开工具集合。
 *
 * @param invoke 底层 runtime 调用函数；默认使用本地子进程调用。
 * @returns 返回与 `v0.1.0` 公开 action 一一对应的薄封装工具。
 */
export function createTools(
  invoke: ToolInvoker = invokeCore,
) {
  return {
    wikiInit: ({ repoRoot }: { repoRoot: string }) =>
      invoke({ action: "init", repoRoot }) as Promise<{ ok: boolean; data?: WorkflowTerminalData }>,
    wikiStatus: ({ repoRoot }: { repoRoot: string }) =>
      invoke({ action: "status", repoRoot }) as Promise<{ ok: boolean; data?: StatusPreflightData }>,
    wikiUpdate: ({ repoRoot }: { repoRoot: string }) =>
      invoke({ action: "update", repoRoot }) as Promise<{ ok: boolean; data?: WorkflowTerminalData }>,
    wikiQuery: ({ repoRoot, term }: { repoRoot: string; term: string }) =>
      invoke({ action: "query", repoRoot, term }) as Promise<{ ok: boolean; data?: QueryProfileData }>,
  };
}

/** 默认工具集合直接给宿主或测试消费。 */
export const tools = createTools();
