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

