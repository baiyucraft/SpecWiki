/** Public API for the TypeScript-only SpecWiki Lite package. */
export { runBin, type RunBinOptions } from "./bin.js";
export { type CliIo, EXIT_CODES, runCli } from "./cli.js";
export { type AssetSyncOptions, type AssetSyncReport, syncProjectAssets } from "./core/assets/sync.js";
export {
  archiveChange,
  type ArchiveClock,
  type ArchiveOptions,
  type ChangeArchiveReport,
  ChangeNotReadyError,
} from "./core/change/archive.js";
export { type ArtifactId, ARTIFACTS, isArtifactId } from "./core/change/artifacts.js";
export {
  CHANGE_STAGES,
  type ChangeMetadata,
  type ChangeStage,
  DELIVERY_SHAPES,
  type DeliveryShape,
  readMetadata,
} from "./core/change/metadata.js";
export { showChange, type ShowChangeResult } from "./core/change/show.js";
export { type ChangeStatusReport, getChangeStatus } from "./core/change/status.js";
export {
  type ChangeIssue,
  type ChangeIssueKind,
  type ChangeValidationResult,
  requiredArtifactsForStage,
  validateChange,
  type ValidateChangeOptions,
} from "./core/change/validate.js";
export {
  DEFAULT_WIKI_LANGUAGE,
  type ProjectConfig,
  readProjectConfig,
  WIKI_LANGUAGES,
  type WikiLanguage,
  writeProjectLanguage,
} from "./core/config.js";
export {
  assertCanonicalChangeId,
  PathSafetyError,
  resolveSafePath,
} from "./core/path.js";
export {
  getProjectStatus,
  type ProjectStatusOptions,
  type ProjectStatusReport,
  type ProjectToolsStatus,
  type SkillStatus,
} from "./core/status.js";
export {
  inspectWiki,
  type WikiInspectionReport,
  type WikiIssue,
  type WikiIssueKind,
} from "./core/wiki/inspect.js";
export {
  type CodeGraphCommandResult,
  type CodeGraphCommandRunner,
  type CodeGraphIntegrationOptions,
  type CodeGraphResult,
  type CodeGraphWarning,
  inspectCodeGraph,
  runCodeGraphIntegration,
} from "./orchestration/codegraph/runner.js";
export {
  inspectAoci,
  resolveInstalledAoci,
  runAociBootstrap,
  runAociIntegration,
  type AociDatabaseStatus,
  type AociIntegrationOptions,
  type AociResult,
  type AociRunOptions,
  type AociWarning,
} from "./orchestration/aoci/runner.js";
export {
  defaultAociInstallRoot,
  installAociRelease,
  type AociInstallerOptions,
  type AociInstallResult,
} from "./orchestration/aoci/installer.js";
export { AOCI_RELEASE, CODEGRAPH_RELEASE, selectAociAsset } from "./orchestration/tools/manifest.js";
export { invokeTool, runToolCommand, type ToolCommandResult, type ToolCommandRunner } from "./orchestration/tools/command.js";
export { type BootstrapInitResult, type ToolBootstrapReport, runBootstrapInit } from "./orchestration/init/runInit.js";
