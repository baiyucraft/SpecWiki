/**
 * CodeBuddy v1 UserPromptSubmit 包络适配器。
 * 它只提取官方 `user_prompt` 字段，把宿主无关决策映射为 context delivery。
 */
import {
  evaluateHostTrigger,
  type TriggerDecision,
} from "../shared/triggerContract.js";

export const CODEBUDDY_TRIGGER_DELIVERIES = [
  "action_context",
  "clarification_context",
  "none",
] as const;

export type CodeBuddyTriggerDelivery = (typeof CODEBUDDY_TRIGGER_DELIVERIES)[number];

export type CodeBuddyTriggerEvaluation = {
  decision: TriggerDecision;
  delivery: CodeBuddyTriggerDelivery;
  additionalContext?: string;
};

type HostTriggerEvaluator = (input: { kind: "user_prompt"; text: unknown }) => TriggerDecision;

/**
 * 创建可嵌入生成 hook 的 CodeBuddy v1 adapter。
 * Factory 只依赖注入的共享 evaluator，不捕获会话或模块状态。
 */
export function createCodeBuddyUserPromptEvaluator(
  evaluateTrigger: HostTriggerEvaluator,
): (raw: string) => CodeBuddyTriggerEvaluation {
  function invalidInput(): CodeBuddyTriggerEvaluation {
    return {
      decision: evaluateTrigger({ kind: "user_prompt", text: null }),
      delivery: "none",
    };
  }

  function renderActionContext(decision: TriggerDecision): string {
    return [
      `The shared trigger contract selected the wiki-${decision.targetAction} skill.`,
      `Decision reason: ${decision.reason}.`,
      "Use the repo-local skill guidance and stable runtime fields; this hook only supplies context and does not execute the CLI.",
    ].join(" ");
  }

  function renderClarificationContext(decision: TriggerDecision): string {
    const guidance = decision.reason === "missing_query_term"
      ? "Ask the user which term, symbol, module, file, or call path they want to query."
      : decision.reason === "conflicting_intent"
        ? "Ask the user to choose one Wiki action and resolve any opt-out conflict."
        : "Ask the user to name the intended Wiki action or request more context.";
    return `${guidance} Do not execute a Wiki action until the request is unambiguous.`;
  }

  return function evaluateCodeBuddyUserPrompt(raw: string): CodeBuddyTriggerEvaluation {
    let value: unknown;
    try {
      value = JSON.parse(raw);
    } catch {
      return invalidInput();
    }

    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      return invalidInput();
    }
    const event = value as Record<string, unknown>;
    if (event.hook_event_name !== "UserPromptSubmit" || typeof event.user_prompt !== "string") {
      return invalidInput();
    }

    const decision = evaluateTrigger({ kind: "user_prompt", text: event.user_prompt });
    if (decision.decision === "should_trigger") {
      return {
        decision,
        delivery: "action_context",
        additionalContext: renderActionContext(decision),
      };
    }
    if (decision.decision === "ambiguous" && decision.reason !== "invalid_input") {
      return {
        decision,
        delivery: "clarification_context",
        additionalContext: renderClarificationContext(decision),
      };
    }

    return { decision, delivery: "none" };
  };
}

/** 解析 CodeBuddy v1 stdin 并返回 context-only delivery。 */
export const evaluateCodeBuddyUserPrompt = createCodeBuddyUserPromptEvaluator(evaluateHostTrigger);
