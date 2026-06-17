import { expect, test } from "vitest";

import { collectProject } from "../collect-test-project-analysis.mjs";

test("storybook 2.0 数据面停在 research 阶段，但保留 planning 与 research 计数", () => {
  const storybook = collectProject("storybook");

  expect(storybook.runtimeState).toBe("missing");
  expect(storybook.incompleteReason).toContain("未发现 `.wiki` runtime 产物");
  expect(storybook.planning.knowledgeUnitCount).toBe(0);
  expect(storybook.planning.unitTypeBreakdown.length).toBe(0);
  expect(storybook.research.unitResearchCount).toBe(0);
  expect(storybook.research.unitSectionPlanCount).toBe(0);
  expect(storybook.compose.pageDigestCount).toBe(0);
  expect(storybook.compose.pageDraftCount).toBe(0);
  expect(storybook.assemble.wikiPageCount).toBe(0);
  expect(storybook.pages.pageCount).toBe(0);
});

test("dagger 2.0 数据面缺少共享 fixture 时保持空诊断", () => {
  const dagger = collectProject("dagger");

  expect(dagger.runtimeState).toBe("missing");
  expect(dagger.incompleteReason).toContain("未发现 `.wiki` runtime 产物");
  expect(dagger.runtime.dbCounts.knowledge_units).toBe(0);
  expect(dagger.planning.unitTypeBreakdown.length).toBe(0);
  expect(dagger.research.unitResearchCount).toBe(0);
  expect(dagger.research.unitSectionPlanCount).toBe(0);
  expect(dagger.compose.pageDigestCount).toBe(0);
  expect(dagger.compose.pageDraftCount).toBe(0);
  expect(dagger.assemble.wikiPageCount).toBe(0);
  expect(dagger.pages.pageCount).toBe(0);
});
