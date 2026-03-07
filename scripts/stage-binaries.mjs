import { cpSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";

import {
  resolveBuiltBinary,
  resolvePlatformManifestConstraints,
  resolvePlatformPackageName,
} from "./build-core.mjs";

const ADAPTER_PACKAGE_PATH = path.join("packages", "codebuddy", "package.json");

function loadJson(filePath) {
  return JSON.parse(readFileSync(filePath, "utf8"));
}

function buildMainManifest(sourceManifest, platformPackageName) {
  return {
    name: sourceManifest.name,
    version: sourceManifest.version,
    private: false,
    type: sourceManifest.type,
    description: sourceManifest.description,
    bin: sourceManifest.bin,
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

function copyMainPackageBins(rootDir, mainDir, sourceManifest) {
  for (const relativePath of Object.values(sourceManifest.bin ?? {})) {
    const sourcePath = path.join(rootDir, "packages", "codebuddy", relativePath);
    const targetPath = path.join(mainDir, relativePath);
    mkdirSync(path.dirname(targetPath), { recursive: true });
    cpSync(sourcePath, targetPath);
  }
}

export async function stagePackages({
  rootDir,
  profile = "debug",
  platform = process.platform,
  arch = process.arch,
  outputDir = path.join(rootDir, "dist", "npm"),
} = {}) {
  const sourceManifest = loadJson(path.join(rootDir, ADAPTER_PACKAGE_PATH));
  const mainPackageName = sourceManifest.name;
  const platformPackageName = resolvePlatformPackageName(mainPackageName, {
    platform,
    arch,
  });
  const distDir = outputDir;
  const mainDir = path.join(outputDir, mainPackageName);
  const platformDir = path.join(outputDir, platformPackageName);
  const binaryPath = resolveBuiltBinary(rootDir, { profile, platform });
  const templatePath = path.join(
    rootDir,
    "packages",
    "codebuddy",
    "templates",
    "platform-package.json",
  );

  rmSync(distDir, { recursive: true, force: true, maxRetries: 5, retryDelay: 50 });
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
  copyMainPackageBins(rootDir, mainDir, sourceManifest);
  writeFileSync(
    path.join(platformDir, "package.json"),
    `${JSON.stringify(platformManifest, null, 2)}\n`,
  );
  cpSync(binaryPath, path.join(platformDir, "bin", path.basename(binaryPath)));

  return {
    distDir,
    binaryPath,
    mainDir,
    platformDir,
    mainManifest,
    platformManifest,
  };
}
