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
const DEFAULT_CHANGE = "iteration-9-5-provider-first-research-evidence-and-unit-decomposition";
const VALIDATION_PROJECTS = ["storybook", "dagger"];
const REAL_REPO_MAP = {
  aLocal: "E:\\project\\aLocal",
};

const FILE_MENTION_PATTERN =
  /[A-Za-z0-9_./-]+\.(?:go|rs|ts|tsx|js|jsx|py|java|kt|php|swift|md|toml|json|ya?ml|conf|ini|sql)/g;
const ASCII_TOKEN_PATTERN = /[A-Za-z_][A-Za-z0-9_/-]*/g;
const EVIDENCE_HEADING_PATTERN = /\*\*[^*\n]*(来源|证据)[^*\n]*\*\*/g;
const FILE_LINK_PATTERN = /\(file:\/\/([^)]+)\)/g;
const TOPIC_KEYWORD_PATTERN =
  /(主题|机制|能力|专题|流程主题|routing|extract|extractor|response|middleware|handler|router)/i;
const LOW_FIDELITY_NOTE_PATTERNS = [
  "内容明显短于 reference",
  "解释性段落明显不足",
  "章节拆分比 reference 粗",
  "图表少于 reference",
  "evidence block 少于 reference",
  "缺少引用/出处块",
  "主章节骨架偏离 reference",
  "文件名仍偏向英文 raw docs",
];
const COLLAPSE_NOTE_PATTERNS = ["reference 专题被折叠进非专题页"];
const REFERENCE_OUTLINE_PATTERNS = [
  ["cite", ["cite", "引用", "出处"]],
  ["toc", ["目录", "table of contents"]],
  ["intro", ["简介", "概述", "introduction", "overview"]],
  ["structure", ["项目结构", "结构", "project structure"]],
  ["components", ["核心组件", "components"]],
  ["architecture", ["架构总览", "架构", "architecture"]],
  ["component-analysis", ["详细组件分析", "组件详解", "detailed component analysis"]],
  ["dependencies", ["依赖关系分析", "依赖关系", "dependencies"]],
  ["performance", ["性能考量", "performance"]],
  ["troubleshooting", ["故障排查指南", "故障排查", "troubleshooting"]],
  ["conclusion", ["结论", "conclusion"]],
  ["appendix", ["附录", "appendix"]],
];
const RESEARCH_PROFILE_LABELS = {
  runtime: "Runtime",
  "compiler-pipeline": "CompilerPipeline",
  "api-surface": "ApiSurface",
  "config-surface": "ConfigSurface",
  "docs-guide": "DocsGuide",
  testing: "Testing",
  "example-tutorial": "ExampleTutorial",
  troubleshooting: "Troubleshooting",
  "integration-platform": "IntegrationPlatform",
};
const STOP_REASON_ORDER = [
  "completed",
  "no_further_tool_calls",
  "no_meaningful_delta",
  "turn_budget_exhausted",
  "call_budget_rejected",
  "provider_error",
  "invalid_output",
  "not_run",
];
const MAX_INIT_RESUME_ATTEMPTS = 4;
const DEFAULT_INIT_TIMEOUT_MS = 120 * 60 * 1000;

function discoverProjects() {
  if (!existsSync(REFERENCE_DIR)) {
    return [];
  }

  return readdirSync(REFERENCE_DIR)
    .filter((entry) => statSync(path.join(REFERENCE_DIR, entry)).isDirectory())
    .filter((entry) => VALIDATION_PROJECTS.includes(entry))
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

function querySqliteRows(dbPath, sql) {
  if (!existsSync(dbPath)) {
    return [];
  }

  const statement = `PRAGMA busy_timeout=30000; ${sql}`;
  let lastError = null;

  for (let attempt = 0; attempt < 5; attempt++) {
    try {
      const output = execFileSync("sqlite3", [dbPath, statement], {
        encoding: "utf-8",
        timeout: 35_000,
        maxBuffer: 64 * 1024 * 1024,
      })
        .split(/\r?\n/)
        .map((line) => line.trim())
        .filter(Boolean);
      if (output[0] === "30000") {
        output.shift();
      }
      return output;
    } catch (error) {
      lastError = error;
      if (!String(error.stderr || error.message || "").includes("database is locked")) {
        throw error;
      }
      Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 250);
    }
  }

  throw lastError;
}

