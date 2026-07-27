import { existsSync, readFileSync, statSync } from "node:fs";
import path from "node:path";

import { parseYamlFrontmatter } from "../markdown/frontmatter.js";
import { assertCanonicalChangeId } from "../path.js";
import { ARTIFACTS, type ArtifactId } from "./artifacts.js";
import {
  changeFilePath,
  isChangeStage,
  isDeliveryShape,
  readMetadata,
  type ChangeStage,
} from "./metadata.js";

const REQUIRED_ARTIFACTS: Record<ChangeStage, ArtifactId[]> = {
  exploration: ["metadata"],
  proposal: ["proposal", "metadata"],
  delivery: ["proposal", "metadata"],
  design: ["proposal", "design", "metadata"],
  cases: ["proposal", "design", "cases", "metadata"],
  tasks: ["proposal", "design", "cases", "tasks", "metadata"],
  implementation: ["proposal", "design", "cases", "tasks", "metadata"],
  review: ["proposal", "design", "cases", "tasks", "metadata"],
  verification: ["proposal", "design", "cases", "tasks", "review-report", "test-report", "metadata"],
  archive: ["proposal", "design", "cases", "tasks", "review-report", "test-report", "metadata"],
};

export type ChangeIssueKind
  = | "invalid_change_id"
    | "missing_metadata"
    | "invalid_metadata"
    | "metadata_id_mismatch"
    | "invalid_stage"
    | "invalid_delivery_shape"
    | "invalid_multi_change"
    | "missing_artifact"
    | "empty_artifact"
    | "incomplete_tasks"
    | "review_not_passed"
    | "verification_not_passed";

export type ChangeIssue = {
  kind: ChangeIssueKind;
  path: string;
  message: string;
  blocking: true;
};

export type ChangeValidationResult = {
  id: string;
  valid: boolean;
  metadata?: Record<string, unknown>;
  requiredArtifacts: ArtifactId[];
  artifacts: Record<ArtifactId, boolean>;
  issues: ChangeIssue[];
};

export type ValidateChangeOptions = {
  strict?: boolean;
};

function artifactPresence(projectRoot: string, changeId: string): Record<ArtifactId, boolean> {
  return Object.fromEntries(
    Object.entries(ARTIFACTS).map(([id, fileName]) => [id, existsSync(changeFilePath(projectRoot, changeId, fileName))]),
  ) as Record<ArtifactId, boolean>;
}

function emptyArtifactPresence(): Record<ArtifactId, boolean> {
  return Object.fromEntries(Object.keys(ARTIFACTS).map(id => [id, false])) as Record<ArtifactId, boolean>;
}

