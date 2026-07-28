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

const PROJECT_SCAFFOLD_WIKI_PATHS = [
  "INDEX.md",
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

export const PROJECT_ASSETS: readonly ProjectAsset[] = [
  ...PROJECT_SCAFFOLD_WIKI_PATHS.map(wikiPath => ({
    source: `wiki/${wikiPath}`,
    target: `.wiki/${wikiPath}`,
    ownership: "scaffold" as const,
  })),
  {
    source: "wiki/00-conventions/INDEX.md",
    target: ".wiki/00-conventions/INDEX.md",
    ownership: "managed",
  },
  ...PROJECT_SKILL_NAMES.map(name => ({
    source: `skills/${name}/SKILL.md`,
    target: `.agents/skills/${name}/SKILL.md`,
    ownership: "skill" as const,
  })),
];

export const PROJECT_DIRECTORIES = [
  ".wiki",
  ".spec/changes",
  ".spec/archive",
  ".agents/skills",
] as const;
