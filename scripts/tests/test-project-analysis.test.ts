import { expect, test } from "vitest";

import { collectProject } from "../collect-test-project-analysis.mjs";

test("storybook 使用 2.0 数据面统计 research/compose/assemble，不再误报为 0", () => {
  const result = collectProject("storybook");

  expect(result.runtimeState).toBe("ready");
  expect(result.knowledgeMetrics.knowledgeUnitCount).toBeGreaterThan(0);
  expect(result.knowledgeMetrics.unitResearchRows).toBeGreaterThan(0);
  expect(result.knowledgeMetrics.sectionPlanUnits).toBeGreaterThan(0);
  expect(result.knowledgeMetrics.pageDraftCount).toBeGreaterThan(0);
  expect(result.knowledgeMetrics.wikiPageCount).toBeGreaterThan(0);
});

test("dagger runtime incomplete 时脚本给出诊断而不是直接崩溃", () => {
  const result = collectProject("dagger");

  expect(result.runtimeState).toBe("runtime_incomplete");
  expect(result.incompleteReason).toBeTruthy();
  expect(result.knowledgeMetrics.knowledgeUnitCount).toBeGreaterThan(0);
  expect(result.knowledgeMetrics.unitResearchRows).toBeGreaterThan(0);
  expect(result.runtimeSnapshot.markdownPageCount).toBe(0);
});
