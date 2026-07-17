import { expect, test } from "vitest";

import { renderCodeBuddyActionSkill, renderHostActionSkill } from "./commandAssets.js";

test("query skill consumes canonical groups and defers richer inputs", () => {
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
  expect(skill).toContain("`readiness`");
  expect(skill).toContain("`query_mode`");
  expect(skill).toContain("`query_trust`");
  expect(skill).toContain("`recommended_action`");
  expect(skill).toContain("`governance`");
  expect(skill).toContain("`route_groups`");
  expect(skill).toContain("`answer`");
  expect(skill).not.toContain("matched_pages");
  expect(skill).not.toContain("provenance_summary");
  expect(skill).not.toContain("flat `results`");
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

test("sync 与 rebuild skill 使用正式公开入口文案", () => {
  const syncSkill = renderHostActionSkill("sync");
  const rebuildSkill = renderCodeBuddyActionSkill("rebuild");

  expect(syncSkill).toContain("name: wiki-sync");
  expect(syncSkill).toContain("Sync managed `.wiki` page edits back into runtime state, metadata, and cache.");
  expect(syncSkill).toContain("`sync` only applies `.wiki` page edits back into runtime state. It does not replace `update`.");
  expect(syncSkill).not.toContain("formal guarantee");
  expect(rebuildSkill).toContain("name: wiki-rebuild");
  expect(rebuildSkill).toContain("Use when you need to force a full rebuild of the wiki runtime.");
  expect(rebuildSkill).toContain("`rebuild` is the explicit full runtime rebuild entry point. It does not replace `update`.");
  expect(rebuildSkill).not.toContain("formal guarantee");
});

test("shared skills project semantic and explicit-only trigger policies", () => {
  for (const action of ["query", "status"] as const) {
    const skill = renderHostActionSkill(action);
    expect(skill).toContain("Trigger policy: semantic or explicit");
    expect(skill).toContain("does not execute from trigger guidance alone");
  }

  for (const action of ["init", "update", "sync", "rebuild"] as const) {
    const hostSkill = renderHostActionSkill(action);
    const codeBuddySkill = renderCodeBuddyActionSkill(action);
    expect(hostSkill).toContain("Trigger policy: explicit request only");
    expect(hostSkill).toContain("Do not select or run this action from a description, discussion, or ambiguous request");
    expect(codeBuddySkill).toContain("Trigger policy: explicit request only");
  }
});
