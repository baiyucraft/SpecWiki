import path from "node:path";
import os from "node:os";
import path from "node:path";
import { expect, test } from "vitest";

import { runCodeGraphIntegration, type CodeGraphCommandResult } from "./runner.js";

function successful(stdout = "1.5.0"): CodeGraphCommandResult {
  return { code: 0, stdout, stderr: "" };
}

test("runs version, Codex MCP install, and project init with argv-safe commands", async () => {
  const calls: Array<{ command: string; args: string[]; cwd?: string }> = [];
  const result = await runCodeGraphIntegration({
    projectRoot: "C:/tmp/project;safe",
    env: { CODEX_HOME: path.join(os.tmpdir(), "spec-wiki-lite-empty-codex") },
    mode: "force",
    runner: async (command, args, options) => {
      calls.push({ command, args, cwd: options.cwd });
      return successful();
    },
  });

  expect(result.requested).toBe(true);
  expect(result.cli).toEqual({ available: true, version: "1.5.0", installed: false });
  expect(result.codexMcp.configured).toBe(true);
  expect(result.project).toEqual({ initialized: true, path: ".codegraph" });
  expect(calls).toEqual([
    { command: "codegraph", args: ["--version"], cwd: "C:/tmp/project;safe" },
    { command: "codegraph", args: ["install", "--target=codex", "--location=global", "--yes", "--no-permissions"], cwd: "C:/tmp/project;safe" },
    { command: "codegraph", args: ["init", "C:/tmp/project;safe"], cwd: "C:/tmp/project;safe" },
  ]);
});

test("installs the CLI after a missing version command and continues on stage failures", async () => {
  const calls: string[] = [];
  const result = await runCodeGraphIntegration({
    projectRoot: "C:/project",
    env: {},
    mode: "force",
    runner: async (command, args) => {
      calls.push([command, ...args].join(" "));
      if (command === "codegraph") {
        if (args[0] === "--version") {
          return { code: 1, stdout: "", stderr: "not found" };
        }
        if (args[0] === "install") {
          return { code: 1, stdout: "", stderr: "permission denied" };
        }
      }
      if (command === "npm") {
        return successful();
      }
      return { code: 1, stdout: "", stderr: "index failed" };
    },
  });

  expect(calls[0]).toBe("codegraph --version");
  expect(calls[1]).toBe("npm install -g @colbymchenry/codegraph@latest");
  expect(result.cli.installed).toBe(true);
  expect(result.warnings.map(warning => warning.stage)).toEqual(["codex-mcp", "project"]);
  expect(result.project.initialized).toBe(false);
});

test("no-codegraph performs no external calls", async () => {
  let called = false;
  const result = await runCodeGraphIntegration({
    projectRoot: "C:/project",
    env: {},
    enabled: false,
    runner: async () => {
      called = true;
      return successful();
    },
  });
  expect(called).toBe(false);
  expect(result).toEqual({
    requested: false,
    cli: { available: false, installed: false },
    codexMcp: { configured: false },
    project: { initialized: false, path: ".codegraph" },
    warnings: [],
  });
});

test("normal init only detects and never installs or configures", async () => {
  const calls: string[] = [];
  const result = await runCodeGraphIntegration({
    projectRoot: "C:/project",
    env: { CODEX_HOME: path.join(os.tmpdir(), "spec-wiki-lite-empty-codex") },
    runner: async (command, args) => {
      calls.push([command, ...args].join(" "));
      return successful();
    },
  });
  expect(result.requested).toBe(true);
  expect(result.codexMcp.configured).toBe(false);
  expect(result.project.initialized).toBe(false);
  expect(calls).toEqual(["codegraph --version"]);
});

test("existing project index is reused in force mode", async () => {
  const calls: string[] = [];
  const result = await runCodeGraphIntegration({
    projectRoot: path.resolve(process.cwd(), "../.."),
    env: {},
    mode: "force",
    runner: async (command, args) => {
      calls.push([command, ...args].join(" "));
      return successful();
    },
  });
  expect(result.project.initialized).toBe(true);
  expect(calls).toEqual(["codegraph --version"]);
});

test("normalizes runner exceptions into warnings without blocking the workflow", async () => {
  const result = await runCodeGraphIntegration({
    projectRoot: "C:/project",
    env: {},
    mode: "force",
    runner: async () => {
      throw new Error("runner crashed");
    },
  });
  expect(result.warnings).toHaveLength(3);
  expect(result.warnings.every(item => item.message.includes("runner crashed"))).toBe(true);
});
