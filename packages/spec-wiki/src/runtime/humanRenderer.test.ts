import { expect, test } from "vitest";

import { renderHumanResponse } from "./humanRenderer.js";

test("human renderer relays DTO readiness and recommended action", () => {
  const text = renderHumanResponse({ ok: true, data: { state: "stale", recommended_action: "update" } }, { action: "status" });
  expect(text).toContain("state: stale");
  expect(text).toContain("recommended_action: update");
});

test("human renderer relays archive operation and recovery fields", () => {
  const text = renderHumanResponse({
    ok: true,
    data: {
      governance: {},
      validation: { valid: true },
      manifest: {
        outcome: "recovery_required",
        operation_id: "archive-op-7",
        mode: "resume",
        status: "recovery_required",
        step: "parent_meta_updated",
        source_path: ".spec/changes/child-change",
        target_path: ".spec/archive/2026-07-13-child-change",
        persisted: true,
        resumable: true,
        recovery_hint: "rerun with --resume archive-op-7",
      },
    },
  }, { action: "archive" });

  expect(text).toContain("archive: recovery_required");
  expect(text).toContain("operation_id: archive-op-7");
  expect(text).toContain("mode: resume");
  expect(text).toContain("step: parent_meta_updated");
  expect(text).toContain("recovery_hint: rerun with --resume archive-op-7");
});

test("human renderer relays canonical query groups and answer without re-ranking", () => {
  const text = renderHumanResponse({
    ok: true,
    data: {
      query_trust: "stale_but_queryable",
      recommended_action: "update",
      route_groups: [
        { route_tag: "index_symbol_hit", returned_count: 2, results: [{}, {}] },
        { route_tag: "projection_ref", returned_count: 1, results: [{}] },
      ],
      answer: {
        text: "Inspect the returned references before relying on this answer.",
        answer_mode: "degraded",
        answer_trust: "constrained",
      },
    },
  }, { action: "query" });

  expect(text).toContain("query_trust: stale_but_queryable");
  expect(text).toContain("route_groups: 2");
  expect(text).toContain("results: 3");
  expect(text).toContain("answer_mode: degraded");
  expect(text).toContain("answer_trust: constrained");
  expect(text).toContain("answer: Inspect the returned references");
  expect(text).not.toContain("matched_pages");
  expect(text).not.toContain("provenance_summary");
});
