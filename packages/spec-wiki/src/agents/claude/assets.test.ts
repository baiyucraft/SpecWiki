import { expect, test } from "vitest";

import { renderCodexSkill } from "../codex/assets.js";
import { CLAUDE_ACTIONS, renderClaudeSkill, resolveClaudeSkillPath } from "./assets.js";

function triggerPolicyLines(skill: string): string[] {
  return skill.split("\n").filter(line => line.includes("Trigger policy:"));
}

test("claude is a repo-local compatible projection of Codex trigger guidance", () => {
  for (const action of CLAUDE_ACTIONS) {
    expect(resolveClaudeSkillPath("E:\\demo", action)).toMatch(
      new RegExp(`\\\\.claude\\\\skills\\\\wiki-${action}\\\\SKILL\\.md$`),
    );
    const claudeSkill = renderClaudeSkill(action);
    expect(triggerPolicyLines(claudeSkill)).toEqual(triggerPolicyLines(renderCodexSkill(action)));
    if (action === "query") {
      expect(claudeSkill).toContain("`route_groups`");
      expect(claudeSkill).not.toMatch(/matched_pages|provenance_summary|intent|scope|traversal/);
    }
  }
});
