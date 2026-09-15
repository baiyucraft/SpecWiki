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

function healthyToolOptions(root: string) {
  const codexHome = path.join(root, "codex-home");
  mkdirSync(codexHome, { recursive: true });
  writeFileSync(path.join(codexHome, "config.toml"), "[mcp_servers.codegraph]\ncommand = 'codegraph'\n", "utf8");
  return {
    env: { CODEX_HOME: codexHome },
    aociExecutablePath: path.join(root, "aoci"),
    codegraphRunner: async (_command: string, args: string[]) => ({
      code: 0,
      stdout: args[0] === "status" ? JSON.stringify({
        initialized: true,
        pendingChanges: { added: 0, modified: 0, removed: 0 },
        worktreeMismatch: null,
        index: { state: "complete", reindexRecommended: false, pendingRefs: 0 },
      }) : "1.6.0",
      stderr: "",
    }),
    aociRunner: async (_command: string, args: string[]) => ({
      code: 0,
      stdout: args.includes("--version") ? "aoci version 0.1.0-rc12" : JSON.stringify({ ok: true }),
      stderr: "",
    }),
  };
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

  const options = healthyToolOptions(root);
  expect((await getProjectStatus(root, options)).skills.every(skill => skill.installed)).toBe(true);

  const reference = path.join(root, ".agents", "skills", "wiki-plan", "references", "tasks-template.md");
  writeFileSync(reference, "stale localized reference", "utf8");
  const stale = await getProjectStatus(root, options);
  expect(stale.skills.find(skill => skill.name === "wiki-plan")?.installed).toBe(false);
  expect(stale.ready).toBe(false);

  await syncProjectAssets(root);
  expect((await getProjectStatus(root, options)).skills.find(skill => skill.name === "wiki-plan")?.installed).toBe(true);
});

test("reports a Skill missing when any registered reference is absent", async () => {
  const root = createProject();
  await syncProjectAssets(root, { language: "en" });
  rmSync(path.join(root, ".agents", "skills", "wiki-review", "references", "review-standard.python.md"));

  const report = await getProjectStatus(root, healthyToolOptions(root));
  expect(report.wiki.language).toBe("en");
  expect(report.skills.find(skill => skill.name === "wiki-review")?.installed).toBe(false);
});

test("treats an unreadable registered path as not installed instead of throwing", async () => {
  const root = createProject();
  await syncProjectAssets(root);
  const reference = path.join(root, ".agents", "skills", "wiki-plan", "references", "tasks-template.md");
  rmSync(reference);
  mkdirSync(reference);

  const report = await getProjectStatus(root, healthyToolOptions(root));
  expect(report.skills.find(skill => skill.name === "wiki-plan")?.installed).toBe(false);
});

test("aggregates official tool health into readiness", async () => {
  const root = createProject();
  await syncProjectAssets(root);
  const report = await getProjectStatus(root, healthyToolOptions(root));
  expect(report.tools.ready).toBe(true);
  expect(report.tools.codegraph.project.healthy).toBe(true);
  expect(report.tools.aoci.governanceAligned).toBe(true);
});
