import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..");
const authorityPath = path.join(root, ".wiki", "06-设计文档", "06-Runtime查询合同.md");
const runtimeDesignPath = path.join(root, ".wiki", "06-设计文档", "01-Runtime设计.md");
const agentsDesignPath = path.join(root, ".wiki", "06-设计文档", "02-Agents设计.md");
const wikiIndexPath = path.join(root, ".wiki", "INDEX.md");
const authorityLink = "./06-Runtime查询合同.md";

function readText(file: string): string {
  return readFileSync(file, "utf8");
}

function collectMissingMarkers(content: string, context: string, markers: readonly string[]): string[] {
  return markers
    .filter(marker => !content.includes(marker))
    .map(marker => `${context}: missing marker ${marker}`);
}

test("runtime query authority is uniquely reachable from long-term Wiki navigation", () => {
  expect(existsSync(authorityPath), "missing canonical Runtime query authority").toBe(true);
  expect(readText(runtimeDesignPath)).toContain(authorityLink);
  expect(readText(agentsDesignPath)).toContain(authorityLink);
  expect(readText(wikiIndexPath)).toContain("./06-设计文档/06-Runtime查询合同.md");
});

test("runtime query authority defines canonical and deferred contracts", () => {
  const authority = existsSync(authorityPath) ? readText(authorityPath) : "";
  const issues = collectMissingMarkers(authority, "Runtime query authority", [
    "# Runtime 查询合同",
    "非空 `term`",
    "`route_groups` 是唯一结果 authority",
    "`runtime_state`",
    "`readiness`",
    "`query_mode`",
    "`query_trust`",
    "`recommended_action`",
    "`governance`",
    "`answer`",
    "`index_module_hit`",
    "`source_module`",
    "`ranking_basis`",
    "`score_direction`",
    "`total_count`",
    "`returned_count`",
    "`truncated`",
    "跨 route",
    "formal layer fusion",
    "`rendered_page_debug_fallback`",
    "`invalid_argument`",
    "`index_not_ready`",
    "`route_groups=[]`",
    "owner",
    "entrypoint",
    "impact",
    "process/community",
    "延期",
    "升级条件",
  ]);

  expect(issues).toEqual([]);
});

test("wiki authority stays aligned with source and host consumption contracts", () => {
  const model = readText(path.join(root, "crates", "wiki-model", "src", "domain", "query.rs"));
  const transport = readText(path.join(root, "crates", "wiki-runtime", "src", "transport", "query_payload.rs"));
  const parser = readText(path.join(root, "packages", "spec-wiki", "src", "runtime", "parseResult.ts"));
  const agents = readText(path.join(root, "packages", "spec-wiki", "src", "agents", "shared", "commandAssets.ts"));

  expect(collectMissingMarkers(model, "wiki-model query contract", [
    "IndexModuleHit",
    "SourceModule",
    "pub rank: usize",
    "pub ranking_basis: QueryRankingBasis",
    "pub score_direction: QueryScoreDirection",
    "pub total_count: usize",
    "pub returned_count: usize",
    "pub truncated: bool",
  ])).toEqual([]);

  expect(collectMissingMarkers(transport, "Runtime query transport", [
    "pub term: String",
    "pub runtime_state: String",
    "pub readiness: RuntimeReadiness",
    "pub query_mode: QueryMode",
    "pub query_trust: QueryTrust",
    "pub recommended_action: RecommendedAction",
    "pub governance: GovernanceSummary",
    "pub route_groups: Vec<QueryRouteGroup>",
    "pub answer: AnswerEnvelope",
  ])).toEqual([]);
  expect(transport).not.toMatch(/pub (?:results|matched_pages|provenance_summary|summary|hits):/);

  expect(collectMissingMarkers(parser, "TypeScript query parser", [
    "route_groups: QueryRouteGroup[]",
    "answer: AnswerEnvelope",
    "index_module_hit",
    "source_module",
    "legacy query field",
  ])).toEqual([]);
  expect(collectMissingMarkers(agents, "Agents query assets", [
    "`route_groups`",
    "`answer`",
    "only result authority",
    "without comparing scores across routes",
  ])).toEqual([]);
});
