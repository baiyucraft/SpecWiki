import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..");
const capabilitiesDirectory = path.join(root, ".wiki", "05-规格基线", "capabilities");
const capabilityIndexPath = path.join(root, ".wiki", "05-规格基线", "INDEX.md");
const releaseContractPath = path.join(root, ".wiki", "04-对外方法", "02-v0.2.0发布合同.md");
const designDirectory = path.join(root, ".wiki", "06-设计文档");
const docsDirectory = path.join(root, ".docs");
const activeChangesDirectory = path.join(root, ".spec", "changes");
const docsReferences = [".docs/research/karpathy-llm-wiki-analysis.md"];

function readText(file: string): string {
  return readFileSync(file, "utf8");
}

function capabilityIds(): string[] {
  return readdirSync(capabilitiesDirectory, { withFileTypes: true })
    .filter(entry => entry.isDirectory() && existsSync(path.join(capabilitiesDirectory, entry.name, "spec.md")))
    .map(entry => entry.name)
    .sort();
}

function readMarkdownSection(markdown: string, heading: string): string {
  const start = markdown.indexOf(heading);
  if (start < 0)
    return "";
  const nextHeading = markdown.indexOf("\n## ", start + heading.length);
  return markdown.slice(start + heading.length, nextHeading < 0 ? undefined : nextHeading).trim();
}

function collectMissingMarkers(content: string, context: string, markers: readonly string[]): string[] {
  return markers
    .filter(marker => !content.includes(marker))
    .map(marker => `${context}: missing marker ${marker}`);
}

function collectCapabilityPurposeIssues(): string[] {
  const issues: string[] = [];
  for (const id of capabilityIds()) {
    const purpose = readMarkdownSection(
      readText(path.join(capabilitiesDirectory, id, "spec.md")),
      "## Purpose",
    );
    if (!purpose)
      issues.push(`${id}: missing or empty Purpose`);
    if (/TBD\s*-\s*created by archiving/i.test(purpose))
      issues.push(`${id}: archived Purpose placeholder`);
  }
  return issues;
}

function indexedCapabilityIds(): string[] {
  return [...readText(capabilityIndexPath).matchAll(/\.\/capabilities\/([^/]+)\/spec\.md/g)]
    .map(match => match[1])
    .sort();
}

function collectCapabilityInventoryIssues(): string[] {
  const actual = capabilityIds();
  const indexed = indexedCapabilityIds();
  return [
    ...(actual.includes("content-family-planner")
      ? ["capability inventory: content-family-planner must be merged into knowledge-unit-decomposition"]
      : []),
    ...actual.filter(id => !indexed.includes(id)).map(id => `capability index: missing ${id}`),
    ...indexed.filter(id => !actual.includes(id)).map(id => `capability index: stale ${id}`),
  ];
}

function collectFamilyIdentityIssues(): string[] {
  const decomposition = readText(path.join(
    capabilitiesDirectory,
    "knowledge-unit-decomposition",
    "spec.md",
  ));
  const issues = [
    "family 只能作为 deterministic signal 或 projection style",
    "不得形成脱离 KnowledgeTree 的平行身份或父子链",
  ]
    .filter(marker => !decomposition.includes(marker))
    .map(marker => `knowledge-unit-decomposition: missing marker ${marker}`);

  const legacyIdentityFiles = [
    "page-evidence-layer",
    "page-research-dossier",
    "research-driven-page-composition",
    "wiki-llm-enhancement",
  ];
  const legacyIdentityPattern = /FamilyDossier|family_scoped_evidence|family-(?:leaf-doc|child|index)/g;
  for (const id of legacyIdentityFiles) {
    const matches = readText(path.join(capabilitiesDirectory, id, "spec.md")).match(legacyIdentityPattern) ?? [];
    if (matches.length > 0)
      issues.push(`${id}: legacy family identities ${[...new Set(matches)].join(", ")}`);
  }
  return issues;
}

function collectQueryProjectionIssues(): string[] {
  const bm25 = readText(path.join(capabilitiesDirectory, "wiki-bm25-query", "spec.md"));
  const runtime = readText(path.join(capabilitiesDirectory, "repo-wiki-runtime", "spec.md"));
  const issues = collectMissingMarkers(bm25, "wiki-bm25-query", [
    "route-local BM25",
    "不得定义公开 request/response",
  ]);
  if (bm25.includes("matched_symbols"))
    issues.push("wiki-bm25-query: exposes removed matched_symbols transport");
  if (runtime.includes("query route 必须遵循 `index -> knowledge -> page fallback`"))
    issues.push("repo-wiki-runtime: preserves sequential query fallback as the public contract");
  if (/`matched_(?:symbols|sources|modules|symbol_edges)`/.test(runtime))
    issues.push("repo-wiki-runtime: exposes removed matched_* transport fields");
  return issues;
}

