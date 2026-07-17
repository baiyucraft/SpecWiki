/**
 * 这个文件严格解析版本化宿主触发测试语料。
 * Corpus 只服务 conformance，不接收真实用户 prompt，也不进入 Runtime 存储。
 */
import { PUBLIC_WIKI_ACTIONS, type PublicWikiAction } from "../../wikiActions.js";
import {
  TRIGGER_CONTRACT_VERSION,
  TRIGGER_DECISIONS,
  TRIGGER_REASONS,
  type TriggerDecisionKind,
  type TriggerEvaluationInput,
  type TriggerReason,
} from "./triggerContract.js";

export const HOST_TRIGGER_CORPUS_SCHEMA = "spec-wiki/host-trigger-corpus/v1" as const;

export type HostTriggerCorpusExpectedDecision = {
  decision: TriggerDecisionKind;
  targetAction: PublicWikiAction | null;
  reason: TriggerReason;
};

export type HostTriggerSemanticCase = {
  id: string;
  locale: "zh-CN" | "en";
  tags: readonly string[];
  input: TriggerEvaluationInput;
  expected: HostTriggerCorpusExpectedDecision;
};

export type HostTriggerAdapterCase = {
  id: string;
  host: "codebuddy";
  event: "UserPromptSubmit";
  stdin: string;
  expectedDecision: HostTriggerCorpusExpectedDecision;
  expectedDelivery: "action_context" | "clarification_context" | "none";
};

export type HostTriggerCorpus = {
  $schema: typeof HOST_TRIGGER_CORPUS_SCHEMA;
  contractVersion: typeof TRIGGER_CONTRACT_VERSION;
  revision: number;
  semanticCases: readonly HostTriggerSemanticCase[];
  adapterCases: readonly HostTriggerAdapterCase[];
};

function requireRecord(value: unknown, context: string): Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error(`${context} must be an object`);
  }
  return value as Record<string, unknown>;
}

function requireString(value: unknown, context: string): string {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new Error(`${context} must be a non-empty string`);
  }
  return value;
}

function requireStringArray(value: unknown, context: string): string[] {
  if (!Array.isArray(value) || value.some(item => typeof item !== "string" || item.length === 0)) {
    throw new Error(`${context} must be a string array`);
  }
  return [...value] as string[];
}

function parseExpectedDecision(value: unknown, context: string): HostTriggerCorpusExpectedDecision {
  const record = requireRecord(value, context);
  if (!TRIGGER_DECISIONS.includes(record.decision as TriggerDecisionKind)) {
    throw new Error(`${context}.decision is invalid`);
  }
  if (!TRIGGER_REASONS.includes(record.reason as TriggerReason)) {
    throw new Error(`${context}.reason is invalid`);
  }
  if (record.targetAction !== null && !PUBLIC_WIKI_ACTIONS.includes(record.targetAction as PublicWikiAction)) {
    throw new Error(`${context}.targetAction is invalid`);
  }
  if (record.decision === "should_trigger" && record.targetAction === null) {
    throw new Error(`${context}.targetAction is required for should_trigger`);
  }
  if (record.decision === "should_not_trigger" && record.targetAction !== null) {
    throw new Error(`${context}.targetAction must be null for should_not_trigger`);
  }
  if (
    record.decision === "ambiguous"
    && record.targetAction !== null
    && !(record.reason === "missing_query_term" && record.targetAction === "query")
  ) {
    throw new Error(`${context}.targetAction is invalid for ambiguous`);
  }
  return {
    decision: record.decision as TriggerDecisionKind,
    targetAction: record.targetAction as PublicWikiAction | null,
    reason: record.reason as TriggerReason,
  };
}

function parseSemanticCase(value: unknown, index: number): HostTriggerSemanticCase {
  const context = `semanticCases[${index}]`;
  const record = requireRecord(value, context);
  const locale = record.locale;
  if (locale !== "zh-CN" && locale !== "en") {
    throw new Error(`${context}.locale is invalid`);
  }
  const input = requireRecord(record.input, `${context}.input`);
  if (input.kind !== "user_prompt" || !("text" in input)) {
    throw new Error(`${context}.input must be a user_prompt with text`);
  }
  return {
    id: requireString(record.id, `${context}.id`),
    locale,
    tags: Object.freeze(requireStringArray(record.tags, `${context}.tags`)),
    input: Object.freeze({ kind: "user_prompt", text: input.text }),
    expected: Object.freeze(parseExpectedDecision(record.expected, `${context}.expected`)),
  };
}

function parseAdapterCase(value: unknown, index: number): HostTriggerAdapterCase {
  const context = `adapterCases[${index}]`;
  const record = requireRecord(value, context);
  if (record.host !== "codebuddy" || record.event !== "UserPromptSubmit") {
    throw new Error(`${context} must target codebuddy UserPromptSubmit`);
  }
  if (!["action_context", "clarification_context", "none"].includes(String(record.expectedDelivery))) {
    throw new Error(`${context}.expectedDelivery is invalid`);
  }
  return {
    id: requireString(record.id, `${context}.id`),
    host: "codebuddy",
    event: "UserPromptSubmit",
    stdin: requireString(record.stdin, `${context}.stdin`),
    expectedDecision: Object.freeze(parseExpectedDecision(record.expectedDecision, `${context}.expectedDecision`)),
    expectedDelivery: record.expectedDelivery as HostTriggerAdapterCase["expectedDelivery"],
  };
}

/**
 * 严格解析 host-trigger corpus v1。
 *
 * @param value 从 JSON 读取的未知值。
 * @returns 冻结的 conformance corpus。
 * @throws schema、版本、case 字段或 ID 不合法时抛出错误。
 */
export function parseHostTriggerCorpus(value: unknown): HostTriggerCorpus {
  const record = requireRecord(value, "host trigger corpus");
  if (record.$schema !== HOST_TRIGGER_CORPUS_SCHEMA) {
    throw new Error("host trigger corpus $schema is invalid");
  }
  if (record.contractVersion !== TRIGGER_CONTRACT_VERSION) {
    throw new Error("host trigger corpus contractVersion is invalid");
  }
  if (!Number.isInteger(record.revision) || Number(record.revision) < 1) {
    throw new Error("host trigger corpus revision must be a positive integer");
  }
  if (!Array.isArray(record.semanticCases) || !Array.isArray(record.adapterCases)) {
    throw new TypeError("host trigger corpus cases must be arrays");
  }

  const semanticCases = record.semanticCases.map(parseSemanticCase);
  const adapterCases = record.adapterCases.map(parseAdapterCase);
  const caseIds = [...semanticCases, ...adapterCases].map(testCase => testCase.id);
  if (new Set(caseIds).size !== caseIds.length) {
    throw new Error("host trigger corpus contains duplicate case ids");
  }

  return Object.freeze({
    $schema: HOST_TRIGGER_CORPUS_SCHEMA,
    contractVersion: TRIGGER_CONTRACT_VERSION,
    revision: record.revision as number,
    semanticCases: Object.freeze(semanticCases.map(testCase => Object.freeze(testCase))),
    adapterCases: Object.freeze(adapterCases.map(testCase => Object.freeze(testCase))),
  });
}
