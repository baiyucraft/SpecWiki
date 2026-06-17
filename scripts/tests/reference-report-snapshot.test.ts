import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, rmSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..", "..");
const scriptPath = path.join(rootDir, "scripts", "collect-reference-project-reports.mjs");
const changesDir = path.join(rootDir, ".spec", "changes");

function runReport(changeName: string, extraArgs: string[] = []) {
  const changeDir = path.join(changesDir, changeName);
  rmSync(changeDir, { recursive: true, force: true, maxRetries: 5, retryDelay: 50 });

  const output = execFileSync(
    "node",
    [
      scriptPath,
      "--skip-init",
      "storybook",
      "dagger",
      "--change",
      changeName,
      ...extraArgs,
    ],
    {
      cwd: rootDir,
      encoding: "utf-8",
      timeout: 120_000,
    },
  );

  const parsed = JSON.parse(output.slice(output.indexOf("{")));
  return {
    parsed,
    changeDir,
    reportDir: parsed.reportDir,
    summaryPath: parsed.summaryPath,
    snapshotPath: parsed.snapshotPath,
    stabilityPath: path.join(parsed.reportDir, "_stability.md"),
  };
}

test("reference report snapshot、summary 与项目结果来自同一批 results", () => {
  const changeName = "__vitest-reference-report-snapshot";
  const run = runReport(changeName);

  try {
    expect(existsSync(run.snapshotPath)).toBe(true);
    expect(existsSync(run.summaryPath)).toBe(true);

    const snapshot = JSON.parse(readFileSync(run.snapshotPath, "utf-8"));
    const summary = readFileSync(run.summaryPath, "utf-8");
    const storybookReport = readFileSync(path.join(run.reportDir, "storybook.md"), "utf-8");
    const daggerReport = readFileSync(path.join(run.reportDir, "dagger.md"), "utf-8");
    const storybookLedger = readFileSync(path.join(run.reportDir, "storybook-gap-ledger.md"), "utf-8");
    const storybook = snapshot.results.find((item: { project: string }) => item.project === "storybook");
    const dagger = snapshot.results.find((item: { project: string }) => item.project === "dagger");

    expect(snapshot.results).toHaveLength(2);
    expect(snapshot.primary_gate_summary).toBeTruthy();
    expect(snapshot.primary_gate_summary.gate_level).toBe("primary_gate");
    expect(snapshot.primary_gate_summary.gate_scope).toBe("reference_fidelity");
    expect(snapshot.primary_gate_summary.fidelity_input_only).toBe(true);
    expect(snapshot.primary_gate_summary.required_companion_gates).toContain("artifact_validity");
    expect(snapshot.primary_gate_summary.formal_gates.artifact_validity.decision).toBe("blocker");
    expect(storybook?.status).toBe("runtime_incomplete");
    expect(dagger?.status).toBe("runtime_incomplete");
    expect(storybook?.runtime_metrics?.runtime_state).toBe("missing");
    expect(dagger?.runtime_metrics?.runtime_state).toBe("missing");
    expect(storybook?.runtime_metrics?.incomplete_reason).toContain("未发现 `.wiki` runtime 产物");
    expect(dagger?.runtime_metrics?.incomplete_reason).toContain("未发现 `.wiki` runtime 产物");
    expect(storybook?.fidelity_metrics).toBeNull();
    expect(dagger?.fidelity_metrics).toBeNull();
    expect(storybook?.generatedPageCount).toBe(0);
    expect(dagger?.generatedPageCount).toBe(0);
    expect(summary).toContain("| storybook |");
    expect(summary).toContain("| dagger |");
    expect(summary).toContain("- storybook：blocker，runtime_state=missing");
    expect(summary).toContain("- dagger：blocker，runtime_state=missing");
    expect(storybookReport).toContain("- status：runtime_incomplete");
    expect(storybookReport).toContain("runtime 仍处于 missing");
    expect(daggerReport).toContain("- status：runtime_incomplete");
    expect(daggerReport).toContain("runtime 仍处于 missing");
    expect(storybookLedger).toContain("- status：runtime_incomplete");
    expect(run.parsed.primaryGateSummary.decision).toBe("blocker");
    expect(run.parsed.primaryGateSummary.fidelity_input_only).toBe(true);
  } finally {
    rmSync(run.changeDir, { recursive: true, force: true, maxRetries: 5, retryDelay: 50 });
  }
});

test("warm stability 会落盘并反映当前共享 fixture 的 mixed runtime 样本", () => {
  const changeName = "__vitest-reference-report-stability";
  const run = runReport(changeName, ["--warm-reruns", "2"]);

  try {
    expect(existsSync(run.stabilityPath)).toBe(true);

    const snapshot = JSON.parse(readFileSync(run.snapshotPath, "utf-8"));
    const stability = readFileSync(run.stabilityPath, "utf-8");
    const storybook = snapshot.results.find((item: { project: string }) => item.project === "storybook");
    const dagger = snapshot.results.find((item: { project: string }) => item.project === "dagger");

    expect(storybook?.stability).toBeTruthy();
    expect(storybook?.stability?.stable).toBe(false);
    expect(dagger?.stability).toBeTruthy();
    expect(dagger?.stability?.stable).toBe(false);
    expect(stability).toContain("| storybook |");
    expect(stability).toContain("| dagger |");
    expect(stability).toContain("| storybook | no |");
    expect(stability).toContain("| dagger | no |");
  } finally {
    rmSync(run.changeDir, { recursive: true, force: true, maxRetries: 5, retryDelay: 50 });
  }
});
