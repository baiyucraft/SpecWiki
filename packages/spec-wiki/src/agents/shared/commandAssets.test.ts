import { expect, test } from "vitest";

import { renderCodeBuddyActionSkill, renderHostActionSkill } from "./commandAssets.js";

test("共享 host skill 引导宿主薄消费稳定 query 字段", () => {
  const skill = renderHostActionSkill("query");

  expect(skill).toContain("name: wiki-query");
  expect(skill).toContain(
    "description: \"Use when you need a fast structured map of where code lives and how files, modules, symbols, or call paths relate before deeper inspection.\"",
  );
  expect(skill).toContain("This is a working pattern, not a rigid output template.");
  expect(skill).toContain("Use it proactively when the agent needs a fast structured map of how code works together");
  expect(skill).toContain("## When To Use");
  expect(skill).toContain("## How To Work");
  expect(skill).toContain("## After This");
  expect(skill).toContain("## Action Notes");
  expect(skill).toContain("`query_mode`");
  expect(skill).toContain("`query_trust`");
  expect(skill).toContain("`recommended_action`");
  expect(skill).toContain("`matched_pages`");
  expect(skill).toContain("`provenance_summary`");
  expect(skill).toContain("instead of restating the JSON payload");
  expect(skill).toContain("Do not rebuild a new Wiki state machine or page semantic layer from query results");
  expect(skill).toContain("Do not treat wiki-query as a replacement for `rg`, targeted file reads, or full implementation review");
  expect(skill).not.toContain("owner");
  expect(skill).not.toContain("entrypoint");
  expect(skill).not.toContain("impact");
});

test("共享 host skill 与 CodeBuddy skill 共享同一动作骨架", () => {
  const hostSkill = renderHostActionSkill("status");
  const codeBuddySkill = renderCodeBuddyActionSkill("status");

  expect(hostSkill).toContain(
    "description: \"Use when you need to know whether the wiki is ready, stale, blocked, or needs refresh.\"",
  );
  expect(hostSkill).toContain("## When To Use");
  expect(hostSkill).toContain("## Run");
  expect(hostSkill).toContain("## Interpret");
  expect(hostSkill).toContain("## Guardrails");
  expect(hostSkill).toContain("`status` is only an inspection entry point");
  expect(hostSkill).not.toContain("knowledge/page runtime refresh");
  expect(codeBuddySkill).toContain("## When To Use");
  expect(codeBuddySkill).toContain("## Run");
  expect(codeBuddySkill).toContain("## Interpret");
  expect(codeBuddySkill).toContain("## Guardrails");
});
