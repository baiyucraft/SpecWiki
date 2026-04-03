/**
 * 这个文件覆盖宿主检测、模板写入、幂等刷新和 Codex 旧资产清理。
 * 它确保 spec-wiki init 真正落到三类宿主的真实目录结构。
 */
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { afterEach, expect, test } from "vitest";

import { detectHosts, resolveSelectedHosts } from "./agents/shared/hosts.js";
import { runBootstrapInit } from "./orchestration/init/runInit.js";

const tempDirs: string[] = [];

function makeTempDir(prefix: string): string {
  const dir = mkdtempSync(path.join(os.tmpdir(), prefix));
  tempDirs.push(dir);
  return dir;
}

afterEach(() => {
  while (tempDirs.length > 0) {
    rmSync(tempDirs.pop()!, { recursive: true, force: true });
  }
});

test("detectHosts returns the hosts that already exist in the repo", () => {
  const repoRoot = makeTempDir("spec-wiki-host-detect-");
  writeFileSync(path.join(repoRoot, ".claude"), "not-a-dir");
  mkdirSync(path.join(repoRoot, ".codebuddy"), { recursive: true });
  mkdirSync(path.join(repoRoot, ".codex"), { recursive: true });

  expect(detectHosts(repoRoot)).toEqual(["codex", "codebuddy"]);
});

test("resolveSelectedHosts requires explicit selection when multiple hosts exist", () => {
  const repoRoot = makeTempDir("spec-wiki-host-select-");
  mkdirSync(path.join(repoRoot, ".claude"), { recursive: true });
  mkdirSync(path.join(repoRoot, ".codebuddy"), { recursive: true });

  expect(() => resolveSelectedHosts(repoRoot)).toThrow(/multiple hosts detected/);
});

test("runBootstrapInit writes Claude skills and removes legacy command assets", async () => {
  const repoRoot = makeTempDir("spec-wiki-claude-init-");
  mkdirSync(path.join(repoRoot, ".claude", "commands", "wiki"), { recursive: true });
  const legacySyncPath = path.join(repoRoot, ".claude", "commands", "wiki", "sync.md");
  const legacyRebuildPath = path.join(repoRoot, ".claude", "commands", "wiki", "rebuild.md");
  const legacyStatusPath = path.join(repoRoot, ".claude", "commands", "wiki", "status.md");
  const legacyQueryPath = path.join(repoRoot, ".claude", "commands", "wiki", "query.md");
  const legacySkillPath = path.join(repoRoot, ".claude", "skills", "spec-wiki", "SKILL.md");
  writeFileSync(legacySyncPath, "legacy sync command");
  writeFileSync(legacyRebuildPath, "legacy rebuild command");
  writeFileSync(legacyStatusPath, "legacy status command");
  writeFileSync(legacyQueryPath, "legacy query command");
  mkdirSync(path.dirname(legacySkillPath), { recursive: true });
  writeFileSync(legacySkillPath, "legacy skill");

  const result = await runBootstrapInit({
    repoRoot,
    env: process.env,
  });

  expect(result.hosts.map((host) => host.host)).toEqual(["claude"]);

  const statusSkillPath = path.join(repoRoot, ".claude", "skills", "wiki-status", "SKILL.md");
  const querySkillPath = path.join(repoRoot, ".claude", "skills", "wiki-query", "SKILL.md");
  const syncSkillPath = path.join(repoRoot, ".claude", "skills", "wiki-sync", "SKILL.md");
  const rebuildSkillPath = path.join(repoRoot, ".claude", "skills", "wiki-rebuild", "SKILL.md");

  expect(readFileSync(statusSkillPath, "utf8")).toContain(
    "description: \"Use when you need to know whether the wiki is ready, stale, blocked, or needs refresh.\"",
  );
  expect(readFileSync(statusSkillPath, "utf8")).toContain("## When To Use");
  expect(readFileSync(querySkillPath, "utf8")).toContain("name: wiki-query");
  expect(readFileSync(querySkillPath, "utf8")).toContain("`query_mode`");
  expect(readFileSync(querySkillPath, "utf8")).toContain("## How To Work");
  expect(readFileSync(querySkillPath, "utf8")).toContain("Do not rebuild a new Wiki state machine or page semantic layer from query results");
  expect(readFileSync(syncSkillPath, "utf8")).toContain("name: wiki-sync");
  expect(readFileSync(rebuildSkillPath, "utf8")).toContain("name: wiki-rebuild");
  expect(existsSync(legacySyncPath)).toBe(false);
  expect(existsSync(legacyRebuildPath)).toBe(false);
  expect(existsSync(legacyStatusPath)).toBe(false);
  expect(existsSync(legacyQueryPath)).toBe(false);
  expect(existsSync(legacySkillPath)).toBe(false);
});

