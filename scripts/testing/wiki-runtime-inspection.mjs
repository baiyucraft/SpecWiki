/**
 * 统一检查 `.wiki/` runtime 是否已经进入可验收状态。
 *
 * 上游消费方包括 reference fidelity 报告和 test-project 分析脚本；
 * 本模块只负责读取最终产物与 SQLite 摘要，不负责生成或修复 runtime。
 */

import { existsSync, readdirSync, readFileSync } from "node:fs";
import path from "node:path";

import {
  parseSqliteNumber,
  querySqliteRows,
  querySqliteValue,
} from "./helpers.mjs";

export const RUNTIME_COUNT_TABLES = [
  "knowledge_units",
  "knowledge_domains",
  "research_cache",
  "page_digests",
  "page_drafts",
  "wiki_pages",
  "pipeline_checkpoint",
];

/**
 * 递归列出目录下的 Markdown 文件。
 *
 * @param dir 起始目录。
 * @returns 返回相对路径数组；统一使用 `/` 作为分隔符。
 */
export function listMarkdownFiles(dir) {
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

function quoteSqliteName(name) {
  return `"${String(name).replaceAll("\"", "\"\"")}"`;
}

function hasRuntimeSignals(dbCounts) {
  return Object.entries(dbCounts)
    .some(([name, count]) => name !== "pipeline_checkpoint" && count > 0);
}

/**
 * 从 runtime SQLite 里读取关键表计数。
 *
 * @param cacheDbPath `.wiki/.cache/wiki-cache.db` 路径。
 * @param tables 要统计的表名列表。
 * @returns 返回 `{ tableName: count }` 结构。
 */
export function readRuntimeDbCounts(cacheDbPath, tables = RUNTIME_COUNT_TABLES) {
  if (!existsSync(cacheDbPath)) {
    return Object.fromEntries(tables.map((name) => [name, 0]));
  }

  return Object.fromEntries(
    tables.map((name) => {
      const count = parseSqliteNumber(
        querySqliteValue(cacheDbPath, `select count(*) from ${quoteSqliteName(name)};`),
      );
      return [name, count];
    }),
  );
}

/**
 * 读取首个 pipeline checkpoint，帮助解释 assemble 中断在哪里。
 *
 * @param cacheDbPath `.wiki/.cache/wiki-cache.db` 路径。
 * @returns 返回 checkpoint 摘要；没有 checkpoint 时返回 `null`。
 */
export function readPipelineCheckpoint(cacheDbPath) {
  const rows = querySqliteRows(
    cacheDbPath,
    [
      "select",
      "coalesce(checkpoint_id, ''),",
      "coalesce(interrupted_stage, ''),",
      "coalesce(interrupted_target_id, ''),",
      "coalesce(error_message, '')",
      "from pipeline_checkpoint",
      "order by created_at desc",
      "limit 1;",
    ].join(" "),
  );
  if (rows.length === 0) {
    return null;
  }

  const [checkpointId = "", stage = "", targetId = "", errorMessage = ""] = rows[0].split("|");
  return {
    checkpointId,
    stage,
    targetId,
    errorMessage,
  };
}

function classifyRuntimeState(snapshot) {
  const hasAnyRuntimeArtifact =
    snapshot.metadataExists
    || snapshot.cacheDbExists
    || snapshot.markdownPageCount > 0
    || hasRuntimeSignals(snapshot.dbCounts);
  if (!hasAnyRuntimeArtifact) {
    return "missing";
  }

  const hasRenderablePages =
    snapshot.metadataExists
    && snapshot.markdownPageCount > 0
    && snapshot.cacheDbExists
    && snapshot.dbCounts.wiki_pages > 0;
  if (hasRenderablePages) {
    return "ready";
  }

  return "runtime_incomplete";
}

function inferIncompleteReason(snapshot) {
  if (snapshot.runtimeState === "missing") {
    return "未发现 `.wiki` runtime 产物";
  }
  if (!snapshot.cacheDbExists) {
    return "缺少 `.wiki/.cache/wiki-cache.db`";
  }
  if (!snapshot.metadataExists && snapshot.markdownPageCount === 0 && hasRuntimeSignals(snapshot.dbCounts)) {
    return "已有 knowledge/research 数据，但 assemble 尚未写出 metadata 与 Markdown";
  }
  if (!snapshot.metadataExists) {
    return "缺少 `wiki.metadata.json`";
  }
  if (snapshot.markdownPageCount === 0) {
    return "缺少最终 Markdown 页面";
  }
  if (snapshot.dbCounts.wiki_pages === 0) {
    return "SQLite 中 `wiki_pages=0`，页面装配尚未完成";
  }
  return "runtime 产物不完整";
}

/**
 * 读取 `.wiki/` 的 runtime 快照并给出可验收性结论。
 *
 * @param wikiDir `.wiki` 目录路径。
 * @returns 返回 runtime 快照。
 */
export function inspectWikiRuntime(wikiDir) {
  const metadataPath = path.join(wikiDir, "wiki.metadata.json");
  const cacheDbPath = path.join(wikiDir, ".cache", "wiki-cache.db");
  const wikiDirExists = existsSync(wikiDir);
  const metadataExists = existsSync(metadataPath);
  const cacheDbExists = existsSync(cacheDbPath);
  const markdownFiles = listMarkdownFiles(wikiDir);
  const dbCounts = readRuntimeDbCounts(cacheDbPath);
  const checkpoint = readPipelineCheckpoint(cacheDbPath);
  const metadata = metadataExists ? JSON.parse(readFileSync(metadataPath, "utf-8")) : null;

  const snapshot = {
    wikiDir,
    wikiDirExists,
    metadataPath,
    metadataExists,
    cacheDbPath,
    cacheDbExists,
    markdownFiles,
    markdownPageCount: markdownFiles.length,
    dbCounts,
    checkpoint,
    metadata,
  };

  const runtimeState = classifyRuntimeState(snapshot);
  return {
    ...snapshot,
    runtimeState,
    incompleteReason: runtimeState === "ready" ? null : inferIncompleteReason({ ...snapshot, runtimeState }),
    baselineClass: runtimeState === "ready" ? "acceptance_candidate" : "diagnostic_only",
  };
}

/**
 * 为需要 hard gate 的流程断言 runtime 已 ready。
 *
 * @param snapshot `inspectWikiRuntime()` 的返回值。
 * @param options 可选附加信息；目前只用于拼接项目名。
 * @returns 返回原始 snapshot，便于链式调用。
 */
export function requireReadyRuntime(snapshot, options = {}) {
  if (snapshot.runtimeState === "ready") {
    return snapshot;
  }

  const prefix = options.project ? `${options.project} ` : "";
  const checkpointSummary = snapshot.checkpoint
    ? ` stage=${snapshot.checkpoint.stage || "unknown"} target=${snapshot.checkpoint.targetId || "n/a"}`
    : "";
  throw new Error(
    `${prefix}runtime is ${snapshot.runtimeState}: ${snapshot.incompleteReason}.${checkpointSummary}`.trim(),
  );
}
