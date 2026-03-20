import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { expect, test } from "vitest";

import {
  analyzeReferenceFidelity,
  computeKeySourceCoverage,
  extractHeadingSkeleton,
  extractKeySources,
  normalizeHeading,
} from "../testing/reference-fidelity.mjs";
import { toMarkdown } from "../collect-test-project-analysis.mjs";
import { inspectWikiRuntime } from "../testing/wiki-runtime-inspection.mjs";

function writeMarkdown(root: string, relativePath: string, content: string) {
  const fullPath = path.join(root, relativePath);
  mkdirSync(path.dirname(fullPath), { recursive: true });
  writeFileSync(fullPath, content);
}

function createFixtureDir(prefix: string) {
  return mkdtempSync(path.join(os.tmpdir(), prefix));
}

test("heading normalize 忽略目录/附录/来源块，并保留稳定 skeleton", () => {
  const content = [
    "# 页面",
    "",
    "## 目录",
    "## 简介",
    "### 项目结构",
    "## 章节来源",
    "## 附录",
    "## 架构总览",
  ].join("\n");

  expect(normalizeHeading("目录")).toBeNull();
  expect(normalizeHeading("简介")).toBe("intro");
  expect(extractHeadingSkeleton(content)).toEqual(["intro", "structure", "architecture"]);
});

test("key source 提取优先 citation，没有 citation 时退回文件提及", () => {
  const cited = extractKeySources(
    "来源见 [router](file://src/router.ts) 和 [handler](file://src/handler.ts)",
  );
  const fallback = extractKeySources("核心文件：src/router.ts、docs/handler.md");

  expect(cited.mode).toBe("citation");
  expect(cited.keySources).toEqual(["src/handler.ts", "src/router.ts"]);
  expect(fallback.mode).toBe("mention_fallback");
  expect(fallback.keySources).toEqual(["docs/handler.md", "src/router.ts"]);
});

test("key source coverage 允许 basename fallback，避免路径形式差异误判为缺失", () => {
  const result = computeKeySourceCoverage(["src/router.ts", "src/handler.ts"], ["router.ts"]);

  expect(result.coverage).toBe(0.5);
  expect(result.matchedSources).toEqual(["src/router.ts"]);
  expect(result.missingSources).toEqual(["src/handler.ts"]);
});

test("many-to-one reuse 与 topical demote 会进入正式 collapse 指标", () => {
  const fixtureDir = createFixtureDir("reference-fidelity-");
  const referenceDir = path.join(fixtureDir, "reference");
  const generatedDir = path.join(fixtureDir, "generated");

  try {
    writeMarkdown(
      referenceDir,
      "专题/router-生命周期.md",
      [
        "# Router 生命周期",
        "",
        "## 简介",
        "围绕 src/router.ts 组织。",
        "",
        "## 架构总览",
        "[router](file://src/router.ts)",
      ].join("\n"),
    );
    writeMarkdown(
      referenceDir,
      "专题/router-扩展.md",
      [
        "# Router 扩展",
        "",
        "## 简介",
        "围绕 src/router.ts 与 src/plugin.ts 组织。",
        "",
        "## 架构总览",
        "[plugin](file://src/plugin.ts)",
      ].join("\n"),
    );
    writeMarkdown(
      generatedDir,
      "模块/router.md",
      [
        "# Router 模块",
        "",
        "## 简介",
        "src/router.ts 与 src/plugin.ts 汇总页。",
      ].join("\n"),
    );

    const result = analyzeReferenceFidelity({ referenceDir, generatedDir, matchThreshold: 20 });

    expect(result.matchedCount).toBe(2);
    expect(result.reusePages).toBe(1);
    expect(result.severeReusePages).toBe(0);
    expect(result.reuseOverage).toBe(1);
    expect(result.collapsedPages).toBe(2);
    expect(result.topReuseOffenders).toEqual([
      {
        generatedPath: "模块/router.md",
        generatedTitle: "Router 模块",
        count: 2,
      },
    ]);
    expect(result.comparisons.every((item) => item.reuseCount === 2)).toBe(true);
    expect(
      result.comparisons.every((item) => item.notes.includes("reference 专题被折叠进非专题页")),
    ).toBe(true);
  } finally {
    rmSync(fixtureDir, { recursive: true, force: true });
  }
});