function parseSqliteNumber(value) {
    const parsed = Number(value ?? 0);
    return Number.isFinite(parsed) ? parsed : 0;
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function isRetryableInitErrorMessage(message) {
  const normalized = String(message ?? "").toLowerCase();
  return (
    normalized.includes("error sending request for url")
    || normalized.includes("error decoding response body")
    || normalized.includes("provider returned 408")
    || normalized.includes("provider returned 429")
    || /provider returned 5\d\d/.test(normalized)
  );
}

function readPipelineCheckpointSummary(projectRoot) {
  const dbPath = path.join(projectRoot, ".wiki", ".cache", "wiki-cache.db");
  const rows = querySqliteRows(
    dbPath,
    `
      SELECT
        interrupted_stage,
        COALESCE(interrupted_target_id, ''),
        COALESCE(error_message, '')
      FROM pipeline_checkpoint
      LIMIT 1
    `,
  );
  if (rows.length === 0) {
    return null;
  }
  const [stage = "", targetId = "", errorMessage = ""] = rows[0].split("|");
  return { stage, targetId, errorMessage };
}

async function runInitAttempt(projectRoot, repoRootArg, progressPrinter, cacheMode, timeoutMs) {
  return await withTemporaryDevConfig(
    projectRoot,
    () =>
      callCoreStreaming(
        { action: "init", repoRoot: repoRootArg },
        {
          onProgress: (event) => progressPrinter.onProgress(event),
          timeoutMs,
        },
      ),
    { cacheMode },
  );
}

function readResearchStopMetrics(generatedWikiDir) {
  const dbPath = path.join(generatedWikiDir, ".cache", "wiki-cache.db");
  const aggregateRow = querySqliteRows(
    dbPath,
    `
      WITH unit_research AS (
        SELECT
          COALESCE(json_extract(result, '$.provider_stop_reason'), 'not_run') AS stop_reason,
          COALESCE(json_extract(result, '$.provider_session_stats.turns_used'), 0) AS turns_used,
          COALESCE(json_extract(result, '$.provider_session_stats.tool_calls'), 0) AS tool_calls,
          COALESCE(json_extract(result, '$.provider_session_stats.delta_evidence_count'), 0) AS delta_evidence_count,
          COALESCE(json_extract(result, '$.provider_session_stats.delta_section_count'), 0) AS delta_section_count,
          COALESCE(json_extract(result, '$.provider_session_stats.delta_diagram_count'), 0) AS delta_diagram_count,
          COALESCE(json_extract(result, '$.provider_session_stats.child_digest_delta'), 0) AS child_digest_delta
        FROM research_cache
        WHERE research_type = 'unit'
      )
      SELECT
        COUNT(*),
        SUM(CASE WHEN stop_reason IN ('turn_budget_exhausted', 'call_budget_rejected') THEN 1 ELSE 0 END),
        SUM(CASE WHEN stop_reason = 'no_meaningful_delta' THEN 1 ELSE 0 END),
        SUM(CASE WHEN stop_reason = 'invalid_output' THEN 1 ELSE 0 END),
        SUM(CASE WHEN stop_reason = 'provider_error' THEN 1 ELSE 0 END),
        SUM(turns_used),
        SUM(tool_calls),
        SUM(delta_evidence_count),
        SUM(delta_section_count),
        SUM(delta_diagram_count),
        SUM(child_digest_delta)
      FROM unit_research;
    `,
  )[0];
  const countRows = querySqliteRows(
    dbPath,
    `
      SELECT
        COALESCE(json_extract(result, '$.provider_stop_reason'), 'not_run') || char(9) || COUNT(*)
      FROM research_cache
      WHERE research_type = 'unit'
      GROUP BY COALESCE(json_extract(result, '$.provider_stop_reason'), 'not_run');
    `,
  );

  const counts = new Map(
    countRows
      .map((row) => row.split("\t"))
      .filter(([reason, count]) => reason && count)
      .map(([reason, count]) => [reason, parseSqliteNumber(count)]),
  );
  const [
    totalUnitResearchPages,
    budgetStoppedPages,
    stalledPages,
    invalidOutputPages,
    providerFailedPages,
    turnsUsed,
    toolCalls,
    deltaEvidenceCount,
    deltaSectionCount,
    deltaDiagramCount,
    childDigestDelta,
  ] = (aggregateRow?.split("|") ?? []).map((value) => parseSqliteNumber(value));

  return {
    totalUnitResearchPages,
    budgetStoppedPages,
    stalledPages,
    invalidOutputPages,
    providerFailedPages,
    stopReasonCounts: STOP_REASON_ORDER
      .map((reason) => [reason, counts.get(reason) ?? 0])
      .filter(([, count]) => count > 0),
    aggregateStats: {
      turnsUsed,
      toolCalls,
      deltaEvidenceCount,
      deltaSectionCount,
      deltaDiagramCount,
      childDigestDelta,
    },
  };
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
  const citations = [...content.matchAll(FILE_LINK_PATTERN)].map((match) => match[1]);
  const fileMentions = extractFileMentions(content);
  const tokens = extractAsciiTokens(`${relativePath}\n${title}\n${content}`);
  const evidenceBlocks = [...content.matchAll(EVIDENCE_HEADING_PATTERN)].length;
  const category = pageCategory({ relativePath, title, content });
  const topicLabel = inferTopicLabel({ relativePath, title, sectionTitles, fileMentions });
  const decompositionSignals = inferDecompositionSignals({
    relativePath,
    title,
    content,
    fileMentions,
    category,
  });
  const outlineSkeleton = extractOutlineSkeleton(sectionTitles, content);
  const englishRawDocsLike = isEnglishRawDocsPage({ relativePath, title, sectionTitles });

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
    decompositionSignals,
    outlineSkeleton,
    englishRawDocsLike,
  };
}

