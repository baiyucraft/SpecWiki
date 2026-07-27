import { randomUUID } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";

import { parseDocument } from "yaml";

import { resolveSafePath } from "../path.js";
import { changeDirectory } from "./metadata.js";
import { validateChange } from "./validate.js";

export type ArchiveClock = () => Date;

export type ArchiveOptions = {
  writeParentFile?: (filePath: string, content: string) => void;
};

export class ChangeNotReadyError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "ChangeNotReadyError";
  }
}

export type ChangeArchiveReport = {
  id: string;
  archivedAt: string;
  archivedTo: string;
};

type ParentUpdate = {
  metaPath: string;
  splitPath: string;
  originalMeta: string;
  originalSplit: string;
  updatedMeta: string;
  updatedSplit: string;
};

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function stringArray(value: unknown): string[] | undefined {
  return Array.isArray(value) && value.every(item => typeof item === "string")
    ? value
    : undefined;
}

function sameStrings(left: string[] | undefined, right: string[] | undefined): boolean {
  return JSON.stringify(left ?? []) === JSON.stringify(right ?? []);
}

function atomicWrite(filePath: string, content: string): void {
  const temporary = path.join(path.dirname(filePath), `.${path.basename(filePath)}.${randomUUID()}.tmp`);
  try {
    writeFileSync(temporary, content, "utf8");
    renameSync(temporary, filePath);
  } finally {
    rmSync(temporary, { force: true });
  }
}

function splitSection(content: string, childId: string, order: number): {
  end: number;
  lines: string[];
  newline: string;
  start: number;
} {
  const newline = content.includes("\r\n") ? "\r\n" : "\n";
  const lines = content.replaceAll("\r\n", "\n").split("\n");
  const heading = `### ${order}. ${childId}`;
  const start = lines.findIndex(line => line.trimEnd() === heading);
  if (start < 0) {
    throw new ChangeNotReadyError(`parent split is missing child heading: ${heading}`);
  }
  const next = lines.findIndex((line, index) => index > start && line.startsWith("### "));
  const end = next < 0 ? lines.length : next;
  return { end, lines, newline, start };
}

function updateSplitMarker(content: string, childId: string, order: number): string {
  const { end, lines, newline, start } = splitSection(content, childId, order);
  const marker = lines.findIndex((line, index) => index > start && index < end && line === "- 归档状态：[ ] pending");
  if (marker < 0) {
    throw new ChangeNotReadyError(`parent split is missing pending archive marker for ${childId}`);
  }
  lines[marker] = "- 归档状态：[x] archived";
  return lines.join(newline);
}

function assertSplitArchived(content: string, childId: string, order: number): void {
  const { end, lines, start } = splitSection(content, childId, order);
  if (!lines.slice(start + 1, end).includes("- 归档状态：[x] archived")) {
    throw new ChangeNotReadyError(`parent split archive marker is missing: ${childId}`);
  }
}

async function prepareParentUpdate(
  projectRoot: string,
  childId: string,
  childMetadata: Record<string, unknown>,
  archivedAt: string,
  archivedTo: string,
): Promise<ParentUpdate | undefined> {
  const childMulti = childMetadata.multiChange;
  if (!isRecord(childMulti) || childMulti.role !== "child") {
    return undefined;
  }
  const parentId = childMulti.parent;
  const childOrder = childMulti.order;
  const childDependencies = stringArray(childMulti.dependsOn);
  if (typeof parentId !== "string" || !Number.isInteger(childOrder) || Number(childOrder) < 1 || !childDependencies) {
    throw new ChangeNotReadyError(`child metadata is incomplete: ${childId}`);
  }

  const parentValidation = await validateChange(projectRoot, parentId, { strict: true });
  if (!parentValidation.valid || !parentValidation.metadata) {
    throw new ChangeNotReadyError(`active parent is invalid: ${String(parentId)}`);
  }
  const parentMulti = parentValidation.metadata.multiChange;
  if (!isRecord(parentMulti) || parentMulti.role !== "parent" || !Array.isArray(parentMulti.children)) {
    throw new ChangeNotReadyError(`active parent does not declare a child list: ${String(parentId)}`);
  }
  const childIndex = parentMulti.children.findIndex(entry => isRecord(entry) && entry.id === childId);
  if (childIndex < 0) {
    throw new ChangeNotReadyError(`active parent does not declare child: ${childId}`);
  }
  const parentChild = parentMulti.children[childIndex];
  if (!isRecord(parentChild)
    || parentChild.order !== childOrder
    || !sameStrings(stringArray(parentChild.dependsOn), childDependencies)) {
    throw new ChangeNotReadyError(`parent and child metadata disagree for ${childId}`);
  }
  if (parentChild.archiveStatus !== undefined) {
    throw new ChangeNotReadyError(`parent already records an archive status for ${childId}`);
  }

  const parentRoot = changeDirectory(projectRoot, parentId);
  const metaPath = path.join(parentRoot, "meta.yaml");
  const splitPath = path.join(parentRoot, "split.md");
  const originalMeta = readFileSync(metaPath, "utf8");
  const originalSplit = readFileSync(splitPath, "utf8");
  const document = parseDocument(originalMeta);
  if (document.errors.length > 0) {
    throw new Error(`active parent metadata is invalid: ${document.errors[0].message}`);
  }
  document.setIn(["multiChange", "children", childIndex, "archiveStatus"], "archived");
  document.setIn(["multiChange", "children", childIndex, "archivedAt"], archivedAt);
  document.setIn(["multiChange", "children", childIndex, "archivedTo"], archivedTo);

  return {
    metaPath,
    splitPath,
    originalMeta,
    originalSplit,
    updatedMeta: document.toString(),
    updatedSplit: updateSplitMarker(originalSplit, childId, Number(childOrder)),
  };
}

