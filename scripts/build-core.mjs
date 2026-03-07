import path from "node:path";
import { existsSync } from "node:fs";

const PLATFORM_ABI_SUFFIX = {
  win32: "msvc",
  linux: "gnu",
};

export function getCoreBinaryName(platform = process.platform) {
  return platform === "win32" ? "wiki-core.exe" : "wiki-core";
}

export function getBuildProfileDir(rootDir, { profile = "debug" } = {}) {
  return path.join(rootDir, "target", profile);
}

export function resolveBuiltBinary(
  rootDir,
  {
    profile = "debug",
    platform = process.platform,
    targetDir = path.join(rootDir, "target"),
  } = {},
) {
  const binaryName = getCoreBinaryName(platform);
  const binaryPath = path.join(targetDir, profile, binaryName);

  if (!existsSync(binaryPath)) {
    throw new Error(`wiki-core binary not found at ${binaryPath}`);
  }

  return binaryPath;
}

export function resolvePlatformPackageSuffix({
  platform = process.platform,
  arch = process.arch,
} = {}) {
  if (platform === "darwin") {
    return `${platform}-${arch}`;
  }

  const abi = PLATFORM_ABI_SUFFIX[platform];
  if (abi) {
    return `${platform}-${arch}-${abi}`;
  }

  return `${platform}-${arch}`;
}

export function resolvePlatformPackageName(mainPackageName, options = {}) {
  return `${mainPackageName}-${resolvePlatformPackageSuffix(options)}`;
}

export function resolvePlatformManifestConstraints({
  platform = process.platform,
  arch = process.arch,
} = {}) {
  return {
    os: [platform],
    cpu: [arch],
  };
}
