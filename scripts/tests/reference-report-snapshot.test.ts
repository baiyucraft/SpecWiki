import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, rmSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..", "..");
const scriptPath = path.join(rootDir, "scripts", "collect-reference-project-reports.mjs");
const changesDir = path.join(rootDir, "openspec", "changes");

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
    const storybookLedger = readFileSync(path.join(run.reportDir, "storybook-gap-ledger.md"), "utf-8");
    const storybook = snapshot.results.find((item: { project: string }) => item.project === "storybook");
    const dagger = snapshot.results.find((item: { project: string }) => item.project === "dagger");

    expect(snapshot.results).toHaveLength(2);
    expect(storybook?.status).toBe("ready");
    expect(storybook?.run_metrics?.baseline_mode).toBe("warm_runtime_reuse");
    expect(dagger?.run_metrics?.baseline_mode).toBe("warm_runtime_reuse");
    expect(dagger?.fidelity_metrics === null).toBe(dagger?.status !== "ready");
    expect(summary).toContain("| storybook |");
    expect(summary).toContain("| dagger |");
    expect(storybookReport).toContain("- baseline_mode：warm_runtime_reuse");
    expect(storybookLedger).toContain("- baseline_mode：warm_runtime_reuse");
  } finally {
    rmSync(run.changeDir, { recursive: true, force: true, maxRetries: 5, retryDelay: 50 });
  }
});

test("warm stability 会落盘并区分 stable 与 runtime_incomplete", () => {
  const changeName = "__vitest-reference-report-stability";
  const run = runReport(changeName, ["--warm-reruns", "2"]);

  try {
    expect(existsSync(run.stabilityPath)).toBe(true);

    const snapshot = JSON.parse(readFileSync(run.snapshotPath, "utf-8"));
    const stability = readFileSync(run.stabilityPath, "utf-8");
    const storybook = snapshot.results.find((item: { project: string }) => item.project === "storybook");
    const dagger = snapshot.results.find((item: { project: string }) => item.project === "dagger");

    expect(storybook?.stability).toBeTruthy();
    expect(dagger?.stability).toBeTruthy();
    expect(storybook?.run_metrics?.baseline_mode).toBe("warm_runtime_reuse");
    expect(dagger?.run_metrics?.baseline_mode).toBe("warm_runtime_reuse");
    expect(stability).toContain("| storybook |");
    expect(stability).toContain("| dagger |");
  } finally {
    rmSync(run.changeDir, { recursive: true, force: true, maxRetries: 5, retryDelay: 50 });
  }
});
