/**
 * 这个文件收口 `wiki-runtime` 的最小响应协议校验。
 * `spec-wiki` 只消费稳定的宿主字段，不在这里重写 Wiki 业务语义。
 */

export type QueryReadiness = "ready" | "needs_init" | "needs_update" | "blocked";
export type RecommendedAction = "none" | "init" | "update" | "rebuild" | "sync";
export type LlmModeHint = "provider_configured" | "deterministic_default";
export type QueryMode = "index_first" | "knowledge_first" | "page_fallback" | "mixed";
export type QueryTrust = "ready" | "stale_but_queryable" | "blocked";
export type LlmExecutionMode = "provider_direct" | "agent_bridge" | "deterministic_only";

export type CoreUsageBucket = {
  key: string;
  request_count: number;
  input_tokens: number;
  output_tokens: number;
  total_tokens: number;
};

export type CoreUsageSnapshot = {
  request_count: number;
  input_tokens: number;
  output_tokens: number;
  total_tokens: number;
  by_prompt_type: CoreUsageBucket[];
  by_provider_model: CoreUsageBucket[];
};

export type RuntimeSummaryProjection = {
  workflow_action: string;
  runtime_state: string;
  researched_units: number;
  compose_ready_units: number;
  composed_units: number;
  assembled_pages: number;
  blocked_units: string[];
  last_ready_stage?: string | null;
  summary_reason?: string | null;
};

export type RuntimeGateBlocker = {
  unit_id: string;
  unit_type: string;
  blocked_reason?: string | null;
  missing_dependencies: string[];
};

export type RuntimeGateSummary = {
  total_units: number;
  ready_for_compose_units: number;
  composed_units: number;
  assembled_units: number;
  blocked_units: number;
  blockers: RuntimeGateBlocker[];
};

export type WorkflowTerminalData = {
  runtime_summary?: RuntimeSummaryProjection | null;
  llm_execution_mode?: LlmExecutionMode | null;
  blocker_hint?: string | null;
  [key: string]: unknown;
};

export type WikiStatusData = {
  state: string;
  dirty_sources: string[];
  dirty_pages: string[];
  needs_rebuild_reason?: string | null;
  facts_ready: boolean;
  query_readiness: QueryReadiness;
  recommended_action: RecommendedAction;
  llm_mode_hint: LlmModeHint;
  runtime_summary?: RuntimeSummaryProjection | null;
  gate_summary?: RuntimeGateSummary | null;
};

export type StatusPreflightData = WikiStatusData;

/** `v0.2.0` 正式保证 query route tags 与 mode 字段稳定；其余返回允许继续扩展。 */
export type WikiQueryData = {
  term: string;
  runtime_state: string;
  query_mode: QueryMode;
  query_trust: QueryTrust;
  recommended_action: RecommendedAction;
  matched_pages: string[];
  provenance_summary: string;
  [key: string]: unknown;
};

export type QueryProfileData = WikiQueryData;

export type WikiInitData = WorkflowTerminalData & {
  initialized: boolean;
  state: string;
  generated_pages: string[];
};

export type WikiUpdateData = WorkflowTerminalData & {
  previous_state: string;
  state: string;
  updated_pages: string[];
};

export type WikiRebuildData = WorkflowTerminalData & {
  state: string;
  updated_pages: string[];
  warnings?: string[];
};

export type WikiSyncData = {
  state: string;
  [key: string]: unknown;
};

export type CoreKnownData
  = | WikiStatusData
    | WikiQueryData
    | WikiInitData
    | WikiUpdateData
    | WikiRebuildData
    | WikiSyncData
    | WorkflowTerminalData
    | Record<string, unknown>
    | null;

/** `wiki-runtime` 的统一终态响应外壳。 */
export type CoreResponse<TData = CoreKnownData> = {
  /** 是否执行成功。 */
  ok: boolean;
  /** 失败时的错误消息。 */
  error?: string | null;
  /** 成功或失败时附带的数据载荷。 */
  data?: TData;
};

/** 长流程事件流里的阶段进度事件。 */
export type CoreProgressEvent = {
  type: "progress";
  action: string;
  phase: string;
  message: string;
  elapsed_ms: number;
  processed: number | null;
  total: number | null;
  usage?: CoreUsageSnapshot | null;
};

