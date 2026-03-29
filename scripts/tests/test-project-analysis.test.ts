import { expect, test } from "vitest";

import { collectProject } from "../collect-test-project-analysis.mjs";

test("storybook 当前共享 fixture 停在 research 阶段且尚未进入 compose", () => {
  const result = collectProject("storybook");

  expect(result.runtimeState).toBe("runtime_incomplete");
  expect(result.incompleteReason).toContain("仍在 research");
  expect(result.knowledgeMetrics.knowledgeUnitCount).toBeGreaterThan(0);
  expect(result.knowledgeMetrics.unitResearchRows).toBeGreaterThan(0);
  expect(result.knowledgeMetrics.sectionPlanUnits).toBeGreaterThan(0);
  expect(result.knowledgeMetrics.pageDraftCount).toBe(0);
  expect(result.knowledgeMetrics.wikiPageCount).toBe(0);
  expect(result.runtimeSnapshot.markdownPageCount).toBe(0);
});

test("dagger 当前共享 fixture 已进入 ready，且保留 compose 与 assemble 产物", () => {
  const result = collectProject("dagger");

  expect(result.runtimeState).toBe("ready");
  expect(result.incompleteReason).toBeNull();
  expect(result.knowledgeMetrics.knowledgeUnitCount).toBeGreaterThan(0);
  expect(result.knowledgeMetrics.unitResearchRows).toBeGreaterThan(0);
  expect(result.knowledgeMetrics.sectionPlanUnits).toBeGreaterThan(0);
  expect(result.knowledgeMetrics.pageDraftCount).toBeGreaterThan(0);
  expect(result.knowledgeMetrics.wikiPageCount).toBeGreaterThan(0);
  expect(result.runtimeSnapshot.markdownPageCount).toBeGreaterThan(0);
  expect(result.knowledgeMetrics.composeCoverage).toBe(1);
  expect(result.knowledgeMetrics.assembleCoverage).toBe(1);
});
