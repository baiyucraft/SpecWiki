/**
 * 针对带 reference 的测试项目，批量执行带 provider 的 init，
 * 并把 generated `.wiki/*.md` 与 reference content 目录下的 Markdown 文件
 * 做逐项目、逐文件的结构化对比，输出到当前 OpenSpec change 目录。
 *
 * 默认输出：
 * - openspec/changes/<change>/reference-project-reports/*.md
 * - openspec/changes/<change>/reference-project-reports/_summary.md
 * - openspec/changes/<change>/reference-project-reports/_optimization-notes.md
 *
 * 用法：
 *   node scripts/collect-reference-project-reports.mjs
 *   node scripts/collect-reference-project-reports.mjs --jobs 2
 *   node scripts/collect-reference-project-reports.mjs chi axum
 */

import { execFileSync } from "node:child_process";
import {
  cpSync,
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  statSync,
  writeFileSync,
} from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

import {
  callCoreStreaming,
  formatUsageSnapshot,
  ROOT_DIR,
  TEST_DIR,
  TMP_DIR,
  removePathWithRetry,
  resolveProjectJobs,
  runTaskPool,
  withTemporaryDevConfig,
} from "./testing/helpers.mjs";

const REFERENCE_DIR = path.join(TMP_DIR, "reference");
const DEFAULT_CHANGE = "iteration-9-4-family-planner-and-research-first-composition";
const REAL_REPO_MAP = {
  aLocal: "E:\\project\\aLocal",
};

const FILE_MENTION_PATTERN =
  /[A-Za-z0-9_./-]+\.(?:go|rs|ts|tsx|js|jsx|py|java|kt|php|swift|md|toml|json|ya?ml|conf|ini|sql)/g;
const ASCII_TOKEN_PATTERN = /[A-Za-z_][A-Za-z0-9_/-]*/g;
const EVIDENCE_HEADING_PATTERN = /\*\*[^*\n]*(来源|证据)[^*\n]*\*\*/g;
const TOPIC_KEYWORD_PATTERN =
  /(主题|机制|能力|专题|流程主题|routing|extract|extractor|response|middleware|handler|router)/i;

function discoverProjects() {
  if (!existsSync(REFERENCE_DIR)) {
    return [];
  }

  return readdirSync(REFERENCE_DIR)
    .filter((entry) => statSync(path.join(REFERENCE_DIR, entry)).isDirectory())
    .sort();
}

function listMarkdownFiles(dir) {
  if (!existsSync(dir)) {
    return [];
  }

  const files = [];
  const walk = (currentDir) => {
    for (const entry of readdirSync(currentDir, { withFileTypes: true })) {
      const fullPath = path.join(currentDir, entry.name);
      if (entry.isDirectory()) {
        walk(fullPath);
        continue;
      }
      if (entry.name.endsWith(".md")) {
        files.push(path.relative(dir, fullPath).replaceAll("\\", "/"));
      }
    }
  };

  walk(dir);
  return files.sort();
}

function resolveRunMode(runMode) {
  if (runMode === "warm") {
    return { label: "warm", cacheMode: "preserve" };
  }
  return { label: "cold", cacheMode: "clear" };
}

function formatElapsed(elapsedMs) {
  if (elapsedMs < 1_000) {
    return `${elapsedMs}ms`;
  }
  return `${(elapsedMs / 1_000).toFixed(elapsedMs >= 10_000 ? 0 : 1)}s`;
}

function createProjectProgressPrinter(project, runLabel) {
  const countedPercents = new Map();
  const phaseMessages = new Map();
  let lastUsageTotal = -1;

  return {
    info(message) {
      console.log(`[${project}/${runLabel}] ${message}`);
    },
    onProgress(event) {
      const prefix = `[${project}/${runLabel}] ${formatElapsed(event.elapsed_ms)} ${event.phase}`;
      if (event.phase === "llm_usage" && event.usage) {
        if (event.usage.total_tokens === lastUsageTotal) {
          return;
        }
        lastUsageTotal = event.usage.total_tokens;
        console.log(`${prefix} ${formatUsageSnapshot(event.usage)}`);
        return;
      }
      if (event.processed != null && event.total != null && event.total > 0) {
        const percent = Math.floor((event.processed / event.total) * 100);
        const lastPercent = countedPercents.get(event.phase) ?? -1;
        const shouldPrint =
          event.processed === 0
          || event.processed === event.total
          || percent >= lastPercent + 10;
        if (!shouldPrint) {
          return;
        }
        countedPercents.set(event.phase, percent);
        console.log(
          `${prefix} ${event.processed}/${event.total} ${event.message}`,
        );
        return;
      }

      if (phaseMessages.get(event.phase) === event.message) {
        return;
      }
      phaseMessages.set(event.phase, event.message);
      console.log(`${prefix} ${event.message}`);
    },
  };
}