function inferDecompositionSignals(page) {
  const combined =
    `${page.relativePath} ${page.title} ${page.content} ${page.fileMentions.join(" ")}`.toLowerCase();
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

function containsNonAscii(value) {
  return /[^\x00-\x7F]/.test(value);
}

function isEnglishRawDocsPage(page) {
  const normalizedPath = page.relativePath.replaceAll("\\", "/");
  const fileName = path.posix.basename(normalizedPath, ".md");
  const asciiHeavyPath = /^[A-Za-z0-9/_\-. ]+$/.test(normalizedPath);
  const asciiHeavyTitle = /^[A-Za-z0-9 .:/_\-()]+$/.test(page.title);
  const docsLike =
    normalizedPath.startsWith("概念指南/")
    || normalizedPath.startsWith("API-参考/")
    || normalizedPath.startsWith("配置参考/")
    || normalizedPath.startsWith("故障排除/");
  return docsLike && asciiHeavyPath && asciiHeavyTitle && !containsNonAscii(fileName);
}

function normalizeOutlineSectionKey(title) {
  const normalized = title.trim().toLowerCase();
  if (!normalized) {
    return null;
  }
  for (const [key, aliases] of REFERENCE_OUTLINE_PATTERNS) {
    if (aliases.some((alias) => normalized.includes(alias))) {
      return key;
    }
  }
  return null;
}

function extractOutlineSkeleton(sectionTitles, content) {
  const keys = [];
  if (content.includes("<cite>") || content.includes("章节来源") || content.includes("图表来源")) {
    keys.push("cite");
  }
  for (const title of sectionTitles) {
    const key = normalizeOutlineSectionKey(title);
    if (key && !keys.includes(key)) {
      keys.push(key);
    }
  }
  return keys;
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
  if (
    referencePage.outlineSkeleton.length >= 6
    && outlineOverlapRate(referencePage.outlineSkeleton, generatedPage.outlineSkeleton) < 0.75
  ) {
    notes.push("主章节骨架偏离 reference");
  }
  if (containsNonAscii(referencePage.relativePath) && generatedPage.englishRawDocsLike) {
    notes.push("文件名仍偏向英文 raw docs");
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
    outlineSkeleton: {
      reference: referencePage.outlineSkeleton,
      generated: generatedPage.outlineSkeleton,
    },
    decompositionSignals: {
      reference: referencePage.decompositionSignals,
      generated: generatedPage.decompositionSignals,
    },
  };
}

function classificationCounts(comparisons) {
  const missingPages = comparisons.filter((item) => !item.matched).length;
  const collapsedPages = comparisons.filter((item) =>
    item.matched && item.notes?.some((note) => COLLAPSE_NOTE_PATTERNS.includes(note))
  ).length;
  const lowFidelityMatchedPages = comparisons.filter((item) =>
    item.matched
      && item.notes?.some((note) => LOW_FIDELITY_NOTE_PATTERNS.includes(note))
  ).length;
  return { missingPages, collapsedPages, lowFidelityMatchedPages };
}

function outlineOverlapRate(referenceOutline, generatedOutline) {
  if (referenceOutline.length === 0) {
    return 1;
  }
  const generatedSet = new Set(generatedOutline);
  const overlap = referenceOutline.filter((key) => generatedSet.has(key)).length;
  return overlap / referenceOutline.length;
}

function summarizeDecompositionCoverage(generatedPages, referencePages, comparisons) {
  const signalKeys = [
    "runtime",
    "compiler-pipeline",
    "api-surface",
    "config-surface",
    "docs-guide",
    "testing",
    "example-tutorial",
    "troubleshooting",
    "integration-platform",
  ];
  const countSignals = (pages) =>
    signalKeys.map((signal) => [
      signal,
      pages.filter((page) => page.decompositionSignals.includes(signal)).length,
    ]);

  const generated = new Map(countSignals(generatedPages));
  const reference = new Map(countSignals(referencePages));
  const missingSignals = new Map();

  for (const comparison of comparisons) {
    if (!comparison.matched) {
      for (const signal of inferDecompositionSignals({
        relativePath: comparison.referencePath,
        title: comparison.referenceTitle,
        content: "",
        fileMentions: [],
        category: "other",
      })) {
        missingSignals.set(signal, (missingSignals.get(signal) ?? 0) + 1);
      }
      continue;
    }

    const referenceSignals = comparison.decompositionSignals.reference ?? [];
    const generatedSignals = new Set(comparison.decompositionSignals.generated ?? []);
    for (const signal of referenceSignals) {
      if (!generatedSignals.has(signal)) {
        missingSignals.set(signal, (missingSignals.get(signal) ?? 0) + 1);
      }
    }
  }

  return {
    generated: signalKeys.map((signal) => [signal, generated.get(signal) ?? 0]),
    reference: signalKeys.map((signal) => [signal, reference.get(signal) ?? 0]),
    missingSignals: [...missingSignals.entries()].sort((left, right) => right[1] - left[1]),
  };
}

async function runInitForProject(project, progressPrinter, runMode, initTimeoutMs) {
  const { cacheMode, label } = resolveRunMode(runMode);
  const projectRoot = path.join(TEST_DIR, project);
  const wikiDir = path.join(projectRoot, ".wiki");
  progressPrinter.info(
    `start init cache_mode=${cacheMode} timeout=${Math.round(initTimeoutMs / 60000)}m`,
  );

  if (REAL_REPO_MAP[project]) {
    const realRepoRoot = REAL_REPO_MAP[project];
    const realWikiDir = path.join(realRepoRoot, ".wiki");
    const result = await runInitWithResume(
      project,
      realRepoRoot,
      realRepoRoot,
      progressPrinter,
      cacheMode,
      initTimeoutMs,
    );

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

  const result = await runInitWithResume(
    project,
    projectRoot,
    `tmp/test/${project}`,
    progressPrinter,
    cacheMode,
    initTimeoutMs,
  );
  progressPrinter.info("init done");
  return {
    label,
    cacheMode,
    usage: summarizeUsage(result.progressEvents),
  };
}

async function runInitWithResume(
  project,
  projectRoot,
  repoRootArg,
  progressPrinter,
  initialCacheMode,
  initTimeoutMs,
) {
  let cacheMode = initialCacheMode;
  let lastError = null;

  for (let attempt = 1; attempt <= MAX_INIT_RESUME_ATTEMPTS; attempt++) {
    const attemptLabel = cacheMode === "preserve" ? "resume" : "cold";
    if (attempt > 1) {
      progressPrinter.info(`retry init attempt=${attempt}/${MAX_INIT_RESUME_ATTEMPTS} mode=${attemptLabel}`);
    }

    try {
      const result = await runInitAttempt(
        projectRoot,
        repoRootArg,
        progressPrinter,
        cacheMode,
        initTimeoutMs,
      );
      if (!result.response.ok) {
        throw new Error(result.response.error || `${project} init failed`);
      }
      return result;
    } catch (error) {
      lastError = error;
      const message = String(error?.message ?? error);
      const checkpoint = readPipelineCheckpointSummary(projectRoot);
      const retryable = isRetryableInitErrorMessage(message);
      if (!retryable || attempt >= MAX_INIT_RESUME_ATTEMPTS || !checkpoint) {
        throw error;
      }
      progressPrinter.info(
        `provider failure is retryable; resume from checkpoint stage=${checkpoint.stage || "unknown"} target=${checkpoint.targetId || "n/a"}`,
      );
      cacheMode = "preserve";
      await sleep(Math.min(30_000, attempt * 5_000));
    }
  }

  throw lastError ?? new Error(`${project} init failed`);
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
        referenceCategory: referencePage.category,
        referenceTopicLabel: referencePage.topicLabel,
        decompositionSignals: {
          reference: referencePage.decompositionSignals,
          generated: [],
        },
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
  const classifications = classificationCounts(comparisons);
  const decomposition = summarizeDecompositionCoverage(
    generatedPages,
    referencePages,
    comparisons,
  );
  const commonGaps = summarizeProjectGaps(comparisons, generatedPages, referencePages, coverage);
  const stopReasons = readResearchStopMetrics(generatedWikiDir);
  const overallMatchRate = Number(
    (
      (referencePages.length === 0 ? 0 : matchedCount / referencePages.length)
      * 100
    ).toFixed(2),
  );

  assertGeneratedWikiReady(project, generatedWikiDir, generatedPages, referencePages);

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
    classifications,
    decomposition,
    overallMatchRate,
    commonGaps,
    stopReasons,
  };
}

function assertGeneratedWikiReady(project, generatedWikiDir, generatedPages, referencePages) {
  if (generatedPages.length > 0 || referencePages.length === 0) {
    return;
  }

  const cacheDbPath = path.join(generatedWikiDir, ".cache", "wiki-cache.db");
  const checkpoint = readPipelineCheckpointSummary(path.dirname(generatedWikiDir));
  if (!existsSync(cacheDbPath) && !checkpoint) {
    return;
  }

  const checkpointSummary = checkpoint
    ? ` stage=${checkpoint.stage || "unknown"} target=${checkpoint.targetId || "n/a"}`
    : "";
  throw new Error(
    `${project} generated wiki is incomplete: no markdown pages found under ${generatedWikiDir}, but cache/checkpoint exists.${checkpointSummary}`,
  );
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
    matchedOutlineShortfall: matched.filter((item) => item.notes.includes("主章节骨架偏离 reference")).length,
    matchedEnglishNamingShortfall: matched.filter((item) => item.notes.includes("文件名仍偏向英文 raw docs")).length,
    extraEnglishRawDocsPages: generatedPages.filter((page) =>
      page.englishRawDocsLike && !comparisons.some((comparison) => comparison.generatedPath === page.relativePath)
    ).length,
    topicLabels: [...topicLabelCounts.entries()].sort((left, right) => right[1] - left[1]),
    missingTopicLabels: [...missingTopicLabels.entries()].sort((left, right) => right[1] - left[1]),
  };
}

