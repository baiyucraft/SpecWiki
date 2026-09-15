import { syncProjectAssets, type AssetSyncReport } from "../../core/assets/sync.js";
import { readProjectConfig, type WikiLanguage } from "../../core/config.js";
import {
  runAociIntegration,
  type AociIntegrationOptions,
  type AociResult,
} from "../aoci/runner.js";
import {
  runCodeGraphIntegration,
  type CodeGraphCommandRunner,
  type CodeGraphResult,
} from "../codegraph/runner.js";

export type BootstrapOutcome = "ready" | "incomplete" | "failed";

export type ToolBootstrapReport = {
  codegraph: CodeGraphResult;
  aoci: AociResult;
  ready: boolean;
  nextActions: string[];
};

export type BootstrapInitResult = {
  outcome: BootstrapOutcome;
  host: "codex";
  assets?: AssetSyncReport;
  tools?: ToolBootstrapReport;
  recoveryHint?: string;
  error?: string;
};

export type BootstrapInitOptions = {
  repoRoot: string;
  hosts?: string;
  env: NodeJS.ProcessEnv;
  force?: boolean;
  language?: WikiLanguage;
  codegraph?: {
    mode?: "required" | "skip";
    runner?: CodeGraphCommandRunner;
  };
  aoci?: {
    mode?: "required" | "skip";
    runner?: AociIntegrationOptions["runner"];
    installRoot?: string;
    installer?: AociIntegrationOptions["installer"];
    installerOptions?: AociIntegrationOptions["installerOptions"];
  };
};

function codeGraphReady(result: CodeGraphResult): boolean {
  return result.requested
    && result.cli.compatible
    && result.codexMcp.configured
    && result.project.healthy;
}

function aociReady(result: AociResult): boolean {
  return result.requested
    && result.available
    && result.version === result.expectedVersion
    && result.initialized
    && result.governanceAligned
    && result.database.ready;
}

export async function runBootstrapInit(options: BootstrapInitOptions): Promise<BootstrapInitResult> {
  if (options.hosts && options.hosts.trim().toLowerCase() !== "codex") {
    throw new Error(`unsupported host "${options.hosts}". Supported hosts: codex`);
  }
  try {
    const assets = await syncProjectAssets(options.repoRoot, {
      force: options.force,
      language: options.language,
    });
    const language = readProjectConfig(options.repoRoot).language;
    const codegraph = await runCodeGraphIntegration({
      projectRoot: options.repoRoot,
      env: options.env,
      mode: options.codegraph?.mode,
      runner: options.codegraph?.runner,
    });
    const aoci = await runAociIntegration({
      projectRoot: options.repoRoot,
      language,
      env: options.env,
      mode: options.aoci?.mode,
      runner: options.aoci?.runner,
      installRoot: options.aoci?.installRoot,
      installer: options.aoci?.installer,
      installerOptions: options.aoci?.installerOptions,
    });
    const ready = codeGraphReady(codegraph) && aociReady(aoci);
    const tools = {
      codegraph,
      aoci,
      ready,
      nextActions: [...new Set([...codegraph.nextActions, ...aoci.nextActions])],
    };
    return { outcome: ready ? "ready" : "incomplete", host: "codex", assets, tools };
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    return {
      outcome: "failed",
      host: "codex",
      error: message,
      recoveryHint: "resolve the failed project asset write and rerun spec-wiki-lite init",
    };
  }
}
