export type AssetOwnership = "managed" | "scaffold" | "skill";

export type ProjectAsset = {
  source: string;
  target: string;
  ownership: AssetOwnership;
};

const SKILL_NAMES = [
  "wiki-continue",
  "wiki-explore",
  "wiki-propose",
  "wiki-design",
  "wiki-plan",
  "wiki-apply",
  "wiki-review",
  "wiki-archive",
] as const;

export const PROJECT_ASSETS: readonly ProjectAsset[] = [
  {
    source: "wiki/INDEX.md",
    target: ".wiki/INDEX.md",
    ownership: "scaffold",
  },
  {
    source: "wiki/00-conventions/INDEX.md",
    target: ".wiki/00-conventions/INDEX.md",
    ownership: "managed",
  },
  ...SKILL_NAMES.map(name => ({
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
