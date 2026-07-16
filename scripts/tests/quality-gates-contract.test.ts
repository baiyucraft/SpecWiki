import { expect, test } from "vitest";

import { buildRunTestProjectsSummary } from "../run-test-projects.mjs";
import { buildPrimaryGateSummary } from "../collect-reference-project-reports.mjs";
import { buildLifecycleSummary } from "../test-wiki-lifecycle.mjs";
import * as qualityGates from "../testing/quality-gates.mjs";
import {
  CAPABILITY_TEST_MATRIX,
  FORMAL_QUALITY_GATES,
} from "../testing/quality-gates.mjs";

function referenceAcceptancePlan(primaryFixtures = ["fixture-a"], overallMatchRateMin = 95) {
  return {
    plan_id: `reference-${overallMatchRateMin}`,
    primary_fixtures: primaryFixtures,
    required_primary_gates: ["reference_fidelity_primary"],
    required_gates: [...FORMAL_QUALITY_GATES],
    required_guards: [],
    thresholds: {
      overall_match_rate_min: overallMatchRateMin,
      reuse_overage_max: 0,
      median_skeleton_fidelity_min: 0.8,
      median_key_source_coverage_min: 0.7,
      require_warm_stability: true,
    },
    report_only: false,
  };
}

test("同一 failure identity 只产生一个 owning blocker", () => {
  expect(typeof qualityGates.aggregateGateResults).toBe("function");
  if (!qualityGates.aggregateGateResults)
    return;

  const plan = {
    plan_id: "single-owner-plan",
    primary_fixtures: ["fixture-a"],
    required_gates: [...FORMAL_QUALITY_GATES],
    required_guards: ["run_test_projects_baseline"],
    report_only: false,
  };
  const failures = [{
    failure_id: "F-001",
    owner_gate_id: "artifact_validity",
    source_ref: "fixture-a",
    assertion_ref: "artifact manifest is valid",
    evidence_refs: ["evidence/artifact.json"],
  }];
  const summary = qualityGates.aggregateGateResults({
    plan,
    gate_results: {
      artifact_validity: {
        decision: "blocker",
        evidence_refs: ["evidence/artifact.json"],
        failure_refs: ["F-001"],
      },
      restore_validity: { decision: "pass", evidence_refs: [], failure_refs: [] },
      query_route_contract: { decision: "pass", evidence_refs: [], failure_refs: [] },
      status_recommended_action_stability: { decision: "pass", evidence_refs: [], failure_refs: [] },
      run_test_projects_baseline: {
        decision: "blocker",
        evidence_refs: ["evidence/artifact.json"],
        failure_refs: ["F-001"],
      },
    },
    failures,
    diagnostics: [],
    scenario_results: [],
  });

  expect(summary.failures).toHaveLength(1);
  expect(summary.failures[0].owner_gate_id).toBe("artifact_validity");
  expect(Object.values(summary.gate_results)
    .filter(gate => gate.gate_level === "formal_quality_gate" && gate.decision === "blocker"))
    .toHaveLength(1);
  expect(summary.gate_results.run_test_projects_baseline.failure_refs).toEqual(["F-001"]);
  expect(summary.gate_results.run_test_projects_baseline.blocking).toBe(false);
  expect(Object.values(summary.gate_results).filter(gate => gate.blocking)).toHaveLength(1);

  expect(() => qualityGates.aggregateGateResults!({
    plan,
    gate_results: {},
    failures: [
      failures[0],
      { ...failures[0], owner_gate_id: "restore_validity" },
    ],
    diagnostics: [],
    scenario_results: [],
  })).toThrow(/multiple owners/i);

  expect(() => qualityGates.aggregateGateResults!({
    plan,
    gate_results: {
      artifact_validity: {
        decision: "blocker",
        evidence_refs: ["evidence/artifact.json"],
        failure_refs: ["F-001"],
      },
    },
    failures: [
      failures[0],
      { ...failures[0], assertion_ref: "a different assertion" },
    ],
    diagnostics: [],
    scenario_results: [],
  })).toThrow(/conflicting identity/i);

  expect(() => qualityGates.aggregateGateResults!({
    plan,
    gate_results: {
      artifact_validity: {
        decision: "pass",
        evidence_refs: ["evidence/artifact.json"],
        failure_refs: ["F-001"],
      },
    },
    failures,
    diagnostics: [],
    scenario_results: [],
  })).toThrow(/owner gate.*blocker/i);

  expect(() => qualityGates.aggregateGateResults!({
    plan,
    gate_results: {
      artifact_validity: {
        decision: "blocker",
        evidence_refs: ["evidence/artifact.json"],
        failure_refs: [],
      },
    },
    failures: [],
    diagnostics: [],
    scenario_results: [],
  })).toThrow(/blocker.*failure/i);

  expect(() => qualityGates.aggregateGateResults!({
    plan,
    gate_results: {
      artifact_validity: {
        decision: "blocker",
        evidence_refs: ["evidence/artifact.json"],
        failure_refs: ["F-001"],
      },
      restore_validity: {
        decision: "blocker",
        evidence_refs: ["evidence/artifact.json"],
        failure_refs: ["F-001"],
      },
    },
    failures,
    diagnostics: [],
    scenario_results: [],
  })).toThrow(/non-owner/i);

  const guardRequired = qualityGates.aggregateGateResults!({
    plan: {
      ...plan,
      required_gates: [],
      required_guards: ["run_test_projects_baseline"],
    },
    gate_results: {
      artifact_validity: {
        decision: "blocker",
        evidence_refs: ["evidence/artifact.json"],
        failure_refs: ["F-001"],
      },
      run_test_projects_baseline: {
        decision: "blocker",
        evidence_refs: ["evidence/artifact.json"],
        failure_refs: ["F-001"],
      },
    },
    failures,
    diagnostics: [],
    scenario_results: [],
  });
  expect(guardRequired.decision).toBe("blocker");
  expect(guardRequired.gate_results.artifact_validity.blocking).toBe(true);
  expect(guardRequired.gate_results.run_test_projects_baseline.blocking).toBe(false);
});

