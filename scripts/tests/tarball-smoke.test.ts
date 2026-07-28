import {
  mkdtempSync,
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

function run(command: string, args: string[], cwd: string, shell = false) {
  const result = spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    shell,
  });
  if (result.status !== 0) {
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
    run(process.execPath, [executable, "status", "--json"], projectRoot).stdout,
  ) as {
    data: {
      ready: boolean;
      wiki: { bootstrapPending: boolean; language: string; ready: boolean };
    };
  }).data;
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
