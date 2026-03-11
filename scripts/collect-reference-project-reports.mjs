/**
 * 针对带 reference 的测试项目，批量执行带 provider 的 init，
 * 并把 generated `.wiki/*.md` 与 reference content 目录下的 Markdown 文件
 * 做逐项目、逐文件的结构化对比，输出到当前 OpenSpec change 目录。
 *
 * 默认输出：
 * - openspec/changes/iteration-9-llm-enhanced-content-generation/reference-project-reports/*.md
 * - openspec/changes/iteration-9-llm-enhanced-content-generation/reference-project-reports/_summary.md
 *
 * 用法：
 *   node scripts/collect-reference-project-reports.mjs
 *   node scripts/collect-reference-project-reports.mjs --jobs 2
 *   node scripts/collect-reference-project-reports.mjs chi axum
 */

import { execFileSync, spawn } from "node:child_process";
import {
  cpSync,
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  statSync,
  unlinkSync,
  writeFileSync,
} from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

import {
  ROOT_DIR,
  TEST_DIR,
  TMP_DIR,
  removePathWithRetry,
  resolveProjectJobs,
  runTaskPool,
} from "./testing/helpers.mjs";

const BINARY_NAME = process.platform === "win32" ? "wiki-core.exe" : "wiki-core";
const DEBUG_BINARY = path.join(ROOT_DIR, "target", "debug", BINARY_NAME);
const RELEASE_BINARY = path.join(ROOT_DIR, "target", "release", BINARY_NAME);
const BINARY_PATH = resolveBinaryPath();

const REFERENCE_DIR = path.join(TMP_DIR, "reference");
const CHANGE_DIR = path.join(
  ROOT_DIR,
  "openspec",
  "changes",
  "iteration-9-llm-enhanced-content-generation",
);
const REPORT_DIR = path.join(CHANGE_DIR, "reference-project-reports");
const SUMMARY_PATH = path.join(REPORT_DIR, "_summary.md");
const ROOT_DEV_CONFIG_PATH = path.join(ROOT_DIR, "wiki.dev.yaml");
const REAL_REPO_MAP = {
  aLocal: "E:\\project\\aLocal",
};

const FILE_MENTION_PATTERN =
  /[A-Za-z0-9_./-]+\.(?:go|rs|ts|tsx|js|jsx|py|java|kt|php|swift|md|toml|json|ya?ml|conf|ini|sql)/g;
const ASCII_TOKEN_PATTERN = /[A-Za-z_][A-Za-z0-9_/-]*/g;

function resolveBinaryPath() {
  if (process.env.WIKI_CORE_BINARY) {
    return process.env.WIKI_CORE_BINARY;
  }
  if (!existsSync(DEBUG_BINARY) && !existsSync(RELEASE_BINARY)) {
    throw new Error(`缺少 wiki-core binary: ${DEBUG_BINARY} / ${RELEASE_BINARY}`);
  }
  if (!existsSync(DEBUG_BINARY)) {
    return RELEASE_BINARY;
  }
  if (!existsSync(RELEASE_BINARY)) {
    return DEBUG_BINARY;
  }

  const debugMtime = statSync(DEBUG_BINARY).mtimeMs;
  const releaseMtime = statSync(RELEASE_BINARY).mtimeMs;
  return releaseMtime >= debugMtime ? RELEASE_BINARY : DEBUG_BINARY;
}

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

async function callCoreStreaming(command, options = {}) {
  const timeoutMs = options.timeoutMs ?? 90 * 60 * 1000;

  return await new Promise((resolve, reject) => {
    const child = spawn(BINARY_PATH, ["--json"], {
      cwd: ROOT_DIR,
      stdio: ["pipe", "pipe", "pipe"],
    });

    let stdoutBuffer = "";
    let stderr = "";
    let terminal = null;
    let timedOut = false;
    const timer = setTimeout(() => {
      timedOut = true;
      child.kill();
    }, timeoutMs);

    child.stdout.on("data", (chunk) => {
      stdoutBuffer += chunk.toString();
      drainOutput(false);
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk.toString();
    });
    child.on("error", (error) => {
      clearTimeout(timer);
      reject(error);
    });
    child.on("close", (code) => {
      clearTimeout(timer);
      drainOutput(true);

      if (timedOut) {
        reject(new Error(`wiki-core init timed out after ${timeoutMs}ms`));
        return;
      }
      if (code !== 0) {
        reject(new Error(stderr || `wiki-core exited with code ${code}`));
        return;
      }
      if (!terminal) {
        reject(new Error("wiki-core 输出中缺少终态响应"));
        return;
      }
      resolve(terminal);
    });

    child.stdin.end(`${JSON.stringify({ ...command, streamProgress: true })}\n`);

    function drainOutput(flushRemainder) {
      while (true) {
        const newlineIndex = stdoutBuffer.indexOf("\n");
        if (newlineIndex < 0) {
          break;
        }
        const line = stdoutBuffer.slice(0, newlineIndex).trim();
        stdoutBuffer = stdoutBuffer.slice(newlineIndex + 1);
        if (!line) {
          continue;
        }
        consumeEventLine(line);
      }

      if (flushRemainder && stdoutBuffer.trim()) {
        consumeEventLine(stdoutBuffer.trim());
        stdoutBuffer = "";
      }
    }

    function consumeEventLine(line) {
      const event = JSON.parse(line);
      if (event.type === "progress") {
        options.onProgress?.(event);
        return;
      }
      if ((event.type === "result" || event.type === "error") && event.response) {
        terminal = event.response;
      }
    }
  });
}

