import { invokeCore, type CoreCommand } from "../runtime/invokeCore.js";

export type ToolInvoker = (command: CoreCommand) => Promise<unknown>;

export function createWikiQuery(invoke: ToolInvoker = invokeCore) {
  return async ({ repoRoot, term }: { repoRoot: string; term: string }) =>
    invoke({ action: "query", repoRoot, term });
}
