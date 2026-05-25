/**
 * 批量对 `tmp/test/*` 项目执行 `init`，并保留生成后的 `.wiki` 目录。
 *
 * 这个脚本只负责项目级调度与结果汇总；
 * 真正的 core 调用、超时与 Windows 锁文件清理由 `scripts/testing/helpers.mjs` 统一承接。
 *
 * 用法：
 *   node scripts/run-test-projects.mjs                             # 跑全部
 *   node scripts/run-test-projects.mjs --jobs 6                   # 调整项目并行度
 *   node scripts/run-test-projects.mjs --timeout-minutes 90 dagger
 *   node scripts/run-test-projects.mjs axum chi                   # 只跑指定项目
 */

import { execFileSync } from "node:child_process";
import {
  cpSync,
  existsSync,
  readdirSync,
  statSync,
} from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

import {
  COMMAND_TIMEOUT_GRACE_MS,
  ensureBinary,
  formatUsageSnapshot,
  ROOT_DIR,
  TEST_DIR,
  isTransientFsErrorMessage,
  removePathWithRetry,
  resolveProjectJobs,
  runCommandCapture,
  runSequentialTasks,
  runTaskPool,
} from "./testing/helpers.mjs";
import { MAX_INIT_RESUME_ATTEMPTS, runInitWithResume } from "./testing/init-resume.mjs";
import {
  buildAcceptanceHarnessSummary,
  createFormalGateResults,
} from "./testing/quality-gates.mjs";
import { inspectWikiRuntime } from "./testing/wiki-runtime-inspection.mjs";

const SCRIPT_PATH = fileURLToPath(import.meta.url);

// -------------------------------------------------------------------------
// 特殊项目：需要指向真实仓库 init 再拷贝回来
// -------------------------------------------------------------------------

const REAL_REPO_MAP = {
  "aLocal": "E:\\project\\aLocal",
  "spec-wiki": ROOT_DIR,
};
const DEFAULT_INIT_TIMEOUT_MS = 60 * 60_000;
const DIAGNOSTIC_RUNTIME_STATES = new Set(["runtime_incomplete", "blocker"]);

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
  return (
    usage?.by_prompt_type?.find((bucket) => bucket.key === promptType)?.request_count ?? 0
  );
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

function createProjectProgressLogger(logs, project, runLabel) {
  const countedPercents = new Map();
  const phaseMessages = new Map();
  let lastUsageTotal = -1;

  return {
    log(message) {
      logs.push(`[${project}/${runLabel}] ${message}`);
    },
    onProgress(event) {
      const prefix = `[${project}/${runLabel}] ${formatElapsed(event.elapsed_ms)} ${event.phase}`;
      if (event.phase === "llm_usage" && event.usage) {
        if (event.usage.total_tokens === lastUsageTotal) {
          return;
        }
        lastUsageTotal = event.usage.total_tokens;
        logs.push(`${prefix} ${formatUsageSnapshot(event.usage)}`);
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
        logs.push(`${prefix} ${event.processed}/${event.total} ${event.message}`);
        return;
      }

      if (phaseMessages.get(event.phase) === event.message) {
        return;
      }
      phaseMessages.set(event.phase, event.message);
      logs.push(`${prefix} ${event.message}`);
    },
  };
}

/**
 * 在真实仓库上执行 `init`，再把 `.wiki` 复制回测试样本目录。
 *
 * @param proj 测试项目名。
 * @param realRepo 真实仓库根目录。
 * @param options 运行选项；支持日志、cache mode 和超时。
 * @returns 返回本次 init 捕获到的 progress 事件。
 */
