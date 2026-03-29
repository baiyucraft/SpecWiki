import { existsSync } from "node:fs";
import path from "node:path";

export function getCoreBinaryName(platform = process.platform) {
  return platform === "win32" ? "wiki-runtime.exe" : "wiki-runtime";
}

export function getCoreTargetDir(rootDir) {
  // The Rust workspace shares a single root target directory so every build/test entry resolves the same binary.
  return path.join(rootDir, "target");
}

export function resolveBuiltBinary(
  rootDir,
  {
    profile = "debug",
    platform = process.platform,
    targetDir = getCoreTargetDir(rootDir),
  } = {},
) {
  const binaryName = getCoreBinaryName(platform);
  const binaryPath = path.join(targetDir, profile, binaryName);

  if (!existsSync(binaryPath)) {
    throw new Error(`wiki-runtime binary not found at ${binaryPath}`);
  }

  return binaryPath;
}