test("required coverage 与 exit policy 保持一致", () => {
  expect(typeof qualityGates.exitCodeForAcceptance).toBe("function");
  if (!qualityGates.exitCodeForAcceptance)
    return;

  for (const [decision, expectedExit] of [
    ["pass", 0],
    ["blocker", 1],
    ["incomplete", 2],
    ["diagnostic", 2],
  ] as const) {
    expect(qualityGates.exitCodeForAcceptance({ decision, report_only: false })).toBe(expectedExit);
  }

  const incomplete = qualityGates.aggregateGateResults!({
    plan: {
      plan_id: "missing-required",
      primary_fixtures: ["fixture-a"],
      required_gates: ["artifact_validity"],
      required_guards: [],
      report_only: false,
    },
    gate_results: {},
    failures: [],
    diagnostics: [{
      diagnostic_id: "D-001",
      state: "runtime_incomplete",
      reason: "primary flow did not run",
      recommended_action: "resume",
      evidence_refs: [],
    }],
    scenario_results: [],
  });
  expect(incomplete.decision).toBe("incomplete");
  expect(incomplete.gate_results.artifact_validity.decision).toBe("not_covered");
  expect(qualityGates.exitCodeForAcceptance(incomplete)).toBe(2);
  expect(qualityGates.exitCodeForAcceptance({ ...incomplete, report_only: true })).toBe(0);
  expect(incomplete.decision).toBe("incomplete");

  expect(() => qualityGates.aggregateGateResults!({
    plan: {
      plan_id: "wrong-level-guard",
      required_gates: [],
      required_guards: ["artifact_validity"],
      report_only: false,
    },
    gate_results: {},
    failures: [],
    diagnostics: [],
    scenario_results: [],
  })).toThrow(/baseline guard/i);
  expect(() => qualityGates.aggregateGateResults!({
    plan: {
      plan_id: "missing-primary-companions",
      required_primary_gates: ["reference_fidelity_primary"],
      required_gates: [],
      required_guards: [],
      report_only: false,
    },
    gate_results: {},
    failures: [],
    diagnostics: [],
    scenario_results: [],
  })).toThrow(/companion gate/i);
});

test("lifecycle 按 assertion owner 聚合而非批量阻断", () => {
  const summary = buildLifecycleSummary([{
    proj: "fixture-a",
    ok: false,
    skipped: false,
    total: 1,
    failed: 1,
    runs: [{ label: "cold" }],
    failures: [{
      failure_id: "restore-manifest-invalid",
      owner_gate_id: "restore_validity",
      source_ref: "fixture-a",
      assertion_ref: "restore manifest validates",
      evidence_refs: ["evidence/restore.json"],
    }],
  }], {
    phase: "steady",
    acceptancePlan: {
      plan_id: "lifecycle-owner-plan",
      primary_fixtures: ["fixture-a"],
      required_gates: [...FORMAL_QUALITY_GATES],
      required_guards: [],
      report_only: false,
    },
  });

  expect(summary.gate_results).toBeDefined();
  if (!summary.gate_results)
    return;
  expect(summary.gate_results.restore_validity.decision).toBe("blocker");
  expect(summary.gate_results.query_route_contract.decision).toBe("not_covered");
  expect(summary.gate_results.status_recommended_action_stability.decision).toBe("not_covered");
  expect(summary.failures).toHaveLength(1);
});

