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
  callCore,
  ensureBinary,
  ROOT_DIR,
  TEST_DIR,
  removePathWithRetry,
  resolveProjectJobs,
  runCommandCapture,
  runTaskPool,
} from "./testing/helpers.mjs";

const SCRIPT_PATH = fileURLToPath(import.meta.url);

// -------------------------------------------------------------------------
// 特殊项目：需要指向真实仓库 init 再拷贝回来
// -------------------------------------------------------------------------

const REAL_REPO_MAP = {
  aLocal: "E:\\project\\aLocal",
  "spec-wiki": ROOT_DIR,
};

function initViaRealRepo(proj, realRepo) {
  const projDir = path.join(TEST_DIR, proj);
  const wikiInReal = path.join(realRepo, ".wiki");

  // 清理真实仓库残留
  removePathWithRetry(wikiInReal);

  // init on real repo
  callCore({ action: "init", repoRoot: realRepo });

  // 拷贝结果
  if (existsSync(path.join(projDir, ".wiki"))) {
    removePathWithRetry(path.join(projDir, ".wiki"));
  }
  cpSync(wikiInReal, path.join(projDir, ".wiki"), { recursive: true });

  // 清理真实仓库
  removePathWithRetry(wikiInReal);
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

function runSingleProject(proj) {
  const projDir = path.join(TEST_DIR, proj);
  const wikiDir = path.join(projDir, ".wiki");

  if (!existsSync(projDir)) {
    return {
      proj,
      skipped: true,
      ok: true,
      message: "SKIP (not found)",
    };
  }

  removePathWithRetry(wikiDir);

  try {
    if (REAL_REPO_MAP[proj]) {
      initViaRealRepo(proj, REAL_REPO_MAP[proj]);
    } else {
      const result = callCore({ action: "init", repoRoot: `tmp/test/${proj}` });
      if (!result.ok) throw new Error(result.error || "init failed");
    }

    const pages = countPages(wikiDir);
    const graph = readGraphCounts(wikiDir);
    return {
      proj,
      ok: true,
      skipped: false,
      pages,
      graph,
    };
  } catch (error) {
    return {
      proj,
      ok: false,
      skipped: false,
      error: error instanceof Error ? error.message : String(error),
    };
  }
}

function printProjectResult(result, index, total) {
  if (result.skipped) {
    console.log(`[${index + 1}/${total}] ${result.proj} ${result.message}`);
    return;
  }

  if (result.ok) {
    console.log(
      `[${index + 1}/${total}] ${result.proj}  OK  ${result.pages} pages, ${result.graph.symbols} symbols, ${result.graph.edges} edges, ${result.graph.communities} communities, ${result.graph.processes} processes`,
    );
    return;
  }

  console.log(`[${index + 1}/${total}] ${result.proj}  FAIL ${result.error}`);
}

async function runProjectInChild(proj) {
  const child = await runCommandCapture(
    process.execPath,
    [SCRIPT_PATH, "--child-json", "--no-build", proj],
    { cwd: ROOT_DIR },
  );

  if (!child.stdout.trim()) {
    throw new Error(child.stderr || `child worker for ${proj} produced empty stdout`);
  }

  return JSON.parse(child.stdout.trim());
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
      const result = await runProjectInChild(proj);
      printProjectResult(result, index, total);
      return result;
    })
    : projects.map((proj, index) => {
      const result = runSingleProject(proj);
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
  return failed === 0;
}

function parseCliArgs(argv) {
  const names = [];
  let jobs;
  let childMode = false;
  let ensureFresh = true;

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
    names.push(arg);
  }

  return { childMode, ensureFresh, jobs, names };
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const args = parseCliArgs(process.argv.slice(2));
  if (args.childMode) {
    const result = runSingleProject(args.names[0]);
    process.stdout.write(JSON.stringify(result));
    process.exit(0);
  }

  const ok = await runTestProjects(args.names.length > 0 ? args.names : undefined, {
    ensureFresh: args.ensureFresh,
    jobs: args.jobs,
  });
  if (!ok) process.exit(1);
}
