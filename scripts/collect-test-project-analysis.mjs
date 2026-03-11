/**
 * 汇总测试项目集的 `.wiki`、SQLite 与 query 指标，供 OpenSpec 分析报告复用。
 *
 * 只读取 `tmp/test/*` 与 `tmp/reference/*`，不修改仓库内容。
 *
 * 用法：
 *   node scripts/collect-test-project-analysis.mjs
 *   node scripts/collect-test-project-analysis.mjs axum chi
 */

import { execFileSync } from "node:child_process";
import {
  existsSync,
  readdirSync,
  readFileSync,
  statSync,
  writeFileSync,
} from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

import {
  ROOT_DIR,
  TEST_DIR,
  TMP_DIR,
  callCore,
  countFilesWithMarker,
  countMdFiles,
} from "./testing/helpers.mjs";

const REFERENCE_DIR = path.join(TMP_DIR, "reference");
const REPORT_PATH = path.join(ROOT_DIR, "test-project-analysis.md");
const CORE_PAGE_TITLES = {
  overview: "项目概述",
  architecture: "系统架构",
  workflow: "工作流与部署",
};
const GRAPH_FACT_LINE_PATTERN =
  /(关系：|跨模块关系|流程：|检测到流程|社区：|循环：|循环依赖：|循环提示|图热点：)/;
const EVIDENCE_HEADING_PATTERN = /\*\*[^*\n]*(来源|证据)[^*\n]*\*\*/g;
const TOPIC_PAGE_PATTERN =
  /(专题\/|主题：|流程主题|机制|能力|routing|extract|response|middleware|handler|router)/i;

/**
 * 读取 sqlite 单值查询结果。
 *
 * @param dbPath SQLite 文件路径。
 * @param sql 需要执行的查询语句。
 * @returns 去掉首尾空白后的结果文本。
 */
function querySqlite(dbPath, sql) {
  const rows = querySqliteLines(dbPath, sql);
  return rows.at(-1) ?? "";
}

