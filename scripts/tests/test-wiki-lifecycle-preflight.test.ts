import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { expect, test } from "vitest";

import {
  shouldAcceptStatusAfterWarmRestore,
  shouldAttemptWarmRestorePreflight,
  shouldSkipInitAfterWarmRestore,
} from "../test-wiki-lifecycle.mjs";

test("warm restore preflight 仅在 formal artifact 存在且 cache 缺失时触发", () => {
  const wikiDir = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-lifecycle-"));
  mkdirSync(path.join(wikiDir, ".knowledge", "runtime"), { recursive: true });
  writeFileSync(path.join(wikiDir, ".knowledge", "runtime", "recovery-manifest.json"), "{}\n");

  const ctx = {
    isRealRepo: false,
    cacheMode: "preserve",
    wikiDir,
  };
  const runtimeSnapshot = {
    metadataExists: true,
    cacheDbExists: false,
  };

  try {
    expect(shouldAttemptWarmRestorePreflight(ctx, runtimeSnapshot)).toBe(true);
    expect(shouldAttemptWarmRestorePreflight({ ...ctx, cacheMode: "clear" }, runtimeSnapshot)).toBe(false);
    expect(shouldAttemptWarmRestorePreflight({ ...ctx, isRealRepo: true }, runtimeSnapshot)).toBe(false);
    expect(shouldAttemptWarmRestorePreflight(ctx, { ...runtimeSnapshot, metadataExists: false })).toBe(false);
    expect(shouldAttemptWarmRestorePreflight(ctx, { ...runtimeSnapshot, cacheDbExists: true })).toBe(false);
  } finally {
    rmSync(wikiDir, { recursive: true, force: true });
  }
});

test("warm restore preflight 只在 lifecycle 支持消费的状态下跳过首次 init", () => {
  expect(shouldSkipInitAfterWarmRestore({ data: { state: "fresh" } })).toBe(true);
  expect(shouldSkipInitAfterWarmRestore({ data: { state: "needs_update" } })).toBe(true);
  expect(shouldSkipInitAfterWarmRestore({ data: { state: "runtime_incomplete" } })).toBe(true);
  expect(shouldSkipInitAfterWarmRestore({ data: { state: "blocker" } })).toBe(true);
  expect(shouldSkipInitAfterWarmRestore({ data: { state: "missing" } })).toBe(false);
  expect(shouldSkipInitAfterWarmRestore({ data: { state: "needs_rebuild" } })).toBe(false);
});

test("warm restore status after init 接受 fresh 与 needs_update", () => {
  expect(shouldAcceptStatusAfterWarmRestore({ data: { state: "fresh" } })).toBe(true);
  expect(shouldAcceptStatusAfterWarmRestore({ data: { state: "needs_update" } })).toBe(true);
  expect(shouldAcceptStatusAfterWarmRestore({ data: { state: "runtime_incomplete" } })).toBe(false);
});
