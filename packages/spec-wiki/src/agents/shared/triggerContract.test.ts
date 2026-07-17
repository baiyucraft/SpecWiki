import { existsSync } from "node:fs";
import { expect, test } from "vitest";

const contractPath = new URL("./triggerContract.ts", import.meta.url);

async function loadContract() {
  return import(contractPath.href);
}

test("exposes closed policies and rejects invalid decisions", async () => {
  expect(existsSync(contractPath), "shared trigger contract must exist").toBe(true);

  const contract = await loadContract();
  expect(Object.fromEntries(contract.PUBLIC_TRIGGER_ACTIONS.map((action: string) => [
    action,
    contract.getTriggerActionPolicy(action),
  ]))).toEqual({
    init: "explicit_only",
    status: "semantic_or_explicit",
    update: "explicit_only",
    query: "semantic_or_explicit",
    sync: "explicit_only",
    rebuild: "explicit_only",
  });

  expect(() => contract.validateTriggerDecision({
    contractVersion: contract.TRIGGER_CONTRACT_VERSION,
    decision: "should_trigger",
    targetAction: null,
    reason: "explicit_action_request",
    evidence: [{ kind: "explicit_action", ruleId: "action.query.explicit.v1" }],
  })).toThrow(/targetAction/);
  expect(() => contract.validateTriggerDecision({
    contractVersion: contract.TRIGGER_CONTRACT_VERSION,
    decision: "should_trigger",
    targetAction: "query",
    reason: "explicit_opt_out",
    evidence: [{ kind: "negation", ruleId: "intent.opt_out.v1" }],
  })).toThrow(/reason/);

  const valid = contract.validateTriggerDecision({
    contractVersion: contract.TRIGGER_CONTRACT_VERSION,
    decision: "should_trigger",
    targetAction: "status",
    reason: "explicit_action_request",
    evidence: [{ kind: "explicit_action", ruleId: "action.status.explicit.v1" }],
  });
  expect(Object.isFrozen(valid)).toBe(true);
  expect(Object.isFrozen(valid.evidence)).toBe(true);

  const normalizedEvidence = contract.validateTriggerDecision({
    contractVersion: contract.TRIGGER_CONTRACT_VERSION,
    decision: "ambiguous",
    targetAction: null,
    reason: "conflicting_intent",
    evidence: [
      { kind: "conflict", ruleId: "intent.conflict.v1" },
      { kind: "negation", ruleId: "intent.opt_out.v1" },
      { kind: "conflict", ruleId: "intent.conflict.v1" },
    ],
  });
  expect(normalizedEvidence.evidence).toEqual([
    { kind: "negation", ruleId: "intent.opt_out.v1" },
    { kind: "conflict", ruleId: "intent.conflict.v1" },
  ]);
});

test("classifies normalized explicit, semantic, negative, conflict, and near-collision inputs", async () => {
  const contract = await loadContract();
  expect(typeof contract.evaluateHostTrigger, "shared evaluator must be exported").toBe("function");

  const cases = [
    ["帮我定位认证模块的调用路径", "should_trigger", "query", "semantic_query_request"],
    ["Show me where the auth module is implemented", "should_trigger", "query", "semantic_query_request"],
    ["Wiki 是否已经就绪，需要 update 吗？", "should_trigger", "status", "semantic_status_request"],
    ["show whether the repo wiki is stale", "should_trigger", "status", "semantic_status_request"],
    ["spec-wiki query auth module", "should_trigger", "query", "explicit_action_request"],
    ["spec-wiki query", "ambiguous", "query", "missing_query_term"],
    ["please run spec-wiki rebuild", "should_trigger", "rebuild", "explicit_action_request"],
    ["how does rebuild work?", "should_not_trigger", null, "mutation_requires_explicit_request"],
    ["不要查询 wiki", "should_not_trigger", null, "explicit_opt_out"],
    ["query auth but do not use the wiki", "ambiguous", null, "conflicting_intent"],
    ["spec-wiki update and spec-wiki rebuild", "ambiguous", null, "conflicting_intent"],
    ["wiki", "ambiguous", null, "insufficient_context"],
    ["module.exports = value", "should_not_trigger", null, "no_supported_intent"],
    ["classpath status quo", "should_not_trigger", null, "no_supported_intent"],
  ] as const;

  for (const [text, decision, targetAction, reason] of cases) {
    expect(contract.evaluateHostTrigger({ kind: "user_prompt", text }), text).toMatchObject({
      decision,
      targetAction,
      reason,
    });
  }

  expect(contract.evaluateHostTrigger({ kind: "user_prompt", text: null })).toMatchObject({
    decision: "ambiguous",
    targetAction: null,
    reason: "invalid_input",
  });
});
