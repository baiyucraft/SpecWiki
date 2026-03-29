/**
 * 这个文件负责合并 `.codebuddy/settings.json` 中由 `spec-wiki` 管理的 hook 配置。
 * 它只更新 `spec-wiki` 自己的 hook 条目，不覆盖无关用户配置。
 */
import { existsSync, readFileSync } from "node:fs";

import type { CodeBuddySettingsPatch } from "../shared/commandAssets.js";
import { listManagedCodeBuddyHookCommands } from "../shared/commandAssets.js";

type JsonObject = Record<string, unknown>;

function isPlainObject(value: unknown): value is JsonObject {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function readExistingSettings(filePath: string): JsonObject {
  if (!existsSync(filePath)) {
    return {};
  }

  const parsed = JSON.parse(readFileSync(filePath, "utf8")) as unknown;
  if (!isPlainObject(parsed)) {
    throw new Error(`expected ${filePath} to contain a JSON object`);
  }

  return parsed;
}

function extractManagedCommands(entry: unknown): string[] {
  if (!isPlainObject(entry)) {
    return [];
  }

  const hooks = entry.hooks;
  if (!Array.isArray(hooks)) {
    return [];
  }

  return hooks
    .filter((hook) => isPlainObject(hook) && typeof hook.command === "string")
    .map((hook) => String(hook.command));
}

function buildManagedHookEntry(command: string): JsonObject {
  return {
    matcher: "*",
    hooks: [
      {
        type: "command",
        command,
      },
    ],
  };
}

/**
 * 生成合并后的 settings.json 文本。
 *
 * @param filePath settings 文件路径。
 * @param patch 本轮需要写入的 spec-wiki hook patch。
 * @returns 返回可直接写盘的 JSON 文本。
 */
export function renderMergedCodeBuddySettings(
  filePath: string,
  patch: CodeBuddySettingsPatch,
): string {
  const current = readExistingSettings(filePath);
  const existingHooks = isPlainObject(current.hooks) ? { ...current.hooks } : {};
  const managedCommands = new Set(listManagedCodeBuddyHookCommands(patch));

  for (const [event, commands] of Object.entries(patch.hooks)) {
    const currentEntries = Array.isArray(existingHooks[event]) ? [...existingHooks[event]] : [];
    const retainedEntries = currentEntries.filter((entry) =>
      extractManagedCommands(entry).every((command) => !managedCommands.has(command)),
    );

    existingHooks[event] = [...retainedEntries, ...commands.map(buildManagedHookEntry)];
  }

  const merged: JsonObject = {
    ...current,
    hooks: existingHooks,
    specWiki: {
      ...(isPlainObject(current.specWiki) ? current.specWiki : {}),
      managedHooksVersion: 1,
      managedHookCommands: Array.from(managedCommands),
    },
  };

  return `${JSON.stringify(merged, null, 2)}\n`;
}
