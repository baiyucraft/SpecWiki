import { existsSync, realpathSync } from "node:fs";
import path from "node:path";

const CHANGE_ID_PATTERN = /^[a-z0-9]+(?:-[a-z0-9]+)*$/u;

export class PathSafetyError extends Error {
  readonly code = "unsafe_path";

  constructor(message: string) {
    super(`unsafe path: ${message}`);
    this.name = "PathSafetyError";
  }
}

function isWithin(root: string, target: string): boolean {
  const relative = path.relative(root, target);
  return relative === "" || (!relative.startsWith("..") && !path.isAbsolute(relative));
}

function nearestExistingPath(target: string): string {
  let current = target;
  while (!existsSync(current)) {
    const parent = path.dirname(current);
    if (parent === current) {
      throw new PathSafetyError(`no existing ancestor for ${target}`);
    }
    current = parent;
  }
  return current;
}

export function assertCanonicalChangeId(changeId: string): void {
  if (!CHANGE_ID_PATTERN.test(changeId)) {
    throw new PathSafetyError(`change id must be canonical kebab-case: ${changeId}`);
  }
}

export function resolveSafePath(projectRoot: string, relativePath: string): string {
  if (relativePath.length === 0 || relativePath.includes("\0")) {
    throw new PathSafetyError("path must be a non-empty relative path");
  }
  if (path.isAbsolute(relativePath) || path.win32.isAbsolute(relativePath) || path.posix.isAbsolute(relativePath)) {
    throw new PathSafetyError(`absolute paths are not allowed: ${relativePath}`);
  }
  const segments = relativePath.replaceAll("\\", "/").split("/");
  if (segments.includes("..")) {
    throw new PathSafetyError(`parent traversal is not allowed: ${relativePath}`);
  }

  const root = path.resolve(projectRoot);
  const target = path.resolve(root, relativePath);
  if (!isWithin(root, target)) {
    throw new PathSafetyError(`target escapes project root: ${relativePath}`);
  }

  const existingRoot = nearestExistingPath(root);
  const realRoot = path.resolve(realpathSync(existingRoot), path.relative(existingRoot, root));
  const existingTarget = nearestExistingPath(target);
  const realTarget = path.resolve(realpathSync(existingTarget), path.relative(existingTarget, target));
  if (!isWithin(realRoot, realTarget)) {
    throw new PathSafetyError(`symlink target escapes project root: ${relativePath}`);
  }
  return target;
}