function summarizeProjectGaps(comparisons, generatedPages, referencePages, coverage) {
  const gaps = [];
  const matched = comparisons.filter((item) => item.matched);
  const reusedGeneratedPages = countReusedGeneratedPages(comparisons).length;
  const citationDensityGap =
    coverage.referenceCitationDensity === 0
      ? 0
      : 1 - (coverage.generatedCitationDensity / coverage.referenceCitationDensity);
  const diagramCoverageGap =
    coverage.referenceDiagramPages === 0
      ? 0
      : 1 - (coverage.generatedDiagramPages / coverage.referenceDiagramPages);

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
  if (
    matched.some((item) => item.notes.includes("evidence block 少于 reference"))
    && citationDensityGap > 0.1
  ) {
    gaps.push("evidence block 已进入页面，但覆盖率和密度仍低于 reference");
  }
  if (
    matched.some((item) => item.notes.includes("图表少于 reference"))
    && diagramCoverageGap > 0.1
  ) {
    gaps.push("Mermaid/结构图表达仍然不足");
  }
  if (coverage.matchedDiagramShortfall > 0 && coverage.generatedDiagramPages === 0) {
    gaps.push("facts-driven 图输入尚未稳定覆盖到代表性页面");
  }
  if (matched.some((item) => item.notes.includes("章节拆分比 reference 粗"))) {
    gaps.push("单页章节拆分比 reference 粗，主题混杂在同一页里");
  }
  if (matched.some((item) => item.notes.includes("主章节骨架偏离 reference"))) {
    gaps.push("docs-backed 页面主章节骨架仍偏离 reference，存在英文原始 docs heading 或泛化模板回退");
  }
  if (matched.some((item) => item.notes.includes("解释性段落明显不足"))) {
    gaps.push("解释层正文密度仍低于 reference");
  }
  if (coverage.extraEnglishRawDocsPages > 0 || matched.some((item) => item.notes.includes("文件名仍偏向英文 raw docs"))) {
    gaps.push("最终 `.wiki/` 中仍残留英文 raw docs 文件名，说明 docs corpus 优先级还未完全收敛");
  }
  if (coverage.missingTopicLabels.length > 0) {
    gaps.push(`高频缺失专题集中在：${coverage.missingTopicLabels.slice(0, 4).map(([label]) => label).join("、")}`);
  }
  if (reusedGeneratedPages > Math.max(15, Math.ceil(referencePages.length * 0.1))) {
    gaps.push("仍存在明显 page collapse，同一生成页承担多个 reference 页面");
  }

  return gaps;
}

