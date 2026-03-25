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
    "agents/codebuddy/package.json",
    "agents/codebuddy/tsconfig.json",
    "agents/codebuddy/vite.config.mjs",
    "agents/codebuddy/src/index.ts",
    "agents/codebuddy/bin/codebuddy.js",
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
    "scripts/templates/platform-package.json",
  ];

  for (const requiredPath of requiredPaths) {
    expect(existsSync(path.join(rootDir, requiredPath))).toBe(true);
  }
});

test("staging script creates platform package manifest", async () => {
  const outputDir = mkdtempSync(path.join(os.tmpdir(), "codebuddy-stage-"));
  const profile = "test-integration";
  const binaryDir = path.join(rootDir, "target", profile);
  const binaryName = process.platform === "win32" ? "wiki-runtime.exe" : "wiki-runtime";
  const binaryPath = path.join(binaryDir, binaryName);
  const adapterManifest = JSON.parse(
    readFileSync(path.join(rootDir, "agents", "codebuddy", "package.json"), "utf8"),
  );

  mkdirSync(binaryDir, { recursive: true });
  writeFileSync(binaryPath, "mock-binary");

  const { resolvePlatformPackageName } = await import("../build/core-paths.mjs");
  const { stagePackages } = await import("../build-dist.mjs");
  const staged = await stagePackages({ rootDir, profile, outputDir });
  const platformPackageName = resolvePlatformPackageName(adapterManifest.name);
  const mainManifestPath = path.join(outputDir, adapterManifest.name, "package.json");
  const platformManifestPath = path.join(outputDir, platformPackageName, "package.json");
  const mainManifest = JSON.parse(readFileSync(mainManifestPath, "utf8"));
  const platformManifest = JSON.parse(readFileSync(platformManifestPath, "utf8"));

  expect(existsSync(mainManifestPath)).toBe(true);
  expect(existsSync(platformManifestPath)).toBe(true);
  expect(mainManifest.name).toBe(adapterManifest.name);
  expect(mainManifest.version).toBe(adapterManifest.version);
  expect(mainManifest.bin).toEqual({
    [adapterManifest.name]: adapterManifest.bin,
  });
  expect(mainManifest.main).toBe(adapterManifest.main);
  expect(mainManifest.exports).toEqual(adapterManifest.exports);
  expect(mainManifest.optionalDependencies[platformPackageName]).toBe(adapterManifest.version);
  expect(platformManifest.name).toBe(platformPackageName);
  expect(platformManifest.version).toBe(adapterManifest.version);
  expect(platformManifest.os).toEqual([process.platform]);
  expect(platformManifest.cpu).toEqual([process.arch]);
  expect(existsSync(path.join(outputDir, adapterManifest.name, "bin", "codebuddy.js"))).toBe(true);
  expect(existsSync(path.join(outputDir, adapterManifest.name, "dist", "index.js"))).toBe(true);
  expect(
    existsSync(path.join(outputDir, platformPackageName, "bin", path.basename(binaryPath))),
  ).toBe(true);
  expect(staged.mainManifest.name).toBe(adapterManifest.name);
  expect(staged.platformManifest.name).toBe(platformPackageName);

  rmSync(outputDir, { recursive: true, force: true, maxRetries: 5, retryDelay: 50 });
});

test("codebuddy build writes bundle into the agent package dist directory", async () => {
  const agentDir = path.join(rootDir, "agents", "codebuddy");
  const viteConfig = (await import("../../agents/codebuddy/vite.config.mjs")).default;

  expect(path.normalize(viteConfig.root)).toBe(path.normalize(agentDir));
  expect(path.normalize(viteConfig.build.outDir)).toBe(
    path.normalize(path.join(agentDir, "dist")),
  );
});
