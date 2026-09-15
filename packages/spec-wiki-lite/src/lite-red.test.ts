import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
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

test("init creates the default Chinese bootstrap Wiki, config, spec directories, and skills", async () => {
  const root = makeTempDir();

  await runBootstrapInit({
    repoRoot: root,
    hosts: "codex",
    env: process.env,
    codegraph: { mode: "skip" },
    aoci: { mode: "skip" },
  });

  expect(existsSync(path.join(root, ".wiki", "INDEX.md"))).toBe(true);
  expect(existsSync(path.join(root, ".wiki", "00-文档约定", "01-页面模板.md"))).toBe(true);
  expect(existsSync(path.join(root, ".wiki", "01-快速上手", "INDEX.md"))).toBe(true);
  expect(existsSync(path.join(root, ".wiki", "02-开发指南", "00-代码注释规范.md"))).toBe(true);
  expect(existsSync(path.join(root, ".wiki", "03-模块指南", "INDEX.md"))).toBe(true);
  expect(existsSync(path.join(root, ".wiki", "04-对外方法", "INDEX.md"))).toBe(true);
  expect(readFileSync(path.join(root, ".wiki", "config.yaml"), "utf8")).toContain("language: zh");
  expect(readFileSync(path.join(root, ".wiki", "INDEX.md"), "utf8")).toContain("spec-wiki-lite:bootstrap-pending");
  expect(existsSync(path.join(root, ".spec", "changes"))).toBe(true);
  expect(existsSync(path.join(root, ".agents", "skills", "wiki-continue", "SKILL.md"))).toBe(true);
  expect(existsSync(path.join(root, ".agents", "skills", "wiki-plan", "references", "tasks-template.md"))).toBe(true);
  expect(existsSync(path.join(root, ".agents", "skills", "wiki-review", "references", "review-standard.python.md"))).toBe(true);
  expect(readFileSync(path.join(root, ".agents", "skills", "wiki-continue", "SKILL.md"), "utf8")).toContain("跨阶段调度");
  expect(existsSync(path.join(root, ".codex"))).toBe(false);
});

test("cli initializes English explicitly and reports bootstrap readiness", async () => {
  const root = makeTempDir();
  const initStdout: string[] = [];
  expect(await runCli(["init", "--language", "en", "--no-codegraph", "--no-aoci"], {
    cwd: root,
    env: process.env,
    stdout: text => initStdout.push(text),
    stderr: () => undefined,
  })).toBe(2);
  expect(existsSync(path.join(root, ".wiki", "01-quick-start", "INDEX.md"))).toBe(true);
  expect(readFileSync(path.join(root, ".wiki", "config.yaml"), "utf8")).toContain("language: en");
  expect(readFileSync(path.join(root, ".agents", "skills", "wiki-continue", "SKILL.md"), "utf8")).toContain("Cross-stage routing");

  const stdout: string[] = [];
  expect(await runCli(["status", "--json"], {
    cwd: root,
    env: process.env,
    stdout: text => stdout.push(text),
    stderr: () => undefined,
  })).toBe(2);
  expect(JSON.parse(stdout.join("")).data).toEqual(expect.objectContaining({
    ready: false,
    wiki: expect.objectContaining({ language: "en", bootstrapPending: true, ready: true }),
  }));

  const index = path.join(root, ".wiki", "INDEX.md");
  writeFileSync(index, readFileSync(index, "utf8").replace("<!-- spec-wiki-lite:bootstrap-pending -->", ""), "utf8");
  const completed: string[] = [];
  await runCli(["status", "--json"], {
    cwd: root,
    env: process.env,
    stdout: text => completed.push(text),
    stderr: () => undefined,
  });
  expect(JSON.parse(completed.join("")).data).toEqual(expect.objectContaining({
    ready: false,
    wiki: expect.objectContaining({ bootstrapPending: false }),
    tools: expect.objectContaining({ ready: false }),
  }));
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
  expect(stdout.join("")).toContain("--language zh|en");
  expect(stdout.join("")).toContain("--no-codegraph");
  expect(stdout.join("")).toContain("--no-aoci");
  expect(stdout.join("")).toContain("update [--force] [--tools]");
  expect(stdout.join("")).not.toContain("--codegraph]");
  expect(stdout.join("")).toContain("--json");
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
  expect(await runCli(["init", "--language", "fr"], io)).toBe(64);
  expect(await runCli(["update", "--language", "en"], io)).toBe(64);
  expect(await runCli(["query", "anything"], io)).toBe(64);
  expect(stderr.join("")).toContain("Supported hosts: codex");
  expect(stderr.join("")).toContain("Supported languages: zh, en");
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

    const code = await runCli(["init", input, "--no-codegraph", "--no-aoci"], {
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

test("init json reports both tool deferrals and returns not-ready", async () => {
  const root = makeTempDir();
  const stdout: string[] = [];
  const stderr: string[] = [];
  const code = await runCli(["init", "--no-codegraph", "--no-aoci", "--json"], {
    cwd: root,
    env: process.env,
    stdout: text => stdout.push(text),
    stderr: text => stderr.push(text),
  });
  expect(code).toBe(2);
  expect(stderr).toEqual([]);
  expect(JSON.parse(stdout.join("")).data.tools.codegraph).toEqual(expect.objectContaining({
    requested: false,
    warnings: [],
  }));
  expect(JSON.parse(stdout.join("")).data.tools.aoci).toEqual(expect.objectContaining({ requested: false }));
});
