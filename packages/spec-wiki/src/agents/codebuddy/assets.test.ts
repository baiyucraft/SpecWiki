import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";

import { afterAll, expect, test } from "vitest";

import { buildCodeBuddyBootstrapAssets, CODEBUDDY_ACTIONS } from "./assets.js";
import { evaluateCodeBuddyUserPrompt } from "./triggerAdapter.js";
import { parseHostTriggerCorpus } from "../shared/triggerCorpus.js";

const fixturePath = new URL("../../../tests/fixtures/host-trigger-corpus.v1.json", import.meta.url);
const temporaryDirectories: string[] = [];

function writeTemporaryHook(content: string): string {
  const directory = mkdtempSync(path.join(tmpdir(), "spec-wiki-hook-"));
  temporaryDirectories.push(directory);
  const filePath = path.join(directory, "hook.mjs");
  writeFileSync(filePath, content, "utf8");
  return filePath;
}

function executeHook(filePath: string, stdin: string): Record<string, unknown> {
  const result = spawnSync(process.execPath, [filePath], { input: stdin, encoding: "utf8" });
  expect(result.status, result.stderr).toBe(0);
  return JSON.parse(result.stdout) as Record<string, unknown>;
}

afterAll(() => {
  for (const directory of temporaryDirectories) {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("codeBuddy hooks 承载共享边界，不再生成 shared skill", () => {
  const assets = buildCodeBuddyBootstrapAssets("E:\\demo", CODEBUDDY_ACTIONS);
  const sessionStartHook = assets.find((asset) =>
    asset.kind === "hook" && asset.filePath.endsWith("\\session-start.mjs"),
  );
  const userPromptHook = assets.find((asset) =>
    asset.kind === "hook" && asset.filePath.endsWith("\\user-prompt-submit.mjs"),
  );
  const sessionStartContent = sessionStartHook?.kind === "hook" ? sessionStartHook.content : undefined;
  const userPromptContent = userPromptHook?.kind === "hook" ? userPromptHook.content : undefined;

  expect(
    assets.some((asset) => asset.kind === "skill" && asset.filePath.endsWith("\\spec-wiki-runtime\\SKILL.md")),
  ).toBe(false);
  expect(sessionStartContent).toContain("when the user asks about repo structure");
  expect(sessionStartContent).toContain("when you need a fast structured map of how the code works together");
  expect(userPromptContent).toContain("The shared trigger contract selected the wiki-");
  expect(userPromptContent).toContain("this hook only supplies context and does not execute the CLI");
  expect(userPromptContent).toContain("Do not execute a Wiki action until the request is unambiguous");
  expect(userPromptContent).not.toContain("const lowered = rawInput");
  expect(userPromptContent).not.toMatch(/child_process|spawnSync|execFile|spec-wiki query --json/);
  expect(userPromptContent?.match(/stdout\.write/g)).toHaveLength(1);
  expect(
    assets.some((asset) => asset.kind === "skill" && asset.filePath.endsWith("\\wiki-sync\\SKILL.md")),
  ).toBe(true);
  expect(
    assets.some((asset) => asset.kind === "skill" && asset.filePath.endsWith("\\wiki-rebuild\\SKILL.md")),
  ).toBe(true);
});

test("codeBuddy wiki-query skill 只引导宿主薄消费稳定 query 字段", () => {
  const assets = buildCodeBuddyBootstrapAssets("E:\\demo", CODEBUDDY_ACTIONS);
  const querySkill = assets.find((asset) =>
    asset.kind === "skill" && asset.filePath.endsWith("\\wiki-query\\SKILL.md"),
  );
  const queryContent = querySkill?.kind === "skill" ? querySkill.content : undefined;

  expect(queryContent).toContain(
    "description: \"Use when you need a fast structured map of where code lives and how files, modules, symbols, or call paths relate before deeper inspection.\"",
  );
  expect(queryContent).toContain("agent needs a fast map of how code works together");
  expect(queryContent).toContain("This is a working pattern, not a rigid output template.");
  expect(queryContent).toContain("`query_mode`");
  expect(queryContent).toContain("`query_trust`");
  expect(queryContent).toContain("`recommended_action`");
  expect(queryContent).toContain("`route_groups`");
  expect(queryContent).toContain("`answer`");
  expect(queryContent).not.toContain("matched_pages");
  expect(queryContent).not.toContain("provenance_summary");
  expect(queryContent).toContain("## When To Use");
  expect(queryContent).toContain("## How To Work");
  expect(queryContent).toContain("## After This");
  expect(queryContent).toContain("Do not post-process query output into host-specific Wiki business conclusions");
  expect(queryContent).toContain("Do not treat wiki-query as a replacement for `rg`, targeted file reads, or full implementation review");
  expect(queryContent).not.toContain("owner");
  expect(queryContent).not.toContain("entrypoint");
  expect(queryContent).not.toContain("impact");
  expect(queryContent).not.toContain("knowledge/page runtime refresh");
});

test("executes generated hooks against adapter corpus", () => {
  const assets = buildCodeBuddyBootstrapAssets("E:\\demo", CODEBUDDY_ACTIONS);
  const userPromptAsset = assets.find(asset =>
    asset.kind === "hook" && asset.filePath.endsWith("\\user-prompt-submit.mjs"),
  );
  const sessionStartAsset = assets.find(asset =>
    asset.kind === "hook" && asset.filePath.endsWith("\\session-start.mjs"),
  );
  expect(userPromptAsset?.kind).toBe("hook");
  expect(sessionStartAsset?.kind).toBe("hook");
  if (userPromptAsset?.kind !== "hook" || sessionStartAsset?.kind !== "hook") {
    throw new Error("generated CodeBuddy hook assets are missing");
  }

  const userPromptHook = writeTemporaryHook(userPromptAsset.content);
  const corpus = parseHostTriggerCorpus(JSON.parse(readFileSync(fixturePath, "utf8")));
  const adapterCases = [...corpus.adapterCases].sort(testCase =>
    testCase.id === "codebuddy-transcript-near-collision-001" ? -1 : 0,
  );
  for (const testCase of adapterCases) {
    const adapter = evaluateCodeBuddyUserPrompt(testCase.stdin);
    const expected = adapter.additionalContext
      ? { continue: true, additionalContext: adapter.additionalContext }
      : { continue: true };
    expect(executeHook(userPromptHook, testCase.stdin), testCase.id).toEqual(expected);
  }
  for (const testCase of corpus.semanticCases) {
    const stdin = JSON.stringify({
      hook_event_name: "UserPromptSubmit",
      user_prompt: testCase.input.text,
    });
    const adapter = evaluateCodeBuddyUserPrompt(stdin);
    const expected = adapter.additionalContext
      ? { continue: true, additionalContext: adapter.additionalContext }
      : { continue: true };
    expect(executeHook(userPromptHook, stdin), testCase.id).toEqual(expected);
  }

  const sessionStartHook = writeTemporaryHook(sessionStartAsset.content);
  const orientation = executeHook(sessionStartHook, "{malformed");
  expect(orientation).toMatchObject({ continue: true });
  expect(orientation.additionalContext).toContain("explicit skills wiki-status and wiki-query");
  expect(orientation.additionalContext).not.toMatch(/selected|executed|session_id/);
});