function querySqliteLines(dbPath, sql) {
  const statement = `PRAGMA busy_timeout=30000; ${sql}`;
  let lastError = null;

  for (let attempt = 0; attempt < 5; attempt++) {
    try {
      const output = execFileSync("sqlite3", [dbPath, statement], {
        encoding: "utf-8",
        timeout: 35_000,
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

/**
 * 读取 sqlite 表格查询结果。
 *
 * @param dbPath SQLite 文件路径。
 * @param sql 需要执行的查询语句。
 * @returns 按行切分后的结果。
 */
function querySqliteRows(dbPath, sql) {
  return querySqliteLines(dbPath, sql);
}

/**
 * 在没有 reference 的情况下保持输出结构一致。
 *
 * @param project 项目名。
 * @returns reference 摘要。
 */
function collectReference(project) {
  const contentDir = path.join(REFERENCE_DIR, project, "content");
  if (!existsSync(contentDir)) {
    return {
      hasReference: false,
      pageCount: 0,
      pageNames: [],
    };
  }

  return {
    hasReference: true,
    pageCount: countMdFiles(contentDir),
    pageNames: listMarkdownFiles(contentDir),
  };
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

function readWikiPages(wikiDir) {
  return listMarkdownFiles(wikiDir).map((relativePath) => {
    const fullPath = path.join(wikiDir, relativePath);
    const content = readFileSync(fullPath, "utf-8");
    const lines = content.split(/\r?\n/);
    const title =
      lines.find((line) => line.startsWith("# "))?.replace(/^#\s+/, "").trim()
      || path.basename(relativePath, ".md");
    let bulletLines = 0;
    let proseLines = 0;
    let graphFactLines = 0;
    let headingLines = 0;
    let markerLines = 0;
    let mermaidBlocks = 0;
    let evidenceBlocks = 0;
    let nonEmptyLines = 0;
    let inCodeFence = false;

    for (const line of lines) {
      const trimmed = line.trim();
      if (!trimmed) {
        continue;
      }

      nonEmptyLines += 1;
      if (trimmed.startsWith("<!-- wiki:managed:")) {
        markerLines += 1;
        continue;
      }
      if (trimmed.startsWith("```")) {
        if (trimmed === "```mermaid") {
          mermaidBlocks += 1;
        }
        inCodeFence = !inCodeFence;
        continue;
      }
      if (inCodeFence) {
        continue;
      }
      if (trimmed.startsWith("#")) {
        headingLines += 1;
        continue;
      }
      if (trimmed.startsWith("- ")) {
        bulletLines += 1;
      } else {
        proseLines += 1;
      }
      if (GRAPH_FACT_LINE_PATTERN.test(trimmed)) {
        graphFactLines += 1;
      }
    }

    evidenceBlocks = [...content.matchAll(EVIDENCE_HEADING_PATTERN)].length;

    return {
      title,
      relativePath,
      bulletLines,
      proseLines,
      graphFactLines,
      headingLines,
      markerLines,
      mermaidBlocks,
      evidenceBlocks,
      nonEmptyLines,
      isTopicPage: TOPIC_PAGE_PATTERN.test(`${relativePath} ${title}`),
    };
  });
}

function pickPageByTitle(pages, title) {
  return pages.find((page) => page.title === title) ?? null;
}

function round(value) {
  return Number(value.toFixed(1));
}

function buildPageMetrics(wikiDir) {
  const pages = readWikiPages(wikiDir);
  const totalNonEmptyLines = pages.reduce((sum, page) => sum + page.nonEmptyLines, 0);
  const totalBulletLines = pages.reduce((sum, page) => sum + page.bulletLines, 0);
  const totalProseLines = pages.reduce((sum, page) => sum + page.proseLines, 0);
  const totalGraphFactLines = pages.reduce((sum, page) => sum + page.graphFactLines, 0);
  const totalMermaidBlocks = pages.reduce((sum, page) => sum + page.mermaidBlocks, 0);
  const totalEvidenceBlocks = pages.reduce((sum, page) => sum + page.evidenceBlocks, 0);
  const graphLandingPages = pages.filter((page) => page.graphFactLines > 0).length;
  const topicPages = pages.filter((page) => page.isTopicPage).length;
  const evidenceLandingPages = pages.filter((page) => page.evidenceBlocks > 0).length;
  const mermaidLandingPages = pages.filter((page) => page.mermaidBlocks > 0).length;
  const densestPage = pages.reduce(
    (best, page) => (page.nonEmptyLines > best.nonEmptyLines ? page : best),
    pages[0] ?? {
      title: "n/a",
      relativePath: "",
      nonEmptyLines: 0,
      graphFactLines: 0,
    },
  );

  const overviewPage = pickPageByTitle(pages, CORE_PAGE_TITLES.overview);
  const architecturePage = pickPageByTitle(pages, CORE_PAGE_TITLES.architecture);
  const workflowPage = pickPageByTitle(pages, CORE_PAGE_TITLES.workflow);

  return {
    pages,
    totalNonEmptyLines,
    totalBulletLines,
    totalProseLines,
    totalGraphFactLines,
    totalMermaidBlocks,
    totalEvidenceBlocks,
    graphLandingPages,
    topicPages,
    evidenceLandingPages,
    mermaidLandingPages,
    avgNonEmptyLinesPerPage: round(totalNonEmptyLines / Math.max(pages.length, 1)),
    avgBulletLinesPerPage: round(totalBulletLines / Math.max(pages.length, 1)),
    avgProseLinesPerPage: round(totalProseLines / Math.max(pages.length, 1)),
    densestPage: {
      title: densestPage.title,
      relativePath: densestPage.relativePath,
      nonEmptyLines: densestPage.nonEmptyLines,
      graphFactLines: densestPage.graphFactLines,
    },
    corePages: {
      overview: overviewPage,
      architecture: architecturePage,
      workflow: workflowPage,
    },
  };
}

function describeDensity(metrics) {
  if (metrics.avgNonEmptyLinesPerPage >= 80) {
    return "高";
  }
  if (metrics.avgNonEmptyLinesPerPage >= 35) {
    return "中";
  }
  return "低";
}

function describeReferenceDelta(result) {
  if (!result.reference.hasReference) {
    return "无 reference，对照以 query 与 graph 命中为主。";
  }

  const delta = result.pageCount - result.reference.pageCount;
  const ratio = result.reference.pageCount === 0 ? 0 : result.pageCount / result.reference.pageCount;
  if (ratio < 0.25) {
    return `与 reference 相比明显压缩（${result.pageCount} vs ${result.reference.pageCount}，${delta} 页），当前仍以 repo 级总览 + 模块页为主。`;
  }
  if (ratio > 1.25) {
    return `比 reference 更展开（${result.pageCount} vs ${result.reference.pageCount}，+${delta} 页），页面拆分已经超过参考样例。`;
  }
  return `与 reference 接近（${result.pageCount} vs ${result.reference.pageCount}，${delta >= 0 ? "+" : ""}${delta} 页）。`;
}

function describeEnhancementObservation(result) {
  const { pageMetrics } = result;
  if (
    pageMetrics.totalMermaidBlocks > 0
    || pageMetrics.totalEvidenceBlocks > 0
    || pageMetrics.topicPages > 0
    || pageMetrics.avgProseLinesPerPage >= 6
  ) {
    return "项目集页面已经出现专题页、evidence block 或 Mermaid，说明 9.1 的结构增强进入了正式产物。";
  }

  return [
    "项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；",
    "这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。",
  ].join("");
}

function formatQuerySummary(result) {
  if (!result.querySummary) {
    return "无可用符号 query 样本。";
  }

  return [
    `符号 query 以 \`${result.querySummary.term}\` 为样本，`,
    `命中 ${result.querySummary.matchedSymbols} 个符号、${result.querySummary.matchedPages} 个页面；`,
    `图 query 以 \`${result.graphQuerySummary?.term ?? "n/a"}\` 为样本，`,
    `扩展 ${result.graphQuerySummary?.matchedGraphEdges ?? 0} 条图边、`,
    `${result.graphQuerySummary?.matchedCommunities ?? 0} 个社区、`,
    `${result.graphQuerySummary?.matchedProcesses ?? 0} 个流程。`,
  ].join("");
}

/**
 * 列出适合做 query 验证的候选符号。
 *
 * 优先唯一、长度适中且更像真实业务名的定义，后续再逐个试探 query 命中。
 *
 * @param dbPath SQLite 文件路径。
 * @returns 候选符号名列表。
 */
function listRepresentativeSymbols(dbPath) {
  return querySqliteRows(
    dbPath,
    [
      "select name",
      "from symbols",
      "where length(name) >= 6",
      "and name glob '[A-Za-z_]*'",
      "and name not glob '[A-Z0-9_]*'",
      "group by name",
      "having count(*) = 1",
      "order by",
      "max(is_exported) desc,",
      "case label",
      "when 'function' then 0",
      "when 'class' then 1",
      "when 'struct' then 2",
      "when 'interface' then 3",
      "when 'trait' then 4",
      "when 'enum' then 5",
      "when 'method' then 6",
      "when 'type' then 7",
      "else 8 end,",
      "case",
      "when length(name) between 8 and 24 then 0",
      "when length(name) between 25 and 40 then 1",
      "else 2",
      "end,",
      "length(name) asc,",
      "name asc",
      "limit 24;",
    ].join(" "),
  );
}

function listRepresentativeGraphSymbols(dbPath) {
  return querySqliteRows(
    dbPath,
    [
      "select s.name",
      "from edges e",
      "join symbols s on s.id = e.source_id",
      "where length(s.name) >= 6",
      "and s.name glob '[A-Za-z_]*'",
      "and s.name not glob '[A-Z0-9_]*'",
      "group by s.name",
      "having count(*) = 1",
      "order by max(s.is_exported) desc, count(*) desc, length(s.name) asc, s.name asc",
      "limit 24;",
    ].join(" "),
  );
}

/**
 * 把 `key|value` 风格的 sqlite group by 结果转成对象数组。
 *
 * @param {string[]} rows sqlite 返回的行集合。
 * @returns {{name: string, count: number}[]} 解析后的条目。
 */
function parseGroupedRows(rows) {
  return rows.map((row) => {
    const [name, count] = row.split("|");
    return {
      name,
      count: Number(count),
    };
  });
}

/**
 * 收集单个项目的验证指标。
 *
 * @param project 项目名。
 * @returns 结构化分析结果。
 */
function collectProject(project) {
  const repoRoot = path.join(TEST_DIR, project);
  const wikiDir = path.join(repoRoot, ".wiki");
  const dbPath = path.join(wikiDir, ".cache", "wiki-cache.db");
  const metadataPath = path.join(wikiDir, "wiki.metadata.json");
  const metadata = JSON.parse(readFileSync(metadataPath, "utf-8"));
  const pageMetrics = buildPageMetrics(wikiDir);

  const pageCount = countMdFiles(wikiDir);
  const markerCount = countFilesWithMarker(wikiDir);
  const moduleCount = metadata.modules?.length ?? 0;
  const sourceCount = metadata.source_files?.length ?? 0;
  const symbolCount = Number(querySqlite(dbPath, "select count(*) from symbols;") || "0");
  const exportedSymbolCount = Number(
    querySqlite(dbPath, "select count(*) from symbols where is_exported = 1;") || "0",
  );
  const edgeCount = Number(querySqlite(dbPath, "select count(*) from edges;") || "0");
  const communityCount = Number(querySqlite(dbPath, "select count(*) from communities;") || "0");
  const processCount = Number(querySqlite(dbPath, "select count(*) from processes;") || "0");
  const languageBreakdown = parseGroupedRows(
    querySqliteRows(
      dbPath,
      "select language, count(*) from symbols group by language order by count(*) desc, language asc;",
    ),
  );
  const labelBreakdown = parseGroupedRows(
    querySqliteRows(
      dbPath,
      "select label, count(*) from symbols group by label order by count(*) desc, label asc limit 6;",
    ),
  );
  const representativeSymbols = listRepresentativeSymbols(dbPath);
  const representativeGraphSymbols = listRepresentativeGraphSymbols(dbPath);

  let representativeSymbol = "";
  let querySummary = null;
  let fallbackQuerySummary = null;
  for (const candidate of representativeSymbols) {
    const query = callCore({
      action: "query",
      repoRoot: path.relative(process.cwd(), repoRoot).replaceAll("\\", "/"),
      term: candidate,
    });
    if (!query.ok) {
      throw new Error(`${project} query failed: ${query.error ?? "unknown error"}`);
    }

    const report = query.data;
    const exactSymbol = report.matched_symbols.find((symbol) => symbol.name === candidate) ?? null;
    representativeSymbol = candidate;
    const summary = {
      term: candidate,
      matchedSymbols: report.matched_symbols.length,
      matchedPages: report.matched_pages.length,
      matchedSources: report.matched_sources.length,
      exactSymbolMatched: Boolean(exactSymbol),
      exactSymbol: exactSymbol
        ? {
            name: exactSymbol.name,
            label: exactSymbol.label,
            filePath: exactSymbol.file_path,
          }
        : null,
      topPage: report.matches[0]
        ? {
            title: report.matches[0].title,
            reasons: report.matches[0].reasons,
          }
        : null,
    };
    fallbackQuerySummary ??= summary;
    if (summary.exactSymbolMatched) {
      querySummary = summary;
      break;
    }
  }
  querySummary ??= fallbackQuerySummary;

  let graphQuerySummary = null;
  let fallbackGraphQuerySummary = null;
  for (const candidate of representativeGraphSymbols) {
    const query = callCore({
      action: "query",
      repoRoot: path.relative(process.cwd(), repoRoot).replaceAll("\\", "/"),
      term: candidate,
    });
    if (!query.ok) {
      throw new Error(`${project} graph query failed: ${query.error ?? "unknown error"}`);
    }

    const report = query.data;
    const summary = {
      term: candidate,
      matchedGraphEdges: report.matched_symbol_edges?.length ?? 0,
      matchedProcesses: report.matched_processes?.length ?? 0,
      matchedCommunities: report.matched_communities?.length ?? 0,
      provenanceSummary: report.provenance_summary ?? "",
    };
    fallbackGraphQuerySummary ??= summary;
    if (
      summary.matchedGraphEdges > 0
      || summary.matchedProcesses > 0
      || summary.matchedCommunities > 0
    ) {
      graphQuerySummary = summary;
      break;
    }
  }
  graphQuerySummary ??= fallbackGraphQuerySummary;

  return {
    project,
    pageCount,
    markerCount,
    moduleCount,
    sourceCount,
    symbolCount,
    exportedSymbolCount,
    edgeCount,
    communityCount,
    processCount,
    languageBreakdown,
    labelBreakdown,
    representativeSymbol,
    querySummary,
    graphQuerySummary,
    pageMetrics,
    reference: collectReference(project),
  };
}

function discoverProjects() {
  if (!existsSync(TEST_DIR)) return [];
  return readdirSync(TEST_DIR)
    .filter((entry) => statSync(path.join(TEST_DIR, entry)).isDirectory())
    .sort();
}

function toMarkdown(results) {
  const lines = [
    "# Test Project Analysis",
    "",
    `生成时间：${new Date().toISOString()}`,
    "基线命令：`node scripts/run-test-projects.mjs --jobs 1`",
    "说明：项目集脚本当前直接调用 release binary，不会协商 Agent LLM bridge；但如果目标 repo 根存在可用的 `wiki.dev.yaml` provider 配置，core 仍会优先走 provider 直连。当前这份报告默认描述的是未提供 provider dev 覆盖时的 deterministic baseline，LLM 增强正文、provider 优先和双向协议桥接由 `crates/wiki-core/tests/llm_runtime.rs` 与 `agents/codebuddy/src/runtime/invokeCore.test.ts` 单独覆盖。",
    "",
    "## 总览",
    "",
    "| Project | Pages | Avg lines/page | Avg prose/page | Topic pages | Evidence pages | Graph pages | Mermaid | Reference delta |",
    "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |",
  ];

  for (const result of results) {
    const referenceDelta = result.reference.hasReference
      ? `${result.pageCount - result.reference.pageCount >= 0 ? "+" : ""}${result.pageCount - result.reference.pageCount}`
      : "n/a";
    lines.push(
      `| ${result.project} | ${result.pageCount} | ${result.pageMetrics.avgNonEmptyLinesPerPage} | ${result.pageMetrics.avgProseLinesPerPage} | ${result.pageMetrics.topicPages} | ${result.pageMetrics.evidenceLandingPages} | ${result.pageMetrics.graphLandingPages}/${result.pageCount} | ${result.pageMetrics.totalMermaidBlocks} | ${referenceDelta} |`,
    );
  }

  lines.push("");

  for (const result of results) {
    const density = describeDensity(result.pageMetrics);
    const architecturePage = result.pageMetrics.corePages.architecture;
    const workflowPage = result.pageMetrics.corePages.workflow;
    const overviewPage = result.pageMetrics.corePages.overview;

    lines.push(`## ${result.project}`);
    lines.push("");
    lines.push(
      `- 页面密度：${result.pageCount} 页，平均 ${result.pageMetrics.avgNonEmptyLinesPerPage} 行/页（密度${density}），其中段落 ${result.pageMetrics.avgProseLinesPerPage} 行/页、列表 ${result.pageMetrics.avgBulletLinesPerPage} 行/页；最长页面是 \`${result.pageMetrics.densestPage.title}\`（${result.pageMetrics.densestPage.nonEmptyLines} 行）。`,
    );
    lines.push(
      `- 主题与 evidence：专题页 ${result.pageMetrics.topicPages} 个，evidence 落页 ${result.pageMetrics.evidenceLandingPages} 页/${result.pageCount} 页，总计 ${result.pageMetrics.totalEvidenceBlocks} 个 evidence block。`,
    );
    lines.push(
      `- 图事实落页：${result.pageMetrics.graphLandingPages}/${result.pageCount} 页面包含 graph facts，总计 ${result.pageMetrics.totalGraphFactLines} 行；概述=${overviewPage?.graphFactLines ?? 0}、架构=${architecturePage?.graphFactLines ?? 0}、工作流=${workflowPage?.graphFactLines ?? 0}，Mermaid ${result.pageMetrics.totalMermaidBlocks} 个，落在 ${result.pageMetrics.mermaidLandingPages} 页。`,
    );
    lines.push(`- Query/图验证：${formatQuerySummary(result)}`);
    lines.push(`- Reference 对照：${describeReferenceDelta(result)}`);
    lines.push(`- 增强观测：${describeEnhancementObservation(result)}`);
    lines.push("");
  }

  return `${lines.join("\n")}\n`;
}

function parseCliArgs(argv) {
  const projects = [];
  let markdown = false;
  let write = false;

  for (const arg of argv) {
    if (arg === "--markdown") {
      markdown = true;
      continue;
    }
    if (arg === "--write") {
      write = true;
      continue;
    }
    projects.push(arg);
  }

  return { markdown, projects, write };
}

function main(argv) {
  const { markdown, projects, write } = parseCliArgs(argv);
  const names = projects.length > 0 ? projects : discoverProjects();
  const results = names.map(collectProject);
  const output = markdown ? toMarkdown(results) : `${JSON.stringify(results, null, 2)}\n`;

  if (write) {
    if (!markdown) {
      throw new Error("--write 只支持和 --markdown 一起使用");
    }
    writeFileSync(REPORT_PATH, output);
  }

  process.stdout.write(output);
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  main(process.argv.slice(2));
}
