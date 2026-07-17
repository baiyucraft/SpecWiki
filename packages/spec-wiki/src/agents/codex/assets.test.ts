import { expect, test } from "vitest";

import { CODEX_ACTIONS, renderCodexSkill, resolveCodexSkillPath } from "./assets.js";

test("codex is the repo-local reference projection for every public action", () => {
  expect(CODEX_ACTIONS).toEqual(["init", "status", "update", "query", "sync", "rebuild"]);
  for (const action of CODEX_ACTIONS) {
    expect(resolveCodexSkillPath("E:\\demo", action)).toMatch(
      new RegExp(`\\\\.codex\\\\skills\\\\wiki-${action}\\\\SKILL\\.md$`),
    );
    const skill = renderCodexSkill(action);
    expect(skill).toContain(`name: wiki-${action}`);
    expect(skill).toContain(action === "query" || action === "status"
      ? "Trigger policy: semantic or explicit"
      : "Trigger policy: explicit request only");
  }
});