function primaryReferenceSignal(comparison) {
  const preferredOrder = [
    "api-surface",
    "config-surface",
    "compiler-pipeline",
    "runtime",
    "testing",
    "example-tutorial",
    "troubleshooting",
    "integration-platform",
    "docs-guide",
  ];
  const referenceSignals = comparison.decompositionSignals?.reference ?? [];
  return preferredOrder.find((signal) => referenceSignals.includes(signal)) ?? referenceSignals[0] ?? "docs-guide";
}

function knowledgeUnitTarget(signal) {
  switch (signal) {
    case "api-surface":
      return "ApiDoc / API 参考";
    case "config-surface":
      return "ConfigDoc / 配置参考";
    case "testing":
      return "TestDoc / 测试基础设施";
    case "example-tutorial":
      return "ExampleDoc / 概念指南";
    case "troubleshooting":
      return "TroubleshootDoc / 故障排除";
    case "integration-platform":
      return "IntegrationDoc / 概念指南";
    case "compiler-pipeline":
      return "ModuleDoc / 编译工具链";
    case "runtime":
      return "ModuleDoc / 核心运行时";
    default:
      return "ConceptGuide / 概念指南";
  }
}

function researchProfileTarget(signal) {
  return RESEARCH_PROFILE_LABELS[signal] ?? "DocsGuide";
}

function remediationContract(kind, comparison) {
  const notes = comparison.notes ?? [];
  if (kind === "missing") {
    return "KnowledgeUnit decomposition / planner";
  }
  if (kind === "collapsed") {
    return "leaf-first child digest / parent-consume-child";
  }
  if (
    notes.some((note) =>
      note.includes("引用/出处")
      || note.includes("evidence block")
      || note.includes("图表少于 reference")
    )
  ) {
    return "renderer markdown contract / citation / mermaid";
  }
  return "research profile / section plan / evidence density";
}

function ledgerEntry(kind, comparison) {
  const signal = primaryReferenceSignal(comparison);
  return {
    referencePath: comparison.referencePath,
    referenceTitle: comparison.referenceTitle,
    generatedPath: comparison.generatedPath ?? "缺失",
    knowledgeUnit: knowledgeUnitTarget(signal),
    researchProfile: researchProfileTarget(signal),
    contract: remediationContract(kind, comparison),
    signal,
    notes: comparison.notes ?? [],
  };
}

function buildGapLedger(result) {
  const missingEntries = result.comparisons
    .filter((comparison) => !comparison.matched)
    .map((comparison) => ledgerEntry("missing", comparison));
  const collapsedEntries = result.comparisons
    .filter((comparison) =>
      comparison.matched
      && comparison.notes?.some((note) => COLLAPSE_NOTE_PATTERNS.includes(note))
    )
    .map((comparison) => ledgerEntry("collapsed", comparison));
  const lowFidelityEntries = result.comparisons
    .filter((comparison) =>
      comparison.matched
      && comparison.notes?.some((note) => LOW_FIDELITY_NOTE_PATTERNS.includes(note))
    )
    .map((comparison) => ledgerEntry("low_fidelity", comparison));

  return {
    missingEntries,
    collapsedEntries,
    lowFidelityEntries,
  };
}