function collectReleaseContractIssues(): string[] {
  const issues: string[] = [];
  if (!existsSync(releaseContractPath)) {
    issues.push("release contract: missing .wiki/04-对外方法/02-v0.2.0发布合同.md");
}
  else {
    const manifest = JSON.parse(readText(path.join(root, "packages", "spec-wiki", "package.json"))) as { version: string };
    issues.push(...collectMissingMarkers(readText(releaseContractPath), "release contract", [
      "# spec-wiki v0.2.0 发布合同",
      "decisionStatus",
      "implementationEvidence",
      "verificationEvidence",
      "releaseEvidence",
      "不自动证明 released",
      `spec-wiki@${manifest.version}`,
    ]));
  }

  const distribution = readText(path.join(capabilitiesDirectory, "adapter-distribution", "spec.md"));
  const workflow = readText(path.join(capabilitiesDirectory, "repo-wiki-workflow", "spec.md"));
  issues.push(...collectMissingMarkers(distribution, "adapter-distribution", [
    "staging/publish contract",
    "不得单独证明 released",
  ]));
  if (distribution.includes("当前真实发布"))
    issues.push("adapter-distribution: staging is described as a real release");
  if (!workflow.includes("product-release contract"))
    issues.push("repo-wiki-workflow: missing product-release contract boundary");
  if (workflow.includes("当前真实公开支持"))
    issues.push("repo-wiki-workflow: adopted contract is described as release evidence");
  return issues;
}

function collectDesignStatusIssues(): string[] {
  const index = readText(path.join(designDirectory, "INDEX.md"));
  const overall = readText(path.join(designDirectory, "00-总体设计.md"));
  const baseline = readText(path.join(designDirectory, "05-产品基线与设计治理.md"));
  const issues = collectMissingMarkers(index, "design index", [
    "adopted authority",
    "implementationEvidence",
    "verificationEvidence",
    "releaseEvidence",
    "当前扩展场景分类 authority",
  ]);
  if (index.includes("不保存未实现阶段草案"))
    issues.push("design index: conflates adopted design with implementation evidence");
  if (overall.includes("implementation-roadmap"))
    issues.push("overall design: links the retired implementation roadmap");
  if (baseline.includes(".docs/release/v0-2-0.md"))
    issues.push("product baseline: points product-release authority to .docs");
  for (const stale of ["本 change 只建立", "后续五个合同 change", "documentation-closure` 修正", "documentation-closure` 完成 capability"]) {
    if (baseline.includes(stale))
      issues.push(`product baseline: preserves iteration-time wording ${stale}`);
}
  issues.push(...collectMissingMarkers(baseline, "product baseline", [
    "../04-对外方法/02-v0.2.0发布合同.md",
    "已执行最终材料迁移",
  ]));
  return issues;
}

function collectHostProjectionIssues(): string[] {
  const agents = readText(path.join(designDirectory, "02-Agents设计.md"));
  const scenarios = readText(path.join(designDirectory, "03-核心场景.md"));
  const issues = collectMissingMarkers(agents, "Agents design", [
    "HostDefinition { id, displayName, detectDir, compatibilityRole, capabilities }",
    "公共 asset validator",
    "CodeBuddy 结构化解析 `user_prompt`",
  ]);
  if (agents.includes("增加 required `compatibilityRole` 与 trigger capabilities"))
    issues.push("Agents design: implemented host role/capabilities remain in the target column");
  issues.push(...collectMissingMarkers(scenarios, "core scenarios", [
    "semantic-or-explicit trigger",
    "task-aware scope 延期",
  ]));
  if (scenarios.includes("task-aware scope 和宿主 trigger 延期"))
    issues.push("core scenarios: implemented trigger remains deferred");
  return issues;
}

function collectMarkdownFiles(directory: string): string[] {
  const files: string[] = [];
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const absolute = path.join(directory, entry.name);
    if (entry.isDirectory())
      files.push(...collectMarkdownFiles(absolute));
    else if (entry.isFile() && entry.name.endsWith(".md"))
      files.push(path.relative(root, absolute).replaceAll("\\", "/"));
  }
  return files.sort();
}

