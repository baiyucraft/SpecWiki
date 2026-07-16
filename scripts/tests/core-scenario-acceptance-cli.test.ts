import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..");
const scriptPath = path.join(rootDir, "scripts", "run-core-scenario-acceptance.mjs");

function scenarioResults() {
  return Array.from({ length: 9 }, (_, index) => ({
    scenario_id: `CS-${String(index + 1).padStart(2, "0")}`,
    decision: "pass",
    evidence_refs: [`fixture/CS-${String(index + 1).padStart(2, "0")}.json`],
  }));
}

function fixtureFor(decision: "pass" | "blocker" | "incomplete" | "diagnostic") {
  const requiredGates = decision === "diagnostic" ? [] : ["artifact_validity"];
  const failures = decision === "blocker"
    ? [{
        failure_id: "F-CLI-001",
        owner_gate_id: "artifact_validity",
        source_ref: "fixture-a",
        assertion_ref: "artifact contract failed",
        evidence_refs: ["fixture/artifact.json"],
      }]
    : [];
  const gateResults = decision === "pass"
    ? {
        artifact_validity: {
          decision: "pass",
          evidence_refs: ["fixture/artifact.json"],
          failure_refs: [],
        },
      }
    : decision === "blocker"
      ? {
          artifact_validity: {
            decision: "blocker",
            evidence_refs: ["fixture/artifact.json"],
            failure_refs: ["F-CLI-001"],
          },
        }
      : {};

  return {
    plan: {
      plan_id: `fixture-${decision}`,
      primary_fixtures: ["fixture-a"],
      required_primary_gates: [],
      required_gates: requiredGates,
      required_guards: [],
      report_only: false,
    },
    gate_results: gateResults,
    failures,
    diagnostics: decision === "diagnostic"
      ? [{
          diagnostic_id: "D-CLI-001",
          state: "runtime_incomplete",
          reason: "diagnostic-only fixture",
          recommended_action: "review",
          evidence_refs: [],
        }]
      : [],
    scenario_results: scenarioResults(),
  };
}

test("orchestrator 的进程退出码与 summary decision 一致", () => {
  expect(existsSync(scriptPath)).toBe(true);
  if (!existsSync(scriptPath))
    return;

  const tempDir = mkdtempSync(path.join(os.tmpdir(), "core-scenario-acceptance-"));
  try {
    for (const [decision, expectedExit] of [
      ["pass", 0],
      ["blocker", 1],
      ["incomplete", 2],
      ["diagnostic", 2],
    ] as const) {
      const fixturePath = path.join(tempDir, `${decision}.json`);
      const fixture = fixtureFor(decision);
      fixture.scenario_results.reverse();
      writeFileSync(fixturePath, JSON.stringify(fixture));
      const run = spawnSync(process.execPath, [scriptPath, "--fixture-plan", fixturePath, "--json"], {
        cwd: rootDir,
        encoding: "utf8",
      });
      expect(run.stderr).toBe("");
      const summary = JSON.parse(run.stdout);
      expect(summary.decision).toBe(decision);
      expect(summary.scenario_results).toHaveLength(9);
      expect(summary.scenario_results.map((result: { scenario_id: string }) => result.scenario_id))
        .toEqual(Array.from({ length: 9 }, (_, index) => `CS-${String(index + 1).padStart(2, "0")}`));
      expect(run.status).toBe(expectedExit);
      expect(summary.exit_code).toBe(expectedExit);

      const repeated = spawnSync(
        process.execPath,
        [scriptPath, "--fixture-plan", fixturePath, "--json"],
        { cwd: rootDir, encoding: "utf8" },
      );
      expect(repeated.stdout).toBe(run.stdout);

      if (decision !== "pass") {
        const reportOnly = spawnSync(
          process.execPath,
          [scriptPath, "--fixture-plan", fixturePath, "--json", "--report-only"],
          { cwd: rootDir, encoding: "utf8" },
        );
        const reportSummary = JSON.parse(reportOnly.stdout);
        expect(reportOnly.status).toBe(0);
        expect(reportSummary.decision).toBe(decision);
        expect(reportSummary.exit_code).toBe(0);
      }
    }
  } finally {
    rmSync(tempDir, { recursive: true, force: true });
  }
});

test.each([
  ["blocker", "blocker", 1],
  ["incomplete", "incomplete", 2],
  ["diagnostic", "incomplete", 2],
] as const)("场景级 %s 不会被 pass gate 掩盖", (scenarioDecision, expectedDecision, expectedExit) => {
  const fixture = fixtureFor("pass");
  fixture.scenario_results[0].decision = scenarioDecision;

  const tempDir = mkdtempSync(path.join(os.tmpdir(), "core-scenario-decision-"));
  try {
    const fixturePath = path.join(tempDir, `${scenarioDecision}.json`);
    writeFileSync(fixturePath, JSON.stringify(fixture));
    const run = spawnSync(process.execPath, [scriptPath, "--fixture-plan", fixturePath, "--json"], {
      cwd: rootDir,
      encoding: "utf8",
    });
    expect(run.stderr).toBe("");
    const summary = JSON.parse(run.stdout);
    expect(summary.decision).toBe(expectedDecision);
    expect(summary.exit_code).toBe(expectedExit);
    expect(run.status).toBe(expectedExit);
  } finally {
    rmSync(tempDir, { recursive: true, force: true });
  }
});