function assertParentReady(projectRoot: string, metadata: Record<string, unknown>, splitPath: string): void {
  const multi = metadata.multiChange;
  if (!isRecord(multi) || multi.role !== "parent" || !Array.isArray(multi.children) || multi.children.length === 0) {
    throw new ChangeNotReadyError("parent change must declare at least one child");
  }
  const split = readFileSync(splitPath, "utf8");
  for (const value of multi.children) {
    if (!isRecord(value)
      || typeof value.id !== "string"
      || !Number.isInteger(value.order)
      || value.archiveStatus !== "archived"
      || typeof value.archivedAt !== "string"
      || typeof value.archivedTo !== "string") {
      throw new ChangeNotReadyError("parent child archive evidence is incomplete");
    }
    if (existsSync(changeDirectory(projectRoot, value.id))) {
      throw new ChangeNotReadyError(`parent child is still active: ${value.id}`);
    }
    const archivedPath = resolveSafePath(projectRoot, value.archivedTo);
    if (!existsSync(archivedPath)) {
      throw new ChangeNotReadyError(`parent child archive target is missing: ${value.archivedTo}`);
    }
    assertSplitArchived(split, value.id, Number(value.order));
  }
}

export async function archiveChange(
  projectRoot: string,
  changeId: string,
  clock: ArchiveClock = () => new Date(),
  options: ArchiveOptions = {},
): Promise<ChangeArchiveReport> {
  const root = path.resolve(projectRoot);
  const now = clock();
  if (Number.isNaN(now.getTime())) {
    throw new TypeError("archive clock returned an invalid date");
  }
  const archivedAt = now.toISOString();
  const archivedTo = `.spec/archive/${archivedAt.slice(0, 10)}-${changeId}`;
  const source = changeDirectory(root, changeId);
  const target = resolveSafePath(root, archivedTo);

  if (!existsSync(source)) {
    throw new Error(`active change does not exist: ${changeId}`);
  }
  if (existsSync(target)) {
    throw new Error(`archive target already exists: ${archivedTo}`);
  }

  const validation = await validateChange(root, changeId, { strict: true });
  if (!validation.valid || !validation.metadata) {
    throw new ChangeNotReadyError(`change is not ready to archive: ${validation.issues.map(issue => issue.kind).join(", ")}`);
  }
  const multi = validation.metadata.multiChange;
  const isParent = isRecord(multi) && multi.role === "parent";
  if (isParent) {
    assertParentReady(root, validation.metadata, path.join(source, "split.md"));
  } else if (validation.metadata.stage !== "verification" && validation.metadata.stage !== "archive") {
    throw new ChangeNotReadyError(`change is not ready to archive from stage ${String(validation.metadata.stage)}`);
  }

  const parentUpdate = await prepareParentUpdate(root, changeId, validation.metadata, archivedAt, archivedTo);
  const writeParentFile = options.writeParentFile ?? atomicWrite;
  mkdirSync(path.dirname(target), { recursive: true });
  renameSync(source, target);
  try {
    if (parentUpdate) {
      writeParentFile(parentUpdate.metaPath, parentUpdate.updatedMeta);
      writeParentFile(parentUpdate.splitPath, parentUpdate.updatedSplit);
    }
  } catch (error) {
    if (parentUpdate) {
      atomicWrite(parentUpdate.metaPath, parentUpdate.originalMeta);
      atomicWrite(parentUpdate.splitPath, parentUpdate.originalSplit);
    }
    if (existsSync(target) && !existsSync(source)) {
      renameSync(target, source);
    }
    throw error;
  }
  return { id: changeId, archivedAt, archivedTo };
}