/** 长流程事件流里的 LLM 请求载荷。 */
export type CoreLlmRequest = {
  request_id: string;
  prompt_type: string;
  prompt_version: string;
  input_hash: string;
  model?: string | null;
  system: string;
  instruction: string;
  input: unknown;
  response_schema: unknown;
};

/** 长流程事件流里的 `llm_request` 事件。 */
export type CoreLlmRequestEvent = {
  type: "llm_request";
  request: CoreLlmRequest;
};

/** 长流程事件流里的成功终态事件。 */
export type CoreResultEvent = {
  type: "result";
  response: CoreResponse;
};

/** 长流程事件流里的失败终态事件。 */
export type CoreErrorEvent = {
  type: "error";
  response: CoreResponse;
};

/** `wiki-runtime` 长流程允许输出的完整事件集合。 */
export type CoreStreamEvent
  = | CoreProgressEvent
    | CoreLlmRequestEvent
    | CoreResultEvent
    | CoreErrorEvent;

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function parseStringArray(value: unknown, field: string): string[] {
  if (!Array.isArray(value) || value.some((item) => typeof item !== "string")) {
    throw new Error(`invalid wiki-runtime ${field}`);
  }
  return value as string[];
}

function parseLiteral<T extends string>(
  value: unknown,
  allowed: readonly T[],
  field: string,
): T {
  if (typeof value !== "string" || !allowed.includes(value as T)) {
    throw new Error(`invalid wiki-runtime ${field}`);
  }
  return value as T;
}

function parseNullableString(value: unknown, field: string): string | null | undefined {
  if (value === undefined) {
    return undefined;
  }
  if (value === null || typeof value === "string") {
    return value;
  }
  throw new Error(`invalid wiki-runtime ${field}`);
}

function parseUsageBucket(value: unknown): CoreUsageBucket {
  const parsed = value as Partial<CoreUsageBucket>;

  if (
    !isRecord(parsed)
    || typeof parsed.key !== "string"
    || typeof parsed.request_count !== "number"
    || typeof parsed.input_tokens !== "number"
    || typeof parsed.output_tokens !== "number"
    || typeof parsed.total_tokens !== "number"
  ) {
    throw new Error("invalid wiki-runtime usage bucket");
  }

  return parsed as CoreUsageBucket;
}

function parseUsageSnapshot(value: unknown): CoreUsageSnapshot {
  const parsed = value as Partial<CoreUsageSnapshot>;

  if (
    !isRecord(parsed)
    || typeof parsed.request_count !== "number"
    || typeof parsed.input_tokens !== "number"
    || typeof parsed.output_tokens !== "number"
    || typeof parsed.total_tokens !== "number"
    || !Array.isArray(parsed.by_prompt_type)
    || !Array.isArray(parsed.by_provider_model)
  ) {
    throw new Error("invalid wiki-runtime usage snapshot");
  }

  return {
    request_count: parsed.request_count,
    input_tokens: parsed.input_tokens,
    output_tokens: parsed.output_tokens,
    total_tokens: parsed.total_tokens,
    by_prompt_type: parsed.by_prompt_type.map(parseUsageBucket),
    by_provider_model: parsed.by_provider_model.map(parseUsageBucket),
  };
}

function parseRuntimeSummary(value: unknown): RuntimeSummaryProjection {
  const parsed = value as Partial<RuntimeSummaryProjection>;

  if (
    !isRecord(parsed)
    || typeof parsed.workflow_action !== "string"
    || typeof parsed.runtime_state !== "string"
    || typeof parsed.researched_units !== "number"
    || typeof parsed.compose_ready_units !== "number"
    || typeof parsed.composed_units !== "number"
    || typeof parsed.assembled_pages !== "number"
  ) {
    throw new Error("invalid wiki-runtime runtime_summary");
  }

  return {
    workflow_action: parsed.workflow_action,
    runtime_state: parsed.runtime_state,
    researched_units: parsed.researched_units,
    compose_ready_units: parsed.compose_ready_units,
    composed_units: parsed.composed_units,
    assembled_pages: parsed.assembled_pages,
    blocked_units: parseStringArray(parsed.blocked_units ?? [], "runtime_summary.blocked_units"),
    last_ready_stage: parseNullableString(
      parsed.last_ready_stage,
      "runtime_summary.last_ready_stage",
    ),
    summary_reason: parseNullableString(parsed.summary_reason, "runtime_summary.summary_reason"),
  };
}

