import {
  mkdtempSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";

import { afterAll, beforeAll, expect, test } from "vitest";

import { stagePackage } from "../build-dist.mjs";

const root = path.resolve(import.meta.dirname, "../..");
const temporaryRoot = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-lite-tarball-"));
const stagedPackage = path.join(temporaryRoot, "staged");
const installRoot = path.join(temporaryRoot, "install");
let executable: string;
let toolEnv: NodeJS.ProcessEnv;

function run(command: string, args: string[], cwd: string, shell = false, allowedStatuses = [0]) {
  const result = spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    env: toolEnv ?? process.env,
    shell,
  });
  if (!allowedStatuses.includes(result.status ?? -1)) {
    throw new Error([
      `${command} ${args.join(" ")} exited with ${result.status}`,
      result.stdout,
      result.stderr,
    ].join("\n"));
  }
  return result;
}

function status(projectRoot: string) {
  return (JSON.parse(
    run(process.execPath, [executable, "status", "--json"], projectRoot, false, [0, 2]).stdout,
  ) as {
    data: {
      ready: boolean;
      wiki: { bootstrapPending: boolean; language: string; ready: boolean };
      skills: Array<{ installed: boolean; name: string; path: string }>;
    };
  }).data;
}

function countFiles(directory: string): number {
  return readdirSync(directory, { recursive: true, withFileTypes: true })
    .filter(entry => entry.isFile())
    .length;
}

beforeAll(() => {
  stagePackage({ rootDir: root, outputDir: stagedPackage });
  const useShell = process.platform === "win32";
  const packed = run("npm", ["pack", "--silent"], stagedPackage, useShell);
  const tarballName = packed.stdout.trim().split(/\r?\n/u).at(-1);
  if (!tarballName) {
    throw new Error("npm pack did not return a tarball name");
  }

  writeFileSync(
    path.join(temporaryRoot, "package.json"),
    `${JSON.stringify({ name: "spec-wiki-lite-smoke", private: true })}\n`,
    "utf8",
  );
  run(
    "npm",
    ["install", "--ignore-scripts", "--no-audit", "--no-fund", path.join(stagedPackage, tarballName)],
    temporaryRoot,
    useShell,
  );
  executable = path.join(
    temporaryRoot,
    "node_modules",
    "spec-wiki-lite",
    "bin",
    "spec-wiki-lite.js",
  );
  const codegraphFixture = path.join(temporaryRoot, "fake-codegraph.mjs");
  const aociFixture = path.join(temporaryRoot, "fake-aoci.mjs");
  const codexHome = path.join(temporaryRoot, "codex-home");
  writeFileSync(codegraphFixture, [
    "const args = process.argv.slice(2);",
    "if (args[0] === '--version') console.log('1.6.0');",
    "else if (args[0] === 'status') console.log(JSON.stringify({initialized:true,version:'1.6.0',pendingChanges:{added:0,modified:0,removed:0},worktreeMismatch:null,index:{state:'complete',reindexRecommended:false,pendingRefs:0}}));",
  ].join("\n"), "utf8");
  writeFileSync(aociFixture, [
    "const args = process.argv.slice(2);",
    "if (args.includes('--version')) console.log('aoci version 0.1.0-rc12');",
    "else if (args.includes('list')) console.log(JSON.stringify({sources:[]}));",
    "else if (args.includes('--json')) console.log(JSON.stringify({ok:true}));",
  ].join("\n"), "utf8");
  mkdirSync(codexHome, { recursive: true });
  writeFileSync(path.join(codexHome, "config.toml"), "[mcp_servers.codegraph]\ncommand = 'codegraph'\n", "utf8");
  toolEnv = {
    ...process.env,
    CODEX_HOME: codexHome,
    SPEC_WIKI_LITE_CODEGRAPH_NODE_SCRIPT: codegraphFixture,
    SPEC_WIKI_LITE_AOCI_NODE_SCRIPT: aociFixture,
  };
}, 120_000);

afterAll(() => rmSync(temporaryRoot, { recursive: true, force: true }));

