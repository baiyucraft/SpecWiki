// wiki-core 生命周期测试脚本。
// 现在支持按阶段拆分运行，避免每次把 init → sync → query → update → rebuild 全链路一次跑完。
//
// 用法：
//   node scripts/test-wiki-lifecycle.mjs --list-phases
//   node scripts/test-wiki-lifecycle.mjs --phase bootstrap
//   node scripts/test-wiki-lifecycle.mjs --phase steady axum zustand
//   node scripts/test-wiki-lifecycle.mjs --phase mutation
//   node scripts/test-wiki-lifecycle.mjs --phase rebuild
//   node scripts/test-wiki-lifecycle.mjs                    # 默认 full

import {
  appendFileSync,
  cpSync,
  existsSync,
  readdirSync,
  readFileSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { execFileSync } from "node:child_process";
import path from "node:path";
import { pathToFileURL } from "node:url";

import {
  callCore,
  ensureBinary,
  ROOT_DIR,
  TEST_DIR,
  TestRunner,
} from "./testing/helpers.mjs";

const REAL_REPO_MAP = {
  aLocal: "E:\\project\\aLocal",
  "spec-wiki": ROOT_DIR,
};

export const LIFECYCLE_PHASES = {
  full: "完整链路：init → status → sync → query → update → touch/update → rebuild → status",
  bootstrap: "初始化链路：init → status",
  steady: "稳定态链路：init → sync → query → update(no-op)",
  mutation: "变更链路：init → touch source → status → update",
  rebuild: "重建链路：init → rebuild → status",
};

function initViaRealRepo(proj, realRepo) {
  const projDir = path.join(TEST_DIR, proj);
  const wikiInReal = path.join(realRepo, ".wiki");

  if (existsSync(wikiInReal)) rmSync(wikiInReal, { recursive: true });

  callCore({ action: "init", repoRoot: realRepo });

  const wikiDest = path.join(projDir, ".wiki");
  if (existsSync(wikiDest)) rmSync(wikiDest, { recursive: true });
  cpSync(wikiInReal, wikiDest, { recursive: true });

  rmSync(wikiInReal, { recursive: true });
}

function discoverProjects() {
  if (!existsSync(TEST_DIR)) return [];
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
  if (!existsSync(dbPath)) return 0;
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
  if (!existsSync(dbPath)) return "";
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
  if (!existsSync(dbPath)) return "";
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

  const queryResult = callCore({
    action: "query",
    repoRoot: ctx.repoArg,
    term: probe.term,
  });
  t.assertOk(`${label} symbol query returns ok`, queryResult);

  const exactMatch = queryResult.data?.matched_symbols?.some((symbol) => symbol.name === probe.term);
  exactMatch
    ? t.pass(`${label} exact symbol hit present`)
    : t.fail(`${label} exact symbol hit present`, `missing ${probe.term} in matched_symbols`);
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

  const queryResult = callCore({
    action: "query",
    repoRoot: ctx.repoArg,
    term: probe.term,
  });
  t.assertOk(`${label} graph query returns ok`, queryResult);

  const matchedGraphEdges = queryResult.data?.matched_symbol_edges?.length ?? 0;
  const matchedProcesses = queryResult.data?.matched_processes?.length ?? 0;
  const matchedCommunities = queryResult.data?.matched_communities?.length ?? 0;
  const matchedGraphTotal = matchedGraphEdges + matchedProcesses + matchedCommunities;

  matchedGraphTotal > 0
    ? t.pass(`${label} graph query returns graph context`)
    : t.fail(
      `${label} graph query returns graph context`,
      `missing graph matches for ${probe.term}`,
    );

  queryResult.data?.provenance_summary?.includes("扩展")
    ? t.pass(`${label} graph provenance summary recorded`)
    : t.fail(
      `${label} graph provenance summary recorded`,
      `summary=${JSON.stringify(queryResult.data?.provenance_summary ?? "")}`,
    );
}

function findSourceFile(projDir) {
  const walk = (dir, depth) => {
    if (depth > 6) return null;
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = path.join(dir, entry.name);
      if (entry.isDirectory() && !entry.name.startsWith(".")) {
        const found = walk(full, depth + 1);
        if (found) return found;
      } else if (SOURCE_EXTS.has(path.extname(entry.name))) {
        return full;
      }
    }
    return null;
  };
  return walk(projDir, 0);
}

