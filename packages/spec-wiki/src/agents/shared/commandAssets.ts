/**
 * 这个文件收口 Claude / Codex / CodeBuddy 的显式入口渲染逻辑。
 * 它消费共享 workflow semantics，而不是继续直接读取宿主 markdown 模板。
 */
import type { WikiAction } from "../../wikiActions.js";
import {
  getWorkflowActionSemantics,
  type WorkflowActionSemantics,
} from "./workflowSemantics.js";

function renderBulletList(items: string[]): string {
  return items.map((item) => `- ${item}`).join("\n");
}

function renderFrontmatterString(value: string): string {
  return JSON.stringify(value);
}

const QUERY_STABLE_FIELDS = [
  "`query_mode`",
  "`query_trust`",
  "`recommended_action`",
  "`matched_pages`",
  "`provenance_summary`",
];

function renderActionDescription(action: WikiAction): string {
  switch (action) {
    case "status":
      return "Use when you need to know whether the wiki is ready, stale, blocked, or needs refresh.";
    case "query":
      return "Use when you need a fast structured map of where code lives and how files, modules, symbols, or call paths relate before deeper inspection.";
    case "init":
      return "Use when the repository has not been bootstrapped for spec-wiki and you need to initialize the wiki runtime.";
    case "update":
      return "Use when the wiki already exists and you need to refresh the current knowledge runtime after source changes.";
    case "sync":
      return "Use when you need to sync managed `.wiki` page edits back into the runtime state.";
    case "rebuild":
      return "Use when you need to force a full rebuild of the wiki runtime.";
  }
}

function renderActionSection(title: string, items: string[]): string {
  return [`## ${title}`, renderBulletList(items), ""].join("\n");
}

function renderQuerySkillIntro(): string[] {
  return [
    "Use `wiki-query` when you need a fast structured map of how code works together before deeper inspection.",
    "",
    "This is a working pattern, not a rigid output template.",
    "",
  ];
}

function renderQueryWhenToUse(): string[] {
  return [
    "Use it when the user asks where code lives, what a module does, or which files, modules, symbols, or call paths relate to a concept.",
    "Use it proactively when the agent needs a fast structured map of how code works together before deciding which files to inspect next.",
  ];
}

function renderQueryHowToWork(): string[] {
  return [
    "Extract a concise query term from the request, or use the provided term directly.",
    "Run `spec-wiki wiki query --term \"$ARGUMENTS\"` from the repository root.",
    `Look at these stable fields first to decide whether the result is sufficient: ${QUERY_STABLE_FIELDS.join(", ")}.`,
    "Treat the query result as a structured map for narrowing the search space. Prioritize relevant pages, files, symbols, or call-path clues instead of restating the JSON payload.",
  ];
}

function renderQueryAfterThis(): string[] {
  return [
    "If query already answers the question, respond directly from the hits.",
    "If query only gives direction, continue with `rg`, targeted file reads, symbol-level analysis, or implementation-level verification.",
    "If the result is insufficient, state the coverage gap clearly and suggest a better next action or a narrower query.",
  ];
}

function renderHostActionPurpose(action: WikiAction): string[] {
  switch (action) {
    case "status":
      return ["Check whether the wiki is ready, stale, blocked, or what should happen next."];
    case "query":
      return [
        "Use this when the user asks where code lives, what a module does, or which files, modules, symbols, or call paths relate to a concept.",
        "Use it proactively when the agent needs a fast structured map of how code works together before deciding which files to inspect in depth.",
      ];
    case "init":
      return ["Initialize the wiki runtime for a repository that has not been bootstrapped yet."];
    case "update":
      return ["Refresh the current knowledge runtime after source changes."];
    case "sync":
      return ["Sync managed `.wiki` page edits back into runtime state, metadata, and cache."];
    case "rebuild":
      return ["Force a full rebuild of the wiki runtime when the user explicitly asks for it."];
  }
}

function _renderHostActionInputs(action: WikiAction): string[] {
  if (action === "query") {
    return ["Accept explicit query terms from the user, or derive a concise query term from the current task or question."];
  }

  return ["Accept the minimal arguments required by the current action. If none are needed, run it directly."];
}

