import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { afterEach, expect, test } from "vitest";

import { runCodeGraphIntegration, type CodeGraphCommandResult } from "./runner.js";

const roots: string[] = [];
afterEach(() => { for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true }); });

function project(): { root: string; env: NodeJS.ProcessEnv } {
  const root = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-codegraph-"));
  const codex = path.join(root, "codex");
  mkdirSync(codex);
  writeFileSync(path.join(codex, "config.toml"), "[mcp_servers.codegraph]\ncommand = 'codegraph'\n");
  roots.push(root);
  return { root, env: { CODEX_HOME: codex } };
}

const healthyStatus = JSON.stringify({
  initialized: true,
  version: "1.6.0",
  pendingChanges: { added: 0, modified: 0, removed: 0 },
  worktreeMismatch: null,
  index: { state: "complete", reindexRecommended: false, pendingRefs: 0 },
});

function successful(stdout = "1.6.0"): CodeGraphCommandResult {
  return { code: 0, stdout, stderr: "" };
}

test("reuses an exact healthy CodeGraph installation", async () => {
  const fixture = project();
  const calls: string[] = [];
  const result = await runCodeGraphIntegration({
    projectRoot: fixture.root,
    env: fixture.env,
    runner: async (command, args) => {
      calls.push([command, ...args].join(" "));
      return successful(args[0] === "status" ? healthyStatus : "1.6.0");
    },
  });
  expect(result.cli).toEqual(expect.objectContaining({ compatible: true, version: "1.6.0" }));
  expect(result.project.healthy).toBe(true);
  expect(result.codexMcp.configured).toBe(true);
  expect(calls).toEqual(["codegraph --version", "codegraph status --json", "codegraph --version", "codegraph status --json"]);
});

test("installs the exact pinned package and initializes the project with argv arrays", async () => {
  const fixture = project();
  rmSync(path.join(fixture.env.CODEX_HOME!, "config.toml"));
  const calls: string[][] = [];
  let installed = false;
  let initialized = false;
  const result = await runCodeGraphIntegration({
    projectRoot: `${fixture.root};safe`,
    env: fixture.env,
    runner: async (command, args) => {
      calls.push([command, ...args]);
      if (command === "npm") { installed = true; return successful(); }
      if (args[0] === "--version") return installed ? successful("1.6.0") : { code: 1, stdout: "", stderr: "missing" };
      if (args[0] === "install") return successful();
      if (args[0] === "init") { initialized = true; return successful(); }
      if (args[0] === "status") return successful(initialized ? healthyStatus : JSON.stringify({ initialized: false }));
      return successful();
    },
  });
  expect(calls).toContainEqual(["npm", "install", "-g", "@colbymchenry/codegraph@1.6.0"]);
  expect(calls).toContainEqual(["codegraph", "init", `${fixture.root};safe`]);
  expect(result.cli.installed).toBe(true);
  expect(result.project.healthy).toBe(true);
});

test("no-codegraph defers all external work and remains not ready", async () => {
  let called = false;
  const result = await runCodeGraphIntegration({
    projectRoot: "C:/project",
    env: {},
    mode: "skip",
    runner: async () => { called = true; return successful(); },
  });
  expect(called).toBe(false);
  expect(result.requested).toBe(false);
  expect(result.project.healthy).toBe(false);
  expect(result.nextActions).not.toHaveLength(0);
});

test("syncs a stale initialized index", async () => {
  const fixture = project();
  const calls: string[] = [];
  let synced = false;
  await runCodeGraphIntegration({
    projectRoot: fixture.root,
    env: fixture.env,
    runner: async (command, args) => {
      calls.push([command, ...args].join(" "));
      if (args[0] === "--version") return successful("1.6.0");
      if (args[0] === "sync") { synced = true; return successful(); }
      if (args[0] === "status") return successful(synced ? healthyStatus : JSON.stringify({
        ...JSON.parse(healthyStatus),
        pendingChanges: { added: 1, modified: 0, removed: 0 },
      }));
      return successful();
    },
  });
  expect(calls).toContain("codegraph sync");
});

test("normalizes runner failures into actionable warnings", async () => {
  const result = await runCodeGraphIntegration({
    projectRoot: "C:/project",
    env: {},
    runner: async () => { throw new Error("runner crashed"); },
  });
  expect(result.project.healthy).toBe(false);
  expect(result.warnings.some(item => item.message.includes("runner crashed"))).toBe(true);
  expect(result.nextActions).not.toHaveLength(0);
});
