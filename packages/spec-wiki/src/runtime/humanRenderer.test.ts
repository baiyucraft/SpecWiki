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
