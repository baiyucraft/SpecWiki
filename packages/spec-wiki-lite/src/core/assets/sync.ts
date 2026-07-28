import {
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  renameSync,
  rmdirSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import path from "node:path";
import { randomUUID } from "node:crypto";

import { readPackageAsset } from "../../packageRoot.js";
import {
  PROJECT_CONFIG_PATH,
  projectConfigContent,
  readProjectConfig,
  type WikiLanguage,
  writeProjectLanguage,
} from "../config.js";
import { resolveSafePath } from "../path.js";
import {
  LEGACY_EN_V0_ASSETS,
  PROJECT_DIRECTORIES,
  type ProjectAsset,
  projectAssetsForLanguage,
  projectWikiAssetsForLanguage,
} from "./registry.js";

export type AssetSyncOptions = {
  force?: boolean;
  language?: WikiLanguage;
};

export type AssetSyncReport = {
  created: string[];
  updated: string[];
  unchanged: string[];
  preserved: string[];
  removed: string[];
};

export type AssetSyncOperation
  = | { kind: "write"; path: string; content: string }
    | { kind: "remove"; path: string };

export type AssetSyncTestHooks = {
  beforeOperation?: (operation: AssetSyncOperation, index: number) => void;
};

type FileSnapshot = {
  existed: boolean;
  content?: string;
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

function hasLanguageSpecificAsset(projectRoot: string, assets: readonly ProjectAsset[]): boolean {
  return assets
    .filter(asset => asset.target !== ".wiki/INDEX.md")
    .some(asset => existsSync(resolveSafePath(projectRoot, asset.target)));
}

function hasLegacyEnglishProject(projectRoot: string): boolean {
  return existsSync(resolveSafePath(projectRoot, ".wiki/01-project/INDEX.md"));
}

function migrationSource(
  projectRoot: string,
  targetLanguage: WikiLanguage,
): { name: string; assets: readonly ProjectAsset[] } | undefined {
  const otherLanguage: WikiLanguage = targetLanguage === "zh" ? "en" : "zh";
  const candidates: Array<{ name: string; assets: readonly ProjectAsset[] }> = [];
  const otherAssets = projectWikiAssetsForLanguage(otherLanguage);
  if (hasLanguageSpecificAsset(projectRoot, otherAssets)) {
    candidates.push({ name: otherLanguage, assets: otherAssets });
  }
  if (hasLegacyEnglishProject(projectRoot)) {
    candidates.push({ name: "legacy-en-v0", assets: LEGACY_EN_V0_ASSETS });
  }
  if (candidates.length > 1) {
    throw new Error(`migration conflict: multiple Wiki asset sources detected: ${candidates.map(item => item.name).join(", ")}`);
  }
  return candidates[0];
}

function snapshotFiles(paths: Iterable<string>): Map<string, FileSnapshot> {
  const snapshots = new Map<string, FileSnapshot>();
  for (const file of paths) {
    snapshots.set(file, existsSync(file)
      ? { existed: true, content: readFileSync(file, "utf8") }
      : { existed: false });
  }
  return snapshots;
}

function restoreFiles(snapshots: Map<string, FileSnapshot>): void {
  for (const [file, snapshot] of [...snapshots.entries()].reverse()) {
    if (snapshot.existed) {
      atomicWrite(file, snapshot.content!);
    } else {
      rmSync(file, { force: true });
    }
  }
}

function cleanEmptySourceDirectories(projectRoot: string, sourceFiles: string[]): void {
  const wikiRoot = resolveSafePath(projectRoot, ".wiki");
  const directories = [...new Set(sourceFiles.map(file => path.dirname(file)))]
    .sort((left, right) => right.length - left.length);
  for (const startingDirectory of directories) {
    let directory = startingDirectory;
    while (directory !== wikiRoot && path.relative(wikiRoot, directory) !== "") {
      try {
        rmdirSync(directory);
      } catch {
        break;
      }
      directory = path.dirname(directory);
    }
  }
}

function preflightMigration(
  projectRoot: string,
  sourceAssets: readonly ProjectAsset[],
  targetAssets: readonly ProjectAsset[],
  force: boolean,
): void {
  const conflicts: string[] = [];
  const sourceTargets = new Set(sourceAssets.map(asset => asset.target));

  for (const asset of sourceAssets) {
    const source = resolveSafePath(projectRoot, asset.target);
    if (!existsSync(source)) {
      continue;
    }
    const actual = readFileSync(source, "utf8");
    const expected = readPackageAsset(asset.source);
    if (actual !== expected && !(asset.ownership === "managed" && force)) {
      conflicts.push(asset.target);
    }
  }

  for (const asset of targetAssets) {
    if (sourceTargets.has(asset.target)) {
      continue;
    }
    const target = resolveSafePath(projectRoot, asset.target);
    if (!existsSync(target)) {
      continue;
    }
    const actual = readFileSync(target, "utf8");
    const expected = readPackageAsset(asset.source);
    if (actual !== expected && !(asset.ownership === "managed" && force)) {
      conflicts.push(asset.target);
    }
  }

  if (conflicts.length > 0) {
    throw new Error(`migration conflict: ${[...new Set(conflicts)].sort().join(", ")}`);
  }
}

export async function syncProjectAssets(
  projectRoot: string,
  options: AssetSyncOptions = {},
  hooks: AssetSyncTestHooks = {},
): Promise<AssetSyncReport> {
  const root = path.resolve(projectRoot);
  const currentConfig = readProjectConfig(root);
  const language = options.language ?? currentConfig.language;
  const force = options.force === true;
  const targetAssets = projectAssetsForLanguage(language);
  const targetWikiAssets = projectWikiAssetsForLanguage(language);
  let source = migrationSource(root, language);
  const report: AssetSyncReport = {
    created: [],
    updated: [],
    unchanged: [],
    preserved: [],
    removed: [],
  };

  if (source) {
    try {
      preflightMigration(root, source.assets, targetWikiAssets, force);
    } catch (error) {
      // An existing English config makes modified legacy pages user-owned instead of migration inputs.
      const preserveLegacyEnglish = currentConfig.exists
        && language === "en"
        && source.name === "legacy-en-v0"
        && error instanceof Error
        && error.message.startsWith("migration conflict:");
      if (!preserveLegacyEnglish) {
        throw error;
      }
      source = undefined;
    }
  }

  for (const directory of PROJECT_DIRECTORIES) {
    mkdirSync(resolveSafePath(root, directory), { recursive: true });
  }

  const sourceTargets = new Set(source?.assets.map(asset => asset.target) ?? []);
  const registeredTargets = new Set([
    PROJECT_CONFIG_PATH,
    ...targetAssets.map(asset => asset.target),
    ...sourceTargets,
  ]);
  for (const file of listFiles(path.join(root, ".wiki"))) {
    const relative = toProjectPath(path.relative(root, file));
    if (!registeredTargets.has(relative)) {
      report.preserved.push(relative);
    }
  }

  const operations: AssetSyncOperation[] = [];
  const removedSourceFiles: string[] = [];
  if (source) {
    for (const asset of source.assets) {
      const target = resolveSafePath(root, asset.target);
      if (!existsSync(target)) {
        continue;
      }
      operations.push({ kind: "remove", path: target });
      removedSourceFiles.push(target);
      if (!targetWikiAssets.some(current => current.target === asset.target)) {
        report.removed.push(asset.target);
      }
    }
  }

  for (const asset of targetAssets) {
    const target = resolveSafePath(root, asset.target);
    const content = readPackageAsset(asset.source);
    const sourceWillRemoveTarget = sourceTargets.has(asset.target) && existsSync(target);
    if (!existsSync(target) || sourceWillRemoveTarget) {
      operations.push({ kind: "write", path: target, content });
      (sourceWillRemoveTarget ? report.updated : report.created).push(asset.target);
      continue;
    }

    const previous = readFileSync(target, "utf8");
    if (previous === content) {
      report.unchanged.push(asset.target);
      continue;
    }

    if (shouldUpdate(asset, force)) {
      operations.push({ kind: "write", path: target, content });
      report.updated.push(asset.target);
    } else {
      report.preserved.push(asset.target);
    }
  }

  const config = projectConfigContent(root, language);
  report[config.outcome].push(PROJECT_CONFIG_PATH);
  const configPath = resolveSafePath(root, PROJECT_CONFIG_PATH);
  const touchedPaths = new Set(operations.map(operation => operation.path));
  if (config.outcome !== "unchanged") {
    touchedPaths.add(configPath);
  }
  const snapshots = snapshotFiles(touchedPaths);

  try {
    for (const [index, operation] of operations.entries()) {
      hooks.beforeOperation?.(operation, index);
      if (operation.kind === "remove") {
        rmSync(operation.path, { force: true });
      } else {
        atomicWrite(operation.path, operation.content);
      }
    }
    if (config.outcome !== "unchanged") {
      writeProjectLanguage(root, language);
    }
  } catch (error) {
    restoreFiles(snapshots);
    throw error;
  }

  if (source) {
    cleanEmptySourceDirectories(root, removedSourceFiles);
  }
  for (const values of Object.values(report)) {
    values.sort();
  }
  return report;
}
