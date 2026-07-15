import { expect, test } from "vitest";

import { exitCodeForResponse } from "./exitPolicy.js";
import { parseResult } from "./parseResult.js";

test("parseResult validates errorKind and exit policy maps structured partials", () => {
  expect(parseResult("{\"ok\":false,\"error\":\"bad\",\"errorKind\":\"invalid_argument\"}").errorKind).toBe("invalid_argument");
  expect(() => parseResult("{\"ok\":false,\"errorKind\":\"unknown\"}")).toThrow("errorKind");
  expect(exitCodeForResponse({ ok: true, data: { outcome: "partial" } })).toBe(2);
  expect(exitCodeForResponse({ ok: true, data: { validation: { valid: false } } })).toBe(2);
  expect(exitCodeForResponse({ ok: false, errorKind: "workflow_failed" })).toBe(1);
  expect(exitCodeForResponse({ ok: false, errorKind: "index_not_ready" })).toBe(1);
  expect(exitCodeForResponse({ ok: false, errorKind: "invalid_argument" })).toBe(64);
});

test.each([
  "archive_not_ready",
  "archive_precondition_changed",
  "archive_conflict",
  "archive_locked",
  "archive_recovery_required",
] as const)("archive typed error %s exits with partial", (errorKind) => {
  const response = parseResult(JSON.stringify({ ok: false, error: "archive stopped", errorKind }));
  expect(exitCodeForResponse(response)).toBe(2);
});

test("archive manifest failures remain hard failures", () => {
  const response = parseResult(JSON.stringify({ ok: false, error: "invalid manifest", errorKind: "archive_manifest_invalid" }));
  expect(exitCodeForResponse(response)).toBe(1);
});
