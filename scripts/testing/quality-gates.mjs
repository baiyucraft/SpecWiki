/**
 * 统一收口当前 knowledge quality gates / acceptance harness 的最小共享契约。
 *
 * 这里不负责执行测试；它只定义正式 gate 名称、测试面维度和脚本汇总结果的稳定结构，
 * 让 `run-test-projects`、`test-wiki-lifecycle` 与 reference reporting 不再各说各话。
 */

/** 当前 quality gate 聚合合同版本。 */
export const QUALITY_GATE_CONTRACT_VERSION = "knowledge-quality-gates.v2";

export const FORMAL_QUALITY_GATES = Object.freeze([
  "artifact_validity",
  "restore_validity",
  "query_route_contract",
  "status_recommended_action_stability",
]);

/** Gate 层级闭集；diagnostic 是 observation，不是 gate level。 */
export const QUALITY_GATE_LEVELS = Object.freeze([
  "formal_quality_gate",
  "primary_gate",
  "baseline_guard",
]);

/** 单个 gate 的结果闭集。 */
export const QUALITY_GATE_DECISIONS = Object.freeze([
  "pass",
  "blocker",
  "not_covered",
]);

/** 整体验收结果闭集。 */
export const ACCEPTANCE_DECISIONS = Object.freeze([
  "pass",
  "blocker",
  "incomplete",
  "diagnostic",
]);

const FAILURE_OWNER_LEVELS = new Set(["formal_quality_gate", "primary_gate"]);

/**
 * 当前 gate registry。脚本 adapter 只能引用这里的稳定 gate id，不得自造 level/decision。
 */
export const QUALITY_GATE_REGISTRY = Object.freeze(Object.fromEntries([
  ...FORMAL_QUALITY_GATES.map(gateId => [gateId, Object.freeze({
    gate_id: gateId,
    gate_level: "formal_quality_gate",
    gate_scope: "workspace",
    required_companion_gates: Object.freeze([]),
  })]),
  ["core_scenario_primary", Object.freeze({
    gate_id: "core_scenario_primary",
    gate_level: "primary_gate",
    gate_scope: "core_scenarios",
    required_companion_gates: Object.freeze([...FORMAL_QUALITY_GATES]),
  })],
  ["reference_fidelity_primary", Object.freeze({
    gate_id: "reference_fidelity_primary",
    gate_level: "primary_gate",
    gate_scope: "reference_fidelity",
    required_companion_gates: Object.freeze([...FORMAL_QUALITY_GATES]),
  })],
  ["run_test_projects_baseline", Object.freeze({
    gate_id: "run_test_projects_baseline",
    gate_level: "baseline_guard",
    gate_scope: "batch_init",
    required_companion_gates: Object.freeze([]),
  })],
  ["lifecycle_baseline", Object.freeze({
    gate_id: "lifecycle_baseline",
    gate_level: "baseline_guard",
    gate_scope: "lifecycle",
    required_companion_gates: Object.freeze([]),
  })],
]));

function validateAcceptancePlanGateSets(plan) {
  const requiredPrimaryGates = plan.required_primary_gates ?? [];
  if (!Array.isArray(requiredPrimaryGates))
    throw new TypeError("plan requires required_primary_gates array");
  for (const gateId of plan.required_gates) {
    if (QUALITY_GATE_REGISTRY[gateId]?.gate_level !== "formal_quality_gate")
      throw new TypeError(`required_gates must reference a formal quality gate: ${gateId}`);
  }
  for (const gateId of requiredPrimaryGates) {
    const definition = QUALITY_GATE_REGISTRY[gateId];
    if (definition?.gate_level !== "primary_gate")
      throw new TypeError(`required_primary_gates must reference a primary gate: ${gateId}`);
    const missingCompanions = definition.required_companion_gates
      .filter(companion => !plan.required_gates.includes(companion));
    if (missingCompanions.length > 0)
      throw new TypeError(`primary gate ${gateId} requires companion gates: ${missingCompanions.join(", ")}`);
  }
  for (const gateId of plan.required_guards) {
    if (QUALITY_GATE_REGISTRY[gateId]?.gate_level !== "baseline_guard")
      throw new TypeError(`required_guards must reference a baseline guard: ${gateId}`);
  }
}

