/**
 * 这个文件收口 `wiki-runtime` 的最小响应协议校验。
 * `spec-wiki` 只消费稳定的宿主字段，不在这里重写 Wiki 业务语义。
 */

export type LayerReadiness
  = | "ready"
    | "stale"
    | "missing"
    | "rebuilding"
    | "conflict"
    | "blocked"
    | "not_enabled";
export type FusionReadiness = "ready" | "degraded" | "blocked";
export type RestoredLevel = "none" | "level1" | "level2";
export type RecommendedAction
  = | "none"
    | "init"
    | "review"
    | "review_governance"
    | "update"
    | "rebuild"
    | "sync";
export type LlmModeHint = "provider_configured" | "deterministic_default";
export type QueryMode = "index_first" | "knowledge_first" | "page_fallback" | "mixed";
export type QueryTrust = "ready" | "stale_but_queryable" | "blocked";
export type LlmExecutionMode = "provider_direct" | "agent_bridge" | "deterministic_only";
export type QueryRouteTag
  = | "index_symbol_hit"
    | "index_path_hit"
    | "index_graph_hit"
    | "knowledge_declared_hit"
    | "knowledge_derived_hit"
    | "governance_evidence_ref"
    | "governance_summary_hit"
    | "projection_ref"
    | "rendered_page_debug_fallback";
export type QueryRefKind
  = | "source_path"
    | "source_symbol"
    | "index_graph_edge"
    | "knowledge_page"
    | "knowledge_record"
    | "projection_page"
    | "projection_section"
    | "governance_change"
    | "governance_artifact"
    | "rendered_page";
export type QueryConfidence = "high" | "medium" | "low";
export type QueryResultRecommendedAction
  = | "none"
    | "open_reference"
    | "open_source_ref"
    | "open_knowledge_ref"
    | "open_projection_ref"
    | "review_governance"
    | "rebuild_index"
    | "update_knowledge"
    | "rebuild"
    | "update"
    | "sync";
export type QueryProvenance = {
  layer: string;
  state?: string | null;
  reason?: string | null;
};
export type QuerySourceRef = {
  ref_kind: QueryRefKind;
  ref_id: string;
  label?: string | null;
  path?: string | null;
  start_line?: number | null;
  end_line?: number | null;
  provenance?: string[];
  diagnostics?: string[];
  [key: string]: unknown;
};
export type QueryResultDto = {
  route_tag: QueryRouteTag;
  ref_kind: QueryRefKind;
  ref_id: string;
  label: string;
  score: number;
  provenance: QueryProvenance;
  confidence: QueryConfidence;
  recommended_action: QueryResultRecommendedAction;
  source_refs: QuerySourceRef[];
  [key: string]: unknown;
};
export type QueryRouteGroup = {
  route_tag: QueryRouteTag;
  results: QueryResultDto[];
  score_basis?: string | null;
  [key: string]: unknown;
};

export type GovernanceReadiness
  = | "not_enabled"
    | "ready"
    | "stale"
    | "blocked"
    | "conflict";
export type GovernanceIssueSeverity = "blocking" | "warning";
export type GovernanceRecommendedAction = "none" | "update" | "review_governance";
export type GovernanceArtifactStatus = "missing" | "empty" | "present";
export type GovernanceArtifactRef = {
  change_id: string;
  kind: string;
  relative_path: string;
  status: GovernanceArtifactStatus;
  content_hash?: string | null;
};
export type GovernanceBlockingIssue = {
  rule_id: string;
  severity: GovernanceIssueSeverity;
  message: string;
  change_id?: string | null;
  artifact_ref?: GovernanceArtifactRef | null;
  recommended_action: GovernanceRecommendedAction;
};
export type GovernanceSummary = {
  readiness: GovernanceReadiness;
  fingerprint?: string | null;
  active_count: number;
  archived_count: number;
  issues: GovernanceBlockingIssue[];
  recommended_action: GovernanceRecommendedAction;
};