function collectDocsInventoryIssues(): string[] {
  const allowed = [".docs/INDEX.md", ...docsReferences];
  const actual = collectMarkdownFiles(docsDirectory);
  const issues = [
    ...actual.filter(file => !allowed.includes(file)).map(file => `.docs inventory: unexpected ${file}`),
    ...allowed.filter(file => !actual.includes(file)).map(file => `.docs inventory: missing ${file}`),
  ];

  const index = readText(path.join(docsDirectory, "INDEX.md"));
  issues.push(...collectMissingMarkers(index, ".docs index", [
    "无 authority",
    "./research/karpathy-llm-wiki-analysis.md",
  ]));
  const indexedRefs = [...index.matchAll(/`(\.wiki\/[^`]+\.md)`/g)].map(match => match[1]).sort();
  const declaredRefs: string[] = [];
  for (const file of docsReferences) {
    const reference = readText(path.join(root, file));
    issues.push(...collectMissingMarkers(reference, file, [
      "status: reference",
      "authority: none",
      "adoptedRefs:",
      "不构成 active backlog",
    ]));
    issues.push(...collectAdoptedRefIssues(reference));
    declaredRefs.push(...adoptedRefs(reference));
  }
  declaredRefs.sort();
  if (JSON.stringify(indexedRefs) !== JSON.stringify(declaredRefs))
    issues.push(`.docs index: adopted refs differ (index=${indexedRefs.join(", ")}; front matter=${declaredRefs.join(", ")})`);
  for (const removedDirectory of ["./design/", "./roadmap/", "./release/", "./quality/"]) {
    if (index.includes(removedDirectory))
      issues.push(`.docs index: preserves removed directory ${removedDirectory}`);
}
  if (index.includes("活跃草案") || index.includes("authoritative source 已切换"))
    issues.push(".docs index: preserves active/authority wording");

  const testingAuthority = readText(path.join(root, ".wiki", "02-开发指南", "01-测试与验收.md"));
  issues.push(...collectMissingMarkers(testingAuthority, "testing authority", [
    "formal gate",
    "primary gate",
    "baseline guard",
    "diagnostic",
  ]));
  return issues;
}

function adoptedRefs(markdown: string): string[] {
  const lines = markdown.split(/\r?\n/u);
  const refs: string[] = [];
  let inFrontMatter = lines[0] === "---";
  let inAdoptedRefs = false;
  for (const line of lines.slice(1)) {
    if (!inFrontMatter)
      break;
    if (line === "---") {
      inFrontMatter = false;
      continue;
    }
    if (line === "adoptedRefs:") {
      inAdoptedRefs = true;
      continue;
    }
    if (!inAdoptedRefs)
      continue;
    if (!line.startsWith("  - "))
      break;
    refs.push(line.slice(4).trim());
  }
  return refs;
}

function collectAdoptedRefIssues(markdown: string): string[] {
  const refs = adoptedRefs(markdown);
  if (refs.length === 0)
    return [".docs reference: adoptedRefs must contain at least one current Wiki authority"];
  return refs.flatMap((ref) => {
    const normalized = ref.replaceAll("\\", "/");
    if (!isCurrentWikiPage(normalized))
      return [`.docs reference: adopted ref is not a current Wiki page ${ref}`];
    if (!existsSync(path.join(root, normalized)))
      return [`.docs reference: adopted ref does not exist ${ref}`];
    return [];
  });
}

function collectReadmeProjectionIssues(file: string, locale: "en" | "zh-CN"): string[] {
  const readme = readText(path.join(root, file));
  const commonMarkers = [
    "spec-wiki archive <change-id>",
    "--dry-run",
    "--apply",
    "--resume <operation-id>",
    "./.wiki/04-对外方法/02-v0.2.0发布合同.md",
    "`route_groups`",
  ];
  const localizedMarkers = locale === "en"
    ? ["Codex is the only reference host", "Claude and CodeBuddy are compatible hosts", "release contract target"]
    : ["Codex 是唯一 reference host", "Claude、CodeBuddy 是 compatible hosts", "发布合同目标"];
  const issues = collectMissingMarkers(readme, file, [...commonMarkers, ...localizedMarkers]);
  if (readme.includes("/wiki:*"))
    issues.push(`${file}: preserves removed Claude command identity`);
  if (readme.includes("index -> knowledge -> page fallback"))
    issues.push(`${file}: preserves the retired sequential query contract`);
  return issues;
}

function currentAuthorityFiles(): string[] {
  const currentRoots = [
    "README.md",
    "README-CN.md",
    ".docs/INDEX.md",
  ];
  return [
    ...currentRoots,
    ...collectMarkdownFiles(path.join(root, ".wiki")).filter(isCurrentWikiPage),
  ];
}

function isCurrentWikiPage(file: string): boolean {
  const slashNormalized = file.replaceAll("\\", "/");
  const normalized = path.posix.normalize(slashNormalized);
  if (normalized !== slashNormalized)
    return false;
  if (!normalized.startsWith(".wiki/") || !normalized.endsWith(".md"))
    return false;
  return ![".wiki/.knowledge/", ".wiki/pages/", ".wiki/.cache/"]
    .some(runtimePrefix => normalized.startsWith(runtimePrefix));
}

function decodeMarkdownTarget(target: string): string {
  try {
    return decodeURIComponent(target);
  }
  catch {
    return target;
  }
}

function relativeMarkdownTargets(markdown: string): string[] {
  return [...markdown.matchAll(/!?\[[^\]]*\]\(([^)]+)\)/g)]
    .map(match => match[1].trim())
    .filter(target => target.length > 0 && !target.startsWith("#"))
    .filter(target => !/^(?:[a-z][a-z\d+.-]*:|\/)/i.test(target))
    .map(target => target.startsWith("<")
      ? target.slice(1, target.indexOf(">"))
      : target.split(/\s+["']/u, 1)[0])
    .map(target => target.split(/[?#]/u, 1)[0])
    .map(decodeMarkdownTarget);
}

function collectCurrentLinkIssues(): string[] {
  const issues: string[] = [];
  for (const file of currentAuthorityFiles()) {
    for (const target of relativeMarkdownTargets(readText(path.join(root, file)))) {
      const resolved = path.resolve(path.dirname(path.join(root, file)), target);
      if (!existsSync(resolved))
        issues.push(`${file}: missing relative link ${target}`);
    }
  }

  const artifactAuthority = readText(path.join(root, ".wiki", "04-对外方法", "01-配置与运行时产物.md"));
  issues.push(...collectMissingMarkers(artifactAuthority, "runtime artifacts authority", [
    "../../.docs/INDEX.md",
    "authority: none",
    "./02-v0.2.0发布合同.md",
  ]));
  if (artifactAuthority.includes("这些原文保留在根目录或 `.docs/**`"))
    issues.push("runtime artifacts authority: preserves retired .docs release/roadmap source wording");
  return issues;
}

function collectConcreteActivePointerIssues(): string[] {
  const issues: string[] = [];
  const concretePointer = /\.spec\/changes\/([a-z0-9][a-z0-9-]*)/g;
  for (const file of currentAuthorityFiles()) {
    const ids = [...readText(path.join(root, file)).matchAll(concretePointer)].map(match => match[1]);
    for (const id of new Set(ids)) {
      if (!existsSync(path.join(activeChangesDirectory, id)))
        issues.push(`${file}: concrete active pointer is not active ${id}`);
    }
  }
  return issues;
}

test("capability purposes and inventory are closed", () => {
  expect([
    ...collectCapabilityPurposeIssues(),
    ...collectCapabilityInventoryIssues(),
    ...collectFamilyIdentityIssues(),
  ]).toEqual([]);
});

test("query and release projections use canonical authorities", () => {
  expect([
    ...collectQueryProjectionIssues(),
    ...collectReleaseContractIssues(),
  ]).toEqual([]);
});

test("design authorities separate adoption from delivery evidence", () => {
  expect([
    ...collectDesignStatusIssues(),
    ...collectHostProjectionIssues(),
  ]).toEqual([]);
});

test("docs contains only registered non-authoritative references", () => {
  expect(collectDocsInventoryIssues()).toEqual([]);
});

test("public readmes project the current CLI and host contract", () => {
  expect([
    ...collectReadmeProjectionIssues("README.md", "en"),
    ...collectReadmeProjectionIssues("README-CN.md", "zh-CN"),
  ]).toEqual([]);
});

test("current authority links resolve without archived active pointers", () => {
  expect([
    ...collectCurrentLinkIssues(),
    ...collectConcreteActivePointerIssues(),
  ]).toEqual([]);
});

test("docs adopted refs resolve to current Wiki authorities", () => {
  expect(collectAdoptedRefIssues(`---\nadoptedRefs:\n  - .wiki/missing.md\n---`)).not.toEqual([]);
  expect(collectAdoptedRefIssues(`---\nadoptedRefs:\n  - .wiki/../README.md\n---`)).not.toEqual([]);
});

test("current Wiki authority excludes runtime Markdown layers", () => {
  expect(isCurrentWikiPage(".wiki/06-设计文档/INDEX.md")).toBe(true);
  expect(isCurrentWikiPage(".wiki/.knowledge/generated.md")).toBe(false);
  expect(isCurrentWikiPage(".wiki/pages/generated.md")).toBe(false);
  expect(isCurrentWikiPage(".wiki/.cache/generated.md")).toBe(false);
});
