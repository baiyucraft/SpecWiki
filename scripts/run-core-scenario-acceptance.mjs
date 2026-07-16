/**
 * 核心场景验收的离线编排入口。
 *
 * adapter 或测试 fixture 负责产出 evidence/failures；本脚本只校验 plan 与 9/9 场景身份，
 * 再交给共享 Gate Kernel 生成唯一 decision 和进程退出码。
 */

import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

import {
  CORE_SCENARIO_ACCEPTANCE_MATRIX,
  createAcceptancePlan,
  validateCoreScenarioMatrix,
} from "./testing/core-scenario-acceptance.mjs";
import { aggregateGateResults } from "./testing/quality-gates.mjs";

function parseCliArgs(argv) {
  let fixturePlan;
  let reportOnly = false;
  for (let index = 0; index < argv.length; index++) {
    const arg = argv[index];
    if (arg === "--fixture-plan") {
      fixturePlan = argv[index + 1];
      index++;
      continue;
    }
    if (arg === "--report-only") {
      reportOnly = true;
      continue;
    }
    if (arg === "--json")
      continue;
    throw new TypeError(`unknown argument: ${arg}`);
  }
  if (!fixturePlan)
    throw new TypeError("--fixture-plan is required");
  return { fixturePlan, reportOnly };
}

function validateScenarioResults(results) {
  if (!Array.isArray(results) || results.length !== CORE_SCENARIO_ACCEPTANCE_MATRIX.length)
    throw new TypeError("scenario_results must contain exactly 9 records");
  const expectedIds = CORE_SCENARIO_ACCEPTANCE_MATRIX.map(scenario => scenario.scenario_id);
  const actualIds = results.map(result => result?.scenario_id);
  if (new Set(actualIds).size !== actualIds.length
    || actualIds.some(scenarioId => !expectedIds.includes(scenarioId))) {
    throw new TypeError("scenario_results must use the canonical CS-01..CS-09 identities");
  }
  for (const result of results) {
    if (!["pass", "blocker", "incomplete", "diagnostic"].includes(result.decision))
      throw new TypeError(`invalid scenario decision for ${result.scenario_id}`);
    if (!Array.isArray(result.evidence_refs)
      || result.evidence_refs.length === 0
      || result.evidence_refs.some(ref => typeof ref !== "string" || ref.trim() === "")) {
      throw new TypeError(`${result.scenario_id} requires non-empty evidence_refs`);
    }
  }
  return results;
}

export function runCoreScenarioAcceptance(input, options = {}) {
  validateCoreScenarioMatrix(CORE_SCENARIO_ACCEPTANCE_MATRIX);
  if (!input || typeof input !== "object")
    throw new TypeError("acceptance fixture must be an object");
  const plan = createAcceptancePlan({
    ...input.plan,
    report_only: options.reportOnly === true || input.plan?.report_only === true,
  });
  return aggregateGateResults({
    plan,
    gate_results: input.gate_results ?? {},
    failures: input.failures ?? [],
    diagnostics: input.diagnostics ?? [],
    scenario_results: validateScenarioResults(input.scenario_results),
  });
}

const entryHref = process.argv[1] ? pathToFileURL(process.argv[1]).href : null;
if (entryHref && import.meta.url === entryHref) {
  const args = parseCliArgs(process.argv.slice(2));
  const fixture = JSON.parse(readFileSync(args.fixturePlan, "utf8"));
  const summary = runCoreScenarioAcceptance(fixture, { reportOnly: args.reportOnly });
  process.stdout.write(`${JSON.stringify(summary)}\n`);
  process.exitCode = summary.exit_code;
}
