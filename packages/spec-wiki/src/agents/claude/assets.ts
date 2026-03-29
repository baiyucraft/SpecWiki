/**
 * 这个文件只表达 Claude 宿主的 skill 路径和模板入口。
 */
import path from "node:path";

import { HOST_EXPOSED_ACTIONS, type WikiAction } from "../../wikiActions.js";
import { renderHostActionSkill as renderHostActionSkillFromSemantics } from "../shared/commandAssets.js";

export const CLAUDE_ACTIONS = HOST_EXPOSED_ACTIONS;

export function renderClaudeSkill(action: WikiAction): string {
  return renderHostActionSkillFromSemantics(action);
}

export function resolveClaudeSkillPath(repoRoot: string, action: WikiAction): string {
  return path.join(repoRoot, ".claude", "skills", `wiki-${action}`, "SKILL.md");
}

export function listObsoleteClaudeCommandPaths(repoRoot: string): string[] {
  return [
    path.join(repoRoot, ".claude", "commands", "wiki", "init.md"),
    path.join(repoRoot, ".claude", "commands", "wiki", "status.md"),
    path.join(repoRoot, ".claude", "commands", "wiki", "query.md"),
    path.join(repoRoot, ".claude", "commands", "wiki", "update.md"),
    path.join(repoRoot, ".claude", "commands", "wiki", "sync.md"),
    path.join(repoRoot, ".claude", "commands", "wiki", "rebuild.md"),
  ];
}