function renderGapLedger(result) {
  const ledger = buildGapLedger(result);
  const lines = [
    `# ${result.project} 95% 差距台账`,
    "",
    "## 当前口径",
    "",
    `- overall_match_rate：${result.matchedCount}/${result.referencePageCount} = ${result.overallMatchRate}%`,
    `- missing pages：${ledger.missingEntries.length}`,
    `- collapsed pages：${ledger.collapsedEntries.length}`,
    `- low-fidelity matched pages：${ledger.lowFidelityEntries.length}`,
    `- extra generated pages：${result.extraGeneratedPages.length}`,
    "",
  ];

  for (const [label, entries] of [
    ["missing pages", ledger.missingEntries],
    ["collapsed pages", ledger.collapsedEntries],
    ["low-fidelity matched pages", ledger.lowFidelityEntries],
  ]) {
    lines.push(`## ${label}`);
    lines.push("");
    if (entries.length === 0) {
      lines.push("- 当前没有该类差距。");
      lines.push("");
      continue;
    }
    for (const entry of entries) {
      lines.push(`### ${entry.referencePath}`);
      lines.push("");
      lines.push(`- reference 标题：${entry.referenceTitle}`);
      lines.push(`- 当前生成页：${entry.generatedPath}`);
      lines.push(`- 建议承载 KnowledgeUnit：${entry.knowledgeUnit}`);
      lines.push(`- 对应 ResearchProfile：${entry.researchProfile}`);
      lines.push(`- 优先修复 contract：${entry.contract}`);
      lines.push(`- Decomposition 信号：${entry.signal}`);
      lines.push(`- 诊断：${entry.notes.join("；") || "无"}`);
      lines.push("");
    }
  }

  return `${lines.join("\n")}\n`;
}