test("project-set 与 reference adapters 输出 v2 decision 和 exit", () => {
  const projectSummary = buildRunTestProjectsSummary([{
    proj: "fixture-a",
    ok: true,
    skipped: false,
    runs: [{ diagnosticState: "runtime_incomplete" }],
  }]);
  expect(projectSummary.gate_results).toBeDefined();
  expect(projectSummary.decision).toBe("incomplete");
  expect(projectSummary.exit_code).toBe(2);

  const referenceSummary = buildPrimaryGateSummary([{
    project: "fixture-a",
    status: "runtime_incomplete",
    runtime_metrics: { runtime_state: "runtime_incomplete" },
    fidelity_metrics: {
      overall_match_rate: 0,
      reuse_overage: 0,
      median_skeleton_fidelity: 0,
      median_key_source_coverage: 0,
    },
    stability: { stable: false },
  }], { acceptancePlan: referenceAcceptancePlan() });
  expect(referenceSummary.gate_results.reference_fidelity_primary.decision).toBe("blocker");
  expect(referenceSummary.exit_code).toBe(1);

  for (const summary of [
    buildRunTestProjectsSummary([]),
    buildRunTestProjectsSummary([{ proj: "missing", ok: true, skipped: true, runs: [] }]),
    buildLifecycleSummary([], { phase: "full" }),
    buildPrimaryGateSummary([], { acceptancePlan: referenceAcceptancePlan() }),
  ]) {
    expect(summary.decision).toBe("incomplete");
    expect(summary.exit_code).toBe(2);
  }
});

test("reference primary 阈值来自 acceptance plan", () => {
  const result = {
    project: "fixture-a",
    status: "ready",
    runtime_metrics: { runtime_state: "ready" },
    fidelity_metrics: {
      overall_match_rate: 90,
      reuse_overage: 0,
      median_skeleton_fidelity: 1,
      median_key_source_coverage: 1,
    },
    stability: { stable: true },
  };
  const strict = buildPrimaryGateSummary([result], {
    acceptancePlan: referenceAcceptancePlan(["fixture-a"], 95),
  });
  const relaxed = buildPrimaryGateSummary([result], {
    acceptancePlan: referenceAcceptancePlan(["fixture-a"], 80),
  });
  expect(strict.gate_results.reference_fidelity_primary.decision).toBe("blocker");
  expect(relaxed.gate_results.reference_fidelity_primary.decision).toBe("pass");
});

test("run-test-projects summary 会把 diagnostic runtime 与 blocker 区分开", () => {
  const summary = buildRunTestProjectsSummary([
    {
      proj: "storybook",
      ok: true,
      skipped: false,
      runs: [
        {
          diagnosticState: "runtime_incomplete",
        },
      ],
    },
    {
      proj: "dagger",
      ok: true,
      skipped: false,
      runs: [],
    },
  ]);

  expect(summary.gate_level).toBe("baseline_guard");
  expect(summary.gate_scope).toBe("batch_init");
  expect(summary.decision).toBe("incomplete");
  expect(summary.blocking).toBe(false);
  expect(summary.gate_results.artifact_validity.decision).toBe("not_covered");
  expect(summary.gate_results.run_test_projects_baseline.decision).toBe("not_covered");
  expect(summary.exit_code).toBe(2);
  expect(summary.totals.diagnostic_projects).toBe(1);
  expect(summary.capability_test_matrix).toHaveLength(3);
});

test("lifecycle summary 会把 formal lifecycle failures 聚合成 blocker", () => {
  const summary = buildLifecycleSummary([
    {
      proj: "storybook",
      ok: false,
      skipped: false,
      total: 24,
      failed: 2,
      runs: [{ label: "cold" }],
    },
    {
      proj: "dagger",
      ok: true,
      skipped: false,
      total: 20,
      failed: 0,
      runs: [{ label: "cold" }],
    },
  ], { phase: "steady" });

  expect(summary.gate_level).toBe("baseline_guard");
  expect(summary.gate_scope).toBe("lifecycle");
  expect(summary.phase).toBe("steady");
  expect(summary.decision).toBe("blocker");
  expect(summary.blocking).toBe(true);
  expect(summary.gate_results.artifact_validity.decision).toBe("blocker");
  expect(summary.gate_results.restore_validity.decision).toBe("not_covered");
  expect(summary.gate_results.query_route_contract.decision).toBe("not_covered");
  expect(summary.gate_results.status_recommended_action_stability.decision).toBe("not_covered");
  expect(summary.gate_results.lifecycle_baseline.decision).toBe("blocker");
  expect(summary.totals.total_assertions).toBe(44);
  expect(summary.totals.failed_assertions).toBe(2);
});

test("quality gate helper 固定 formal gates 与 capability matrix", () => {
  expect(FORMAL_QUALITY_GATES).toEqual([
    "artifact_validity",
    "restore_validity",
    "query_route_contract",
    "status_recommended_action_stability",
  ]);
  expect(CAPABILITY_TEST_MATRIX.answer_assembly_contract.sample).toEqual([
    "acceptance plan primary fixtures",
  ]);
  expect(CAPABILITY_TEST_MATRIX.knowledge_quality_gates.workflow).toEqual([
    "run-test-projects",
    "test-wiki-lifecycle",
  ]);
  expect(qualityGates).not.toHaveProperty("createFormalGateResults");
  expect(qualityGates).not.toHaveProperty("buildAcceptanceHarnessSummary");
});
