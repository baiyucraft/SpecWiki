import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { expect, test } from "vitest";

import { computeKeySourceCoverage } from "../testing/reference-fidelity.mjs";
import { inspectWikiRuntime } from "../testing/wiki-runtime-inspection.mjs";

function createFixtureDir(prefix: string) {
  return mkdtempSync(path.join(os.tmpdir(), prefix));
}

test("key source coverage 会把同一文件的不同行号引用归一后再比较", () => {
  const result = computeKeySourceCoverage(
    ["src/router.ts#L10-L40", "src/handler.ts#L1-L12"],
    ["src/router.ts#L1-L8"],
  );

  expect(result.coverage).toBe(0.5);
  expect(result.matchedSources).toEqual(["src/router.ts#L10-L40"]);
  expect(result.missingSources).toEqual(["src/handler.ts#L1-L12"]);
});

test("runtime inspection 计算 grounding gap 时会忽略同一文件的不同行号锚点", () => {
  const fixtureDir = createFixtureDir("wiki-runtime-anchored-grounding-");
  const wikiDir = path.join(fixtureDir, ".wiki");
  const cacheDir = path.join(wikiDir, ".cache");
  const dbPath = path.join(cacheDir, "wiki-cache.db");

  try {
    mkdirSync(cacheDir, { recursive: true });
    writeFileSync(
      path.join(wikiDir, "wiki.metadata.json"),
      JSON.stringify({
        wiki_items: [
          {
            id: "page-runtime",
            path: ".wiki/核心模块/运行时.md",
            title: "运行时",
          },
        ],
      }),
    );
    execFileSync("sqlite3", [
      dbPath,
      [
        "create table page_context_cache(page_id text primary key, input_hash text, context text);",
        "create table page_digests(unit_id text primary key, digest text, content_hash text);",
        "insert into page_context_cache values ('page-runtime', 'hash-1', '{\"page_id\":\"page-runtime\",\"page_type\":\"module\",\"unit_id\":\"unit-runtime\",\"unit_type\":\"module-doc\",\"domain_id\":\"domain-runtime\",\"readiness_status\":\"compose_ready\",\"child_digest_ids\":[]}');",
        "insert into page_digests values ('unit-runtime', '{\"unit_id\":\"unit-runtime\",\"page_id\":\"page-runtime\",\"title\":\"运行时\",\"planned_key_sources\":[\"src/runtime.ts\"],\"grounded_key_sources\":[\"src/runtime.ts#L10-L20\"],\"section_grounding_refs\":[{\"section_key\":\"runtime-flow\",\"key_source_cluster_keys\":[\"runtime-entry\"],\"evidence_cluster_keys\":[\"cluster-a\"],\"child_digest_refs\":[],\"diagram_refs\":[]}],\"readiness_stage\":\"compose_ready\"}', 'hash-digest');",
      ].join(" "),
    ]);

    const snapshot = inspectWikiRuntime(wikiDir);
    const binding = snapshot.pageRuntimeByPath["核心模块/运行时.md"];

    expect(snapshot.composeDiagnostics.pagesWithGroundingGap).toBe(0);
    expect(binding.groundingGap).toBe(false);
    expect(binding.missingGroundedSources).toEqual([]);
  } finally {
    rmSync(fixtureDir, { recursive: true, force: true });
  }
});
