/**
 * 汇总测试项目集的 `.wiki` runtime、2.0 SQLite 数据面与 reference 对照指标。
 *
 * 该脚本只读取 `tmp/test/*` 与 `tmp/reference/*` 的现有产物，
 * 用于验证 runtime 是否 ready、KnowledgeUnit/Research/Compose/Assemble 各层是否对齐。
 */

import { existsSync, readdirSync, statSync, writeFileSync } from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

import {
  parseSqliteNumber,
  querySqliteRows,
  querySqliteValue,
  ROOT_DIR,
  TEST_DIR,
  TMP_DIR,
} from "./testing/helpers.mjs";
import { readMarkdownPages } from "./testing/reference-fidelity.mjs";
import { inspectWikiRuntime, listMarkdownFiles } from "./testing/wiki-runtime-inspection.mjs";

const REFERENCE_DIR = path.join(TMP_DIR, "reference");
const REPORT_PATH = path.join(ROOT_DIR, "test-project-analysis.md");

function round(value) {
  return Number(value.toFixed(2));
}

function discoverProjects() {
  if (!existsSync(TEST_DIR)) {
    return [];
  }
  return readdirSync(TEST_DIR)
    .filter((entry) => statSync(path.join(TEST_DIR, entry)).isDirectory())
    .sort();
}

/**
 * 读取 reference 页面清单，作为项目分析里的页数对照基线。
 *
 * @param project 测试项目名。
 * @returns 返回 reference 是否存在及其页面列表。
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
    pageCount: listMarkdownFiles(contentDir).length,
    pageNames: listMarkdownFiles(contentDir),
  };
}

function groupedCounts(dbPath, table, column) {
  return querySqliteRows(
    dbPath,
    `select coalesce(${column}, '(null)') || char(9) || count(*) from ${table} group by ${column} order by count(*) desc, ${column} asc;`,
  ).map((row) => {
    const [name = "", count = "0"] = row.split("\t");
    return {
      name,
      count: parseSqliteNumber(count),
    };
  });
}

/**
 * 基于最终 `.wiki/*.md` 统计页面层指标。
 *
 * 只有 runtime ready 时才读取页面内容，避免把 assemble 未完成的项目误报成“空页面质量”。
 *
 * @param wikiDir `.wiki` 目录。
 * @param runtimeSnapshot runtime 检查结果。
 * @returns 返回页面层聚合指标。
 */
function buildPageMetrics(wikiDir, runtimeSnapshot) {
  const pages = runtimeSnapshot.runtimeState === "ready" ? readMarkdownPages(wikiDir) : [];
  const totalNonEmptyLines = pages.reduce((sum, page) => sum + page.nonEmptyLines, 0);
  const totalProseLines = pages.reduce((sum, page) => sum + page.proseLines, 0);
  const totalMermaidBlocks = pages.reduce((sum, page) => sum + page.mermaidBlocks, 0);
  const totalEvidenceBlocks = pages.reduce((sum, page) => sum + page.evidenceBlocks, 0);
  const topicPages = pages.filter((page) => page.category === "topic").length;
  const evidenceLandingPages = pages.filter((page) => page.evidenceBlocks > 0 || page.citations.length > 0).length;
  const mermaidLandingPages = pages.filter((page) => page.mermaidBlocks > 0).length;
  const decompositionCounts = new Map();

  for (const page of pages) {
    for (const signal of page.decompositionSignals) {
      decompositionCounts.set(signal, (decompositionCounts.get(signal) ?? 0) + 1);
    }
  }

  return {
    totalNonEmptyLines,
    totalProseLines,
    totalMermaidBlocks,
    totalEvidenceBlocks,
    topicPages,
    evidenceLandingPages,
    mermaidLandingPages,
    avgNonEmptyLinesPerPage: pages.length === 0 ? 0 : round(totalNonEmptyLines / pages.length),
    avgProseLinesPerPage: pages.length === 0 ? 0 : round(totalProseLines / pages.length),
    decompositionCounts: [...decompositionCounts.entries()].sort((left, right) => right[1] - left[1]),
  };
}

