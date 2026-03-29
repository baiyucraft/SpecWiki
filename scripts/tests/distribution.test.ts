/**
 * 这个文件覆盖 spec-wiki 的单包分发骨架和 staging 结果。
 * 它确保 dist 发布真相已经收口到 `dist/spec-wiki`。
 */
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..", "..");

test("workspace skeleton files exist", () => {
  const requiredPaths = [
    "Cargo.toml",
    "package.json",
    "pnpm-workspace.yaml",
    "README.md",
    "crates/wiki-runtime/Cargo.toml",
    "crates/wiki-runtime/package.json",
    "crates/wiki-runtime/src/lib.rs",
    "crates/wiki-runtime/src/main.rs",
    "packages/spec-wiki/package.json",
    "packages/spec-wiki/tsconfig.json",
    "packages/spec-wiki/vite.config.mjs",
    "packages/spec-wiki/src/index.ts",
    "packages/spec-wiki/bin/spec-wiki.js",
    "scripts/build/core-paths.mjs",
    "scripts/build-dist.mjs",
    "scripts/publish-packages.mjs",
    "scripts/run-tests.mjs",
    "scripts/testing/helpers.mjs",
    "scripts/testing/lifecycle/bootstrap.mjs",
    "scripts/testing/lifecycle/steady.mjs",
    "scripts/testing/lifecycle/mutation.mjs",
    "scripts/testing/lifecycle/rebuild.mjs",
    "vitest.config.mjs",
    "scripts/tests/distribution.test.ts",
    "scripts/tests/e2e.test.ts",
    "scripts/tests/streaming-protocol.test.ts",
  ];

  for (const requiredPath of requiredPaths) {
    expect(existsSync(path.join(rootDir, requiredPath))).toBe(true);
  }
});

test("staging script creates a single publishable Windows x64 spec-wiki package with bundled runtime", async () => {
  const outputDir = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-stage-"));
  const profile = "test-integration";
  const stagePlatform = "win32";
  const binaryDir = path.join(rootDir, "target", profile);
  const binaryName = "wiki-runtime.exe";
  const binaryPath = path.join(binaryDir, binaryName);
  const mainManifestSource = JSON.parse(
    readFileSync(path.join(rootDir, "packages", "spec-wiki", "package.json"), "utf8"),
  );

  mkdirSync(binaryDir, { recursive: true });
  writeFileSync(binaryPath, "mock-binary");

  const { stagePackage } = await import("../build-dist.mjs");
  const staged = stagePackage({ rootDir, profile, outputDir, platform: stagePlatform });
  const stagedManifestPath = path.join(outputDir, "package.json");
  const stagedManifest = JSON.parse(readFileSync(stagedManifestPath, "utf8"));
  const stagedBinaryPath = path.join(outputDir, "lib", "x64-win32", path.basename(binaryPath));

  expect(existsSync(stagedManifestPath)).toBe(true);
  expect(stagedManifest.name).toBe(mainManifestSource.name);
  expect(stagedManifest.version).toBe(mainManifestSource.version);
  expect(stagedManifest.bin).toEqual(mainManifestSource.bin);
  expect(stagedManifest.main).toBe(mainManifestSource.main);
  expect(stagedManifest.exports).toEqual(mainManifestSource.exports);
  expect(stagedManifest.os).toEqual(mainManifestSource.os);
  expect(stagedManifest.cpu).toEqual(mainManifestSource.cpu);
  expect(stagedManifest.os).toEqual(["win32"]);
  expect(stagedManifest.cpu).toEqual(["x64"]);
  expect(stagedManifest.optionalDependencies).toBeUndefined();
  expect(stagedManifest.files).toContain("lib/**");
  expect(existsSync(path.join(outputDir, "bin", "spec-wiki.js"))).toBe(true);
  expect(existsSync(path.join(outputDir, "dist", "index.js"))).toBe(true);
  expect(existsSync(path.join(outputDir, "README.md"))).toBe(true);
  expect(existsSync(stagedBinaryPath)).toBe(true);
  expect(staged.packageDir).toBe(outputDir);
  expect(staged.stagedBinaryPath).toBe(stagedBinaryPath);
  expect(staged.manifest.name).toBe(mainManifestSource.name);
  expect(staged.manifest.os).toEqual(["win32"]);
  expect(staged.manifest.cpu).toEqual(["x64"]);

  rmSync(outputDir, { recursive: true, force: true, maxRetries: 5, retryDelay: 50 });
});

test("spec-wiki build writes bundle into the main package dist directory", async () => {
  const mainPackageDir = path.join(rootDir, "packages", "spec-wiki");
  const viteConfig = (await import("../../packages/spec-wiki/vite.config.mjs")).default;

  expect(path.normalize(viteConfig.root)).toBe(path.normalize(mainPackageDir));
  expect(path.normalize(viteConfig.build.outDir)).toBe(
    path.normalize(path.join(mainPackageDir, "dist")),
  );
});