function renderHostActionNotes(action: WikiAction, entryName: string): string[] {
  switch (action) {
    case "init":
      return [
        `\`v0.2.0\` only guarantees a formal knowledge runtime init. Do not confuse \`${entryName}\` with top-level \`spec-wiki init\`.`,
      ];
    case "status":
      return ["`status` inspects runtime state only. It does not imply full knowledge/page completion."];
    case "update":
      return ["`v0.2.0` treats knowledge runtime refresh as the formal update contract."];
    case "query":
      return ["`v0.2.0` keeps the external term-only query contract, but routes results through index -> knowledge -> page fallback."];
    case "sync":
      return ["`sync` only applies `.wiki` page edits back into runtime state. It does not replace `update`."];
    case "rebuild":
      return ["`rebuild` is the explicit full runtime rebuild entry point. It does not replace `update`."];
  }
}

function renderHostActionSteps(action: WikiAction): string[] {
  const semantics = getWorkflowActionSemantics(action);
  const items = [...semantics.steps];

  if (action === "status") {
    items.push(
      "Prefer `state`, `query_readiness`, and `recommended_action` from runtime output when explaining whether the wiki is queryable, stale, and what should happen next.",
    );
  }

  return items;
}

function renderHostActionOutput(action: WikiAction): string[] {
  const semantics = getWorkflowActionSemantics(action);

  if (action === "query") {
    return [
      `Start with the structured query conclusion from: ${QUERY_STABLE_FIELDS.join(", ")}.`,
      "Then state whether deeper work such as `rg`, targeted file reads, or implementation-level verification is still needed.",
      "If the result is insufficient, say so explicitly and suggest a better next action.",
    ];
  }

  return semantics.output;
}

function renderHostActionGuardrails(action: WikiAction): string[] {
  const semantics = getWorkflowActionSemantics(action);

  if (action === "query") {
    return [
      ...semantics.guardrails,
      "Do not rebuild a new Wiki state machine or page semantic layer from query results.",
      "Do not invent missing pages, modules, symbols, or knowledge projections.",
      "Do not treat wiki-query as a replacement for `rg`, targeted file reads, or full implementation review. Read code when precise implementation details matter.",
    ];
  }

  if (action === "status") {
    return [
      ...semantics.guardrails,
      "`status` is only an inspection entry point. Do not misstate it as proof that the full knowledge/page runtime is complete.",
    ];
  }

  return semantics.guardrails;
}

function renderHostActionBody(action: WikiAction, entryName: string): string {
  if (action === "query") {
    return [
      `# ${entryName}`,
      "",
      ...renderQuerySkillIntro(),
      renderActionSection("When To Use", renderQueryWhenToUse()),
      renderActionSection("How To Work", renderQueryHowToWork()),
      renderActionSection("After This", renderQueryAfterThis()),
      renderActionSection("Action Notes", renderHostActionNotes(action, entryName)),
      renderActionSection("Guardrails", renderHostActionGuardrails(action)),
    ].join("\n");
  }

  return [
    `# ${entryName}`,
    "",
    renderActionSection("When To Use", renderHostActionPurpose(action)),
    renderActionSection("Run", renderHostActionSteps(action)),
    renderActionSection(
      "Interpret",
      [...renderHostActionOutput(action), ...renderHostActionNotes(action, entryName)],
    ),
    renderActionSection("Guardrails", renderHostActionGuardrails(action)),
  ].join("\n");
}

export function renderHostActionSkill(action: WikiAction): string {
  const semantics = getWorkflowActionSemantics(action);

  return [
    "---",
    `name: ${semantics.codeBuddySkillName}`,
    `description: ${renderFrontmatterString(renderActionDescription(action))}`,
    "user-invocable: true",
    "---",
    "",
    renderHostActionBody(action, semantics.codeBuddySkillName),
  ].join("\n");
}

