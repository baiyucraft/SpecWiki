import { existsSync } from "node:fs";

import { PROJECT_ASSETS } from "./assets/registry.js";
import { getChangeStatus, type ChangeStatusReport } from "./change/status.js";
import { resolveSafePath } from "./path.js";
import { inspectWiki, type WikiInspectionReport } from "./wiki/inspect.js";

export type SkillStatus = {
  installed: boolean;
  name: string;
  path: string;
};

export type ProjectStatusReport = {
  ready: boolean;
  wiki: WikiInspectionReport;
  skills: SkillStatus[];
  changes: ChangeStatusReport;
};

export async function getProjectStatus(projectRoot: string): Promise<ProjectStatusReport> {
  const [wiki, changes] = await Promise.all([
    inspectWiki(projectRoot),
    getChangeStatus(projectRoot),
  ]);
  const skills = PROJECT_ASSETS
    .filter(asset => asset.ownership === "skill")
    .map(asset => ({
      installed: existsSync(resolveSafePath(projectRoot, asset.target)),
      name: asset.target.split("/").at(-2)!,
      path: asset.target,
    }));
  return {
    ready: wiki.ready
      && skills.every(skill => skill.installed)
      && changes.activeChanges.every(change => change.valid),
    wiki,
    skills,
    changes,
  };
}
