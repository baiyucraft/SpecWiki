import { existsSync, readFileSync } from "node:fs";

import { expect, test } from "vitest";

import { parseHostTriggerCorpus } from "../shared/triggerCorpus.js";

const adapterPath = new URL("./triggerAdapter.ts", import.meta.url);
const fixturePath = new URL("../../../tests/fixtures/host-trigger-corpus.v1.json", import.meta.url);

async function loadAdapter() {
  return import(adapterPath.href);
}

test("parses only the supported user_prompt envelope and fails closed", async () => {
  expect(existsSync(adapterPath), "CodeBuddy trigger adapter must exist").toBe(true);
  const { evaluateCodeBuddyUserPrompt } = await loadAdapter();
  const corpus = parseHostTriggerCorpus(JSON.parse(readFileSync(fixturePath, "utf8")));

  for (const testCase of corpus.adapterCases) {
    const result = evaluateCodeBuddyUserPrompt(testCase.stdin);
    expect(result.decision, testCase.id).toMatchObject(testCase.expectedDecision);
    expect(result.delivery, testCase.id).toBe(testCase.expectedDelivery);
  }

  const wrongEvent = evaluateCodeBuddyUserPrompt(JSON.stringify({
    hook_event_name: "PreToolUse",
    user_prompt: "spec-wiki rebuild",
  }));
  expect(wrongEvent).toMatchObject({
    delivery: "none",
    decision: { decision: "ambiguous", reason: "invalid_input" },
  });
  expect(evaluateCodeBuddyUserPrompt(JSON.stringify({
    hook_event_name: "UserPromptSubmit",
    userPrompt: "spec-wiki rebuild",
  }))).toMatchObject({
    delivery: "none",
    decision: { decision: "ambiguous", reason: "invalid_input" },
  });
  expect(evaluateCodeBuddyUserPrompt("[]")).toMatchObject({
    delivery: "none",
    decision: { decision: "ambiguous", reason: "invalid_input" },
  });
  expect(JSON.stringify(wrongEvent)).not.toMatch(/session_id|session_summary|recent_turns|tool_artifact_refs/);

  const positive = evaluateCodeBuddyUserPrompt(JSON.stringify({
    hook_event_name: "UserPromptSubmit",
    user_prompt: "spec-wiki query auth",
    session_id: "must-not-leak",
  }));
  expect(positive.additionalContext).not.toMatch(/must-not-leak|session_id|transcript/);
});
