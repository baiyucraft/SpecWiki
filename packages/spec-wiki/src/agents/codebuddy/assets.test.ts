import { expect, test } from "vitest";

import { buildCodeBuddyBootstrapAssets, CODEBUDDY_ACTIONS } from "./assets.js";

test("codeBuddy hooks 承载共享边界，不再生成 shared skill", () => {
  const assets = buildCodeBuddyBootstrapAssets("E:\\demo", CODEBUDDY_ACTIONS);
  const sessionStartHook = assets.find((asset) =>
    asset.kind === "hook" && asset.filePath.endsWith("\\session-start.mjs"),
  );
  const userPromptHook = assets.find((asset) =>
    asset.kind === "hook" && asset.filePath.endsWith("\\user-prompt-submit.mjs"),
  );

  expect(
    assets.some((asset) => asset.kind === "skill" && asset.filePath.endsWith("\\spec-wiki-runtime\\SKILL.md")),
  ).toBe(false);
  expect(sessionStartHook?.content).toContain("when the user asks about repo structure");
  expect(sessionStartHook?.content).toContain("when you need a fast structured map of how the code works together");
  expect(userPromptHook?.content).toContain("authoritative runtime data");
  expect(userPromptHook?.content).toContain("shared rules live in hooks and action skill guardrails");
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

  expect(querySkill?.content).toContain(
    "description: \"Use when you need a fast structured map of where code lives and how files, modules, symbols, or call paths relate before deeper inspection.\"",
  );
  expect(querySkill?.content).toContain("agent needs a fast map of how code works together");
  expect(querySkill?.content).toContain("This is a working pattern, not a rigid output template.");
  expect(querySkill?.content).toContain("`query_mode`");
  expect(querySkill?.content).toContain("`query_trust`");
  expect(querySkill?.content).toContain("`recommended_action`");
  expect(querySkill?.content).toContain("`matched_pages`");
  expect(querySkill?.content).toContain("`provenance_summary`");
  expect(querySkill?.content).toContain("## When To Use");
  expect(querySkill?.content).toContain("## How To Work");
  expect(querySkill?.content).toContain("## After This");
  expect(querySkill?.content).toContain("Do not post-process query output into host-specific Wiki business conclusions");
  expect(querySkill?.content).toContain("Do not treat wiki-query as a replacement for `rg`, targeted file reads, or full implementation review");
  expect(querySkill?.content).not.toContain("owner");
  expect(querySkill?.content).not.toContain("entrypoint");
  expect(querySkill?.content).not.toContain("impact");
  expect(querySkill?.content).not.toContain("knowledge/page runtime refresh");
});