function findTrackedSourceFile(ctx) {
  const dbPath = path.join(ctx.wikiDir, ".cache", "wiki-cache.db");
  if (existsSync(dbPath)) {
    const trackedPaths = querySqliteRows(
      dbPath,
      "select path from source_states order by sort_order limit 32;",
    );
    for (const trackedPath of trackedPaths) {
      const full = path.join(ctx.projDir, trackedPath);
      if (existsSync(full)) {
        return full;
      }
    }
  }

  return findSourceFile(ctx.projDir);
}

function initProject(ctx, t) {
  const { proj, projDir, wikiDir, repoArg, isRealRepo } = ctx;

  console.log("  [init]");
  if (existsSync(wikiDir)) rmSync(wikiDir, { recursive: true });

  try {
    if (isRealRepo) {
      initViaRealRepo(proj, REAL_REPO_MAP[proj]);
      t.assertFileExists(".wiki directory created", wikiDir);
    } else {
      const result = callCore({ action: "init", repoRoot: repoArg });
      t.assertOk("init returns ok", result);
      t.assertFileExists(".wiki directory created", wikiDir);
    }
  } catch (error) {
    t.fail("init", error.message);
    return false;
  }

  t.assertMarkerCoverage("all pages have managed markers", wikiDir);
  t.assertFileExists("wiki.metadata.json exists", path.join(wikiDir, "wiki.metadata.json"));
  return true;
}

function runStatusAfterInit(ctx, t) {
  console.log("  [status after init]");
  const statusAfterInit = callCore({ action: "status", repoRoot: ctx.repoArg });
  t.assertOk("status returns ok", statusAfterInit);
  if (!ctx.isRealRepo) {
    t.assertContains("state is fresh", statusAfterInit.data, "state", "fresh");
  }
}

function runSymbolSnapshotAfterInit(ctx, t) {
  console.log("  [symbols after init]");
  ctx.symbolProbe = captureSymbolProbe(ctx, t, "after init");
  ctx.initialSymbolCount = assertSymbolSnapshot(ctx, t, "after init");
  ctx.graphProbe = captureGraphProbe(ctx, t, "after init");
  ctx.initialGraphCounts = assertGraphSnapshot(ctx, t, "after init");
}

function runSyncNoChange(ctx, t) {
  console.log("  [sync no-change]");
  if (!ctx.isRealRepo) {
    const syncResult = callCore({ action: "sync", repoRoot: ctx.repoArg });
    t.assertOk("sync returns ok", syncResult);
  } else {
    t.skip("sync skipped (real-repo project, .wiki not at repoRoot)");
  }
}

function runQuery(ctx, t) {
  console.log("  [query]");
  assertSymbolQuery(ctx, t, ctx.symbolProbe || captureSymbolProbe(ctx, t, "query"), "steady");
  assertGraphQuery(ctx, t, ctx.graphProbe || captureGraphProbe(ctx, t, "query"), "steady");
}

function runUpdateNoop(ctx, t) {
  console.log("  [update no-op]");
  const updateNoop = callCore({ action: "update", repoRoot: ctx.repoArg });
  t.assertOk("update returns ok", updateNoop);
  t.assertContains("state is fresh", updateNoop.data, "state", "fresh");
  assertSymbolSnapshot(ctx, t, "after no-op update", ctx.initialSymbolCount);
  assertGraphSnapshot(ctx, t, "after no-op update", ctx.initialGraphCounts);
  assertSymbolQuery(ctx, t, ctx.symbolProbe, "after no-op update");
  assertGraphQuery(ctx, t, ctx.graphProbe, "after no-op update");
}

