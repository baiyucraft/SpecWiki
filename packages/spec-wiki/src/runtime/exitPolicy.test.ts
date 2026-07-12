import { expect, test } from "vitest";

import { exitCodeForResponse } from "./exitPolicy.js";
import { parseResult } from "./parseResult.js";

test("parseResult validates errorKind and exit policy maps structured partials", () => {
  expect(parseResult("{\"ok\":false,\"error\":\"bad\",\"errorKind\":\"invalid_argument\"}").errorKind).toBe("invalid_argument");
  expect(() => parseResult("{\"ok\":false,\"errorKind\":\"unknown\"}")).toThrow("errorKind");
  expect(exitCodeForResponse({ ok: true, data: { outcome: "partial" } })).toBe(2);
  expect(exitCodeForResponse({ ok: true, data: { validation: { valid: false } } })).toBe(2);
  expect(exitCodeForResponse({ ok: false, errorKind: "workflow_failed" })).toBe(1);
  expect(exitCodeForResponse({ ok: false, errorKind: "invalid_argument" })).toBe(64);
});
