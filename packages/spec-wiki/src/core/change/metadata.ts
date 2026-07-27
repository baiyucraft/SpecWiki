import { readFileSync } from "node:fs";
import path from "node:path";

import { parseDocument } from "yaml";

import { assertCanonicalChangeId, resolveSafePath } from "../path.js";

export const CHANGE_STAGES = [
  "exploration",
  "proposal",
  "delivery",
  "design",
  "cases",
  "tasks",
  "implementation",
  "review",
  "verification",
  "archive",
] as const;

export const DELIVERY_SHAPES = ["single-change", "multi-change"] as const;

export type ChangeStage = typeof CHANGE_STAGES[number];
export type DeliveryShape = typeof DELIVERY_SHAPES[number];

export type ChangeMetadata = Record<string, unknown> & {
  id: string;
  stage: ChangeStage;
  deliveryShape: DeliveryShape;
  multiChange?: Record<string, unknown>;
};

export function isChangeStage(value: unknown): value is ChangeStage {
  return typeof value === "string" && CHANGE_STAGES.includes(value as ChangeStage);
}

export function isDeliveryShape(value: unknown): value is DeliveryShape {
  return typeof value === "string" && DELIVERY_SHAPES.includes(value as DeliveryShape);
}

export function changeDirectory(projectRoot: string, changeId: string): string {
  assertCanonicalChangeId(changeId);
  return resolveSafePath(projectRoot, `.spec/changes/${changeId}`);
}

export function parseMetadata(content: string): Record<string, unknown> {
  const document = parseDocument(content);
  if (document.errors.length > 0) {
    throw new Error(document.errors.map(error => error.message).join("; "));
  }
  const value: unknown = document.toJS();
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error("meta.yaml must contain a YAML object");
  }
  return value as Record<string, unknown>;
}

export function readMetadata(projectRoot: string, changeId: string): Record<string, unknown> {
  return parseMetadata(readFileSync(path.join(changeDirectory(projectRoot, changeId), "meta.yaml"), "utf8"));
}
