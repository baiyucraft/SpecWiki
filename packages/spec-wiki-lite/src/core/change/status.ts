import { existsSync, readdirSync } from "node:fs";

import { resolveSafePath } from "../path.js";
import { validateChange, type ChangeValidationResult } from "./validate.js";

export type ChangeStatusReport = {
  activeChanges: ChangeValidationResult[];
};

export async function getChangeStatus(projectRoot: string): Promise<ChangeStatusReport> {
  const changesRoot = resolveSafePath(projectRoot, ".spec/changes");
  if (!existsSync(changesRoot)) {
    return { activeChanges: [] };
  }
  const ids = readdirSync(changesRoot, { withFileTypes: true })
    .filter(entry => entry.isDirectory())
    .map(entry => entry.name)
    .sort();
  return {
    activeChanges: await Promise.all(ids.map(id => validateChange(projectRoot, id))),
  };
}
