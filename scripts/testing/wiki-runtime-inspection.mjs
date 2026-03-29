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
  "unit_runtime_gates",
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

function safeQuerySqliteRows(cacheDbPath, sql) {
  try {
    return querySqliteRows(cacheDbPath, sql);
  } catch {
    return [];
  }
}

function safeQuerySqliteValue(cacheDbPath, sql) {
  try {
    return querySqliteValue(cacheDbPath, sql);
  } catch {
    return "";
  }
}

function normalizeSourcePath(value) {
  return String(value)
    .replaceAll("\\", "/")
    .replace(/#L\d+(?:-L?\d+)?$/i, "");
}

function sqliteTableExists(cacheDbPath, tableName) {
  return safeQuerySqliteValue(
    cacheDbPath,
    [
      "select count(*)",
      "from sqlite_master",
      "where type = 'table'",
      `and name = '${String(tableName).replaceAll("'", "''")}';`,
    ].join(" "),
  ) === "1";
}

function sqliteColumnExists(cacheDbPath, tableName, columnName) {
  if (!sqliteTableExists(cacheDbPath, tableName)) {
    return false;
  }

  return safeQuerySqliteRows(
    cacheDbPath,
    `pragma table_info(${quoteSqliteName(tableName)});`,
  ).some((row) => {
    const [, name = ""] = row.split("|");
    return name === columnName;
  });
}

function escapeSqliteString(value) {
  return String(value ?? "").replaceAll("'", "''");
}

function parseJsonObject(raw) {
  if (!raw) {
    return null;
  }

  try {
    const parsed = JSON.parse(raw);
    return parsed && typeof parsed === "object" ? parsed : null;
  } catch {
    return null;
  }
}

function readJsonValue(source, keys) {
  if (!source || typeof source !== "object") {
    return null;
  }

  for (const key of keys) {
    const value = source[key];
    if (value !== undefined && value !== null && value !== "") {
      return value;
    }
  }

  return null;
}

function readJsonString(source, keys) {
  const value = readJsonValue(source, keys);
  return value === null ? null : String(value);
}

function readJsonNumber(source, keys) {
  const value = readJsonValue(source, keys);
  if (value === null) {
    return null;
  }
  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : null;
}

function readJsonBoolean(source, keys) {
  const value = readJsonValue(source, keys);
  if (typeof value === "boolean") {
    return value;
  }
  if (value === "true") {
    return true;
  }
  if (value === "false") {
    return false;
  }
  return null;
}

function findLatestUnitResearchId(cacheDbPath) {
  if (!sqliteTableExists(cacheDbPath, "research_cache")) {
    return null;
  }

  const targetId = safeQuerySqliteValue(
    cacheDbPath,
    [
      "select coalesce(target_id, '')",
      "from research_cache",
      "where research_type = 'unit'",
      "order by rowid desc, target_id asc",
      "limit 1;",
    ].join(" "),
  );
  return targetId || null;
}

function readUnitResearchResult(cacheDbPath, unitId) {
  if (!unitId || !sqliteColumnExists(cacheDbPath, "research_cache", "result")) {
    return null;
  }

  const resultJson = safeQuerySqliteValue(
    cacheDbPath,
    [
      "select coalesce(result, '')",
      "from research_cache",
      "where research_type = 'unit'",
      `and target_id = '${escapeSqliteString(unitId)}'`,
      "limit 1;",
    ].join(" "),
  );
  return parseJsonObject(resultJson);
}

function summarizeProviderStats(unitId, researchResult) {
  if (!researchResult) {
    return null;
  }

  const sessionStats = readJsonValue(researchResult, ["provider_session_stats", "providerSessionStats"]);
  const stopReason = readJsonString(researchResult, ["provider_stop_reason", "providerStopReason"]);
  const turnsUsed = readJsonNumber(sessionStats, ["turns_used", "turnsUsed"]);
  const toolCalls = readJsonNumber(sessionStats, ["tool_calls", "toolCalls"]);
  const elapsedMs = readJsonNumber(sessionStats, ["elapsed_ms", "elapsedMs"]);
  const cacheHit = readJsonBoolean(sessionStats, ["cache_hit", "cacheHit"]);
  const toolsMode = readJsonString(sessionStats, ["tools_mode", "toolsMode"]);
  const retryInputApplied = readJsonBoolean(sessionStats, ["retry_input_applied", "retryInputApplied"]);

  if (
    !stopReason
    && turnsUsed === null
    && toolCalls === null
    && elapsedMs === null
    && cacheHit === null
    && !toolsMode
    && retryInputApplied === null
  ) {
    return null;
  }

  return {
    unitId: unitId || null,
    stopReason,
    turnsUsed,
    toolCalls,
    elapsedMs,
    cacheHit,
    toolsMode,
    retryInputApplied,
  };
}

/**
 * 把 runtime summary 与 research_cache.result 归一成脚本可消费的最小研究进度摘要。
 *
 * `pipeline_runtime_summary` 负责描述“当前在跑谁”和“最近完成了谁”，
 * `research_cache.result.provider_session_stats` 负责补最近一次 provider 观测指标。
 *
 * @param cacheDbPath `.wiki/.cache/wiki-cache.db` 路径。
 * @param runtimeSummary workflow 级 runtime summary。
 * @returns 返回兼容旧数据的研究进度摘要；字段缺失时统一退化为 `null`。
 */
function readResearchProgressSummary(cacheDbPath, runtimeSummary) {
  const currentUnitId = readJsonString(runtimeSummary, [
    "current_research_unit_id",
    "currentResearchUnitId",
  ]);
  const currentUnitType = readJsonString(runtimeSummary, [
    "current_research_unit_type",
    "currentResearchUnitType",
  ]);
  const currentStartedAt = readJsonString(runtimeSummary, [
    "current_research_started_at",
    "currentResearchStartedAt",
  ]);
  const lastUnitId
    = readJsonString(runtimeSummary, ["last_researched_unit_id", "lastResearchedUnitId"])
      ?? findLatestUnitResearchId(cacheDbPath);
  const lastResearchResult = readUnitResearchResult(cacheDbPath, lastUnitId);
  const lastElapsedMs
    = readJsonNumber(runtimeSummary, ["last_research_elapsed_ms", "lastResearchElapsedMs"])
      ?? readJsonNumber(
      readJsonValue(lastResearchResult, ["provider_session_stats", "providerSessionStats"]),
      ["elapsed_ms", "elapsedMs"],
    );

  return {
    currentResearchUnit:
      currentUnitId || currentUnitType || currentStartedAt
        ? {
          unitId: currentUnitId,
          unitType: currentUnitType,
          startedAt: currentStartedAt,
        }
        : null,
    lastCompletedResearch:
      lastUnitId || lastElapsedMs !== null
        ? {
          unitId: lastUnitId,
          elapsedMs: lastElapsedMs,
        }
        : null,
    providerStats: summarizeProviderStats(lastUnitId, lastResearchResult),
  };
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
      if (!sqliteTableExists(cacheDbPath, name)) {
        return [name, 0];
      }
      const count = parseSqliteNumber(
        safeQuerySqliteValue(cacheDbPath, `select count(*) from ${quoteSqliteName(name)};`),
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
  if (!sqliteTableExists(cacheDbPath, "pipeline_checkpoint")) {
    return null;
  }
  const rows = safeQuerySqliteRows(
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

/**
 * 读取 workflow 级 runtime summary，优先解释当前 pipeline 停在哪个阶段。
 *
 * @param cacheDbPath `.wiki/.cache/wiki-cache.db` 路径。
 * @returns 返回 runtime summary；不存在或损坏时返回 `null`。
 */
export function readPipelineRuntimeSummary(cacheDbPath) {
  if (!sqliteTableExists(cacheDbPath, "runtime_meta")) {
    return null;
  }
  const raw = safeQuerySqliteValue(
    cacheDbPath,
    [
      "select coalesce(value, '')",
      "from runtime_meta",
      "where key = 'pipeline_runtime_summary'",
      "limit 1;",
    ].join(" "),
  );
  if (!raw) {
    return null;
  }

  try {
    return JSON.parse(raw);
  } catch {
    return null;
  }
}

/**
 * 读取 unit 级 runtime gate，帮助报告脚本定位哪些页面还没进入 compose / assemble。
 *
 * @param cacheDbPath `.wiki/.cache/wiki-cache.db` 路径。
 * @returns 返回 unit gate 列表；表不存在时返回空数组。
 */
export function readUnitRuntimeGates(cacheDbPath) {
  if (!sqliteTableExists(cacheDbPath, "unit_runtime_gates")) {
    return [];
  }
  const rows = safeQuerySqliteRows(
    cacheDbPath,
    [
      "select",
      "coalesce(unit_id, ''),",
      "coalesce(unit_type, ''),",
      "coalesce(research_status, ''),",
      "coalesce(compose_status, ''),",
      "coalesce(assemble_status, ''),",
      "coalesce(last_ready_stage, ''),",
      "coalesce(blocked_reason, ''),",
      "coalesce(missing_dependencies, '[]')",
      "from unit_runtime_gates",
      "order by unit_id asc;",
    ].join(" "),
  );

  return rows.map((row) => {
    const [
      unitId = "",
      unitType = "",
      researchStatus = "",
      composeStatus = "",
      assembleStatus = "",
      lastReadyStage = "",
      blockedReason = "",
      missingDependencies = "[]",
    ] = row.split("|");
    return {
      unitId,
      unitType,
      researchStatus,
      composeStatus,
      assembleStatus,
      lastReadyStage: lastReadyStage || null,
      blockedReason: blockedReason || null,
      missingDependencies: JSON.parse(missingDependencies || "[]"),
    };
  });
}

function summarizeUnitRuntimeGates(unitRuntimeGates) {
  return {
    total: unitRuntimeGates.length,
    composeReady: unitRuntimeGates.filter((gate) => gate.composeStatus === "ready").length,
    composePending: unitRuntimeGates.filter((gate) => gate.composeStatus === "pending").length,
    composeBlocked: unitRuntimeGates.filter((gate) => gate.composeStatus === "blocked").length,
    assembleDone: unitRuntimeGates.filter((gate) => gate.assembleStatus === "done").length,
    blockedUnits: unitRuntimeGates
      .filter((gate) => gate.blockedReason || gate.missingDependencies.length > 0)
      .map((gate) => ({
        unitId: gate.unitId,
        composeStatus: gate.composeStatus,
        blockedReason: gate.blockedReason,
        missingDependencies: gate.missingDependencies,
      })),
  };
}

function readParentContractMetrics(cacheDbPath) {
  if (!sqliteTableExists(cacheDbPath, "page_context_cache")) {
    return {
      parentPages: 0,
      composeReadyParents: 0,
      missingReadinessParents: 0,
      childDigestParents: 0,
      childPageParents: 0,
      citationDigestParents: 0,
      diagramDigestParents: 0,
    };
  }
  const row = safeQuerySqliteRows(
    cacheDbPath,
    [
      "select",
      "count(*),",
      "sum(case when coalesce(json_extract(context, '$.readiness_status'), '') = 'compose_ready' then 1 else 0 end),",
      "sum(case when coalesce(json_extract(context, '$.readiness_status'), '') = '' then 1 else 0 end),",
      "sum(case when coalesce(json_array_length(json_extract(context, '$.child_digest_ids')), 0) > 0 then 1 else 0 end),",
      "sum(case when coalesce(json_array_length(json_extract(context, '$.child_page_ids')), 0) > 0 then 1 else 0 end),",
      "sum(case when coalesce(json_array_length(json_extract(context, '$.citation_digest_refs')), 0) > 0 then 1 else 0 end),",
      "sum(case when coalesce(json_array_length(json_extract(context, '$.diagram_digest_refs')), 0) > 0 then 1 else 0 end)",
      "from page_context_cache",
      "where coalesce(json_extract(context, '$.page_type'), '') in ('overview', 'architecture', 'domain-index');",
    ].join(" "),
  )[0];

  const [
    parentPages = "0",
    composeReadyParents = "0",
    missingReadinessParents = "0",
    childDigestParents = "0",
    childPageParents = "0",
    citationDigestParents = "0",
    diagramDigestParents = "0",
  ] = (row ?? "").split("|");

  return {
    parentPages: parseSqliteNumber(parentPages),
    composeReadyParents: parseSqliteNumber(composeReadyParents),
    missingReadinessParents: parseSqliteNumber(missingReadinessParents),
    childDigestParents: parseSqliteNumber(childDigestParents),
    childPageParents: parseSqliteNumber(childPageParents),
    citationDigestParents: parseSqliteNumber(citationDigestParents),
    diagramDigestParents: parseSqliteNumber(diagramDigestParents),
  };
}

/**
 * 从 `page_digests` 读取 compose 诊断字段，并按 unit_id 组织回页面级视图。
 *
 * @param {string} cacheDbPath runtime cache 数据库路径。
 * @returns {{byUnitId: Record<string, object>, summary: object}} 返回逐页诊断映射与聚合摘要；老 schema 会返回空结果。
 */
function readPageDigestDiagnostics(cacheDbPath) {
  if (
    !sqliteTableExists(cacheDbPath, "page_digests")
    || !sqliteColumnExists(cacheDbPath, "page_digests", "digest")
  ) {
    return {
      byUnitId: {},
      summary: {
        digestPages: 0,
        pagesWithSkeletonProfile: 0,
        pagesWithSectionGroundingRefs: 0,
        pagesWithPlannedKeySources: 0,
        pagesWithGroundedKeySources: 0,
        pagesWithGroundingGap: 0,
      },
    };
  }

  const rows = safeQuerySqliteRows(
    cacheDbPath,
    "select coalesce(unit_id, ''), coalesce(digest, '{}') from page_digests order by unit_id asc;",
  );
  const byUnitId = {};
  let pagesWithSkeletonProfile = 0;
  let pagesWithSectionGroundingRefs = 0;
  let pagesWithPlannedKeySources = 0;
  let pagesWithGroundedKeySources = 0;
  let pagesWithGroundingGap = 0;

  for (const row of rows) {
    const [unitId = "", digestJson = "{}"] = row.split("|");
    const digest = parseJsonObject(digestJson);
    const plannedKeySources = Array.isArray(digest?.planned_key_sources)
      ? digest.planned_key_sources.map((item) => String(item))
      : [];
    const groundedKeySources = Array.isArray(digest?.grounded_key_sources)
      ? digest.grounded_key_sources.map((item) => String(item))
      : [];
    const sectionGroundingRefs = Array.isArray(digest?.section_grounding_refs)
      ? digest.section_grounding_refs
      : [];
    const skeletonProfile = digest?.skeleton_profile && typeof digest.skeleton_profile === "object"
      ? digest.skeleton_profile
      : null;

    const groundedBasenames = new Set(
      groundedKeySources.map((item) => path.posix.basename(normalizeSourcePath(item)).toLowerCase()),
    );
    const missingGroundedSources = plannedKeySources.filter((item) => {
      const normalized = normalizeSourcePath(item);
      const basename = path.posix.basename(normalized).toLowerCase();
      return !groundedKeySources.some((candidate) => normalizeSourcePath(candidate) === normalized)
        && !groundedBasenames.has(basename);
    });

    if (skeletonProfile) {
      pagesWithSkeletonProfile += 1;
    }
    if (sectionGroundingRefs.length > 0) {
      pagesWithSectionGroundingRefs += 1;
    }
    if (plannedKeySources.length > 0) {
      pagesWithPlannedKeySources += 1;
    }
    if (groundedKeySources.length > 0) {
      pagesWithGroundedKeySources += 1;
    }
    if (plannedKeySources.length > 0 && missingGroundedSources.length > 0) {
      pagesWithGroundingGap += 1;
    }

    byUnitId[unitId] = {
      unitId,
      plannedKeySources,
      groundedKeySources,
      missingGroundedSources,
      sectionGroundingRefs,
      sectionGroundingRefCount: sectionGroundingRefs.length,
      skeletonProfile,
      skeletonProfileKey: skeletonProfile?.profile_key ?? null,
      plannedKeySourceCount: plannedKeySources.length,
      groundedKeySourceCount: groundedKeySources.length,
      groundingGapCount: missingGroundedSources.length,
      groundingGap: plannedKeySources.length > 0 && missingGroundedSources.length > 0,
      readinessStage: readJsonString(digest, ["readiness_stage", "readinessStage"]) ?? "",
      digestTitle: readJsonString(digest, ["title"]) ?? null,
    };
  }

  return {
    byUnitId,
    summary: {
      digestPages: rows.length,
      pagesWithSkeletonProfile,
      pagesWithSectionGroundingRefs,
      pagesWithPlannedKeySources,
      pagesWithGroundedKeySources,
      pagesWithGroundingGap,
    },
  };
}

function normalizeWikiItemPath(itemPath) {
  return String(itemPath ?? "")
    .replaceAll("\\", "/")
    .replace(/^\.wiki\//, "");
}

function readPageRuntimeByPath(cacheDbPath, metadata, pageDigestDiagnostics) {
  if (!sqliteTableExists(cacheDbPath, "page_context_cache")) {
    return {};
  }
  const contextRows = safeQuerySqliteRows(
    cacheDbPath,
    "select coalesce(page_id, ''), coalesce(context, '{}') from page_context_cache order by page_id asc;",
  );
  const contextByPageId = new Map(
    contextRows.map((row) => {
      const [pageId = "", contextJson = "{}"] = row.split("|");
      try {
        return [pageId, JSON.parse(contextJson)];
      } catch {
        return [pageId, null];
      }
    }),
  );

  return Object.fromEntries(
    (metadata?.wiki_items ?? [])
      .map((item) => {
        const context = contextByPageId.get(item.id);
        if (!context) {
          return null;
        }
        const digestDiagnostics = context.unit_id
          ? pageDigestDiagnostics?.byUnitId?.[context.unit_id] ?? null
          : null;
        return [
          normalizeWikiItemPath(item.path),
          {
            pageId: item.id,
            unitId: context.unit_id ?? null,
            unitType: context.unit_type ?? null,
            domainId: context.domain_id ?? null,
            readinessStatus: context.readiness_status ?? "",
            childDigestCount: Array.isArray(context.child_digest_ids)
              ? context.child_digest_ids.length
              : 0,
            plannedKeySources: digestDiagnostics?.plannedKeySources ?? [],
            groundedKeySources: digestDiagnostics?.groundedKeySources ?? [],
            missingGroundedSources: digestDiagnostics?.missingGroundedSources ?? [],
            plannedKeySourceCount: digestDiagnostics?.plannedKeySourceCount ?? 0,
            groundedKeySourceCount: digestDiagnostics?.groundedKeySourceCount ?? 0,
            groundingGapCount: digestDiagnostics?.groundingGapCount ?? 0,
            groundingGap: digestDiagnostics?.groundingGap ?? false,
            sectionGroundingRefs: digestDiagnostics?.sectionGroundingRefs ?? [],
            sectionGroundingRefCount: digestDiagnostics?.sectionGroundingRefCount ?? 0,
            skeletonProfile: digestDiagnostics?.skeletonProfile ?? null,
            skeletonProfileKey: digestDiagnostics?.skeletonProfileKey ?? null,
            digestReadinessStage: digestDiagnostics?.readinessStage ?? "",
          },
        ];
      })
      .filter(Boolean),
  );
}

function classifyRuntimeState(snapshot) {
  const hasAnyRuntimeArtifact
    = snapshot.metadataExists
      || snapshot.cacheDbExists
      || snapshot.markdownPageCount > 0
      || hasRuntimeSignals(snapshot.dbCounts);
  if (!hasAnyRuntimeArtifact) {
    return "missing";
  }

  const hasRenderablePages
    = snapshot.metadataExists
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
  const runtimeState = snapshot.runtimeSummary?.runtime_state ?? "";
  const interruptedStage = snapshot.runtimeSummary?.last_interrupted_stage ?? "";
  if (runtimeState === "researching") {
    return "workflow 仍在 research，尚未进入 compose";
  }
  if (runtimeState === "compose_pending") {
    return "research 已完成，但 compose 尚未完成";
  }
  if (runtimeState === "compose_complete") {
    return "compose 已完成，但 assemble 尚未写出 metadata 与 Markdown";
  }
  if (runtimeState === "interrupted") {
    if (interruptedStage.startsWith("research")) {
      return `research 阶段中断：${interruptedStage}`;
    }
    if (interruptedStage.startsWith("compose")) {
      return `compose 阶段中断：${interruptedStage}`;
    }
    if (interruptedStage) {
      return `workflow 中断：${interruptedStage}`;
    }
    return "workflow 已中断，但缺少明确阶段信息";
  }
  if (!snapshot.metadataExists && snapshot.markdownPageCount === 0 && hasRuntimeSignals(snapshot.dbCounts)) {
    if (runtimeState) {
      return `已有 knowledge/research 数据，pipeline_runtime_summary.runtime_state=${runtimeState}，但 assemble 尚未写出 metadata 与 Markdown`;
    }
    return "已有 knowledge/research 数据，但 assemble 尚未写出 metadata 与 Markdown";
  }
  if (!snapshot.metadataExists) {
    return "缺少 `wiki.metadata.json`";
  }
  if (snapshot.markdownPageCount === 0) {
    return "缺少最终 Markdown 页面";
  }
  if (snapshot.dbCounts.wiki_pages === 0) {
    if (runtimeState) {
      return `SQLite 中 wiki_pages=0，pipeline_runtime_summary.runtime_state=${runtimeState}`;
    }
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
  const runtimeSummary = readPipelineRuntimeSummary(cacheDbPath);
  const unitRuntimeGates = readUnitRuntimeGates(cacheDbPath);
  const runtimeGateSummary = summarizeUnitRuntimeGates(unitRuntimeGates);
  const researchProgressSummary = readResearchProgressSummary(cacheDbPath, runtimeSummary);
  const parentContract = readParentContractMetrics(cacheDbPath);
  const pageDigestDiagnostics = readPageDigestDiagnostics(cacheDbPath);
  const metadata = metadataExists ? JSON.parse(readFileSync(metadataPath, "utf-8")) : null;
  const pageRuntimeByPath = readPageRuntimeByPath(cacheDbPath, metadata, pageDigestDiagnostics);

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
    runtimeSummary,
    unitRuntimeGates,
    runtimeGateSummary,
    researchProgressSummary,
    parentContract,
    composeDiagnostics: pageDigestDiagnostics.summary,
    pageRuntimeByPath,
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
