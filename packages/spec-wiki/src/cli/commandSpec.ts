export type CommandName = "init" | "status" | "query" | "update" | "sync" | "rebuild" | "changes" | "change" | "validate";

export const MAIN_COMMANDS: readonly CommandName[] = ["init", "status", "query", "update"];
export const ADVANCED_COMMANDS: readonly CommandName[] = ["sync", "rebuild", "changes", "change", "validate"];
export const STREAMING_COMMANDS: ReadonlySet<CommandName> = new Set(["init", "update", "rebuild"]);

export function isCommand(value: string): value is CommandName {
  return [...MAIN_COMMANDS, ...ADVANCED_COMMANDS].includes(value as CommandName);
}