function stringArray(value, field) {
  if (!Array.isArray(value) || value.some(item => typeof item !== "string" || item.trim() === ""))
    throw new TypeError(`${field} must be a string array`);
  return [...new Set(value)].sort();
}

function normalizeFailures(failures) {
  if (!Array.isArray(failures))
    throw new TypeError("failures must be an array");

  const normalized = new Map();
  for (const failure of failures) {
    if (!failure || typeof failure !== "object")
      throw new TypeError("failure must be an object");
    for (const field of ["failure_id", "owner_gate_id", "source_ref", "assertion_ref"]) {
      if (typeof failure[field] !== "string" || failure[field].trim() === "")
        throw new TypeError(`failure requires ${field}`);
    }
    const owner = QUALITY_GATE_REGISTRY[failure.owner_gate_id];
    if (!owner)
      throw new TypeError(`unknown owner gate: ${failure.owner_gate_id}`);
    if (!FAILURE_OWNER_LEVELS.has(owner.gate_level))
      throw new TypeError(`failure owner must be formal or primary: ${failure.owner_gate_id}`);

    const current = normalized.get(failure.failure_id);
    if (current && current.owner_gate_id !== failure.owner_gate_id)
      throw new TypeError(`failure ${failure.failure_id} has multiple owners`);
    if (current) {
      if (current.source_ref !== failure.source_ref
        || current.assertion_ref !== failure.assertion_ref) {
        throw new TypeError(`failure ${failure.failure_id} has conflicting identity fields`);
      }
      current.evidence_refs = [...new Set([
        ...current.evidence_refs,
        ...stringArray(failure.evidence_refs ?? [], "failure.evidence_refs"),
      ])].sort();
      continue;
    }
    normalized.set(failure.failure_id, {
      failure_id: failure.failure_id,
      owner_gate_id: failure.owner_gate_id,
      source_ref: failure.source_ref,
      assertion_ref: failure.assertion_ref,
      evidence_refs: stringArray(failure.evidence_refs ?? [], "failure.evidence_refs"),
    });
  }

  return [...normalized.values()].sort((left, right) => left.failure_id.localeCompare(right.failure_id));
}

function normalizeGateResult(gateId, current, plan, failureIds) {
  const definition = QUALITY_GATE_REGISTRY[gateId];
  if (!definition)
    throw new TypeError(`unknown gate: ${gateId}`);
  const decision = current?.decision ?? "not_covered";
  if (!QUALITY_GATE_DECISIONS.includes(decision))
    throw new TypeError(`invalid gate decision for ${gateId}: ${decision}`);
  const failureRefs = stringArray(current?.failure_refs ?? [], `${gateId}.failure_refs`);
  for (const failureRef of failureRefs) {
    if (!failureIds.has(failureRef))
      throw new TypeError(`${gateId} references unknown failure: ${failureRef}`);
  }
  const isRequired = plan.required_gates.includes(gateId)
    || (plan.required_primary_gates ?? []).includes(gateId)
    || plan.required_guards.includes(gateId);
  if (decision === "blocker" && failureRefs.length === 0)
    throw new TypeError(`blocker gate ${gateId} requires failure refs`);

  return {
    ...definition,
    decision,
    blocking: decision === "blocker" && isRequired && FAILURE_OWNER_LEVELS.has(definition.gate_level),
    evidence_refs: stringArray(current?.evidence_refs ?? [], `${gateId}.evidence_refs`),
    failure_refs: failureRefs,
  };
}

/**
 * 将 overall decision 映射到脚本进程退出码。
 *
 * `report_only` 只允许报告命令以 0 退出，不会修改 summary 中的真实 decision。
 *
 * @param {Record<string, unknown>} summary acceptance summary 或最小 decision 对象。
 * @returns {0 | 1 | 2} pass、blocker、incomplete/diagnostic 的统一退出码。
 */