export function reportIsFullPass(filePath: string, resultField: string): boolean {
  const value = parseYamlFrontmatter(readFileSync(filePath, "utf8"));
  return value?.[resultField] === "pass" && value.scope === "full";
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function canonicalId(value: unknown): value is string {
  if (typeof value !== "string") {
    return false;
  }
  try {
    assertCanonicalChangeId(value);
    return true;
  } catch {
    return false;
  }
}

function stringArray(value: unknown): string[] | undefined {
  return Array.isArray(value) && value.every(item => typeof item === "string") ? value : undefined;
}

function sameStrings(left: string[] | undefined, right: string[] | undefined): boolean {
  return JSON.stringify(left ?? []) === JSON.stringify(right ?? []);
}

export function hasUncheckedTasks(content: string): boolean {
  let fence: "```" | "~~~" | undefined;
  return content.replaceAll("\r\n", "\n").split("\n").some((line) => {
    const marker = line.match(/^\s*(```|~~~)/u)?.[1] as "```" | "~~~" | undefined;
    if (marker) {
      fence = fence === marker ? undefined : (fence ?? marker);
      return false;
    }
    return fence === undefined && /^\s*[-*+]\s+\[\s\]\s+/u.test(line);
  });
}

function hasDependencyCycle(entries: Array<Record<string, unknown>>): boolean {
  const graph = new Map(entries.map(entry => [String(entry.id), stringArray(entry.dependsOn) ?? []]));
  const visiting = new Set<string>();
  const visited = new Set<string>();
  function visit(id: string): boolean {
    if (visiting.has(id)) {
      return true;
    }
    if (visited.has(id)) {
      return false;
    }
    visiting.add(id);
    for (const dependency of graph.get(id) ?? []) {
      if (visit(dependency)) {
        return true;
      }
    }
    visiting.delete(id);
    visited.add(id);
    return false;
  }
  return [...graph.keys()].some(visit);
}

function multiChangeMessage(
  projectRoot: string,
  changeId: string,
  metadata: Record<string, unknown>,
): string | undefined {
  const multi = metadata.multiChange;
  if (!isRecord(multi)) {
    return metadata.deliveryShape === "multi-change"
      ? "multi-change metadata must declare multiChange"
      : undefined;
  }
  if (multi.role === "parent") {
    if (metadata.deliveryShape !== "multi-change" || !Array.isArray(multi.children) || multi.children.length === 0) {
      return "parent metadata requires deliveryShape multi-change and a non-empty child list";
    }
    const ids = new Set<string>();
    const orders = new Set<number>();
    const entries = multi.children;
    for (const value of entries) {
      if (!isRecord(value)
        || !canonicalId(value.id)
        || !Number.isInteger(value.order)
        || Number(value.order) < 1
        || !stringArray(value.dependsOn)) {
        return "parent child entries require canonical id, positive order, and dependsOn";
      }
      if (ids.has(value.id) || orders.has(Number(value.order))) {
        return "parent child ids and orders must be unique";
      }
      ids.add(value.id);
      orders.add(Number(value.order));
    }
    for (const value of entries as Array<Record<string, unknown>>) {
      const dependencies = stringArray(value.dependsOn)!;
      if (dependencies.some(dependency => !ids.has(dependency) || dependency === value.id)) {
        return `parent child ${String(value.id)} has an invalid dependency`;
      }
      if (new Set(dependencies).size !== dependencies.length) {
        return `parent child ${String(value.id)} has duplicate dependencies`;
      }
    }
    if (hasDependencyCycle(entries as Array<Record<string, unknown>>)) {
      return "parent child dependency graph must be acyclic";
    }
    for (const value of entries as Array<Record<string, unknown>>) {
      const dependencies = stringArray(value.dependsOn)!;
      const archived = value.archiveStatus === "archived";
      if (value.archiveStatus !== undefined && !archived) {
        return `parent child ${String(value.id)} has an invalid archive status`;
      }
      if (archived) {
        if (typeof value.archivedAt !== "string"
          || Number.isNaN(Date.parse(value.archivedAt))
          || typeof value.archivedTo !== "string") {
          return `parent child ${String(value.id)} has incomplete archive evidence`;
        }
        continue;
      }
      try {
        const child = readMetadata(projectRoot, String(value.id));
        const childMulti = child.multiChange;
        if (child.id !== value.id
          || child.deliveryShape !== "single-change"
          || !isRecord(childMulti)
          || childMulti.role !== "child"
          || childMulti.parent !== changeId
          || childMulti.order !== value.order
          || !sameStrings(stringArray(childMulti.dependsOn), dependencies)) {
          return `parent and active child metadata disagree for ${String(value.id)}`;
        }
      } catch {
        return `parent active child metadata is missing or invalid for ${String(value.id)}`;
      }
    }
    return undefined;
  }
  if (multi.role === "child") {
    const dependencies = stringArray(multi.dependsOn);
    if (metadata.deliveryShape !== "single-change"
      || !canonicalId(multi.parent)
      || !Number.isInteger(multi.order)
      || Number(multi.order) < 1
      || !dependencies
      || new Set(dependencies).size !== dependencies.length
      || dependencies.includes(changeId)) {
      return "child metadata requires a canonical parent, positive order, and valid dependsOn";
    }
    try {
      const parent = readMetadata(projectRoot, multi.parent);
      const parentMulti = parent.multiChange;
      if (parent.deliveryShape !== "multi-change"
        || !isRecord(parentMulti)
        || parentMulti.role !== "parent"
        || !Array.isArray(parentMulti.children)) {
        return `active parent metadata is invalid for ${changeId}`;
      }
      const parentMessage = multiChangeMessage(projectRoot, multi.parent, parent);
      if (parentMessage) {
        return `active parent metadata is invalid for ${changeId}: ${parentMessage}`;
      }
      const matches = parentMulti.children.filter(entry => isRecord(entry) && entry.id === changeId);
      const entry = matches[0];
      if (matches.length !== 1
        || !isRecord(entry)
        || entry.order !== multi.order
        || entry.archiveStatus !== undefined
        || !sameStrings(stringArray(entry.dependsOn), dependencies)) {
        return `parent and child metadata disagree for ${changeId}`;
      }
    } catch {
      return `active parent metadata is missing or invalid for ${changeId}`;
    }
    return undefined;
  }
  return "multiChange role must be parent or child";
}

export function requiredArtifactsForStage(stage: ChangeStage): ArtifactId[] {
  return [...REQUIRED_ARTIFACTS[stage]];
}

function requiredArtifactsForMetadata(metadata: Record<string, unknown>): ArtifactId[] {
  if (isParentMetadata(metadata)) {
    return ["split", "metadata"];
  }
  return isChangeStage(metadata.stage) ? requiredArtifactsForStage(metadata.stage) : [];
}

function isParentMetadata(metadata: Record<string, unknown>): boolean {
  const multiChange = metadata.multiChange;
  return typeof multiChange === "object"
    && multiChange !== null
    && !Array.isArray(multiChange)
    && (multiChange as Record<string, unknown>).role === "parent";
}

export async function validateChange(
  projectRoot: string,
  changeId: string,
  options: ValidateChangeOptions = {},
): Promise<ChangeValidationResult> {
  const issues: ChangeIssue[] = [];
  try {
    assertCanonicalChangeId(changeId);
  } catch {
    issues.push({
      kind: "invalid_change_id",
      path: changeId,
      message: "change id must be canonical kebab-case",
      blocking: true,
    });
    return {
      id: changeId,
      valid: false,
      requiredArtifacts: [],
      artifacts: emptyArtifactPresence(),
      issues,
    };
  }

  const artifacts = artifactPresence(projectRoot, changeId);
  const metadataPath = changeFilePath(projectRoot, changeId, "meta.yaml");
  if (!existsSync(metadataPath)) {
    issues.push({
      kind: "missing_metadata",
      path: toProjectPath(path.relative(projectRoot, metadataPath)),
      message: "change metadata is missing",
      blocking: true,
    });
    return { id: changeId, valid: false, requiredArtifacts: [], artifacts, issues };
  }

  let metadata: Record<string, unknown>;
  try {
    metadata = readMetadata(projectRoot, changeId);
  } catch (error) {
    issues.push({
      kind: "invalid_metadata",
      path: toProjectPath(path.relative(projectRoot, metadataPath)),
      message: error instanceof Error ? error.message : String(error),
      blocking: true,
    });
    return { id: changeId, valid: false, requiredArtifacts: [], artifacts, issues };
  }

  if (metadata.id !== changeId) {
    issues.push({
      kind: "metadata_id_mismatch",
      path: toProjectPath(path.relative(projectRoot, metadataPath)),
      message: `metadata id must equal directory id ${changeId}`,
      blocking: true,
    });
  }
  if (!isChangeStage(metadata.stage)) {
    issues.push({
      kind: "invalid_stage",
      path: toProjectPath(path.relative(projectRoot, metadataPath)),
      message: `unknown change stage: ${String(metadata.stage)}`,
      blocking: true,
    });
  }
  if (!isDeliveryShape(metadata.deliveryShape)) {
    issues.push({
      kind: "invalid_delivery_shape",
      path: toProjectPath(path.relative(projectRoot, metadataPath)),
      message: `unknown delivery shape: ${String(metadata.deliveryShape)}`,
      blocking: true,
    });
  }
  if (options.strict) {
    const message = multiChangeMessage(projectRoot, changeId, metadata);
    if (message) {
      issues.push({
        kind: "invalid_multi_change",
        path: toProjectPath(path.relative(projectRoot, metadataPath)),
        message,
        blocking: true,
      });
    }
  }

  const requiredArtifacts = requiredArtifactsForMetadata(metadata);
  for (const artifact of requiredArtifacts) {
    const artifactPath = changeFilePath(projectRoot, changeId, ARTIFACTS[artifact]);
    if (!artifacts[artifact]) {
      issues.push({
        kind: "missing_artifact",
        path: toProjectPath(path.relative(projectRoot, artifactPath)),
        message: `required artifact is missing: ${artifact}`,
        blocking: true,
      });
    } else if (options.strict && statSync(artifactPath).size === 0) {
      issues.push({
        kind: "empty_artifact",
        path: toProjectPath(path.relative(projectRoot, artifactPath)),
        message: `required artifact is empty: ${artifact}`,
        blocking: true,
      });
    }
  }

  if (!isParentMetadata(metadata) && (metadata.stage === "verification" || metadata.stage === "archive")) {
    const tasksPath = changeFilePath(projectRoot, changeId, ARTIFACTS.tasks);
    if (options.strict && artifacts.tasks && hasUncheckedTasks(readFileSync(tasksPath, "utf8"))) {
      issues.push({
        kind: "incomplete_tasks",
        path: toProjectPath(path.relative(projectRoot, tasksPath)),
        message: "tasks and checklists must be complete before verification",
        blocking: true,
      });
    }
    const reviewPath = changeFilePath(projectRoot, changeId, ARTIFACTS["review-report"]);
    if (artifacts["review-report"] && !reportIsFullPass(reviewPath, "review-result")) {
      issues.push({
        kind: "review_not_passed",
        path: toProjectPath(path.relative(projectRoot, reviewPath)),
        message: "review report must declare review-result: pass and scope: full",
        blocking: true,
      });
    }
    const testPath = changeFilePath(projectRoot, changeId, ARTIFACTS["test-report"]);
    if (artifacts["test-report"] && !reportIsFullPass(testPath, "verification-result")) {
      issues.push({
        kind: "verification_not_passed",
        path: toProjectPath(path.relative(projectRoot, testPath)),
        message: "test report must declare verification-result: pass and scope: full",
        blocking: true,
      });
    }
  }

  return {
    id: changeId,
    valid: issues.length === 0,
    metadata,
    requiredArtifacts,
    artifacts,
    issues,
  };
}

function toProjectPath(value: string): string {
  return value.replaceAll("\\", "/");
}
