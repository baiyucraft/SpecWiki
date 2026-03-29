/* eslint-disable regexp/no-dupe-disjunctions, regexp/no-unused-capturing-group, regexp/no-super-linear-backtracking, no-control-regex */
/**
 * 统一读取 Markdown 页面，并基于 `reference -> generated` 映射计算 fidelity 指标。
 *
 * 这个模块只负责页面读取、匹配、reuse/skeleton/key-source 诊断，
 * 不负责 runtime gate、provider 运行或 Markdown 报告渲染。
 */

import { readFileSync } from "node:fs";
import path from "node:path";

import { listMarkdownFiles } from "./wiki-runtime-inspection.mjs";

const FILE_MENTION_PATTERN
  = /[\w./-]+\.(?:go|rs|ts|tsx|js|jsx|py|java|kt|php|swift|mdx?|toml|json|ya?ml|conf|ini|sql)/g;
const FILE_LINK_PATTERN = /\(file:\/\/([^)]+)\)/g;
const ASCII_TOKEN_PATTERN = /[A-Z_][\w/-]*/gi;
const EVIDENCE_HEADING_PATTERN = /\*\*[^*\n]*(来源|证据)[^*\n]*\*\*/g;
const TOPIC_KEYWORD_PATTERN
  = /(主题|机制|能力|专题|流程主题|routing|extract|extractor|response|middleware|handler|router)/i;
const HEADING_LINE_PATTERN = /^#{2,3}\s+(.+?)\s*$/;
const HEADING_IGNORE_PATTERNS = [
  /^目录$/,
  /^table of contents$/i,
  /^附录$/,
  /^appendix$/i,
  /^章节结构图$/,
  /^section structure diagram$/i,
  /^章节来源$/,
  /^图表来源$/,
  /^sources?$/i,
  /^references?$/i,
];
const HEADING_ALIAS_PATTERNS = [
  ["intro", ["简介", "概述", "introduction", "overview"]],
  ["structure", ["项目结构", "结构", "project structure"]],
  ["components", ["核心组件", "components"]],
  ["architecture", ["架构总览", "架构", "architecture"]],
  ["dependencies", ["依赖关系分析", "依赖关系", "dependencies"]],
  ["performance", ["性能考量", "performance"]],
  ["troubleshooting", ["故障排查指南", "故障排查", "troubleshooting"]],
  ["conclusion", ["结论", "conclusion"]],
];
export const LOW_FIDELITY_NOTE_PATTERNS = [
  "内容明显短于 reference",
  "解释性段落明显不足",
  "章节拆分比 reference 粗",
  "图表少于 reference",
  "evidence block 少于 reference",
  "缺少引用/出处块",
  "主章节骨架偏离 reference",
  "关键文件覆盖不足",
  "文件名仍偏向英文 raw docs",
];
export const SEVERE_REUSE_THRESHOLD = 3;

