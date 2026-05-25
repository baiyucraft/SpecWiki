import { expect, test } from "vitest";

import { buildRunTestProjectsSummary } from "../run-test-projects.mjs";
import { buildLifecycleSummary } from "../test-wiki-lifecycle.mjs";
import {
  CAPABILITY_TEST_MATRIX,
  FORMAL_QUALITY_GATES,
} from "../testing/quality-gates.mjs";

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
  expect(summary.decision).toBe("diagnostic");
  expect(summary.blocking).toBe(false);
  expect(summary.formal_gates.artifact_validity.decision).toBe("diagnostic");
  expect(summary.formal_gates.artifact_validity.blocking).toBe(false);
  expect(summary.formal_gates.restore_validity.decision).toBe("not_covered");
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
  expect(summary.formal_gates.restore_validity.decision).toBe("blocker");
  expect(summary.formal_gates.query_route_contract.decision).toBe("blocker");
  expect(summary.formal_gates.status_recommended_action_stability.decision).toBe("blocker");
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
    "storybook",
    "dagger",
  ]);
  expect(CAPABILITY_TEST_MATRIX.knowledge_quality_gates.workflow).toEqual([
    "run-test-projects",
    "test-wiki-lifecycle",
  ]);
});
