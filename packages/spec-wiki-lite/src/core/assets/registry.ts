import type { WikiLanguage } from "../config.js";

export type AssetOwnership = "managed" | "scaffold" | "skill";

export type ProjectAsset = {
  source: string;
  target: string;
  ownership: AssetOwnership;
};

export const PROJECT_SKILL_NAMES = [
  "wiki-continue",
  "wiki-explore",
  "wiki-propose",
  "wiki-design",
  "wiki-plan",
  "wiki-apply",
  "wiki-review",
  "wiki-archive",
] as const;

export type ProjectSkillName = (typeof PROJECT_SKILL_NAMES)[number];

const PROJECT_SKILL_REFERENCES: Readonly<Record<ProjectSkillName, readonly string[]>> = {
  "wiki-continue": [],
  "wiki-explore": ["research-template.md", "aoci-explore.md", "codegraph-explore.md"],
  "wiki-propose": ["research-template.md", "proposal-template.md"],
  "wiki-design": ["research-template.md", "design-template.md", "aoci-design.md", "codegraph-design.md"],
  "wiki-plan": [
    "system-tests-template.md",
    "unit-tests-template.md",
    "tasks-template.md",
    "browser-automation.md",
  ],
  "wiki-apply": [],
  "wiki-review": [
    "review-report-template.md",
    "test-report-template.md",
    "review-standard.md",
    "review-standard.frontend.md",
    "review-standard.go.md",
    "review-standard.java.md",
    "review-standard.python.md",
  ],
  "wiki-archive": [],
};

type WikiAssetDefinition = {
  key: string;
  ownership: Exclude<AssetOwnership, "skill">;
  paths: Record<WikiLanguage, string>;
};

const WIKI_ASSET_DEFINITIONS: readonly WikiAssetDefinition[] = [
  { key: "root", ownership: "scaffold", paths: { zh: "INDEX.md", en: "INDEX.md" } },
  {
    key: "conventions-index",
    ownership: "scaffold",
    paths: { zh: "00-文档约定/INDEX.md", en: "00-documentation-conventions/INDEX.md" },
  },
  {
    key: "boundaries-ssot",
    ownership: "managed",
    paths: { zh: "00-文档约定/00-边界与SSOT规则.md", en: "00-documentation-conventions/00-boundaries-and-ssot.md" },
  },
  {
    key: "page-template",
    ownership: "managed",
    paths: { zh: "00-文档约定/01-页面模板.md", en: "00-documentation-conventions/01-page-template.md" },
  },
  {
    key: "workflow",
    ownership: "managed",
    paths: { zh: "00-文档约定/02-SpecWiki-Lite工作流.md", en: "00-documentation-conventions/02-spec-wiki-lite-workflow.md" },
  },
  {
    key: "quick-start",
    ownership: "scaffold",
    paths: { zh: "01-快速上手/INDEX.md", en: "01-quick-start/INDEX.md" },
  },
  {
    key: "development-guide",
    ownership: "scaffold",
    paths: { zh: "02-开发指南/INDEX.md", en: "02-development-guide/INDEX.md" },
  },
  {
    key: "code-commenting",
    ownership: "scaffold",
    paths: { zh: "02-开发指南/00-代码注释规范.md", en: "02-development-guide/00-code-commenting.md" },
  },
  {
    key: "module-guide",
    ownership: "scaffold",
    paths: { zh: "03-模块指南/INDEX.md", en: "03-module-guide/INDEX.md" },
  },
  {
    key: "public-interfaces",
    ownership: "scaffold",
    paths: { zh: "04-对外方法/INDEX.md", en: "04-public-interfaces/INDEX.md" },
  },
] as const;

export type LocalizedWikiAsset = ProjectAsset & {
  key: string;
  language: WikiLanguage;
};

export function projectWikiAssetsForLanguage(language: WikiLanguage): LocalizedWikiAsset[] {
  return WIKI_ASSET_DEFINITIONS.map(definition => ({
    key: definition.key,
    language,
    ownership: definition.ownership,
    source: `wiki/${language}/${definition.paths[language]}`,
    target: `.wiki/${definition.paths[language]}`,
  }));
}

export function projectSkillAssetsForLanguage(language: WikiLanguage): ProjectAsset[] {
  return PROJECT_SKILL_NAMES.flatMap((name) => {
    const base = `skills/${language}/${name}`;
    const target = `.agents/skills/${name}`;
    return [
      { source: `${base}/SKILL.md`, target: `${target}/SKILL.md`, ownership: "skill" as const },
      ...PROJECT_SKILL_REFERENCES[name].map(reference => ({
        source: `${base}/references/${reference}`,
        target: `${target}/references/${reference}`,
        ownership: "skill" as const,
      })),
    ];
  });
}

export function projectSkillAssetsByNameForLanguage(
  language: WikiLanguage,
): Map<ProjectSkillName, ProjectAsset[]> {
  const assets = projectSkillAssetsForLanguage(language);
  return new Map(PROJECT_SKILL_NAMES.map(name => [
    name,
    assets.filter(asset => asset.target.startsWith(`.agents/skills/${name}/`)),
  ]));
}

export function projectAssetsForLanguage(language: WikiLanguage): ProjectAsset[] {
  return [...projectWikiAssetsForLanguage(language), ...projectSkillAssetsForLanguage(language)];
}

const LEGACY_EN_V0_PATHS = [
  "INDEX.md",
  "00-conventions/INDEX.md",
  "00-conventions/00-page-template.md",
  "01-project/INDEX.md",
  "01-project/00-overview.md",
  "02-development/INDEX.md",
  "02-development/00-getting-started.md",
  "02-development/01-testing.md",
  "03-architecture/INDEX.md",
  "03-architecture/00-system-overview.md",
  "04-reference/INDEX.md",
] as const;

export const LEGACY_EN_V0_ASSETS: readonly ProjectAsset[] = LEGACY_EN_V0_PATHS.map(wikiPath => ({
  source: `migrations/wiki-en-v0/${wikiPath}`,
  target: `.wiki/${wikiPath}`,
  ownership: wikiPath === "00-conventions/INDEX.md" ? "managed" : "scaffold",
}));

export const PROJECT_DIRECTORIES = [
  ".wiki",
  ".spec/changes",
  ".spec/archive",
  ".agents/skills",
] as const;
