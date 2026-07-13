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
