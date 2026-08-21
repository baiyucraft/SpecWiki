import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { afterEach, expect, test } from "vitest";

import { syncProjectAssets } from "./assets/sync.js";
import { getProjectStatus } from "./status.js";

const roots: string[] = [];

function createProject(): string {
  const root = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-lite-status-"));
  roots.push(root);
  return root;
}

afterEach(() => {
  for (const root of roots.splice(0)) {
    rmSync(root, { recursive: true, force: true });
  }
});

test("requires every registered Skill file to match the selected locale", async () => {
  const root = createProject();
  await syncProjectAssets(root);
  const index = path.join(root, ".wiki", "INDEX.md");
  writeFileSync(index, readFileSync(index, "utf8").replace("<!-- spec-wiki-lite:bootstrap-pending -->", ""), "utf8");

  expect((await getProjectStatus(root)).skills.every(skill => skill.installed)).toBe(true);

  const reference = path.join(root, ".agents", "skills", "wiki-plan", "references", "tasks-template.md");
  writeFileSync(reference, "stale localized reference", "utf8");
  const stale = await getProjectStatus(root);
  expect(stale.skills.find(skill => skill.name === "wiki-plan")?.installed).toBe(false);
  expect(stale.ready).toBe(false);

  await syncProjectAssets(root);
  expect((await getProjectStatus(root)).skills.find(skill => skill.name === "wiki-plan")?.installed).toBe(true);
});

test("reports a Skill missing when any registered reference is absent", async () => {
  const root = createProject();
  await syncProjectAssets(root, { language: "en" });
  rmSync(path.join(root, ".agents", "skills", "wiki-review", "references", "review-standard.python.md"));

  const report = await getProjectStatus(root);
  expect(report.wiki.language).toBe("en");
  expect(report.skills.find(skill => skill.name === "wiki-review")?.installed).toBe(false);
});

test("treats an unreadable registered path as not installed instead of throwing", async () => {
  const root = createProject();
  await syncProjectAssets(root);
  const reference = path.join(root, ".agents", "skills", "wiki-plan", "references", "tasks-template.md");
  rmSync(reference);
  mkdirSync(reference);

  const report = await getProjectStatus(root);
  expect(report.skills.find(skill => skill.name === "wiki-plan")?.installed).toBe(false);
});

test("reports CodeGraph project state without running external commands", async () => {
  const root = createProject();
  await syncProjectAssets(root);
  const before = await getProjectStatus(root);
  expect(before.codegraph).toEqual({ initialized: false, path: ".codegraph" });
  mkdirSync(path.join(root, ".codegraph"));
  const after = await getProjectStatus(root);
  expect(after.codegraph).toEqual({ initialized: true, path: ".codegraph" });
});
