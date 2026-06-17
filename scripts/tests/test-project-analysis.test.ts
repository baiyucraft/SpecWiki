import { expect, test } from "vitest";

import { collectProject } from "../collect-test-project-analysis.mjs";

test("storybook 当前共享 fixture 停在 research 阶段且尚未进入 compose", () => {
  const result = collectProject("storybook");

  expect(result.runtimeState).toBe("missing");
  expect(result.incompleteReason).toContain("未发现 `.wiki` runtime 产物");
  expect(result.knowledgeMetrics.knowledgeUnitCount).toBe(0);
  expect(result.knowledgeMetrics.unitResearchRows).toBe(0);
  expect(result.knowledgeMetrics.sectionPlanUnits).toBe(0);
  expect(result.knowledgeMetrics.pageDraftCount).toBe(0);
  expect(result.knowledgeMetrics.wikiPageCount).toBe(0);
  expect(result.runtimeSnapshot.markdownPageCount).toBe(0);
});

test("dagger 当前共享 fixture 未装配时保持 missing 诊断", () => {
  const result = collectProject("dagger");

  expect(result.runtimeState).toBe("missing");
  expect(result.incompleteReason).toContain("未发现 `.wiki` runtime 产物");
  expect(result.knowledgeMetrics.knowledgeUnitCount).toBe(0);
  expect(result.knowledgeMetrics.unitResearchRows).toBe(0);
  expect(result.knowledgeMetrics.sectionPlanUnits).toBe(0);
  expect(result.knowledgeMetrics.pageDraftCount).toBe(0);
  expect(result.knowledgeMetrics.wikiPageCount).toBe(0);
  expect(result.runtimeSnapshot.markdownPageCount).toBe(0);
  expect(result.knowledgeMetrics.composeCoverage).toBe(0);
  expect(result.knowledgeMetrics.assembleCoverage).toBe(0);
});
