/**
 * 这个文件定义宿主无关的 Wiki action 触发对象语言。
 * 它只归约用户意图，不读取文件、不调用 CLI，也不携带宿主或会话状态。
 */
import { PUBLIC_WIKI_ACTIONS, type PublicWikiAction } from "../../wikiActions.js";
import {
  createHostTriggerRuntime,
  type EmbeddedTriggerRuntimeConfig,
} from "./triggerRuntime.js";

/** Trigger 层复用的公开 action 列表，不创建第二份 action 闭集。 */
export const PUBLIC_TRIGGER_ACTIONS = PUBLIC_WIKI_ACTIONS;

/** 当前宿主触发 taxonomy 与 reducer 的稳定版本。 */
export const TRIGGER_CONTRACT_VERSION = "host-trigger/v1" as const;

/** 宿主触发决策的闭集。 */
export const TRIGGER_DECISIONS = [
  "should_trigger",
  "should_not_trigger",
  "ambiguous",
] as const;

/** 宿主触发决策原因的 v1 闭集。 */
export const TRIGGER_REASONS = [
  "explicit_action_request",
  "semantic_query_request",
  "semantic_status_request",
  "explicit_opt_out",
  "mutation_requires_explicit_request",
  "no_supported_intent",
  "insufficient_context",
  "missing_query_term",
  "conflicting_intent",
  "invalid_input",
] as const;

/** 决策证据类型的 v1 闭集。 */
export const TRIGGER_EVIDENCE_KINDS = [
  "explicit_action",
  "semantic_intent",
  "negation",
  "mutation_guard",
  "conflict",
  "input_validation",
  "no_match",
] as const;

/** 公开 action 的触发策略闭集。 */
export const TRIGGER_ACTION_POLICIES = ["semantic_or_explicit", "explicit_only"] as const;

export type TriggerDecisionKind = (typeof TRIGGER_DECISIONS)[number];
export type TriggerReason = (typeof TRIGGER_REASONS)[number];
export type TriggerEvidenceKind = (typeof TRIGGER_EVIDENCE_KINDS)[number];
export type TriggerActionPolicy = (typeof TRIGGER_ACTION_POLICIES)[number];

/** 单条稳定、可审计且不包含原始 prompt 的决策证据。 */
export type TriggerEvidence = {
  kind: TriggerEvidenceKind;
  ruleId: string;
};

/** 宿主触发层的唯一输出对象。 */
export type TriggerDecision = {
  contractVersion: typeof TRIGGER_CONTRACT_VERSION;
  decision: TriggerDecisionKind;
  targetAction: PublicWikiAction | null;
  reason: TriggerReason;
  evidence: readonly TriggerEvidence[];
};

/** Evaluator 接受的宿主无关 prompt 输入。 */
export type TriggerEvaluationInput = {
  kind: "user_prompt";
  text: unknown;
};

const ACTION_POLICIES: Readonly<Record<PublicWikiAction, TriggerActionPolicy>> = {
  query: "semantic_or_explicit",
  status: "semantic_or_explicit",
  init: "explicit_only",
  update: "explicit_only",
  sync: "explicit_only",
  rebuild: "explicit_only",
};

/** 可序列化到生成 hook 的共享 reducer 配置。 */
export const HOST_TRIGGER_RUNTIME_CONFIG: EmbeddedTriggerRuntimeConfig = Object.freeze({
  contractVersion: TRIGGER_CONTRACT_VERSION,
  actions: PUBLIC_WIKI_ACTIONS,
});

const hostTriggerRuntime = createHostTriggerRuntime(HOST_TRIGGER_RUNTIME_CONFIG);

/**
 * 返回公开 Wiki action 的 trigger policy。
 *
 * @param action 公开 action。
 * @returns query/status 允许语义建议，其余 action 只允许显式请求。
 */
export function getTriggerActionPolicy(action: PublicWikiAction): TriggerActionPolicy {
  return ACTION_POLICIES[action];
}

function isClosedValue<T extends string>(values: readonly T[], value: unknown): value is T {
  return typeof value === "string" && values.includes(value as T);
}