function ensureDir(dir) {
  mkdirSync(dir, { recursive: true });
}

function readPage(baseDir, relativePath) {
  const fullPath = path.join(baseDir, relativePath);
  const content = readFileSync(fullPath, "utf-8");
  const lines = content.split(/\r?\n/);
  const title =
    lines.find((line) => line.startsWith("# "))?.replace(/^#\s+/, "").trim()
    || path.basename(relativePath, ".md");
  const sectionTitles = lines
    .filter((line) => line.startsWith("## "))
    .map((line) => line.replace(/^##\s+/, "").trim());
  const nonEmptyLines = lines.filter((line) => line.trim()).length;
  const proseLines = lines.filter((line) => {
    const trimmed = line.trim();
    return trimmed && !trimmed.startsWith("#") && !trimmed.startsWith("- ") && !trimmed.startsWith("```") && !trimmed.startsWith("<!--");
  }).length;
  const bulletLines = lines.filter((line) => line.trim().startsWith("- ")).length;
  const mermaidBlocks = lines.filter((line) => line.trim() === "```mermaid").length;
  const citations = [...content.matchAll(/\(file:\/\/([^)]+)\)/g)].map((match) => match[1]);
  const fileMentions = extractFileMentions(content);
  const tokens = extractAsciiTokens(`${relativePath}\n${title}\n${content}`);
  const evidenceBlocks = [...content.matchAll(EVIDENCE_HEADING_PATTERN)].length;
  const category = pageCategory({ relativePath, title, content });
  const topicLabel = inferTopicLabel({ relativePath, title, sectionTitles, fileMentions });

  return {
    title,
    relativePath,
    content,
    sectionTitles,
    nonEmptyLines,
    proseLines,
    bulletLines,
    mermaidBlocks,
    evidenceBlocks,
    citations,
    fileMentions,
    tokens,
    category,
    topicLabel,
  };
}

function extractFileMentions(content) {
  const mentions = new Set();
  for (const match of content.matchAll(FILE_MENTION_PATTERN)) {
    mentions.add(match[0].replaceAll("\\", "/"));
  }
  return [...mentions].sort();
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

function fileBasename(filePath) {
  return path.posix.basename(filePath.replaceAll("\\", "/"));
}

function pageCategory(page) {
  const combined = `${page.relativePath} ${page.title}`.toLowerCase();
  if (combined.includes("项目概述")) {
    return "overview";
  }
  if (
    combined.includes("系统架构")
    || combined.includes("核心架构")
    || combined.includes("架构")
  ) {
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

function inferTopicLabel(page) {
  const relativePath = page.relativePath.toLowerCase();
  const title = page.title.toLowerCase();
  const content = `${page.sectionTitles?.join(" ")} ${page.fileMentions.join(" ")}`.toLowerCase();
  const combined = `${relativePath} ${title} ${content}`;

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

function scorePageMatch(referencePage, generatedPage) {
  let score = 0;

  if (referencePage.relativePath === generatedPage.relativePath) {
    score += 240;
  }
  if (referencePage.title === generatedPage.title) {
    score += 220;
  }
  if (pageCategory(referencePage) === pageCategory(generatedPage) && pageCategory(referencePage) !== "other") {
    score += 80;
  }

  const referenceMentionSet = new Set(referencePage.fileMentions.map((item) => item.toLowerCase()));
  const generatedMentionSet = new Set(generatedPage.fileMentions.map((item) => item.toLowerCase()));
  const referenceBasenames = new Set(referencePage.fileMentions.map((item) => fileBasename(item).toLowerCase()));
  const generatedBasenames = new Set(generatedPage.fileMentions.map((item) => fileBasename(item).toLowerCase()));
  const fileOverlap = intersectionSize(referenceMentionSet, generatedMentionSet);
  const basenameOverlap = intersectionSize(referenceBasenames, generatedBasenames);
  score += fileOverlap * 18;
  score += basenameOverlap * 8;

  const tokenOverlap = intersectionSize(referencePage.tokens, generatedPage.tokens);
  score += Math.min(tokenOverlap, 20) * 2;

  if (referencePage.title.includes(generatedPage.title) || generatedPage.title.includes(referencePage.title)) {
    score += 16;
  }

  return score;
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

function chooseGeneratedCounterpart(referencePage, generatedPages) {
  let best = null;

  for (const generatedPage of generatedPages) {
    const score = scorePageMatch(referencePage, generatedPage);
    if (!best || score > best.score) {
      best = { generatedPage, score };
    }
  }

  return best && best.score >= 60 ? best : null;
}

function comparePagePair(referencePage, generatedPage, score) {
  const referenceFileSet = new Set(referencePage.fileMentions.map((item) => item.toLowerCase()));
  const generatedFileSet = new Set(generatedPage.fileMentions.map((item) => item.toLowerCase()));
  const referenceBasenames = new Set(referencePage.fileMentions.map((item) => fileBasename(item).toLowerCase()));
  const generatedBasenames = new Set(generatedPage.fileMentions.map((item) => fileBasename(item).toLowerCase()));
  const overlappingFiles = [...referenceFileSet].filter((item) => generatedFileSet.has(item));
  const overlappingBasenames = [...referenceBasenames].filter((item) => generatedBasenames.has(item));
  const missingBasenames = [...referenceBasenames]
    .filter((item) => !generatedBasenames.has(item))
    .slice(0, 8);

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
  if (referencePage.citations.length > 0 && generatedPage.citations.length === 0) {
    notes.push("缺少引用/出处块");
  }
  if (referencePage.category === "topic" && generatedPage.category !== "topic") {
    notes.push("reference 专题被折叠进非专题页");
  }
  if (missingBasenames.length > 0) {
    notes.push(`缺少关键文件提及：${missingBasenames.join("、")}`);
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
    missingBasenames,
    notes,
  };
}

async function runInitForProject(project, progressPrinter, runMode) {
  const { cacheMode, label } = resolveRunMode(runMode);
  const projectRoot = path.join(TEST_DIR, project);
  const wikiDir = path.join(projectRoot, ".wiki");
  progressPrinter.info(`start init cache_mode=${cacheMode}`);

  if (REAL_REPO_MAP[project]) {
    const realRepoRoot = REAL_REPO_MAP[project];
    const realWikiDir = path.join(realRepoRoot, ".wiki");

    const result = await withTemporaryDevConfig(
      realRepoRoot,
      () =>
        callCoreStreaming(
          { action: "init", repoRoot: realRepoRoot },
          { onProgress: (event) => progressPrinter.onProgress(event) },
        ),
      { cacheMode },
    );
    if (!result.response.ok) {
      throw new Error(result.response.error || `${project} init failed`);
    }

    removePathWithRetry(wikiDir);
    cpSync(realWikiDir, wikiDir, { recursive: true });
    removePathWithRetry(realWikiDir);
    progressPrinter.info("init done");
    return {
      label,
      cacheMode,
      usage: summarizeUsage(result.progressEvents),
    };
  }

  const result = await withTemporaryDevConfig(
    projectRoot,
    () =>
      callCoreStreaming(
        { action: "init", repoRoot: `tmp/test/${project}` },
        { onProgress: (event) => progressPrinter.onProgress(event) },
      ),
    { cacheMode },
  );
  if (!result.response.ok) {
    throw new Error(result.response.error || `${project} init failed`);
  }
  progressPrinter.info("init done");
  return {
    label,
    cacheMode,
    usage: summarizeUsage(result.progressEvents),
  };
}

function summarizeUsage(progressEvents) {
  for (let index = progressEvents.length - 1; index >= 0; index--) {
    const event = progressEvents[index];
    if (event.phase === "llm_usage" && event.usage) {
      return event.usage;
    }
  }
  return null;
}

function promptCount(usage, promptType) {
  return usage?.by_prompt_type?.find((bucket) => bucket.key === promptType)?.request_count ?? 0;
}

function collectProject(project, run) {
  const projectRoot = path.join(TEST_DIR, project);
  const generatedWikiDir = path.join(projectRoot, ".wiki");
  const referenceWikiDir = path.join(REFERENCE_DIR, project, "content");
  const generatedPages = listMarkdownFiles(generatedWikiDir).map((relativePath) =>
    readPage(generatedWikiDir, relativePath)
  );
  const referencePages = listMarkdownFiles(referenceWikiDir).map((relativePath) =>
    readPage(referenceWikiDir, relativePath)
  );

  const comparisons = [];
  const matchedGeneratedPaths = new Set();

  for (const referencePage of referencePages) {
    const matched = chooseGeneratedCounterpart(referencePage, generatedPages);
    if (!matched) {
      comparisons.push({
        referencePath: referencePage.relativePath,
        referenceTitle: referencePage.title,
        matched: false,
        notes: ["缺少对应生成页面"],
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
    }));

  const matchedCount = comparisons.filter((item) => item.matched).length;
  const missingCount = comparisons.length - matchedCount;
  const reusedGeneratedPages = countReusedGeneratedPages(comparisons);
  const coverage = summarizeCoverage(comparisons, generatedPages, referencePages);
  const commonGaps = summarizeProjectGaps(comparisons, generatedPages, referencePages, coverage);

  return {
    project,
    runLabel: run.label,
    cacheMode: run.cacheMode,
    usage: run.usage,
    generatedPageCount: generatedPages.length,
    referencePageCount: referencePages.length,
    matchedCount,
    missingCount,
    extraGeneratedPages,
    comparisons,
    reusedGeneratedPages,
    coverage,
    commonGaps,
  };
}

function countReusedGeneratedPages(comparisons) {
  const counts = new Map();
  for (const comparison of comparisons) {
    if (!comparison.matched) {
      continue;
    }
    counts.set(
      comparison.generatedPath,
      (counts.get(comparison.generatedPath) ?? 0) + 1,
    );
  }
  return [...counts.entries()]
    .filter(([, count]) => count > 1)
    .map(([generatedPath, count]) => ({ generatedPath, count }));
}

function summarizeCoverage(comparisons, generatedPages, referencePages) {
  const matched = comparisons.filter((item) => item.matched);
  const generatedTopicPages = generatedPages.filter((page) => page.category === "topic");
  const referenceTopicPages = referencePages.filter((page) => page.category === "topic");
  const generatedEvidencePages = generatedPages.filter((page) => page.evidenceBlocks > 0);
  const referenceEvidencePages = referencePages.filter((page) => page.evidenceBlocks > 0 || page.citations.length > 0);
  const generatedDiagramPages = generatedPages.filter((page) => page.mermaidBlocks > 0);
  const referenceDiagramPages = referencePages.filter((page) => page.mermaidBlocks > 0);
  const archetypeTopicPages = generatedPages.filter((page) =>
    page.relativePath.startsWith("专题/repo-archetype/")
  ).length;
  const generatedCitationUnits = generatedPages.reduce(
    (sum, page) => sum + page.evidenceBlocks + page.citations.length,
    0,
  );
  const referenceCitationUnits = referencePages.reduce(
    (sum, page) => sum + page.evidenceBlocks + page.citations.length,
    0,
  );
  const topicLabelCounts = new Map();

  for (const page of generatedTopicPages) {
    const label = page.topicLabel ?? "专题页";
    topicLabelCounts.set(label, (topicLabelCounts.get(label) ?? 0) + 1);
  }

  const missingTopicLabels = new Map();
  for (const comparison of comparisons) {
    if (!comparison.matched && comparison.referenceTopicLabel) {
      missingTopicLabels.set(
        comparison.referenceTopicLabel,
        (missingTopicLabels.get(comparison.referenceTopicLabel) ?? 0) + 1,
      );
      continue;
    }
    if (comparison.notes?.includes("reference 专题被折叠进非专题页") && comparison.referenceTopicLabel) {
      missingTopicLabels.set(
        comparison.referenceTopicLabel,
        (missingTopicLabels.get(comparison.referenceTopicLabel) ?? 0) + 1,
      );
    }
  }

  return {
    generatedTopicPages: generatedTopicPages.length,
    referenceTopicPages: referenceTopicPages.length,
    generatedEvidencePages: generatedEvidencePages.length,
    referenceEvidencePages: referenceEvidencePages.length,
    generatedDiagramPages: generatedDiagramPages.length,
    referenceDiagramPages: referenceDiagramPages.length,
    archetypeTopicPages,
    generatedCitationUnits,
    referenceCitationUnits,
    generatedCitationDensity:
      generatedPages.length === 0 ? 0 : Number((generatedCitationUnits / generatedPages.length).toFixed(2)),
    referenceCitationDensity:
      referencePages.length === 0 ? 0 : Number((referenceCitationUnits / referencePages.length).toFixed(2)),
    matchedEvidenceShortfall: matched.filter((item) => item.generatedEvidence < item.referenceEvidence).length,
    matchedDiagramShortfall: matched.filter((item) => item.generatedMermaid < item.referenceMermaid).length,
    topicLabels: [...topicLabelCounts.entries()].sort((left, right) => right[1] - left[1]),
    missingTopicLabels: [...missingTopicLabels.entries()].sort((left, right) => right[1] - left[1]),
  };
}

function summarizeProjectGaps(comparisons, generatedPages, referencePages, coverage) {
  const gaps = [];
  const matched = comparisons.filter((item) => item.matched);

  if (referencePages.length > generatedPages.length * 2) {
    gaps.push("reference 文档树明显更细，当前 planner 仍以总览页和少量模块页为主");
  }
  if (comparisons.some((item) => !item.matched)) {
    gaps.push("存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足");
  }
  if (
    coverage.referenceTopicPages > 0
    && coverage.generatedTopicPages < Math.max(1, Math.floor(coverage.referenceTopicPages * 0.4))
  ) {
    gaps.push("专题页覆盖仍明显不足，很多 reference 主题还没有被 planner 单独承载");
  }
  if (matched.some((item) => item.notes.includes("缺少引用/出处块"))) {
    gaps.push("生成页普遍缺少 reference 那种源码引用/出处层");
  }
  if (matched.some((item) => item.notes.includes("evidence block 少于 reference"))) {
    gaps.push("evidence block 已进入页面，但覆盖率和密度仍低于 reference");
  }
  if (matched.some((item) => item.notes.includes("图表少于 reference"))) {
    gaps.push("Mermaid/结构图表达仍然不足");
  }
  if (coverage.matchedDiagramShortfall > 0 && coverage.generatedDiagramPages === 0) {
    gaps.push("facts-driven 图输入尚未稳定覆盖到代表性页面");
  }
  if (matched.some((item) => item.notes.includes("章节拆分比 reference 粗"))) {
    gaps.push("单页章节拆分比 reference 粗，主题混杂在同一页里");
  }
  if (matched.some((item) => item.notes.includes("解释性段落明显不足"))) {
    gaps.push("解释层正文密度仍低于 reference");
  }
  if (coverage.missingTopicLabels.length > 0) {
    gaps.push(`高频缺失专题集中在：${coverage.missingTopicLabels.slice(0, 4).map(([label]) => label).join("、")}`);
  }

  return gaps;
}

function renderProjectReport(result) {
  const lines = [
    `# ${result.project} Reference 对比报告`,
    "",
    `生成页面：${result.generatedPageCount} 页`,
    `reference 页面：${result.referencePageCount} 页`,
    `命中对比：${result.matchedCount} 页`,
    `缺失对比：${result.missingCount} 页`,
    `运行模式：${result.runLabel} (cache_mode=${result.cacheMode})`,
    `LLM usage：requests=${result.usage?.request_count ?? 0}, total_tokens=${result.usage?.total_tokens ?? 0}, page_research=${promptCount(result.usage, "page_research")}, page_enrichment=${promptCount(result.usage, "page_enrichment")}`,
    "",
    "## 覆盖统计",
    "",
    `- 专题页覆盖：generated ${result.coverage.generatedTopicPages} / reference ${result.coverage.referenceTopicPages}（repo-archetype=${result.coverage.archetypeTopicPages}）`,
    `- evidence 落页：generated ${result.coverage.generatedEvidencePages} / reference ${result.coverage.referenceEvidencePages}`,
    `- citation 密度：generated ${result.coverage.generatedCitationDensity} / reference ${result.coverage.referenceCitationDensity}`,
    `- 图表达覆盖：generated ${result.coverage.generatedDiagramPages} / reference ${result.coverage.referenceDiagramPages}`,
    `- page research 请求：${promptCount(result.usage, "page_research")}`,
    `- page enrichment 请求：${promptCount(result.usage, "page_enrichment")}`,
    `- 已规划专题类型：${result.coverage.topicLabels.slice(0, 6).map(([label, count]) => `${label}(${count})`).join("、") || "无"}`,
    `- 高频缺失专题：${result.coverage.missingTopicLabels.slice(0, 6).map(([label, count]) => `${label}(${count})`).join("、") || "无"}`,
    "",
    "## 项目结论",
    "",
  ];

  for (const gap of result.commonGaps) {
    lines.push(`- ${gap}`);
  }
  if (result.commonGaps.length === 0) {
    lines.push("- 当前项目没有出现明显的结构性差距。");
  }

  if (result.reusedGeneratedPages.length > 0) {
    lines.push("");
    lines.push("## 多页折叠现象");
    lines.push("");
    for (const item of result.reusedGeneratedPages) {
      lines.push(`- ${item.generatedPath} 被 ${item.count} 个 reference 页面共享映射`);
    }
  }

  if (result.extraGeneratedPages.length > 0) {
    lines.push("");
    lines.push("## 额外生成页面");
    lines.push("");
    for (const page of result.extraGeneratedPages) {
      lines.push(`- ${page.relativePath} (${page.title}, ${page.lines} 行)`);
    }
  }

  lines.push("");
  lines.push("## 逐文件对比");
  lines.push("");
  lines.push("| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |");
  lines.push("| --- | --- | ---: | ---: | ---: | ---: | --- |");

  for (const comparison of result.comparisons) {
    if (!comparison.matched) {
      lines.push(
        `| ${comparison.referencePath} | 缺失 | - | - | - | - | ${comparison.notes.join("；")} |`,
      );
      continue;
    }

    const note = comparison.notes.length > 0 ? comparison.notes.join("；") : "基本可对应";
    lines.push(
      `| ${comparison.referencePath} | ${comparison.generatedPath} | ${comparison.referenceLines}/${comparison.generatedLines} | ${comparison.referenceProse}/${comparison.generatedProse} | ${comparison.referenceEvidence}/${comparison.generatedEvidence} | ${comparison.referenceMermaid}/${comparison.generatedMermaid} | ${note} |`,
    );
  }

  lines.push("");
  lines.push("## 逐文件详情");
  lines.push("");

  for (const comparison of result.comparisons) {
    lines.push(`### ${comparison.referencePath}`);
    lines.push("");
    lines.push(`- reference 标题：${comparison.referenceTitle}`);
    if (!comparison.matched) {
      lines.push("- 生成页：无");
      lines.push(`- 问题：${comparison.notes.join("；")}`);
      lines.push("");
      continue;
    }

    lines.push(`- 生成页：${comparison.generatedPath}（${comparison.generatedTitle}）`);
    lines.push(`- 匹配分数：${comparison.score}`);
    lines.push(`- 页面类型：${comparison.referenceCategory} / ${comparison.generatedCategory}`);
    lines.push(`- 行数：${comparison.referenceLines} / ${comparison.generatedLines}`);
    lines.push(`- 段落行数：${comparison.referenceProse} / ${comparison.generatedProse}`);
    lines.push(`- Evidence：${comparison.referenceEvidence} / ${comparison.generatedEvidence}`);
    lines.push(`- Mermaid：${comparison.referenceMermaid} / ${comparison.generatedMermaid}`);
    lines.push(
      `- 文件提及重合：${comparison.overlappingBasenames.slice(0, 12).join("、") || "无"}`,
    );
    if (comparison.missingBasenames.length > 0) {
      lines.push(`- reference 关键文件未覆盖：${comparison.missingBasenames.join("、")}`);
    }
    lines.push(`- 结论：${comparison.notes.join("；") || "基本可对应"}`);
    lines.push("");
  }

  return `${lines.join("\n")}\n`;
}

function renderSummary(results) {
  const lines = [
    "# Reference 项目集汇总",
    "",
    `生成时间：${new Date().toISOString()}`,
    "",
    "| Project | generated | reference | matched | missing | topic(gen/ref) | archetype | citation(gen/ref) | diagram(gen/ref) | extra generated |",
    "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
  ];

  for (const result of results) {
    lines.push(
    `| ${result.project} | ${result.generatedPageCount} | ${result.referencePageCount} | ${result.matchedCount} | ${result.missingCount} | ${result.coverage.generatedTopicPages}/${result.coverage.referenceTopicPages} | ${result.coverage.archetypeTopicPages} | ${result.coverage.generatedCitationDensity}/${result.coverage.referenceCitationDensity} | ${result.coverage.generatedDiagramPages}/${result.coverage.referenceDiagramPages} | ${result.extraGeneratedPages.length} |`,
    );
  }

  const commonGapCounts = new Map();
  for (const result of results) {
    for (const gap of result.commonGaps) {
      commonGapCounts.set(gap, (commonGapCounts.get(gap) ?? 0) + 1);
    }
  }

  lines.push("");
  lines.push("## 高频差距");
  lines.push("");
  for (const [gap, count] of [...commonGapCounts.entries()].sort((left, right) => right[1] - left[1])) {
    lines.push(`- ${gap}：${count} 个项目`);
  }

  return `${lines.join("\n")}\n`;
}

function renderOptimizationNotes(results) {
  const lines = [
    "# Reference 对比后的优化收敛",
    "",
    "## 当前收敛",
    "",
    "这轮 9.3 已经把 targeted dossier、section-plan-driven 页面组装和 archetype 专题页接入主链，但 reference 项目集仍然能看出剩余差距主要集中在 planner 覆盖率、citation 密度和 research 结果对正文结构的主导程度。",
    "",
    "## 高频观察",
    "",
  ];

  const topTopicGaps = new Map();
  for (const result of results) {
    for (const [label, count] of result.coverage.missingTopicLabels.slice(0, 4)) {
      topTopicGaps.set(label, (topTopicGaps.get(label) ?? 0) + count);
    }
  }

  const projectsWithTopicPages = results.filter((result) => result.coverage.generatedTopicPages > 0).length;
  const projectsWithEvidence = results.filter((result) => result.coverage.generatedEvidencePages > 0).length;
  const projectsWithDiagrams = results.filter((result) => result.coverage.generatedDiagramPages > 0).length;
  const projectsWithArchetypeTopics = results.filter((result) => result.coverage.archetypeTopicPages > 0).length;
  const avgGeneratedCitationDensity =
    results.length === 0
      ? 0
      : Number(
        (
          results.reduce((sum, result) => sum + result.coverage.generatedCitationDensity, 0)
          / results.length
        ).toFixed(2),
      );
  const avgReferenceCitationDensity =
    results.length === 0
      ? 0
      : Number(
        (
          results.reduce((sum, result) => sum + result.coverage.referenceCitationDensity, 0)
          / results.length
        ).toFixed(2),
      );

  lines.push(`- 已生成专题页的项目：${projectsWithTopicPages}/${results.length}`);
  lines.push(`- 已生成 repo-archetype 专题的项目：${projectsWithArchetypeTopics}/${results.length}`);
  lines.push(`- 已落 evidence block 的项目：${projectsWithEvidence}/${results.length}`);
  lines.push(`- 已落 Mermaid 图的项目：${projectsWithDiagrams}/${results.length}`);
  lines.push(`- 平均 citation 密度：generated ${avgGeneratedCitationDensity} / reference ${avgReferenceCitationDensity}`);
  lines.push(
    `- 高频缺失专题：${[...topTopicGaps.entries()].sort((left, right) => right[1] - left[1]).slice(0, 6).map(([label, count]) => `${label}(${count})`).join("、") || "无"}`,
  );
  lines.push("");
  lines.push("## 下一步建议");
  lines.push("");
  lines.push("- 优先继续调 planner 阈值和 topic seed 规则，让根级机制页、流程主题页、repo-archetype 专题覆盖更多 reference 高频主题。");
  lines.push("- evidence layer 下一步应补 citation 密度和 section 内证据命中率，而不是回退到全文文件清单。");
  lines.push("- overview/architecture 的 research 结果需要更稳定落页，否则 section-plan 对总览页的收益会被 budget 和 fallback 抵消。");
  lines.push("- Mermaid 已进入主链，下一步重点是让更多页面拥有 diagram inputs，而不是放宽 LLM 自由生成结构图。");

  return `${lines.join("\n")}\n`;
}

function writeReports(results, reportDir, summaryPath, optimizationNotesPath) {
  ensureDir(reportDir);
  for (const result of results) {
    writeFileSync(path.join(reportDir, `${result.project}.md`), renderProjectReport(result));
  }
  writeFileSync(summaryPath, renderSummary(results));
  writeFileSync(optimizationNotesPath, renderOptimizationNotes(results));
}

function parseCliArgs(argv) {
  const names = [];
  let jobs;
  let runMode = "cold";
  let change = DEFAULT_CHANGE;

  for (let index = 0; index < argv.length; index++) {
    const arg = argv[index];
    if (arg === "--jobs") {
      jobs = argv[index + 1];
      index++;
      continue;
    }
    if (arg === "--run-mode") {
      runMode = argv[index + 1] || runMode;
      index++;
      continue;
    }
    if (arg === "--change") {
      change = argv[index + 1] || change;
      index++;
      continue;
    }
    names.push(arg);
  }

  return { change, jobs, names, runMode };
}

async function main(argv) {
  const changeDir = path.join(ROOT_DIR, "openspec", "changes", argv.change);
  const reportDir = path.join(changeDir, "reference-project-reports");
  const summaryPath = path.join(reportDir, "_summary.md");
  const optimizationNotesPath = path.join(reportDir, "_optimization-notes.md");
  const projects = argv.names.length > 0 ? argv.names : discoverProjects();
  const jobs = argv.jobs == null ? 1 : resolveProjectJobs(argv.jobs, projects.length);
  const results = await runTaskPool(projects, jobs, async (project, index) => {
    const progressPrinter = createProjectProgressPrinter(project, argv.runMode);
    progressPrinter.info(`queue ${index + 1}/${projects.length}`);
    const run = await runInitForProject(project, progressPrinter, argv.runMode);
    const result = collectProject(project, run);
    progressPrinter.info(
      `report ready: generated=${result.generatedPageCount}, reference=${result.referencePageCount}, matched=${result.matchedCount}, missing=${result.missingCount}, total_tokens=${run.usage?.total_tokens ?? 0}`,
    );
    return result;
  });

  writeReports(results, reportDir, summaryPath, optimizationNotesPath);
  process.stdout.write(`${JSON.stringify({
    change: argv.change,
    reportDir,
    summaryPath,
    optimizationNotesPath,
    jobs,
    runMode: argv.runMode,
    projects: results.map((result) => ({
      project: result.project,
      runMode: result.runLabel,
      generatedPageCount: result.generatedPageCount,
      referencePageCount: result.referencePageCount,
      matchedCount: result.matchedCount,
      missingCount: result.missingCount,
      generatedTopicPages: result.coverage.generatedTopicPages,
      generatedEvidencePages: result.coverage.generatedEvidencePages,
      generatedDiagramPages: result.coverage.generatedDiagramPages,
      totalTokens: result.usage?.total_tokens ?? 0,
      pageResearchRequests: promptCount(result.usage, "page_research"),
    })),
  }, null, 2)}\n`);
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  await main(parseCliArgs(process.argv.slice(2)));
}