function parseGateBlocker(value: unknown): RuntimeGateBlocker {
  const parsed = value as Partial<RuntimeGateBlocker>;

  if (
    !isRecord(parsed)
    || typeof parsed.unit_id !== "string"
    || typeof parsed.unit_type !== "string"
  ) {
    throw new Error("invalid wiki-runtime gate blocker");
  }

  return {
    unit_id: parsed.unit_id,
    unit_type: parsed.unit_type,
    blocked_reason: parseNullableString(parsed.blocked_reason, "gate_summary.blocked_reason"),
    missing_dependencies: parseStringArray(
      parsed.missing_dependencies ?? [],
      "gate_summary.missing_dependencies",
    ),
  };
}

function parseGateSummary(value: unknown): RuntimeGateSummary {
  const parsed = value as Partial<RuntimeGateSummary>;
  const blockers = parsed.blockers ?? [];

  if (
    !isRecord(parsed)
    || typeof parsed.total_units !== "number"
    || typeof parsed.ready_for_compose_units !== "number"
    || typeof parsed.composed_units !== "number"
    || typeof parsed.assembled_units !== "number"
    || typeof parsed.blocked_units !== "number"
    || !Array.isArray(blockers)
  ) {
    throw new Error("invalid wiki-runtime gate_summary");
  }

  return {
    total_units: parsed.total_units,
    ready_for_compose_units: parsed.ready_for_compose_units,
    composed_units: parsed.composed_units,
    assembled_units: parsed.assembled_units,
    blocked_units: parsed.blocked_units,
    blockers: blockers.map(parseGateBlocker),
  };
}

function parseWorkflowTerminalData(value: Record<string, unknown>): WorkflowTerminalData {
  return {
    ...value,
    runtime_summary:
      value.runtime_summary == null ? value.runtime_summary : parseRuntimeSummary(value.runtime_summary),
    llm_execution_mode:
      value.llm_execution_mode == null
        ? (value.llm_execution_mode ?? undefined)
        : parseLiteral(
            value.llm_execution_mode,
            ["provider_direct", "agent_bridge", "deterministic_only"] as const,
            "llm_execution_mode",
          ),
    blocker_hint: parseNullableString(value.blocker_hint, "blocker_hint"),
  };
}

function parseStatusData(value: Record<string, unknown>): WikiStatusData {
  if (
    typeof value.state !== "string"
    || typeof value.facts_ready !== "boolean"
    || typeof value.llm_mode_hint !== "string"
  ) {
    throw new TypeError("invalid wiki-runtime status payload");
  }

  return {
    state: value.state,
    dirty_sources: parseStringArray(value.dirty_sources ?? [], "dirty_sources"),
    dirty_pages: parseStringArray(value.dirty_pages ?? [], "dirty_pages"),
    needs_rebuild_reason: parseNullableString(
      value.needs_rebuild_reason,
      "needs_rebuild_reason",
    ),
    facts_ready: value.facts_ready,
    query_readiness: parseLiteral(
      value.query_readiness,
      ["ready", "needs_init", "needs_update", "blocked"] as const,
      "query_readiness",
    ),
    recommended_action: parseLiteral(
      value.recommended_action,
      ["none", "init", "update", "rebuild", "sync"] as const,
      "recommended_action",
    ),
    llm_mode_hint: parseLiteral(
      value.llm_mode_hint,
      ["provider_configured", "deterministic_default"] as const,
      "llm_mode_hint",
    ),
    runtime_summary:
      value.runtime_summary == null ? value.runtime_summary : parseRuntimeSummary(value.runtime_summary),
    gate_summary:
      value.gate_summary == null ? value.gate_summary : parseGateSummary(value.gate_summary),
  };
}

function parseQueryData(value: Record<string, unknown>): WikiQueryData {
  if (
    typeof value.term !== "string"
    || typeof value.runtime_state !== "string"
    || typeof value.provenance_summary !== "string"
  ) {
    throw new TypeError("invalid wiki-runtime query payload");
  }

  return {
    ...value,
    term: value.term,
    runtime_state: value.runtime_state,
    query_mode: parseLiteral(
      value.query_mode,
      ["index_first", "knowledge_first", "page_fallback", "mixed"] as const,
      "query_mode",
    ),
    query_trust: parseLiteral(
      value.query_trust,
      ["ready", "stale_but_queryable", "blocked"] as const,
      "query_trust",
    ),
    recommended_action: parseLiteral(
      value.recommended_action,
      ["none", "init", "update", "rebuild", "sync"] as const,
      "recommended_action",
    ),
    matched_pages: parseStringArray(value.matched_pages ?? [], "matched_pages"),
    provenance_summary: value.provenance_summary,
  };
}