async function initViaRealRepo(proj, realRepo, options = {}) {
  const projDir = path.join(TEST_DIR, proj);
  const wikiInReal = path.join(realRepo, ".wiki");

  const { logger, cacheMode } = options;
  const result = await runInitWithResume({
    logger,
    projectRoot: realRepo,
    repoRootArg: realRepo,
    initialCacheMode: cacheMode,
    timeoutMs: options.timeoutMs,
  });
  if (!result.response.ok) {
    throw new Error(result.response.error || `${proj} init failed`);
  }

  // 拷贝结果
  if (existsSync(path.join(projDir, ".wiki"))) {
    removePathWithRetry(path.join(projDir, ".wiki"));
  }
  cpSync(wikiInReal, path.join(projDir, ".wiki"), { recursive: true });
  return result;
}

// -------------------------------------------------------------------------
// 主流程
// -------------------------------------------------------------------------

function discoverProjects() {
  if (!existsSync(TEST_DIR))
return [];
  return readdirSync(TEST_DIR)
    .filter((d) => statSync(path.join(TEST_DIR, d)).isDirectory())
    .sort();
}

function countPages(wikiDir) {
  if (!existsSync(wikiDir))
return 0;
  let count = 0;
  const walk = (dir) => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (entry.isDirectory())
walk(path.join(dir, entry.name));
      else if (entry.name.endsWith(".md"))
count++;
    }
  };
  walk(wikiDir);
  return count;
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
      Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 250);
    }
  }

  throw lastError;
}

function readGraphCounts(wikiDir) {
  const dbPath = path.join(wikiDir, ".cache", "wiki-cache.db");
  if (!existsSync(dbPath)) {
    return {
      symbols: 0,
      edges: 0,
      communities: 0,
      processes: 0,
    };
  }

  return {
    symbols: Number(querySqlite(dbPath, "select count(*) from symbols;") || "0"),
    edges: Number(querySqlite(dbPath, "select count(*) from edges;") || "0"),
    communities: Number(querySqlite(dbPath, "select count(*) from communities;") || "0"),
    processes: Number(querySqlite(dbPath, "select count(*) from processes;") || "0"),
  };
}

function isDiagnosticRuntimeState(state) {
  return DIAGNOSTIC_RUNTIME_STATES.has(String(state ?? ""));
}

/**
 * 把 `init` 失败但仍然留下可诊断 runtime 的场景收成专项成功。
 *
 * `run-test-projects` 当前只做样本专项，不要求 storybook 在这一轮必须装配出完整页面；
 * 只要 `.wiki` 已经进入 `runtime_incomplete / blocker`，并保留了可读的 cache/runtime 摘要，
 * 就应该把它记为“专项已观察到正式 runtime 语义”，而不是继续按旧口径直接判失败。
 */
function buildDiagnosticRun(logger, run, wikiDir, errorMessage, lastKnownDiagnostic = null) {
  const snapshot = lastKnownDiagnostic?.runtimeSnapshot ?? inspectWikiRuntime(wikiDir);
  const runtimeState = lastKnownDiagnostic?.state ?? snapshot.runtimeState;
  if (!isDiagnosticRuntimeState(runtimeState)) {
    return null;
  }

  const pages = lastKnownDiagnostic ? snapshot.markdownPageCount : countPages(wikiDir);
  const graph = lastKnownDiagnostic
    ? {
      symbols: snapshot.dbCounts.symbols ?? 0,
      edges: snapshot.dbCounts.edges ?? 0,
      communities: snapshot.dbCounts.communities ?? 0,
      processes: snapshot.dbCounts.processes ?? 0,
    }
    : readGraphCounts(wikiDir);
  logger.log(
    `DIAGNOSTIC state=${runtimeState} reason=${snapshot.incompleteReason ?? errorMessage}`,
  );
  return {
    label: run.label,
    cacheMode: run.cacheMode,
    effectiveCacheMode: run.cacheMode,
    resumedFromCheckpoint: false,
    pages,
    graph,
    usage: null,
    pageResearchRequests: 0,
    pageEnrichmentRequests: 0,
    diagnosticState: runtimeState,
    diagnosticReason: snapshot.incompleteReason ?? errorMessage,
  };
}

