import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..");
const designDirectory = path.join(root, ".wiki", "06-设计文档");
const baselinePath = path.join(designDirectory, "05-产品基线与设计治理.md");
const designIndexPath = path.join(designDirectory, "INDEX.md");
const overallDesignPath = path.join(designDirectory, "00-总体设计.md");
const baselineLink = "./05-产品基线与设计治理.md";
const artifactManifests = [
  { id: "wiki-model", file: path.join(root, "crates", "wiki-model", "Cargo.toml") },
  { id: "wiki-index", file: path.join(root, "crates", "wiki-index", "Cargo.toml") },
  { id: "wiki-knowledge", file: path.join(root, "crates", "wiki-knowledge", "Cargo.toml") },
  { id: "wiki-runtime", file: path.join(root, "crates", "wiki-runtime", "Cargo.toml") },
] as const;

function readText(file: string): string {
  return readFileSync(file, "utf8");
}

function collectMissingMarkers(content: string, context: string, markers: string[]): string[] {
  return markers
    .filter(marker => !content.includes(marker))
    .map(marker => `${context}: missing marker ${marker}`);
}

function readSection(markdown: string, heading: string): string {
  const start = markdown.indexOf(heading);
  if (start < 0)
    return "";
  const nextHeading = markdown.indexOf("\n## ", start + heading.length);
  return markdown.slice(start, nextHeading < 0 ? undefined : nextHeading);
}

function collectCanonicalBaselineRequirements(): string[] {
  const issues: string[] = [];
  if (!existsSync(baselinePath))
    issues.push(".wiki/06-设计文档/05-产品基线与设计治理.md: missing canonical baseline");

  const designIndex = readText(designIndexPath);
  if (!designIndex.includes(baselineLink))
    issues.push(".wiki/06-设计文档/INDEX.md: missing canonical baseline link");

  const overallDesign = readText(overallDesignPath);
  if (!overallDesign.includes(baselineLink))
    issues.push(".wiki/06-设计文档/00-总体设计.md: missing canonical baseline link");

  if (existsSync(baselinePath)) {
    const baseline = readText(baselinePath);
    issues.push(...collectMissingMarkers(baseline, "canonical baseline", [
      "# Repo Wiki 3.0 产品基线与设计治理",
      "## 基线身份",
      "## 产品范围",
      "## Authority 职责边界",
      "`05-产品基线与设计治理.md`",
      "`00-总体设计.md`",
    ]));
  }

  return issues;
}

function readCargoPackageVersion(file: string): string {
  const match = readText(file).match(/\[package\][\s\S]+?^version\s*=\s*"([^"]+)"/m);
  if (!match)
    throw new Error(`missing package version in ${path.relative(root, file)}`);
  return match[1];
}

function readArtifactVersionLabels(): string[] {
  return artifactManifests.map(({ id, file }) => `${id}@${readCargoPackageVersion(file)}`);
}

function collectVersionContractIssues(baseline: string): string[] {
  const npmManifest = JSON.parse(readText(path.join(root, "packages", "spec-wiki", "package.json"))) as { version: string };
  const issues = collectMissingMarkers(baseline, "version contract", [
    "## 版本域与 Authority",
    "| `architecture` |",
    "| `product-release` |",
    "| `artifact` |",
    "VersionRelation",
    "distributed-by",
    "只有 authority、scope 或显式 relation 冲突",
    "数值相等不自动建立映射",
    "数值不同不自动构成漂移",
    `spec-wiki@${npmManifest.version}`,
  ]);

  for (const label of readArtifactVersionLabels()) {
    if (!baseline.includes(label))
      issues.push(`version contract: missing manifest value ${label}`);
  }

  return issues;
}

function collectStatusModelIssues(baseline: string): string[] {
  const section = readSection(baseline, "## 设计决策与交付证据");
  return collectMissingMarkers(section, "status model", [
    "## 设计决策与交付证据",
    "decisionStatus",
    "implementationEvidence",
    "verificationEvidence",
    "releaseEvidence",
    "evidenceRefs",
    "adopted + none",
    "implemented + none",
    "passed + none",
    "互不推导",
  ]);
}

function collectCompletionRuleIssues(baseline: string): string[] {
  const section = readSection(baseline, "## 设计完成规则");
  return collectMissingMarkers(section, "completion rules", [
    "### 单合同域设计完成",
    "decisionStatus=adopted",
    "唯一 authority",
    "适用范围",
    "非目标",
    "依赖",
    "可验证条件",
    "review 与 verification",
    "不要求产品实现或发布完成",
    "### Repo Wiki 3.0 全项目设计完成",
    "六个 child",
    "实际归档",
    "documentation-closure",
    "blocking conflict",
    "active changes 为空",
  ]);
}

function collectMaterialBoundaryIssues(baseline: string): string[] {
  const section = readSection(baseline, "## 稳定材料分类与维护责任");
  return collectMissingMarkers(section, "material boundaries", [
    ".wiki/06-设计文档/**",
    ".wiki/05-规格基线/capabilities/**",
    ".wiki/04-对外方法/02-v0.2.0发布合同.md",
    "packages/spec-wiki/package.json",
    "crates/*/Cargo.toml",
    ".docs/**",
    ".spec/archive/**",
    "不可改",
    "不作为当前 authority",
    "authority: none",
    "每个 capability 必须有有效 Purpose",
    "Runtime query",
    "核心场景",
    "可靠性生命周期",
    "宿主触发",
  ]);
}

test("canonical product baseline is uniquely reachable", () => {
  expect(collectCanonicalBaselineRequirements()).toEqual([]);
});

test("version domains keep independent authorities and explicit mappings", () => {
  expect(collectVersionContractIssues(readText(baselinePath))).toEqual([]);
});

test("decision status is orthogonal to delivery evidence", () => {
  expect(collectStatusModelIssues(readText(baselinePath))).toEqual([]);
});

test("completion rules separate one contract domain from the whole program", () => {
  expect(collectCompletionRuleIssues(readText(baselinePath))).toEqual([]);
});

test("material boundaries preserve history after repository-wide closure", () => {
  expect(collectMaterialBoundaryIssues(readText(baselinePath))).toEqual([]);
});