function runMutation(ctx, t) {
  console.log("  [simulate source change]");
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
    const statusTouch = callCore({ action: "status", repoRoot: ctx.repoArg });
    t.assertOk("status after touch returns ok", statusTouch);
    t.assertContains("state is stale after touch", statusTouch.data, "state", "stale");

    const updateTouch = callCore({ action: "update", repoRoot: ctx.repoArg });
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

function runRebuild(ctx, t) {
  console.log("  [rebuild]");
  const rebuildResult = callCore({ action: "rebuild", repoRoot: ctx.repoArg });
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
  const statusFinal = callCore({ action: "status", repoRoot: ctx.repoArg });
  t.assertOk("status returns ok", statusFinal);
  t.assertContains("state is fresh", statusFinal.data, "state", "fresh");
}

function runProjectPhase(ctx, phase, t) {
  const initialized = initProject(ctx, t);
  if (!initialized) {
    console.log("");
    return;
  }

  if (phase === "bootstrap") {
    runStatusAfterInit(ctx, t);
    runSymbolSnapshotAfterInit(ctx, t);
    console.log("");
    return;
  }

  if (phase === "steady") {
    runSymbolSnapshotAfterInit(ctx, t);
    runSyncNoChange(ctx, t);
    runQuery(ctx, t);
    runUpdateNoop(ctx, t);
    console.log("");
    return;
  }

  if (phase === "mutation") {
    runSymbolSnapshotAfterInit(ctx, t);
    runMutation(ctx, t);
    console.log("");
    return;
  }

  if (phase === "rebuild") {
    runSymbolSnapshotAfterInit(ctx, t);
    runRebuild(ctx, t);
    runStatusAfterRebuild(ctx, t);
    console.log("");
    return;
  }

  runStatusAfterInit(ctx, t);
  runSymbolSnapshotAfterInit(ctx, t);
  runSyncNoChange(ctx, t);
  runQuery(ctx, t);
  runUpdateNoop(ctx, t);
  runMutation(ctx, t);
  runRebuild(ctx, t);
  runStatusAfterRebuild(ctx, t);
  console.log("");
}

export function runLifecycleTests(names, options = {}) {
  ensureBinary({ fresh: true });

  const phase = options.phase || "full";
  if (!(phase in LIFECYCLE_PHASES)) {
    throw new Error(`unknown lifecycle phase: ${phase}`);
  }

  const projects = names && names.length > 0 ? names : discoverProjects();
  const total = projects.length;
  const t = new TestRunner();

  for (let index = 0; index < total; index++) {
    const proj = projects[index];
    const projDir = path.join(TEST_DIR, proj);
    const wikiDir = path.join(projDir, ".wiki");
    const repoArg = REAL_REPO_MAP[proj] || `tmp/test/${proj}`;
    const isRealRepo = !!REAL_REPO_MAP[proj];

    if (!existsSync(projDir)) {
      console.log(`[${index + 1}/${total}] SKIP ${proj} (not found)`);
      continue;
    }

    console.log(`[${index + 1}/${total}] ${proj} [phase=${phase}]`);
    runProjectPhase({ isRealRepo, phase, proj, projDir, repoArg, wikiDir }, phase, t);
  }

  return t.summary();
}

function printPhaseList() {
  console.log("Available lifecycle phases:");
  for (const [phase, description] of Object.entries(LIFECYCLE_PHASES)) {
    console.log(`  ${phase.padEnd(10)} ${description}`);
  }
}

function parseCliArgs(argv) {
  const names = [];
  let phase = "full";
  let listPhases = false;

  for (let index = 0; index < argv.length; index++) {
    const arg = argv[index];
    if (arg === "--phase") {
      phase = argv[index + 1] || phase;
      index++;
      continue;
    }
    if (arg === "--list-phases") {
      listPhases = true;
      continue;
    }
    names.push(arg);
  }

  return { listPhases, names, phase };
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const { listPhases, names, phase } = parseCliArgs(process.argv.slice(2));
  if (listPhases) {
    printPhaseList();
    process.exit(0);
  }

  const ok = runLifecycleTests(names.length > 0 ? names : undefined, { phase });
  if (!ok) process.exit(1);
}
