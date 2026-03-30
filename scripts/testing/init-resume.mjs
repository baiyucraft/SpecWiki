/**
 * 重仓库 init 的 preserve-resume 共享策略。
 *
 * 这里把 timeout / provider 重试型失败后的 checkpoint 复用逻辑抽成共享 helper，
 * 让 `run-test-projects` 与 `test-wiki-lifecycle` 复用同一套判定，避免脚本间漂移。
 */

import path from "node:path";

import {
  callCoreStreaming,
  isTransientFsErrorMessage,
  isPreserveResumeEligibleInitErrorMessage,
  withTemporaryDevConfig,
} from "./helpers.mjs";
import {
  inspectWikiRuntime,
  readPipelineCheckpoint,
} from "./wiki-runtime-inspection.mjs";

export const MAX_INIT_RESUME_ATTEMPTS = 4;

function diagnosticStateFromSnapshot(runtimeSnapshot, response = null) {
  if (response?.data?.blocker_hint) {
    return "blocker";
  }
  return runtimeSnapshot?.runtimeState ?? null;
}

function rememberLastKnownDiagnostic(lastKnownDiagnostic, runtimeSnapshot, response = null) {
  const state = diagnosticStateFromSnapshot(runtimeSnapshot, response);
  if (
    state !== "blocker"
    && !(state === "runtime_incomplete" && runtimeSnapshot?.cacheDbExists)
  ) {
    return lastKnownDiagnostic;
  }

  return {
    state,
    responseData: response?.data ?? null,
    responseError: response?.error ?? null,
    runtimeSnapshot,
  };
}

function emitLoggerMessage(logger, message) {
  if (typeof logger?.log === "function") {
    logger.log(message);
    return;
  }
  if (typeof logger?.info === "function") {
    logger.info(message);
  }
}

async function runInitAttempt(projectRoot, repoRootArg, logger, cacheMode, timeoutMs) {
  return await withTemporaryDevConfig(
    projectRoot,
    (devContext) =>
      callCoreStreaming(
        devContext.command({ action: "init", repoRoot: repoRootArg }),
        {
          onProgress: (event) => logger?.onProgress(event),
          timeoutMs,
        },
      ),
    { cacheMode },
  );
}

function nextResumeDelayMs(message, attempt) {
  return isTransientFsErrorMessage(message)
    ? Math.min(30_000, attempt * 10_000)
    : Math.min(30_000, attempt * 5_000);
}

export function shouldResumeInitFromFailure({
  message,
  checkpoint,
  runtimeSnapshot,
  attempt,
  maxAttempts = MAX_INIT_RESUME_ATTEMPTS,
}) {
  const retryableMessage = isPreserveResumeEligibleInitErrorMessage(message)
    || isTransientFsErrorMessage(message);
  if (attempt >= maxAttempts || !retryableMessage) {
    return false;
  }
  const normalized = String(message ?? "").toLowerCase();
  if (
    normalized.includes("timed out after")
    && (
      runtimeSnapshot?.runtimeState === "missing"
      || runtimeSnapshot?.runtimeState === "runtime_incomplete"
      || !runtimeSnapshot?.cacheDbExists
    )
  ) {
    return true;
  }
  if (
    isTransientFsErrorMessage(message)
    && runtimeSnapshot?.cacheDbExists
    && !runtimeSnapshot?.metadataExists
    && runtimeSnapshot?.markdownPageCount === 0
  ) {
    return true;
  }
  if (!runtimeSnapshot?.cacheDbExists || runtimeSnapshot.metadataExists) {
    return false;
  }
  if (runtimeSnapshot.markdownPageCount > 0) {
    return false;
  }
  if (runtimeSnapshot.runtimeState !== "runtime_incomplete") {
    return false;
  }

  const workflowRuntimeState = runtimeSnapshot.runtimeSummary?.runtime_state;
  if (
    !["researching", "compose_pending", "compose_complete", "interrupted"].includes(
      workflowRuntimeState,
    )
  ) {
    return false;
  }

  if (normalized.includes("timed out after") || isTransientFsErrorMessage(message)) {
    return true;
  }

  return checkpoint != null;
}

