import { existsSync, mkdirSync, mkdtempSync, rmSync, symlinkSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { afterEach, expect, test } from "vitest";

import { runCli } from "./cli.js";
import { runBootstrapInit } from "./orchestration/init/runInit.js";

const tempDirs: string[] = [];

function makeTempDir(): string {
  const dir = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-lite-red-"));
  tempDirs.push(dir);
  return dir;
}

afterEach(() => {
  for (const dir of tempDirs.splice(0)) {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("init creates the Lite wiki, spec directories, and canonical agent skills", async () => {
  const root = makeTempDir();

  await runBootstrapInit({ repoRoot: root, hosts: "codex", env: process.env });

  expect(existsSync(path.join(root, ".wiki", "INDEX.md"))).toBe(true);
  expect(existsSync(path.join(root, ".wiki", "00-conventions", "00-page-template.md"))).toBe(true);
  expect(existsSync(path.join(root, ".wiki", "01-project", "00-overview.md"))).toBe(true);
  expect(existsSync(path.join(root, ".wiki", "02-development", "00-getting-started.md"))).toBe(true);
  expect(existsSync(path.join(root, ".wiki", "02-development", "01-testing.md"))).toBe(true);
  expect(existsSync(path.join(root, ".wiki", "03-architecture", "00-system-overview.md"))).toBe(true);
  expect(existsSync(path.join(root, ".wiki", "04-reference", "INDEX.md"))).toBe(true);
  expect(existsSync(path.join(root, ".spec", "changes"))).toBe(true);
  expect(existsSync(path.join(root, ".agents", "skills", "wiki-continue", "SKILL.md"))).toBe(true);
  expect(existsSync(path.join(root, ".codex"))).toBe(false);
});

test("help exposes only the SpecWiki Lite command surface", async () => {
  const stdout: string[] = [];
  const stderr: string[] = [];

  const code = await runCli(["--help"], {
    cwd: makeTempDir(),
    env: process.env,
    stdout: text => stdout.push(text),
    stderr: text => stderr.push(text),
  });

  expect(code).toBe(0);
  expect(stderr).toEqual([]);
  expect(stdout.join("")).toContain("spec-wiki-lite init");
  expect(stdout.join("")).toContain("spec-wiki-lite show");
  expect(stdout.join("")).not.toMatch(/spec-wiki (query|sync|rebuild)/);
});

test("cli rejects unknown hosts and removed commands as usage errors", async () => {
  const root = makeTempDir();
  const stderr: string[] = [];
  const io = {
    cwd: root,
    env: process.env,
    stdout: () => undefined,
    stderr: (text: string) => stderr.push(text),
  };

  expect(await runCli(["init", "--host", "claude"], io)).toBe(64);
  expect(await runCli(["query", "anything"], io)).toBe(64);
  expect(stderr.join("")).toContain("Supported hosts: codex");
  expect(stderr.join("")).toContain("unknown command: query");
});

test.each(["parent traversal", "absolute path", "junction escape"])(
  "init rejects %s before creating files outside cwd",
  async (mode) => {
    const container = makeTempDir();
    const cwd = path.join(container, "project");
    const outside = makeTempDir();
    mkdirSync(cwd);
    let input: string;
    let forbidden: string;
    if (mode === "parent traversal") {
      input = "../escape";
      forbidden = path.join(container, "escape");
    } else if (mode === "absolute path") {
      input = path.join(container, "absolute");
      forbidden = input;
    } else {
      const link = path.join(cwd, "linked");
      symlinkSync(outside, link, "junction");
      input = "linked/nested";
      forbidden = path.join(outside, "nested");
    }
    const stderr: string[] = [];

    const code = await runCli(["init", input], {
      cwd,
      env: process.env,
      stdout: () => undefined,
      stderr: text => stderr.push(text),
    });

    expect(code).toBe(1);
    expect(stderr.join("")).toContain("unsafe path");
    expect(existsSync(forbidden)).toBe(false);
  },
);

test("json validate returns not-ready on stdout only", async () => {
  const root = makeTempDir();
  const change = path.join(root, ".spec", "changes", "missing-design");
  const { mkdirSync, writeFileSync } = await import("node:fs");
  mkdirSync(change, { recursive: true });
  writeFileSync(path.join(change, "meta.yaml"), [
    "id: missing-design",
    "stage: design",
    "deliveryShape: single-change",
  ].join("\n"), "utf8");
  writeFileSync(path.join(change, "proposal.md"), "# Proposal\n", "utf8");
  const stdout: string[] = [];
  const stderr: string[] = [];

  const code = await runCli(["validate", "missing-design", "--json"], {
    cwd: root,
    env: process.env,
    stdout: text => stdout.push(text),
    stderr: text => stderr.push(text),
  });

  expect(code).toBe(2);
  expect(stderr).toEqual([]);
  expect(JSON.parse(stdout.join(""))).toEqual(expect.objectContaining({ ok: false }));
});

test("json show returns not-ready for an invalid or missing change", async () => {
  const stdout: string[] = [];
  const stderr: string[] = [];

  const code = await runCli(["show", "missing-change", "--json"], {
    cwd: makeTempDir(),
    env: process.env,
    stdout: text => stdout.push(text),
    stderr: text => stderr.push(text),
  });

  expect(code).toBe(2);
  expect(stderr).toEqual([]);
  expect(JSON.parse(stdout.join(""))).toEqual(expect.objectContaining({ ok: false }));
});
