import { existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

import { expect, test } from "vitest";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..");
const matrixModulePath = path.join(root, "scripts", "testing", "core-scenario-acceptance.mjs");

test("固定 9 个核心场景及其支持边界", async () => {
  expect(existsSync(matrixModulePath), "missing core scenario acceptance matrix module").toBe(true);
  if (!existsSync(matrixModulePath))
    return;

  const module = await import(pathToFileURL(matrixModulePath).href) as {
    CORE_SCENARIO_ACCEPTANCE_MATRIX: Array<Record<string, unknown>>;
    validateCoreScenarioMatrix: (matrix: Array<Record<string, unknown>>) => unknown;
  };
  const matrix = module.CORE_SCENARIO_ACCEPTANCE_MATRIX;
  const expectedIds = Array.from(
    { length: 9 },
    (_, index) => `CS-${String(index + 1).padStart(2, "0")}`,
  );

  expect(matrix.map(item => item.scenario_id)).toEqual(expectedIds);
  expect(matrix.map(item => item.support_level)).toEqual([
    "supported",
    "degraded",
    "degraded",
    "degraded",
    "supported",
    "degraded",
    "degraded",
    "degraded",
    "supported",
  ]);

  for (const scenario of matrix) {
    for (const field of [
      "title",
      "actor",
      "trigger",
      "public_entrypoints",
      "formal_artifacts",
      "state_readiness",
      "success",
      "failure_or_degraded",
      "recovery",
      "verification_fixtures",
      "evidence_refs",
      "owning_gates",
      "deferred_capabilities",
    ]) {
      expect(scenario[field], `${String(scenario.scenario_id)} missing ${field}`).toBeDefined();
    }
    for (const evidenceRef of scenario.evidence_refs as string[]) {
      expect(existsSync(path.join(root, evidenceRef)), `missing evidence ref ${evidenceRef}`).toBe(true);
    }
  }

  for (const scenarioId of ["CS-03", "CS-04", "CS-07"]) {
    const scenario = matrix.find(item => item.scenario_id === scenarioId)!;
    const publicSurface = JSON.stringify(scenario.public_entrypoints);
    expect(publicSurface).not.toMatch(/\b(?:intent|owner|entrypoint|callers|callees|impact|provenance_summary)\b/);
    expect((scenario.deferred_capabilities as string[]).length).toBeGreaterThan(0);
  }

  const duplicateIds = matrix.map(item => ({ ...item }));
  duplicateIds[1].scenario_id = duplicateIds[0].scenario_id;
  expect(() => module.validateCoreScenarioMatrix(duplicateIds)).toThrow(/duplicate scenario_id/i);
  expect(() => module.validateCoreScenarioMatrix([
    { ...matrix[0], support_level: "unknown" },
    ...matrix.slice(1),
  ])).toThrow(/support_level/i);
  expect(() => module.validateCoreScenarioMatrix([
    { ...matrix[0], owning_gates: ["unknown_gate"] },
    ...matrix.slice(1),
  ])).toThrow(/owning_gates/i);
});

test("acceptance plan 不硬编码历史样本", async () => {
  const module = await import(pathToFileURL(matrixModulePath).href) as {
    createAcceptancePlan?: (input: Record<string, unknown>) => Record<string, unknown>;
  };

  expect(typeof module.createAcceptancePlan).toBe("function");
  if (!module.createAcceptancePlan)
    return;

  const input = {
    plan_id: "fixture-plan",
    primary_fixtures: ["fixture-a"],
    required_gates: ["artifact_validity"],
    required_guards: [],
    thresholds: {
      sample_minimum: 1,
    },
    report_only: false,
  };
  const plan = module.createAcceptancePlan(input);

  expect(plan).toMatchObject(input);
  expect(plan.primary_fixtures).not.toContain("storybook");
  expect(plan.primary_fixtures).not.toContain("dagger");
  expect(Object.isFrozen(plan)).toBe(true);
  expect(Object.isFrozen(plan.primary_fixtures)).toBe(true);
  expect(Object.isFrozen(plan.thresholds)).toBe(true);

  expect(() => module.createAcceptancePlan!({
    ...input,
    required_gates: ["unknown_gate"],
  })).toThrow(/unknown gate/i);
  expect(() => module.createAcceptancePlan!({
    ...input,
    required_primary_gates: ["artifact_validity"],
  })).toThrow(/primary gate/i);
  expect(() => module.createAcceptancePlan!({
    ...input,
    required_primary_gates: ["reference_fidelity_primary"],
    required_gates: [],
  })).toThrow(/companion gate/i);
  expect(() => module.createAcceptancePlan!({
    ...input,
    required_guards: ["core_scenario_primary"],
  })).toThrow(/baseline guard/i);
  expect(() => module.createAcceptancePlan!({
    ...input,
    primary_fixtures: [],
  })).toThrow(/primary_fixtures/i);
});
