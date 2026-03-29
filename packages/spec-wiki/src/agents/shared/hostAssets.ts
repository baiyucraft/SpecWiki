/**
 * 这个文件负责把宿主差异实现统一成 orchestration 可消费的资产描述。
 * 它不决定本次 init 选择哪些宿主，只负责单宿主的受管资产集合。
 */
import path from "node:path";

import { renderClaudeSkill, resolveClaudeSkillPath, CLAUDE_ACTIONS, listObsoleteClaudeCommandPaths } from "../claude/assets.js";
import {
  buildCodeBuddyBootstrapAssets,
  CODEBUDDY_ACTIONS,
  listObsoleteCodeBuddySkillDirs,
  renderCodeBuddySettingsAsset,
  type CodeBuddyBootstrapAsset,
} from "../codebuddy/assets.js";
import { renderCodexSkill, resolveCodexSkillPath, CODEX_ACTIONS } from "../codex/assets.js";
import type { SupportedHost } from "./hosts.js";

export type HostTextAsset = {
  kind: "skill" | "hook";
  filePath: string;
  content: string;
};

export type HostSettingsAsset = {
  kind: "settings";
  filePath: string;
  content: string;
};

export type HostBootstrapAsset = HostTextAsset | HostSettingsAsset;

function resolveLegacySharedSkillDir(repoRoot: string, host: "claude" | "codex"): string {
  return path.join(repoRoot, `.${host}`, "skills", "spec-wiki");
}

function buildClaudeAssets(repoRoot: string): HostBootstrapAsset[] {
  return CLAUDE_ACTIONS.map((action) => ({
    kind: "skill" as const,
    filePath: resolveClaudeSkillPath(repoRoot, action),
    content: renderClaudeSkill(action),
  }));
}

function buildCodexAssets(repoRoot: string): HostBootstrapAsset[] {
  return CODEX_ACTIONS.map((action) => ({
    kind: "skill" as const,
    filePath: resolveCodexSkillPath(repoRoot, action),
    content: renderCodexSkill(action),
  }));
}

function normalizeCodeBuddyAsset(asset: CodeBuddyBootstrapAsset): HostBootstrapAsset {
  if (asset.kind === "settings") {
    return {
      kind: "settings",
      filePath: asset.filePath,
      content: renderCodeBuddySettingsAsset(asset.filePath, asset.patch),
    };
  }

  return asset;
}

export function buildHostBootstrapAssets(
  repoRoot: string,
  env: NodeJS.ProcessEnv,
  host: SupportedHost,
): HostBootstrapAsset[] {
  switch (host) {
    case "codex":
      return buildCodexAssets(repoRoot);
    case "claude":
      return buildClaudeAssets(repoRoot);
    case "codebuddy":
      return buildCodeBuddyBootstrapAssets(repoRoot, CODEBUDDY_ACTIONS).map(normalizeCodeBuddyAsset);
  }

  const unreachableHost: never = host;
  throw new Error(`unsupported host ${unreachableHost}`);
}

export function listObsoleteHostBootstrapFiles(
  repoRoot: string,
  env: NodeJS.ProcessEnv,
  host: SupportedHost,
): string[] {
  switch (host) {
    case "codebuddy":
      return listObsoleteCodeBuddySkillDirs(repoRoot);
    case "claude":
      return [...listObsoleteClaudeCommandPaths(repoRoot), resolveLegacySharedSkillDir(repoRoot, "claude")];
    case "codex":
      return [resolveLegacySharedSkillDir(repoRoot, "codex")];
  }

  const unreachableHost: never = host;
  throw new Error(`unsupported host ${unreachableHost}`);
}
