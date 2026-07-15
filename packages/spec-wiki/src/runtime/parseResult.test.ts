import { expect, test } from "vitest";

import { parseResult } from "./parseResult.js";

function archiveResponse(overrides: Record<string, unknown> = {}) {
  return {
    ok: true,
    data: {
      governance: {
        readiness: "ready",
        fingerprint: "governance-1",
        active_count: 1,
        archived_count: 0,
        issues: [],
        recommended_action: "none",
      },
      validation: { valid: true, readiness: "ready", rule_results: [], issues: [] },
      manifest: {
        schema_version: "archive-operation/v1",
        policy_version: "archive-policy/v1",
        algorithm_version: "archive-algorithm/v1",
        operation_id: "archive-op-7",
        change_id: "child-change",
        mode: "dry_run",
        outcome: "ready",
        status: "planned",
        step: "prepared",
        step_status: "completed",
        source_path: ".spec/changes/child-change",
        target_path: ".spec/archive/2026-07-13-child-change",
        operation_root: null,
        created_at: "2026-07-13T00:00:00Z",
        persisted: false,
        resumable: false,
        validation: { valid: true, readiness: "ready", rule_results: [], issues: [] },
        artifact_hash_summary: [],
        parent_diff: null,
        precondition_digest: "blake3:abc",
        completed_steps: ["prepared"],
        failure_step: null,
        recovery_hint: null,
        wiki_sync_issues: [],
        evidence_refs: [],
        ...overrides,
      },
    },
  };
}

function canonicalQueryData(overrides: Record<string, unknown> = {}) {
  return {
    term: "payment flow",
    runtime_state: "ready",
    readiness: {
      index: "ready",
      knowledge: "ready",
      projection: "ready",
      fusion: "ready",
      restored_level: "level2",
      snapshot_id: "snapshot-1",
      reasons: [],
    },
    query_mode: "mixed",
    query_trust: "ready",
    recommended_action: "none",
    governance: {
      readiness: "not_enabled",
      active_count: 0,
      archived_count: 0,
      issues: [],
      recommended_action: "none",
    },
    route_groups: [
      {
        route_tag: "index_module_hit",
        ranking_basis: "structural_match",
        score_direction: "none",
        total_count: 1,
        returned_count: 1,
        truncated: false,
        results: [
          {
            route_tag: "index_module_hit",
            ref_kind: "source_module",
            ref_id: "module:payments",
            label: "payments",
            rank: 1,
            provenance: {
              layer: "index",
              state: "ready",
              reason: "module name matched",
            },
            confidence: "high",
            recommended_action: "open_source_ref",
            source_refs: [
              {
                ref_kind: "source_path",
                ref_id: "packages/payments",
                path: "packages/payments",
              },
            ],
          },
        ],
      },
    ],
    answer: {
      text: "Inspect the payments module.",
      answer_mode: "direct",
      answer_trust: "grounded",
      recommended_action: "none",
      provenance: ["index_hit"],
      supporting_refs: [
        {
          ref_kind: "source_module",
          ref_id: "module:payments",
          label: "payments",
          provenance: ["index_hit"],
        },
      ],
    },
    ...overrides,
  };
}

function queryResponse(overrides: Record<string, unknown> = {}) {
  return JSON.stringify({ ok: true, data: canonicalQueryData(overrides) });
}

test("parseResult validates and preserves archive reports", () => {
  const parsed = parseResult(JSON.stringify(archiveResponse()));
  expect(parsed.ok).toBe(true);
  expect(parsed.data).toMatchObject({
    manifest: {
      operation_id: "archive-op-7",
      mode: "dry_run",
      outcome: "ready",
      persisted: false,
      resumable: false,
    },
  });
});

test.each([
  ["mode", "unknown"],
  ["outcome", "unknown"],
  ["status", "unknown"],
  ["step", "unknown"],
  ["step_status", "unknown"],
] as const)("parseResult rejects unknown archive %s", (field, value) => {
  expect(() => parseResult(JSON.stringify(archiveResponse({ [field]: value })))).toThrow(field);
});

test("query parser requires canonical route groups and answer", () => {
  const parsed = parseResult(queryResponse());

  expect(parsed.data).toMatchObject({
    route_groups: [{
      route_tag: "index_module_hit",
      ranking_basis: "structural_match",
      score_direction: "none",
      total_count: 1,
      returned_count: 1,
      truncated: false,
      results: [{ rank: 1, ref_kind: "source_module" }],
    }],
    answer: {
      answer_mode: "direct",
      answer_trust: "grounded",
    },
  });
  expect(() => parseResult(queryResponse({ route_groups: undefined }))).toThrow("route_groups");
  expect(() => parseResult(queryResponse({ answer: undefined }))).toThrow("answer");
});

test.each([
  ["route tag", { route_groups: [{ ...canonicalQueryData().route_groups[0], route_tag: "unknown_route" }] }],
  ["ref kind", { route_groups: [{ ...canonicalQueryData().route_groups[0], results: [{ ...canonicalQueryData().route_groups[0].results[0], ref_kind: "owner" }] }] }],
  ["provenance layer", { route_groups: [{ ...canonicalQueryData().route_groups[0], results: [{ ...canonicalQueryData().route_groups[0].results[0], provenance: { layer: "page", state: "ready" } }] }] }],
  ["provenance state", { route_groups: [{ ...canonicalQueryData().route_groups[0], results: [{ ...canonicalQueryData().route_groups[0].results[0], provenance: { layer: "index", state: "derived" } }] }] }],
  ["ranking basis", { route_groups: [{ ...canonicalQueryData().route_groups[0], ranking_basis: "global_score" }] }],
  ["score direction", { route_groups: [{ ...canonicalQueryData().route_groups[0], score_direction: "descending" }] }],
  ["answer mode", { answer: { ...canonicalQueryData().answer, answer_mode: "partial" } }],
  ["answer trust", { answer: { ...canonicalQueryData().answer, answer_trust: "ready" } }],
] as const)("query parser rejects unknown %s", (_label, override) => {
  expect(() => parseResult(queryResponse(override))).toThrow();
});

test("query parser rejects invalid ranking counts and legacy result views", () => {
  const group = canonicalQueryData().route_groups[0];
  expect(() => parseResult(queryResponse({
    route_groups: [{ ...group, returned_count: 2 }],
  }))).toThrow("returned_count");
  expect(() => parseResult(queryResponse({
    route_groups: [{ ...group, results: [{ ...group.results[0], rank: 0 }] }],
  }))).toThrow("rank");

  for (const field of ["results", "matched_pages", "provenance_summary", "summary", "hits"]) {
    expect(() => parseResult(queryResponse({ [field]: [] })), field).toThrow("legacy");
  }
});

test("parseResult accepts typed index_not_ready recovery data", () => {
  const parsed = parseResult(JSON.stringify({
    ok: false,
    error: "index not ready: facts snapshot missing",
    errorKind: "index_not_ready",
    data: {
      reason: "facts_snapshot_missing",
      recommended_action: "init",
    },
  }));

  expect(parsed).toMatchObject({
    ok: false,
    errorKind: "index_not_ready",
    data: { recommended_action: "init" },
  });
});
