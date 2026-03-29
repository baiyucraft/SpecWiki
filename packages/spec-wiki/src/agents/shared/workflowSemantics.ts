/**
 * 这个文件定义宿主无关的 Repo Wiki workflow semantics。
 * 各宿主 renderer 只消费这里的动作语义，不在各自模板里维护漂移副本。
 */
import { WIKI_ACTIONS, type WikiAction } from "../../wikiActions.js";

export type WorkflowActionSemantics = {
  /** runtime action 名称。 */
  action: WikiAction;
  /** 当前 action 的显式入口标签。 */
  slashCommandName: string;
  /** CodeBuddy action skill 名称。 */
  codeBuddySkillName: string;
  /** 面向用户的简短描述。 */
  description: string;
  /** 宿主最终需要调用的 CLI。 */
  cliCall: string;
  /** 需要参数提示时使用的 hint。 */
  argumentHint: string;
  /** 当前 action 的发布边界说明。 */
  actionNote: string;
  /** 更细粒度的执行步骤。 */
  steps: string[];
  /** 结果解释规则。 */
  output: string[];
  /** 动作级 guardrails。 */
  guardrails: string[];
};

const ACTION_DESCRIPTIONS: Record<WikiAction, string> = {
  init: "Initialize the repository's index-only Repo Wiki runtime.",
  status: "Check the current Repo Wiki runtime state for this repository.",
  update: "Refresh the repository's index-only Repo Wiki runtime.",
  query: "Query the repository's index-first Repo Wiki results.",
  sync: "Keep this entry point available, but do not treat it as a formal v0.1.0 guarantee.",
  rebuild: "Keep this entry point available, but do not treat it as a formal v0.1.0 guarantee.",
};

function renderCliCall(action: WikiAction): string {
  if (action === "query") {
    return "spec-wiki wiki query --term \"$ARGUMENTS\"";
  }

  return `spec-wiki wiki ${action}`;
}

function renderArgumentHint(action: WikiAction): string {
  return action === "query" ? "query terms" : "command arguments";
}

function renderActionNote(action: WikiAction): string {
  switch (action) {
    case "init":
      return "`v0.1.0` only guarantees an index-only init. Do not confuse `spec-wiki wiki init` with top-level `spec-wiki init`.";
    case "update":
      return "`v0.1.0` only guarantees index refresh, not a complete knowledge/page update.";
    case "query":
      return "`v0.1.0` only formally guarantees index-first structured results. Page fallback and knowledge/page projection are not release blockers.";
    case "sync":
    case "rebuild":
      return "This entry point can stay available, but `v0.1.0` does not describe it as a formal guarantee.";
    default:
      return "Do not reimplement Wiki business rules in host templates. Let `spec-wiki` and `wiki-runtime` own execution and state interpretation.";
  }
}

function createActionSteps(action: WikiAction): string[] {
  switch (action) {
    case "query":
      return [
        "Extract a concise query term from the user request, or use the user-provided query term directly.",
        `Run \`${renderCliCall(action)}\` from the repository root.`,
        "Summarize only from structured hits. Do not invent missing wiki/page data.",
      ];
    case "status":
      return [
        `Run \`${renderCliCall(action)}\` from the repository root.`,
        "Explain the current runtime state in user-facing language.",
      ];
    default:
      return [
        `Run \`${renderCliCall(action)}\` from the repository root.`,
        "Relay the final result or error as-is. Do not rewrite business semantics in the host layer.",
      ];
  }
}

function createActionOutput(action: WikiAction): string[] {
  switch (action) {
    case "query":
      return [
        "Prefer index-first structured hits.",
        "If the result is not enough to answer the question, state the coverage gap explicitly.",
      ];
    case "status":
      return [
        "Make it clear whether the repo has been bootstrapped and which runtime state it is in.",
        "Point to the next action when needed.",
      ];
    default:
      return [
        "Report the CLI end state without overstating runtime completeness.",
      ];
  }
}

function createActionGuardrails(action: WikiAction): string[] {
  const shared = [
    "Route the entry point back to `spec-wiki` CLI only. Do not rewrite Wiki business logic in the host layer.",
    "If CLI or runtime returns an error, pass the clear error through directly. Do not silently rewrite the state.",
  ];

  if (action === "query") {
    return [
      ...shared,
      "If the query result is insufficient, say so directly. Do not fill in symbol, module, or page content that does not exist.",
    ];
  }

  return shared;
}

const ACTION_SEMANTICS: Record<WikiAction, WorkflowActionSemantics> = Object.fromEntries(
  WIKI_ACTIONS.map((action) => [
    action,
    {
      action,
      slashCommandName: `/wiki:${action}`,
      codeBuddySkillName: `wiki-${action}`,
      description: ACTION_DESCRIPTIONS[action],
      cliCall: renderCliCall(action),
      argumentHint: renderArgumentHint(action),
      actionNote: renderActionNote(action),
      steps: createActionSteps(action),
      output: createActionOutput(action),
      guardrails: createActionGuardrails(action),
    },
  ]),
) as Record<WikiAction, WorkflowActionSemantics>;

export function getWorkflowActionSemantics(action: WikiAction): WorkflowActionSemantics {
  return ACTION_SEMANTICS[action];
}

export function listWorkflowActionSemantics(): WorkflowActionSemantics[] {
  return WIKI_ACTIONS.map((action) => ACTION_SEMANTICS[action]);
}
