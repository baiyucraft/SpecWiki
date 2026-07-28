import { mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { afterEach, expect, test } from "vitest";

import { syncProjectAssets } from "../assets/sync.js";
import { inspectWiki } from "./inspect.js";

const roots: string[] = [];

function makeTempDir(): string {
  const root = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-lite-wiki-"));
  roots.push(root);
  return root;
}

function createBrokenWikiFixture(): string {
  const root = makeTempDir();
  const wiki = path.join(root, ".wiki");
  mkdirSync(path.join(wiki, "missing-index"), { recursive: true });
  writeFileSync(path.join(wiki, "INDEX.md"), [
    "---",
    "title: Root",
    "description: Root navigation.",
    "updated: 2026-07-27",
    "owner: project",
    "---",
    "",
    "# Root",
    "",
    "[Broken](./missing.md)",
  ].join("\n"), "utf8");
  writeFileSync(path.join(wiki, "missing-index", "01-invalid.md"), "# No frontmatter\n", "utf8");
  writeFileSync(path.join(wiki, "01-orphan.md"), [
    "---",
    "title: Orphan",
    "description: Unlinked page.",
    "updated: 2026-07-27",
    "owner: project",
    "source_of_truth: shared-contract",
    "---",
    "",
    "# Orphan",
  ].join("\n"), "utf8");
  writeFileSync(path.join(wiki, "02-duplicate.md"), [
    "---",
    "title: Duplicate",
    "description: Duplicate source of truth.",
    "updated: 2026-07-27",
    "owner: project",
    "source_of_truth: shared-contract",
    "---",
    "",
    "# Duplicate",
  ].join("\n"), "utf8");
  return root;
}

afterEach(() => {
  for (const root of roots.splice(0)) {
    rmSync(root, { recursive: true, force: true });
  }
});

test("reports structural wiki issues without runtime fields", async () => {
  const report = await inspectWiki(createBrokenWikiFixture());
  expect(report.issues.map(issue => issue.kind)).toEqual(expect.arrayContaining([
    "missing_index",
    "invalid_frontmatter",
    "broken_link",
    "orphan_page",
    "duplicate_ssot",
  ]));
  expect(report).not.toHaveProperty("runtime_readiness");
});

test("accepts the built-in scaffold and excludes legacy runtime directories", async () => {
  const root = makeTempDir();
  await syncProjectAssets(root);
  const legacy = path.join(root, ".wiki", ".knowledge");
  mkdirSync(legacy, { recursive: true });
  writeFileSync(path.join(legacy, "broken.md"), "not a current Wiki page", "utf8");

  const report = await inspectWiki(root);

  expect(report.ready).toBe(true);
  expect(report.issues).toEqual([]);
  expect(report.language).toBe("zh");
  expect(report.bootstrapPending).toBe(true);
  expect(report.pages).toHaveLength(10);
  expect(report.pages).toContain(".wiki/02-开发指南/00-代码注释规范.md");
  expect(report.pages).not.toContain(".wiki/.knowledge/broken.md");

  const rootIndex = path.join(root, ".wiki", "INDEX.md");
  writeFileSync(
    rootIndex,
    readFileSync(rootIndex, "utf8").replace("<!-- spec-wiki-lite:bootstrap-pending -->", ""),
    "utf8",
  );
  const completed = await inspectWiki(root);
  expect(completed.bootstrapPending).toBe(false);
  expect(completed.ready).toBe(true);
});

test("rejects a Wiki junction that escapes the project", async () => {
  const root = makeTempDir();
  const outside = makeTempDir();
  writeFileSync(path.join(outside, "INDEX.md"), [
    "---",
    "title: Outside",
    "description: Outside",
    "updated: 2026-07-28",
    "owner: test",
    "---",
  ].join("\n"), "utf8");
  symlinkSync(outside, path.join(root, ".wiki"), "junction");

  await expect(inspectWiki(root)).rejects.toThrow("unsafe path");
});
