/**
 * 这个文件只表达 Codex 宿主的 skill 路径和模板入口。
 */
import path from "node:path";

import { HOST_EXPOSED_ACTIONS, type WikiAction } from "../../wikiActions.js";
import { renderHostActionSkill as renderHostActionSkillFromSemantics } from "../shared/commandAssets.js";

export const CODEX_ACTIONS = HOST_EXPOSED_ACTIONS;

export function renderCodexSkill(action: WikiAction): string {
  return renderHostActionSkillFromSemantics(action);
}

export function resolveCodexSkillPath(repoRoot: string, action: WikiAction): string {
  return path.join(repoRoot, ".codex", "skills", `wiki-${action}`, "SKILL.md");
}