/**
 * 校验并冻结一个 trigger decision。
 *
 * @param decision 待校验对象。
 * @returns 保持输入语义的只读 decision。
 * @throws 当闭集值、target invariant 或 evidence 非法时抛出合同错误。
 */
export function validateTriggerDecision(decision: TriggerDecision): TriggerDecision {
  if (decision.contractVersion !== TRIGGER_CONTRACT_VERSION) {
    throw new Error(`unsupported trigger contract version: ${String(decision.contractVersion)}`);
  }
  if (!isClosedValue(TRIGGER_DECISIONS, decision.decision)) {
    throw new Error(`invalid trigger decision: ${String(decision.decision)}`);
  }
  if (!isClosedValue(TRIGGER_REASONS, decision.reason)) {
    throw new Error(`invalid trigger reason: ${String(decision.reason)}`);
  }
  if (decision.targetAction !== null && !PUBLIC_WIKI_ACTIONS.includes(decision.targetAction)) {
    throw new Error(`invalid trigger targetAction: ${String(decision.targetAction)}`);
  }
  if (decision.decision === "should_trigger" && decision.targetAction === null) {
    throw new Error("should_trigger requires a targetAction");
  }
  if (decision.decision === "should_not_trigger" && decision.targetAction !== null) {
    throw new Error("should_not_trigger requires targetAction=null");
  }
  if (
    decision.decision === "ambiguous"
    && decision.targetAction !== null
    && !(decision.reason === "missing_query_term" && decision.targetAction === "query")
  ) {
    throw new Error("ambiguous targetAction is only valid for a missing query term");
  }
  if (!Array.isArray(decision.evidence) || decision.evidence.length === 0) {
    throw new Error("trigger decision requires evidence");
  }

  const allowedReasons: Readonly<Record<TriggerDecisionKind, readonly TriggerReason[]>> = {
    should_trigger: [
      "explicit_action_request",
      "semantic_query_request",
      "semantic_status_request",
    ],
    should_not_trigger: [
      "explicit_opt_out",
      "mutation_requires_explicit_request",
      "no_supported_intent",
    ],
    ambiguous: [
      "insufficient_context",
      "missing_query_term",
      "conflicting_intent",
      "invalid_input",
    ],
  };
  if (!allowedReasons[decision.decision].includes(decision.reason)) {
    throw new Error(`trigger reason ${decision.reason} is invalid for ${decision.decision}`);
  }
  if (decision.reason === "semantic_query_request" && decision.targetAction !== "query") {
    throw new Error("semantic_query_request requires targetAction=query");
  }
  if (decision.reason === "semantic_status_request" && decision.targetAction !== "status") {
    throw new Error("semantic_status_request requires targetAction=status");
  }

  const evidence = decision.evidence.map((item) => {
    if (!isClosedValue(TRIGGER_EVIDENCE_KINDS, item.kind) || item.ruleId.trim().length === 0) {
      throw new Error("invalid trigger evidence");
    }
    return { kind: item.kind, ruleId: item.ruleId };
  }).filter((item, index, items) =>
    items.findIndex(candidate => candidate.kind === item.kind && candidate.ruleId === item.ruleId) === index,
  ).sort((left, right) => {
    const kindOrder = TRIGGER_EVIDENCE_KINDS.indexOf(left.kind) - TRIGGER_EVIDENCE_KINDS.indexOf(right.kind);
    return kindOrder === 0 ? left.ruleId.localeCompare(right.ruleId) : kindOrder;
  }).map(item => Object.freeze(item));

  return Object.freeze({ ...decision, evidence: Object.freeze(evidence) });
}

/** 对 prompt 做 evaluator 唯一允许的标准化。 */
export function normalizeTriggerText(text: string): string {
  return hostTriggerRuntime.normalize(text);
}

/**
 * 把单条用户 prompt 归约为稳定 trigger decision。
 *
 * @param input 宿主无关的 prompt 输入。
 * @returns 冻结的三态决策；函数不执行任何 I/O 或公开 action。
 */
export function evaluateHostTrigger(input: TriggerEvaluationInput): TriggerDecision {
  return validateTriggerDecision(hostTriggerRuntime.evaluate(input) as TriggerDecision);
}
