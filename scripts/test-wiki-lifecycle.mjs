/**
 * wiki-runtime 生命周期测试脚本。
 *
 * 这个入口负责把 `init -> status -> sync -> query -> update -> rebuild`
 * 按 phase 组合成可复跑的验收链路；
 * 进程超时、Windows 锁文件清理与顺序执行则复用共享 helpers。
 *
 * 用法：
 *   node scripts/test-wiki-lifecycle.mjs --list-phases
 *   node scripts/test-wiki-lifecycle.mjs --phase bootstrap
 *   node scripts/test-wiki-lifecycle.mjs --phase steady axum zustand
 *   node scripts/test-wiki-lifecycle.mjs --phase steady --timeout-minutes 120 dagger
 *   node scripts/test-wiki-lifecycle.mjs --phase mutation
 *   node scripts/test-wiki-lifecycle.mjs --phase rebuild
 *   node scripts/test-wiki-lifecycle.mjs --jobs 6
 *   node scripts/test-wiki-lifecycle.mjs                    # 默认 full
 */

import {
  appendFileSync,
  cpSync,
  existsSync,
  readdirSync,
  readFileSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { execFileSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

import {
  callCore,
  callCoreStreaming,
  COMMAND_TIMEOUT_GRACE_MS,
  ensureBinary,
  formatUsageSnapshot,
  isTransientFsErrorMessage,
  ROOT_DIR,
  removePathWithRetry,
  resolveProjectJobs,
  syncTemporaryDevConfig,
  runCommandCapture,
  runSequentialTasks,
  runTaskPool,
  TEST_DIR,
  TestRunner,
  withDevelopmentMode,
  withTemporaryDevConfig,
} from "./testing/helpers.mjs";
import { runInitWithResume } from "./testing/init-resume.mjs";
import {
  buildAcceptanceHarnessSummary,
  createFormalGateResults,
} from "./testing/quality-gates.mjs";
import { inspectWikiRuntime } from "./testing/wiki-runtime-inspection.mjs";

const SCRIPT_PATH = fileURLToPath(import.meta.url);

const REAL_REPO_MAP = {
  "aLocal": "E:\\project\\aLocal",
  "spec-wiki": ROOT_DIR,
};
const DIAGNOSTIC_RUNTIME_STATES = new Set(["runtime_incomplete", "blocker"]);
const WARM_RESTORE_SUPPORTED_STATES = new Set([
  "fresh",
  "needs_update",
  "runtime_incomplete",
  "blocker",
]);

export const LIFECYCLE_PHASES = {
  full: "完整链路：init → status → sync → query → update → touch/update → rebuild → status",
  bootstrap: "初始化链路：init → status",
  steady: "稳定态链路：init → sync → query → update(no-op)",
  mutation: "变更链路：init → touch source → status → update",
  rebuild: "重建链路：init → rebuild → status",
};

function resolveRunModes(runMode) {
  if (runMode === "both") {
    return [
      { label: "cold", cacheMode: "clear" },
      { label: "warm", cacheMode: "preserve" },
    ];
  }
  if (runMode === "warm") {
    return [{ label: "warm", cacheMode: "preserve" }];
  }
  return [{ label: "cold", cacheMode: "clear" }];
}

function formatElapsed(elapsedMs) {
  if (elapsedMs < 1_000) {
    return `${elapsedMs}ms`;
  }
  return `${(elapsedMs / 1_000).toFixed(elapsedMs >= 10_000 ? 0 : 1)}s`;
}

function promptCount(usage, promptType) {
  return usage?.by_prompt_type?.find((bucket) => bucket.key === promptType)?.request_count ?? 0;
}

function summarizeProgressUsage(progressEvents) {
  for (let index = progressEvents.length - 1; index >= 0; index--) {
    const event = progressEvents[index];
    if (event.phase === "llm_usage" && event.usage) {
      return event.usage;
    }
  }
  return null;
}

function createEmptyUsageSnapshot() {
  return {
    request_count: 0,
    input_tokens: 0,
    output_tokens: 0,
    total_tokens: 0,
    by_prompt_type: [],
    by_provider_model: [],
  };
}

function mergeUsageBuckets(targetBuckets, sourceBuckets) {
  const bucketMap = new Map(targetBuckets.map((bucket) => [bucket.key, { ...bucket }]));
  for (const bucket of sourceBuckets ?? []) {
    const current = bucketMap.get(bucket.key) ?? {
      key: bucket.key,
      request_count: 0,
      input_tokens: 0,
      output_tokens: 0,
      total_tokens: 0,
      provider: bucket.provider ?? null,
      model: bucket.model ?? null,
    };
    current.request_count += bucket.request_count ?? 0;
    current.input_tokens += bucket.input_tokens ?? 0;
    current.output_tokens += bucket.output_tokens ?? 0;
    current.total_tokens += bucket.total_tokens ?? 0;
    current.provider ??= bucket.provider ?? null;
    current.model ??= bucket.model ?? null;
    bucketMap.set(bucket.key, current);
  }
  return [...bucketMap.values()].sort((left, right) => left.key.localeCompare(right.key));
}

function mergeUsageSnapshot(target, usage) {
  if (!usage) {
    return target;
  }

  target.request_count += usage.request_count ?? 0;
  target.input_tokens += usage.input_tokens ?? 0;
  target.output_tokens += usage.output_tokens ?? 0;
  target.total_tokens += usage.total_tokens ?? 0;
  target.by_prompt_type = mergeUsageBuckets(target.by_prompt_type, usage.by_prompt_type);
  target.by_provider_model = mergeUsageBuckets(target.by_provider_model, usage.by_provider_model);
  return target;
}

function createLifecycleProgressLogger(project, runLabel) {
  const countedPercents = new Map();
  const phaseMessages = new Map();
  let lastUsageTotal = -1;

  return {
    info(message) {
      console.log(`[${project}/${runLabel}] ${message}`);
    },
    onProgress(event) {
      const prefix = `[${project}/${runLabel}] ${formatElapsed(event.elapsed_ms)} ${event.phase}`;
      if (event.phase === "llm_usage" && event.usage) {
        if (event.usage.total_tokens === lastUsageTotal) {
          return;
        }
        lastUsageTotal = event.usage.total_tokens;
        console.log(`${prefix} ${formatUsageSnapshot(event.usage)}`);
        return;
      }

      if (event.processed != null && event.total != null && event.total > 0) {
        const percent = Math.floor((event.processed / event.total) * 100);
        const lastPercent = countedPercents.get(event.phase) ?? -1;
        const shouldPrint
          = event.processed === 0
            || event.processed === event.total
            || percent >= lastPercent + 10;
        if (!shouldPrint) {
          return;
        }
        countedPercents.set(event.phase, percent);
        console.log(`${prefix} ${event.processed}/${event.total} ${event.message}`);
        return;
      }

      if (phaseMessages.get(event.phase) === event.message) {
        return;
      }
      phaseMessages.set(event.phase, event.message);
      console.log(`${prefix} ${event.message}`);
    },
  };
}

async function initViaRealRepo(proj, realRepo, ctx) {
  const projDir = path.join(TEST_DIR, proj);
  const wikiInReal = path.join(realRepo, ".wiki");
  const result = await runInitWithResume({
    logger: ctx.progressLogger,
    projectRoot: realRepo,
    repoRootArg: realRepo,
    initialCacheMode: ctx.cacheMode,
    timeoutMs: ctx.timeoutMs,
  });
  if (!result.response.ok) {
    throw new Error(result.response.error || `${proj} init failed`);
  }

  const wikiDest = path.join(projDir, ".wiki");
  removePathWithRetry(wikiDest);
  cpSync(wikiInReal, wikiDest, { recursive: true });
  return result;
}

function resolveProjectConfigRoot(ctx) {
  return ctx.isRealRepo ? REAL_REPO_MAP[ctx.proj] : ctx.projDir;
}

function callCoreWithProjectTimeout(ctx, command) {
  return callCore(withDevelopmentMode(command, ctx.developmentMode), {
    timeoutMs: ctx.timeoutMs,
  });
}

function isDiagnosticRuntimeState(state) {
  return DIAGNOSTIC_RUNTIME_STATES.has(String(state ?? ""));
}

export function coerceLifecycleStatusResult(statusResult, runtimeSnapshot) {
  if (!isDiagnosticRuntimeState(runtimeSnapshot?.runtimeState)) {
    return statusResult;
  }

  if (isDiagnosticRuntimeState(statusResult?.data?.state)) {
    return statusResult;
  }

  const data = {
    ...(statusResult?.data ?? {}),
    state: runtimeSnapshot.runtimeState,
  };

  if (runtimeSnapshot.runtimeState === "blocker") {
    data.query_readiness = "blocked";
    data.recommended_action = "rebuild";
    data.blocker_hint ??= runtimeSnapshot.incompleteReason ?? "runtime blocker captured on disk";
  } else if (runtimeSnapshot.runtimeState === "runtime_incomplete") {
    if (runtimeSnapshot.runtimeSummary && !data.runtime_summary) {
      data.runtime_summary = runtimeSnapshot.runtimeSummary;
    }
    if (data.runtime_summary) {
      data.query_readiness = "needs_update";
      data.recommended_action = "update";
    } else {
      data.query_readiness ??= "needs_init";
      data.recommended_action ??= "init";
    }
  }

  return {
    ...(statusResult ?? {}),
    ok: statusResult?.ok ?? true,
    error: statusResult?.error ?? null,
    data,
  };
}

export function shouldAttemptWarmRestorePreflight(ctx, runtimeSnapshot) {
  return !ctx.isRealRepo
    && ctx.cacheMode === "preserve"
    && runtimeSnapshot?.metadataExists === true
    && runtimeSnapshot?.cacheDbExists === false
    && existsSync(path.join(ctx.wikiDir, ".knowledge", "runtime", "recovery-manifest.json"));
}

export function shouldSkipInitAfterWarmRestore(statusResult) {
  return WARM_RESTORE_SUPPORTED_STATES.has(String(statusResult?.data?.state ?? ""));
}

export function shouldAcceptStatusAfterWarmRestore(statusResult) {
  return new Set(["fresh", "needs_update"]).has(String(statusResult?.data?.state ?? ""));
}

function applyLastKnownDiagnosticFallback(ctx, statusResult, runtimeSnapshot) {
  if (statusResult?.data?.state !== "missing" || !ctx.lastKnownDiagnostic?.runtimeSnapshot) {
    return { statusResult, runtimeSnapshot };
  }

  const fallbackSnapshot = ctx.lastKnownDiagnostic.runtimeSnapshot;
  const fallbackStatusResult = coerceLifecycleStatusResult(
    {
      ok: true,
      error: null,
      data: {
        ...(ctx.lastKnownDiagnostic.responseData ?? {}),
        state: ctx.lastKnownDiagnostic.state,
      },
    },
    fallbackSnapshot,
  );

  if (!isDiagnosticRuntimeState(fallbackStatusResult.data?.state)) {
    return { statusResult, runtimeSnapshot };
  }

  ctx.latestStatusResult = fallbackStatusResult;
  ctx.runtimeSnapshot = fallbackSnapshot;
  ctx.runtimeState = fallbackStatusResult.data.state;
  ctx.lifecycleMode = "diagnostic";
  console.log(
    `  [init/${ctx.runLabel}] fallback_to_last_known_diagnostic state=${fallbackStatusResult.data.state}`,
  );
  return {
    statusResult: fallbackStatusResult,
    runtimeSnapshot: fallbackSnapshot,
  };
}

function loadLifecycleRuntimeSnapshot(ctx) {
  const rawStatusResult = callCoreWithProjectTimeout(ctx, {
    action: "status",
    repoRoot: ctx.repoArg,
  });
  const runtimeSnapshot = inspectWikiRuntime(ctx.wikiDir);
  const statusResult = coerceLifecycleStatusResult(rawStatusResult, runtimeSnapshot);
  applyLifecycleRuntimeSnapshot(ctx, statusResult, runtimeSnapshot);
  return { runtimeSnapshot, statusResult };
}

function applyLifecycleRuntimeSnapshot(ctx, statusResult, runtimeSnapshot) {
  ctx.latestStatusResult = statusResult;
  ctx.runtimeSnapshot = runtimeSnapshot;
  ctx.runtimeState = statusResult.data?.state ?? runtimeSnapshot.runtimeState;
  ctx.lifecycleMode = ctx.runtimeState === "fresh"
    ? "acceptance"
    : isDiagnosticRuntimeState(ctx.runtimeState)
      ? "diagnostic"
      : "unknown";
}

function tryWarmRestorePreflight(ctx, t) {
  const runtimeSnapshot = inspectWikiRuntime(ctx.wikiDir);
  if (!shouldAttemptWarmRestorePreflight(ctx, runtimeSnapshot)) {
    return false;
  }

  console.log(`  [init/${ctx.runLabel}] warm-restore-preflight`);
  const rawStatusResult = callCoreWithProjectTimeout(ctx, {
    action: "status",
    repoRoot: ctx.repoArg,
  });
  const restoredSnapshot = inspectWikiRuntime(ctx.wikiDir);
  const statusResult = coerceLifecycleStatusResult(rawStatusResult, restoredSnapshot);
  if (!statusResult?.ok || !shouldSkipInitAfterWarmRestore(statusResult)) {
    console.log(
      `  [init/${ctx.runLabel}] warm-restore-preflight fallback state=${statusResult?.data?.state ?? "unknown"}`,
    );
    return false;
  }

  applyLifecycleRuntimeSnapshot(ctx, statusResult, restoredSnapshot);
  ctx.warmRestored = true;
  t.assertOk("warm restore preflight returns ok", statusResult);
  t.assertFileExists("warm restore rebuilt cache db", restoredSnapshot.cacheDbPath);
  console.log(
    `  [init/${ctx.runLabel}] warm-restore-preflight skipped init state=${ctx.runtimeState}`,
  );
  return true;
}

function assertDiagnosticRuntimeState(ctx, t, label, statusResult, runtimeSnapshot) {
  const state = statusResult.data?.state;
  if (!isDiagnosticRuntimeState(state)) {
    t.fail(`${label} enters diagnostic runtime state`, `unexpected state=${JSON.stringify(state)}`);
    return;
  }

  t.pass(`${label} enters ${state}`);
  t.assertFileExists(`${label} .wiki directory created`, ctx.wikiDir);
  runtimeSnapshot.cacheDbExists
    ? t.pass(`${label} cache db exists`)
    : t.fail(`${label} cache db exists`, `not found: ${runtimeSnapshot.cacheDbPath}`);

  if (state === "runtime_incomplete") {
    if (statusResult.data?.runtime_summary) {
      t.assertContains(`${label} query remains stale-but-usable`, statusResult.data, "query_readiness", "needs_update");
      t.pass(`${label} runtime summary present`);
    } else {
      t.pass(`${label} snapshot-backed runtime incomplete captured`);
    }
    runtimeSnapshot.incompleteReason
      ? t.pass(`${label} incomplete reason captured`)
      : t.fail(`${label} incomplete reason captured`, "missing incompleteReason");
    return;
  }

  t.assertContains(`${label} query is blocked`, statusResult.data, "query_readiness", "blocked");
  statusResult.data?.blocker_hint
    ? t.pass(`${label} blocker hint present`)
    : t.fail(`${label} blocker hint present`, "missing blocker_hint");
}

function finalizeInitOutcome(ctx, t, response, errorMessage = null, lastKnownDiagnostic = null) {
  let statusResult;
  let runtimeSnapshot;
  const transientFsFailure = isTransientFsErrorMessage(errorMessage)
    || isTransientFsErrorMessage(response?.error);
  ctx.lastKnownDiagnostic = lastKnownDiagnostic ?? ctx.lastKnownDiagnostic ?? null;

  try {
    ({ statusResult, runtimeSnapshot } = loadLifecycleRuntimeSnapshot(ctx));
  } catch (error) {
    if (response) {
      t.assertOk("init returns ok", response);
    } else {
      t.fail("init", errorMessage ?? error.message);
    }
    return false;
  }

  if (transientFsFailure || statusResult.data?.state === "missing") {
    ({ statusResult, runtimeSnapshot } = applyLastKnownDiagnosticFallback(
      ctx,
      statusResult,
      runtimeSnapshot,
    ));
  }

  if (response?.ok && statusResult.data?.state === "fresh") {
    t.pass("init returns ok");
    t.assertFileExists(".wiki directory created", ctx.wikiDir);
    t.assertMarkerCoverage("all pages have managed markers", ctx.wikiDir);
    t.assertFileExists("wiki.metadata.json exists", path.join(ctx.wikiDir, "wiki.metadata.json"));
    return true;
  }

  if (!transientFsFailure && isDiagnosticRuntimeState(statusResult.data?.state)) {
    const reason = errorMessage
      ? `error=${errorMessage}`
      : response?.error
        ? `response_error=${response.error}`
        : `state=${statusResult.data?.state}`;
    console.log(`  [init/${ctx.runLabel}] diagnostic_runtime ${reason}`);
    assertDiagnosticRuntimeState(ctx, t, "init", statusResult, runtimeSnapshot);
    return true;
  }

  if (response) {
    t.assertOk("init returns ok", response);
  } else {
    t.fail("init", errorMessage ?? "init failed");
  }
  t.fail(
    "status after init reaches supported state",
    `expected fresh/runtime_incomplete/blocker, got ${JSON.stringify(statusResult.data?.state)}`,
  );
  return false;
}

function discoverProjects() {
  if (!existsSync(TEST_DIR))
return [];
  return readdirSync(TEST_DIR)
    .filter((d) => statSync(path.join(TEST_DIR, d)).isDirectory())
    .sort();
}

const SOURCE_EXTS = new Set([
  ".rs",
  ".go",
  ".py",
  ".ts",
  ".js",
  ".java",
  ".kt",
  ".vue",
]);

const MUTATION_HOTSPOT_SEGMENTS = new Set([
  ".storybook",
  ".github",
  ".circleci",
  ".husky",
  ".nx",
  "buildsrc",
  "gradle",
  "scripts",
  "tools",
]);

const PREFERRED_SOURCE_SEGMENTS = new Set([
  "src",
  "main",
  "internal",
  "lib",
  "app",
]);

const CONFIG_BASENAME_PATTERN
  = /^(?:main|preview|manager|settings|build|vite|webpack|rollup|tsconfig|vitest|playwright|jest|eslint|prettier|babel|gradle)(?:[.-].+)?\.[a-z0-9]+$/i;

function sleepSync(ms) {
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
}

function querySqlite(dbPath, sql) {
  const statement = `PRAGMA busy_timeout=30000; ${sql}`;
  let lastError = null;

  for (let attempt = 0; attempt < 5; attempt++) {
    try {
      const output = execFileSync("sqlite3", [dbPath, statement], {
        encoding: "utf-8",
        timeout: 35_000,
      })
        .split(/\r?\n/)
        .map((line) => line.trim())
        .filter(Boolean);
      if (output[0] === "30000") {
        output.shift();
      }
      return output.at(-1) ?? "";
    } catch (error) {
      lastError = error;
      if (!String(error.stderr || error.message || "").includes("database is locked")) {
        throw error;
      }
      sleepSync(250);
    }
  }

  throw lastError;
}

function readSymbolCount(wikiDir) {
  const dbPath = path.join(wikiDir, ".cache", "wiki-cache.db");
  if (!existsSync(dbPath))
return 0;
  return Number(querySqlite(dbPath, "select count(*) from symbols;") || "0");
}

function readGraphCounts(wikiDir) {
  const dbPath = path.join(wikiDir, ".cache", "wiki-cache.db");
  if (!existsSync(dbPath)) {
    return {
      edges: 0,
      communities: 0,
      processes: 0,
    };
  }

  return {
    edges: Number(querySqlite(dbPath, "select count(*) from edges;") || "0"),
    communities: Number(querySqlite(dbPath, "select count(*) from communities;") || "0"),
    processes: Number(querySqlite(dbPath, "select count(*) from processes;") || "0"),
  };
}

function querySqliteRows(dbPath, sql) {
  const statement = `PRAGMA busy_timeout=30000; ${sql}`;
  let lastError = null;

  for (let attempt = 0; attempt < 5; attempt++) {
    try {
      return execFileSync("sqlite3", [dbPath, statement], {
        encoding: "utf-8",
        timeout: 35_000,
      })
        .split(/\r?\n/)
        .map((line) => line.trim())
        .filter(Boolean);
    } catch (error) {
      lastError = error;
      if (!String(error.stderr || error.message || "").includes("database is locked")) {
        throw error;
      }
      sleepSync(250);
    }
  }

  throw lastError;
}

function pickRepresentativeSymbol(wikiDir) {
  const dbPath = path.join(wikiDir, ".cache", "wiki-cache.db");
  if (!existsSync(dbPath))
return "";
  return querySqlite(
    dbPath,
    [
      "select name",
      "from symbols",
      "where length(name) >= 6",
      "and name glob '[A-Za-z_]*'",
      "and name not glob '[A-Z0-9_]*'",
      "group by name",
      "having count(*) = 1",
      "order by max(is_exported) desc,",
      "case min(label)",
      "when 'function' then 0",
      "when 'class' then 1",
      "when 'struct' then 2",
      "when 'interface' then 3",
      "when 'trait' then 4",
      "when 'enum' then 5",
      "when 'method' then 6",
      "when 'type' then 7",
      "else 8 end,",
      "case",
      "when length(name) between 8 and 24 then 0",
      "when length(name) between 25 and 40 then 1",
      "else 2 end,",
      "length(name) asc,",
      "name asc",
      "limit 1;",
    ].join(" "),
  );
}

function pickRepresentativeGraphSymbol(wikiDir) {
  const dbPath = path.join(wikiDir, ".cache", "wiki-cache.db");
  if (!existsSync(dbPath))
return "";
  return querySqlite(
    dbPath,
    [
      "select s.name",
      "from edges e",
      "join symbols s on s.id = e.source_id",
      "where length(s.name) >= 6",
      "and s.name glob '[A-Za-z_]*'",
      "and s.name not glob '[A-Z0-9_]*'",
      "group by s.name",
      "having count(*) = 1",
      "order by max(s.is_exported) desc,",
      "count(*) desc,",
      "length(s.name) asc,",
      "s.name asc",
      "limit 1;",
    ].join(" "),
  );
}

function captureSymbolProbe(ctx, t, label) {
  const symbolCount = readSymbolCount(ctx.wikiDir);
  const probe = {
    count: symbolCount,
    term: symbolCount > 0 ? pickRepresentativeSymbol(ctx.wikiDir) : "",
  };

  if (symbolCount > 0 && !probe.term) {
    t.fail(`${label} representative symbol selected`, "symbol table is non-empty but no probe term was chosen");
  } else if (symbolCount > 0) {
    t.pass(`${label} representative symbol selected`);
  } else {
    t.skip(`${label} symbol probe skipped (no symbols)`);
  }

  return probe;
}

function assertSymbolSnapshot(ctx, t, label, expectedCount = null) {
  const symbolCount = readSymbolCount(ctx.wikiDir);
  if (Number.isNaN(symbolCount)) {
    t.fail(`${label} symbol table readable`, "symbol count is NaN");
    return symbolCount;
  }
  expectedCount === null
    ? t.pass(`${label} symbol table readable (${symbolCount})`)
    : t.assertEqual(`${label} symbol count stable`, symbolCount, expectedCount);
  return symbolCount;
}

function captureGraphProbe(ctx, t, label) {
  const counts = readGraphCounts(ctx.wikiDir);
  const term = counts.edges > 0 ? pickRepresentativeGraphSymbol(ctx.wikiDir) : "";

  if (counts.edges > 0 && !term) {
    t.fail(
      `${label} representative graph symbol selected`,
      "graph edges exist but no representative graph symbol was chosen",
    );
  } else if (counts.edges > 0) {
    t.pass(`${label} representative graph symbol selected`);
  } else {
    t.skip(`${label} graph probe skipped (no graph edges)`);
  }

  return { counts, term };
}

function assertGraphSnapshot(ctx, t, label, expectedCounts = null) {
  const counts = readGraphCounts(ctx.wikiDir);
  if (expectedCounts === null) {
    t.pass(
      `${label} graph tables readable (edges=${counts.edges}, communities=${counts.communities}, processes=${counts.processes})`,
    );
    return counts;
  }

  t.assertEqual(`${label} edge count stable`, counts.edges, expectedCounts.edges);
  t.assertEqual(
    `${label} community count stable`,
    counts.communities,
    expectedCounts.communities,
  );
  t.assertEqual(
    `${label} process count stable`,
    counts.processes,
    expectedCounts.processes,
  );
  return counts;
}

function assertSymbolQuery(ctx, t, probe, label) {
  if (ctx.isRealRepo) {
    t.skip(`${label} symbol query skipped (real-repo project, .wiki not at repoRoot)`);
    return;
  }

  if (!probe.term) {
    t.skip(`${label} symbol query skipped (no representative symbol)`);
    return;
  }

  const queryResult = callCoreWithProjectTimeout(ctx, {
    action: "query",
    repoRoot: ctx.repoArg,
    term: probe.term,
  });
  t.assertOk(`${label} symbol query returns ok`, queryResult);

  const exactMatch = queryResult.data?.hits?.some(
    (hit) => hit.hit_type === "symbol" && hit.title === probe.term,
  );
  exactMatch
    ? t.pass(`${label} exact symbol hit present`)
    : t.fail(`${label} exact symbol hit present`, `missing ${probe.term} in compact hits`);
}

function assertGraphQuery(ctx, t, probe, label) {
  if (ctx.isRealRepo) {
    t.skip(`${label} graph query skipped (real-repo project, .wiki not at repoRoot)`);
    return;
  }

  if (!probe.term) {
    t.skip(`${label} graph query skipped (no representative graph symbol)`);
    return;
  }

  const queryResult = callCoreWithProjectTimeout(ctx, {
    action: "query",
    repoRoot: ctx.repoArg,
    term: probe.term,
  });
  t.assertOk(`${label} graph query returns ok`, queryResult);

  const matchedGraphTotal = queryResult.data?.hits?.filter(
    (hit) => hit.hit_type === "call_edge",
  ).length ?? 0;

  matchedGraphTotal > 0
    ? t.pass(`${label} graph query returns graph context`)
    : t.fail(
      `${label} graph query returns graph context`,
      `missing graph hits for ${probe.term}`,
    );

  queryResult.data?.provenance_summary?.includes("index_hit")
    ? t.pass(`${label} graph provenance summary recorded`)
    : t.fail(
      `${label} graph provenance summary recorded`,
      `summary=${JSON.stringify(queryResult.data?.provenance_summary ?? "")}`,
    );
}

function findSourceFile(projDir) {
  const walk = (dir, depth) => {
    if (depth > 6)
return null;
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = path.join(dir, entry.name);
      if (entry.isDirectory() && !entry.name.startsWith(".")) {
        const found = walk(full, depth + 1);
        if (found)
return found;
      } else if (SOURCE_EXTS.has(path.extname(entry.name))) {
        return full;
      }
    }
    return null;
  };
  return walk(projDir, 0);
}

function normalizeCandidateSegments(relativePath) {
  return String(relativePath ?? "")
    .replaceAll("\\", "/")
    .split("/")
    .filter(Boolean);
}

/**
 * 为 lifecycle mutation 选择尽量低扇出的源码叶子文件。
 *
 * 这个选择器优先真实实现文件，避开 `buildSrc`、`.storybook`、顶层 build/config
 * 之类会触发大面积失效的热点入口；否则 `update` 验证会退化成“重跑整仓 init”。
 *
 * @param {string[]} trackedPaths source_states 中的候选路径。
 * @param {string} projDir 测试项目根目录。
 * @returns {string | null} 返回命中的绝对路径；没有可用候选时返回 `null`。
 */
export function pickMutationSourceCandidate(trackedPaths, projDir) {
  const ranked = trackedPaths
    .map((relativePath) => {
      const fullPath = path.join(projDir, relativePath);
      if (!existsSync(fullPath)) {
        return null;
      }
      if (!SOURCE_EXTS.has(path.extname(fullPath))) {
        return null;
      }

      const segments = normalizeCandidateSegments(relativePath);
      if (segments.length === 0) {
        return null;
      }

      const lowerSegments = segments.map((segment) => segment.toLowerCase());
      const basename = lowerSegments.at(-1) ?? "";
      let score = Math.min(lowerSegments.length, 8);

      if (lowerSegments.some((segment) => PREFERRED_SOURCE_SEGMENTS.has(segment))) {
        score += 6;
      }
      if (lowerSegments.includes("src") && lowerSegments.includes("main")) {
        score += 4;
      }
      if (lowerSegments.some((segment) => MUTATION_HOTSPOT_SEGMENTS.has(segment))) {
        score -= 8;
      }
      if (
        basename.includes(".test.")
        || basename.includes(".spec.")
        || lowerSegments.includes("__tests__")
        || lowerSegments.includes("javatests")
      ) {
        score -= 4;
      }
      if (CONFIG_BASENAME_PATTERN.test(basename)) {
        score -= 6;
      }
      if (lowerSegments.length <= 2) {
        score -= 4;
      }

      return {
        fullPath,
        score,
        depth: lowerSegments.length,
        relativePath: String(relativePath).replaceAll("\\", "/"),
      };
    })
    .filter(Boolean)
    .sort((left, right) =>
      right.score - left.score
      || right.depth - left.depth
      || left.relativePath.localeCompare(right.relativePath),
    );

  return ranked[0]?.fullPath ?? null;
}

function findTrackedSourceFile(ctx) {
  const dbPath = path.join(ctx.wikiDir, ".cache", "wiki-cache.db");
  if (existsSync(dbPath)) {
    const trackedPaths = querySqliteRows(
      dbPath,
      "select path from source_states order by sort_order limit 256;",
    );
    const preferred = pickMutationSourceCandidate(trackedPaths, ctx.projDir);
    if (preferred) {
      return preferred;
    }
  }

  return findSourceFile(ctx.projDir);
}

async function runStreamingCommand(ctx, command, usageLabel) {
  const result = await callCoreStreaming(
    withDevelopmentMode(command, ctx.developmentMode),
    {
      onProgress: (event) => ctx.progressLogger?.onProgress(event),
      timeoutMs: ctx.timeoutMs,
    },
  );

  const usage = summarizeProgressUsage(result.progressEvents);
  if (usage) {
    mergeUsageSnapshot(ctx.usageSummary, usage);
    console.log(
      `  [usage/${ctx.runLabel}/${usageLabel}] ${formatUsageSnapshot(usage)} page_research=${promptCount(usage, "page_research")} page_enrichment=${promptCount(usage, "page_enrichment")}`,
    );
  }

  return result.response;
}

function syncRealRepoWikiSnapshot(ctx) {
  if (!ctx.isRealRepo) {
    return;
  }

  const realWikiDir = path.join(REAL_REPO_MAP[ctx.proj], ".wiki");
  if (!existsSync(realWikiDir)) {
    return;
  }

  removePathWithRetry(ctx.wikiDir);
  cpSync(realWikiDir, ctx.wikiDir, { recursive: true });
}

async function initProject(ctx, t) {
  const { proj, wikiDir, repoArg, isRealRepo } = ctx;

  console.log(`  [init/${ctx.runLabel}] cache_mode=${ctx.cacheMode}`);

  if (!isRealRepo && ctx.cacheMode === "clear" && existsSync(wikiDir)) {
    try {
      removePathWithRetry(wikiDir);
    } catch (error) {
      console.log(`  [init/${ctx.runLabel}] preclean skipped: ${error.message}`);
    }
  }

  if (tryWarmRestorePreflight(ctx, t)) {
    return true;
  }

  try {
    if (isRealRepo) {
      const result = await initViaRealRepo(proj, REAL_REPO_MAP[proj], ctx);
      ctx.cacheMode = result.effectiveCacheMode;
      const usage = summarizeProgressUsage(result.progressEvents);
      if (usage) {
        mergeUsageSnapshot(ctx.usageSummary, usage);
        console.log(
          `  [usage/${ctx.runLabel}/init] ${formatUsageSnapshot(usage)} page_research=${promptCount(usage, "page_research")} page_enrichment=${promptCount(usage, "page_enrichment")}`,
        );
      }
      if (result.resumedFromCheckpoint) {
        console.log(`  [init/${ctx.runLabel}] resumed with cache_mode=${ctx.cacheMode}`);
      }
      syncTemporaryDevConfig(resolveProjectConfigRoot(ctx), { cacheMode: ctx.cacheMode });
      return finalizeInitOutcome(ctx, t, result.response, null, result.lastKnownDiagnostic);
    } else {
      const result = await runInitWithResume({
        logger: ctx.progressLogger,
        projectRoot: resolveProjectConfigRoot(ctx),
        repoRootArg: repoArg,
        initialCacheMode: ctx.cacheMode,
        timeoutMs: ctx.timeoutMs,
      });
      ctx.cacheMode = result.effectiveCacheMode;
      const usage = summarizeProgressUsage(result.progressEvents);
      if (usage) {
        mergeUsageSnapshot(ctx.usageSummary, usage);
        console.log(
          `  [usage/${ctx.runLabel}/init] ${formatUsageSnapshot(usage)} page_research=${promptCount(usage, "page_research")} page_enrichment=${promptCount(usage, "page_enrichment")}`,
        );
      }
      if (result.resumedFromCheckpoint) {
        console.log(`  [init/${ctx.runLabel}] resumed with cache_mode=${ctx.cacheMode}`);
      }
      syncTemporaryDevConfig(resolveProjectConfigRoot(ctx), { cacheMode: ctx.cacheMode });
      return finalizeInitOutcome(ctx, t, result.response, null, result.lastKnownDiagnostic);
    }
  } catch (error) {
    return finalizeInitOutcome(ctx, t, null, error.message, error.lastKnownDiagnostic ?? null);
  }
}

function runStatusAfterInit(ctx, t) {
  console.log("  [status after init]");
  const statusAfterInit = ctx.latestStatusResult
    ?? callCoreWithProjectTimeout(ctx, { action: "status", repoRoot: ctx.repoArg });
  t.assertOk("status returns ok", statusAfterInit);
  if (ctx.lifecycleMode === "diagnostic") {
    assertDiagnosticRuntimeState(
      ctx,
      t,
      "status after init",
      statusAfterInit,
      ctx.runtimeSnapshot ?? inspectWikiRuntime(ctx.wikiDir),
    );
    return;
  }
  if (!ctx.isRealRepo) {
    if (ctx.warmRestored) {
      if (shouldAcceptStatusAfterWarmRestore(statusAfterInit)) {
        t.pass("state is warm-restore supported");
      } else {
        t.fail(
          "state is warm-restore supported",
          `expected state to be fresh/needs_update after warm restore, got ${JSON.stringify(statusAfterInit.data?.state)}`,
        );
      }
      return;
    }
    t.assertContains("state is fresh", statusAfterInit.data, "state", "fresh");
  }
}

function runSymbolSnapshotAfterInit(ctx, t) {
  console.log("  [symbols after init]");
  if (ctx.lifecycleMode === "diagnostic") {
    t.skip(`symbol snapshot skipped (runtime state ${ctx.runtimeState})`);
    return;
  }
  ctx.symbolProbe = captureSymbolProbe(ctx, t, "after init");
  ctx.initialSymbolCount = assertSymbolSnapshot(ctx, t, "after init");
  ctx.graphProbe = captureGraphProbe(ctx, t, "after init");
  ctx.initialGraphCounts = assertGraphSnapshot(ctx, t, "after init");
}

function runSyncNoChange(ctx, t) {
  console.log("  [sync no-change]");
  if (ctx.lifecycleMode === "diagnostic") {
    t.skip(`sync skipped (runtime state ${ctx.runtimeState})`);
    return;
  }
  if (!ctx.isRealRepo) {
    const syncResult = callCoreWithProjectTimeout(ctx, { action: "sync", repoRoot: ctx.repoArg });
    t.assertOk("sync returns ok", syncResult);
  } else {
    t.skip("sync skipped (real-repo project, .wiki not at repoRoot)");
  }
}

function runQuery(ctx, t) {
  console.log("  [query]");
  if (ctx.lifecycleMode === "diagnostic" && ctx.latestStatusResult?.data?.query_readiness !== "ready") {
    t.skip(`query skipped (runtime state ${ctx.runtimeState} is not query-ready)`);
    return;
  }
  assertSymbolQuery(ctx, t, ctx.symbolProbe || captureSymbolProbe(ctx, t, "query"), "steady");
  assertGraphQuery(ctx, t, ctx.graphProbe || captureGraphProbe(ctx, t, "query"), "steady");
}

async function runUpdateNoop(ctx, t) {
  console.log(`  [update no-op/${ctx.runLabel}]`);
  if (ctx.lifecycleMode === "diagnostic") {
    t.skip(`update skipped (runtime state ${ctx.runtimeState})`);
    return;
  }
  const updateNoop = await runStreamingCommand(
    ctx,
    { action: "update", repoRoot: ctx.repoArg },
    "update-noop",
  );
  syncRealRepoWikiSnapshot(ctx);
  t.assertOk("update returns ok", updateNoop);
  t.assertContains("state is fresh", updateNoop.data, "state", "fresh");
  assertSymbolSnapshot(ctx, t, "after no-op update", ctx.initialSymbolCount);
  assertGraphSnapshot(ctx, t, "after no-op update", ctx.initialGraphCounts);
  assertSymbolQuery(ctx, t, ctx.symbolProbe, "after no-op update");
  assertGraphQuery(ctx, t, ctx.graphProbe, "after no-op update");
}

async function runMutation(ctx, t) {
  console.log(`  [simulate source change/${ctx.runLabel}]`);
  if (ctx.lifecycleMode === "diagnostic") {
    t.skip(`simulate source change skipped (runtime state ${ctx.runtimeState})`);
    return;
  }
  if (ctx.isRealRepo) {
    t.skip("simulate source change skipped (real-repo project)");
    return;
  }

  const srcFile = findTrackedSourceFile(ctx);
  if (!srcFile) {
    t.skip("no source file found to touch");
    return;
  }

  const original = readFileSync(srcFile, "utf-8");
  appendFileSync(srcFile, "\n");

  try {
    const statusTouchRaw = callCoreWithProjectTimeout(ctx, {
      action: "status",
      repoRoot: ctx.repoArg,
    });
    const touchedRuntimeSnapshot = inspectWikiRuntime(ctx.wikiDir);
    let statusTouch = coerceLifecycleStatusResult(statusTouchRaw, touchedRuntimeSnapshot);
    if (ctx.lifecycleMode === "diagnostic" && statusTouch.data?.state === "missing") {
      ({ statusResult: statusTouch } = applyLastKnownDiagnosticFallback(
        ctx,
        statusTouch,
        touchedRuntimeSnapshot,
      ));
    }
    t.assertOk("status after touch returns ok", statusTouch);
    t.assertContains("state is needs_update after touch", statusTouch.data, "state", "needs_update");

    const updateTouch = await runStreamingCommand(
      ctx,
      { action: "update", repoRoot: ctx.repoArg },
      "update-touch",
    );
    t.assertOk("update after touch returns ok", updateTouch);
    t.assertContains("state is fresh after touch update", updateTouch.data, "state", "fresh");
    assertSymbolSnapshot(ctx, t, "after touch update", ctx.initialSymbolCount);
    assertGraphSnapshot(ctx, t, "after touch update", ctx.initialGraphCounts);
    assertSymbolQuery(ctx, t, ctx.symbolProbe, "after touch update");
    assertGraphQuery(ctx, t, ctx.graphProbe, "after touch update");
  } finally {
    writeFileSync(srcFile, original);
  }
}

async function runRebuild(ctx, t) {
  console.log(`  [rebuild/${ctx.runLabel}]`);
  if (ctx.lifecycleMode === "diagnostic") {
    t.skip(`rebuild skipped (runtime state ${ctx.runtimeState})`);
    return;
  }
  const rebuildResult = await runStreamingCommand(
    ctx,
    { action: "rebuild", repoRoot: ctx.repoArg },
    "rebuild",
  );
  syncRealRepoWikiSnapshot(ctx);
  t.assertOk("rebuild returns ok", rebuildResult);
  t.assertContains("rebuild state is fresh", rebuildResult.data, "state", "fresh");
  t.assertMarkerCoverage("markers preserved after rebuild", ctx.wikiDir);
  assertSymbolSnapshot(ctx, t, "after rebuild", ctx.initialSymbolCount);
  assertGraphSnapshot(ctx, t, "after rebuild", ctx.initialGraphCounts);
  assertSymbolQuery(ctx, t, ctx.symbolProbe, "after rebuild");
  assertGraphQuery(ctx, t, ctx.graphProbe, "after rebuild");
}

function runStatusAfterRebuild(ctx, t) {
  console.log("  [status after rebuild]");
  if (ctx.lifecycleMode === "diagnostic") {
    t.skip(`status after rebuild skipped (runtime state ${ctx.runtimeState})`);
    return;
  }
  const statusFinal = callCoreWithProjectTimeout(ctx, { action: "status", repoRoot: ctx.repoArg });
  t.assertOk("status returns ok", statusFinal);
  t.assertContains("state is fresh", statusFinal.data, "state", "fresh");
}

async function runProjectPhase(ctx, phase, t) {
  const initialized = await initProject(ctx, t);
  if (!initialized) {
    console.log("");
    return;
  }

  if (phase === "bootstrap") {
    runStatusAfterInit(ctx, t);
    runSymbolSnapshotAfterInit(ctx, t);
    console.log(
      `  [usage-summary/${ctx.runLabel}] ${formatUsageSnapshot(ctx.usageSummary)} page_research=${promptCount(ctx.usageSummary, "page_research")} page_enrichment=${promptCount(ctx.usageSummary, "page_enrichment")}`,
    );
    console.log("");
    return;
  }

  if (phase === "steady") {
    runSymbolSnapshotAfterInit(ctx, t);
    runSyncNoChange(ctx, t);
    runQuery(ctx, t);
    await runUpdateNoop(ctx, t);
    console.log(
      `  [usage-summary/${ctx.runLabel}] ${formatUsageSnapshot(ctx.usageSummary)} page_research=${promptCount(ctx.usageSummary, "page_research")} page_enrichment=${promptCount(ctx.usageSummary, "page_enrichment")}`,
    );
    console.log("");
    return;
  }

  if (phase === "mutation") {
    runSymbolSnapshotAfterInit(ctx, t);
    await runMutation(ctx, t);
    console.log(
      `  [usage-summary/${ctx.runLabel}] ${formatUsageSnapshot(ctx.usageSummary)} page_research=${promptCount(ctx.usageSummary, "page_research")} page_enrichment=${promptCount(ctx.usageSummary, "page_enrichment")}`,
    );
    console.log("");
    return;
  }

  if (phase === "rebuild") {
    runSymbolSnapshotAfterInit(ctx, t);
    await runRebuild(ctx, t);
    runStatusAfterRebuild(ctx, t);
    console.log(
      `  [usage-summary/${ctx.runLabel}] ${formatUsageSnapshot(ctx.usageSummary)} page_research=${promptCount(ctx.usageSummary, "page_research")} page_enrichment=${promptCount(ctx.usageSummary, "page_enrichment")}`,
    );
    console.log("");
    return;
  }

  runStatusAfterInit(ctx, t);
  runSymbolSnapshotAfterInit(ctx, t);
  runSyncNoChange(ctx, t);
  runQuery(ctx, t);
  await runUpdateNoop(ctx, t);
  await runMutation(ctx, t);
  await runRebuild(ctx, t);
  runStatusAfterRebuild(ctx, t);
  console.log(
    `  [usage-summary/${ctx.runLabel}] ${formatUsageSnapshot(ctx.usageSummary)} page_research=${promptCount(ctx.usageSummary, "page_research")} page_enrichment=${promptCount(ctx.usageSummary, "page_enrichment")}`,
  );
  console.log("");
}

async function runLifecycleProject(proj, options = {}) {
  const phase = options.phase || "full";
  const projDir = path.join(TEST_DIR, proj);
  const wikiDir = path.join(projDir, ".wiki");
  const repoArg = REAL_REPO_MAP[proj] || `tmp/test/${proj}`;
  const isRealRepo = !!REAL_REPO_MAP[proj];

  if (!existsSync(projDir)) {
    return {
      proj,
      ok: true,
      skipped: true,
      total: 0,
      passed: 0,
      failed: 0,
      logs: [],
    };
  }

  const captureLogs = options.captureLogs ?? false;
  const logs = [];
  const originalLog = console.log;
  if (captureLogs) {
    console.log = (...args) => {
      logs.push(args.join(" "));
    };
  }

  try {
    const runs = [];
    for (const run of resolveRunModes(options.runMode || "cold")) {
      const t = new TestRunner();
      const ctx = {
        isRealRepo,
        phase,
        proj,
        projDir,
        repoArg,
        wikiDir,
        cacheMode: run.cacheMode,
        developmentMode: false,
        runLabel: run.label,
        timeoutMs: options.timeoutMs,
        usageSummary: createEmptyUsageSnapshot(),
        progressLogger: createLifecycleProgressLogger(proj, run.label),
      };
      const configRepoRoot = isRealRepo ? REAL_REPO_MAP[proj] : projDir;
      await withTemporaryDevConfig(
        configRepoRoot,
        (devContext) => {
          ctx.developmentMode = devContext.developmentMode;
          return runProjectPhase(ctx, phase, t);
        },
        { cacheMode: ctx.cacheMode },
      );
      runs.push({
        label: run.label,
        cacheMode: run.cacheMode,
        effectiveCacheMode: ctx.cacheMode,
        total: t.total,
        passed: t.passed,
        failed: t.failed,
        usage: ctx.usageSummary,
      });
    }
    if (isRealRepo) {
      removePathWithRetry(path.join(REAL_REPO_MAP[proj], ".wiki"));
    }
    return {
      proj,
      ok: runs.every((run) => run.failed === 0),
      skipped: false,
      total: runs.reduce((sum, run) => sum + run.total, 0),
      passed: runs.reduce((sum, run) => sum + run.passed, 0),
      failed: runs.reduce((sum, run) => sum + run.failed, 0),
      runs,
      logs,
    };
  } finally {
    if (captureLogs) {
      console.log = originalLog;
    }
  }
}

function printLifecycleProjectResult(result, index, total, phase) {
  if (result.skipped) {
    console.log(`[${index + 1}/${total}] SKIP ${result.proj} (not found)`);
    return;
  }

  console.log(`[${index + 1}/${total}] ${result.proj} [phase=${phase}]`);
  for (const line of result.logs ?? []) {
    console.log(line);
  }
  for (const run of result.runs ?? []) {
    console.log(
      `    summary ${run.label}: assertions=${run.total} passed=${run.passed} failed=${run.failed} cache=${run.cacheMode}->${run.effectiveCacheMode || run.cacheMode} ${formatUsageSnapshot(run.usage)} page_research=${promptCount(run.usage, "page_research")} page_enrichment=${promptCount(run.usage, "page_enrichment")}`,
    );
  }
}

async function runLifecycleProjectInChild(proj, phase, runMode, timeoutMs) {
  const childArgs = buildLifecycleProjectChildArgs(proj, { phase, runMode, timeoutMs });

  for (let attempt = 0; attempt < 3; attempt++) {
    const child = await runCommandCapture(
      process.execPath,
      childArgs,
      {
        cwd: ROOT_DIR,
        killTreeOnTimeout: true,
        timeoutMs:
          (Number.isFinite(timeoutMs) && timeoutMs > 0 ? timeoutMs : 60 * 60_000)
          + COMMAND_TIMEOUT_GRACE_MS,
      },
    );

    if (child.timedOut) {
      throw new Error(child.stderr || `child worker for ${proj} timed out`);
    }
    if (child.stdout.trim()) {
      return JSON.parse(child.stdout.trim());
    }

    if (
      attempt < 2
      && isTransientFsErrorMessage(child.stderr || "")
    ) {
      await new Promise((resolve) => setTimeout(resolve, 1_000 * (attempt + 1)));
      continue;
    }

    throw new Error(child.stderr || `child worker for ${proj} produced empty stdout`);
  }

  throw new Error(`child worker for ${proj} exhausted retry budget`);
}

export function buildLifecycleSummary(results, options = {}) {
  let totalAssertions = 0;
  let failedAssertions = 0;
  let skipped = 0;
  let failedProjects = 0;
  let passedProjects = 0;

  const projectResults = results.map((result) => {
    totalAssertions += result.total ?? 0;
    failedAssertions += result.failed ?? 0;
    if (result.skipped) {
      skipped += 1;
    } else if ((result.failed ?? 0) > 0 || result.ok === false) {
      failedProjects += 1;
    } else {
      passedProjects += 1;
    }
    return {
      project: result.proj,
      ok: result.ok,
      skipped: result.skipped,
      total_assertions: result.total ?? 0,
      failed_assertions: result.failed ?? 0,
      run_labels: (result.runs ?? []).map((run) => run.label),
    };
  });

  return buildAcceptanceHarnessSummary({
    gateLevel: "baseline_guard",
    gateScope: "lifecycle",
    command: "node scripts/test-wiki-lifecycle.mjs",
    decision: failedAssertions > 0 ? "blocker" : "pass",
    phase: options.phase ?? "full",
    totals: {
      totalProjects: results.length,
      passedProjects,
      failedProjects,
      skippedProjects: skipped,
      totalAssertions,
      failedAssertions,
    },
    projectResults,
    formalGates: createFormalGateResults({
      artifact_validity: {
        decision: failedAssertions > 0 ? "blocker" : "pass",
        blocking: failedAssertions > 0,
        evidence_refs: ["init", "status after init"],
      },
      restore_validity: {
        decision: failedAssertions > 0 ? "blocker" : "pass",
        blocking: failedAssertions > 0,
        evidence_refs: ["warm restore preflight", "rebuild", "status after rebuild"],
      },
      query_route_contract: {
        decision: failedAssertions > 0 ? "blocker" : "pass",
        blocking: failedAssertions > 0,
        evidence_refs: ["query", "graph query", "symbol query"],
      },
      status_recommended_action_stability: {
        decision: failedAssertions > 0 ? "blocker" : "pass",
        blocking: failedAssertions > 0,
        evidence_refs: ["status after init", "status after touch", "status after rebuild"],
      },
    }),
    notes: [
      "`test-wiki-lifecycle.mjs` 是 baseline guard，用来验证 lifecycle 与 formal gate consumption，不替代 primary gate 样本。",
    ],
    relevantCapabilities: [
      "projection_readiness_recovery",
      "query_route_completeness",
      "answer_assembly_contract",
      "knowledge_quality_gates",
    ],
    samples: options.samples ?? [],
  });
}

export async function runLifecycleTestsWithSummary(names, options = {}) {
  const phase = options.phase || "full";
  if (!(phase in LIFECYCLE_PHASES)) {
    throw new Error(`unknown lifecycle phase: ${phase}`);
  }

  const projects = names && names.length > 0 ? names : discoverProjects();
  ensureBinary({ fresh: options.ensureFresh ?? true });

  const total = projects.length;
  const jobs = resolveProjectJobs(options.jobs, total);
  const useParallel = total > 1 && jobs > 1 && !options.childMode;
  let passed = 0;
  let failed = 0;
  let assertionTotal = 0;

  const results = useParallel
    ? await runTaskPool(projects, jobs, async (proj, index) => {
      const result = await runLifecycleProjectInChild(
        proj,
        phase,
        options.runMode || "cold",
        options.timeoutMs,
      );
      printLifecycleProjectResult(result, index, total, phase);
      return result;
    })
    : await runSequentialTasks(projects, async (proj, index) => {
      console.log(`[${index + 1}/${total}] ${proj} [phase=${phase}]`);
      const result = await runLifecycleProject(proj, {
        phase,
        runMode: options.runMode || "cold",
        timeoutMs: options.timeoutMs,
      });
      for (const run of result.runs ?? []) {
        console.log(
          `    summary ${run.label}: assertions=${run.total} passed=${run.passed} failed=${run.failed} cache=${run.cacheMode}->${run.effectiveCacheMode || run.cacheMode} ${formatUsageSnapshot(run.usage)} page_research=${promptCount(run.usage, "page_research")} page_enrichment=${promptCount(run.usage, "page_enrichment")}`,
        );
      }
      return result;
    });

  for (const result of results) {
    assertionTotal += result.total ?? 0;
    passed += result.passed ?? 0;
    failed += result.failed ?? 0;
  }

  console.log("");
  console.log("=== Results ===");
  console.log(
    `Total: ${assertionTotal}  \x1B[32mPassed: ${passed}\x1B[0m  \x1B[31mFailed: ${failed}\x1B[0m  jobs=${jobs}`,
  );
  return {
    ok: failed === 0,
    results,
    summary: buildLifecycleSummary(results, {
      phase,
      samples: projects.filter((project) => ["storybook", "dagger"].includes(project)),
    }),
  };
}

export async function runLifecycleTests(names, options = {}) {
  const result = await runLifecycleTestsWithSummary(names, options);
  return result.ok;
}

function printPhaseList() {
  console.log("Available lifecycle phases:");
  for (const [phase, description] of Object.entries(LIFECYCLE_PHASES)) {
    console.log(`  ${phase.padEnd(10)} ${description}`);
  }
}

export function parseCliArgs(argv) {
  const names = [];
  let phase = "full";
  let listPhases = false;
  let jobs;
  let childMode = false;
  let ensureFresh = true;
  let jsonSummary = false;
  let runMode = "cold";
  let timeoutMs;

  for (let index = 0; index < argv.length; index++) {
    const arg = argv[index];
    if (arg === "--phase") {
      phase = argv[index + 1] || phase;
      index++;
      continue;
    }
    if (arg === "--jobs") {
      jobs = argv[index + 1];
      index++;
      continue;
    }
    if (arg === "--list-phases") {
      listPhases = true;
      continue;
    }
    if (arg === "--run-mode") {
      runMode = argv[index + 1] || runMode;
      index++;
      continue;
    }
    if (arg === "--timeout-minutes") {
      const timeoutMinutes = Number(argv[index + 1]);
      if (Number.isFinite(timeoutMinutes) && timeoutMinutes > 0) {
        timeoutMs = Math.round(timeoutMinutes * 60_000);
      }
      index++;
      continue;
    }
    if (arg === "--child-json") {
      childMode = true;
      continue;
    }
    if (arg === "--json-summary") {
      jsonSummary = true;
      continue;
    }
    if (arg === "--no-build") {
      ensureFresh = false;
      continue;
    }
    names.push(arg);
  }

  return { childMode, ensureFresh, jobs, jsonSummary, listPhases, names, phase, runMode, timeoutMs };
}

/**
 * 构造 lifecycle child worker 的命令参数。
 *
 * @param proj 测试项目名。
 * @param options child worker 运行选项。
 * @returns 返回可直接传给 `node` 的参数数组。
 */
export function buildLifecycleProjectChildArgs(proj, options = {}) {
  const args = [
    SCRIPT_PATH,
    "--child-json",
    "--phase",
    options.phase || "full",
    "--run-mode",
    options.runMode || "cold",
    "--no-build",
  ];
  if (Number.isFinite(options.timeoutMs) && options.timeoutMs > 0) {
    args.push("--timeout-minutes", String(Math.ceil(options.timeoutMs / 60_000)));
  }
  args.push(proj);
  return args;
}

const entryHref = process.argv[1] ? pathToFileURL(process.argv[1]).href : null;

if (entryHref && import.meta.url === entryHref) {
  const { listPhases } = parseCliArgs(process.argv.slice(2));
  if (listPhases) {
    printPhaseList();
    process.exit(0);
  }

  const args = parseCliArgs(process.argv.slice(2));
  if (args.childMode) {
    const result = runLifecycleProject(args.names[0], {
      captureLogs: true,
      phase: args.phase,
      runMode: args.runMode,
      timeoutMs: args.timeoutMs,
    });
    process.stdout.write(JSON.stringify(await result));
    process.exit(0);
  }

  const result = await runLifecycleTestsWithSummary(args.names.length > 0 ? args.names : undefined, {
    ensureFresh: args.ensureFresh,
    jobs: args.jobs,
    phase: args.phase,
    runMode: args.runMode,
    timeoutMs: args.timeoutMs,
  });
  if (args.jsonSummary) {
    process.stdout.write(`${JSON.stringify(result.summary, null, 2)}\n`);
  }
  if (!result.ok)
process.exit(1);
}
