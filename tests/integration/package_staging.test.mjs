import assert from "node:assert/strict";
import test from "node:test";
import { existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";

test("staging script creates platform package manifest", async () => {
  const rootDir = process.cwd();
  const distDir = path.join(rootDir, "dist");
  const binaryDir = path.join(rootDir, "target", "debug");
  const binaryName = process.platform === "win32" ? "wiki-core.exe" : "wiki-core";
  const binaryPath = path.join(binaryDir, binaryName);

  rmSync(distDir, { recursive: true, force: true });
  mkdirSync(binaryDir, { recursive: true });
  writeFileSync(binaryPath, "mock-binary");

  const { stagePackages } = await import("../../scripts/stage-binaries.mjs");
  await stagePackages({ rootDir });

  assert.equal(
    existsSync(path.join(distDir, "npm", "codebuddy-wiki", "package.json")),
    true,
  );
  assert.equal(
    existsSync(
      path.join(distDir, "npm", "codebuddy-wiki-win32-x64-msvc", "package.json"),
    ),
    true,
  );
});