function normalizePath(value) {
  return String(value)
    .replaceAll("\\", "/")
    .replace(/#L\d+(?:-L?\d+)?$/i, "");
}

function fileBasename(filePath) {
  return path.posix.basename(normalizePath(filePath));
}

function containsNonAscii(value) {
  return /[^\x00-\x7F]/.test(value);
}

function intersectionSize(left, right) {
  let size = 0;
  for (const item of left) {
    if (right.has(item)) {
      size += 1;
    }
  }
  return size;
}

/**
 * 统一规范化 heading，并过滤 renderer 自动块或 reference 噪声块。
 *
 * @param title 原始 heading 文本。
 * @returns 返回标准化 key；若该 heading 不应进入 skeleton，则返回 `null`。
 */
export function normalizeHeading(title) {
  const normalized = title
    .trim()
    .replace(/[*_`]/g, "")
    .replace(/^[\d.()\-[\]、\s]+/, "")
    .replace(/[：:]+$/, "")
    .replace(/\s+/g, " ")
    .toLowerCase();
  if (!normalized) {
    return null;
  }
  if (HEADING_IGNORE_PATTERNS.some((pattern) => pattern.test(normalized))) {
    return null;
  }
  for (const [canonical, aliases] of HEADING_ALIAS_PATTERNS) {
    if (aliases.some((alias) => normalized.includes(alias))) {
      return canonical;
    }
  }
  return normalized;
}

/**
 * 从最终 Markdown 提取用于骨架对比的 H2/H3 skeleton。
 *
 * @param content Markdown 正文。
 * @returns 返回去噪后的 heading key 列表。
 */
export function extractHeadingSkeleton(content) {
  const headings = [];

  for (const rawLine of content.split(/\r?\n/)) {
    const match = rawLine.match(HEADING_LINE_PATTERN);
    if (!match) {
      continue;
    }
    const normalized = normalizeHeading(match[1] ?? "");
    if (!normalized || headings.at(-1) === normalized) {
      continue;
    }
    headings.push(normalized);
  }

  return headings;
}

/**
 * 提取 `file://` citation，作为关键源码覆盖的第一优先级证据。
 *
 * @param content Markdown 正文。
 * @returns 返回去重后的源码路径。
 */
export function extractFileCitations(content) {
  const citations = new Set();
  for (const match of content.matchAll(FILE_LINK_PATTERN)) {
    citations.add(normalizePath(match[1]).replace(/^\/+/, ""));
  }
  return [...citations].sort();
}

/**
 * 在没有 citation 时，退回到正文里的文件名提及。
 *
 * @param content Markdown 正文。
 * @returns 返回去重后的文件路径或文件名。
 */
export function extractFileMentions(content) {
  const mentions = new Set();
  for (const match of content.matchAll(FILE_MENTION_PATTERN)) {
    mentions.add(normalizePath(match[0]).replace(/^\/+/, ""));
  }
  return [...mentions].sort();
}

/**
 * 关键源码集合优先走 citation，再退回文件提及，避免“正文提到文件但没有任何出处”被高估。
 *
 * @param content Markdown 正文。
 * @returns 返回关键源码集合与命中模式。
 */
export function extractKeySources(content) {
  const citations = extractFileCitations(content);
  if (citations.length > 0) {
    return { keySources: citations, mode: "citation" };
  }

  const mentions = extractFileMentions(content);
  if (mentions.length > 0) {
    return { keySources: mentions, mode: "mention_fallback" };
  }

  return { keySources: [], mode: "none" };
}

function extractAsciiTokens(content) {
  const tokens = new Set();
  for (const match of content.matchAll(ASCII_TOKEN_PATTERN)) {
    const token = match[0].toLowerCase();
    if (token.length >= 3) {
      tokens.add(token);
    }
  }
  return tokens;
}

/**
 * 用页面标题、路径与正文信号推断 2.0 下可复用的分解信号。
 *
 * @param page Markdown 页面摘要。
 * @returns 返回排序后的 decomposition signals。
 */
export function inferDecompositionSignals(page) {
  const combined
    = `${page.relativePath} ${page.title} ${page.content} ${page.fileMentions.join(" ")}`.toLowerCase();
  const signals = new Set();

  if (/(runtime|engine|kernel|store|preview|manager|renderer|core runtime)/.test(combined)) {
    signals.add("runtime");
  }
  if (/(compiler|codegen|annotation|processor|transform|spi)/.test(combined)) {
    signals.add("compiler-pipeline");
  }
  if (/(api|public interface|public api|addon api|preview api|store api|reference)/.test(combined)) {
    signals.add("api-surface");
  }
  if (/(config|configuration|main\.|preview\.|manager\.|preset|settings)/.test(combined)) {
    signals.add("config-surface");
  }
  if (/(docs|guide|concept|architecture|overview|introduction|getting started|概述|指南|原理)/.test(combined)) {
    signals.add("docs-guide");
  }
  if (/(test|testing|spec|javatests|assert|integration test|benchmark)/.test(combined)) {
    signals.add("testing");
  }
  if (/(example|examples|tutorial|quick start|quickstart|sample|walkthrough|示例|教程|快速开始)/.test(combined)) {
    signals.add("example-tutorial");
  }
  if (/(troubleshoot|faq|trouble|debug|pitfall|error|warning|排查|故障)/.test(combined)) {
    signals.add("troubleshooting");
  }
  if (/(android|ios|web|platform|framework|integration|hilt|react|vue|angular|svelte)/.test(combined)) {
    signals.add("integration-platform");
  }

  return [...signals].sort();
}

/**
 * 仅基于最终产物特征推断页面类别，供报告层做可读分组。
 *
 * @param page Markdown 页面摘要。
 * @returns 返回粗粒度页面类别。
 */
export function pageCategory(page) {
  const combined = `${page.relativePath} ${page.title}`.toLowerCase();
  if (combined.includes("项目概述")) {
    return "overview";
  }
  if (combined.includes("系统架构") || combined.includes("核心架构") || combined.includes("架构")) {
    return "architecture";
  }
  if (combined.includes("工作流") || combined.includes("部署")) {
    return "workflow";
  }
  if (page.relativePath.startsWith("专题/") || combined.includes("主题：") || combined.includes("流程主题")) {
    return "topic";
  }
  if (combined.includes("模块") || combined.includes("component")) {
    return "module";
  }
  if (TOPIC_KEYWORD_PATTERN.test(`${page.relativePath} ${page.title}`)) {
    return "topic";
  }
  return "other";
}

/**
 * 给 topic 类页面补一个更细的 label，方便 gap ledger 解释“丢的是哪类专题”。
 *
 * @param page Markdown 页面摘要。
 * @returns 返回 topic label；非 topic 页面时返回 `null`。
 */
export function inferTopicLabel(page) {
  const combined = `${page.relativePath} ${page.title} ${page.sectionTitles.join(" ")} ${page.fileMentions.join(" ")}`.toLowerCase();

  if (combined.includes("flow") || combined.includes("流程")) {
    return "流程主题";
  }
  if (combined.includes("middleware")) {
    return "中间件主题";
  }
  if (combined.includes("router") || combined.includes("routing")) {
    return "路由主题";
  }
  if (combined.includes("extract")) {
    return "提取器主题";
  }
  if (combined.includes("response")) {
    return "响应主题";
  }
  if (combined.includes("handler")) {
    return "处理器主题";
  }
  if (combined.includes("context") || combined.includes("chain") || combined.includes("tree")) {
    return "核心机制主题";
  }
  if (page.category === "topic") {
    return "专题页";
  }
  return null;
}

function isEnglishRawDocsPage(page) {
  const normalizedPath = normalizePath(page.relativePath);
  const fileName = path.posix.basename(normalizedPath, ".md");
  const asciiHeavyPath = /^[\w/\-. ]+$/.test(normalizedPath);
  const asciiHeavyTitle = /^[\w .:/\-()]+$/.test(page.title);
  const docsLike
    = normalizedPath.startsWith("概念指南/")
      || normalizedPath.startsWith("API-参考/")
      || normalizedPath.startsWith("配置参考/")
      || normalizedPath.startsWith("故障排除/");
  return docsLike && asciiHeavyPath && asciiHeavyTitle && !containsNonAscii(fileName);
}

/**
 * 读取单个 Markdown 页面，并补齐 fidelity 分析需要的衍生信号。
 *
 * @param baseDir 页面根目录。
 * @param relativePath 页面相对路径。
 * @returns 返回分析后的页面对象。
 */
export function readMarkdownPage(baseDir, relativePath) {
  const fullPath = path.join(baseDir, relativePath);
  const content = readFileSync(fullPath, "utf-8");
  const lines = content.split(/\r?\n/);
  const title
    = lines.find((line) => line.startsWith("# "))?.replace(/^#\s+/, "").trim()
      || path.basename(relativePath, ".md");
  const sectionTitles = lines
    .filter((line) => line.startsWith("## ") || line.startsWith("### "))
    .map((line) => line.replace(/^#{2,3}\s+/, "").trim());
  const nonEmptyLines = lines.filter((line) => line.trim()).length;
  const proseLines = lines.filter((line) => {
    const trimmed = line.trim();
    return trimmed
      && !trimmed.startsWith("#")
      && !trimmed.startsWith("- ")
      && !trimmed.startsWith("```")
      && !trimmed.startsWith("<!--");
  }).length;
  const bulletLines = lines.filter((line) => line.trim().startsWith("- ")).length;
  const mermaidBlocks = lines.filter((line) => line.trim() === "```mermaid").length;
  const evidenceBlocks = [...content.matchAll(EVIDENCE_HEADING_PATTERN)].length;
  const citations = extractFileCitations(content);
  const fileMentions = extractFileMentions(content);
  const { keySources, mode: keySourceMode } = extractKeySources(content);
  const page = {
    title,
    relativePath: normalizePath(relativePath),
    content,
    sectionTitles,
    nonEmptyLines,
    proseLines,
    bulletLines,
    mermaidBlocks,
    evidenceBlocks,
    citations,
    fileMentions,
    keySources,
    keySourceMode,
    tokens: extractAsciiTokens(`${relativePath}\n${title}\n${content}`),
  };
  const category = pageCategory(page);
  const headingSkeleton = extractHeadingSkeleton(content);

  return {
    ...page,
    category,
    topicLabel: inferTopicLabel({ ...page, category }),
    decompositionSignals: inferDecompositionSignals({ ...page, category }),
    headingSkeleton,
    templateDuplicationFingerprint: `${category}|${headingSkeleton.join(">") || "__empty__"}`,
    englishRawDocsLike: isEnglishRawDocsPage({ ...page, category }),
  };
}

/**
 * 批量读取目录下的 Markdown 页面。
 *
 * @param baseDir 页面根目录。
 * @returns 返回排好序的页面对象列表。
 */
export function readMarkdownPages(baseDir) {
  return listMarkdownFiles(baseDir).map((relativePath) => readMarkdownPage(baseDir, relativePath));
}

/**
 * 对骨架重合度做统一评分，便于汇总为 page-level 和 project-level fidelity。
 *
 * @param referenceSkeleton reference 页面骨架。
 * @param generatedSkeleton generated 页面骨架。
 * @returns 返回 0-1 之间的骨架得分。
 */
export function computeSkeletonScore(referenceSkeleton, generatedSkeleton) {
  if (referenceSkeleton.length === 0) {
    return 1;
  }
  const generatedSet = new Set(generatedSkeleton);
  const overlap = referenceSkeleton.filter((key) => generatedSet.has(key)).length;
  return Number((overlap / referenceSkeleton.length).toFixed(4));
}

/**
 * 关键源码覆盖优先按 citation 精确匹配，再放宽到 basename 对齐。
 *
 * @param referenceSources reference 页面关键源码集合。
 * @param generatedSources generated 页面关键源码集合。
 * @returns 返回 coverage 与缺失项。
 */
export function computeKeySourceCoverage(referenceSources, generatedSources) {
  if (referenceSources.length === 0) {
    return {
      coverage: 1,
      matchedSources: [],
      missingSources: [],
    };
  }

  const generatedSet = new Set(generatedSources.map((item) => normalizePath(item).toLowerCase()));
  const generatedBasenames = new Set(generatedSources.map((item) => fileBasename(item).toLowerCase()));
  const matchedSources = [];
  const missingSources = [];

  for (const source of referenceSources) {
    const normalized = normalizePath(source).toLowerCase();
    if (generatedSet.has(normalized) || generatedBasenames.has(fileBasename(source).toLowerCase())) {
      matchedSources.push(source);
      continue;
    }
    missingSources.push(source);
  }

  return {
    coverage: Number((matchedSources.length / referenceSources.length).toFixed(4)),
    matchedSources,
    missingSources,
  };
}

function scorePageMatch(referencePage, generatedPage) {
  let score = 0;

  if (referencePage.relativePath === generatedPage.relativePath) {
    score += 240;
  }
  if (referencePage.title === generatedPage.title) {
    score += 220;
  }
  if (referencePage.category === generatedPage.category && referencePage.category !== "other") {
    score += 80;
  }

  const referenceMentionSet = new Set(referencePage.fileMentions.map((item) => item.toLowerCase()));
  const generatedMentionSet = new Set(generatedPage.fileMentions.map((item) => item.toLowerCase()));
  const referenceBasenames = new Set(referencePage.fileMentions.map((item) => fileBasename(item).toLowerCase()));
  const generatedBasenames = new Set(generatedPage.fileMentions.map((item) => fileBasename(item).toLowerCase()));
  score += intersectionSize(referenceMentionSet, generatedMentionSet) * 18;
  score += intersectionSize(referenceBasenames, generatedBasenames) * 8;
  score += Math.min(intersectionSize(referencePage.tokens, generatedPage.tokens), 20) * 2;

  if (referencePage.title.includes(generatedPage.title) || generatedPage.title.includes(referencePage.title)) {
    score += 16;
  }

  return score;
}

/**
 * 为 reference 页面选择最像的 generated 页面。
 *
 * @param referencePage reference 页面。
 * @param generatedPages generated 页面集合。
 * @param options 匹配配置。
 * @returns 返回最佳匹配；若分数过低则返回 `null`。
 */
export function chooseGeneratedCounterpart(referencePage, generatedPages, options = {}) {
  const matchThreshold = options.matchThreshold ?? 60;
  let best = null;

  for (const generatedPage of generatedPages) {
    const score = scorePageMatch(referencePage, generatedPage);
    if (!best || score > best.score) {
      best = { generatedPage, score };
    }
  }

  return best && best.score >= matchThreshold ? best : null;
}

/**
 * 对单个映射对做 page-level fidelity 分析。
 *
 * @param referencePage reference 页面。
 * @param generatedPage generated 页面。
 * @param score 匹配分数。
 * @returns 返回对比结果，供项目级汇总进一步计算 reuse 与 gate。
 */
export function comparePagePair(referencePage, generatedPage, score) {
  const referenceFileSet = new Set(referencePage.fileMentions.map((item) => item.toLowerCase()));
  const generatedFileSet = new Set(generatedPage.fileMentions.map((item) => item.toLowerCase()));
  const referenceBasenames = new Set(referencePage.keySources.map((item) => fileBasename(item).toLowerCase()));
  const generatedBasenames = new Set(generatedPage.keySources.map((item) => fileBasename(item).toLowerCase()));
  const overlappingFiles = [...referenceFileSet].filter((item) => generatedFileSet.has(item));
  const overlappingBasenames = [...referenceBasenames].filter((item) => generatedBasenames.has(item));
  const keySource = computeKeySourceCoverage(referencePage.keySources, generatedPage.keySources);
  const skeletonScore = computeSkeletonScore(referencePage.headingSkeleton, generatedPage.headingSkeleton);
  const notes = [];

  if (generatedPage.nonEmptyLines < referencePage.nonEmptyLines * 0.4) {
    notes.push("内容明显短于 reference");
  } else if (generatedPage.nonEmptyLines > referencePage.nonEmptyLines * 1.4) {
    notes.push("内容比 reference 更展开");
  }
  if (generatedPage.proseLines < referencePage.proseLines * 0.4) {
    notes.push("解释性段落明显不足");
  }
  if (generatedPage.sectionTitles.length + 2 < referencePage.sectionTitles.length) {
    notes.push("章节拆分比 reference 粗");
  }
  if (referencePage.mermaidBlocks > generatedPage.mermaidBlocks) {
    notes.push("图表少于 reference");
  }
  if (referencePage.evidenceBlocks > generatedPage.evidenceBlocks) {
    notes.push("evidence block 少于 reference");
  }
  if (referencePage.keySources.length > 0 && generatedPage.keySources.length === 0) {
    notes.push("缺少引用/出处块");
  }
  if (referencePage.category === "topic" && generatedPage.category !== "topic") {
    notes.push("reference 专题被折叠进非专题页");
  }
  if (referencePage.headingSkeleton.length >= 3 && skeletonScore < 0.75) {
    notes.push("主章节骨架偏离 reference");
  }
  if (containsNonAscii(referencePage.relativePath) && generatedPage.englishRawDocsLike) {
    notes.push("文件名仍偏向英文 raw docs");
  }
  if (keySource.missingSources.length > 0) {
    notes.push("关键文件覆盖不足");
    notes.push(`缺少关键文件提及：${keySource.missingSources.slice(0, 8).map(fileBasename).join("、")}`);
  }

  return {
    generatedPath: generatedPage.relativePath,
    generatedTitle: generatedPage.title,
    score,
    referenceLines: referencePage.nonEmptyLines,
    generatedLines: generatedPage.nonEmptyLines,
    referenceProse: referencePage.proseLines,
    generatedProse: generatedPage.proseLines,
    referenceMermaid: referencePage.mermaidBlocks,
    generatedMermaid: generatedPage.mermaidBlocks,
    referenceEvidence: referencePage.evidenceBlocks,
    generatedEvidence: generatedPage.evidenceBlocks,
    referenceCategory: referencePage.category,
    generatedCategory: generatedPage.category,
    referenceTopicLabel: referencePage.topicLabel,
    generatedTopicLabel: generatedPage.topicLabel,
    overlappingFiles,
    overlappingBasenames,
    missingBasenames: keySource.missingSources.slice(0, 8).map(fileBasename),
    missingKeySources: keySource.missingSources,
    notes,
    headingSkeleton: {
      reference: referencePage.headingSkeleton,
      generated: generatedPage.headingSkeleton,
    },
    decompositionSignals: {
      reference: referencePage.decompositionSignals,
      generated: generatedPage.decompositionSignals,
    },
    keySource: {
      reference: referencePage.keySources,
      generated: generatedPage.keySources,
      coverage: keySource.coverage,
      matchedSources: keySource.matchedSources,
      missingSources: keySource.missingSources,
    },
    skeletonScore,
  };
}

function median(values) {
  if (values.length === 0) {
    return null;
  }
  const sorted = [...values].sort((left, right) => left - right);
  const middle = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 1
    ? sorted[middle]
    : Number(((sorted[middle - 1] + sorted[middle]) / 2).toFixed(4));
}

function enrichReuseMetrics(comparisons, generatedPages) {
  const reuseCountByGeneratedPage = new Map();

  for (const comparison of comparisons) {
    if (!comparison.matched) {
      continue;
    }
    reuseCountByGeneratedPage.set(
      comparison.generatedPath,
      (reuseCountByGeneratedPage.get(comparison.generatedPath) ?? 0) + 1,
    );
  }

  const generatedByPath = new Map(generatedPages.map((page) => [page.relativePath, page]));
  const topReuseOffenders = [...reuseCountByGeneratedPage.entries()]
    .filter(([, count]) => count > 1)
    .map(([generatedPath, count]) => ({
      generatedPath,
      generatedTitle: generatedByPath.get(generatedPath)?.title ?? path.posix.basename(generatedPath, ".md"),
      count,
    }))
    .sort((left, right) => right.count - left.count || left.generatedPath.localeCompare(right.generatedPath));
  const offenderPaths = new Set(topReuseOffenders.map((item) => item.generatedPath));
  const enrichedComparisons = comparisons.map((comparison) => ({
    ...comparison,
    reuseCount: comparison.matched ? reuseCountByGeneratedPage.get(comparison.generatedPath) ?? 1 : 0,
  }));
  const collapsedPairs = enrichedComparisons.filter((comparison) =>
    comparison.matched
    && (
        offenderPaths.has(comparison.generatedPath)
        || comparison.notes.includes("reference 专题被折叠进非专题页")
      ),
  );

  return {
    comparisons: enrichedComparisons,
    reuseCountByGeneratedPage: Object.fromEntries(
      [...reuseCountByGeneratedPage.entries()].sort((left, right) => right[1] - left[1] || left[0].localeCompare(right[0])),
    ),
    reusePages: topReuseOffenders.length,
    severeReusePages: topReuseOffenders.filter((item) => item.count >= SEVERE_REUSE_THRESHOLD).length,
    reuseOverage: topReuseOffenders.reduce((sum, item) => sum + item.count - 1, 0),
    topReuseOffenders,
    collapsedPairs,
    collapsedPages: new Set(collapsedPairs.map((item) => item.referencePath)).size,
  };
}

/**
 * 对已有映射对单独计算 reuse/collapse 指标。
 *
 * @param comparisons 对比结果数组。
 * @param generatedPages 可选的 generated 页面摘要，用于补标题。
 * @returns 返回 reuse 相关结构化指标。
 */
export function analyzeReuse(comparisons, generatedPages = []) {
  return enrichReuseMetrics(comparisons, generatedPages);
}

/**
 * 基于两个目录直接产出 reference fidelity 项目级结构化结果。
 *
 * @param options 分析配置。
 * @returns 返回页面映射、reuse、skeleton 与 key-source 的完整快照。
 */
export function analyzeReferenceFidelity(options) {
  const matchThreshold = options.matchThreshold ?? 60;
  const generatedPages = options.generatedPages ?? readMarkdownPages(options.generatedDir);
  const referencePages = options.referencePages ?? readMarkdownPages(options.referenceDir);
  const comparisons = [];
  const matchedGeneratedPaths = new Set();

  for (const referencePage of referencePages) {
    const matched = chooseGeneratedCounterpart(referencePage, generatedPages, { matchThreshold });
    if (!matched) {
      comparisons.push({
        referencePath: referencePage.relativePath,
        referenceTitle: referencePage.title,
        referenceCategory: referencePage.category,
        referenceTopicLabel: referencePage.topicLabel,
        matched: false,
        notes: ["缺少对应生成页面"],
        decompositionSignals: {
          reference: referencePage.decompositionSignals,
          generated: [],
        },
        skeletonScore: 0,
        keySource: {
          reference: referencePage.keySources,
          generated: [],
          coverage: referencePage.keySources.length === 0 ? 1 : 0,
          matchedSources: [],
          missingSources: referencePage.keySources,
        },
      });
      continue;
    }

    matchedGeneratedPaths.add(matched.generatedPage.relativePath);
    comparisons.push({
      referencePath: referencePage.relativePath,
      referenceTitle: referencePage.title,
      matched: true,
      ...comparePagePair(referencePage, matched.generatedPage, matched.score),
    });
  }

  const extraGeneratedPages = generatedPages
    .filter((page) => !matchedGeneratedPaths.has(page.relativePath))
    .map((page) => ({
      relativePath: page.relativePath,
      title: page.title,
      lines: page.nonEmptyLines,
      templateDuplicationFingerprint: page.templateDuplicationFingerprint,
    }));
  const reuse = enrichReuseMetrics(comparisons, generatedPages);
  const matchedComparisons = reuse.comparisons.filter((item) => item.matched);

  return {
    referencePages,
    generatedPages,
    comparisons: reuse.comparisons,
    matchedCount: matchedComparisons.length,
    missingCount: reuse.comparisons.length - matchedComparisons.length,
    extraGeneratedPages,
    reuseCountByGeneratedPage: reuse.reuseCountByGeneratedPage,
    reusePages: reuse.reusePages,
    severeReusePages: reuse.severeReusePages,
    reuseOverage: reuse.reuseOverage,
    topReuseOffenders: reuse.topReuseOffenders,
    collapsedPairs: reuse.collapsedPairs,
    collapsedPages: reuse.collapsedPages,
    lowFidelityMatchedPages: matchedComparisons.filter((item) =>
      item.notes.some((note) => LOW_FIDELITY_NOTE_PATTERNS.includes(note)),
    ).length,
    medianSkeletonScore: median(matchedComparisons.map((item) => item.skeletonScore)),
    medianKeySourceCoverage: median(matchedComparisons.map((item) => item.keySource.coverage)),
    skeletonLowestPages: matchedComparisons
      .slice()
      .sort((left, right) => left.skeletonScore - right.skeletonScore || left.referencePath.localeCompare(right.referencePath)),
    keySourceLowestPages: matchedComparisons
      .slice()
      .sort((left, right) => left.keySource.coverage - right.keySource.coverage || left.referencePath.localeCompare(right.referencePath)),
  };
}
