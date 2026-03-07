import path from "node:path";
import { existsSync } from "node:fs";

export function resolveBuiltBinary(rootDir) {
  const binaryName = process.platform === "win32" ? "wiki-core.exe" : "wiki-core";
  const binaryPath = path.join(rootDir, "target", "debug", binaryName);

  if (!existsSync(binaryPath)) {
    throw new Error(`wiki-core binary not found at ${binaryPath}`);
  }

  return binaryPath;
}