function formatElapsed(elapsedMs) {
  if (elapsedMs < 1_000) {
    return `${elapsedMs}ms`;
  }
  return `${(elapsedMs / 1_000).toFixed(elapsedMs >= 10_000 ? 0 : 1)}s`;
}

function createProjectProgressPrinter(project) {
  const countedPercents = new Map();
  const phaseMessages = new Map();

  return {
    info(message) {
      console.log(`[${project}] ${message}`);
    },
    onProgress(event) {
      const prefix = `[${project}] ${formatElapsed(event.elapsed_ms)} ${event.phase}`;
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

  return {
    title,
    relativePath,
    content,
    sectionTitles,
    nonEmptyLines,
    proseLines,
    bulletLines,
    mermaidBlocks,
    citations,
    fileMentions,
    tokens,
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
  if (combined.includes("模块") || combined.includes("component") || combined.includes("middleware")) {
    return "module";
  }
  return "other";
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
  if (referencePage.citations.length > 0 && generatedPage.citations.length === 0) {
    notes.push("缺少引用/出处块");
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
    overlappingFiles,
    overlappingBasenames,
    missingBasenames,
    notes,
  };
}

async function withTemporaryDevConfig(repoRoot, callback) {
  const targetPath = path.join(repoRoot, "wiki.dev.yaml");
  const rootConfig = readFileSync(ROOT_DEV_CONFIG_PATH, "utf-8");
  const previous = existsSync(targetPath) ? readFileSync(targetPath, "utf-8") : null;

  writeFileSync(targetPath, rootConfig);
  try {
    return await callback();
  } finally {
    if (previous === null) {
      unlinkSync(targetPath);
    } else {
      writeFileSync(targetPath, previous);
    }
  }
}

async function runInitForProject(project, progressPrinter) {
  const projectRoot = path.join(TEST_DIR, project);
  const wikiDir = path.join(projectRoot, ".wiki");
  removePathWithRetry(wikiDir);
  progressPrinter.info("start init");

  if (REAL_REPO_MAP[project]) {
    const realRepoRoot = REAL_REPO_MAP[project];
    const realWikiDir = path.join(realRepoRoot, ".wiki");
    removePathWithRetry(realWikiDir);

    const result = await withTemporaryDevConfig(realRepoRoot, () =>
      callCoreStreaming(
        { action: "init", repoRoot: realRepoRoot },
        { onProgress: (event) => progressPrinter.onProgress(event) },
      ),
    );
    if (!result.ok) {
      throw new Error(result.error || `${project} init failed`);
    }

    removePathWithRetry(wikiDir);
    cpSync(realWikiDir, wikiDir, { recursive: true });
    removePathWithRetry(realWikiDir);
    progressPrinter.info("init done");
    return;
  }

  const result = await withTemporaryDevConfig(projectRoot, () =>
    callCoreStreaming(
      { action: "init", repoRoot: `tmp/test/${project}` },
      { onProgress: (event) => progressPrinter.onProgress(event) },
    ),
  );
  if (!result.ok) {
    throw new Error(result.error || `${project} init failed`);
  }
  progressPrinter.info("init done");
}

function collectProject(project) {
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
  const commonGaps = summarizeProjectGaps(comparisons, generatedPages, referencePages);

  return {
    project,
    generatedPageCount: generatedPages.length,
    referencePageCount: referencePages.length,
    matchedCount,
    missingCount,
    extraGeneratedPages,
    comparisons,
    reusedGeneratedPages,
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

function summarizeProjectGaps(comparisons, generatedPages, referencePages) {
  const gaps = [];
  const matched = comparisons.filter((item) => item.matched);

  if (referencePages.length > generatedPages.length * 2) {
    gaps.push("reference 文档树明显更细，当前 planner 仍以总览页和少量模块页为主");
  }
  if (comparisons.some((item) => !item.matched)) {
    gaps.push("存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足");
  }
  if (matched.some((item) => item.notes.includes("缺少引用/出处块"))) {
    gaps.push("生成页普遍缺少 reference 那种源码引用/出处层");
  }
  if (matched.some((item) => item.notes.includes("图表少于 reference"))) {
    gaps.push("Mermaid/结构图表达仍然不足");
  }
  if (matched.some((item) => item.notes.includes("章节拆分比 reference 粗"))) {
    gaps.push("单页章节拆分比 reference 粗，主题混杂在同一页里");
  }
  if (matched.some((item) => item.notes.includes("解释性段落明显不足"))) {
    gaps.push("解释层正文密度仍低于 reference");
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
  lines.push("| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Mermaid(ref/gen) | 主要结论 |");
  lines.push("| --- | --- | ---: | ---: | ---: | --- |");

  for (const comparison of result.comparisons) {
    if (!comparison.matched) {
      lines.push(
        `| ${comparison.referencePath} | 缺失 | - | - | - | ${comparison.notes.join("；")} |`,
      );
      continue;
    }

    const note = comparison.notes.length > 0 ? comparison.notes.join("；") : "基本可对应";
    lines.push(
      `| ${comparison.referencePath} | ${comparison.generatedPath} | ${comparison.referenceLines}/${comparison.generatedLines} | ${comparison.referenceProse}/${comparison.generatedProse} | ${comparison.referenceMermaid}/${comparison.generatedMermaid} | ${note} |`,
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
    lines.push(`- 行数：${comparison.referenceLines} / ${comparison.generatedLines}`);
    lines.push(`- 段落行数：${comparison.referenceProse} / ${comparison.generatedProse}`);
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
    "| Project | generated | reference | matched | missing | extra generated |",
    "| --- | ---: | ---: | ---: | ---: | ---: |",
  ];

  for (const result of results) {
    lines.push(
      `| ${result.project} | ${result.generatedPageCount} | ${result.referencePageCount} | ${result.matchedCount} | ${result.missingCount} | ${result.extraGeneratedPages.length} |`,
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

function writeReports(results) {
  ensureDir(REPORT_DIR);
  for (const result of results) {
    writeFileSync(path.join(REPORT_DIR, `${result.project}.md`), renderProjectReport(result));
  }
  writeFileSync(SUMMARY_PATH, renderSummary(results));
}

function parseCliArgs(argv) {
  const names = [];
  let jobs;

  for (let index = 0; index < argv.length; index++) {
    const arg = argv[index];
    if (arg === "--jobs") {
      jobs = argv[index + 1];
      index++;
      continue;
    }
    names.push(arg);
  }

  return { jobs, names };
}

async function main(argv) {
  if (!existsSync(ROOT_DEV_CONFIG_PATH)) {
    throw new Error(`缺少 root wiki.dev.yaml: ${ROOT_DEV_CONFIG_PATH}`);
  }

  const projects = argv.names.length > 0 ? argv.names : discoverProjects();
  const jobs = argv.jobs == null ? 1 : resolveProjectJobs(argv.jobs, projects.length);
  const results = await runTaskPool(projects, jobs, async (project, index) => {
    const progressPrinter = createProjectProgressPrinter(project);
    progressPrinter.info(`queue ${index + 1}/${projects.length}`);
    await runInitForProject(project, progressPrinter);
    const result = collectProject(project);
    progressPrinter.info(
      `report ready: generated=${result.generatedPageCount}, reference=${result.referencePageCount}, matched=${result.matchedCount}, missing=${result.missingCount}`,
    );
    return result;
  });

  writeReports(results);
  process.stdout.write(`${JSON.stringify({
    reportDir: REPORT_DIR,
    summaryPath: SUMMARY_PATH,
    jobs,
    projects: results.map((result) => ({
      project: result.project,
      generatedPageCount: result.generatedPageCount,
      referencePageCount: result.referencePageCount,
      matchedCount: result.matchedCount,
      missingCount: result.missingCount,
    })),
  }, null, 2)}\n`);
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  await main(parseCliArgs(process.argv.slice(2)));
}