/**
 * 从 2.0 SQLite 数据面提取 planning/research/compose/assemble 聚合指标。
 *
 * @param runtimeSnapshot runtime 检查结果。
 * @param dbPath runtime SQLite 路径。
 * @returns 返回 KnowledgeUnit 主线的各层统计。
 */
function buildKnowledgeMetrics(runtimeSnapshot, dbPath) {
  const knowledgeUnitCount = runtimeSnapshot.dbCounts.knowledge_units;
  const knowledgeDomainCount = runtimeSnapshot.dbCounts.knowledge_domains;
  const unitResearchRows = parseSqliteNumber(
    querySqliteValue(dbPath, "select count(*) from research_cache where research_type = 'unit';"),
  );
  const domainResearchRows = parseSqliteNumber(
    querySqliteValue(dbPath, "select count(*) from research_cache where research_type = 'domain';"),
  );
  const systemResearchRows = parseSqliteNumber(
    querySqliteValue(dbPath, "select count(*) from research_cache where research_type = 'system';"),
  );
  const sectionPlanUnits = parseSqliteNumber(
    querySqliteValue(
      dbPath,
      [
        "select count(*)",
        "from research_cache",
        "where research_type = 'unit'",
        "and coalesce(json_array_length(json_extract(result, '$.section_plan')), 0) > 0;",
      ].join(" "),
    ),
  );

  return {
    knowledgeUnitCount,
    knowledgeDomainCount,
    unitResearchRows,
    domainResearchRows,
    systemResearchRows,
    sectionPlanUnits,
    pageDigestCount: runtimeSnapshot.dbCounts.page_digests,
    pageDraftCount: runtimeSnapshot.dbCounts.page_drafts,
    wikiPageCount: runtimeSnapshot.dbCounts.wiki_pages,
    markdownPageCount: runtimeSnapshot.markdownPageCount,
    researchCoverage: knowledgeUnitCount === 0 ? 0 : round(unitResearchRows / knowledgeUnitCount),
    composeCoverage: knowledgeUnitCount === 0 ? 0 : round(runtimeSnapshot.dbCounts.page_drafts / knowledgeUnitCount),
    assembleCoverage: knowledgeUnitCount === 0 ? 0 : round(runtimeSnapshot.dbCounts.wiki_pages / knowledgeUnitCount),
    unitTypeCounts: groupedCounts(dbPath, "knowledge_units", "unit_type"),
    domainTypeCounts: groupedCounts(dbPath, "knowledge_domains", "domain_type"),
    researchTypeCounts: groupedCounts(dbPath, "research_cache", "research_type"),
    wikiPageTypeCounts: groupedCounts(dbPath, "wiki_pages", "page_type"),
  };
}

/**
 * 汇总单个测试项目的 runtime、2.0 数据面和 reference 对照摘要。
 *
 * 返回值同时保留结构化层级视图和兼容当前脚本消费的聚合块；
 * 本轮只补注释，不收缩字段，避免在 9.6 收尾阶段引入额外契约变化。
 *
 * @param project 测试项目名。
 * @returns 返回单项目分析结果。
 */
