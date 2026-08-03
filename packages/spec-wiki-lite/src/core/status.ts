import { existsSync, readFileSync } from "node:fs";

import { readPackageAsset } from "../packageRoot.js";
import { PROJECT_SKILL_NAMES, projectSkillAssetsByNameForLanguage } from "./assets/registry.js";
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
  const skillAssets = projectSkillAssetsByNameForLanguage(wiki.language);
  const skills = PROJECT_SKILL_NAMES.map((name) => {
    const assets = skillAssets.get(name) ?? [];
    const installed = assets.length > 0 && assets.every((asset) => {
      const target = resolveSafePath(projectRoot, asset.target);
      if (!existsSync(target)) {
        return false;
      }
      try {
        return readFileSync(target, "utf8") === readPackageAsset(asset.source);
      } catch {
        return false;
      }
    });
    return {
      installed,
      name,
      path: `.agents/skills/${name}/SKILL.md`,
    };
  });
  return {
    ready: wiki.ready
      && !wiki.bootstrapPending
      && skills.every(skill => skill.installed)
      && changes.activeChanges.every(change => change.valid),
    wiki,
    skills,
    changes,
  };
}
