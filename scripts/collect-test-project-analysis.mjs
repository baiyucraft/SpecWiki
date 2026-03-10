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
} from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

import {
  TEST_DIR,
  TMP_DIR,
  callCore,
  countFilesWithMarker,
  countMdFiles,
} from "./test-helpers.mjs";

const REFERENCE_DIR = path.join(TMP_DIR, "reference");

/**
 * 读取 sqlite 单值查询结果。
 *
 * @param dbPath SQLite 文件路径。
 * @param sql 需要执行的查询语句。
 * @returns 去掉首尾空白后的结果文本。
 */
function querySqlite(dbPath, sql) {
  return execFileSync("sqlite3", [dbPath, sql], {
    encoding: "utf-8",
    timeout: 30_000,
  }).trim();
}

/**
 * 读取 sqlite 表格查询结果。
 *
 * @param dbPath SQLite 文件路径。
 * @param sql 需要执行的查询语句。
 * @returns 按行切分后的结果。
 */
function querySqliteRows(dbPath, sql) {
  const output = querySqlite(dbPath, sql);
  return output ? output.split(/\r?\n/).filter(Boolean) : [];
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
    };
  }

  return {
    hasReference: true,
    pageCount: countMdFiles(contentDir),
  };
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

  const pageCount = countMdFiles(wikiDir);
  const markerCount = countFilesWithMarker(wikiDir);
  const moduleCount = metadata.modules?.length ?? 0;
  const sourceCount = metadata.source_files?.length ?? 0;
  const symbolCount = Number(querySqlite(dbPath, "select count(*) from symbols;") || "0");
  const exportedSymbolCount = Number(
    querySqlite(dbPath, "select count(*) from symbols where is_exported = 1;") || "0",
  );
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

  return {
    project,
    pageCount,
    markerCount,
    moduleCount,
    sourceCount,
    symbolCount,
    exportedSymbolCount,
    languageBreakdown,
    labelBreakdown,
    representativeSymbol,
    querySummary,
    reference: collectReference(project),
  };
}

function discoverProjects() {
  if (!existsSync(TEST_DIR)) return [];
  return readdirSync(TEST_DIR)
    .filter((entry) => statSync(path.join(TEST_DIR, entry)).isDirectory())
    .sort();
}

function main(projects) {
  const names = projects.length > 0 ? projects : discoverProjects();
  const results = names.map(collectProject);
  process.stdout.write(`${JSON.stringify(results, null, 2)}\n`);
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  main(process.argv.slice(2));
}