export type GovernanceChangeSummary = {
  id: string;
  location: "active" | "archived";
  stage: string;
  role?: string | null;
  parent?: string | null;
  order?: number | null;
  depends_on: string[];
  gate: Record<string, unknown>;
};
export type GovernanceValidationResult = {
  valid: boolean;
  readiness: GovernanceReadiness;
  rule_results: Array<Record<string, unknown>>;
  issues: GovernanceBlockingIssue[];
};
export type GovernanceChangesReport = { governance: GovernanceSummary; changes: GovernanceChangeSummary[] };
export type GovernanceChangeReport = { governance: GovernanceSummary; change: GovernanceChangeSummary };
export type GovernanceValidateReport = { governance: GovernanceSummary; change_id: string; validation: GovernanceValidationResult };
export type BootstrapReport = { outcome: "ready" | "partial" | "failed"; hosts: Array<Record<string, unknown>>; recoveryHint?: string | null };
export type UnifiedInitReport = { outcome: "ready" | "partial"; bootstrap: BootstrapReport; runtime: unknown; landing: WikiStatusData; recovery_hint?: string | null };

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

export type RuntimeReadiness = {
  index: LayerReadiness;
  knowledge: LayerReadiness;
  projection: LayerReadiness;
  fusion: FusionReadiness;
  restored_level: RestoredLevel;
  snapshot_id?: string | null;
  reasons: string[];
};

export type WorkflowTerminalData = {
  runtime_summary?: RuntimeSummaryProjection | null;
  llm_execution_mode?: LlmExecutionMode | null;
  blocker_hint?: string | null;
  governance?: GovernanceSummary | null;
  [key: string]: unknown;
};

export type WikiStatusData = {
  state: string;
  dirty_sources: string[];
  dirty_pages: string[];
  needs_rebuild_reason?: string | null;
  readiness: RuntimeReadiness;
  recommended_action: RecommendedAction;
  llm_mode_hint: LlmModeHint;
  runtime_summary?: RuntimeSummaryProjection | null;
  gate_summary?: RuntimeGateSummary | null;
  governance: GovernanceSummary;
};

export type StatusPreflightData = WikiStatusData;