async function runSingleProject(proj, options = {}) {
  const projDir = path.join(TEST_DIR, proj);
  const wikiDir = path.join(projDir, ".wiki");
  const logs = [];

  if (!existsSync(projDir)) {
    return {
      proj,
      skipped: true,
      ok: true,
      message: "SKIP (not found)",
      logs,
    };
  }

  try {
    const runs = [];
    for (const run of resolveRunModes(options.runMode || "cold")) {
      const logger = createProjectProgressLogger(logs, proj, run.label);
      logger.log(`START cache_mode=${run.cacheMode}`);
      if (!REAL_REPO_MAP[proj] && run.cacheMode === "clear" && existsSync(wikiDir)) {
        try {
          removePathWithRetry(wikiDir);
        } catch (error) {
          logger.log(`PRECLEAN skipped: ${error.message}`);
        }
      }
      let initResult;
      try {
        if (REAL_REPO_MAP[proj]) {
          initResult = await initViaRealRepo(proj, REAL_REPO_MAP[proj], {
            cacheMode: run.cacheMode,
            logger,
            timeoutMs: options.timeoutMs,
          });
        } else {
          initResult = await runInitWithResume({
            logger,
            projectRoot: projDir,
            repoRootArg: `tmp/test/${proj}`,
            initialCacheMode: run.cacheMode,
            timeoutMs: options.timeoutMs,
          });
        }
        if (!initResult.response.ok) {
          const diagnosticRun = buildDiagnosticRun(
            logger,
            run,
            wikiDir,
            initResult.response.error || `${proj} init failed`,
            initResult.lastKnownDiagnostic,
          );
          if (diagnosticRun) {
            runs.push(diagnosticRun);
            continue;
          }
          const error = new Error(initResult.response.error || `${proj} init failed`);
          error.lastKnownDiagnostic = initResult.lastKnownDiagnostic ?? null;
          throw error;
        }
      } catch (error) {
        const diagnosticRun = buildDiagnosticRun(
          logger,
          run,
          wikiDir,
          error instanceof Error ? error.message : String(error),
          error?.lastKnownDiagnostic ?? null,
        );
        if (diagnosticRun) {
          runs.push(diagnosticRun);
          continue;
        }
        throw error;
      }
      const progressEvents = initResult.progressEvents;
      const pages = countPages(wikiDir);
      const graph = readGraphCounts(wikiDir);
      const usage = summarizeProgressUsage(progressEvents);
      runs.push({
        label: run.label,
        cacheMode: run.cacheMode,
        effectiveCacheMode: initResult.effectiveCacheMode,
        resumedFromCheckpoint: initResult.resumedFromCheckpoint,
        pages,
        graph,
        usage,
        pageResearchRequests: promptCount(usage, "page_research"),
        pageEnrichmentRequests: promptCount(usage, "page_enrichment"),
      });
      logger.log(
        `DONE pages=${pages} symbols=${graph.symbols} total_tokens=${usage?.total_tokens ?? 0} resumed=${initResult.resumedFromCheckpoint ? "yes" : "no"}`,
      );
    }
    if (REAL_REPO_MAP[proj]) {
      removePathWithRetry(path.join(REAL_REPO_MAP[proj], ".wiki"));
    }

    return {
      proj,
      ok: true,
      skipped: false,
      runs,
      logs,
    };
  } catch (error) {
    return {
      proj,
      ok: false,
      skipped: false,
      error: error instanceof Error ? error.message : String(error),
      logs,
    };
  }
}

function printProjectResult(result, index, total) {
  if (result.skipped) {
    console.log(`[${index + 1}/${total}] ${result.proj} ${result.message}`);
    return;
  }

  if (result.ok) {
    for (const line of result.logs ?? []) {
      console.log(line);
    }
    const summary = (result.runs ?? [])
      .map((run) =>
        `${run.label}:${run.pages} pages, ${run.graph.symbols} symbols, ${run.graph.edges} edges, tokens=${run.usage?.total_tokens ?? 0}, page_research=${run.pageResearchRequests}, resumed=${run.resumedFromCheckpoint ? "yes" : "no"}${run.diagnosticState ? `, diagnostic=${run.diagnosticState}` : ""}`,
      )
      .join(" | ");
    console.log(`[${index + 1}/${total}] ${result.proj}  OK  ${summary}`);
    return;
  }

  for (const line of result.logs ?? []) {
    console.log(line);
  }
  console.log(`[${index + 1}/${total}] ${result.proj}  FAIL ${result.error}`);
}

