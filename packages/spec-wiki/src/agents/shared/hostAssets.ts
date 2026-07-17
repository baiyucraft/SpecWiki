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
import { HOSTS, type HostDefinition, type SupportedHost } from "./hosts.js";
import { HOST_EXPOSED_ACTIONS } from "../../wikiActions.js";
import { buildCodeBuddySettingsPatch } from "./commandAssets.js";

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

/**
 * 校验 registry 的 Codex-first 角色约束。
 *
 * @param hosts 完整宿主注册表。
 * @throws reference 数量或身份错误时抛出合同错误。
 */
export function validateHostRegistry(hosts: readonly HostDefinition[]): void {
  const referenceHosts = hosts.filter(host => host.compatibilityRole === "reference");
  if (referenceHosts.length !== 1 || referenceHosts[0].id !== "codex") {
    throw new Error("host registry requires Codex as the only reference host");
  }
}

function assetPath(asset: HostBootstrapAsset): string {
  return asset.filePath.replaceAll("\\", "/");
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function extractSettingsHookCommands(settings: Record<string, unknown>, event: string): string[] {
  if (!isRecord(settings.hooks) || !Array.isArray(settings.hooks[event])) {
    return [];
  }
  return settings.hooks[event].flatMap((entry) => {
    if (!isRecord(entry) || !Array.isArray(entry.hooks)) {
      return [];
    }
    return entry.hooks
      .filter(hook => isRecord(hook) && typeof hook.command === "string")
      .map(hook => String(hook.command));
  });
}

function validateManagedSettingsCommands(asset: HostSettingsAsset): void {
  let settings: unknown;
  try {
    settings = JSON.parse(asset.content);
  } catch {
    throw new Error("codebuddy managed settings must be valid JSON");
  }
  if (!isRecord(settings)) {
    throw new Error("codebuddy managed settings must be a JSON object");
  }

  const expected = buildCodeBuddySettingsPatch();
  for (const [event, commands] of Object.entries(expected.hooks)) {
    const actualCommands = extractSettingsHookCommands(settings, event);
    for (const command of commands) {
      if (!actualCommands.includes(command)) {
        throw new Error(`codebuddy managed hook command is missing for ${event}: ${command}`);
      }
    }
  }
}

/**
 * 核对宿主 capability 声明与实际 bootstrap assets。
 *
 * @param host 单宿主 registry 定义。
 * @param assets 即将进入写入计划的资产。
 * @throws action skill 或声明的 hook/settings 缺失、越界时抛出合同错误。
 */
export function validateHostAssetsAgainstCapabilities(
  host: HostDefinition,
  assets: readonly HostBootstrapAsset[],
): void {
  for (const action of HOST_EXPOSED_ACTIONS) {
    const suffix = `/skills/wiki-${action}/SKILL.md`;
    if (!assets.some(asset => asset.kind === "skill" && assetPath(asset).endsWith(suffix))) {
      throw new Error(`${host.id} native skill discovery requires wiki-${action}`);
    }
  }

  const userPromptHook = assets.some(asset =>
    asset.kind === "hook" && assetPath(asset).endsWith("/hooks/spec-wiki/user-prompt-submit.mjs"),
  );
  const sessionStartHook = assets.some(asset =>
    asset.kind === "hook" && assetPath(asset).endsWith("/hooks/spec-wiki/session-start.mjs"),
  );
  const settingsAssets = assets.filter(asset => asset.kind === "settings");
  const capabilities = host.triggerCapabilities;

  if ((capabilities.promptInspection === "user_prompt_submit") !== userPromptHook) {
    throw new Error(`${host.id} UserPromptSubmit hook does not match trigger capabilities`);
  }
  if ((capabilities.sessionStartContext === "orientation") !== sessionStartHook) {
    throw new Error(`${host.id} SessionStart hook does not match trigger capabilities`);
  }
  if ((capabilities.settingsIntegration === "managed_settings") !== (settingsAssets.length === 1)) {
    throw new Error(`${host.id} settings asset does not match trigger capabilities`);
  }
  if (capabilities.deterministicDelivery === "generated_hook" && !userPromptHook) {
    throw new Error(`${host.id} generated hook delivery requires UserPromptSubmit`);
  }
  if (capabilities.deterministicDelivery === "skill_guidance" && (userPromptHook || sessionStartHook)) {
    throw new Error(`${host.id} skill guidance delivery must not generate hooks`);
  }
  if (capabilities.settingsIntegration === "managed_settings") {
    validateManagedSettingsCommands(settingsAssets[0]);
  }
}

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
  let assets: HostBootstrapAsset[];
  switch (host) {
    case "codex":
      assets = buildCodexAssets(repoRoot);
      break;
    case "claude":
      assets = buildClaudeAssets(repoRoot);
      break;
    case "codebuddy":
      assets = buildCodeBuddyBootstrapAssets(repoRoot, CODEBUDDY_ACTIONS).map(normalizeCodeBuddyAsset);
      break;
    default: {
      const unreachableHost: never = host;
      throw new Error(`unsupported host ${unreachableHost}`);
    }
  }

  validateHostRegistry(HOSTS);
  const hostDefinition = HOSTS.find(definition => definition.id === host);
  if (!hostDefinition) {
    throw new Error(`missing host definition for ${host}`);
  }
  validateHostAssetsAgainstCapabilities(hostDefinition, assets);
  return assets;
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
