import { expect, test } from "vitest";

import {
  PROJECT_SKILL_NAMES,
  projectSkillAssetsForLanguage,
  projectSkillAssetsByNameForLanguage,
} from "./registry.js";

const expectedReferenceCounts: Record<(typeof PROJECT_SKILL_NAMES)[number], number> = {
  "wiki-continue": 0,
  "wiki-explore": 1,
  "wiki-propose": 2,
  "wiki-design": 2,
  "wiki-plan": 4,
  "wiki-apply": 0,
  "wiki-review": 7,
  "wiki-archive": 0,
};

test.each(["zh", "en"] as const)("registers the complete %s Skill inventory", (language) => {
  const assets = projectSkillAssetsForLanguage(language);
  const byName = projectSkillAssetsByNameForLanguage(language);

  expect(assets).toHaveLength(24);
  expect([...byName.keys()]).toEqual(PROJECT_SKILL_NAMES);
  expect(new Set(assets.map(asset => asset.target)).size).toBe(24);
  expect(assets.every(asset => asset.ownership === "skill")).toBe(true);
  expect(assets.every(asset => asset.source.startsWith(`skills/${language}/`))).toBe(true);

  for (const name of PROJECT_SKILL_NAMES) {
    const skillAssets = byName.get(name)!;
    expect(skillAssets).toHaveLength(1 + expectedReferenceCounts[name]);
    expect(skillAssets[0]?.target).toBe(`.agents/skills/${name}/SKILL.md`);
    expect(skillAssets.filter(asset => asset.target.includes("/references/"))).toHaveLength(expectedReferenceCounts[name]);
  }
});

test("keeps stable installed paths while language changes package sources", () => {
  const zh = projectSkillAssetsForLanguage("zh");
  const en = projectSkillAssetsForLanguage("en");

  expect(zh.map(asset => asset.target)).toEqual(en.map(asset => asset.target));
  expect(zh.map(asset => asset.source)).not.toEqual(en.map(asset => asset.source));
});
