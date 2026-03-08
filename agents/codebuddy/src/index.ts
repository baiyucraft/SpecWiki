/**
 * 这个文件是 CodeBuddy Agent 的公开入口。
 * 它只负责把各个 Repo Wiki 工具组装出来，不承担 Wiki 业务判断。
 */
import { invokeCore, type CoreCommand } from "./runtime/invokeCore.js";
import { createWikiInit } from "./tools/wikiInit.js";
import { createWikiQuery } from "./tools/wikiQuery.js";
import { createWikiRebuild } from "./tools/wikiRebuild.js";
import { createWikiStatus } from "./tools/wikiStatus.js";
import { createWikiSync } from "./tools/wikiSync.js";
import { createWikiUpdate } from "./tools/wikiUpdate.js";

/**
 * `createTools` 是 Agent 层的总装配入口。
 * 这里仅组合各个薄封装工具，不承载 Repo Wiki 的业务判断。
 *
 * @param invoke 用于调用 `wiki-core` 的底层执行函数；默认使用本地子进程调用。
 * @returns 返回一组可直接暴露给宿主的 Repo Wiki 工具函数。
 */
export function createTools(invoke: (command: CoreCommand) => Promise<unknown> = invokeCore) {
  return {
    wikiInit: createWikiInit(invoke),
    wikiStatus: createWikiStatus(invoke),
    wikiUpdate: createWikiUpdate(invoke),
    wikiQuery: createWikiQuery(invoke),
    wikiSync: createWikiSync(invoke),
    wikiRebuild: createWikiRebuild(invoke),
  };
}

/** 默认工具集合直接给宿主消费。 */
export const tools = createTools();
