/**
 * 这个脚本负责构建 spec-wiki 的单包发布产物。
 * 它收口主包 staging、Windows runtime 复制与发布资产整理，不承载 Wiki 业务逻辑。
 */
import { cpSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";
import { spawn } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";

import { resolveBuiltBinary } from "./build/core-paths.mjs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const DEFAULT_ROOT_DIR = path.resolve(__dirname, "..");
const MAIN_PACKAGE_DIR = path.join("packages", "spec-wiki");
const MAIN_PACKAGE_PATH = path.join(MAIN_PACKAGE_DIR, "package.json");
const STAGED_PACKAGE_DIR = path.join("dist", "spec-wiki");
const STAGED_RUNTIME_DIR = path.join("lib", "x64-win32");
const README_PATH = "README.md";
const REMOVE_RETRY_DELAY_MS = 500;
const REMOVE_RETRY_ATTEMPTS = 40;

function loadJson(filePath) {
  return JSON.parse(readFileSync(filePath, "utf8"));
}

async function runCommand(command, args, { cwd = DEFAULT_ROOT_DIR } = {}) {
  // Root orchestration scripts call sub-builds explicitly so each layer keeps a single responsibility.
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd,
      stdio: "inherit",
      shell: process.platform === "win32",
    });

    child.on("error", reject);
    child.on("close", (code) => {
      if (code === 0) {
        resolve();
        return;
      }

      reject(new Error(`${command} ${args.join(" ")} exited with code ${code}`));
    });
  });
}

function sleepSync(ms) {
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
}

function isTransientFsErrorMessage(message) {
  const normalized = String(message ?? "");
  return (
    ["EBUSY", "EPERM", "ENOTEMPTY"].some((code) => normalized.includes(code))
    || normalized.includes("os error 32")
  );
}

function removePathWithRetry(
  targetPath,
  { delayMs = REMOVE_RETRY_DELAY_MS, maxAttempts = REMOVE_RETRY_ATTEMPTS } = {},
) {
  if (!existsSync(targetPath)) {
    return;
  }

  let lastError = null;
  for (let attempt = 0; attempt < maxAttempts; attempt += 1) {
    try {
      rmSync(targetPath, { recursive: true, force: true });
      return;
    } catch (error) {
      lastError = error;
      if (!isTransientFsErrorMessage(error?.code || error?.message || "")) {
        throw error;
      }
      sleepSync(delayMs);
    }
  }

  throw lastError;
}

function buildMainManifest(sourceManifest) {
  const normalizedBin = typeof sourceManifest.bin === "string"
    ? { [sourceManifest.name]: sourceManifest.bin.replace(/^\.\//, "") }
    : sourceManifest.bin;
  const fileEntries = new Set(sourceManifest.files ?? []);

  fileEntries.add("lib/**");

  return {
    name: sourceManifest.name,
    version: sourceManifest.version,
    private: false,
    type: sourceManifest.type,
    description: sourceManifest.description,
    license: sourceManifest.license,
    os: sourceManifest.os,
    cpu: sourceManifest.cpu,
    main: sourceManifest.main,
    exports: sourceManifest.exports,
    files: [...fileEntries],
    bin: normalizedBin,
  };
}

function resolvePublishAssetEntries(sourceManifest) {
  const publishEntries = new Set(["bin", "dist"]);

  for (const pattern of sourceManifest.files ?? []) {
    const normalizedPattern = pattern.replaceAll("\\", "/");
    const topLevelEntry = normalizedPattern.replace(/\/\*\*$/, "").replace(/\/\*$/, "");

    if (topLevelEntry && !topLevelEntry.includes("*")) {
      publishEntries.add(topLevelEntry);
    }
  }

  return [...publishEntries];
}

function copyMainPackageAssets(rootDir, stagedPackageDir, sourceManifest) {
  // The staged package must contain the CLI bundle, executable shim, and bootstrap assets.
  for (const relativeEntry of resolvePublishAssetEntries(sourceManifest)) {
    const sourcePath = path.join(rootDir, MAIN_PACKAGE_DIR, relativeEntry);
    const targetPath = path.join(stagedPackageDir, relativeEntry);

    if (!existsSync(sourcePath)) {
      throw new Error(`required publish asset not found at ${sourcePath}`);
    }

    mkdirSync(path.dirname(targetPath), { recursive: true });
    cpSync(sourcePath, targetPath, { recursive: true });
  }
}

export function stagePackage({
  rootDir = DEFAULT_ROOT_DIR,
  profile = "release",
  platform = process.platform,
  outputDir = path.join(rootDir, STAGED_PACKAGE_DIR),
} = {}) {
  if (platform !== "win32") {
    throw new Error(`single-package staging currently only supports win32, got ${platform}`);
  }

  const sourceManifest = loadJson(path.join(rootDir, MAIN_PACKAGE_PATH));
  const binaryPath = resolveBuiltBinary(rootDir, { profile, platform });
  const stagedRuntimeDir = path.join(outputDir, STAGED_RUNTIME_DIR);
  const stagedBinaryPath = path.join(stagedRuntimeDir, path.basename(binaryPath));
  const stagedManifest = buildMainManifest(sourceManifest);

  removePathWithRetry(outputDir);
  mkdirSync(stagedRuntimeDir, { recursive: true });

  writeFileSync(
    path.join(outputDir, "package.json"),
    `${JSON.stringify(stagedManifest, null, 2)}\n`,
  );
  copyMainPackageAssets(rootDir, outputDir, sourceManifest);
  cpSync(path.join(rootDir, "LICENSE"), path.join(outputDir, "LICENSE"));
  cpSync(path.join(rootDir, README_PATH), path.join(outputDir, README_PATH));
  cpSync(binaryPath, stagedBinaryPath);

  return {
    packageDir: outputDir,
    binaryPath,
    stagedBinaryPath,
    manifest: stagedManifest,
  };
}

export async function buildDistribution({ rootDir = DEFAULT_ROOT_DIR, profile = "release" } = {}) {
  const mainPackageRootDir = path.join(rootDir, MAIN_PACKAGE_DIR);

  removePathWithRetry(path.join(rootDir, "dist"));

  await runCommand("cargo", ["build", "-p", "wiki-runtime", "--release", "--target-dir", "target"], {
    cwd: rootDir,
  });
  await runCommand("pnpm", ["build"], { cwd: mainPackageRootDir });

  const pkg = stagePackage({ rootDir, profile });
  return { package: pkg };
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const built = await buildDistribution();

  console.log(`Staged package in ${built.package.packageDir}`);
  console.log(`Runtime binary: ${built.package.stagedBinaryPath}`);
  console.log(`Package: ${built.package.manifest.name}@${built.package.manifest.version}`);
}
