import { expect, test } from "vitest";

import { collectProject } from "../collect-test-project-analysis.mjs";

test("storybook 2.0 数据面停在 research 阶段，但保留 planning 与 research 计数", () => {
  const storybook = collectProject("storybook");

  expect(storybook.runtimeState).toBe("runtime_incomplete");
  expect(storybook.incompleteReason).toContain("仍在 research");
  expect(storybook.planning.knowledgeUnitCount).toBeGreaterThan(0);
  expect(storybook.planning.unitTypeBreakdown.length).toBeGreaterThan(0);
  expect(storybook.research.unitResearchCount).toBeGreaterThan(0);
  expect(storybook.research.unitSectionPlanCount).toBeGreaterThan(0);
  expect(storybook.compose.pageDigestCount).toBe(0);
  expect(storybook.compose.pageDraftCount).toBe(0);
  expect(storybook.assemble.wikiPageCount).toBe(0);
  expect(storybook.pages.pageCount).toBe(0);
});

test("dagger 2.0 数据面已经 ready，并落出 compose/assemble 结果", () => {
  const dagger = collectProject("dagger");

  expect(dagger.runtimeState).toBe("ready");
  expect(dagger.incompleteReason).toBeNull();
  expect(dagger.runtime.dbCounts.knowledge_units).toBeGreaterThan(0);
  expect(dagger.planning.unitTypeBreakdown.length).toBeGreaterThan(0);
  expect(dagger.research.unitResearchCount).toBeGreaterThan(0);
  expect(dagger.research.unitSectionPlanCount).toBeGreaterThan(0);
  expect(dagger.compose.pageDigestCount).toBeGreaterThan(0);
  expect(dagger.compose.pageDraftCount).toBeGreaterThan(0);
  expect(dagger.assemble.wikiPageCount).toBeGreaterThan(0);
  expect(dagger.pages.pageCount).toBeGreaterThan(0);
});
