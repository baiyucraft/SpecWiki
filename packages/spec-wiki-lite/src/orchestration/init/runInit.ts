import { syncProjectAssets, type AssetSyncReport } from "../../core/assets/sync.js";

export type BootstrapOutcome = "ready" | "failed";

export type BootstrapInitResult = {
  outcome: BootstrapOutcome;
  host: "codex";
  assets?: AssetSyncReport;
  recoveryHint?: string;
  error?: string;
};

export type BootstrapInitOptions = {
  repoRoot: string;
  hosts?: string;
  env: NodeJS.ProcessEnv;
  force?: boolean;
};

export async function runBootstrapInit(
  options: BootstrapInitOptions,
): Promise<BootstrapInitResult> {
  if (options.hosts && options.hosts.trim().toLowerCase() !== "codex") {
    throw new Error(`unsupported host "${options.hosts}". Supported hosts: codex`);
  }

  try {
    const assets = await syncProjectAssets(options.repoRoot, { force: options.force });
    return { outcome: "ready", host: "codex", assets };
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