export function collectProject(project) {
  const repoRoot = path.join(TEST_DIR, project);
  const wikiDir = path.join(repoRoot, ".wiki");
  const dbPath = path.join(wikiDir, ".cache", "wiki-cache.db");
  const runtimeSnapshot = inspectWikiRuntime(wikiDir);
  const pageMetrics = buildPageMetrics(wikiDir, runtimeSnapshot);
  const knowledgeMetrics = buildKnowledgeMetrics(runtimeSnapshot, dbPath);

  return {
    project,
    runtimeState: runtimeSnapshot.runtimeState,
    baselineClass: runtimeSnapshot.baselineClass,
    incompleteReason: runtimeSnapshot.incompleteReason,
    runtimeSnapshot,
    runtime: {
      runtimeState: runtimeSnapshot.runtimeState,
      baselineClass: runtimeSnapshot.baselineClass,
      incompleteReason: runtimeSnapshot.incompleteReason,
      dbCounts: runtimeSnapshot.dbCounts,
    },
    metadataSummary: {
      moduleCount: runtimeSnapshot.metadata?.modules?.length ?? 0,
      sourceCount: runtimeSnapshot.metadata?.source_files?.length ?? 0,
    },
    symbolCount: parseSqliteNumber(querySqliteValue(dbPath, "select count(*) from symbols;")),
    edgeCount: parseSqliteNumber(querySqliteValue(dbPath, "select count(*) from edges;")),
    communityCount: parseSqliteNumber(querySqliteValue(dbPath, "select count(*) from communities;")),
    processCount: parseSqliteNumber(querySqliteValue(dbPath, "select count(*) from processes;")),
    languageBreakdown: groupedCounts(dbPath, "symbols", "language"),
    labelBreakdown: groupedCounts(dbPath, "symbols", "label").slice(0, 8),
    pageMetrics,
    planning: {
      knowledgeUnitCount: knowledgeMetrics.knowledgeUnitCount,
      knowledgeDomainCount: knowledgeMetrics.knowledgeDomainCount,
      unitTypeBreakdown: knowledgeMetrics.unitTypeCounts,
      domainTypeBreakdown: knowledgeMetrics.domainTypeCounts,
    },
    research: {
      unitResearchCount: knowledgeMetrics.unitResearchRows,
      domainResearchCount: knowledgeMetrics.domainResearchRows,
      systemResearchCount: knowledgeMetrics.systemResearchRows,
      unitSectionPlanCount: knowledgeMetrics.sectionPlanUnits,
      coverage: knowledgeMetrics.researchCoverage,
      researchTypeBreakdown: knowledgeMetrics.researchTypeCounts,
    },
    compose: {
      pageDigestCount: knowledgeMetrics.pageDigestCount,
      pageDraftCount: knowledgeMetrics.pageDraftCount,
      coverage: knowledgeMetrics.composeCoverage,
    },
    assemble: {
      wikiPageCount: knowledgeMetrics.wikiPageCount,
      markdownPageCount: knowledgeMetrics.markdownPageCount,
      coverage: knowledgeMetrics.assembleCoverage,
      wikiPageTypeBreakdown: knowledgeMetrics.wikiPageTypeCounts,
    },
    pages: {
      pageCount: runtimeSnapshot.markdownPageCount,
      topicPageCount: pageMetrics.topicPages,
      evidencePageCount: pageMetrics.evidenceLandingPages,
      mermaidPageCount: pageMetrics.mermaidLandingPages,
      avgNonEmptyLinesPerPage: pageMetrics.avgNonEmptyLinesPerPage,
      avgProseLinesPerPage: pageMetrics.avgProseLinesPerPage,
      decompositionBreakdown: pageMetrics.decompositionCounts,
    },
    knowledgeMetrics,
    reference: collectReference(project),
  };
}

function describeReferenceDelta(result) {
  if (!result.reference.hasReference) {
    return "无 reference。";
  }

  const delta = result.runtimeSnapshot.markdownPageCount - result.reference.pageCount;
  if (result.runtimeState !== "ready") {
    return `reference=${result.reference.pageCount}，当前 runtime=${result.runtimeState}，不能直接做页级 fidelity。`;
  }
  return `generated=${result.runtimeSnapshot.markdownPageCount} / reference=${result.reference.pageCount}（delta=${delta >= 0 ? "+" : ""}${delta}）`;
}

/**
 * 将项目分析结果渲染为 Markdown 报告。
 *
 * @param results 项目分析结果数组。
 * @returns 返回 Markdown 文本。
 */
