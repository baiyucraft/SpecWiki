/**
 * 统一收口当前 knowledge quality gates / acceptance harness 的最小共享契约。
 *
 * 这里不负责执行测试；它只定义正式 gate 名称、测试面维度和脚本汇总结果的稳定结构，
 * 让 `run-test-projects`、`test-wiki-lifecycle` 与 reference reporting 不再各说各话。
 */

export const QUALITY_GATE_CONTRACT_VERSION = "knowledge-quality-gates.v1";

export const FORMAL_QUALITY_GATES = Object.freeze([
  "artifact_validity",
  "restore_validity",
  "query_route_contract",
  "status_recommended_action_stability",
]);

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
    sample: ["storybook", "dagger"],
    baseline: ["run-test-projects", "test-wiki-lifecycle"],
  }),
  derived_research_contract: Object.freeze({
    model_schema: ["research summary", "page digest"],
    artifact_recovery: ["research artifact restore", "derived freshness"],
    workflow: ["init", "update", "page_render"],
    sample: ["storybook", "dagger"],
    baseline: ["run-test-projects", "test-wiki-lifecycle"],
  }),
  projection_readiness_recovery: Object.freeze({
    model_schema: ["projection digest", "runtime readiness"],
    artifact_recovery: ["cold restore", "rebuild recovery"],
    workflow: ["status", "query", "rebuild"],
    sample: ["storybook", "dagger"],
    baseline: ["run-test-projects", "test-wiki-lifecycle"],
  }),
  governance_conflict_artifacts: Object.freeze({
    model_schema: ["conflict records", "governance health"],
    artifact_recovery: ["conflict snapshot persistence", "conflict cleanup"],
    workflow: ["sync", "status", "update"],
    sample: ["storybook", "dagger"],
    baseline: ["run-test-projects", "test-wiki-lifecycle"],
  }),
  query_route_completeness: Object.freeze({
    model_schema: ["query mode", "query trust"],
    artifact_recovery: ["cache restore queryability", "route provenance"],
    workflow: ["query", "status"],
    sample: ["storybook", "dagger"],
    baseline: ["run-test-projects", "test-wiki-lifecycle"],
    reference: ["collect-reference-project-reports"],
  }),
  answer_assembly_contract: Object.freeze({
    model_schema: ["answer envelope", "supporting refs"],
    artifact_recovery: ["answer substrate survives restore", "health does not replace substrate"],
    workflow: ["query transport", "query runtime"],
    sample: ["storybook", "dagger"],
    baseline: ["run-test-projects", "test-wiki-lifecycle"],
    reference: ["collect-reference-project-reports"],
  }),
  knowledge_quality_gates: Object.freeze({
    model_schema: ["gate summary", "decision semantics"],
    artifact_recovery: ["restore gate", "artifact validity gate"],
    workflow: ["run-test-projects", "test-wiki-lifecycle"],
    sample: ["storybook", "dagger"],
    baseline: ["run-test-projects", "test-wiki-lifecycle"],
    reference: ["collect-reference-project-reports"],
  }),
});

export function createFormalGateResults(gateResults = {}) {
  return Object.fromEntries(
    FORMAL_QUALITY_GATES.map((gate) => {
      const current = gateResults[gate];
      if (!current) {
        return [gate, {
          covered: false,
          decision: "not_covered",
          blocking: false,
          evidence_refs: [],
        }];
      }
      return [gate, {
        covered: true,
        decision: current.decision ?? "pass",
        blocking: current.blocking ?? current.decision === "blocker",
        evidence_refs: current.evidence_refs ?? [],
      }];
    }),
  );
}

export function buildAcceptanceHarnessSummary({
  gateLevel,
  gateScope,
  command,
  decision,
  totals,
  projectResults = [],
  formalGates = createFormalGateResults(),
  notes = [],
  relevantCapabilities = [],
  samples = [],
  phase = null,
  requiredCompanionGates = [],
  fidelityInputOnly = false,
}) {
  return {
    contract_version: QUALITY_GATE_CONTRACT_VERSION,
    gate_level: gateLevel,
    gate_scope: gateScope,
    command,
    decision,
    blocking: decision === "blocker",
    phase,
    samples,
    totals: {
      total_projects: totals.totalProjects ?? 0,
      passed_projects: totals.passedProjects ?? 0,
      failed_projects: totals.failedProjects ?? 0,
      skipped_projects: totals.skippedProjects ?? 0,
      diagnostic_projects: totals.diagnosticProjects ?? 0,
      total_assertions: totals.totalAssertions ?? 0,
      failed_assertions: totals.failedAssertions ?? 0,
    },
    formal_gates: formalGates,
    required_test_surfaces: [...REQUIRED_TEST_SURFACES],
    relevant_capabilities: relevantCapabilities,
    capability_test_matrix: relevantCapabilities
      .map((capability) => ({
        capability,
        ...(CAPABILITY_TEST_MATRIX[capability] ?? {}),
      })),
    project_results: projectResults,
    notes,
    fidelity_input_only: fidelityInputOnly,
    required_companion_gates: requiredCompanionGates,
  };
}
