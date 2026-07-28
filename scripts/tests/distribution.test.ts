import { existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";

import { afterAll, expect, test } from "vitest";

import { listPackageFiles, stagePackage } from "../build-dist.mjs";

const root = path.resolve(import.meta.dirname, "../..");
const output = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-lite-stage-"));

afterAll(() => rmSync(output, { recursive: true, force: true }));

test("stages a TypeScript-only package with assets and no native runtime", () => {
  const staged = stagePackage({ rootDir: root, outputDir: output });
  const files = listPackageFiles(output);

  expect(files).toContain("dist/index.js");
  expect(files).toContain("assets/wiki/INDEX.md");
  expect(files).toContain("assets/wiki/01-project/00-overview.md");
  expect(files).toContain("assets/wiki/03-architecture/00-system-overview.md");
  expect(files).toContain("assets/skills/wiki-continue/SKILL.md");
  expect(files).toContain("README.md");
  expect(files).toContain("LICENSE");
  expect(files.some(file => /(?:^|\/)(?:lib|crates)(?:\/|$)|\.exe$/u.test(file))).toBe(false);
  expect(staged.manifest).not.toHaveProperty("os");
  expect(staged.manifest).not.toHaveProperty("cpu");
  expect(staged.manifest).toEqual(expect.objectContaining({
    name: "spec-wiki-lite",
    version: "0.1.0",
    engines: { node: ">=20.19.0" },
    bin: { "spec-wiki-lite": "bin/spec-wiki-lite.js" },
  }));
});

test("staged executable exposes the Lite command closure", () => {
  const manifest = JSON.parse(readFileSync(path.join(output, "package.json"), "utf8")) as { bin: Record<string, string> };
  const bin = Object.values(manifest.bin)[0];
  expect(existsSync(path.join(output, bin))).toBe(true);
  const result = spawnSync(process.execPath, [path.join(output, bin), "--help"], { cwd: output, encoding: "utf8" });
  expect(result.status).toBe(0);
  expect(result.stdout).toContain("init [path]");
  expect(result.stdout).toContain("archive <change-id>");
  expect(result.stdout).not.toMatch(/\b(query|sync|rebuild)\b/u);
});