function renderProjectReport(result) {
  const lines = [
    `# ${result.project} Reference 对比报告`,
    "",
    `生成页面：${result.generatedPageCount} 页`,
    `reference 页面：${result.referencePageCount} 页`,
    `命中对比：${result.matchedCount} 页`,
    `缺失对比：${result.missingCount} 页`,
    `总体对齐率：${result.overallMatchRate}%`,
    `运行模式：${result.runLabel} (cache_mode=${result.cacheMode})`,
    `LLM usage：requests=${result.usage?.request_count ?? 0}, total_tokens=${result.usage?.total_tokens ?? 0}, page_research=${promptCount(result.usage, "page_research")}, page_enrichment=${promptCount(result.usage, "page_enrichment")}`,
    "",
    "## 95% 验收口径",
    "",
    `- overall_match_rate：${result.matchedCount}/${result.referencePageCount} = ${result.overallMatchRate}%`,
    `- missing pages：${result.classifications.missingPages}`,
    `- collapsed pages：${result.classifications.collapsedPages}`,
    `- low-fidelity matched pages：${result.classifications.lowFidelityMatchedPages}`,
    `- extra generated pages：${result.extraGeneratedPages.length}`,
    `- provider-backed page research requests：${promptCount(result.usage, "page_research")}`,
    `- budget stopped pages：${result.stopReasons.budgetStoppedPages}`,
    `- stalled pages：${result.stopReasons.stalledPages}`,
    `- invalid output pages：${result.stopReasons.invalidOutputPages}`,
    `- provider failed pages：${result.stopReasons.providerFailedPages}`,
    "",
    "## 覆盖统计",
    "",
    `- 专题页覆盖：generated ${result.coverage.generatedTopicPages} / reference ${result.coverage.referenceTopicPages}（repo-archetype=${result.coverage.archetypeTopicPages}）`,
    `- evidence 落页：generated ${result.coverage.generatedEvidencePages} / reference ${result.coverage.referenceEvidencePages}`,
    `- citation 密度：generated ${result.coverage.generatedCitationDensity} / reference ${result.coverage.referenceCitationDensity}`,
    `- 图表达覆盖：generated ${result.coverage.generatedDiagramPages} / reference ${result.coverage.referenceDiagramPages}`,
    `- 主章节骨架短板：${result.coverage.matchedOutlineShortfall} 页`,
    `- 英文 raw docs 命名残留：matched ${result.coverage.matchedEnglishNamingShortfall} / extra ${result.coverage.extraEnglishRawDocsPages}`,
    `- page research 请求：${promptCount(result.usage, "page_research")}`,
    `- page enrichment 请求：${promptCount(result.usage, "page_enrichment")}`,
    `- stop reason 分布：${result.stopReasons.stopReasonCounts.map(([reason, count]) => `${reason}(${count})`).join("、") || "无"}`,
    `- research session 聚合：turns=${result.stopReasons.aggregateStats.turnsUsed}, tool_calls=${result.stopReasons.aggregateStats.toolCalls}, delta_section=${result.stopReasons.aggregateStats.deltaSectionCount}, delta_evidence=${result.stopReasons.aggregateStats.deltaEvidenceCount}, delta_diagram=${result.stopReasons.aggregateStats.deltaDiagramCount}, child_digest=${result.stopReasons.aggregateStats.childDigestDelta}`,
    `- 已规划专题类型：${result.coverage.topicLabels.slice(0, 6).map(([label, count]) => `${label}(${count})`).join("、") || "无"}`,
    `- 高频缺失专题：${result.coverage.missingTopicLabels.slice(0, 6).map(([label, count]) => `${label}(${count})`).join("、") || "无"}`,
    "",
    "## Decomposition 命中",
    "",
    `- generated：${result.decomposition.generated.map(([label, count]) => `${label}(${count})`).join("、") || "无"}`,
    `- reference：${result.decomposition.reference.map(([label, count]) => `${label}(${count})`).join("、") || "无"}`,
    `- 高频缺口：${result.decomposition.missingSignals.slice(0, 6).map(([label, count]) => `${label}(${count})`).join("、") || "无"}`,
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
    "| Project | generated | reference | matched | overall | missing | collapsed | low-fidelity | topic(gen/ref) | citation(gen/ref) | diagram(gen/ref) | extra generated | budget stop | stalled | invalid | provider failed |",
    "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
  ];

  for (const result of results) {
    lines.push(
    `| ${result.project} | ${result.generatedPageCount} | ${result.referencePageCount} | ${result.matchedCount} | ${result.overallMatchRate}% | ${result.classifications.missingPages} | ${result.classifications.collapsedPages} | ${result.classifications.lowFidelityMatchedPages} | ${result.coverage.generatedTopicPages}/${result.coverage.referenceTopicPages} | ${result.coverage.generatedCitationDensity}/${result.coverage.referenceCitationDensity} | ${result.coverage.generatedDiagramPages}/${result.coverage.referenceDiagramPages} | ${result.extraGeneratedPages.length} | ${result.stopReasons.budgetStoppedPages} | ${result.stopReasons.stalledPages} | ${result.stopReasons.invalidOutputPages} | ${result.stopReasons.providerFailedPages} |`,
    );
  }

  const commonGapCounts = new Map();
  for (const result of results) {
    for (const gap of result.commonGaps) {
      commonGapCounts.set(gap, (commonGapCounts.get(gap) ?? 0) + 1);
    }
  }

  lines.push("");
  lines.push("## 95% Gate");
  lines.push("");
  for (const result of results) {
    const pass =
      result.overallMatchRate >= 95
      && result.classifications.collapsedPages === 0
      && result.extraGeneratedPages.length <= Math.max(5, Math.ceil(result.generatedPageCount * 0.08));
    lines.push(
      `- ${result.project}：${pass ? "通过候选" : "未通过"}，overall=${result.overallMatchRate}% / missing=${result.classifications.missingPages} / collapsed=${result.classifications.collapsedPages} / low-fidelity=${result.classifications.lowFidelityMatchedPages} / extra=${result.extraGeneratedPages.length}`,
    );
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
    "这轮 9.5 的验收口径已经固定为 storybook + dagger，目标不是泛化地“更像 reference”，而是让两个样本都在最终 `.wiki/*.md` 上达到 `overall_match_rate >= 95%`，同时把差距拆成 missing / collapsed / low-fidelity 三类来收敛。",
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
  lines.push(`- 已落 evidence block 的项目：${projectsWithEvidence}/${results.length}`);
  lines.push(`- 已落 Mermaid 图的项目：${projectsWithDiagrams}/${results.length}`);
  lines.push(`- 平均 citation 密度：generated ${avgGeneratedCitationDensity} / reference ${avgReferenceCitationDensity}`);
  lines.push(
    `- 总体对齐率：${results.map((result) => `${result.project}=${result.overallMatchRate}%`).join("、") || "无"}`,
  );
  lines.push(
    `- 三类差距：${results.map((result) => `${result.project}[missing=${result.classifications.missingPages}, collapse=${result.classifications.collapsedPages}, low-fidelity=${result.classifications.lowFidelityMatchedPages}]`).join("；") || "无"}`,
  );
  lines.push(
    `- 高频缺失专题：${[...topTopicGaps.entries()].sort((left, right) => right[1] - left[1]).slice(0, 6).map(([label, count]) => `${label}(${count})`).join("、") || "无"}`,
  );
  lines.push(
    `- 章节骨架短板：${results.map((result) => `${result.project}=${result.coverage.matchedOutlineShortfall}`).join("、") || "无"}`,
  );
  lines.push(
    `- 英文 raw docs 残留：${results.map((result) => `${result.project}=matched ${result.coverage.matchedEnglishNamingShortfall} / extra ${result.coverage.extraEnglishRawDocsPages}`).join("、") || "无"}`,
  );
  lines.push(
    `- extra generated：${results.map((result) => `${result.project}=${result.extraGeneratedPages.length}`).join("、") || "无"}`,
  );
  lines.push(
    `- stop reasons：${results.map((result) => `${result.project}[budget=${result.stopReasons.budgetStoppedPages}, stalled=${result.stopReasons.stalledPages}, invalid=${result.stopReasons.invalidOutputPages}, failed=${result.stopReasons.providerFailedPages}]`).join("；") || "无"}`,
  );
  lines.push("");
  lines.push("## 下一步建议");
  lines.push("");
  if (results.some((result) => result.classifications.missingPages > 0)) {
    lines.push("- 先补 missing pages：把缺失主题继续映射回独立 KnowledgeUnit，避免再被概览页或大模块页吞并。");
  }
  if (
    results.some((result) => result.classifications.collapsedPages > 0)
    || results.some((result) => result.commonGaps.includes("仍存在明显 page collapse，同一生成页承担多个 reference 页面"))
  ) {
    lines.push("- 继续回收 page collapse：父页只能消费 child digest，禁止大页继续吸收多个 reference 主题。");
  }
  if (results.some((result) => result.classifications.lowFidelityMatchedPages > 0)) {
    lines.push("- 继续压低 low-fidelity：优先补 citation 密度、Mermaid 覆盖率和关键文件提及，避免只命中结构不命中正文。");
  }
  if (results.some((result) => result.stopReasons.budgetStoppedPages > 0 || result.stopReasons.stalledPages > 0)) {
    lines.push("- 继续按 stop reason 收敛 research：预算截断优先调 budget / decomposition，stalled 优先补 delta 线索和 tool/evidence 输入。");
  }
  if (results.some((result) => result.stopReasons.invalidOutputPages > 0 || result.stopReasons.providerFailedPages > 0)) {
    lines.push("- 把 invalid_output / provider_error 单独排查，避免把 provider 失败误判成页面内容质量问题。");
  }
  if (
    results.some((result) => result.coverage.matchedOutlineShortfall > 0)
    || results.some((result) => result.coverage.matchedEnglishNamingShortfall > 0)
    || results.some((result) => result.coverage.extraEnglishRawDocsPages > 0)
  ) {
    lines.push("- docs-backed 页面继续按 reference 骨架和本地化命名收敛，避免回退到英文 raw docs 标题或目录。");
  }
  if (results.some((result) => result.extraGeneratedPages.length > Math.max(5, Math.ceil(result.generatedPageCount * 0.08)))) {
    lines.push("- 若 overall 已稳定，优先回收拆分阈值和派生页面预算，先压 extra generated pages，再做尾差润色。");
  }
  if (lines.at(-1) === "") {
    lines.push("- 当前两项目主要指标已收敛，可继续进入 archive 前的收尾验证。");
  }

  return `${lines.join("\n")}\n`;
}

function writeReports(results, reportDir, summaryPath, optimizationNotesPath) {
  ensureDir(reportDir);
  for (const result of results) {
    writeFileSync(path.join(reportDir, `${result.project}.md`), renderProjectReport(result));
    writeFileSync(path.join(reportDir, `${result.project}-gap-ledger.md`), renderGapLedger(result));
  }
  writeFileSync(summaryPath, renderSummary(results));
  writeFileSync(optimizationNotesPath, renderOptimizationNotes(results));
}

function parseCliArgs(argv) {
  const names = [];
  let jobs;
  let runMode = "cold";
  let change = DEFAULT_CHANGE;
  let skipInit = false;
  let initTimeoutMinutes = DEFAULT_INIT_TIMEOUT_MS / 60000;

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
    if (arg === "--skip-init") {
      skipInit = true;
      continue;
    }
    if (arg === "--init-timeout-minutes") {
      initTimeoutMinutes = argv[index + 1] || initTimeoutMinutes;
      index++;
      continue;
    }
    names.push(arg);
  }

  const parsedTimeoutMinutes = Number(initTimeoutMinutes);
  const initTimeoutMs =
    Number.isFinite(parsedTimeoutMinutes) && parsedTimeoutMinutes > 0
      ? Math.floor(parsedTimeoutMinutes * 60_000)
      : DEFAULT_INIT_TIMEOUT_MS;

  return { change, initTimeoutMs, jobs, names, runMode, skipInit };
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
    const run = argv.skipInit
      ? {
          label: "reuse",
          cacheMode: "preserve",
          usage: null,
        }
      : await runInitForProject(project, progressPrinter, argv.runMode, argv.initTimeoutMs);
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
    initTimeoutMs: argv.initTimeoutMs,
    runMode: argv.runMode,
    projects: results.map((result) => ({
      project: result.project,
      runMode: result.runLabel,
      generatedPageCount: result.generatedPageCount,
      referencePageCount: result.referencePageCount,
      matchedCount: result.matchedCount,
      overallMatchRate: result.overallMatchRate,
      missingCount: result.missingCount,
      collapsedPages: result.classifications.collapsedPages,
      lowFidelityMatchedPages: result.classifications.lowFidelityMatchedPages,
      extraGeneratedPages: result.extraGeneratedPages.length,
      budgetStoppedPages: result.stopReasons.budgetStoppedPages,
      stalledPages: result.stopReasons.stalledPages,
      invalidOutputPages: result.stopReasons.invalidOutputPages,
      providerFailedPages: result.stopReasons.providerFailedPages,
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

export {
  comparePagePair,
  extractOutlineSkeleton,
  isEnglishRawDocsPage,
  normalizeOutlineSectionKey,
  outlineOverlapRate,
};