export function exitCodeForAcceptance(summary) {
  if (!summary || typeof summary !== "object" || !ACCEPTANCE_DECISIONS.includes(summary.decision))
    throw new TypeError("acceptance summary has invalid decision");
  if (summary.report_only === true)
    return 0;
  if (summary.decision === "pass")
    return 0;
  if (summary.decision === "blocker")
    return 1;
  return 2;
}

/**
 * 将 adapter evidence 聚合为唯一 gate/failure/overall decision。
 *
 * @param {Record<string, unknown>} input plan、gate results、failures、diagnostics 与 scenario results。
 * @returns {Record<string, unknown>} 排序稳定的 v2 acceptance summary。
 */
export function aggregateGateResults({
  plan,
  gate_results: gateResults = {},
  failures = [],
  diagnostics = [],
  scenario_results: scenarioResults = [],
}) {
  if (!plan || typeof plan !== "object")
    throw new TypeError("aggregateGateResults requires plan");
  for (const field of ["required_gates", "required_guards"]) {
    if (!Array.isArray(plan[field]))
      throw new TypeError(`plan requires ${field}`);
  }
  validateAcceptancePlanGateSets(plan);
  if (!gateResults || typeof gateResults !== "object" || Array.isArray(gateResults))
    throw new TypeError("gate_results must be an object");
  if (!Array.isArray(diagnostics) || !Array.isArray(scenarioResults))
    throw new TypeError("diagnostics and scenario_results must be arrays");

  const normalizedFailures = normalizeFailures(failures);
  const failureIds = new Set(normalizedFailures.map(failure => failure.failure_id));
  const failuresById = new Map(normalizedFailures.map(failure => [failure.failure_id, failure]));
  const requestedGateIds = new Set([
    ...Object.keys(gateResults),
    ...plan.required_gates,
    ...(plan.required_primary_gates ?? []),
    ...plan.required_guards,
  ]);
  const normalizedGateResults = Object.fromEntries(
    [...requestedGateIds]
      .sort()
      .map(gateId => [
        gateId,
        normalizeGateResult(gateId, gateResults[gateId], plan, failureIds),
      ]),
  );

  for (const failure of normalizedFailures) {
    const ownerResult = normalizedGateResults[failure.owner_gate_id];
    if (!ownerResult || !ownerResult.failure_refs.includes(failure.failure_id))
      throw new TypeError(`owner gate ${failure.owner_gate_id} must reference ${failure.failure_id}`);
    if (ownerResult.decision !== "blocker")
      throw new TypeError(`owner gate ${failure.owner_gate_id} must be blocker for ${failure.failure_id}`);
  }

  for (const [gateId, gateResult] of Object.entries(normalizedGateResults)) {
    if (!FAILURE_OWNER_LEVELS.has(gateResult.gate_level))
      continue;
    for (const failureRef of gateResult.failure_refs) {
      if (failuresById.get(failureRef)?.owner_gate_id !== gateId)
        throw new TypeError(`gate ${gateId} has non-owner failure ref: ${failureRef}`);
    }
  }

  for (const guardId of plan.required_guards) {
    const guardResult = normalizedGateResults[guardId];
    for (const failureRef of guardResult?.failure_refs ?? []) {
      const ownerResult = normalizedGateResults[failuresById.get(failureRef)?.owner_gate_id];
      if (ownerResult?.decision === "blocker")
        ownerResult.blocking = true;
    }
  }

  const values = Object.values(normalizedGateResults);
  const requiredIds = [
    ...plan.required_gates,
    ...(plan.required_primary_gates ?? []),
    ...plan.required_guards,
  ];
  const missingRequired = requiredIds.some(gateId => normalizedGateResults[gateId]?.decision === "not_covered");
  const scenarioDecisions = new Set(scenarioResults.map(result => result?.decision));
  const scenarioIncomplete = scenarioDecisions.has("incomplete")
    || (requiredIds.length > 0 && scenarioDecisions.has("diagnostic"));
  const decision = values.some(result => result.blocking) || scenarioDecisions.has("blocker")
    ? "blocker"
    : missingRequired || scenarioIncomplete
      ? "incomplete"
      : (diagnostics.length > 0 || scenarioDecisions.has("diagnostic")) && requiredIds.length === 0
        ? "diagnostic"
        : "pass";

  const summary = {
    contract_version: QUALITY_GATE_CONTRACT_VERSION,
    acceptance_plan_id: plan.plan_id,
    decision,
    blocking: decision === "blocker",
    gate_results: normalizedGateResults,
    diagnostics: [...diagnostics].sort((left, right) =>
      String(left?.diagnostic_id ?? "").localeCompare(String(right?.diagnostic_id ?? ""))),
    failures: normalizedFailures,
    scenario_results: [...scenarioResults].sort((left, right) =>
      String(left?.scenario_id ?? "").localeCompare(String(right?.scenario_id ?? ""))),
    report_only: plan.report_only === true,
  };
  return {
    ...summary,
    exit_code: exitCodeForAcceptance(summary),
  };
}