/** `v0.2.0` 正式保证 query route tags 与 mode 字段稳定；其余返回允许继续扩展。 */
export type WikiQueryData = {
  term: string;
  runtime_state: string;
  readiness: RuntimeReadiness;
  query_mode: QueryMode;
  query_trust: QueryTrust;
  recommended_action: RecommendedAction;
  matched_pages: string[];
  provenance_summary: string;
  governance: GovernanceSummary;
  route_groups: QueryRouteGroup[];
  results: QueryResultDto[];
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
  governance: GovernanceSummary;
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
    | GovernanceChangesReport
    | GovernanceChangeReport
    | GovernanceValidateReport
    | UnifiedInitReport
    | Record<string, unknown>
    | null;

export type CoreErrorKind
  = | "invalid_argument"
    | "governance_not_enabled"
    | "change_not_found"
    | "workflow_failed"
    | "protocol_error"
    | "internal_error";

/** `wiki-runtime` 的统一终态响应外壳。 */
export type CoreResponse<TData = CoreKnownData> = {
  /** 是否执行成功。 */
  ok: boolean;
  /** 失败时的错误消息。 */
  error?: string | null;
  /** 机器可判断的错误分类。 */
  errorKind?: CoreErrorKind | null;
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
export type CoreAgentEvent = {
  type: "agent_session_start" | "agent_message" | "agent_tool_call" | "agent_tool_result" | "agent_final" | "agent_abort";
  [key: string]: unknown;
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
    | CoreAgentEvent
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

function parseNullableNumber(value: unknown, field: string): number | null | undefined {
  if (value === undefined) {
    return undefined;
  }
  if (value === null || typeof value === "number") {
    return value;
  }
  throw new Error(`invalid wiki-runtime ${field}`);
}

function parseQueryRouteTag(value: unknown): QueryRouteTag {
  return parseLiteral(
    value,
    [
      "index_symbol_hit",
      "index_path_hit",
      "index_graph_hit",
      "knowledge_declared_hit",
      "knowledge_derived_hit",
      "governance_evidence_ref",
      "governance_summary_hit",
      "projection_ref",
      "rendered_page_debug_fallback",
    ] as const,
    "query route tag",
  );
}

function parseQueryRefKind(value: unknown): QueryRefKind {
  return parseLiteral(
    value,
    [
      "source_path",
      "source_symbol",
      "index_graph_edge",
      "knowledge_page",
      "knowledge_record",
      "projection_page",
      "projection_section",
      "governance_change",
      "governance_artifact",
      "rendered_page",
    ] as const,
    "query ref kind",
  );
}

function parseQueryConfidence(value: unknown): QueryConfidence {
  return parseLiteral(value, ["high", "medium", "low"] as const, "query confidence");
}

function parseQueryResultRecommendedAction(value: unknown): QueryResultRecommendedAction {
  return parseLiteral(
    value,
    [
      "none",
      "open_reference",
      "open_source_ref",
      "open_knowledge_ref",
      "open_projection_ref",
      "review_governance",
      "rebuild_index",
      "update_knowledge",
      "rebuild",
      "update",
      "sync",
    ] as const,
    "query_result.recommended_action",
  );
}

function parseQueryProvenance(value: unknown): QueryProvenance {
  const parsed = value as Partial<QueryProvenance>;
  if (!isRecord(parsed) || typeof parsed.layer !== "string") {
    throw new Error("invalid wiki-runtime query provenance");
  }
  return {
    layer: parsed.layer,
    state: parseNullableString(parsed.state, "query_provenance.state"),
    reason: parseNullableString(parsed.reason, "query_provenance.reason"),
  };
}

function parseQuerySourceRef(value: unknown): QuerySourceRef {
  const parsed = value as Partial<QuerySourceRef>;
  if (!isRecord(parsed) || !("ref_kind" in parsed) || typeof parsed.ref_id !== "string") {
    throw new Error("invalid wiki-runtime query source ref");
  }
  const result: QuerySourceRef = {
    ...parsed,
    ref_kind: parseQueryRefKind(parsed.ref_kind),
    ref_id: parsed.ref_id,
  };
  if (parsed.label !== undefined) {
    result.label = parseNullableString(parsed.label, "query_source_ref.label");
  }
  if (parsed.path !== undefined) {
    result.path = parseNullableString(parsed.path, "query_source_ref.path");
  }
  if (parsed.start_line !== undefined) {
    result.start_line = parseNullableNumber(parsed.start_line, "query_source_ref.start_line");
  }
  if (parsed.end_line !== undefined) {
    result.end_line = parseNullableNumber(parsed.end_line, "query_source_ref.end_line");
  }
  if (parsed.provenance !== undefined) {
    result.provenance = parseStringArray(parsed.provenance, "query_source_ref.provenance");
  }
  if (parsed.diagnostics !== undefined) {
    result.diagnostics = parseStringArray(parsed.diagnostics, "query_source_ref.diagnostics");
  }
  return result;
}

function parseQueryResult(value: unknown): QueryResultDto {
  const parsed = value as Partial<QueryResultDto>;
  if (
    !isRecord(parsed)
    || !("route_tag" in parsed)
    || !("ref_kind" in parsed)
    || typeof parsed.ref_id !== "string"
    || typeof parsed.label !== "string"
    || typeof parsed.score !== "number"
    || !("provenance" in parsed)
    || !("confidence" in parsed)
    || !("recommended_action" in parsed)
    || !Array.isArray(parsed.source_refs)
  ) {
    throw new Error("invalid wiki-runtime query result");
  }

  return {
    ...parsed,
    route_tag: parseQueryRouteTag(parsed.route_tag),
    ref_kind: parseQueryRefKind(parsed.ref_kind),
    ref_id: parsed.ref_id,
    label: parsed.label,
    score: parsed.score,
    provenance: parseQueryProvenance(parsed.provenance),
    confidence: parseQueryConfidence(parsed.confidence),
    recommended_action: parseQueryResultRecommendedAction(parsed.recommended_action),
    source_refs: parsed.source_refs.map(parseQuerySourceRef),
  };
}

function parseQueryRouteGroup(value: unknown): QueryRouteGroup {
  const parsed = value as Partial<QueryRouteGroup>;
  if (!isRecord(parsed) || !("route_tag" in parsed) || !Array.isArray(parsed.results)) {
    throw new Error("invalid wiki-runtime query route group");
  }

  return {
    ...parsed,
    route_tag: parseQueryRouteTag(parsed.route_tag),
    results: parsed.results.map(parseQueryResult),
    score_basis: parseNullableString(parsed.score_basis, "query_route_group.score_basis"),
  };
}

function parseGovernanceArtifactRef(value: unknown): GovernanceArtifactRef {
  const parsed = value as Partial<GovernanceArtifactRef>;
  if (
    !isRecord(parsed)
    || typeof parsed.change_id !== "string"
    || typeof parsed.kind !== "string"
    || typeof parsed.relative_path !== "string"
  ) {
    throw new Error("invalid wiki-runtime governance artifact ref");
  }
  return {
    change_id: parsed.change_id,
    kind: parsed.kind,
    relative_path: parsed.relative_path,
    status: parseLiteral(
      parsed.status,
      ["missing", "empty", "present"] as const,
      "governance.artifact_ref.status",
    ),
    content_hash: parseNullableString(
      parsed.content_hash,
      "governance.artifact_ref.content_hash",
    ),
  };
}

function parseGovernanceIssue(value: unknown): GovernanceBlockingIssue {
  const parsed = value as Partial<GovernanceBlockingIssue>;
  if (
    !isRecord(parsed)
    || typeof parsed.rule_id !== "string"
    || typeof parsed.message !== "string"
  ) {
    throw new Error("invalid wiki-runtime governance issue");
  }
  return {
    rule_id: parsed.rule_id,
    severity: parseLiteral(
      parsed.severity,
      ["blocking", "warning"] as const,
      "governance.issue.severity",
    ),
    message: parsed.message,
    change_id: parseNullableString(parsed.change_id, "governance.issue.change_id"),
    artifact_ref:
      parsed.artifact_ref == null
        ? parsed.artifact_ref
        : parseGovernanceArtifactRef(parsed.artifact_ref),
    recommended_action: parseLiteral(
      parsed.recommended_action,
      ["none", "update", "review_governance"] as const,
      "governance.issue.recommended_action",
    ),
  };
}

function parseGovernanceSummary(value: unknown): GovernanceSummary {
  const parsed = value as Partial<GovernanceSummary>;
  if (
    !isRecord(parsed)
    || typeof parsed.active_count !== "number"
    || typeof parsed.archived_count !== "number"
    || (parsed.issues !== undefined && !Array.isArray(parsed.issues))
  ) {
    throw new Error("invalid wiki-runtime governance summary");
  }
  return {
    readiness: parseLiteral(
      parsed.readiness,
      ["not_enabled", "ready", "stale", "blocked", "conflict"] as const,
      "governance.readiness",
    ),
    fingerprint: parseNullableString(parsed.fingerprint, "governance.fingerprint"),
    active_count: parsed.active_count,
    archived_count: parsed.archived_count,
    issues: (parsed.issues ?? []).map(parseGovernanceIssue),
    recommended_action: parseLiteral(
      parsed.recommended_action,
      ["none", "update", "review_governance"] as const,
      "governance.recommended_action",
    ),
  };
}

function parseGovernanceChange(value: unknown): GovernanceChangeSummary {
  if (!isRecord(value) || typeof value.id !== "string" || typeof value.stage !== "string" || !isRecord(value.gate)) {
    throw new Error("invalid wiki-runtime governance change");
  }
  return {
    id: value.id,
    location: parseLiteral(value.location, ["active", "archived"] as const, "governance.change.location"),
    stage: value.stage,
    role: parseNullableString(value.role, "governance.change.role"),
    parent: parseNullableString(value.parent, "governance.change.parent"),
    order: parseNullableNumber(value.order, "governance.change.order"),
    depends_on: parseStringArray(value.depends_on ?? [], "governance.change.depends_on"),
    gate: value.gate,
  };
}

function parseGovernanceReport(value: Record<string, unknown>): GovernanceChangesReport | GovernanceChangeReport | GovernanceValidateReport {
  const governance = parseGovernanceSummary(value.governance);
  if (Array.isArray(value.changes))
return { governance, changes: value.changes.map(parseGovernanceChange) };
  if (isRecord(value.change))
return { governance, change: parseGovernanceChange(value.change) };
  if (typeof value.change_id === "string" && isRecord(value.validation)) {
    const validation = value.validation;
    if (typeof validation.valid !== "boolean")
throw new Error("invalid wiki-runtime governance validation");
    return {
      governance,
      change_id: value.change_id,
      validation: {
        valid: validation.valid,
        readiness: parseLiteral(validation.readiness, ["not_enabled", "ready", "stale", "blocked", "conflict"] as const, "governance.validation.readiness"),
        rule_results: Array.isArray(validation.rule_results) ? validation.rule_results.filter(isRecord) : [],
        issues: Array.isArray(validation.issues) ? validation.issues.map(parseGovernanceIssue) : [],
      },
    };
  }
  throw new Error("invalid wiki-runtime governance report");
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

function parseRuntimeReadiness(value: unknown): RuntimeReadiness {
  const parsed = value as Partial<RuntimeReadiness>;

  if (!isRecord(parsed)) {
    throw new Error("invalid wiki-runtime readiness");
  }

  return {
    index: parseLiteral(
      parsed.index,
      ["ready", "stale", "missing", "rebuilding", "conflict", "blocked", "not_enabled"] as const,
      "readiness.index",
    ),
    knowledge: parseLiteral(
      parsed.knowledge,
      ["ready", "stale", "missing", "rebuilding", "conflict", "blocked", "not_enabled"] as const,
      "readiness.knowledge",
    ),
    projection: parseLiteral(
      parsed.projection,
      ["ready", "stale", "missing", "rebuilding", "conflict", "blocked", "not_enabled"] as const,
      "readiness.projection",
    ),
    fusion: parseLiteral(
      parsed.fusion,
      ["ready", "degraded", "blocked"] as const,
      "readiness.fusion",
    ),
    restored_level: parseLiteral(
      parsed.restored_level,
      ["none", "level1", "level2"] as const,
      "readiness.restored_level",
    ),
    snapshot_id: parseNullableString(parsed.snapshot_id, "readiness.snapshot_id"),
    reasons: parseStringArray(parsed.reasons ?? [], "readiness.reasons"),
  };
}

function parseWorkflowTerminalData(value: Record<string, unknown>): WorkflowTerminalData {
  return {
    ...value,
    governance:
      value.governance == null ? value.governance : parseGovernanceSummary(value.governance),
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
    || !isRecord(value.readiness)
    || !isRecord(value.governance)
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
    readiness: parseRuntimeReadiness(value.readiness),
    recommended_action: parseLiteral(
      value.recommended_action,
      ["none", "init", "review", "review_governance", "update", "rebuild", "sync"] as const,
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
    governance: parseGovernanceSummary(value.governance),
  };
}

function parseUpdateData(value: Record<string, unknown>): WikiUpdateData {
  if (
    typeof value.previous_state !== "string"
    || typeof value.state !== "string"
    || !Array.isArray(value.updated_pages)
    || !isRecord(value.governance)
  ) {
    throw new TypeError("invalid wiki-runtime update payload");
  }
  return {
    ...parseWorkflowTerminalData(value),
    previous_state: value.previous_state,
    state: value.state,
    updated_pages: parseStringArray(value.updated_pages, "updated_pages"),
    governance: parseGovernanceSummary(value.governance),
  };
}

function parseQueryData(value: Record<string, unknown>): WikiQueryData {
  if (
    typeof value.term !== "string"
    || typeof value.runtime_state !== "string"
    || !isRecord(value.readiness)
    || !isRecord(value.governance)
    || typeof value.provenance_summary !== "string"
  ) {
    throw new TypeError("invalid wiki-runtime query payload");
  }

  return {
    ...value,
    term: value.term,
    runtime_state: value.runtime_state,
    readiness: parseRuntimeReadiness(value.readiness),
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
      ["none", "init", "review", "review_governance", "update", "rebuild", "sync"] as const,
      "recommended_action",
    ),
    matched_pages: parseStringArray(value.matched_pages ?? [], "matched_pages"),
    provenance_summary: value.provenance_summary,
    governance: parseGovernanceSummary(value.governance),
    route_groups: Array.isArray(value.route_groups)
      ? value.route_groups.map(parseQueryRouteGroup)
      : [],
    results: Array.isArray(value.results)
      ? value.results.map(parseQueryResult)
      : [],
  };
}

function parseKnownData(value: unknown): CoreKnownData {
  if (!isRecord(value)) {
    return value as CoreKnownData;
  }

  if ((value.outcome === "ready" || value.outcome === "partial") && isRecord(value.bootstrap) && isRecord(value.landing)) {
    const bootstrap = value.bootstrap;
    if (!Array.isArray(bootstrap.hosts))
throw new Error("invalid wiki-runtime bootstrap report");
    return {
      outcome: value.outcome,
      bootstrap: {
        outcome: parseLiteral(bootstrap.outcome, ["ready", "partial", "failed"] as const, "bootstrap.outcome"),
        hosts: bootstrap.hosts.filter(isRecord),
        recoveryHint: parseNullableString(bootstrap.recoveryHint, "bootstrap.recoveryHint"),
      },
      runtime: value.runtime,
      landing: parseStatusData(value.landing),
      recovery_hint: parseNullableString(value.recovery_hint, "recovery_hint"),
    };
  }

  if (
    typeof value.state === "string"
    && "readiness" in value
  ) {
    return parseStatusData(value);
  }

  if ("governance" in value && ("changes" in value || "change" in value || "validation" in value)) {
    return parseGovernanceReport(value);
  }

  if (
    typeof value.term === "string"
    && typeof value.runtime_state === "string"
    && "readiness" in value
    && "query_mode" in value
    && "query_trust" in value
  ) {
    return parseQueryData(value);
  }

  if ("previous_state" in value) {
    return parseUpdateData(value);
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
    errorKind:
      parsed.errorKind == null
        ? parsed.errorKind
        : parseLiteral(
            parsed.errorKind,
            [
              "invalid_argument",
              "governance_not_enabled",
              "change_not_found",
              "workflow_failed",
              "protocol_error",
              "internal_error",
            ] as const,
            "errorKind",
          ),
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

  if (parsed.type === "agent_session_start") {
    if (!isRecord(parsed.request)) {
      throw new Error("invalid wiki-runtime agent session event");
    }
    return parsed as CoreAgentEvent;
  }
  if (["agent_message", "agent_tool_call", "agent_tool_result"].includes(parsed.type ?? "")) {
    const agentEvent = parsed as Record<string, unknown>;
    if (typeof agentEvent.requestId !== "string" || !("message" in agentEvent)) {
      throw new Error("invalid wiki-runtime agent message event");
    }
    return parsed as CoreAgentEvent;
  }
  if (parsed.type === "agent_final") {
    const agentEvent = parsed as Record<string, unknown>;
    if (typeof agentEvent.requestId !== "string" || !isRecord(agentEvent.response)) {
      throw new Error("invalid wiki-runtime agent final event");
    }
    return parsed as CoreAgentEvent;
  }
  if (parsed.type === "agent_abort") {
    const agentEvent = parsed as Record<string, unknown>;
    if (typeof agentEvent.requestId !== "string" || typeof agentEvent.reason !== "string") {
      throw new TypeError("invalid wiki-runtime agent abort event");
    }
    return parsed as CoreAgentEvent;
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
