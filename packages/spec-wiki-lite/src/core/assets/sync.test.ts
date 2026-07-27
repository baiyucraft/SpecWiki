import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { afterEach, expect, test } from "vitest";

import { syncProjectAssets } from "./sync.js";

const roots: string[] = [];

function createTempProject(): string {
  const root = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-lite-assets-"));
  roots.push(root);
  return root;
}

afterEach(() => {
  for (const root of roots.splice(0)) {
    rmSync(root, { recursive: true, force: true });
  }
});

test("preserves scaffold and user pages while force only replaces managed baselines", async () => {
  const root = createTempProject();
  await syncProjectAssets(root, { force: false });
  const custom = path.join(root, ".wiki", "custom.md");
  writeFileSync(custom, "custom", "utf8");
  const index = path.join(root, ".wiki", "INDEX.md");
  writeFileSync(index, "user-owned index", "utf8");
  const managed = path.join(root, ".wiki", "00-conventions", "INDEX.md");
  writeFileSync(managed, "locally changed baseline", "utf8");

  const preserved = await syncProjectAssets(root);
  expect(readFileSync(managed, "utf8")).toBe("locally changed baseline");
  expect(preserved.preserved).toContain(".wiki/00-conventions/INDEX.md");

  const report = await syncProjectAssets(root, { force: true });

  expect(readFileSync(custom, "utf8")).toBe("custom");
  expect(readFileSync(index, "utf8")).toBe("user-owned index");
  expect(readFileSync(managed, "utf8")).toContain("name and navigation conventions");
  expect(report.preserved).toContain(".wiki/custom.md");
  expect(report.preserved).toContain(".wiki/INDEX.md");
  expect(report.updated).toContain(".wiki/00-conventions/INDEX.md");
  expect(report.unchanged).toContain(".agents/skills/wiki-continue/SKILL.md");
});

test("is idempotent and restores package-owned skills", async () => {
  const root = createTempProject();
  const first = await syncProjectAssets(root);
  const second = await syncProjectAssets(root);
  const skill = path.join(root, ".agents", "skills", "wiki-continue", "SKILL.md");
  writeFileSync(skill, "stale skill", "utf8");

  const repaired = await syncProjectAssets(root);

  expect(first.created).toHaveLength(10);
  expect(second.unchanged).toHaveLength(10);
  expect(repaired.updated).toContain(".agents/skills/wiki-continue/SKILL.md");
  expect(readFileSync(skill, "utf8")).toContain("name: wiki-continue");
});
