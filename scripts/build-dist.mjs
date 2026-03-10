import { cpSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";
import { spawn } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";

import {
  resolveBuiltBinary,
  resolvePlatformManifestConstraints,
  resolvePlatformPackageName,
} from "./build/core-paths.mjs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const DEFAULT_ROOT_DIR = path.resolve(__dirname, "..");
const AGENT_PACKAGE_DIR = path.join("agents", "codebuddy");
const AGENT_PACKAGE_PATH = path.join(AGENT_PACKAGE_DIR, "package.json");

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

function buildMainManifest(sourceManifest, platformPackageName) {
  // The staged main package should stay as close as possible to the child package manifest,
  // but only production-facing fields belong in the publish output.
  const normalizedBin = typeof sourceManifest.bin === "string"
    ? { [sourceManifest.name]: sourceManifest.bin.replace(/^\.\//, "") }
    : sourceManifest.bin;

  return {
    name: sourceManifest.name,
    version: sourceManifest.version,
    private: false,
    type: sourceManifest.type,
    description: sourceManifest.description,
    main: sourceManifest.main,
    exports: sourceManifest.exports,
    files: sourceManifest.files,
    bin: normalizedBin,
    optionalDependencies: {
      ...(sourceManifest.optionalDependencies ?? {}),
      [platformPackageName]: sourceManifest.version,
    },
  };
}

function buildPlatformManifest(templatePath, { packageName, version, platform, arch }) {
  const template = JSON.parse(
    readFileSync(templatePath, "utf8").replaceAll("__PACKAGE_NAME__", packageName),
  );

  return {
    ...template,
    version,
    ...resolvePlatformManifestConstraints({ platform, arch }),
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

function copyMainPackageAssets(rootDir, mainDir, sourceManifest) {
  // The staged main package must contain the built Agent bundle as well as its executable shim.
  for (const relativeEntry of resolvePublishAssetEntries(sourceManifest)) {
    const sourcePath = path.join(rootDir, AGENT_PACKAGE_DIR, relativeEntry);
    const targetPath = path.join(mainDir, relativeEntry);

    if (!existsSync(sourcePath)) {
      throw new Error(`required publish asset not found at ${sourcePath}`);
    }

    mkdirSync(path.dirname(targetPath), { recursive: true });
    cpSync(sourcePath, targetPath, { recursive: true });
  }
}

export function collectCoreBinary({
  rootDir = DEFAULT_ROOT_DIR,
  profile = "debug",
  platform = process.platform,
  outputDir = path.join(rootDir, "dist", "core"),
} = {}) {
  const binaryPath = resolveBuiltBinary(rootDir, { profile, platform });

  rmSync(outputDir, { recursive: true, force: true, maxRetries: 5, retryDelay: 50 });
  mkdirSync(outputDir, { recursive: true });

  const stagedBinaryPath = path.join(outputDir, path.basename(binaryPath));
  cpSync(binaryPath, stagedBinaryPath);

  return {
    binaryPath,
    stagedBinaryPath,
    outputDir,
  };
}

export async function stagePackages({
  rootDir = DEFAULT_ROOT_DIR,
  profile = "debug",
  platform = process.platform,
  arch = process.arch,
  outputDir = path.join(rootDir, "dist", "npm"),
} = {}) {
  const sourceManifest = loadJson(path.join(rootDir, AGENT_PACKAGE_PATH));
  const mainPackageName = sourceManifest.name;
  const platformPackageName = resolvePlatformPackageName(mainPackageName, {
    platform,
    arch,
  });
  const mainDir = path.join(outputDir, mainPackageName);
  const platformDir = path.join(outputDir, platformPackageName);
  const binaryPath = resolveBuiltBinary(rootDir, { profile, platform });
  const templatePath = path.join(rootDir, "scripts", "templates", "platform-package.json");

  rmSync(outputDir, { recursive: true, force: true, maxRetries: 5, retryDelay: 50 });
  mkdirSync(path.join(mainDir, "bin"), { recursive: true });
  mkdirSync(path.join(platformDir, "bin"), { recursive: true });

  const mainManifest = buildMainManifest(sourceManifest, platformPackageName);
  const platformManifest = buildPlatformManifest(templatePath, {
    packageName: platformPackageName,
    version: sourceManifest.version,
    platform,
    arch,
  });

  writeFileSync(
    path.join(mainDir, "package.json"),
    `${JSON.stringify(mainManifest, null, 2)}\n`,
  );
  copyMainPackageAssets(rootDir, mainDir, sourceManifest);
  writeFileSync(
    path.join(platformDir, "package.json"),
    `${JSON.stringify(platformManifest, null, 2)}\n`,
  );
  cpSync(binaryPath, path.join(platformDir, "bin", path.basename(binaryPath)));

  return {
    distDir: outputDir,
    binaryPath,
    mainDir,
    platformDir,
    mainManifest,
    platformManifest,
  };
}

export async function buildDistribution({ rootDir = DEFAULT_ROOT_DIR, profile = "debug" } = {}) {
  const agentRootDir = path.join(rootDir, AGENT_PACKAGE_DIR);

  rmSync(path.join(rootDir, "dist"), {
    recursive: true,
    force: true,
    maxRetries: 5,
    retryDelay: 50,
  });

  await runCommand("cargo", ["build", "-p", "wiki-core", "--target-dir", "target"], {
    cwd: rootDir,
  });
  await runCommand("pnpm", ["build"], { cwd: agentRootDir });

  const core = collectCoreBinary({ rootDir, profile });
  const npm = await stagePackages({ rootDir, profile });

  return { core, npm };
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const built = await buildDistribution();

  console.log(`Collected core binary in ${built.core.outputDir}`);
  console.log(`Staged npm packages in ${built.npm.distDir}`);
  console.log(`Main package: ${built.npm.mainManifest.name}@${built.npm.mainManifest.version}`);
  console.log(`Platform package: ${built.npm.platformManifest.name}`);
}
