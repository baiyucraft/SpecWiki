import { syncProjectAssets, type AssetSyncReport } from "../../core/assets/sync.js";
import type { WikiLanguage } from "../../core/config.js";
import {
  runCodeGraphIntegration,
  type CodeGraphCommandRunner,
  type CodeGraphResult,
} from "../codegraph/runner.js";

export type BootstrapOutcome = "ready" | "failed";

export type BootstrapInitResult = {
  outcome: BootstrapOutcome;
  host: "codex";
  assets?: AssetSyncReport;
  codegraph?: CodeGraphResult;
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
    enabled?: boolean;
    runner?: CodeGraphCommandRunner;
  };
};

export async function runBootstrapInit(
  options: BootstrapInitOptions,
): Promise<BootstrapInitResult> {
  if (options.hosts && options.hosts.trim().toLowerCase() !== "codex") {
    throw new Error(`unsupported host "${options.hosts}". Supported hosts: codex`);
  }

  try {
    const assets = await syncProjectAssets(options.repoRoot, {
      force: options.force,
      language: options.language,
    });
    const codegraph = await runCodeGraphIntegration({
      projectRoot: options.repoRoot,
      env: options.env,
      enabled: options.codegraph?.enabled,
      runner: options.codegraph?.runner,
    });
    return { outcome: "ready", host: "codex", assets, codegraph };
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
