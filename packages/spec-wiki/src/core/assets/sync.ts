import { existsSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";
import { randomUUID } from "node:crypto";

import { readPackageAsset } from "../../packageRoot.js";
import { resolveSafePath } from "../path.js";
import { PROJECT_ASSETS, PROJECT_DIRECTORIES, type ProjectAsset } from "./registry.js";

export type AssetSyncOptions = {
  force?: boolean;
};

export type AssetSyncReport = {
  created: string[];
  updated: string[];
  unchanged: string[];
  preserved: string[];
};

function toProjectPath(value: string): string {
  return value.replaceAll("\\", "/");
}

function atomicWrite(target: string, content: string): void {
  mkdirSync(path.dirname(target), { recursive: true });
  const temporary = path.join(
    path.dirname(target),
    `.${path.basename(target)}.${randomUUID()}.tmp`,
  );
  try {
    writeFileSync(temporary, content, "utf8");
    renameSync(temporary, target);
  } finally {
    rmSync(temporary, { force: true });
  }
}

function shouldUpdate(asset: ProjectAsset, force: boolean): boolean {
  return asset.ownership === "skill" || (asset.ownership === "managed" && force);
}

function listFiles(root: string): string[] {
  if (!existsSync(root)) {
    return [];
  }
  return readdirSync(root, { recursive: true, withFileTypes: true })
    .filter(entry => entry.isFile())
    .map(entry => path.join(entry.parentPath, entry.name));
}

export async function syncProjectAssets(
  projectRoot: string,
  options: AssetSyncOptions = {},
): Promise<AssetSyncReport> {
  const root = path.resolve(projectRoot);
  const report: AssetSyncReport = {
    created: [],
    updated: [],
    unchanged: [],
    preserved: [],
  };

  for (const directory of PROJECT_DIRECTORIES) {
    mkdirSync(resolveSafePath(root, directory), { recursive: true });
  }

  const registeredTargets = new Set(PROJECT_ASSETS.map(asset => asset.target));
  for (const file of listFiles(path.join(root, ".wiki"))) {
    const relative = toProjectPath(path.relative(root, file));
    if (!registeredTargets.has(relative)) {
      report.preserved.push(relative);
    }
  }

  for (const asset of PROJECT_ASSETS) {
    const target = resolveSafePath(root, asset.target);
    const content = readPackageAsset(asset.source);
    if (!existsSync(target)) {
      atomicWrite(target, content);
      report.created.push(asset.target);
      continue;
    }

    const previous = readFileSync(target, "utf8");
    if (previous === content) {
      report.unchanged.push(asset.target);
      continue;
    }

    if (shouldUpdate(asset, options.force === true)) {
      atomicWrite(target, content);
      report.updated.push(asset.target);
    } else {
      report.preserved.push(asset.target);
    }
  }

  for (const values of Object.values(report)) {
    values.sort();
  }
  return report;
}
