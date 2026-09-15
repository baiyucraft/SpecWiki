import { existsSync, readFileSync } from "node:fs";

import { inspectAoci, resolveInstalledAoci, type AociResult } from "../orchestration/aoci/runner.js";
import { inspectCodeGraph, type CodeGraphCommandRunner, type CodeGraphResult } from "../orchestration/codegraph/runner.js";
import type { ToolCommandRunner } from "../orchestration/tools/command.js";
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

export type ProjectToolsStatus = {
  ready: boolean;
  codegraph: CodeGraphResult;
  aoci: AociResult;
  nextActions: string[];
};

export type ProjectStatusReport = {
  ready: boolean;
  wiki: WikiInspectionReport;
  skills: SkillStatus[];
  changes: ChangeStatusReport;
  tools: ProjectToolsStatus;
};

export type ProjectStatusOptions = {
  env?: NodeJS.ProcessEnv;
  codegraphRunner?: CodeGraphCommandRunner;
  aociRunner?: ToolCommandRunner;
  aociExecutablePath?: string;
};

function codeGraphReady(result: CodeGraphResult): boolean {
  return result.cli.compatible && result.codexMcp.configured && result.project.healthy;
}

function aociReady(result: AociResult): boolean {
  return result.available
    && result.version === result.expectedVersion
    && result.initialized
    && result.governanceAligned
    && result.database.ready;
}

export async function getProjectStatus(
  projectRoot: string,
  options: ProjectStatusOptions = {},
): Promise<ProjectStatusReport> {
  const env = options.env ?? process.env;
  const [wiki, changes, codegraph, aoci] = await Promise.all([
    inspectWiki(projectRoot),
    getChangeStatus(projectRoot),
    inspectCodeGraph({ projectRoot, env, runner: options.codegraphRunner }),
    inspectAoci({
      executablePath: options.aociExecutablePath ?? resolveInstalledAoci(env),
      projectRoot,
      env,
      runner: options.aociRunner,
    }),
  ]);
  const skillAssets = projectSkillAssetsByNameForLanguage(wiki.language);
  const skills = PROJECT_SKILL_NAMES.map((name) => {
    const assets = skillAssets.get(name) ?? [];
    const installed = assets.length > 0 && assets.every((asset) => {
      const target = resolveSafePath(projectRoot, asset.target);
      if (!existsSync(target)) return false;
      try { return readFileSync(target, "utf8") === readPackageAsset(asset.source); } catch { return false; }
    });
    return { installed, name, path: `.agents/skills/${name}/SKILL.md` };
  });
  const toolsReady = codeGraphReady(codegraph) && aociReady(aoci);
  const tools = {
    ready: toolsReady,
    codegraph,
    aoci,
    nextActions: [...new Set([...codegraph.nextActions, ...aoci.nextActions])],
  };
  return {
    ready: wiki.ready
      && !wiki.bootstrapPending
      && skills.every(skill => skill.installed)
      && changes.activeChanges.every(change => change.valid)
      && toolsReady,
    wiki,
    skills,
    changes,
    tools,
  };
}
