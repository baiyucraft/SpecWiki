import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

import { CORE_SCENARIO_ACCEPTANCE_MATRIX } from "../testing/core-scenario-acceptance.mjs";

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..");
const paths = {
  core: ".wiki/06-设计文档/03-核心场景.md",
  tests: ".wiki/02-开发指南/01-测试与验收.md",
  scripts: ".wiki/02-开发指南/02-脚本与工作流.md",
  workflow: ".wiki/05-规格基线/capabilities/workflow-verification/spec.md",
};

function read(relativePath: string) {
  return readFileSync(path.join(rootDir, relativePath), "utf8");
}

test("wiki 与 capability 只发布当前场景矩阵和 gate v2 authority", () => {
  const corePage = read(paths.core);
  const testGuide = read(paths.tests);
  const scriptGuide = read(paths.scripts);
  const workflowSpec = read(paths.workflow);
  const currentAuthority = [corePage, testGuide, scriptGuide, workflowSpec].join("\n");

  for (const scenario of CORE_SCENARIO_ACCEPTANCE_MATRIX) {
    expect(corePage).toContain(scenario.scenario_id);
    expect(corePage).toContain(`| ${scenario.scenario_id} | \`${scenario.support_level}\``);
  }
  expect(corePage).not.toContain("核心场景草案");
  expect(corePage).toContain("route_groups");
  expect(corePage).toContain("answer");

  for (const document of [testGuide, scriptGuide, workflowSpec]) {
    expect(document).toContain("knowledge-quality-gates.v2");
  }
  expect(testGuide).toContain("pass=0");
  expect(testGuide).toContain("blocker=1");
  expect(testGuide).toContain("incomplete/diagnostic=2");
  expect(testGuide).toContain("report-only");
  expect(scriptGuide).toContain("run-core-scenario-acceptance.mjs");
  expect(scriptGuide).toContain("reference_fidelity_primary");
  expect(scriptGuide).toContain("baseline_guard");
  expect(workflowSpec).toContain("required gates");
  expect(workflowSpec).toContain("not_covered");
  expect(workflowSpec).toContain("single owner");

  for (const legacy of [
    "`provenance_summary`",
    "`matched_symbols`",
    "`matched_sources`",
    "`matched_modules`",
    "storybook + dagger",
    "统一 query intents",
  ]) {
    expect(currentAuthority).not.toContain(legacy);
  }
});