export function renderCodeBuddyActionSkill(action: WikiAction): string {
  const semantics = getWorkflowActionSemantics(action);
  const description = renderActionDescription(action);
  const purpose
    = action === "query"
      ? "Use structured query results when the user asks about modules, concepts, keywords, files, symbols, or call paths, or when the agent needs a fast map of how code works together."
      : semantics.description;
  const output
    = action === "query"
      ? [
          `Start with the structured query conclusion from: ${QUERY_STABLE_FIELDS.join(", ")}.`,
          "Then state whether deeper work such as `rg`, targeted file reads, or implementation-level verification is still needed.",
          "If the result is insufficient, say so explicitly and suggest a better next action.",
        ]
      : semantics.output;
  const guardrails
    = action === "query"
      ? [
          ...semantics.guardrails,
          "Do not post-process query output into host-specific Wiki business conclusions.",
          "Do not invent missing pages, modules, symbols, or knowledge projections.",
          "Do not treat wiki-query as a replacement for `rg`, targeted file reads, or full implementation review. Read code when precise implementation details matter.",
        ]
      : semantics.guardrails;

  return [
    "---",
    `name: ${semantics.codeBuddySkillName}`,
    `description: ${renderFrontmatterString(description)}`,
    "user-invocable: true",
    "---",
    "",
    `# ${semantics.codeBuddySkillName}`,
    "",
    ...(action === "query"
      ? [
          ...renderQuerySkillIntro(),
          renderActionSection("When To Use", [purpose]),
          renderActionSection("How To Work", renderQueryHowToWork()),
          renderActionSection("After This", renderQueryAfterThis()),
          renderActionSection("Action Notes", [semantics.actionNote]),
          renderActionSection("Guardrails", guardrails),
        ]
      : [
          renderActionSection("When To Use", [purpose]),
          renderActionSection("Run", semantics.steps),
          renderActionSection("Interpret", [...output, semantics.actionNote]),
          renderActionSection("Guardrails", guardrails),
        ]),
  ].join("\n");
}

export function renderCodeBuddyHookAdditionalContext(
  event: "SessionStart" | "UserPromptSubmit",
): string {
  const baseLines = [
    "This repository uses spec-wiki as the shared Repo Wiki runtime.",
    "Prefer the explicit skills wiki-status and wiki-query when the user asks about repo structure, files, modules, symbols, call paths, or when you need a fast structured map of how the code works together.",
    "Do not confuse top-level spec-wiki init with runtime action spec-wiki wiki init.",
    "Treat wiki-status and wiki-query output as authoritative runtime data instead of rebuilding a host-side wiki state machine.",
    "The shared rules live in hooks and action skill guardrails, not in a separate shared CodeBuddy skill.",
  ];

  if (event === "SessionStart") {
    return baseLines.join(" ");
  }

  return [
    ...baseLines,
    "When in doubt, inspect runtime state first, then answer from the stable structured runtime fields that spec-wiki returns.",
  ].join(" ");
}

export function renderCodeBuddyHookScript(
  event: "SessionStart" | "UserPromptSubmit",
): string {
  const staticContext = JSON.stringify(renderCodeBuddyHookAdditionalContext(event));
  const eventName = JSON.stringify(event);

  return [
    "import { stdin, stdout } from \"node:process\";",
    "",
    "const chunks = [];",
    "stdin.setEncoding(\"utf8\");",
    "stdin.on(\"data\", (chunk) => chunks.push(chunk));",
    "stdin.on(\"end\", () => {",
    "  const rawInput = chunks.join(\"\").trim();",
    `  const eventName = ${eventName};`,
    `  const additionalContext = ${staticContext};`,
    "  let shouldInject = eventName === \"SessionStart\";",
    "",
    "  if (!shouldInject) {",
    "    const lowered = rawInput.toLowerCase();",
    "    shouldInject = [\"结构\", \"模块\", \"文件\", \"路径\", \"符号\", \"调用\", \"概念\", \"query\", \"status\", \"wiki\", \"module\", \"symbol\", \"path\", \"call\"].some((keyword) => lowered.includes(keyword));",
    "  }",
    "",
    "  const payload = shouldInject",
    "    ? { continue: true, additionalContext }",
    "    : { continue: true };",
    "",
    "  stdout.write(JSON.stringify(payload));",
    "});",
    "stdin.resume();",
    "",
  ].join("\n");
}

export type CodeBuddySettingsPatch = {
  hooks: Record<"SessionStart" | "UserPromptSubmit", string[]>;
};

export function buildCodeBuddySettingsPatch(): CodeBuddySettingsPatch {
  return {
    hooks: {
      SessionStart: ["node \"$CODEBUDDY_PROJECT_DIR/.codebuddy/hooks/spec-wiki/session-start.mjs\""],
      UserPromptSubmit: ["node \"$CODEBUDDY_PROJECT_DIR/.codebuddy/hooks/spec-wiki/user-prompt-submit.mjs\""],
    },
  };
}

export function listManagedCodeBuddyHookCommands(
  patch: CodeBuddySettingsPatch,
): string[] {
  return Object.values(patch.hooks).flat();
}

export function getActionSemanticsForRendering(action: WikiAction): WorkflowActionSemantics {
  return getWorkflowActionSemantics(action);
}