test("runtime inspection 会把仅有 research/cache 的样本判定为 runtime_incomplete", () => {
  const fixtureDir = createFixtureDir("wiki-runtime-");
  const wikiDir = path.join(fixtureDir, ".wiki");
  const cacheDir = path.join(wikiDir, ".cache");
  const dbPath = path.join(cacheDir, "wiki-cache.db");

  try {
    mkdirSync(cacheDir, { recursive: true });
    execFileSync("sqlite3", [
      dbPath,
      [
        "create table knowledge_units(id text);",
        "create table knowledge_domains(id text);",
        "create table research_cache(research_type text, target_id text);",
        "create table page_digests(unit_id text);",
        "create table page_drafts(unit_id text);",
        "create table wiki_pages(page_id text);",
        "create table pipeline_checkpoint(checkpoint_id text, interrupted_stage text, interrupted_target_id text, error_message text, created_at text default current_timestamp);",
        "insert into knowledge_units values ('unit-1');",
        "insert into knowledge_domains values ('domain-1');",
        "insert into research_cache values ('unit', 'unit-1');",
      ].join(" "),
    ]);

    const snapshot = inspectWikiRuntime(wikiDir);

    expect(snapshot.runtimeState).toBe("runtime_incomplete");
    expect(snapshot.incompleteReason).toContain("已有 knowledge/research 数据");
    expect(snapshot.baselineClass).toBe("diagnostic_only");
    expect(snapshot.dbCounts.knowledge_units).toBe(1);
    expect(snapshot.dbCounts.research_cache).toBe(1);
    expect(snapshot.dbCounts.wiki_pages).toBe(0);
    expect(snapshot.markdownPageCount).toBe(0);
  } finally {
    rmSync(fixtureDir, { recursive: true, force: true });
  }
});

test("runtime inspection 会优先解释 researching / compose_pending / compose_complete / interrupted", { timeout: 60000 }, () => {
  const fixtureDir = createFixtureDir("wiki-runtime-state-");
  const wikiDir = path.join(fixtureDir, ".wiki");
  const cacheDir = path.join(wikiDir, ".cache");
  const dbPath = path.join(cacheDir, "wiki-cache.db");

  try {
    mkdirSync(cacheDir, { recursive: true });
    execFileSync("sqlite3", [
      dbPath,
      [
        "create table runtime_meta(key text primary key, value text not null);",
        "create table knowledge_units(id text);",
        "create table knowledge_domains(id text);",
        "create table research_cache(research_type text, target_id text);",
        "create table page_digests(unit_id text);",
        "create table page_drafts(unit_id text);",
        "create table wiki_pages(page_id text);",
        "create table pipeline_checkpoint(checkpoint_id text, interrupted_stage text, interrupted_target_id text, error_message text, created_at text default current_timestamp);",
        "insert into knowledge_units values ('unit-1');",
        "insert into research_cache values ('unit', 'unit-1');",
      ].join(" "),
    ]);

    execFileSync("sqlite3", [
      dbPath,
      "insert into runtime_meta values ('pipeline_runtime_summary', '{\"runtime_state\":\"researching\",\"composed_units\":0}');",
    ]);
    let snapshot = inspectWikiRuntime(wikiDir);
    expect(snapshot.incompleteReason).toContain("仍在 research");
    expect(snapshot.researchProgressSummary.currentResearchUnit).toBeNull();
    expect(snapshot.researchProgressSummary.lastCompletedResearch).toEqual({ unitId: "unit-1", elapsedMs: null });
    expect(snapshot.researchProgressSummary.providerStats).toBeNull();

    execFileSync("sqlite3", [
      dbPath,
      "update runtime_meta set value='{\"runtime_state\":\"compose_pending\",\"composed_units\":0}' where key='pipeline_runtime_summary';",
    ]);
    snapshot = inspectWikiRuntime(wikiDir);
    expect(snapshot.incompleteReason).toContain("compose 尚未完成");

    execFileSync("sqlite3", [
      dbPath,
      "update runtime_meta set value='{\"runtime_state\":\"compose_complete\",\"composed_units\":3}' where key='pipeline_runtime_summary';",
    ]);
    snapshot = inspectWikiRuntime(wikiDir);
    expect(snapshot.incompleteReason).toContain("compose 已完成");

    execFileSync("sqlite3", [
      dbPath,
      "update runtime_meta set value='{\"runtime_state\":\"interrupted\",\"last_interrupted_stage\":\"research_unit\"}' where key='pipeline_runtime_summary';",
    ]);
    snapshot = inspectWikiRuntime(wikiDir);
    expect(snapshot.incompleteReason).toContain("research 阶段中断");

    execFileSync("sqlite3", [
      dbPath,
      "update runtime_meta set value='{\"runtime_state\":\"interrupted\",\"last_interrupted_stage\":\"compose_parent\"}' where key='pipeline_runtime_summary';",
    ]);
    snapshot = inspectWikiRuntime(wikiDir);
    expect(snapshot.incompleteReason).toContain("compose 阶段中断");
  } finally {
    rmSync(fixtureDir, { recursive: true, force: true });
  }
});


