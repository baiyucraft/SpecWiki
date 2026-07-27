import { existsSync, readdirSync } from "node:fs";
import path from "node:path";

import { validateChange, type ChangeValidationResult } from "./validate.js";

export type ChangeStatusReport = {
  activeChanges: ChangeValidationResult[];
};

export async function getChangeStatus(projectRoot: string): Promise<ChangeStatusReport> {
  const changesRoot = path.join(path.resolve(projectRoot), ".spec", "changes");
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
