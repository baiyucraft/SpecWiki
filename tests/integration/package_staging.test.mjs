import assert from "node:assert/strict";
import test from "node:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

test("staging script creates platform package manifest", async () => {
  const rootDir = process.cwd();
  const outputDir = mkdtempSync(path.join(os.tmpdir(), "codebuddy-stage-"));
  const profile = "test-integration";
  const binaryDir = path.join(rootDir, "target", profile);
  const binaryName = process.platform === "win32" ? "wiki-core.exe" : "wiki-core";
  const binaryPath = path.join(binaryDir, binaryName);
  const adapterManifest = JSON.parse(
    readFileSync(path.join(rootDir, "packages", "codebuddy", "package.json"), "utf8"),
  );

  mkdirSync(binaryDir, { recursive: true });
  writeFileSync(binaryPath, "mock-binary");

  const { resolvePlatformPackageName } = await import("../../scripts/build-core.mjs");
  const { stagePackages } = await import("../../scripts/stage-binaries.mjs");
  const staged = await stagePackages({ rootDir, profile, outputDir });
  const platformPackageName = resolvePlatformPackageName(adapterManifest.name);
  const mainManifestPath = path.join(outputDir, adapterManifest.name, "package.json");
  const platformManifestPath = path.join(outputDir, platformPackageName, "package.json");
  const mainManifest = JSON.parse(readFileSync(mainManifestPath, "utf8"));
  const platformManifest = JSON.parse(readFileSync(platformManifestPath, "utf8"));

  assert.equal(existsSync(mainManifestPath), true);
  assert.equal(existsSync(platformManifestPath), true);
  assert.equal(mainManifest.name, adapterManifest.name);
  assert.equal(mainManifest.version, adapterManifest.version);
  assert.deepEqual(mainManifest.bin, adapterManifest.bin);
  assert.equal(mainManifest.optionalDependencies[platformPackageName], adapterManifest.version);
  assert.equal(platformManifest.name, platformPackageName);
  assert.equal(platformManifest.version, adapterManifest.version);
  assert.deepEqual(platformManifest.os, [process.platform]);
  assert.deepEqual(platformManifest.cpu, [process.arch]);
  assert.equal(existsSync(path.join(outputDir, adapterManifest.name, "bin", "codebuddy.js")), true);
  assert.equal(
    existsSync(path.join(outputDir, platformPackageName, "bin", path.basename(binaryPath))),
    true,
  );
  assert.equal(staged.mainManifest.name, adapterManifest.name);
  assert.equal(staged.platformManifest.name, platformPackageName);

  rmSync(outputDir, { recursive: true, force: true, maxRetries: 5, retryDelay: 50 });
});
