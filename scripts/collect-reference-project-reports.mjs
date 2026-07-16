/* eslint-disable regexp/no-dupe-disjunctions, regexp/no-unused-capturing-group, no-control-regex */
/**
 * 针对带 reference 的测试项目，批量执行带 provider 的 init，
 * 并把 generated `.wiki/*.md` 与 reference content 目录下的 Markdown 文件
 * 做逐项目、逐文件的结构化对比，输出到当前 UniSpec change 目录。
 *
 * 默认输出：
 * - .spec/changes/<change>/reference-project-reports/*.md
 * - .spec/changes/<change>/reference-project-reports/_summary.md
 * - .spec/changes/<change>/reference-project-reports/_optimization-notes.md
 *
 * 用法：
 *   node scripts/collect-reference-project-reports.mjs --acceptance-plan <plan.json>
 *   node scripts/collect-reference-project-reports.mjs --acceptance-plan <plan.json> --jobs 2
 *   node scripts/collect-reference-project-reports.mjs --acceptance-plan <plan.json> chi axum
 */

import { execFileSync } from "node:child_process";
import {
  cpSync,
  existsSync,
  mkdirSync,
  readdirSync,
  readFileSync,
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
import {
  createAcceptancePlan,
} from "./testing/core-scenario-acceptance.mjs";
import {
  aggregateGateResults,
} from "./testing/quality-gates.mjs";
import {
  analyzeReferenceFidelity,
  readMarkdownPages,
} from "./testing/reference-fidelity.mjs";
import {
  inspectWikiRuntime,
} from "./testing/wiki-runtime-inspection.mjs";

const REFERENCE_DIR = path.join(TMP_DIR, "reference");
const DEFAULT_CHANGE = "iteration-9-6-fidelity-gates-and-reference-report-hardening";
const REAL_REPO_MAP = {
  aLocal: "E:\\project\\aLocal",
};

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
  "runtime": "Runtime",
  "compiler-pipeline": "CompilerPipeline",
  "api-surface": "ApiSurface",
  "config-surface": "ConfigSurface",
  "docs-guide": "DocsGuide",
  "testing": "Testing",
  "example-tutorial": "ExampleTutorial",
  "troubleshooting": "Troubleshooting",
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
    .sort();
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
    (devContext) =>
      callCoreStreaming(
        devContext.command({ action: "init", repoRoot: repoRootArg }),
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
        const shouldPrint
          = event.processed === 0
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

function inferDecompositionSignals(page) {
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

function containsNonAscii(value) {
  return /[^\x00-\x7F]/.test(value);
}

function isEnglishRawDocsPage(page) {
  const normalizedPath = page.relativePath.replaceAll("\\", "/");
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
  return path.posix.basename(
    String(filePath)
      .replaceAll("\\", "/")
      .replace(/#L\d+(?:-L?\d+)?$/i, ""),
  );
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
    item.matched
    && (
        (item.reuseCount ?? 1) > 1
        || item.notes?.some((note) => COLLAPSE_NOTE_PATTERNS.includes(note))
      ),
  ).length;
  const lowFidelityMatchedPages = comparisons.filter((item) =>
    item.matched
    && item.notes?.some((note) => LOW_FIDELITY_NOTE_PATTERNS.includes(note)),
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

function computeOverallMatchRate(matchedCount, referencePageCount) {
  if (referencePageCount === 0) {
    return null;
  }
  return Number(((matchedCount / referencePageCount) * 100).toFixed(2));
}

function buildStabilityMetrics(referenceWikiDir, generatedWikiDir, reruns) {
  if (reruns <= 1) {
    return null;
  }

  const runs = [];
  for (let index = 0; index < reruns; index++) {
    const rerun = analyzeReferenceFidelity({
      referenceDir: referenceWikiDir,
      generatedDir: generatedWikiDir,
    });
    runs.push({
      label: `warm-rerun-${index + 1}`,
      reuseOverage: rerun.reuseOverage,
      medianSkeletonFidelity: rerun.medianSkeletonScore,
      medianKeySourceCoverage: rerun.medianKeySourceCoverage,
    });
  }

  const reuseValues = runs.map((item) => item.reuseOverage);
  const skeletonValues = runs.map((item) => item.medianSkeletonFidelity ?? 0);
  const keySourceValues = runs.map((item) => item.medianKeySourceCoverage ?? 0);
  const deltas = {
    reuseOverage: Math.max(...reuseValues) - Math.min(...reuseValues),
    medianSkeletonFidelity: Number((Math.max(...skeletonValues) - Math.min(...skeletonValues)).toFixed(4)),
    medianKeySourceCoverage: Number((Math.max(...keySourceValues) - Math.min(...keySourceValues)).toFixed(4)),
  };

  return {
    reruns: runs,
    deltas,
    stable:
      deltas.reuseOverage <= 0
      && deltas.medianSkeletonFidelity <= 0.02
      && deltas.medianKeySourceCoverage <= 0.02,
  };
}

/**
 * 收集单个 reference 项目的 runtime、fidelity 与报告聚合结果。
 *
 * 这里先做 runtime gate，再决定是否进入 Markdown fidelity 分析；
 * `runtime_incomplete` 必须直接 hard gate，不能继续伪装成低质量页面。
 *
 * @param project 测试项目名。
 * @param run 本次 collect 对应的运行标签与 usage 摘要。
 * @param options 额外控制项；用于区分 requested run mode 与 warm stability 次数。
 * @returns 返回单项目的结构化报告快照。
 */
function collectProject(project, run, options = {}) {
  const projectRoot = path.join(TEST_DIR, project);
  const generatedWikiDir = path.join(projectRoot, ".wiki");
  const referenceWikiDir = path.join(REFERENCE_DIR, project, "content");
  const runtimeSnapshot = inspectWikiRuntime(generatedWikiDir);
  const referencePages = readMarkdownPages(referenceWikiDir);
  const stopReasons = readResearchStopMetrics(generatedWikiDir);
  const run_metrics = {
    run_mode: options.runMode ?? run.label,
    run_label: run.label,
    cache_mode: run.cacheMode,
    baseline_mode: classifyBaselineMode(run, options),
    usage: run.usage,
    page_research_requests: promptCount(run.usage, "page_research"),
    page_enrichment_requests: promptCount(run.usage, "page_enrichment"),
  };
  const runtime_metrics = {
    runtime_state: runtimeSnapshot.runtimeState,
    baseline_class: runtimeSnapshot.baselineClass,
    incomplete_reason: runtimeSnapshot.incompleteReason,
    db_counts: runtimeSnapshot.dbCounts,
    checkpoint: runtimeSnapshot.checkpoint,
    runtime_summary: runtimeSnapshot.runtimeSummary,
    gate_summary: runtimeSnapshot.runtimeGateSummary,
    parent_contract: runtimeSnapshot.parentContract,
    compose_diagnostics: runtimeSnapshot.composeDiagnostics,
    stop_reasons: stopReasons,
  };

  if (runtimeSnapshot.runtimeState !== "ready") {
    return {
      project,
      status: "runtime_incomplete",
      runLabel: run.label,
      cacheMode: run.cacheMode,
      usage: run.usage,
      run_metrics,
      runtime_metrics,
      generatedPageCount: runtimeSnapshot.markdownPageCount,
      referencePageCount: referencePages.length,
      matchedCount: 0,
      missingCount: 0,
      extraGeneratedPages: [],
      comparisons: [],
      reusedGeneratedPages: [],
      topReuseOffenders: [],
      skeletonLowestPages: [],
      keySourceLowestPages: [],
      coverage: null,
      classifications: null,
      decomposition: null,
      overallMatchRate: null,
      fidelity_metrics: null,
      stability: null,
      page_runtime: runtimeSnapshot.pageRuntimeByPath,
      commonGaps: [
        `runtime 仍处于 ${runtimeSnapshot.runtimeState}，当前只能做诊断，不能纳入 fidelity 验收基线`,
      ],
    };
  }

  const generatedPages = readMarkdownPages(generatedWikiDir);
  const fidelity = analyzeReferenceFidelity({
    generatedPages,
    referencePages,
  });
  const coverage = summarizeCoverage(fidelity.comparisons, generatedPages, referencePages);
  const classifications = classificationCounts(fidelity.comparisons);
  const decomposition = summarizeDecompositionCoverage(
    generatedPages,
    referencePages,
    fidelity.comparisons,
  );
  const overallMatchRate = computeOverallMatchRate(fidelity.matchedCount, referencePages.length);
  const commonGaps = summarizeProjectGaps(fidelity.comparisons, generatedPages, referencePages, coverage);

  return {
    project,
    status: "ready",
    runLabel: run.label,
    cacheMode: run.cacheMode,
    usage: run.usage,
    run_metrics,
    runtime_metrics,
    generatedPageCount: generatedPages.length,
    referencePageCount: referencePages.length,
    matchedCount: fidelity.matchedCount,
    missingCount: fidelity.missingCount,
    extraGeneratedPages: fidelity.extraGeneratedPages,
    comparisons: fidelity.comparisons,
    reusedGeneratedPages: fidelity.topReuseOffenders,
    topReuseOffenders: fidelity.topReuseOffenders,
    skeletonLowestPages: fidelity.skeletonLowestPages,
    keySourceLowestPages: fidelity.keySourceLowestPages,
    coverage,
    classifications,
    decomposition,
    overallMatchRate,
    page_runtime: runtimeSnapshot.pageRuntimeByPath,
    fidelity_metrics: {
      overall_match_rate: overallMatchRate,
      matched_pages: fidelity.matchedCount,
      missing_pages: fidelity.missingCount,
      collapsed_pages: fidelity.collapsedPages,
      reuse_pages: fidelity.reusePages,
      severe_reuse_pages: fidelity.severeReusePages,
      reuse_overage: fidelity.reuseOverage,
      low_fidelity_matched_pages: fidelity.lowFidelityMatchedPages,
      median_skeleton_fidelity: fidelity.medianSkeletonScore,
      median_key_source_coverage: fidelity.medianKeySourceCoverage,
    },
    stability:
      (run.label === "warm" || run.label === "reuse")
      && (options.stabilityReruns ?? 0) > 1
        ? buildStabilityMetrics(referenceWikiDir, generatedWikiDir, options.stabilityReruns)
        : null,
    commonGaps,
    stopReasons,
  };
}

function describeKnowledgeUnitBinding(result, generatedPath) {
  const binding = result.page_runtime?.[generatedPath];
  if (!binding) {
    return "n/a";
  }
  return [
    `unit_id=${binding.unitId ?? "n/a"}`,
    `unit_type=${binding.unitType ?? "n/a"}`,
    `domain_id=${binding.domainId ?? "n/a"}`,
    `readiness=${binding.readinessStatus || "n/a"}`,
    `child_digests=${binding.childDigestCount ?? 0}`,
    `planned_key_sources=${binding.plannedKeySourceCount ?? 0}`,
    `grounded_key_sources=${binding.groundedKeySourceCount ?? 0}`,
    `grounding_refs=${binding.sectionGroundingRefCount ?? 0}`,
  ].join(", ");
}

function classifyBaselineMode(run, options = {}) {
  if (options.skipInit) {
    return "warm_runtime_reuse";
  }
  if (run.label === "warm" || run.label === "reuse" || run.cacheMode === "preserve") {
    return "warm_runtime_reuse";
  }
  return "fresh_init_baseline";
}

function countReusedGeneratedPages(comparisons) {
  const precomputed = comparisons
    .filter((comparison) => comparison.matched && (comparison.reuseCount ?? 1) > 1)
    .map((comparison) => ({
      generatedPath: comparison.generatedPath,
      count: comparison.reuseCount,
    }));
  if (precomputed.length > 0) {
    return [...new Map(precomputed.map((item) => [item.generatedPath, item])).values()];
  }

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
    page.relativePath.startsWith("专题/repo-archetype/"),
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
    matchedKeySourceShortfall: matched.filter((item) => (item.keySource?.coverage ?? 1) < 0.7).length,
    matchedEnglishNamingShortfall: matched.filter((item) => item.notes.includes("文件名仍偏向英文 raw docs")).length,
    extraEnglishRawDocsPages: generatedPages.filter((page) =>
      page.englishRawDocsLike && !comparisons.some((comparison) => comparison.generatedPath === page.relativePath),
    ).length,
    topicLabels: [...topicLabelCounts.entries()].sort((left, right) => right[1] - left[1]),
    missingTopicLabels: [...missingTopicLabels.entries()].sort((left, right) => right[1] - left[1]),
  };
}

function summarizeProjectGaps(comparisons, generatedPages, referencePages, coverage) {
  const gaps = [];
  const matched = comparisons.filter((item) => item.matched);
  const reusedGeneratedPages = countReusedGeneratedPages(comparisons).length;
  const citationDensityGap
    = coverage.referenceCitationDensity === 0
      ? 0
      : 1 - (coverage.generatedCitationDensity / coverage.referenceCitationDensity);
  const diagramCoverageGap
    = coverage.referenceDiagramPages === 0
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
      || note.includes("图表少于 reference"),
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
  if (result.status !== "ready") {
    return [{
      symptom: "runtime_incomplete",
      metric: [
        `runtime_state=${result.runtime_metrics.runtime_state}`,
        `reason=${result.runtime_metrics.incomplete_reason ?? "n/a"}`,
        `knowledge_units=${result.runtime_metrics.db_counts.knowledge_units}`,
        `research_cache=${result.runtime_metrics.db_counts.research_cache}`,
        `wiki_pages=${result.runtime_metrics.db_counts.wiki_pages}`,
      ].join(" / "),
      offendingPages: [],
      contractHypothesis: "assemble / runtime gate / pipeline checkpoint",
    }];
  }

  const entries = [];
  for (const comparison of result.comparisons) {
    const pageDiagnostics = comparison.generatedPath
      ? result.page_runtime?.[comparison.generatedPath] ?? null
      : null;
    if (!comparison.matched) {
      const entry = ledgerEntry("missing", comparison);
      entries.push({
        symptom: "missing_page",
        metric: "missing_pages",
        offendingPages: [comparison.referencePath],
        contractHypothesis: `${entry.contract} / typed surface bundle / leaf decomposition policy`,
      });
      continue;
    }

    if ((comparison.reuseCount ?? 1) > 1) {
      entries.push({
        symptom: "many_to_one_reuse",
        metric: `reuse_count=${comparison.reuseCount}`,
        offendingPages: [comparison.referencePath, comparison.generatedPath],
        contractHypothesis: "planner.collapse_guard / leaf decomposition policy / parent-consume-child boundary",
      });
    }
    if ((comparison.skeletonScore ?? 1) < 0.8) {
      entries.push({
        symptom: "skeleton_shortfall",
        metric: `skeleton_score=${Number(comparison.skeletonScore ?? 0).toFixed(2)}`,
        offendingPages: [comparison.referencePath, comparison.generatedPath],
        contractHypothesis: skeletonContractHypothesis(pageDiagnostics),
      });
    }
    if ((comparison.keySource?.coverage ?? 1) < 0.7) {
      entries.push({
        symptom: "key_source_shortfall",
        metric: `key_source_coverage=${Number(comparison.keySource?.coverage ?? 0).toFixed(2)}`,
        offendingPages: [
          comparison.referencePath,
          comparison.generatedPath,
          ...(comparison.keySource?.missingSources ?? []).slice(0, 5).map((item) => `missing:${item}`),
        ],
        contractHypothesis: keySourceContractHypothesis(pageDiagnostics, comparison),
      });
    }
  }

  return entries;
}

function sourceMatches(left, right) {
  const normalizedLeft = String(left ?? "")
    .replaceAll("\\", "/")
    .replace(/#L\d+(?:-L?\d+)?$/i, "")
    .toLowerCase();
  const normalizedRight = String(right ?? "")
    .replaceAll("\\", "/")
    .replace(/#L\d+(?:-L?\d+)?$/i, "")
    .toLowerCase();
  if (!normalizedLeft || !normalizedRight) {
    return false;
  }
  if (normalizedLeft === normalizedRight) {
    return true;
  }
  return path.posix.basename(normalizedLeft) === path.posix.basename(normalizedRight);
}

/**
 * 根据 compose diagnostics 判断 skeleton 缺口更偏向 research 还是 compose 落页。
 *
 * @param {object | null | undefined} pageDiagnostics 页面级 runtime 诊断。
 * @returns {string} 返回可直接写入 gap ledger 的 contract hypothesis。
 */
function skeletonContractHypothesis(pageDiagnostics) {
  if (!pageDiagnostics?.skeletonProfileKey) {
    return "research.skeleton_profile missing";
  }
  if ((pageDiagnostics.sectionGroundingRefCount ?? 0) === 0) {
    return "compose.section_grounding_refs missing";
  }
  return "compose section contract / renderer 落页没有保住 research skeleton";
}

/**
 * 根据 planned/grounded key sources 与 reference 缺口，定位 key-source fidelity 的主链断点。
 *
 * @param {object | null | undefined} pageDiagnostics 页面级 runtime 诊断。
 * @param {object} comparison 当前 generated/reference 的对比结果。
 * @returns {string} 返回可直接写入 gap ledger 的 contract hypothesis。
 */
function keySourceContractHypothesis(pageDiagnostics, comparison) {
  const planned = pageDiagnostics?.plannedKeySources ?? [];
  const grounded = pageDiagnostics?.groundedKeySources ?? [];
  const missing = comparison.keySource?.missingSources ?? [];

  if (planned.length === 0) {
    return "research.key_source_clusters / planned_key_sources missing";
  }
  if ((pageDiagnostics?.sectionGroundingRefCount ?? 0) === 0 || grounded.length === 0) {
    return "compose.section_grounding_refs / grounded_key_sources missing";
  }

  const plannedMissesReference = missing.some((source) =>
    !planned.some((candidate) => sourceMatches(candidate, source)),
  );
  if (plannedMissesReference) {
    return "planner/research planned_key_sources drift";
  }

  const groundedMissesPlanned = missing.some((source) =>
    !grounded.some((candidate) => sourceMatches(candidate, source)),
  );
  if (groundedMissesPlanned || pageDiagnostics?.groundingGap) {
    return "compose grounded_key_sources 未落到最终 section";
  }

  return "research / compose source grounding contract drift";
}

function formatPercent(value) {
  return value == null ? "N/A" : `${Number(value).toFixed(2)}%`;
}

function formatRatio(value) {
  return value == null ? "N/A" : Number(value).toFixed(2);
}

function referenceThresholds(plan) {
  const thresholds = plan.thresholds;
  const numericFields = [
    "overall_match_rate_min",
    "reuse_overage_max",
    "median_skeleton_fidelity_min",
    "median_key_source_coverage_min",
  ];
  for (const field of numericFields) {
    if (typeof thresholds?.[field] !== "number" || !Number.isFinite(thresholds[field]))
      throw new TypeError(`reference acceptance plan requires numeric threshold: ${field}`);
  }
  if (typeof thresholds.require_warm_stability !== "boolean")
    throw new TypeError("reference acceptance plan requires boolean threshold: require_warm_stability");
  return thresholds;
}

function gateDecision(result, thresholds) {
  if (result.status !== "ready") {
    return {
      decision: "blocker",
      reason: `runtime_state=${result.runtime_metrics.runtime_state}`,
    };
  }

  const pass
    = (result.fidelity_metrics?.overall_match_rate ?? 0) >= thresholds.overall_match_rate_min
      && (result.fidelity_metrics?.reuse_overage ?? 0) <= thresholds.reuse_overage_max
      && (result.fidelity_metrics?.median_skeleton_fidelity ?? 0) >= thresholds.median_skeleton_fidelity_min
      && (result.fidelity_metrics?.median_key_source_coverage ?? 0) >= thresholds.median_key_source_coverage_min
      && (!thresholds.require_warm_stability || result.stability?.stable === true);
  return {
    decision: pass ? "pass" : "blocker",
    reason: [
      `overall=${formatPercent(result.fidelity_metrics?.overall_match_rate ?? null)}`,
      `reuse_overage=${result.fidelity_metrics?.reuse_overage ?? "N/A"}`,
      `median_skeleton=${formatRatio(result.fidelity_metrics?.median_skeleton_fidelity ?? null)}`,
      `median_key_source=${formatRatio(result.fidelity_metrics?.median_key_source_coverage ?? null)}`,
      `warm_stable=${result.stability?.stable ?? "n/a"}`,
    ].join(" / "),
  };
}

export function buildPrimaryGateSummary(results, options = {}) {
  const acceptancePlan = createAcceptancePlan(options.acceptancePlan);
  const thresholds = referenceThresholds(acceptancePlan);
  if (results.length > 0) {
    const actualProjects = [...new Set(results.map(result => result.project))].sort();
    if (JSON.stringify(actualProjects) !== JSON.stringify([...acceptancePlan.primary_fixtures].sort()))
      throw new TypeError("reference results must match acceptance plan primary_fixtures");
  }
  const projectResults = results.map((result) => {
    const gate = gateDecision(result, thresholds);
    return {
      project: result.project,
      status: result.status,
      gate_label: gate.decision,
      gate_reason: gate.reason,
      runtime_state: result.runtime_metrics.runtime_state,
    };
  });
  const blockingProjects = projectResults.filter((project) => project.gate_label !== "pass");
  const decision = blockingProjects.length > 0 ? "blocker" : "pass";

  const failures = blockingProjects.map(project => ({
    failure_id: `reference-fidelity:${project.project}`,
    owner_gate_id: "reference_fidelity_primary",
    source_ref: project.project,
    assertion_ref: project.gate_reason,
    evidence_refs: [project.project],
  }));
  const failureRefs = failures.map(failure => failure.failure_id);
  const v2Summary = aggregateGateResults({
    plan: acceptancePlan,
    gate_results: projectResults.length > 0
      ? {
          reference_fidelity_primary: {
            decision,
            evidence_refs: projectResults.map(project => project.project),
            failure_refs: failureRefs,
          },
        }
      : {},
    failures,
    diagnostics: [],
    scenario_results: [],
  });

  return {
    ...v2Summary,
    gate_level: "primary_gate",
    gate_scope: "reference_fidelity",
    command: "node scripts/collect-reference-project-reports.mjs",
    samples: [...acceptancePlan.primary_fixtures],
    totals: {
      total_projects: results.length,
      passed_projects: results.length - blockingProjects.length,
      failed_projects: blockingProjects.length,
      skipped_projects: 0,
      diagnostic_projects: 0,
      total_assertions: 0,
      failed_assertions: 0,
    },
    project_results: projectResults,
    notes: [
      "reference fidelity 报告是 primary gate 输入，不取代 formal artifact / restore / query route / status gates。",
    ],
    fidelity_input_only: true,
    required_companion_gates: [...acceptancePlan.required_gates],
  };
}

function summarizeStabilitySeries(values) {
  if (values.length === 0 || values.some((value) => value == null)) {
    return {
      min: null,
      max: null,
      delta: null,
    };
  }

  const min = Math.min(...values);
  const max = Math.max(...values);
  return {
    min: Number(min.toFixed(4)),
    max: Number(max.toFixed(4)),
    delta: Number((max - min).toFixed(4)),
  };
}

/**
 * 对同一批项目做 warm report 重跑，检查关键 fidelity 指标是否抖动。
 *
 * 这里复用 `collectProject()`，确保 stability 与正式报告走同一套 gate 和聚合口径。
 *
 * @param projects 需要重跑的项目列表。
 * @param repeats 每个项目的 warm collect 次数。
 * @returns 返回稳定性摘要；次数不足两次时返回 `null`。
 */
function buildWarmStability(projects, repeats) {
  if (repeats < 2) {
    return null;
  }

  const samplesByProject = new Map(projects.map((project) => [project, []]));
  for (let index = 0; index < repeats; index++) {
    for (const project of projects) {
      const sample = collectProject(project, {
        label: "warm-stability",
        cacheMode: "preserve",
        usage: null,
      });
      samplesByProject.get(project).push({
        runtimeState: sample.runtime_metrics.runtime_state,
        reuseOverage: sample.fidelity_metrics?.reuse_overage ?? null,
        medianSkeletonFidelity: sample.fidelity_metrics?.median_skeleton_fidelity ?? null,
        medianKeySourceCoverage: sample.fidelity_metrics?.median_key_source_coverage ?? null,
      });
    }
  }

  return {
    repeats,
    projects: projects.map((project) => {
      const samples = samplesByProject.get(project) ?? [];
      const reuse = summarizeStabilitySeries(samples.map((sample) => sample.reuseOverage));
      const skeleton = summarizeStabilitySeries(samples.map((sample) => sample.medianSkeletonFidelity));
      const keySource = summarizeStabilitySeries(samples.map((sample) => sample.medianKeySourceCoverage));
      const stable
        = samples.every((sample) => sample.runtimeState === "ready")
          && (reuse.delta ?? Number.POSITIVE_INFINITY) <= 0
          && (skeleton.delta ?? Number.POSITIVE_INFINITY) <= 0.02
          && (keySource.delta ?? Number.POSITIVE_INFINITY) <= 0.02;
      return {
        project,
        stable,
        samples,
        deltas: {
          reuseOverage: reuse,
          medianSkeletonFidelity: skeleton,
          medianKeySourceCoverage: keySource,
        },
      };
    }),
  };
}

function renderGapLedger(result) {
  const ledger = buildGapLedger(result);
  const lines = [
    `# ${result.project} Gap Ledger`,
    "",
    "## 当前状态",
    "",
    `- status：${result.status}`,
    `- baseline_mode：${result.run_metrics.baseline_mode}`,
    `- runtime_state：${result.runtime_metrics.runtime_state}`,
    `- baseline_class：${result.runtime_metrics.baseline_class}`,
    "",
    "| Symptom | Metric | Offending Pages | Contract Hypothesis |",
    "| --- | --- | --- | --- |",
  ];

  for (const entry of ledger) {
    lines.push(
      `| ${entry.symptom} | ${entry.metric} | ${entry.offendingPages.join("<br>") || "n/a"} | ${entry.contractHypothesis} |`,
    );
  }

  return `${lines.join("\n")}\n`;
}

/**
 * 将单项目快照渲染为 Markdown 报告。
 *
 * 报告必须先展示 run/runtime 两类指标，再决定是否进入 fidelity 结论，
 * 避免把 warm usage、cache 历史和 runtime 完整性混成一栏。
 *
 * @param result 单项目结构化结果。
 * @param gate 已由 acceptance plan 阈值计算的项目 gate 结果。
 * @returns 返回项目 Markdown 报告。
 */
function renderProjectReport(result, gate) {
  const lines = [
    `# ${result.project} Reference Fidelity Report`,
    "",
    `- project：${result.project}`,
    `- status：${result.status}`,
    `- generated_pages：${result.generatedPageCount}`,
    `- reference_pages：${result.referencePageCount}`,
    "",
    "## Run Metrics",
    "",
    `- run_label：${result.run_metrics.run_label}`,
    `- cache_mode：${result.run_metrics.cache_mode}`,
    `- baseline_mode：${result.run_metrics.baseline_mode}`,
    `- usage：requests=${result.run_metrics.usage?.request_count ?? 0}, total_tokens=${result.run_metrics.usage?.total_tokens ?? 0}, page_research=${result.run_metrics.page_research_requests}, page_enrichment=${result.run_metrics.page_enrichment_requests}`,
    "",
    "## Runtime Metrics",
    "",
    `- runtime_state：${result.runtime_metrics.runtime_state}`,
    `- baseline_class：${result.runtime_metrics.baseline_class}`,
    `- incomplete_reason：${result.runtime_metrics.incomplete_reason ?? "n/a"}`,
    `- db_counts：knowledge_units=${result.runtime_metrics.db_counts.knowledge_units}, knowledge_domains=${result.runtime_metrics.db_counts.knowledge_domains}, research_cache=${result.runtime_metrics.db_counts.research_cache}, page_digests=${result.runtime_metrics.db_counts.page_digests}, page_drafts=${result.runtime_metrics.db_counts.page_drafts}, unit_runtime_gates=${result.runtime_metrics.db_counts.unit_runtime_gates}, wiki_pages=${result.runtime_metrics.db_counts.wiki_pages}, pipeline_checkpoint=${result.runtime_metrics.db_counts.pipeline_checkpoint}`,
    `- pipeline_runtime_summary：state=${result.runtime_metrics.runtime_summary?.runtime_state ?? "missing"}, researched=${result.runtime_metrics.runtime_summary?.researched_units ?? 0}, compose_ready=${result.runtime_metrics.runtime_summary?.compose_ready_units ?? 0}, composed=${result.runtime_metrics.runtime_summary?.composed_units ?? 0}, assembled=${result.runtime_metrics.runtime_summary?.assembled_pages ?? 0}`,
    `- unit_runtime_gates：total=${result.runtime_metrics.gate_summary?.total ?? 0}, compose_ready=${result.runtime_metrics.gate_summary?.composeReady ?? 0}, compose_pending=${result.runtime_metrics.gate_summary?.composePending ?? 0}, compose_blocked=${result.runtime_metrics.gate_summary?.composeBlocked ?? 0}, assemble_done=${result.runtime_metrics.gate_summary?.assembleDone ?? 0}`,
    `- parent_contract：parents=${result.runtime_metrics.parent_contract?.parentPages ?? 0}, compose_ready_parents=${result.runtime_metrics.parent_contract?.composeReadyParents ?? 0}, child_digest_parents=${result.runtime_metrics.parent_contract?.childDigestParents ?? 0}, missing_readiness_parents=${result.runtime_metrics.parent_contract?.missingReadinessParents ?? 0}`,
    `- compose_diagnostics：digest_pages=${result.runtime_metrics.compose_diagnostics?.digestPages ?? 0}, skeleton_profile_pages=${result.runtime_metrics.compose_diagnostics?.pagesWithSkeletonProfile ?? 0}, grounding_ref_pages=${result.runtime_metrics.compose_diagnostics?.pagesWithSectionGroundingRefs ?? 0}, planned_key_source_pages=${result.runtime_metrics.compose_diagnostics?.pagesWithPlannedKeySources ?? 0}, grounded_key_source_pages=${result.runtime_metrics.compose_diagnostics?.pagesWithGroundedKeySources ?? 0}, grounding_gap_pages=${result.runtime_metrics.compose_diagnostics?.pagesWithGroundingGap ?? 0}`,
    `- stop_reasons：${result.runtime_metrics.stop_reasons.stopReasonCounts.map(([reason, count]) => `${reason}(${count})`).join("、") || "无"}`,
  ];

  if (result.runtime_metrics.checkpoint) {
    lines.push(
      `- checkpoint：stage=${result.runtime_metrics.checkpoint.stage || "unknown"}, target=${result.runtime_metrics.checkpoint.targetId || "n/a"}`,
    );
  }
  lines.push("");

  if (result.status !== "ready") {
    lines.push("## Fidelity Gate");
    lines.push("");
    lines.push("- 当前 runtime 不是 ready，本次报告只保留诊断摘要，不输出 overall / reuse / skeleton / key-source 汇总值。");
    lines.push("");
    lines.push("## 当前结论");
    lines.push("");
    for (const gap of result.commonGaps) {
      lines.push(`- ${gap}`);
    }
    lines.push("");
    return `${lines.join("\n")}\n`;
  }

  lines.push("## Fidelity Gate");
  lines.push("");
  lines.push(`- decision：${gate.gate_label}`);
  lines.push(`- reason：${gate.gate_reason}`);
  lines.push(`- overall_match_rate：${formatPercent(result.fidelity_metrics.overall_match_rate)}`);
  lines.push(`- reuse_overage：${result.fidelity_metrics.reuse_overage}`);
  lines.push(`- median_skeleton_fidelity：${formatRatio(result.fidelity_metrics.median_skeleton_fidelity)}`);
  lines.push(`- median_key_source_coverage：${formatRatio(result.fidelity_metrics.median_key_source_coverage)}`);
  lines.push("");
  lines.push("## 四个专项问题");
  lines.push("");
  lines.push(`- 页数是否接近 reference：matched ${result.fidelity_metrics.matched_pages}/${result.referencePageCount}，missing=${result.fidelity_metrics.missing_pages}`);
  lines.push(`- 是否存在 coarse page reuse：reuse_pages=${result.fidelity_metrics.reuse_pages}，severe_reuse_pages=${result.fidelity_metrics.severe_reuse_pages}，reuse_overage=${result.fidelity_metrics.reuse_overage}`);
  lines.push(`- docs-backed 页面是否具备 reference 式骨架：median=${formatRatio(result.fidelity_metrics.median_skeleton_fidelity)}，shortfall=${result.coverage?.matchedOutlineShortfall ?? 0}`);
  lines.push(`- 正文是否覆盖关键文件：median=${formatRatio(result.fidelity_metrics.median_key_source_coverage)}，shortfall=${result.coverage?.matchedKeySourceShortfall ?? 0}`);
  lines.push("");
  lines.push("## Top Reuse Offenders");
  lines.push("");
  if (result.topReuseOffenders.length === 0) {
    lines.push("- 当前没有 many-to-one reuse offender。");
  } else {
    for (const item of result.topReuseOffenders.slice(0, 10)) {
      lines.push(`- ${item.generatedPath}：reuse_count=${item.count}；${describeKnowledgeUnitBinding(result, item.generatedPath)}`);
    }
  }
  lines.push("");
  lines.push("## Skeleton Lowest Pages");
  lines.push("");
  for (const item of result.skeletonLowestPages.slice(0, 5)) {
    lines.push(`- ${item.referencePath} -> ${item.generatedPath}：skeleton=${formatRatio(item.skeletonScore)}`);
  }
  if (result.skeletonLowestPages.length === 0) {
    lines.push("- 无");
  }
  lines.push("");
  lines.push("## Key Source Lowest Pages");
  lines.push("");
  for (const item of result.keySourceLowestPages.slice(0, 5)) {
    lines.push(`- ${item.referencePath} -> ${item.generatedPath}：coverage=${formatRatio(item.keySource.coverage)}，missing=${item.keySource.missingSources.slice(0, 6).join("、") || "无"}`);
  }
  if (result.keySourceLowestPages.length === 0) {
    lines.push("- 无");
  }

  if (result.stability) {
    lines.push("");
    lines.push("## Warm Stability");
    lines.push("");
    lines.push(`- stable：${result.stability.stable}`);
    lines.push(`- delta_reuse_overage：${result.stability.deltas.reuseOverage.delta ?? "N/A"}`);
    lines.push(`- delta_median_skeleton：${formatRatio(result.stability.deltas.medianSkeletonFidelity.delta ?? null)}`);
    lines.push(`- delta_median_key_source：${formatRatio(result.stability.deltas.medianKeySourceCoverage.delta ?? null)}`);
  }

  lines.push("");
  lines.push("## 覆盖统计");
  lines.push("");
  lines.push(`- topic coverage：generated ${result.coverage.generatedTopicPages} / reference ${result.coverage.referenceTopicPages}`);
  lines.push(`- evidence coverage：generated ${result.coverage.generatedEvidencePages} / reference ${result.coverage.referenceEvidencePages}`);
  lines.push(`- citation density：generated ${result.coverage.generatedCitationDensity} / reference ${result.coverage.referenceCitationDensity}`);
  lines.push(`- diagram coverage：generated ${result.coverage.generatedDiagramPages} / reference ${result.coverage.referenceDiagramPages}`);
  lines.push(`- 高频缺失专题：${result.coverage.missingTopicLabels.slice(0, 6).map(([label, count]) => `${label}(${count})`).join("、") || "无"}`);

  lines.push("");
  lines.push("## Decomposition 命中");
  lines.push("");
  lines.push(`- generated：${result.decomposition.generated.map(([label, count]) => `${label}(${count})`).join("、") || "无"}`);
  lines.push(`- reference：${result.decomposition.reference.map(([label, count]) => `${label}(${count})`).join("、") || "无"}`);
  lines.push(`- 高频缺口：${result.decomposition.missingSignals.slice(0, 6).map(([label, count]) => `${label}(${count})`).join("、") || "无"}`);

  lines.push("");
  lines.push("## 项目结论");
  lines.push("");
  for (const gap of result.commonGaps) {
    lines.push(`- ${gap}`);
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
    lines.push(`- KnowledgeUnit：${describeKnowledgeUnitBinding(result, comparison.generatedPath)}`);
    lines.push(`- reuse_count：${comparison.reuseCount}`);
    lines.push(`- skeleton_score：${formatRatio(comparison.skeletonScore)}`);
    lines.push(`- key_source_coverage：${formatRatio(comparison.keySource?.coverage ?? null)}`);
    lines.push(`- missing_key_sources：${comparison.keySource?.missingSources?.join("、") || "无"}`);
    lines.push(`- 结论：${comparison.notes.join("；") || "基本可对应"}`);
    lines.push("");
  }

  return `${lines.join("\n")}\n`;
}

/**
 * 基于同一批 `results[]` 生成项目集摘要。
 *
 * @param results 单次 collect 的全部项目结果。
 * @param meta 本次快照的元信息。
 * @returns 返回 `_summary.md` 内容。
 */
function renderSummary(results, meta = {}) {
  const lines = [
    "# Reference 项目集汇总",
    "",
    `生成时间：${meta.generatedAt ?? new Date().toISOString()}`,
    `变更：${meta.change ?? DEFAULT_CHANGE}`,
    `项目集：${(meta.projects ?? results.map((result) => result.project)).join("、") || "无"}`,
    `requested_run_mode：${meta.runMode ?? "n/a"}`,
    `skip_init：${meta.skipInit ?? false}`,
    "",
    "| Project | Status | Runtime | overall | reuse_overage | median_skeleton | median_key_source | missing | collapsed | extra | warm_stable |",
    "| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |",
  ];

  for (const result of results) {
    lines.push(
    `| ${result.project} | ${result.status} | ${result.runtime_metrics.runtime_state} | ${formatPercent(result.fidelity_metrics?.overall_match_rate ?? null)} | ${result.fidelity_metrics?.reuse_overage ?? "N/A"} | ${formatRatio(result.fidelity_metrics?.median_skeleton_fidelity ?? null)} | ${formatRatio(result.fidelity_metrics?.median_key_source_coverage ?? null)} | ${result.fidelity_metrics?.missing_pages ?? "N/A"} | ${result.fidelity_metrics?.collapsed_pages ?? "N/A"} | ${result.extraGeneratedPages.length} | ${result.stability?.stable ?? "n/a"} |`,
    );
  }

  const commonGapCounts = new Map();
  for (const result of results) {
    for (const gap of result.commonGaps) {
      commonGapCounts.set(gap, (commonGapCounts.get(gap) ?? 0) + 1);
    }
  }

  lines.push("");
  lines.push("## Gate");
  lines.push("");
  const projectGates = new Map(
    (meta.primaryGateSummary?.project_results ?? []).map(item => [item.project, item]),
  );
  for (const result of results) {
    const gate = projectGates.get(result.project);
    if (!gate)
      throw new TypeError(`missing primary gate result for ${result.project}`);
    lines.push(`- ${result.project}：${gate.gate_label}，${gate.gate_reason}`);
  }

  lines.push("");
  lines.push("## 高频差距");
  lines.push("");
  for (const [gap, count] of [...commonGapCounts.entries()].sort((left, right) => right[1] - left[1])) {
    lines.push(`- ${gap}：${count} 个项目`);
  }

  if (meta.stability) {
    lines.push("");
    lines.push("## Stability");
    lines.push("");
    for (const item of meta.stability.projects) {
      lines.push(
        `- ${item.project}：${item.stable ? "stable" : "unstable"}，reuse_delta=${item.deltas.reuseOverage.delta ?? "N/A"} / skeleton_delta=${item.deltas.medianSkeletonFidelity.delta ?? "N/A"} / key_source_delta=${item.deltas.medianKeySourceCoverage.delta ?? "N/A"}`,
      );
    }
  }

  return `${lines.join("\n")}\n`;
}

/**
 * 汇总 9.6 当前样本的主问题，并标记本次快照是否具备基线资格。
 *
 * @param results 单次 collect 的全部项目结果。
 * @param meta 本次快照的元信息。
 * @returns 返回 `_optimization-notes.md` 内容。
 */
function renderOptimizationNotes(results, meta = {}) {
  const lines = [
    "# Reference 对比后的优化收敛",
    "",
    "## 当前收敛",
    "",
    `- ready 样本：${results.filter((result) => result.status === "ready").map((result) => result.project).join("、") || "无"}`,
    `- runtime_incomplete 样本：${results.filter((result) => result.status !== "ready").map((result) => result.project).join("、") || "无"}`,
    "",
    "## 高频观察",
    "",
  ];

  for (const result of results) {
    if (result.status !== "ready") {
      lines.push(`- ${result.project}：runtime_incomplete，需先解决 ${result.runtime_metrics.incomplete_reason ?? "assemble 未完成"}`);
      continue;
    }
    lines.push(
      `- ${result.project}：overall=${formatPercent(result.fidelity_metrics.overall_match_rate)} / reuse_overage=${result.fidelity_metrics.reuse_overage} / median_skeleton=${formatRatio(result.fidelity_metrics.median_skeleton_fidelity)} / median_key_source=${formatRatio(result.fidelity_metrics.median_key_source_coverage)}`,
    );
  }
  lines.push("");
  lines.push("## 基线资格说明");
  lines.push("");
  if (meta.skipInit) {
    lines.push("- 当前产物通过 `--skip-init` 从已有 runtime 读取；它本身只承担现状定位与 warm 稳定性对照，不单独替代 fresh init。若上游 runtime 已由同轮 fresh init 成功生成，则可与那批 fresh 产物一起构成 9.7-9.9 的验收基线。");
  } else {
    lines.push("- 当前产物来自 fresh run，可作为 9.6 的正式基线快照；若后续继续做 warm 对照，应与这批 fresh 产物保持同源。");
  }
  lines.push("");
  lines.push("## 下一步建议");
  lines.push("");
  if (results.some((result) => result.status !== "ready")) {
    lines.push("- 先解决 runtime_incomplete，避免把 assemble 缺口误判成页面质量问题。");
  }
  if (results.some((result) => (result.fidelity_metrics?.reuse_overage ?? 0) > 0)) {
    lines.push("- 优先回收 many-to-one reuse，父页只能消费 child digest，不应继续吞并多个 reference 主题。");
  }
  if (results.some((result) => (result.fidelity_metrics?.median_skeleton_fidelity ?? 1) < 0.8)) {
    lines.push("- 继续收敛 docs-backed 页面骨架，保证 section plan 真正落到最终 Markdown。");
  }
  if (results.some((result) => (result.fidelity_metrics?.median_key_source_coverage ?? 1) < 0.7)) {
    lines.push("- 继续提升 citation / key source grounding，避免只命中结构不命中关键文件。");
  }
  if (lines.at(-1) === "") {
    lines.push("- 当前指标已满足 9.6 的基础验收条件，可继续进入 warm stability 验证。");
  }

  return `${lines.join("\n")}\n`;
}

function renderStability(stability) {
  const lines = [
    "# Warm Stability",
    "",
    `repeats：${stability.repeats}`,
    "",
    "| Project | Stable | reuse_delta | skeleton_delta | key_source_delta |",
    "| --- | --- | ---: | ---: | ---: |",
  ];

  for (const item of stability.projects) {
    lines.push(
      `| ${item.project} | ${item.stable ? "yes" : "no"} | ${item.deltas.reuseOverage.delta ?? "N/A"} | ${item.deltas.medianSkeletonFidelity.delta ?? "N/A"} | ${item.deltas.medianKeySourceCoverage.delta ?? "N/A"} |`,
    );
  }

  return `${lines.join("\n")}\n`;
}

/**
 * 将单次 collect 的原子快照一次性落盘成 JSON 与 Markdown 报告集合。
 *
 * 这里强制 `_snapshot.json`、`_summary.md`、项目报告与 gap ledger 共用同一批 `results[]`，
 * 避免 partial rerun 只覆写其中一部分后造成口径漂移。
 *
 * @param results 单次 collect 的全部项目结果。
 * @param reportDir 报告输出目录。
 * @param summaryPath `_summary.md` 输出路径。
 * @param optimizationNotesPath `_optimization-notes.md` 输出路径。
 * @param snapshotPath `_snapshot.json` 输出路径。
 * @param meta 本次快照的元信息。
 */
function writeReports(results, reportDir, summaryPath, optimizationNotesPath, snapshotPath, meta) {
  ensureDir(reportDir);
  writeFileSync(snapshotPath, `${JSON.stringify({
    generated_at: meta.generatedAt,
    change: meta.change,
    requested_run_mode: meta.runMode,
    skip_init: meta.skipInit,
    projects: meta.projects,
    source_roots: meta.sourceRoots,
    stability: meta.stability ?? null,
    primary_gate_summary: meta.primaryGateSummary ?? null,
    results,
  }, null, 2)}\n`);
  const projectGates = new Map(
    (meta.primaryGateSummary?.project_results ?? []).map(item => [item.project, item]),
  );
  for (const result of results) {
    const gate = projectGates.get(result.project);
    if (!gate)
      throw new TypeError(`missing primary gate result for ${result.project}`);
    writeFileSync(path.join(reportDir, `${result.project}.md`), renderProjectReport(result, gate));
    writeFileSync(path.join(reportDir, `${result.project}-gap-ledger.md`), renderGapLedger(result));
  }
  writeFileSync(summaryPath, renderSummary(results, meta));
  writeFileSync(optimizationNotesPath, renderOptimizationNotes(results, meta));
  if (meta.stability) {
    writeFileSync(path.join(reportDir, "_stability.md"), renderStability(meta.stability));
  }
}

/**
 * 解析 CLI 参数，区分 init 模式、专项项目集与 warm stability 配置。
 *
 * @param argv 原始命令行参数。
 * @returns 返回脚本运行所需的标准化参数。
 */
function parseCliArgs(argv) {
  const names = [];
  let acceptancePlanPath;
  let jobs;
  let runMode = "cold";
  let change = DEFAULT_CHANGE;
  let skipInit = false;
  let initTimeoutMinutes = DEFAULT_INIT_TIMEOUT_MS / 60000;
  let warmReruns = null;

  for (let index = 0; index < argv.length; index++) {
    const arg = argv[index];
    if (arg === "--acceptance-plan") {
      acceptancePlanPath = argv[index + 1];
      index++;
      continue;
    }
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
    if (arg === "--warm-reruns") {
      warmReruns = argv[index + 1] || warmReruns;
      index++;
      continue;
    }
    names.push(arg);
  }

  const parsedTimeoutMinutes = Number(initTimeoutMinutes);
  const parsedWarmReruns = Number(warmReruns);
  const initTimeoutMs
    = Number.isFinite(parsedTimeoutMinutes) && parsedTimeoutMinutes > 0
      ? Math.floor(parsedTimeoutMinutes * 60_000)
      : DEFAULT_INIT_TIMEOUT_MS;

  return {
    acceptancePlanPath,
    change,
    initTimeoutMs,
    jobs,
    names,
    runMode,
    skipInit,
    warmReruns:
      Number.isFinite(parsedWarmReruns) && parsedWarmReruns > 0
        ? Math.floor(parsedWarmReruns)
        : null,
  };
}

async function main(argv) {
  const changeDir = path.join(ROOT_DIR, ".spec", "changes", argv.change);
  const reportDir = path.join(changeDir, "reference-project-reports");
  const summaryPath = path.join(reportDir, "_summary.md");
  const optimizationNotesPath = path.join(reportDir, "_optimization-notes.md");
  const snapshotPath = path.join(reportDir, "_snapshot.json");
  const projects = argv.names.length > 0 ? argv.names : discoverProjects();
  if (!argv.acceptancePlanPath)
    throw new TypeError("--acceptance-plan is required");
  const acceptancePlan = createAcceptancePlan(JSON.parse(
    readFileSync(path.resolve(argv.acceptancePlanPath), "utf8"),
  ));
  const jobs = argv.jobs == null ? 1 : resolveProjectJobs(argv.jobs, projects.length);
  const stabilityReruns
    = argv.warmReruns
      ?? ((argv.runMode === "warm" || argv.skipInit) ? 2 : 0);
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
    const result = collectProject(project, run, {
      runMode: argv.runMode,
      stabilityReruns,
    });
    progressPrinter.info(
      `report ready: status=${result.status}, overall=${result.fidelity_metrics?.overall_match_rate ?? "N/A"}, reuse=${result.fidelity_metrics?.reuse_overage ?? "N/A"}, skeleton=${formatRatio(result.fidelity_metrics?.median_skeleton_fidelity ?? null)}, key-source=${formatRatio(result.fidelity_metrics?.median_key_source_coverage ?? null)}`,
    );
    return result;
  });
  const stability = buildWarmStability(projects, stabilityReruns);
  if (stability) {
    const stabilityByProject = new Map(stability.projects.map((item) => [item.project, item]));
    for (const result of results) {
      result.stability = stabilityByProject.get(result.project) ?? null;
    }
  }

  const generatedAt = new Date().toISOString();
  const primaryGateSummary = buildPrimaryGateSummary(results, { acceptancePlan });
  writeReports(
    results,
    reportDir,
    summaryPath,
    optimizationNotesPath,
    snapshotPath,
    {
      generatedAt,
      change: argv.change,
      projects,
      primaryGateSummary,
      runMode: argv.runMode,
      skipInit: argv.skipInit,
      stability,
      sourceRoots: {
        testDir: TEST_DIR,
        referenceDir: REFERENCE_DIR,
      },
    },
  );
  process.stdout.write(`${JSON.stringify({
    change: argv.change,
    reportDir,
    summaryPath,
    optimizationNotesPath,
    snapshotPath,
    primaryGateSummary,
    jobs,
    initTimeoutMs: argv.initTimeoutMs,
    runMode: argv.runMode,
    projects: results.map((result) => ({
      project: result.project,
      status: result.status,
      runMode: result.runLabel,
      generatedPageCount: result.generatedPageCount,
      referencePageCount: result.referencePageCount,
      overallMatchRate: result.fidelity_metrics?.overall_match_rate ?? null,
      missingCount: result.fidelity_metrics?.missing_pages ?? null,
      collapsedPages: result.fidelity_metrics?.collapsed_pages ?? null,
      reuseOverage: result.fidelity_metrics?.reuse_overage ?? null,
      medianSkeletonFidelity: result.fidelity_metrics?.median_skeleton_fidelity ?? null,
      medianKeySourceCoverage: result.fidelity_metrics?.median_key_source_coverage ?? null,
      lowFidelityMatchedPages: result.fidelity_metrics?.low_fidelity_matched_pages ?? null,
      extraGeneratedPages: result.extraGeneratedPages.length,
      budgetStoppedPages: result.runtime_metrics.stop_reasons.budgetStoppedPages,
      stalledPages: result.runtime_metrics.stop_reasons.stalledPages,
      invalidOutputPages: result.runtime_metrics.stop_reasons.invalidOutputPages,
      providerFailedPages: result.runtime_metrics.stop_reasons.providerFailedPages,
      pipelineRuntimeState: result.runtime_metrics.runtime_summary?.runtime_state ?? null,
      unitRuntimeGates: result.runtime_metrics.gate_summary?.total ?? 0,
      composeBlockedUnits: result.runtime_metrics.gate_summary?.composeBlocked ?? 0,
      parentContractPages: result.runtime_metrics.parent_contract?.parentPages ?? 0,
      generatedTopicPages: result.coverage?.generatedTopicPages ?? null,
      generatedEvidencePages: result.coverage?.generatedEvidencePages ?? null,
      generatedDiagramPages: result.coverage?.generatedDiagramPages ?? null,
      totalTokens: result.usage?.total_tokens ?? 0,
      pageResearchRequests: promptCount(result.usage, "page_research"),
      stability: result.stability ?? null,
    })),
  }, null, 2)}\n`);
  return primaryGateSummary;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const summary = await main(parseCliArgs(process.argv.slice(2)));
  process.exitCode = summary.exit_code;
}

export {
  comparePagePair,
  extractOutlineSkeleton,
  isEnglishRawDocsPage,
  normalizeOutlineSectionKey,
  outlineOverlapRate,
};