test("runtime inspection 会从 runtime/research JSON 提取最小 research 摘要", () => {
  const fixtureDir = createFixtureDir("wiki-runtime-research-summary-");
  const wikiDir = path.join(fixtureDir, ".wiki");
  const cacheDir = path.join(wikiDir, ".cache");
  const dbPath = path.join(cacheDir, "wiki-cache.db");

  try {
    mkdirSync(cacheDir, { recursive: true });
    execFileSync("sqlite3", [
      dbPath,
      [
        "create table runtime_meta(key text primary key, value text not null);",
        "create table knowledge_units(id text);",
        "create table knowledge_domains(id text);",
        "create table research_cache(research_type text, target_id text, input_hash text, result text, model text, created_at text, ttl_seconds integer);",
        "create table page_digests(unit_id text);",
        "create table page_drafts(unit_id text);",
        "create table unit_runtime_gates(unit_id text, unit_type text, research_status text, compose_status text, assemble_status text, last_ready_stage text, blocked_reason text, missing_dependencies text);",
        "create table wiki_pages(page_id text);",
        "create table pipeline_checkpoint(checkpoint_id text, interrupted_stage text, interrupted_target_id text, error_message text, created_at text default current_timestamp);",
        "insert into knowledge_units values ('unit-current');",
        "insert into knowledge_units values ('unit-done');",
        "insert into research_cache values ('unit', 'unit-done', 'hash-1', '{\"provider_stop_reason\":\"completed\",\"provider_session_stats\":{\"turns_used\":3,\"tool_calls\":2,\"elapsed_ms\":4800,\"cache_hit\":false,\"tools_mode\":\"required\"}}', null, '2026-03-19 10:00:00', 604800);",
        "insert into runtime_meta values ('pipeline_runtime_summary', '{\"runtime_state\":\"researching\",\"current_research_unit_id\":\"unit-current\",\"current_research_unit_type\":\"module-doc\",\"current_research_started_at\":\"2026-03-19T10:01:02Z\",\"last_researched_unit_id\":\"unit-done\",\"last_research_elapsed_ms\":4800}');",
      ].join(" "),
    ]);

    const snapshot = inspectWikiRuntime(wikiDir);

    expect(snapshot.incompleteReason).toContain("仍在 research");
    expect(snapshot.researchProgressSummary).toEqual({
      currentResearchUnit: {
        unitId: "unit-current",
        unitType: "module-doc",
        startedAt: "2026-03-19T10:01:02Z",
      },
      lastCompletedResearch: {
        unitId: "unit-done",
        elapsedMs: 4800,
      },
      providerStats: {
        unitId: "unit-done",
        stopReason: "completed",
        turnsUsed: 3,
        toolCalls: 2,
        elapsedMs: 4800,
        cacheHit: false,
        toolsMode: "required",
        retryInputApplied: null,
      },
    });
  } finally {
    rmSync(fixtureDir, { recursive: true, force: true });
  }
});

test("test project analysis Markdown 会输出 research runtime 摘要且保留旧阶段文案", () => {
  const markdown = toMarkdown([
    {
      project: "storybook",
      runtimeState: "runtime_incomplete",
      baselineClass: "diagnostic_only",
      incompleteReason: "workflow 仍在 research，尚未进入 compose",
      runtimeSnapshot: {
        markdownPageCount: 0,
      },
      runtime: {
        runtimeSummary: {
          runtime_state: "researching",
        },
        gateSummary: {
          total: 1,
          composeReady: 0,
          composeBlocked: 0,
          assembleDone: 0,
        },
        researchProgressSummary: {
          currentResearchUnit: {
            unitId: "unit-current",
            unitType: "module-doc",
            startedAt: "2026-03-19T10:01:02Z",
          },
          lastCompletedResearch: {
            unitId: "unit-done",
            elapsedMs: 4800,
          },
          providerStats: {
            unitId: "unit-done",
            stopReason: "completed",
            turnsUsed: 3,
            toolCalls: 2,
            elapsedMs: 4800,
            cacheHit: false,
            toolsMode: "required",
          },
        },
        parentContract: {
          parentPages: 0,
          composeReadyParents: 0,
          childDigestParents: 0,
          missingReadinessParents: 0,
        },
      },
      knowledgeMetrics: {
        knowledgeUnitCount: 1,
        knowledgeDomainCount: 1,
        unitResearchRows: 1,
        sectionPlanUnits: 0,
        pageDraftCount: 0,
        wikiPageCount: 0,
        markdownPageCount: 0,
        researchCoverage: 1,
        composeCoverage: 0,
        assembleCoverage: 0,
        unitTypeCounts: [],
        domainTypeCounts: [],
        researchTypeCounts: [],
        wikiPageTypeCounts: [],
      },
      pageMetrics: {
        topicPages: 0,
        evidenceLandingPages: 0,
        mermaidLandingPages: 0,
        avgNonEmptyLinesPerPage: 0,
        avgProseLinesPerPage: 0,
        decompositionCounts: [],
      },
      symbolCount: 0,
      edgeCount: 0,
      communityCount: 0,
      processCount: 0,
      languageBreakdown: [],
      reference: {
        hasReference: false,
        pageCount: 0,
      },
    },
  ]);

  expect(markdown).toContain("- runtime：runtime_incomplete / diagnostic_only / workflow 仍在 research，尚未进入 compose");
  expect(markdown).toContain("- research runtime：current=unit-current, type=module-doc, since=2026-03-19T10:01:02Z，last_elapsed=4800ms，provider=unit=unit-done, stop=completed, turns=3, tools=2, elapsed=4800ms, cache_hit=false, mode=required");
});



