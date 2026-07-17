/**
 * 可嵌入宿主资产的 trigger runtime factory。
 * Factory 不捕获模块变量，因此生成的 hook 可以序列化并执行同一份 reducer 源码。
 */

type EmbeddedWikiAction = "init" | "update" | "sync" | "status" | "query" | "rebuild";

export type EmbeddedTriggerRuntimeConfig = {
  contractVersion: string;
  actions: readonly EmbeddedWikiAction[];
};

type EmbeddedTriggerInput = {
  kind: "user_prompt";
  text: unknown;
};

type EmbeddedTriggerDecision = {
  contractVersion: string;
  decision: "should_trigger" | "should_not_trigger" | "ambiguous";
  targetAction: EmbeddedWikiAction | null;
  reason: string;
  evidence: readonly { kind: string; ruleId: string }[];
};

export type EmbeddedTriggerRuntime = {
  normalize: (text: string) => string;
  evaluate: (input: EmbeddedTriggerInput) => EmbeddedTriggerDecision;
};

/** 创建库内与生成 hook 共用的确定性 reducer。 */
export function createHostTriggerRuntime(
  config: EmbeddedTriggerRuntimeConfig,
): EmbeddedTriggerRuntime {
  const commandPrefixes = ["spec-wiki ", "/wiki:", "wiki-"];
  const englishQueryActionPhrases = [
    "query the wiki",
    "query repo wiki",
    "search the wiki",
    "search repo wiki",
    "look up the wiki",
  ];
  const chineseQueryActionPhrases = ["查询", "查一下", "搜索"];
  const englishQueryIntents = ["show me", "find", "locate", "map", "trace", "where", "which"];
  const englishQuerySubjects = [
    "code",
    "file",
    "files",
    "module",
    "modules",
    "symbol",
    "symbols",
    "implementation",
    "call path",
    "call paths",
    "entry point",
    "entry points",
  ];
  const chineseQueryIntents = ["帮我", "请", "想要", "需要", "如何", "怎么", "哪里", "哪些"];
  const chineseQueryVerbs = ["定位", "找到", "梳理", "分析", "追踪", "查看", "实现"];
  const chineseQuerySubjects = ["模块", "文件", "代码", "符号", "调用", "入口", "实现", "依赖", "路径"];
  const englishStatusTerms = ["ready", "stale", "blocked", "status", "up-to-date", "up to date", "needs update"];
  const chineseStatusTerms = ["状态", "就绪", "过期", "阻塞", "可用", "需要更新", "需要 update"];
  const englishOptOutTerms = ["do not", "don't", "dont", "no need to", "without", "skip", "avoid"];
  const chineseOptOutTerms = ["不要", "不用", "无需", "别"];
  const triggerSubjectTerms = ["wiki", "spec-wiki", "query", "update", "sync", "rebuild", "init", "知识库", "查询", "更新", "同步", "重建", "初始化"];
  const mutationTerms = ["init", "update", "sync", "rebuild", "初始化", "更新", "同步", "重建"];
  const bareWikiValues = new Set(["wiki", "知识库", "repo wiki", "spec-wiki"]);
  const chineseActions: Record<string, EmbeddedWikiAction> = {
    初始化: "init",
    更新: "update",
    同步: "sync",
    重建: "rebuild",
  };
  const evidenceKindOrder = [
    "explicit_action",
    "semantic_intent",
    "negation",
    "mutation_guard",
    "conflict",
    "input_validation",
    "no_match",
  ];

  function normalize(text: string): string {
    return text.normalize("NFKC").trim().toLowerCase().replace(/\s+/gu, " ");
  }

  function includesAny(text: string, terms: readonly string[]): boolean {
    return terms.some((term) => {
      if (![...term].every(character => (character.codePointAt(0) ?? 0) <= 0x7F)) {
        return text.includes(term);
      }
      let offset = text.indexOf(term);
      while (offset >= 0) {
        const before = offset === 0 ? "" : text[offset - 1];
        const after = text[offset + term.length] ?? "";
        if (!/[a-z0-9]/u.test(before) && !/[a-z0-9]/u.test(after)) {
          return true;
        }
        offset = text.indexOf(term, offset + term.length);
      }
      return false;
    });
  }

  function evidence(kind: string, ruleId: string) {
    return { kind, ruleId };
  }

  function decision(
    kind: EmbeddedTriggerDecision["decision"],
    targetAction: EmbeddedWikiAction | null,
    reason: string,
    items: { kind: string; ruleId: string }[],
  ): EmbeddedTriggerDecision {
    const stableEvidence = items
      .filter((item, index, values) =>
        values.findIndex(candidate => candidate.kind === item.kind && candidate.ruleId === item.ruleId) === index,
      )
      .sort((left, right) => {
        const kindOrder = evidenceKindOrder.indexOf(left.kind) - evidenceKindOrder.indexOf(right.kind);
        return kindOrder === 0 ? left.ruleId.localeCompare(right.ruleId) : kindOrder;
      })
      .map(item => Object.freeze({ ...item }));
    return Object.freeze({
      contractVersion: config.contractVersion,
      decision: kind,
      targetAction,
      reason,
      evidence: Object.freeze(stableEvidence),
    });
  }

  function hasCommandAction(text: string, action: EmbeddedWikiAction): boolean {
    return commandPrefixes.some((prefix) => {
      const marker = `${prefix}${action}`;
      let offset = text.indexOf(marker);
      while (offset >= 0) {
        const before = offset === 0 ? "" : text[offset - 1];
        const after = text[offset + marker.length] ?? "";
        if (!/[a-z0-9]/u.test(before) && !/[a-z0-9]/u.test(after)) {
          return true;
        }
        offset = text.indexOf(marker, offset + marker.length);
      }
      return false;
    });
  }

  function leadingReadAction(text: string): EmbeddedWikiAction | null {
    return (["query", "status"] as const).find(action =>
      text === action || text.startsWith(`${action} `) || text.startsWith(`${action},`),
    ) ?? null;
  }

  function hasEnglishMutationRequest(text: string, action: EmbeddedWikiAction): boolean {
    const prefixes = [
      `${action} wiki`,
      `${action} the wiki`,
      `${action} repo wiki`,
      `please ${action} wiki`,
      `please ${action} the wiki`,
      `please ${action} repo wiki`,
      `run wiki ${action}`,
      `execute wiki ${action}`,
    ];
    return text === action || prefixes.some(prefix => text === prefix || text.startsWith(`${prefix} `));
  }

  function hasChineseMutationRequest(text: string, actionText: string): boolean {
    const prefixes = [
      actionText,
      `${actionText}一下`,
      `请${actionText}`,
      `请${actionText}一下`,
      `帮我${actionText}`,
      `帮我${actionText}一下`,
      `请执行${actionText}`,
      `帮我执行${actionText}`,
      `立即${actionText}`,
    ];
    return prefixes.some(prefix =>
      text === prefix || text.startsWith(`${prefix} `) || text.startsWith(`${prefix}wiki`),
    );
  }

  function collectExplicitActions(text: string): EmbeddedWikiAction[] {
    const actions: EmbeddedWikiAction[] = [];
    const leading = leadingReadAction(text);
    if (leading) {
      actions.push(leading);
    }
    for (const action of config.actions) {
      if (hasCommandAction(text, action)) {
        actions.push(action);
      }
    }
    if (includesAny(text, englishQueryActionPhrases)) {
      actions.push("query");
    }
    if (chineseQueryActionPhrases.some(phrase =>
      text.startsWith(phrase)
      || text.startsWith(`请${phrase}`)
      || text.startsWith(`帮我${phrase}`)
      || text.startsWith(`现在${phrase}`),
    )) {
      actions.push("query");
    }
    if (["check wiki status", "check the wiki status", "show wiki status", "show the wiki status", "get wiki status"].some(phrase => text.includes(phrase))) {
      actions.push("status");
    }
    for (const action of ["init", "update", "sync", "rebuild"] as const) {
      if (hasEnglishMutationRequest(text, action)) {
        actions.push(action);
      }
    }
    for (const [actionText, action] of Object.entries(chineseActions)) {
      if (hasChineseMutationRequest(text, actionText)) {
        actions.push(action);
      }
    }
    return [...new Set(actions)];
  }

  function textAfterMarker(text: string, marker: string): string | null {
    const offset = text.indexOf(marker);
    if (offset < 0) {
      return null;
    }
    return text.slice(offset + marker.length).trim().replace(/^for\s+/u, "");
  }

  function hasMeaningfulQueryTerm(term: string): boolean {
    return /[\p{L}\p{N}_$]/u.test(term);
  }

  function hasExplicitQueryTerm(text: string): boolean {
    for (const prefix of commandPrefixes) {
      const commandTerm = textAfterMarker(text, `${prefix}query`);
      if (commandTerm !== null) {
        return hasMeaningfulQueryTerm(commandTerm);
      }
    }
    if (text === "query" || text.startsWith("query ")) {
      return hasMeaningfulQueryTerm(text.slice("query".length).trim());
    }
    for (const phrase of englishQueryActionPhrases) {
      const term = textAfterMarker(text, phrase);
      if (term !== null) {
        return hasMeaningfulQueryTerm(term);
      }
    }
    for (const phrase of chineseQueryActionPhrases) {
      const marker = [phrase, `请${phrase}`, `帮我${phrase}`, `现在${phrase}`]
        .find(candidate => text.startsWith(candidate));
      if (marker) {
        const term = text.slice(marker.length).trim().replace(/^(?:一下|wiki|知识库|中的|里|中)+/u, "").trim();
        return hasMeaningfulQueryTerm(term);
      }
    }
    return true;
  }

  function evaluate(input: EmbeddedTriggerInput): EmbeddedTriggerDecision {
    if (input.kind !== "user_prompt" || typeof input.text !== "string") {
      return decision("ambiguous", null, "invalid_input", [
        evidence("input_validation", "input.user_prompt.string.v1"),
      ]);
    }
    const text = normalize(input.text);
    if (text.length === 0) {
      return decision("ambiguous", null, "invalid_input", [
        evidence("input_validation", "input.user_prompt.non_empty.v1"),
      ]);
    }

    const explicitActions = collectExplicitActions(text);
    const semanticQuery = (includesAny(text, englishQueryIntents) && includesAny(text, englishQuerySubjects))
      || (
        includesAny(text, chineseQueryIntents)
        && includesAny(text, chineseQueryVerbs)
        && includesAny(text, chineseQuerySubjects)
      );
    const semanticStatus = includesAny(text, ["wiki", "知识库"])
      && (includesAny(text, englishStatusTerms) || includesAny(text, chineseStatusTerms));
    const optedOut = (
      includesAny(text, englishOptOutTerms)
      || includesAny(text, chineseOptOutTerms)
    ) && includesAny(text, triggerSubjectTerms);
    const hasPositiveIntent = explicitActions.length > 0 || semanticQuery || semanticStatus;

    if (optedOut && hasPositiveIntent) {
      return decision("ambiguous", null, "conflicting_intent", [
        evidence("negation", "intent.opt_out.v1"),
        evidence("conflict", "intent.positive_and_opt_out.v1"),
      ]);
    }
    if (optedOut) {
      return decision("should_not_trigger", null, "explicit_opt_out", [
        evidence("negation", "intent.opt_out.v1"),
      ]);
    }
    if (explicitActions.length > 1) {
      return decision("ambiguous", null, "conflicting_intent", [
        evidence("conflict", "action.multiple_explicit.v1"),
      ]);
    }
    if (explicitActions.length === 1) {
      const action = explicitActions[0];
      if (action === "query" && !hasExplicitQueryTerm(text)) {
        return decision("ambiguous", "query", "missing_query_term", [
          evidence("explicit_action", "action.query.explicit.v1"),
          evidence("input_validation", "action.query.term_required.v1"),
        ]);
      }
      return decision("should_trigger", action, "explicit_action_request", [
        evidence("explicit_action", `action.${action}.explicit.v1`),
      ]);
    }
    if (semanticQuery && semanticStatus) {
      return decision("ambiguous", null, "conflicting_intent", [
        evidence("conflict", "intent.query_and_status.v1"),
      ]);
    }
    if (semanticQuery) {
      return decision("should_trigger", "query", "semantic_query_request", [
        evidence("semantic_intent", "query.repo_map.semantic.v1"),
      ]);
    }
    if (semanticStatus) {
      return decision("should_trigger", "status", "semantic_status_request", [
        evidence("semantic_intent", "status.readiness.semantic.v1"),
      ]);
    }
    if (includesAny(text, mutationTerms)) {
      return decision("should_not_trigger", null, "mutation_requires_explicit_request", [
        evidence("mutation_guard", "action.mutation.explicit_only.v1"),
      ]);
    }
    if (bareWikiValues.has(text.replace(/[!?？。]+$/u, ""))) {
      return decision("ambiguous", null, "insufficient_context", [
        evidence("input_validation", "intent.bare_wiki.v1"),
      ]);
    }
    return decision("should_not_trigger", null, "no_supported_intent", [
      evidence("no_match", "intent.no_supported_match.v1"),
    ]);
  }

  return Object.freeze({ normalize, evaluate });
}