export async function runInitWithResume({
  logger,
  projectRoot,
  repoRootArg,
  initialCacheMode,
  timeoutMs,
}) {
  let cacheMode = initialCacheMode;
  let resumedFromCheckpoint = false;
  let lastError = null;
  let lastKnownDiagnostic = null;

  for (let attempt = 1; attempt <= MAX_INIT_RESUME_ATTEMPTS; attempt++) {
    if (attempt > 1) {
      emitLoggerMessage(
        logger,
        `RETRY attempt=${attempt}/${MAX_INIT_RESUME_ATTEMPTS} cache_mode=${cacheMode}`,
      );
    }

    try {
      const result = await runInitAttempt(projectRoot, repoRootArg, logger, cacheMode, timeoutMs);
      if (!result.response?.ok) {
        const message = result.response?.error || `init failed for ${repoRootArg}`;
        const wikiDir = path.join(projectRoot, ".wiki");
        const runtimeSnapshot = inspectWikiRuntime(wikiDir);
        const checkpoint = readPipelineCheckpoint(runtimeSnapshot.cacheDbPath);
        lastKnownDiagnostic = rememberLastKnownDiagnostic(
          lastKnownDiagnostic,
          runtimeSnapshot,
          result.response,
        );
        if (
          shouldResumeInitFromFailure({
            message,
            checkpoint,
            runtimeSnapshot,
            attempt,
          })
        ) {
          resumedFromCheckpoint = true;
          cacheMode = "preserve";
          emitLoggerMessage(
            logger,
            `RESUME runtime_state=${runtimeSnapshot.runtimeState} checkpoint_stage=${checkpoint?.stage || runtimeSnapshot.runtimeSummary?.runtime_state || "unknown"} target=${checkpoint?.targetId || runtimeSnapshot.runtimeSummary?.current_research_unit_id || "n/a"}`,
          );
          await new Promise((resolve) => setTimeout(resolve, nextResumeDelayMs(message, attempt)));
          continue;
        }
      }
      return {
        ...result,
        effectiveCacheMode: cacheMode,
        resumedFromCheckpoint,
        lastKnownDiagnostic,
      };
    } catch (error) {
      lastError = error;
      const message = error instanceof Error ? error.message : String(error);
      const wikiDir = path.join(projectRoot, ".wiki");
      const runtimeSnapshot = inspectWikiRuntime(wikiDir);
      const checkpoint = readPipelineCheckpoint(runtimeSnapshot.cacheDbPath);
      lastKnownDiagnostic = rememberLastKnownDiagnostic(
        lastKnownDiagnostic,
        runtimeSnapshot,
      );
      if (
        !shouldResumeInitFromFailure({
          message,
          checkpoint,
          runtimeSnapshot,
          attempt,
        })
      ) {
        if (lastKnownDiagnostic && error instanceof Error) {
          error.lastKnownDiagnostic = lastKnownDiagnostic;
        }
        throw error;
      }

      resumedFromCheckpoint = true;
      cacheMode = "preserve";
      emitLoggerMessage(
        logger,
        `RESUME runtime_state=${runtimeSnapshot.runtimeState} checkpoint_stage=${checkpoint?.stage || runtimeSnapshot.runtimeSummary?.runtime_state || "unknown"} target=${checkpoint?.targetId || runtimeSnapshot.runtimeSummary?.current_research_unit_id || "n/a"}`,
      );
      await new Promise((resolve) => setTimeout(resolve, nextResumeDelayMs(message, attempt)));
    }
  }

  const terminalError = lastError ?? new Error(`init failed for ${repoRootArg}`);
  if (lastKnownDiagnostic && terminalError instanceof Error) {
    terminalError.lastKnownDiagnostic = lastKnownDiagnostic;
  }
  throw terminalError;
}