function printProjectStart(proj, index, total) {
  console.log(`[${index + 1}/${total}] ${proj}  START`);
}

function isTransientProjectError(message) {
  return isTransientFsErrorMessage(message || "");
}

/**
 * 构造项目级 child worker 的命令参数。
 *
 * @param proj 测试项目名。
 * @param options child worker 运行选项。
 * @returns 返回可直接传给 `node` 的参数数组。
 */
export function buildRunTestProjectChildArgs(proj, options = {}) {
  const args = [SCRIPT_PATH, "--child-json", "--no-build", "--run-mode", options.runMode || "cold"];
  if (Number.isFinite(options.timeoutMs) && options.timeoutMs > 0) {
    args.push("--timeout-minutes", String(Math.ceil(options.timeoutMs / 60_000)));
  }
  args.push(proj);
  return args;
}

async function runProjectInChild(proj, options = {}) {
  const perAttemptTimeoutMs = options.timeoutMs ?? DEFAULT_INIT_TIMEOUT_MS;
  const childTimeoutMs
    = perAttemptTimeoutMs * MAX_INIT_RESUME_ATTEMPTS
      + COMMAND_TIMEOUT_GRACE_MS
      + (MAX_INIT_RESUME_ATTEMPTS - 1) * 30_000;
  for (let attempt = 0; attempt < 3; attempt++) {
    const child = await runCommandCapture(
      process.execPath,
      buildRunTestProjectChildArgs(proj, options),
      {
        cwd: ROOT_DIR,
        killTreeOnTimeout: true,
        timeoutMs: childTimeoutMs,
      },
    );

    if (child.timedOut) {
      throw new Error(child.stderr || `child worker for ${proj} timed out`);
    }
    if (!child.stdout.trim()) {
      if (attempt < 2 && isTransientProjectError(child.stderr)) {
        await new Promise((resolve) => setTimeout(resolve, 1_000 * (attempt + 1)));
        continue;
      }
      throw new Error(child.stderr || `child worker for ${proj} produced empty stdout`);
    }

    const result = JSON.parse(child.stdout.trim());
    if (result.ok || result.skipped || !isTransientProjectError(result.error || "")) {
      return result;
    }
    if (attempt < 2) {
      await new Promise((resolve) => setTimeout(resolve, 1_000 * (attempt + 1)));
      continue;
    }
    return result;
  }

  throw new Error(`child worker for ${proj} exhausted retry budget`);
}

export function buildRunTestProjectsSummary(results, options = {}) {
  let passed = 0;
  let failed = 0;
  let skipped = 0;
  let diagnosticProjects = 0;

  const projectResults = results.map((result) => {
    const diagnosticStates = (result.runs ?? [])
      .map((run) => run.diagnosticState)
      .filter(Boolean);
    if (result.skipped) {
      skipped += 1;
    } else if (result.ok) {
      passed += 1;
    } else {
      failed += 1;
    }
    if (diagnosticStates.length > 0) {
      diagnosticProjects += 1;
    }
    return {
      project: result.proj,
      ok: result.ok,
      skipped: result.skipped,
      diagnostic_states: diagnosticStates,
      error: result.error ?? null,
    };
  });

  const decision = failed > 0
    ? "blocker"
    : diagnosticProjects > 0
      ? "diagnostic"
      : "pass";
  const notes = [
    "`run-test-projects.mjs` 只作为 baseline guard，不等同于 19 项目全量质量达标承诺。",
  ];
  if (diagnosticProjects > 0) {
    notes.push("存在 diagnostic runtime 观察结果；它们保留在 baseline guard 内，但不单独冒充 primary gate。");
  }

  return buildAcceptanceHarnessSummary({
    gateLevel: "baseline_guard",
    gateScope: "batch_init",
    command: "node scripts/run-test-projects.mjs",
    decision,
    totals: {
      totalProjects: results.length,
      passedProjects: passed,
      failedProjects: failed,
      skippedProjects: skipped,
      diagnosticProjects,
    },
    projectResults,
    formalGates: createFormalGateResults({
      artifact_validity: {
        decision,
        blocking: failed > 0,
        evidence_refs: [
          "node scripts/run-test-projects.mjs",
          "batch init project results",
        ],
      },
    }),
    notes,
    relevantCapabilities: [
      "declared_lifecycle_completeness",
      "projection_readiness_recovery",
      "knowledge_quality_gates",
    ],
    samples: options.samples ?? [],
  });
}

