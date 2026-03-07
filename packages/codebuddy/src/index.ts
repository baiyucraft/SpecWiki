import { invokeCore, type CoreCommand } from "./runtime/invokeCore.js";
import { createWikiInit } from "./tools/wikiInit.js";
import { createWikiQuery } from "./tools/wikiQuery.js";
import { createWikiRebuild } from "./tools/wikiRebuild.js";
import { createWikiStatus } from "./tools/wikiStatus.js";
import { createWikiSync } from "./tools/wikiSync.js";
import { createWikiUpdate } from "./tools/wikiUpdate.js";

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

export const tools = createTools();