test("runBootstrapInit supports explicit Codex bootstrap as repo skills without deleting global prompts", async () => {
  const repoRoot = makeTempDir("spec-wiki-codex-init-");
  const codexHome = makeTempDir("spec-wiki-codex-home-");
  mkdirSync(path.join(codexHome, "prompts"), { recursive: true });
  const legacyInitPromptPath = path.join(codexHome, "prompts", "wiki-init.md");
  const legacyStatusPromptPath = path.join(codexHome, "prompts", "wiki-status.md");
  const legacyQueryPromptPath = path.join(codexHome, "prompts", "wiki-query.md");
  const legacyUpdatePromptPath = path.join(codexHome, "prompts", "wiki-update.md");
  const legacySyncPromptPath = path.join(codexHome, "prompts", "wiki-sync.md");
  const legacyRebuildPromptPath = path.join(codexHome, "prompts", "wiki-rebuild.md");
  const legacySkillPath = path.join(repoRoot, ".codex", "skills", "spec-wiki", "SKILL.md");
  writeFileSync(legacyInitPromptPath, "legacy init prompt");
  writeFileSync(legacyStatusPromptPath, "legacy status prompt");
  writeFileSync(legacyQueryPromptPath, "legacy query prompt");
  writeFileSync(legacyUpdatePromptPath, "legacy update prompt");
  writeFileSync(legacySyncPromptPath, "legacy sync prompt");
  writeFileSync(legacyRebuildPromptPath, "legacy rebuild prompt");
  mkdirSync(path.dirname(legacySkillPath), { recursive: true });
  writeFileSync(legacySkillPath, "legacy skill");

  const result = await runBootstrapInit({
    repoRoot,
    tools: "codex",
    env: {
      ...process.env,
      CODEX_HOME: codexHome,
    },
  });

  expect(result.hosts.map((host) => host.host)).toEqual(["codex"]);

  const querySkillPath = path.join(repoRoot, ".codex", "skills", "wiki-query", "SKILL.md");
  const statusSkillPath = path.join(repoRoot, ".codex", "skills", "wiki-status", "SKILL.md");
  const syncSkillPath = path.join(repoRoot, ".codex", "skills", "wiki-sync", "SKILL.md");
  const rebuildSkillPath = path.join(repoRoot, ".codex", "skills", "wiki-rebuild", "SKILL.md");

  expect(readFileSync(statusSkillPath, "utf8")).toContain(
    "description: \"Use when you need to know whether the wiki is ready, stale, blocked, or needs refresh.\"",
  );
  expect(readFileSync(statusSkillPath, "utf8")).toContain("## When To Use");
  expect(readFileSync(querySkillPath, "utf8")).toContain("name: wiki-query");
  expect(readFileSync(querySkillPath, "utf8")).toContain("`query_mode`");
  expect(readFileSync(querySkillPath, "utf8")).toContain("## After This");
  expect(readFileSync(querySkillPath, "utf8")).toContain("Do not rebuild a new Wiki state machine or page semantic layer from query results");
  expect(readFileSync(syncSkillPath, "utf8")).toContain("name: wiki-sync");
  expect(readFileSync(rebuildSkillPath, "utf8")).toContain("name: wiki-rebuild");
  expect(readFileSync(legacyInitPromptPath, "utf8")).toBe("legacy init prompt");
  expect(readFileSync(legacyStatusPromptPath, "utf8")).toBe("legacy status prompt");
  expect(readFileSync(legacyQueryPromptPath, "utf8")).toBe("legacy query prompt");
  expect(readFileSync(legacyUpdatePromptPath, "utf8")).toBe("legacy update prompt");
  expect(readFileSync(legacySyncPromptPath, "utf8")).toBe("legacy sync prompt");
  expect(readFileSync(legacyRebuildPromptPath, "utf8")).toBe("legacy rebuild prompt");
  expect(existsSync(legacySkillPath)).toBe(false);
});

