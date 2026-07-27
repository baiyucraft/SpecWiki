import { existsSync, readFileSync, statSync } from "node:fs";
import path from "node:path";

import { parseYamlFrontmatter } from "../markdown/frontmatter.js";
import { assertCanonicalChangeId } from "../path.js";
import { ARTIFACTS, type ArtifactId } from "./artifacts.js";
import {
  changeDirectory,
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
    | "missing_artifact"
    | "empty_artifact"
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

function artifactPresence(changeRoot: string): Record<ArtifactId, boolean> {
  return Object.fromEntries(
    Object.entries(ARTIFACTS).map(([id, fileName]) => [id, existsSync(path.join(changeRoot, fileName))]),
  ) as Record<ArtifactId, boolean>;
}

function emptyArtifactPresence(): Record<ArtifactId, boolean> {
  return Object.fromEntries(Object.keys(ARTIFACTS).map(id => [id, false])) as Record<ArtifactId, boolean>;
}

function reportIsFullPass(filePath: string, resultField: string): boolean {
  const value = parseYamlFrontmatter(readFileSync(filePath, "utf8"));
  return value?.[resultField] === "pass" && value.scope === "full";
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

  const changeRoot = changeDirectory(projectRoot, changeId);
  const artifacts = artifactPresence(changeRoot);
  const metadataPath = path.join(changeRoot, "meta.yaml");
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

  const requiredArtifacts = requiredArtifactsForMetadata(metadata);
  for (const artifact of requiredArtifacts) {
    const artifactPath = path.join(changeRoot, ARTIFACTS[artifact]);
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
    const reviewPath = path.join(changeRoot, ARTIFACTS["review-report"]);
    if (artifacts["review-report"] && !reportIsFullPass(reviewPath, "review-result")) {
      issues.push({
        kind: "review_not_passed",
        path: toProjectPath(path.relative(projectRoot, reviewPath)),
        message: "review report must declare review-result: pass and scope: full",
        blocking: true,
      });
    }
    const testPath = path.join(changeRoot, ARTIFACTS["test-report"]);
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
