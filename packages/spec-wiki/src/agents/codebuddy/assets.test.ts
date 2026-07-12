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
  const sessionStartContent = sessionStartHook?.kind === "hook" ? sessionStartHook.content : undefined;
  const userPromptContent = userPromptHook?.kind === "hook" ? userPromptHook.content : undefined;

  expect(
    assets.some((asset) => asset.kind === "skill" && asset.filePath.endsWith("\\spec-wiki-runtime\\SKILL.md")),
  ).toBe(false);
  expect(sessionStartContent).toContain("when the user asks about repo structure");
  expect(sessionStartContent).toContain("when you need a fast structured map of how the code works together");
  expect(userPromptContent).toContain("authoritative runtime data");
  expect(userPromptContent).toContain("shared rules live in hooks and action skill guardrails");
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
  expect(queryContent).toContain("`matched_pages`");
  expect(queryContent).toContain("`provenance_summary`");
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