function parseKnownData(value: unknown): CoreKnownData {
  if (!isRecord(value)) {
    return value as CoreKnownData;
  }

  if (
    typeof value.state === "string"
    && typeof value.facts_ready === "boolean"
    && "query_readiness" in value
  ) {
    return parseStatusData(value);
  }

  if (
    typeof value.term === "string"
    && typeof value.runtime_state === "string"
    && "query_mode" in value
    && "query_trust" in value
  ) {
    return parseQueryData(value);
  }

  if (
    "runtime_summary" in value
    || "llm_execution_mode" in value
    || "blocker_hint" in value
  ) {
    return parseWorkflowTerminalData(value);
  }

  return value;
}

function parseCoreResponse(value: unknown): CoreResponse {
  const parsed = value as Partial<CoreResponse>;

  if (!isRecord(parsed) || typeof parsed.ok !== "boolean") {
    throw new Error("invalid wiki-runtime response");
  }

  return {
    ok: parsed.ok,
    error:
      typeof parsed.error === "string" || parsed.error == null ? parsed.error : undefined,
    data: parseKnownData(parsed.data),
  };
}

/**
 * 校验 `wiki-runtime` 的统一响应协议。
 *
 * @param stdout `wiki-runtime` 写到标准输出的 JSON 文本。
 * @returns 返回通过协议校验的响应对象。
 */
export function parseResult(stdout: string): CoreResponse {
  return parseCoreResponse(JSON.parse(stdout));
}

/**
 * 校验 `wiki-runtime` 长流程使用的 NDJSON 事件行。
 *
 * @param line 单行事件 JSON。
 * @returns 返回结构化的进度或终态事件。
 */
export function parseEventLine(line: string): CoreStreamEvent {
  const parsed = JSON.parse(line) as Partial<CoreStreamEvent> & {
    response?: unknown;
    request?: unknown;
    usage?: unknown;
  };

  if (parsed.type === "progress") {
    if (
      typeof parsed.action !== "string"
      || typeof parsed.phase !== "string"
      || typeof parsed.message !== "string"
      || typeof parsed.elapsed_ms !== "number"
      || !("processed" in parsed)
      || !("total" in parsed)
    ) {
      throw new Error("invalid wiki-runtime progress event");
    }

    return {
      type: "progress",
      action: parsed.action,
      phase: parsed.phase,
      message: parsed.message,
      elapsed_ms: parsed.elapsed_ms,
      processed: parsed.processed ?? null,
      total: parsed.total ?? null,
      usage: parsed.usage == null ? null : parseUsageSnapshot(parsed.usage),
    };
  }

  if (parsed.type === "llm_request") {
    const request = parsed.request as Partial<CoreLlmRequest> | undefined;

    if (
      !isRecord(request)
      || typeof request.request_id !== "string"
      || typeof request.prompt_type !== "string"
      || typeof request.prompt_version !== "string"
      || typeof request.input_hash !== "string"
      || typeof request.system !== "string"
      || typeof request.instruction !== "string"
    ) {
      throw new Error("invalid wiki-runtime llm_request event");
    }

    return {
      type: "llm_request",
      request: {
        request_id: request.request_id,
        prompt_type: request.prompt_type,
        prompt_version: request.prompt_version,
        input_hash: request.input_hash,
        model:
          typeof request.model === "string" || request.model == null ? request.model : null,
        system: request.system,
        instruction: request.instruction,
        input: request.input,
        response_schema: request.response_schema,
      },
    };
  }

  if (parsed.type === "result" || parsed.type === "error") {
    return {
      type: parsed.type,
      response: parseCoreResponse(parsed.response),
    };
  }

  throw new Error("invalid wiki-runtime stream event");
}

/**
 * 从终态事件恢复最终 `CoreResponse`。
 *
 * @param event `result` 或 `error` 终态事件。
 * @returns 返回与非流式模式等价的最终响应对象。
 */
export function responseFromTerminalEvent(
  event: CoreResultEvent | CoreErrorEvent,
): CoreResponse {
  return parseCoreResponse(event.response);
}



