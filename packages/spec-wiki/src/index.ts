export {
  detectHosts,
  HOSTS,
  parseRequestedHosts,
  resolveSelectedHosts,
  type SupportedHost,
} from "./agents/shared/hosts.js";
/**
 * 这个文件是 `spec-wiki` 的公开入口。
 * 它汇总 CLI、agents/orchestration 与 runtime 三层的对外导出。
 */
export { runBin, type RunBinOptions } from "./bin.js";
export { type CliIo, runCli } from "./cli.js";
export { type BootstrapInitResult, runBootstrapInit } from "./orchestration/init/runInit.js";
export { forwardCoreCommand, type ForwardCoreOptions } from "./runtime/forwardCore.js";
export {
  type CoreCommand,
  createCoreInvoker,
  invokeCore,
  type InvokeCoreOptions,
  STREAMING_ACTIONS,
} from "./runtime/invokeCore.js";
export {
  type ArchiveOperationManifest,
  type ArchiveReport,
  type CoreResponse,
  type CoreStreamEvent,
  parseEventLine,
  parseResult,
  responseFromTerminalEvent,
} from "./runtime/parseResult.js";
export {
  collectBinaryCandidates,
  resolveBinary,
  type ResolveBinaryOptions,
} from "./runtime/resolveBinary.js";
export { createTools, tools } from "./tools.js";