function toMarkdown(results) {
  const lines = [
    "# Test Project Analysis",
    "",
    `生成时间：${new Date().toISOString()}`,
    "说明：当前报告只基于 2.0 的 `knowledge_units / research_cache / page_drafts / wiki_pages` 与最终 `.wiki/*.md` 读取，不再依赖旧 `page_context_cache.context.research_result` 或 `topic_dossier`。",
    "",
    "## 总览",
    "",
    "| Project | Runtime | Pages | KnowledgeUnits | UnitResearch | SectionPlan | PageDrafts | WikiPages | TopicPages | Reference Delta |",
    "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |",
  ];

  for (const result of results) {
    lines.push(
      `| ${result.project} | ${result.runtimeState} | ${result.runtimeSnapshot.markdownPageCount} | ${result.knowledgeMetrics.knowledgeUnitCount} | ${result.knowledgeMetrics.unitResearchRows} | ${result.knowledgeMetrics.sectionPlanUnits} | ${result.knowledgeMetrics.pageDraftCount} | ${result.knowledgeMetrics.wikiPageCount} | ${result.pageMetrics.topicPages} | ${describeReferenceDelta(result)} |`,
    );
  }

  lines.push("");

  for (const result of results) {
    lines.push(`## ${result.project}`);
    lines.push("");
    lines.push(`- runtime：${result.runtimeState} / ${result.baselineClass} / ${result.incompleteReason ?? "ready"}`);
    lines.push(`- 2.0 pipeline：knowledge_units=${result.knowledgeMetrics.knowledgeUnitCount}，knowledge_domains=${result.knowledgeMetrics.knowledgeDomainCount}，unit_research=${result.knowledgeMetrics.unitResearchRows}，section_plan_units=${result.knowledgeMetrics.sectionPlanUnits}，page_drafts=${result.knowledgeMetrics.pageDraftCount}，wiki_pages=${result.knowledgeMetrics.wikiPageCount}，markdown=${result.knowledgeMetrics.markdownPageCount}`);
    lines.push(`- 覆盖率：research=${result.knowledgeMetrics.researchCoverage}，compose=${result.knowledgeMetrics.composeCoverage}，assemble=${result.knowledgeMetrics.assembleCoverage}`);
    lines.push(`- UnitType：${result.knowledgeMetrics.unitTypeCounts.slice(0, 8).map((item) => `${item.name}(${item.count})`).join("、") || "无"}`);
    lines.push(`- DomainType：${result.knowledgeMetrics.domainTypeCounts.slice(0, 8).map((item) => `${item.name}(${item.count})`).join("、") || "无"}`);
    lines.push(`- ResearchType：${result.knowledgeMetrics.researchTypeCounts.map((item) => `${item.name}(${item.count})`).join("、") || "无"}`);
    if (result.runtimeState === "ready") {
      lines.push(`- 页面质量：topic_pages=${result.pageMetrics.topicPages}，evidence_pages=${result.pageMetrics.evidenceLandingPages}，mermaid_pages=${result.pageMetrics.mermaidLandingPages}，avg_lines=${result.pageMetrics.avgNonEmptyLinesPerPage}，avg_prose=${result.pageMetrics.avgProseLinesPerPage}`);
      lines.push(`- 页面分解信号：${result.pageMetrics.decompositionCounts.slice(0, 8).map(([name, count]) => `${name}(${count})`).join("、") || "无"}`);
    } else {
      lines.push("- 页面质量：runtime 尚未 ready，本轮只输出 DB 诊断，不给页级质量结论。");
    }
    lines.push(`- 图事实/符号：symbols=${result.symbolCount}，edges=${result.edgeCount}，communities=${result.communityCount}，processes=${result.processCount}`);
    lines.push(`- 语言分布：${result.languageBreakdown.slice(0, 8).map((item) => `${item.name}(${item.count})`).join("、") || "无"}`);
    lines.push(`- reference：${describeReferenceDelta(result)}`);
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

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main(process.argv.slice(2));
}
