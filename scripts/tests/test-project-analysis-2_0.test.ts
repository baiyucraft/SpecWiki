import { expect, test } from "vitest";

import { collectProject } from "../collect-test-project-analysis.mjs";

test("storybook 使用 2.0 runtime 数据面统计 planning/research/compose/assemble", () => {
  const storybook = collectProject("storybook");

  expect(storybook.runtimeState).toBe("ready");
  expect(storybook.planning.knowledgeUnitCount).toBeGreaterThan(0);
  expect(storybook.planning.unitTypeBreakdown.some((item: { name: string }) => item.name === "ConceptGuide")).toBe(true);
  expect(storybook.research.unitResearchCount).toBeGreaterThan(0);
  expect(storybook.research.unitSectionPlanCount).toBeGreaterThan(0);
  expect(storybook.compose.pageDigestCount).toBeGreaterThan(0);
  expect(storybook.compose.pageDraftCount).toBeGreaterThan(0);
  expect(storybook.assemble.wikiPageCount).toBeGreaterThan(0);
  expect(storybook.pages.pageCount).toBeGreaterThan(0);
});

test("dagger 的 runtime_incomplete 不会再被误判成 ready 或 research=0", () => {
  const dagger = collectProject("dagger");

  expect(dagger.runtimeState).toBe("runtime_incomplete");
  expect(dagger.runtime.dbCounts.knowledge_units).toBeGreaterThan(0);
  expect(dagger.research.unitResearchCount).toBeGreaterThan(0);
  expect(dagger.research.unitSectionPlanCount).toBeGreaterThan(0);
  expect(dagger.compose.pageDigestCount).toBe(0);
  expect(dagger.compose.pageDraftCount).toBe(0);
  expect(dagger.assemble.wikiPageCount).toBe(0);
  expect(dagger.pages.pageCount).toBe(0);
});
