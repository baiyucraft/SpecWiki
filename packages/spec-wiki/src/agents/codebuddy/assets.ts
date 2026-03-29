/**
 * 这个文件负责生成 CodeBuddy 主链资产：
 * action skills、hook scripts 和 settings patch。
 */
import path from "node:path";

import { HOST_EXPOSED_ACTIONS, type WikiAction } from "../../wikiActions.js";
import {
  buildCodeBuddySettingsPatch,
  renderCodeBuddyActionSkill,
  renderCodeBuddyHookScript,
  type CodeBuddySettingsPatch,
} from "../shared/commandAssets.js";
import { renderMergedCodeBuddySettings } from "./settings.js";

export const CODEBUDDY_ACTIONS = HOST_EXPOSED_ACTIONS;

export type CodeBuddyBootstrapAsset
  = | {
      kind: "skill" | "hook";
      filePath: string;
      content: string;
    }
    | {
      kind: "settings";
      filePath: string;
      patch: CodeBuddySettingsPatch;
    };

export function resolveCodeBuddyActionSkillPath(repoRoot: string, action: WikiAction): string {
  return path.join(repoRoot, ".codebuddy", "skills", `wiki-${action}`, "SKILL.md");
}

export function resolveLegacyCodeBuddySharedSkillDir(repoRoot: string): string {
  return path.join(repoRoot, ".codebuddy", "skills", "spec-wiki-runtime");
}

export function listObsoleteCodeBuddySkillDirs(repoRoot: string): string[] {
  return [
    resolveLegacyCodeBuddySharedSkillDir(repoRoot),
    path.join(repoRoot, ".codebuddy", "skills", "wiki-sync"),
    path.join(repoRoot, ".codebuddy", "skills", "wiki-rebuild"),
  ];
}

export function resolveCodeBuddyHookPath(
  repoRoot: string,
  hookName: "session-start" | "user-prompt-submit",
): string {
  return path.join(repoRoot, ".codebuddy", "hooks", "spec-wiki", `${hookName}.mjs`);
}

export function resolveCodeBuddySettingsPath(repoRoot: string): string {
  return path.join(repoRoot, ".codebuddy", "settings.json");
}

export function buildCodeBuddyBootstrapAssets(
  repoRoot: string,
  actions: readonly WikiAction[],
): CodeBuddyBootstrapAsset[] {
  const assets: CodeBuddyBootstrapAsset[] = [
    {
      kind: "hook",
      filePath: resolveCodeBuddyHookPath(repoRoot, "session-start"),
      content: renderCodeBuddyHookScript("SessionStart"),
    },
    {
      kind: "hook",
      filePath: resolveCodeBuddyHookPath(repoRoot, "user-prompt-submit"),
      content: renderCodeBuddyHookScript("UserPromptSubmit"),
    },
    {
      kind: "settings",
      filePath: resolveCodeBuddySettingsPath(repoRoot),
      patch: buildCodeBuddySettingsPatch(),
    },
  ];

  for (const action of actions) {
    assets.push({
      kind: "skill",
      filePath: resolveCodeBuddyActionSkillPath(repoRoot, action),
      content: renderCodeBuddyActionSkill(action),
    });
  }

  return assets;
}

export function renderCodeBuddySettingsAsset(
  filePath: string,
  patch: CodeBuddySettingsPatch,
): string {
  return renderMergedCodeBuddySettings(filePath, patch);
}
