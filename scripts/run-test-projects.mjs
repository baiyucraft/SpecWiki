// 批量对 tmp/test/* 项目执行 init，保留生成的 .wiki 目录。
// 如果项目已有 .wiki，先删除再重新生成。
// aLocal 和 spec-wiki 指向真实仓库执行 init 后拷贝回来。
//
// 用法：
//   node scripts/run-test-projects.mjs              # 跑全部
//   node scripts/run-test-projects.mjs axum chi      # 只跑指定项目

import { execFileSync } from "node:child_process";
import {
  cpSync,
  existsSync,
  mkdirSync,
  readdirSync,
  rmSync,
  statSync,
} from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

import { callCore, ensureBinary, ROOT_DIR, TEST_DIR, TMP_DIR } from "./testing/helpers.mjs";

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
  if (existsSync(wikiInReal)) rmSync(wikiInReal, { recursive: true });

  // init on real repo
  callCore({ action: "init", repoRoot: realRepo });

  // 拷贝结果
  if (existsSync(path.join(projDir, ".wiki"))) {
    rmSync(path.join(projDir, ".wiki"), { recursive: true });
  }
  cpSync(wikiInReal, path.join(projDir, ".wiki"), { recursive: true });

  // 清理真实仓库
  rmSync(wikiInReal, { recursive: true });
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

export function runTestProjects(names) {
  ensureBinary({ fresh: true });

  const projects =
    names && names.length > 0
      ? names
      : discoverProjects();

  const total = projects.length;
  let passed = 0;
  let failed = 0;

  for (let i = 0; i < total; i++) {
    const proj = projects[i];
    const projDir = path.join(TEST_DIR, proj);
    const wikiDir = path.join(projDir, ".wiki");

    if (!existsSync(projDir)) {
      console.log(`[${i + 1}/${total}] SKIP ${proj} (not found)`);
      continue;
    }

    process.stdout.write(`[${i + 1}/${total}] ${proj}`);

    // 清理已有 .wiki
    if (existsSync(wikiDir)) rmSync(wikiDir, { recursive: true });

    try {
      if (REAL_REPO_MAP[proj]) {
        initViaRealRepo(proj, REAL_REPO_MAP[proj]);
      } else {
        const result = callCore({ action: "init", repoRoot: `tmp/test/${proj}` });
        if (!result.ok) throw new Error(result.error || "init failed");
      }

      const pages = countPages(wikiDir);
      const graph = readGraphCounts(wikiDir);
      console.log(
        `  OK  ${pages} pages, ${graph.symbols} symbols, ${graph.edges} edges, ${graph.communities} communities, ${graph.processes} processes`,
      );
      passed++;
    } catch (e) {
      console.log(`  FAIL ${e.message}`);
      failed++;
    }
  }

  console.log(`\nDone. ${passed} passed, ${failed} failed, ${total} total.`);
  return failed === 0;
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const args = process.argv.slice(2);
  const ok = runTestProjects(args.length > 0 ? args : undefined);
  if (!ok) process.exit(1);
}
