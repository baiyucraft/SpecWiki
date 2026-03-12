// 批量对 tmp/test/* 项目执行 init，保留生成的 .wiki 目录。
// 如果项目已有 .wiki，先删除再重新生成。
// aLocal 和 spec-wiki 指向真实仓库执行 init 后拷贝回来。
//
// 用法：
//   node scripts/run-test-projects.mjs                     # 跑全部
//   node scripts/run-test-projects.mjs --jobs 6           # 调整项目并行度
//   node scripts/run-test-projects.mjs axum chi           # 只跑指定项目

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
  callCoreStreaming,
  ensureBinary,
  formatUsageSnapshot,
  ROOT_DIR,
  TEST_DIR,
  removePathWithRetry,
  resolveProjectJobs,
  runCommandCapture,
  runTaskPool,
  withTemporaryDevConfig,
} from "./testing/helpers.mjs";

const SCRIPT_PATH = fileURLToPath(import.meta.url);

// -------------------------------------------------------------------------
// 特殊项目：需要指向真实仓库 init 再拷贝回来
// -------------------------------------------------------------------------

const REAL_REPO_MAP = {
  aLocal: "E:\\project\\aLocal",
  "spec-wiki": ROOT_DIR,
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
        const shouldPrint =
          event.processed === 0
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

async function initViaRealRepo(proj, realRepo, options = {}) {
  const projDir = path.join(TEST_DIR, proj);
  const wikiInReal = path.join(realRepo, ".wiki");

  // init on real repo
  const { logger, cacheMode } = options;
  const result = await withTemporaryDevConfig(
    realRepo,
    () =>
      callCoreStreaming(
        { action: "init", repoRoot: realRepo },
        { onProgress: (event) => logger?.onProgress(event) },
      ),
    { cacheMode },
  );
  if (!result.response.ok) {
    throw new Error(result.response.error || `${proj} init failed`);
  }

  // 拷贝结果
  if (existsSync(path.join(projDir, ".wiki"))) {
    removePathWithRetry(path.join(projDir, ".wiki"));
  }
  cpSync(wikiInReal, path.join(projDir, ".wiki"), { recursive: true });
  return result.progressEvents;
}

// -------------------------------------------------------------------------
// 主流程
// -------------------------------------------------------------------------

function discoverProjects() {
  if (!existsSync(TEST_DIR)) return [];
  return readdirSync(TEST_DIR)
    .filter((d) => statSync(path.join(TEST_DIR, d)).isDirectory())
    .sort();
}

function countPages(wikiDir) {
  if (!existsSync(wikiDir)) return 0;
  let count = 0;
  const walk = (dir) => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (entry.isDirectory()) walk(path.join(dir, entry.name));
      else if (entry.name.endsWith(".md")) count++;
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
      let progressEvents;
      if (REAL_REPO_MAP[proj]) {
        progressEvents = await initViaRealRepo(proj, REAL_REPO_MAP[proj], {
          cacheMode: run.cacheMode,
          logger,
        });
      } else {
        const stream = await withTemporaryDevConfig(
          projDir,
          () =>
            callCoreStreaming(
              { action: "init", repoRoot: `tmp/test/${proj}` },
              { onProgress: (event) => logger.onProgress(event) },
            ),
          { cacheMode: run.cacheMode },
        );
        if (!stream.response.ok) {
          throw new Error(stream.response.error || `${proj} init failed`);
        }
        progressEvents = stream.progressEvents;
      }
      const pages = countPages(wikiDir);
      const graph = readGraphCounts(wikiDir);
      const usage = summarizeProgressUsage(progressEvents);
      runs.push({
        label: run.label,
        cacheMode: run.cacheMode,
        pages,
        graph,
        usage,
        pageResearchRequests: promptCount(usage, "page_research"),
        pageEnrichmentRequests: promptCount(usage, "page_enrichment"),
      });
      logger.log(
        `DONE pages=${pages} symbols=${graph.symbols} total_tokens=${usage?.total_tokens ?? 0}`,
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
        `${run.label}:${run.pages} pages, ${run.graph.symbols} symbols, ${run.graph.edges} edges, tokens=${run.usage?.total_tokens ?? 0}, page_research=${run.pageResearchRequests}`,
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
  return /EBUSY|EPERM|ENOTEMPTY|os error 32/.test(message || "");
}

async function runProjectInChild(proj, runMode) {
  for (let attempt = 0; attempt < 3; attempt++) {
    const child = await runCommandCapture(
      process.execPath,
      [SCRIPT_PATH, "--child-json", "--no-build", "--run-mode", runMode, proj],
      { cwd: ROOT_DIR },
    );

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

export async function runTestProjects(names, options = {}) {
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
      const result = await runProjectInChild(proj, options.runMode || "cold");
      printProjectResult(result, index, total);
      return result;
    })
    : await Promise.all(projects.map(async (proj, index) => {
      printProjectStart(proj, index, total);
      const result = await runSingleProject(proj, { runMode: options.runMode });
      printProjectResult(result, index, total);
      return result;
    }));

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
  return failed === 0;
}

function parseCliArgs(argv) {
  const names = [];
  let jobs;
  let childMode = false;
  let ensureFresh = true;
  let runMode = "cold";

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
    if (arg === "--run-mode") {
      runMode = argv[index + 1] || runMode;
      index++;
      continue;
    }
    names.push(arg);
  }

  return { childMode, ensureFresh, jobs, names, runMode };
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const args = parseCliArgs(process.argv.slice(2));
  if (args.childMode) {
    const result = await runSingleProject(args.names[0], { runMode: args.runMode });
    process.stdout.write(JSON.stringify(result));
    process.exit(0);
  }

  const ok = await runTestProjects(args.names.length > 0 ? args.names : undefined, {
    ensureFresh: args.ensureFresh,
    jobs: args.jobs,
    runMode: args.runMode,
  });
  if (!ok) process.exit(1);
}