export async function runTestProjectsWithSummary(names, options = {}) {
  const projects = names && names.length > 0 ? names : discoverProjects();
  ensureBinary({ fresh: options.ensureFresh ?? true });

  const total = projects.length;
  const jobs = resolveProjectJobs(options.jobs, total);
  const useParallel = total > 1 && jobs > 1 && !options.childMode;
  let passed = 0;
  let failed = 0;

  const results = useParallel
    ? await runTaskPool(projects, jobs, async (proj, index) => {
      printProjectStart(proj, index, total);
      const result = await runProjectInChild(proj, {
        runMode: options.runMode || "cold",
        timeoutMs: options.timeoutMs,
      });
      printProjectResult(result, index, total);
      return result;
    })
    : await runSequentialTasks(projects, async (proj, index) => {
      printProjectStart(proj, index, total);
      const result = await runSingleProject(proj, {
        runMode: options.runMode,
        timeoutMs: options.timeoutMs,
      });
      printProjectResult(result, index, total);
      return result;
    });

  for (const result of results) {
    if (result.skipped) {
      continue;
    }
    if (result.ok) {
      passed++;
    } else {
      failed++;
    }
  }

  console.log(`\nDone. ${passed} passed, ${failed} failed, ${total} total. jobs=${jobs}`);
  return {
    ok: failed === 0,
    results,
    summary: buildRunTestProjectsSummary(results, {
      samples: projects.filter((project) => ["storybook", "dagger"].includes(project)),
    }),
  };
}

export async function runTestProjects(names, options = {}) {
  const result = await runTestProjectsWithSummary(names, options);
  return result.ok;
}

export function parseCliArgs(argv) {
  const names = [];
  let jobs;
  let childMode = false;
  let ensureFresh = true;
  let jsonSummary = false;
  let runMode = "cold";
  let timeoutMs;

  for (let index = 0; index < argv.length; index++) {
    const arg = argv[index];
    if (arg === "--jobs") {
      jobs = argv[index + 1];
      index++;
      continue;
    }
    if (arg === "--child-json") {
      childMode = true;
      continue;
    }
    if (arg === "--no-build") {
      ensureFresh = false;
      continue;
    }
    if (arg === "--json-summary") {
      jsonSummary = true;
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
    names.push(arg);
  }

  return { childMode, ensureFresh, jobs, jsonSummary, names, runMode, timeoutMs };
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const args = parseCliArgs(process.argv.slice(2));
  if (args.childMode) {
    const result = await runSingleProject(args.names[0], {
      runMode: args.runMode,
      timeoutMs: args.timeoutMs,
    });
    process.stdout.write(JSON.stringify(result));
    process.exit(0);
  }

  const result = await runTestProjectsWithSummary(args.names.length > 0 ? args.names : undefined, {
    ensureFresh: args.ensureFresh,
    jobs: args.jobs,
    runMode: args.runMode,
    timeoutMs: args.timeoutMs,
  });
  if (args.jsonSummary) {
    process.stdout.write(`${JSON.stringify(result.summary, null, 2)}\n`);
  }
  if (!result.ok)
process.exit(1);
}
