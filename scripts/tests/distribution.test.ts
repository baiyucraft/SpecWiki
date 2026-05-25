/**
 * 这个文件覆盖 spec-wiki 的单包分发骨架和 staging 结果。
 * 它确保 dist 发布真相已经收口到 `dist/spec-wiki`。
 */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

import { runCli } from "../../packages/spec-wiki/src/cli.ts";
import { PUBLIC_WIKI_ACTIONS } from "../../packages/spec-wiki/src/wikiActions.ts";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..", "..");
const mainPackageDir = path.join(rootDir, "packages", "spec-wiki");

function buildMainPackageBundle() {
  const result = spawnSync("pnpm", ["build"], {
    cwd: mainPackageDir,
    encoding: "utf8",
    shell: process.platform === "win32",
  });

  expect(result.status).toBe(0);
}

async function renderSourceHelp() {
  const stdout = [];
  const stderr = [];
  const exitCode = await runCli(["--help"], {
    cwd: rootDir,
    env: process.env,
    stdin: process.stdin,
    stdout: (text) => stdout.push(text),
    stderr: (text) => stderr.push(text),
  });

  expect(stderr.join("")).toBe("");
  expect(exitCode).toBe(0);
  return stdout.join("").trim();
}

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

  buildMainPackageBundle();

  const mainManifestSource = JSON.parse(
    readFileSync(path.join(rootDir, "packages", "spec-wiki", "package.json"), "utf8"),
  );
  const sourceHelp = await renderSourceHelp();

  mkdirSync(binaryDir, { recursive: true });
  writeFileSync(binaryPath, "mock-binary");

  const { assertStagedPackageEvidence, collectStagedPackageEvidence, stagePackage }
    = await import("../build-dist.mjs");
  const staged = stagePackage({ rootDir, profile, outputDir, platform: stagePlatform });
  const stagedManifestPath = path.join(outputDir, "package.json");
  const stagedManifest = JSON.parse(readFileSync(stagedManifestPath, "utf8"));
  const stagedBinaryPath = path.join(outputDir, "lib", "x64-win32", path.basename(binaryPath));
  const evidence = await collectStagedPackageEvidence({ rootDir, packageDir: outputDir });

  expect(existsSync(stagedManifestPath)).toBe(true);
  expect(stagedManifest.name).toBe(mainManifestSource.name);
  expect(stagedManifest.version).toBe(mainManifestSource.version);
  expect(stagedManifest.description).toBe(mainManifestSource.description);
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
  expect(evidence.checks.nameMatch).toBe(true);
  expect(evidence.checks.versionMatch).toBe(true);
  expect(evidence.checks.descriptionMatch).toBe(true);
  expect(evidence.checks.binMatch).toBe(true);
  expect(evidence.checks.mainMatch).toBe(true);
  expect(evidence.checks.exportsMatch).toBe(true);
  expect(evidence.checks.osMatch).toBe(true);
  expect(evidence.checks.cpuMatch).toBe(true);
  expect(evidence.checks.readmeMatch).toBe(true);
  expect(evidence.checks.helpMatch).toBe(true);
  expect(evidence.sourceHelpActions).toEqual([...PUBLIC_WIKI_ACTIONS]);
  expect(evidence.stagedHelpActions).toEqual([...PUBLIC_WIKI_ACTIONS]);
  expect(evidence.stagedHelpText.trim()).toBe(sourceHelp);
  expect(evidence.stagedHelpText).toContain("Supported actions: init, status, update, query, sync, rebuild");
  expect(evidence.stagedHelpText).toContain(
    "--bridge-stdio only applies to long-running wiki actions such as init, update, and rebuild",
  );
  expect(evidence.stagedHelpText).not.toContain("unsupported action");
  assertStagedPackageEvidence(evidence);

  rmSync(outputDir, { recursive: true, force: true, maxRetries: 5, retryDelay: 50 });
});

test("staged CLI help matches the source help contract", async () => {
  const outputDir = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-stage-help-"));
  const profile = "test-integration";
  const binaryDir = path.join(rootDir, "target", profile);
  const binaryPath = path.join(binaryDir, "wiki-runtime.exe");

  buildMainPackageBundle();
  const sourceHelp = await renderSourceHelp();

  mkdirSync(binaryDir, { recursive: true });
  writeFileSync(binaryPath, "mock-binary");

  try {
    const { stagePackage } = await import("../build-dist.mjs");
    stagePackage({ rootDir, profile, outputDir, platform: "win32" });

    const stagedHelp = spawnSync(
      "node",
      [path.join(outputDir, "bin", "spec-wiki.js"), "--help"],
      { cwd: outputDir, encoding: "utf8", shell: process.platform === "win32" },
    );

    expect(stagedHelp.status).toBe(0);
    expect(stagedHelp.stdout.trim()).toBe(sourceHelp);
  } finally {
    rmSync(outputDir, { recursive: true, force: true, maxRetries: 5, retryDelay: 50 });
  }
});

test("spec-wiki build writes bundle into the main package dist directory", async () => {
  const viteConfig = (await import("../../packages/spec-wiki/vite.config.mjs")).default;

  expect(path.normalize(viteConfig.root)).toBe(path.normalize(mainPackageDir));
  expect(path.normalize(viteConfig.build.outDir)).toBe(
    path.normalize(path.join(mainPackageDir, "dist")),
  );
});