export const REQUIRED_TEST_SURFACES = Object.freeze([
  "model_schema",
  "artifact_recovery",
  "workflow",
  "sample",
]);

export const CAPABILITY_TEST_MATRIX = Object.freeze({
  declared_lifecycle_completeness: Object.freeze({
    model_schema: ["declared records", "health signals"],
    artifact_recovery: ["declared snapshot writeback", "stale scope propagation"],
    workflow: ["sync", "status", "update"],
    sample: ["acceptance plan primary fixtures"],
    baseline: ["run-test-projects", "test-wiki-lifecycle"],
  }),
  derived_research_contract: Object.freeze({
    model_schema: ["research summary", "page digest"],
    artifact_recovery: ["research artifact restore", "derived freshness"],
    workflow: ["init", "update", "page_render"],
    sample: ["acceptance plan primary fixtures"],
    baseline: ["run-test-projects", "test-wiki-lifecycle"],
  }),
  projection_readiness_recovery: Object.freeze({
    model_schema: ["projection digest", "runtime readiness"],
    artifact_recovery: ["cold restore", "rebuild recovery"],
    workflow: ["status", "query", "rebuild"],
    sample: ["acceptance plan primary fixtures"],
    baseline: ["run-test-projects", "test-wiki-lifecycle"],
  }),
  governance_conflict_artifacts: Object.freeze({
    model_schema: ["conflict records", "governance health"],
    artifact_recovery: ["conflict snapshot persistence", "conflict cleanup"],
    workflow: ["sync", "status", "update"],
    sample: ["acceptance plan primary fixtures"],
    baseline: ["run-test-projects", "test-wiki-lifecycle"],
  }),
  query_route_completeness: Object.freeze({
    model_schema: ["query mode", "query trust"],
    artifact_recovery: ["cache restore queryability", "route provenance"],
    workflow: ["query", "status"],
    sample: ["acceptance plan primary fixtures"],
    baseline: ["run-test-projects", "test-wiki-lifecycle"],
    reference: ["collect-reference-project-reports"],
  }),
  answer_assembly_contract: Object.freeze({
    model_schema: ["answer envelope", "supporting refs"],
    artifact_recovery: ["answer substrate survives restore", "health does not replace substrate"],
    workflow: ["query transport", "query runtime"],
    sample: ["acceptance plan primary fixtures"],
    baseline: ["run-test-projects", "test-wiki-lifecycle"],
    reference: ["collect-reference-project-reports"],
  }),
  knowledge_quality_gates: Object.freeze({
    model_schema: ["gate summary", "decision semantics"],
    artifact_recovery: ["restore gate", "artifact validity gate"],
    workflow: ["run-test-projects", "test-wiki-lifecycle"],
    sample: ["acceptance plan primary fixtures"],
    baseline: ["run-test-projects", "test-wiki-lifecycle"],
    reference: ["collect-reference-project-reports"],
  }),
});