test("installed tarball initializes both languages and becomes ready after bootstrap", () => {
  run(process.execPath, [executable, "init", "install/zh-project", "--host", "codex"], temporaryRoot);
  run(
    process.execPath,
    [executable, "init", "install/en-project", "--host", "codex", "--language", "en"],
    temporaryRoot,
  );

  const zhRoot = path.join(installRoot, "zh-project");
  const enRoot = path.join(installRoot, "en-project");
  expect(readFileSync(path.join(zhRoot, ".wiki", "config.yaml"), "utf8")).toContain("language: zh");
  expect(readFileSync(path.join(enRoot, ".wiki", "config.yaml"), "utf8")).toContain("language: en");
  expect(readFileSync(path.join(zhRoot, ".wiki", "01-快速上手", "INDEX.md"), "utf8")).toContain("快速上手");
  expect(readFileSync(path.join(enRoot, ".wiki", "01-quick-start", "INDEX.md"), "utf8")).toContain("Quick Start");
  expect(readdirSync(path.join(zhRoot, ".agents", "skills"))).toHaveLength(8);
  expect(readdirSync(path.join(enRoot, ".agents", "skills"))).toHaveLength(8);
  expect(countFiles(path.join(zhRoot, ".agents", "skills"))).toBe(28);
  expect(countFiles(path.join(enRoot, ".agents", "skills"))).toBe(28);
  expect(readFileSync(path.join(zhRoot, ".agents", "skills", "wiki-continue", "SKILL.md"), "utf8")).toContain("跨阶段调度");
  expect(readFileSync(path.join(enRoot, ".agents", "skills", "wiki-continue", "SKILL.md"), "utf8")).toContain("Cross-stage routing");

  const zhPending = status(zhRoot);
  const enPending = status(enRoot);
  expect(zhPending).toEqual(expect.objectContaining({
    ready: false,
    wiki: expect.objectContaining({ bootstrapPending: true, language: "zh", ready: true }),
  }));
  expect(enPending).toEqual(expect.objectContaining({
    ready: false,
    wiki: expect.objectContaining({ bootstrapPending: true, language: "en", ready: true }),
  }));
  expect(zhPending.skills.every(skill => skill.installed)).toBe(true);
  expect(enPending.skills.every(skill => skill.installed)).toBe(true);

  writeFileSync(path.join(zhRoot, ".wiki", "config.yaml"), "version: 1\nwiki:\n  language: en\n", "utf8");
  run(process.execPath, [executable, "update", "--json"], zhRoot);
  expect(readFileSync(path.join(zhRoot, ".agents", "skills", "wiki-continue", "SKILL.md"), "utf8")).toContain("Cross-stage routing");
  expect(countFiles(path.join(zhRoot, ".agents", "skills"))).toBe(28);
  expect(status(zhRoot).skills.every(skill => skill.installed)).toBe(true);

  writeFileSync(path.join(enRoot, ".wiki", "config.yaml"), "version: 1\nwiki:\n  language: zh\n", "utf8");
  run(process.execPath, [executable, "update", "--json"], enRoot);
  expect(readFileSync(path.join(enRoot, ".agents", "skills", "wiki-continue", "SKILL.md"), "utf8")).toContain("跨阶段调度");
  expect(countFiles(path.join(enRoot, ".agents", "skills"))).toBe(28);
  expect(status(enRoot).skills.every(skill => skill.installed)).toBe(true);

  const rootIndex = path.join(zhRoot, ".wiki", "INDEX.md");
  writeFileSync(
    rootIndex,
    readFileSync(rootIndex, "utf8").replace("<!-- spec-wiki-lite:bootstrap-pending -->", ""),
    "utf8",
  );
  const completed = status(zhRoot);
  expect(completed).toEqual(expect.objectContaining({
    ready: true,
    wiki: expect.objectContaining({ bootstrapPending: false, ready: true }),
  }));
}, 120_000);

test("installed tarball can explicitly defer both required tools", () => {
  const result = run(
    process.execPath,
    [executable, "init", "install/deferred-project", "--no-codegraph", "--no-aoci", "--json"],
    temporaryRoot,
    false,
    [2],
  );
  const payload = JSON.parse(result.stdout) as { ok: boolean; data: { tools: { ready: boolean } } };
  expect(payload.ok).toBe(false);
  expect(payload.data.tools.ready).toBe(false);
}, 120_000);