test("runBootstrapInit writes CodeBuddy skills, hooks, and settings without creating commands", async () => {
  const repoRoot = makeTempDir("spec-wiki-idempotent-");
  mkdirSync(path.join(repoRoot, ".codebuddy", "commands", "custom"), { recursive: true });
  const unrelatedPath = path.join(repoRoot, ".codebuddy", "commands", "custom", "keep.md");
  const settingsPath = path.join(repoRoot, ".codebuddy", "settings.json");
  const legacySharedSkillPath = path.join(repoRoot, ".codebuddy", "skills", "spec-wiki-runtime", "SKILL.md");
  const legacySyncSkillPath = path.join(repoRoot, ".codebuddy", "skills", "wiki-sync", "SKILL.md");
  const legacyRebuildSkillPath = path.join(repoRoot, ".codebuddy", "skills", "wiki-rebuild", "SKILL.md");
  writeFileSync(unrelatedPath, "custom");
  mkdirSync(path.dirname(legacySharedSkillPath), { recursive: true });
  writeFileSync(legacySharedSkillPath, "legacy shared skill");
  mkdirSync(path.dirname(legacySyncSkillPath), { recursive: true });
  writeFileSync(legacySyncSkillPath, "legacy sync skill");
  mkdirSync(path.dirname(legacyRebuildSkillPath), { recursive: true });
  writeFileSync(legacyRebuildSkillPath, "legacy rebuild skill");
  writeFileSync(
    settingsPath,
    `${JSON.stringify(
      {
        hooks: {
          SessionStart: [
            {
              matcher: "*",
              hooks: [{ type: "command", command: "node ./custom/session-start.mjs" }],
            },
          ],
        },
        userPreference: {
          hookOutputCollapsed: false,
        },
      },
      null,
      2,
    )}\n`,
  );

  await runBootstrapInit({
    repoRoot,
    tools: "codebuddy",
    env: process.env,
  });
  await runBootstrapInit({
    repoRoot,
    tools: "codebuddy",
    env: process.env,
  });

  const initSkill = readFileSync(
    path.join(repoRoot, ".codebuddy", "skills", "wiki-init", "SKILL.md"),
    "utf8",
  );
  const querySkill = readFileSync(
    path.join(repoRoot, ".codebuddy", "skills", "wiki-query", "SKILL.md"),
    "utf8",
  );
  const userPromptHook = readFileSync(
    path.join(repoRoot, ".codebuddy", "hooks", "spec-wiki", "user-prompt-submit.mjs"),
    "utf8",
  );

  expect(readFileSync(unrelatedPath, "utf8")).toBe("custom");
  expect(initSkill).toContain("spec-wiki wiki init");
  expect(initSkill).toContain("Use when the repository has not been bootstrapped for spec-wiki and you need to initialize the wiki runtime.");

  expect(querySkill).toContain("name: wiki-query");
  expect(querySkill).toContain("Use when you need a fast structured map of where code lives and how files, modules, symbols, or call paths relate before deeper inspection.");

  expect(querySkill).toContain("This is a working pattern, not a rigid output template.");
  expect(querySkill).toContain("`query_mode`");
  expect(querySkill).toContain("## When To Use");
  expect(querySkill).toContain("Do not post-process query output into host-specific Wiki business conclusions");
  expect(
    readFileSync(path.join(repoRoot, ".codebuddy", "hooks", "spec-wiki", "session-start.mjs"), "utf8"),
  ).toContain("additionalContext");
  expect(userPromptHook).toContain("authoritative runtime data");
  expect(userPromptHook).toContain("stable structured runtime fields");
  expect(userPromptHook).toContain("shared rules live in hooks and action skill guardrails");
  expect(existsSync(path.join(repoRoot, ".codebuddy", "commands", "wiki", "init.md"))).toBe(false);
  expect(existsSync(legacySharedSkillPath)).toBe(false);
  expect(readFileSync(legacySyncSkillPath, "utf8")).toContain("name: wiki-sync");
  expect(readFileSync(legacyRebuildSkillPath, "utf8")).toContain("name: wiki-rebuild");
  expect(readFileSync(path.join(repoRoot, ".codebuddy", "skills", "wiki-sync", "SKILL.md"), "utf8")).toContain("name: wiki-sync");
  expect(readFileSync(path.join(repoRoot, ".codebuddy", "skills", "wiki-rebuild", "SKILL.md"), "utf8")).toContain("name: wiki-rebuild");

  const mergedSettings = JSON.parse(readFileSync(settingsPath, "utf8"));
  expect(mergedSettings.userPreference).toEqual({ hookOutputCollapsed: false });
  expect(mergedSettings.specWiki.managedHooksVersion).toBe(1);
  expect(mergedSettings.specWiki.managedHookCommands).toContain(
    "node \"$CODEBUDDY_PROJECT_DIR/.codebuddy/hooks/spec-wiki/session-start.mjs\"",
  );
  expect(mergedSettings.hooks.SessionStart).toEqual(
    expect.arrayContaining([
      expect.objectContaining({
        hooks: expect.arrayContaining([
          expect.objectContaining({ command: "node ./custom/session-start.mjs" }),
        ]),
      }),
      expect.objectContaining({
        hooks: expect.arrayContaining([
          expect.objectContaining({
            command: "node \"$CODEBUDDY_PROJECT_DIR/.codebuddy/hooks/spec-wiki/session-start.mjs\"",
          }),
        ]),
      }),
    ]),
  );
});

test("runBootstrapInit no longer depends on Codex global prompt directory", async () => {
  const repoRoot = makeTempDir("spec-wiki-codex-error-");
  const codexHome = makeTempDir("spec-wiki-codex-bad-home-");
  writeFileSync(path.join(codexHome, "prompts"), "occupied");

  const result = await runBootstrapInit({
    repoRoot,
    tools: "codex",
    env: {
      ...process.env,
      CODEX_HOME: codexHome,
    },
  });

  expect(result.hosts.map((host) => host.host)).toEqual(["codex"]);
  expect(existsSync(path.join(repoRoot, ".codex", "skills", "wiki-status", "SKILL.md"))).toBe(true);
});



