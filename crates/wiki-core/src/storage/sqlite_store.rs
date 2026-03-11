//! SQLite 统一缓存与状态存储层。
//! 迭代 6.5 起，SQLite 不再只是 `kv_store` 容器，而是 runtime 的事实主存储。

use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::{Path, PathBuf};

use rusqlite::{
    params, params_from_iter, types::Value, Connection, OpenFlags, OptionalExtension, ToSql,
    Transaction,
};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::domain::metadata::DirtyState;
use crate::domain::module_tree::ModuleNode;
use crate::domain::relation::WikiRelation;
use crate::domain::state::{BuildState, SourceState, WikiPageState, WikiSectionState, WikiState};
use crate::repo::symbol_graph::{
    CommunityMember, CommunityNode, GraphAnalysisSnapshot, ProcessNode, ProcessStep,
    ResolvedGraphSnapshot, ResolvedSymbolEdge,
};
use crate::repo::symbols::SymbolNode;
use crate::storage::cache_store::{cache_dir, ensure_cache_dir};

/// DB 文件名。
const DB_FILENAME: &str = "wiki-cache.db";

/// 页面 FTS 命中结果。
#[derive(Debug, Clone)]
pub struct FtsPageHit {
    /// 命中页面 ID。
    pub page_id: String,
    /// 命中页面标题。
    pub title: String,
    /// 命中页面路径。
    pub path: String,
    /// BM25 分数。
    pub score: f64,
}

/// 符号 FTS 命中结果。
#[derive(Debug, Clone)]
pub struct FtsSymbolHit {
    /// 命中符号 ID。
    pub symbol_id: String,
    /// 命中符号名。
    pub name: String,
    /// 符号标签。
    pub label: String,
    /// 命中符号所在文件。
    pub file_path: String,
    /// 命中符号语言。
    pub language: String,
    /// BM25 分数。
    pub score: f64,
}

/// 基于 `edges` 表递归追踪得到的调用链边。
#[derive(Debug, Clone)]
pub struct GraphTraceEdgeHit {
    /// 图边稳定 ID。
    pub edge_id: String,
    /// 边起点 symbol ID。
    pub source_id: String,
    /// 边终点 symbol ID。
    pub target_id: String,
    /// 追踪方向：`outbound` 表示下游调用链，`inbound` 表示影响范围。
    pub traversal_direction: String,
    /// 相对种子 symbol 的跳数。
    pub hop_distance: usize,
    /// 当前边置信度。
    pub confidence: f64,
    /// resolve 阶段留下的原因文本。
    pub reason: String,
}

/// `LlmCacheEntry` 是 prompt 级缓存记录。
#[derive(Debug, Clone)]
pub struct LlmCacheEntry {
    /// 归一化后的 prompt 输入哈希。
    pub input_hash: String,
    /// prompt 类型。
    pub prompt_type: String,
    /// prompt 版本。
    pub prompt_version: String,
    /// 结构化 JSON 响应文本。
    pub response: String,
    /// 实际使用的模型标识。
    pub model: Option<String>,
    /// 写入时间戳。
    pub created_at: String,
    /// TTL（秒）。
    pub ttl_seconds: i64,
}

/// 返回 DB 文件路径。
pub fn db_path(repo_root: &Path) -> PathBuf {
    cache_dir(repo_root).join(DB_FILENAME)
}

/// 打开（或创建）DB 并初始化当前 schema。
pub fn open_db(repo_root: &Path) -> io::Result<Connection> {
    ensure_cache_dir(repo_root)?;
    let path = db_path(repo_root);
    let conn = Connection::open_with_flags(
        &path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    )
    .map_err(|e| io::Error::other(format!("sqlite open: {e}")))?;

    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .map_err(|e| io::Error::other(format!("sqlite pragmas: {e}")))?;

    init_schema(&conn)?;
    Ok(conn)
}

/// 尝试以只读方式打开已有 DB。
pub fn open_db_readonly(repo_root: &Path) -> io::Result<Connection> {
    let path = db_path(repo_root);
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "wiki-cache.db not found",
        ));
    }

    Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|e| io::Error::other(format!("sqlite open readonly: {e}")))
}

/// DB 是否存在。
pub fn db_exists(repo_root: &Path) -> bool {
    db_path(repo_root).exists()
}

/// 表是否存在。
pub fn table_exists(conn: &Connection, table_name: &str) -> io::Result<bool> {
    let exists = conn
        .query_row(
            "SELECT EXISTS(
                SELECT 1
                FROM sqlite_master
                WHERE type IN ('table', 'view')
                  AND name = ?1
            )",
            params![table_name],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|e| io::Error::other(format!("table_exists({table_name}): {e}")))?;
    Ok(exists != 0)
}

/// 关键 runtime 状态表是否都存在。
pub fn runtime_tables_exist(conn: &Connection) -> io::Result<bool> {
    let required = [
        "wiki_pages",
        "wiki_page_sections",
        "source_states",
        "modules",
        "module_source_map",
        "page_source_map",
        "page_module_map",
        "wiki_relations",
        "runtime_meta",
        "scan_cache",
    ];

    for table_name in required {
        if !table_exists(conn, table_name)? {
            return Ok(false);
        }
    }

    Ok(true)
}

fn init_schema(conn: &Connection) -> io::Result<()> {
    init_legacy_tables(conn)?;
    init_runtime_tables(conn)?;
    Ok(())
}

fn init_legacy_tables(conn: &Connection) -> io::Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS kv_store (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS page_context_cache (
            page_id    TEXT PRIMARY KEY,
            input_hash TEXT NOT NULL,
            context    TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS page_generation_cache (
            page_id      TEXT PRIMARY KEY,
            input_hash   TEXT NOT NULL,
            content_hash TEXT NOT NULL,
            sections     TEXT NOT NULL
        );",
    )
    .map_err(|e| io::Error::other(format!("legacy schema init: {e}")))?;
    Ok(())
}

fn init_runtime_tables(conn: &Connection) -> io::Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS runtime_meta (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS scan_cache (
            cache_key TEXT PRIMARY KEY,
            value     TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS wiki_pages (
            page_id          TEXT PRIMARY KEY,
            sort_order       INTEGER NOT NULL,
            title            TEXT NOT NULL,
            path             TEXT NOT NULL,
            page_type        TEXT NOT NULL,
            parent_id        TEXT,
            ancestor_ids     TEXT NOT NULL,
            input_hash       TEXT NOT NULL,
            content_hash     TEXT NOT NULL,
            source_paths     TEXT NOT NULL,
            summary          TEXT NOT NULL,
            provenance       TEXT NOT NULL,
            section_anchors  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS wiki_page_sections (
            page_id                    TEXT NOT NULL REFERENCES wiki_pages(page_id) ON DELETE CASCADE,
            section_id                 TEXT NOT NULL,
            sort_order                 INTEGER NOT NULL,
            title                      TEXT NOT NULL,
            managed                    INTEGER NOT NULL,
            content_hash               TEXT NOT NULL,
            generated_content_hash     TEXT,
            anchor_after_section_id    TEXT,
            anchor_before_section_id   TEXT,
            source_ids                 TEXT NOT NULL,
            relation_ids               TEXT NOT NULL,
            PRIMARY KEY (page_id, section_id)
        );

        CREATE TABLE IF NOT EXISTS source_states (
            source_id    TEXT PRIMARY KEY,
            sort_order   INTEGER NOT NULL,
            path         TEXT NOT NULL,
            fingerprint  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS modules (
            module_id     TEXT PRIMARY KEY,
            sort_order    INTEGER NOT NULL,
            name          TEXT NOT NULL,
            kind          TEXT NOT NULL,
            root_paths    TEXT NOT NULL,
            parent_id     TEXT,
            child_ids     TEXT NOT NULL,
            entry_points  TEXT NOT NULL,
            tags          TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS module_source_map (
            module_id TEXT NOT NULL REFERENCES modules(module_id) ON DELETE CASCADE,
            source_id TEXT NOT NULL REFERENCES source_states(source_id) ON DELETE CASCADE,
            PRIMARY KEY (module_id, source_id)
        );

        CREATE TABLE IF NOT EXISTS page_source_map (
            page_id   TEXT NOT NULL REFERENCES wiki_pages(page_id) ON DELETE CASCADE,
            source_id TEXT NOT NULL REFERENCES source_states(source_id) ON DELETE CASCADE,
            PRIMARY KEY (page_id, source_id)
        );

        CREATE TABLE IF NOT EXISTS page_module_map (
            page_id    TEXT NOT NULL REFERENCES wiki_pages(page_id) ON DELETE CASCADE,
            module_id  TEXT NOT NULL REFERENCES modules(module_id) ON DELETE CASCADE,
            PRIMARY KEY (page_id, module_id)
        );

        CREATE TABLE IF NOT EXISTS wiki_relations (
            relation_id    TEXT PRIMARY KEY,
            sort_order     INTEGER NOT NULL,
            source_id      TEXT NOT NULL,
            target_id      TEXT NOT NULL,
            relation_type  TEXT NOT NULL,
            evidence       TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS llm_cache (
            input_hash   TEXT PRIMARY KEY,
            prompt_type  TEXT NOT NULL,
            prompt_version TEXT NOT NULL,
            response     TEXT NOT NULL,
            model        TEXT,
            created_at   TEXT NOT NULL,
            ttl_seconds  INTEGER DEFAULT 604800
        );

        CREATE TABLE IF NOT EXISTS symbols (
            id          TEXT PRIMARY KEY,
            name        TEXT NOT NULL,
            label       TEXT NOT NULL,
            file_path   TEXT NOT NULL,
            start_line  INTEGER,
            end_line    INTEGER,
            is_exported INTEGER DEFAULT 0,
            language    TEXT
        );

        CREATE TABLE IF NOT EXISTS edges (
            id          TEXT PRIMARY KEY,
            source_id   TEXT NOT NULL REFERENCES symbols(id),
            target_id   TEXT NOT NULL REFERENCES symbols(id),
            edge_type   TEXT NOT NULL,
            confidence  REAL DEFAULT 1.0,
            reason      TEXT
        );

        CREATE TABLE IF NOT EXISTS communities (
            id           TEXT PRIMARY KEY,
            label        TEXT,
            cohesion     REAL,
            symbol_count INTEGER
        );

        CREATE TABLE IF NOT EXISTS community_members (
            community_id TEXT NOT NULL REFERENCES communities(id),
            symbol_id    TEXT NOT NULL REFERENCES symbols(id),
            PRIMARY KEY (community_id, symbol_id)
        );

        CREATE TABLE IF NOT EXISTS processes (
            id             TEXT PRIMARY KEY,
            label          TEXT,
            process_type   TEXT,
            step_count     INTEGER,
            entry_point_id TEXT,
            terminal_id    TEXT
        );

        CREATE TABLE IF NOT EXISTS process_steps (
            process_id  TEXT NOT NULL REFERENCES processes(id),
            symbol_id   TEXT NOT NULL REFERENCES symbols(id),
            step_order  INTEGER NOT NULL,
            PRIMARY KEY (process_id, step_order)
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS wiki_pages_fts
        USING fts5(
            page_id UNINDEXED,
            title,
            path,
            summary
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS symbols_fts
        USING fts5(
            symbol_id UNINDEXED,
            name,
            file_path,
            content
        );

        CREATE INDEX IF NOT EXISTS idx_wiki_pages_path ON wiki_pages(path);
        CREATE INDEX IF NOT EXISTS idx_sections_page ON wiki_page_sections(page_id, sort_order);
        CREATE INDEX IF NOT EXISTS idx_source_states_path ON source_states(path);
        CREATE INDEX IF NOT EXISTS idx_modules_parent ON modules(parent_id);
        CREATE INDEX IF NOT EXISTS idx_relations_source ON wiki_relations(source_id);
        CREATE INDEX IF NOT EXISTS idx_relations_target ON wiki_relations(target_id);
        CREATE INDEX IF NOT EXISTS idx_symbols_file ON symbols(file_path);
        CREATE INDEX IF NOT EXISTS idx_symbols_name ON symbols(name);
        CREATE INDEX IF NOT EXISTS idx_symbols_label ON symbols(label);
        CREATE INDEX IF NOT EXISTS idx_edges_source ON edges(source_id);
        CREATE INDEX IF NOT EXISTS idx_edges_target ON edges(target_id);
        CREATE INDEX IF NOT EXISTS idx_edges_type ON edges(edge_type);
        CREATE INDEX IF NOT EXISTS idx_llm_cache_lookup
            ON llm_cache(prompt_type, prompt_version, model);",
    )
    .map_err(|e| io::Error::other(format!("runtime schema init: {e}")))?;
    Ok(())
}

fn json_string(value: &[String]) -> io::Result<String> {
    serde_json::to_string(value).map_err(|e| io::Error::other(e.to_string()))
}

fn parse_string_list(json: &str) -> io::Result<Vec<String>> {
    serde_json::from_str(json).map_err(|e| io::Error::other(e.to_string()))
}

fn encode_dirty_state(dirty_state: &DirtyState) -> io::Result<String> {
    serde_json::to_string(dirty_state).map_err(|e| io::Error::other(e.to_string()))
}

fn encode_build_state(build_state: &BuildState) -> io::Result<String> {
    serde_json::to_string(build_state).map_err(|e| io::Error::other(e.to_string()))
}

fn relation_id(relation: &WikiRelation, index: usize) -> String {
    format!(
        "{}:{}:{}:{}",
        index, relation.source_id, relation.target_id, relation.relation_type
    )
}

fn replace_state_rows_tx(tx: &Transaction<'_>, state: &WikiState) -> io::Result<()> {
    tx.execute_batch(
        "DELETE FROM wiki_page_sections;
         DELETE FROM page_source_map;
         DELETE FROM page_module_map;
         DELETE FROM module_source_map;
         DELETE FROM wiki_relations;
         DELETE FROM wiki_pages;
         DELETE FROM source_states;
         DELETE FROM modules;
         DELETE FROM runtime_meta;
         DELETE FROM wiki_pages_fts;",
    )
    .map_err(|e| io::Error::other(format!("clear state tables: {e}")))?;

    for (index, source) in state.sources.iter().enumerate() {
        tx.execute(
            "INSERT INTO source_states (source_id, sort_order, path, fingerprint)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                source.source_id,
                index as i64,
                source.path,
                source.fingerprint
            ],
        )
        .map_err(|e| io::Error::other(format!("insert source_state {}: {e}", source.source_id)))?;
    }

    for (index, module) in state.modules.iter().enumerate() {
        tx.execute(
            "INSERT INTO modules
             (module_id, sort_order, name, kind, root_paths, parent_id, child_ids, entry_points, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                module.id,
                index as i64,
                module.name,
                module.kind,
                json_string(&module.root_paths)?,
                module.parent_id,
                json_string(&module.child_ids)?,
                json_string(&module.entry_points)?,
                json_string(&module.tags)?,
            ],
        )
        .map_err(|e| io::Error::other(format!("insert module {}: {e}", module.id)))?;

        let mut unique_source_ids = BTreeSet::new();
        for source_id in &module.source_ids {
            if !unique_source_ids.insert(source_id.clone()) {
                continue;
            }

            tx.execute(
                "INSERT INTO module_source_map (module_id, source_id) VALUES (?1, ?2)",
                params![module.id, source_id],
            )
            .map_err(|e| {
                io::Error::other(format!("insert module_source_map {}: {e}", module.id))
            })?;
        }
    }

    for (index, page) in state.pages.iter().enumerate() {
        tx.execute(
            "INSERT INTO wiki_pages
             (page_id, sort_order, title, path, page_type, parent_id, ancestor_ids,
              input_hash, content_hash, source_paths, summary, provenance, section_anchors)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                page.page_id,
                index as i64,
                page.title,
                page.path,
                page.page_type,
                page.parent_id,
                json_string(&page.ancestor_ids)?,
                page.input_hash,
                page.content_hash,
                json_string(&page.source_paths)?,
                page.summary,
                json_string(&page.provenance)?,
                json_string(&page.managed_section_anchors())?,
            ],
        )
        .map_err(|e| io::Error::other(format!("insert wiki_page {}: {e}", page.page_id)))?;

        for source_id in &page.source_ids {
            tx.execute(
                "INSERT INTO page_source_map (page_id, source_id) VALUES (?1, ?2)",
                params![page.page_id, source_id],
            )
            .map_err(|e| {
                io::Error::other(format!("insert page_source_map {}: {e}", page.page_id))
            })?;
        }

        for module_id in &page.module_ids {
            tx.execute(
                "INSERT INTO page_module_map (page_id, module_id) VALUES (?1, ?2)",
                params![page.page_id, module_id],
            )
            .map_err(|e| {
                io::Error::other(format!("insert page_module_map {}: {e}", page.page_id))
            })?;
        }

        for (section_index, section) in page.sections.iter().enumerate() {
            tx.execute(
                "INSERT INTO wiki_page_sections
                 (page_id, section_id, sort_order, title, managed, content_hash,
                  generated_content_hash, anchor_after_section_id, anchor_before_section_id,
                  source_ids, relation_ids)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    page.page_id,
                    section.section_id,
                    section_index as i64,
                    section.title,
                    if section.managed { 1 } else { 0 },
                    section.content_hash,
                    section.generated_content_hash,
                    section.anchor_after_section_id,
                    section.anchor_before_section_id,
                    json_string(&section.source_ids)?,
                    json_string(&section.relation_ids)?,
                ],
            )
            .map_err(|e| {
                io::Error::other(format!(
                    "insert wiki_page_sections {}:{}: {e}",
                    page.page_id, section.section_id
                ))
            })?;
        }
    }

    for (index, relation) in state.relations.iter().enumerate() {
        tx.execute(
            "INSERT INTO wiki_relations
             (relation_id, sort_order, source_id, target_id, relation_type, evidence)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                relation_id(relation, index),
                index as i64,
                relation.source_id,
                relation.target_id,
                relation.relation_type,
                json_string(&relation.evidence)?,
            ],
        )
        .map_err(|e| io::Error::other(format!("insert relation {}: {e}", index)))?;
    }

    tx.execute(
        "INSERT INTO runtime_meta (key, value) VALUES ('dirty_state', ?1)",
        params![encode_dirty_state(&state.dirty_state)?],
    )
    .map_err(|e| io::Error::other(format!("insert dirty_state: {e}")))?;
    tx.execute(
        "INSERT INTO runtime_meta (key, value) VALUES ('build_state', ?1)",
        params![encode_build_state(&state.build_state)?],
    )
    .map_err(|e| io::Error::other(format!("insert build_state: {e}")))?;

    for page in &state.pages {
        tx.execute(
            "INSERT INTO wiki_pages_fts (page_id, title, path, summary) VALUES (?1, ?2, ?3, ?4)",
            params![page.page_id, page.title, page.path, page.summary],
        )
        .map_err(|e| io::Error::other(format!("insert wiki_pages_fts {}: {e}", page.page_id)))?;
    }

    Ok(())
}

fn clear_symbol_rows_tx(tx: &Transaction<'_>) -> io::Result<()> {
    tx.execute_batch(
        "DELETE FROM symbols_fts;
         DELETE FROM symbols;",
    )
    .map_err(|e| io::Error::other(format!("clear symbol tables: {e}")))?;
    Ok(())
}

fn insert_symbol_rows_tx(tx: &Transaction<'_>, symbols: &[SymbolNode]) -> io::Result<()> {
    for symbol in symbols {
        tx.execute(
            "INSERT INTO symbols
             (id, name, label, file_path, start_line, end_line, is_exported, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                symbol.symbol_id,
                symbol.name,
                symbol.label,
                symbol.file_path,
                symbol.start_line as i64,
                symbol.end_line as i64,
                if symbol.is_exported { 1 } else { 0 },
                symbol.language,
            ],
        )
        .map_err(|e| io::Error::other(format!("insert symbol {}: {e}", symbol.symbol_id)))?;

        tx.execute(
            "INSERT INTO symbols_fts (symbol_id, name, file_path, content)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                symbol.symbol_id,
                symbol.name,
                symbol.file_path,
                symbol.search_text(),
            ],
        )
        .map_err(|e| io::Error::other(format!("insert symbols_fts {}: {e}", symbol.symbol_id)))?;
    }

    Ok(())
}

fn clear_symbol_graph_rows_tx(tx: &Transaction<'_>) -> io::Result<()> {
    tx.execute_batch(
        "DELETE FROM process_steps;
         DELETE FROM processes;
         DELETE FROM community_members;
         DELETE FROM communities;
         DELETE FROM edges;",
    )
    .map_err(|e| io::Error::other(format!("clear symbol graph tables: {e}")))?;
    Ok(())
}

fn clear_graph_analysis_rows_tx(tx: &Transaction<'_>) -> io::Result<()> {
    tx.execute_batch(
        "DELETE FROM process_steps;
         DELETE FROM processes;
         DELETE FROM community_members;
         DELETE FROM communities;",
    )
    .map_err(|e| io::Error::other(format!("clear graph analysis tables: {e}")))?;
    Ok(())
}

fn insert_edge_rows_tx(
    tx: &Transaction<'_>,
    resolved_graph: &ResolvedGraphSnapshot,
) -> io::Result<()> {
    for edge in &resolved_graph.edges {
        tx.execute(
            "INSERT INTO edges
             (id, source_id, target_id, edge_type, confidence, reason)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                edge.edge_id,
                edge.source_id,
                edge.target_id,
                edge.edge_type,
                edge.confidence,
                edge.reason,
            ],
        )
        .map_err(|e| io::Error::other(format!("insert edge {}: {e}", edge.edge_id)))?;
    }

    Ok(())
}

fn insert_graph_analysis_rows_tx(
    tx: &Transaction<'_>,
    analysis: &GraphAnalysisSnapshot,
) -> io::Result<()> {
    for community in &analysis.communities {
        tx.execute(
            "INSERT INTO communities (id, label, cohesion, symbol_count)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                community.community_id,
                community.label,
                community.cohesion,
                community.symbol_count as i64,
            ],
        )
        .map_err(|e| {
            io::Error::other(format!("insert community {}: {e}", community.community_id))
        })?;
    }

    for member in &analysis.community_members {
        tx.execute(
            "INSERT INTO community_members (community_id, symbol_id)
             VALUES (?1, ?2)",
            params![member.community_id, member.symbol_id],
        )
        .map_err(|e| {
            io::Error::other(format!(
                "insert community member {}:{}: {e}",
                member.community_id, member.symbol_id
            ))
        })?;
    }

    for process in &analysis.processes {
        tx.execute(
            "INSERT INTO processes
             (id, label, process_type, step_count, entry_point_id, terminal_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                process.process_id,
                process.label,
                process.process_type,
                process.step_count as i64,
                process.entry_point_id,
                process.terminal_id,
            ],
        )
        .map_err(|e| io::Error::other(format!("insert process {}: {e}", process.process_id)))?;
    }

    for step in &analysis.process_steps {
        tx.execute(
            "INSERT INTO process_steps (process_id, symbol_id, step_order)
             VALUES (?1, ?2, ?3)",
            params![step.process_id, step.symbol_id, step.step_order as i64],
        )
        .map_err(|e| {
            io::Error::other(format!(
                "insert process step {}:{}: {e}",
                step.process_id, step.step_order
            ))
        })?;
    }

    Ok(())
}

fn delete_symbol_rows_for_files_tx(tx: &Transaction<'_>, file_paths: &[String]) -> io::Result<()> {
    if file_paths.is_empty() {
        return Ok(());
    }

    let symbol_ids = select_symbol_ids_for_files_tx(tx, file_paths)?;
    if !symbol_ids.is_empty() {
        let placeholders = sql_placeholders(symbol_ids.len());
        let sql = format!("DELETE FROM symbols_fts WHERE symbol_id IN ({placeholders})");
        let params = symbol_ids.iter().map(|value| value as &dyn ToSql);
        tx.execute(&sql, params_from_iter(params))
            .map_err(|e| io::Error::other(format!("delete symbols_fts by file: {e}")))?;
    }

    let placeholders = sql_placeholders(file_paths.len());
    let sql = format!("DELETE FROM symbols WHERE file_path IN ({placeholders})");
    let params = file_paths.iter().map(|value| value as &dyn ToSql);
    tx.execute(&sql, params_from_iter(params))
        .map_err(|e| io::Error::other(format!("delete symbols by file: {e}")))?;
    Ok(())
}

fn delete_edge_rows_for_symbol_ids_tx(
    tx: &Transaction<'_>,
    symbol_ids: &[String],
) -> io::Result<()> {
    if symbol_ids.is_empty() {
        return Ok(());
    }

    let placeholders = sql_placeholders(symbol_ids.len());
    let params = symbol_ids.iter().map(|value| value as &dyn ToSql);
    tx.execute(
        &format!("DELETE FROM edges WHERE source_id IN ({placeholders})"),
        params_from_iter(params),
    )
    .map_err(|e| io::Error::other(format!("delete edges by source symbol: {e}")))?;

    let params = symbol_ids.iter().map(|value| value as &dyn ToSql);
    tx.execute(
        &format!("DELETE FROM edges WHERE target_id IN ({placeholders})"),
        params_from_iter(params),
    )
    .map_err(|e| io::Error::other(format!("delete edges by target symbol: {e}")))?;
    Ok(())
}

fn select_symbol_ids_for_files_tx(
    tx: &Transaction<'_>,
    file_paths: &[String],
) -> io::Result<Vec<String>> {
    if file_paths.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = sql_placeholders(file_paths.len());
    let sql = format!("SELECT id FROM symbols WHERE file_path IN ({placeholders})");
    let params = file_paths.iter().map(|value| value as &dyn ToSql);
    let mut stmt = tx
        .prepare(&sql)
        .map_err(|e| io::Error::other(format!("prepare select symbol ids: {e}")))?;
    let rows = stmt
        .query_map(params_from_iter(params), |row| row.get::<_, String>(0))
        .map_err(|e| io::Error::other(format!("query symbol ids by file: {e}")))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect symbol ids by file: {e}")))
}

fn sql_placeholders(count: usize) -> String {
    (1..=count)
        .map(|index| format!("?{index}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// 用事务替换当前状态表内容。
pub fn replace_state_rows(conn: &mut Connection, state: &WikiState) -> io::Result<()> {
    let tx = conn
        .transaction()
        .map_err(|e| io::Error::other(format!("begin replace_state_rows tx: {e}")))?;
    replace_state_rows_tx(&tx, state)?;
    tx.commit()
        .map_err(|e| io::Error::other(format!("commit replace_state_rows tx: {e}")))
}

/// 在同一事务里全量替换 state 与 symbols。
/// 供 `init` / `rebuild` 使用，保证页面状态和 symbol snapshot 同轮提交。
pub fn replace_state_and_symbols(
    conn: &mut Connection,
    state: &WikiState,
    symbols: &[SymbolNode],
) -> io::Result<()> {
    let tx = conn
        .transaction()
        .map_err(|e| io::Error::other(format!("begin replace_state_and_symbols tx: {e}")))?;
    replace_state_rows_tx(&tx, state)?;
    clear_symbol_rows_tx(&tx)?;
    insert_symbol_rows_tx(&tx, symbols)?;
    tx.commit()
        .map_err(|e| io::Error::other(format!("commit replace_state_and_symbols tx: {e}")))
}

/// 在同一事务里全量替换 state、symbols 和 symbol graph。
/// 供 `init` / `rebuild` 接入 graph 主链时使用。
pub fn replace_state_and_symbol_graph(
    conn: &mut Connection,
    state: &WikiState,
    symbols: &[SymbolNode],
    resolved_graph: &ResolvedGraphSnapshot,
    analysis: &GraphAnalysisSnapshot,
) -> io::Result<()> {
    let tx = conn
        .transaction()
        .map_err(|e| io::Error::other(format!("begin replace_state_and_symbol_graph tx: {e}")))?;
    replace_state_rows_tx(&tx, state)?;
    clear_symbol_graph_rows_tx(&tx)?;
    clear_symbol_rows_tx(&tx)?;
    insert_symbol_rows_tx(&tx, symbols)?;
    insert_edge_rows_tx(&tx, resolved_graph)?;
    insert_graph_analysis_rows_tx(&tx, analysis)?;
    tx.commit()
        .map_err(|e| io::Error::other(format!("commit replace_state_and_symbol_graph tx: {e}")))
}

/// 在同一事务里全量替换 state，并只对指定文件刷新 symbols。
/// 供 `update` 使用，避免无关文件的旧 symbol rows 被重写。
pub fn replace_state_and_symbols_for_files(
    conn: &mut Connection,
    state: &WikiState,
    file_paths: &[String],
    symbols: &[SymbolNode],
) -> io::Result<()> {
    let tx = conn.transaction().map_err(|e| {
        io::Error::other(format!("begin replace_state_and_symbols_for_files tx: {e}"))
    })?;
    replace_state_rows_tx(&tx, state)?;
    delete_symbol_rows_for_files_tx(&tx, file_paths)?;
    insert_symbol_rows_tx(&tx, symbols)?;
    tx.commit().map_err(|e| {
        io::Error::other(format!(
            "commit replace_state_and_symbols_for_files tx: {e}"
        ))
    })
}

/// 在同一事务里按文件刷新 symbols / edges，并整体替换 graph-derived 结果。
/// 供 `update` 的 symbol graph 增量阶段使用。
pub fn replace_state_and_symbol_graph_for_files(
    conn: &mut Connection,
    state: &WikiState,
    file_paths: &[String],
    symbols: &[SymbolNode],
    resolved_graph: &ResolvedGraphSnapshot,
    analysis: &GraphAnalysisSnapshot,
) -> io::Result<()> {
    let tx = conn.transaction().map_err(|e| {
        io::Error::other(format!(
            "begin replace_state_and_symbol_graph_for_files tx: {e}"
        ))
    })?;
    replace_state_rows_tx(&tx, state)?;
    let stale_symbol_ids = select_symbol_ids_for_files_tx(&tx, file_paths)?;
    clear_graph_analysis_rows_tx(&tx)?;
    delete_edge_rows_for_symbol_ids_tx(&tx, &stale_symbol_ids)?;
    delete_symbol_rows_for_files_tx(&tx, file_paths)?;
    insert_symbol_rows_tx(&tx, symbols)?;
    insert_edge_rows_tx(&tx, resolved_graph)?;
    insert_graph_analysis_rows_tx(&tx, analysis)?;
    tx.commit().map_err(|e| {
        io::Error::other(format!(
            "commit replace_state_and_symbol_graph_for_files tx: {e}"
        ))
    })
}

/// 从关系型状态表恢复 WikiState。
pub fn load_state_rows(conn: &Connection) -> io::Result<WikiState> {
    if !runtime_tables_exist(conn)? {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "runtime tables missing from sqlite database",
        ));
    }

    let mut pages = conn
        .prepare(
            "SELECT page_id, title, path, page_type, parent_id, ancestor_ids,
                    input_hash, content_hash, source_paths, summary, provenance, section_anchors
             FROM wiki_pages
             ORDER BY sort_order",
        )
        .map_err(|e| io::Error::other(format!("prepare wiki_pages: {e}")))?
        .query_map([], |row| {
            let ancestor_ids = parse_string_list(&row.get::<_, String>(5)?)
                .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
            let source_paths = parse_string_list(&row.get::<_, String>(8)?)
                .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
            let provenance = parse_string_list(&row.get::<_, String>(10)?)
                .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
            let section_anchors = parse_string_list(&row.get::<_, String>(11)?)
                .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
            Ok(WikiPageState {
                page_id: row.get(0)?,
                title: row.get(1)?,
                path: row.get(2)?,
                page_type: row.get(3)?,
                parent_id: row.get(4)?,
                ancestor_ids,
                input_hash: row.get(6)?,
                content_hash: row.get(7)?,
                source_ids: Vec::new(),
                source_paths,
                module_ids: Vec::new(),
                summary: row.get(9)?,
                provenance,
                section_anchors,
                sections: Vec::new(),
            })
        })
        .map_err(|e| io::Error::other(format!("query wiki_pages: {e}")))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect wiki_pages: {e}")))?;

    if pages.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "no wiki_pages rows found in sqlite database",
        ));
    }

    let page_index = pages
        .iter()
        .enumerate()
        .map(|(index, page)| (page.page_id.clone(), index))
        .collect::<BTreeMap<_, _>>();

    let mut section_stmt = conn
        .prepare(
            "SELECT page_id, section_id, title, managed, content_hash,
                    generated_content_hash, anchor_after_section_id, anchor_before_section_id,
                    source_ids, relation_ids
             FROM wiki_page_sections
             ORDER BY page_id, sort_order",
        )
        .map_err(|e| io::Error::other(format!("prepare wiki_page_sections: {e}")))?;
    let section_rows = section_stmt
        .query_map([], |row| {
            let source_ids = parse_string_list(&row.get::<_, String>(8)?)
                .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
            let relation_ids = parse_string_list(&row.get::<_, String>(9)?)
                .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
            Ok((
                row.get::<_, String>(0)?,
                WikiSectionState {
                    section_id: row.get(1)?,
                    title: row.get(2)?,
                    managed: row.get::<_, i64>(3)? != 0,
                    content_hash: row.get(4)?,
                    generated_content_hash: row.get(5)?,
                    anchor_after_section_id: row.get(6)?,
                    anchor_before_section_id: row.get(7)?,
                    source_ids,
                    relation_ids,
                },
            ))
        })
        .map_err(|e| io::Error::other(format!("query wiki_page_sections: {e}")))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect wiki_page_sections: {e}")))?;

    for (page_id, section) in section_rows {
        if let Some(index) = page_index.get(&page_id) {
            pages[*index].sections.push(section);
        }
    }

    let mut page_to_source_ids = BTreeMap::<String, Vec<String>>::new();
    let mut source_to_page_ids = BTreeMap::<String, Vec<String>>::new();
    {
        let mut stmt = conn
            .prepare("SELECT page_id, source_id FROM page_source_map ORDER BY page_id, source_id")
            .map_err(|e| io::Error::other(format!("prepare page_source_map: {e}")))?;
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|e| io::Error::other(format!("query page_source_map: {e}")))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| io::Error::other(format!("collect page_source_map: {e}")))?;
        for (page_id, source_id) in rows {
            page_to_source_ids
                .entry(page_id.clone())
                .or_default()
                .push(source_id.clone());
            source_to_page_ids
                .entry(source_id)
                .or_default()
                .push(page_id);
        }
    }

    let mut page_to_module_ids = BTreeMap::<String, Vec<String>>::new();
    {
        let mut stmt = conn
            .prepare("SELECT page_id, module_id FROM page_module_map ORDER BY page_id, module_id")
            .map_err(|e| io::Error::other(format!("prepare page_module_map: {e}")))?;
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|e| io::Error::other(format!("query page_module_map: {e}")))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| io::Error::other(format!("collect page_module_map: {e}")))?;
        for (page_id, module_id) in rows {
            page_to_module_ids
                .entry(page_id)
                .or_default()
                .push(module_id);
        }
    }

    for page in &mut pages {
        page.source_ids = page_to_source_ids.remove(&page.page_id).unwrap_or_default();
        page.module_ids = page_to_module_ids.remove(&page.page_id).unwrap_or_default();
    }

    let mut source_to_module_ids = BTreeMap::<String, Vec<String>>::new();
    let mut module_to_source_ids = BTreeMap::<String, Vec<String>>::new();
    {
        let mut stmt = conn
            .prepare(
                "SELECT module_id, source_id FROM module_source_map ORDER BY module_id, source_id",
            )
            .map_err(|e| io::Error::other(format!("prepare module_source_map: {e}")))?;
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|e| io::Error::other(format!("query module_source_map: {e}")))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| io::Error::other(format!("collect module_source_map: {e}")))?;
        for (module_id, source_id) in rows {
            module_to_source_ids
                .entry(module_id.clone())
                .or_default()
                .push(source_id.clone());
            source_to_module_ids
                .entry(source_id)
                .or_default()
                .push(module_id);
        }
    }

    let sources = conn
        .prepare(
            "SELECT source_id, path, fingerprint
             FROM source_states
             ORDER BY sort_order",
        )
        .map_err(|e| io::Error::other(format!("prepare source_states: {e}")))?
        .query_map([], |row| {
            let source_id: String = row.get(0)?;
            Ok(SourceState {
                source_id: source_id.clone(),
                path: row.get(1)?,
                fingerprint: row.get(2)?,
                page_ids: source_to_page_ids.remove(&source_id).unwrap_or_default(),
                module_ids: source_to_module_ids.remove(&source_id).unwrap_or_default(),
            })
        })
        .map_err(|e| io::Error::other(format!("query source_states: {e}")))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect source_states: {e}")))?;

    let modules = conn
        .prepare(
            "SELECT module_id, name, kind, root_paths, parent_id, child_ids, entry_points, tags
             FROM modules
             ORDER BY sort_order",
        )
        .map_err(|e| io::Error::other(format!("prepare modules: {e}")))?
        .query_map([], |row| {
            let module_id: String = row.get(0)?;
            let root_paths = parse_string_list(&row.get::<_, String>(3)?)
                .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
            let child_ids = parse_string_list(&row.get::<_, String>(5)?)
                .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
            let entry_points = parse_string_list(&row.get::<_, String>(6)?)
                .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
            let tags = parse_string_list(&row.get::<_, String>(7)?)
                .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
            Ok(ModuleNode {
                id: module_id.clone(),
                name: row.get(1)?,
                kind: row.get(2)?,
                root_paths,
                source_ids: module_to_source_ids.remove(&module_id).unwrap_or_default(),
                parent_id: row.get(4)?,
                child_ids,
                entry_points,
                tags,
            })
        })
        .map_err(|e| io::Error::other(format!("query modules: {e}")))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect modules: {e}")))?;

    let relations = conn
        .prepare(
            "SELECT source_id, target_id, relation_type, evidence
             FROM wiki_relations
             ORDER BY sort_order",
        )
        .map_err(|e| io::Error::other(format!("prepare wiki_relations: {e}")))?
        .query_map([], |row| {
            let evidence = parse_string_list(&row.get::<_, String>(3)?)
                .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
            Ok(WikiRelation {
                source_id: row.get(0)?,
                target_id: row.get(1)?,
                relation_type: row.get(2)?,
                evidence,
            })
        })
        .map_err(|e| io::Error::other(format!("query wiki_relations: {e}")))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect wiki_relations: {e}")))?;

    let dirty_state = runtime_meta_get(conn, "dirty_state")?
        .map(|json| serde_json::from_str(&json).map_err(|e| io::Error::other(e.to_string())))
        .transpose()?
        .unwrap_or_else(DirtyState::fresh);
    let build_state = runtime_meta_get(conn, "build_state")?
        .map(|json| serde_json::from_str(&json).map_err(|e| io::Error::other(e.to_string())))
        .transpose()?
        .unwrap_or(BuildState {
            generated_at: String::new(),
            page_count: pages.len(),
            module_count: modules.len(),
        });

    Ok(WikiState {
        pages,
        sources,
        modules,
        relations,
        dirty_state,
        build_state,
    })
}

/// 页面是否已经持久化了 section 行。
pub fn page_sections_exist(conn: &Connection, page_id: &str) -> io::Result<bool> {
    let exists = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM wiki_page_sections WHERE page_id = ?1 LIMIT 1)",
            params![page_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|e| io::Error::other(format!("page_sections_exist({page_id}): {e}")))?;
    Ok(exists != 0)
}

/// 写入 runtime meta。
pub fn runtime_meta_set(conn: &Connection, key: &str, value: &str) -> io::Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO runtime_meta (key, value) VALUES (?1, ?2)",
        params![key, value],
    )
    .map_err(|e| io::Error::other(format!("runtime_meta_set({key}): {e}")))?;
    Ok(())
}

/// 读取 runtime meta。
pub fn runtime_meta_get(conn: &Connection, key: &str) -> io::Result<Option<String>> {
    conn.query_row(
        "SELECT value FROM runtime_meta WHERE key = ?1",
        params![key],
        |row| row.get::<_, String>(0),
    )
    .optional()
    .map_err(|e| io::Error::other(format!("runtime_meta_get({key}): {e}")))
}

/// 写入扫描缓存。
pub fn scan_cache_set(conn: &Connection, cache_key: &str, value: &str) -> io::Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO scan_cache (cache_key, value) VALUES (?1, ?2)",
        params![cache_key, value],
    )
    .map_err(|e| io::Error::other(format!("scan_cache_set({cache_key}): {e}")))?;
    Ok(())
}

/// 读取扫描缓存。
pub fn scan_cache_get(conn: &Connection, cache_key: &str) -> io::Result<Option<String>> {
    conn.query_row(
        "SELECT value FROM scan_cache WHERE cache_key = ?1",
        params![cache_key],
        |row| row.get::<_, String>(0),
    )
    .optional()
    .map_err(|e| io::Error::other(format!("scan_cache_get({cache_key}): {e}")))
}

/// 扫描缓存是否存在。
pub fn scan_cache_exists(conn: &Connection, cache_key: &str) -> io::Result<bool> {
    let exists = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM scan_cache WHERE cache_key = ?1)",
            params![cache_key],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|e| io::Error::other(format!("scan_cache_exists({cache_key}): {e}")))?;
    Ok(exists != 0)
}

/// 用 `wiki_pages_fts` 执行页面级 FTS/BM25 搜索。
pub fn search_pages_fts(repo_root: &Path, term: &str, limit: usize) -> io::Result<Vec<FtsPageHit>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };

    search_pages_fts_in_conn(&conn, term, limit)
}

fn search_pages_fts_in_conn(
    conn: &Connection,
    term: &str,
    limit: usize,
) -> io::Result<Vec<FtsPageHit>> {
    if !table_exists(conn, "wiki_pages_fts")? {
        return Ok(Vec::new());
    }

    let Some(query) = build_fts_query(term) else {
        return Ok(Vec::new());
    };

    let mut stmt = match conn.prepare(
        "SELECT page_id, title, path, bm25(wiki_pages_fts) AS score
         FROM wiki_pages_fts
         WHERE wiki_pages_fts MATCH ?1
         ORDER BY score
         LIMIT ?2",
    ) {
        Ok(stmt) => stmt,
        Err(_) => return Ok(Vec::new()),
    };

    let rows = match stmt.query_map(params![query, limit as i64], |row| {
        Ok(FtsPageHit {
            page_id: row.get(0)?,
            title: row.get(1)?,
            path: row.get(2)?,
            score: row.get(3)?,
        })
    }) {
        Ok(rows) => rows,
        Err(_) => return Ok(Vec::new()),
    };

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect wiki_pages_fts hits: {e}")))
}

/// 列出当前 DB 内全部符号。
/// 主要供测试、调试和项目集分析脚本验证 symbol snapshot 使用。
pub fn list_symbols(repo_root: &Path) -> io::Result<Vec<SymbolNode>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };

    list_symbols_in_conn(&conn)
}

fn list_symbols_in_conn(conn: &Connection) -> io::Result<Vec<SymbolNode>> {
    if !table_exists(conn, "symbols")? {
        return Ok(Vec::new());
    }

    let mut stmt = conn
        .prepare(
            "SELECT id, name, label, file_path, start_line, end_line, is_exported, language
             FROM symbols
             ORDER BY file_path, start_line, id",
        )
        .map_err(|e| io::Error::other(format!("prepare list symbols: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(SymbolNode {
                symbol_id: row.get(0)?,
                name: row.get(1)?,
                label: row.get(2)?,
                file_path: row.get(3)?,
                start_line: row.get::<_, i64>(4)? as usize,
                end_line: row.get::<_, i64>(5)? as usize,
                is_exported: row.get::<_, i64>(6)? != 0,
                language: row.get(7)?,
            })
        })
        .map_err(|e| io::Error::other(format!("query list symbols: {e}")))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect list symbols: {e}")))
}

/// 按文件路径集合读取 symbol rows，供 update 局部工作集组装使用。
pub fn list_symbols_for_files(
    repo_root: &Path,
    file_paths: &[String],
) -> io::Result<Vec<SymbolNode>> {
    if file_paths.is_empty() || !db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };

    list_symbols_for_files_in_conn(&conn, file_paths)
}

fn list_symbols_for_files_in_conn(
    conn: &Connection,
    file_paths: &[String],
) -> io::Result<Vec<SymbolNode>> {
    if file_paths.is_empty() || !table_exists(conn, "symbols")? {
        return Ok(Vec::new());
    }

    let placeholders = sql_placeholders(file_paths.len());
    let sql = format!(
        "SELECT id, name, label, file_path, start_line, end_line, is_exported, language
         FROM symbols
         WHERE file_path IN ({placeholders})
         ORDER BY file_path, start_line, id"
    );
    let params = file_paths.iter().map(|value| value as &dyn ToSql);
    list_symbols_with_params(conn, &sql, params)
}

/// 返回当前 symbols 表里覆盖的去重文件数。
pub fn count_symbol_files(repo_root: &Path) -> io::Result<usize> {
    if !db_exists(repo_root) {
        return Ok(0);
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(0),
    };

    count_symbol_files_in_conn(&conn)
}

fn count_symbol_files_in_conn(conn: &Connection) -> io::Result<usize> {
    if !table_exists(conn, "symbols")? {
        return Ok(0);
    }

    conn.query_row("SELECT COUNT(DISTINCT file_path) FROM symbols", [], |row| {
        row.get::<_, i64>(0)
    })
    .map(|count| count as usize)
    .map_err(|e| io::Error::other(format!("count distinct symbol files: {e}")))
}

/// 列出当前 DB 内全部 symbol edges。
pub fn list_edges(repo_root: &Path) -> io::Result<Vec<ResolvedSymbolEdge>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };

    list_edges_in_conn(&conn)
}

fn list_edges_in_conn(conn: &Connection) -> io::Result<Vec<ResolvedSymbolEdge>> {
    if !table_exists(conn, "edges")? {
        return Ok(Vec::new());
    }

    let mut stmt = conn
        .prepare(
            "SELECT id, source_id, target_id, edge_type, confidence, COALESCE(reason, '')
             FROM edges
             ORDER BY source_id, target_id, id",
        )
        .map_err(|e| io::Error::other(format!("prepare list edges: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(ResolvedSymbolEdge {
                edge_id: row.get(0)?,
                source_id: row.get(1)?,
                target_id: row.get(2)?,
                edge_type: row.get(3)?,
                confidence: row.get(4)?,
                reason: row.get(5)?,
            })
        })
        .map_err(|e| io::Error::other(format!("query list edges: {e}")))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect list edges: {e}")))
}

/// 按文件路径集合读取与这些文件相邻的 symbol edges。
pub fn list_edges_for_files(
    repo_root: &Path,
    file_paths: &[String],
) -> io::Result<Vec<ResolvedSymbolEdge>> {
    if file_paths.is_empty() || !db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };

    list_edges_for_files_in_conn(&conn, file_paths)
}

fn list_edges_for_files_in_conn(
    conn: &Connection,
    file_paths: &[String],
) -> io::Result<Vec<ResolvedSymbolEdge>> {
    if file_paths.is_empty() || !table_exists(conn, "edges")? || !table_exists(conn, "symbols")? {
        return Ok(Vec::new());
    }

    let placeholders = sql_placeholders(file_paths.len());
    let sql = format!(
        "SELECT DISTINCT e.id, e.source_id, e.target_id, e.edge_type, e.confidence, COALESCE(e.reason, '')
         FROM edges e
         JOIN symbols src ON src.id = e.source_id
         JOIN symbols tgt ON tgt.id = e.target_id
         WHERE src.file_path IN ({placeholders}) OR tgt.file_path IN ({placeholders})
         ORDER BY e.source_id, e.target_id, e.id"
    );
    let param_refs = file_paths.iter().map(|value| value as &dyn ToSql);
    list_edges_with_params(conn, &sql, param_refs)
}

/// 返回与指定文件集合通过 edge 相邻的一跳源码路径。
pub fn list_adjacent_symbol_files(
    repo_root: &Path,
    file_paths: &[String],
) -> io::Result<Vec<String>> {
    if file_paths.is_empty() || !db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };

    list_adjacent_symbol_files_in_conn(&conn, file_paths)
}

fn list_adjacent_symbol_files_in_conn(
    conn: &Connection,
    file_paths: &[String],
) -> io::Result<Vec<String>> {
    if file_paths.is_empty() || !table_exists(conn, "edges")? || !table_exists(conn, "symbols")? {
        return Ok(Vec::new());
    }

    let placeholders = sql_placeholders(file_paths.len());
    let sql = format!(
        "SELECT DISTINCT
             CASE
                 WHEN src.file_path IN ({placeholders}) THEN tgt.file_path
                 ELSE src.file_path
             END AS adjacent_file
         FROM edges e
         JOIN symbols src ON src.id = e.source_id
         JOIN symbols tgt ON tgt.id = e.target_id
         WHERE src.file_path IN ({placeholders}) OR tgt.file_path IN ({placeholders})
         ORDER BY adjacent_file"
    );
    let param_refs = file_paths.iter().map(|value| value as &dyn ToSql);

    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| io::Error::other(format!("prepare list adjacent symbol files: {e}")))?;
    let rows = stmt
        .query_map(rusqlite::params_from_iter(param_refs), |row| {
            row.get::<_, String>(0)
        })
        .map_err(|e| io::Error::other(format!("query adjacent symbol files: {e}")))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect adjacent symbol files: {e}")))
}

fn list_symbols_with_params<'a>(
    conn: &Connection,
    sql: &str,
    params: impl IntoIterator<Item = &'a dyn ToSql>,
) -> io::Result<Vec<SymbolNode>> {
    let mut stmt = conn
        .prepare(sql)
        .map_err(|e| io::Error::other(format!("prepare list symbols by files: {e}")))?;
    let rows = stmt
        .query_map(rusqlite::params_from_iter(params), |row| {
            Ok(SymbolNode {
                symbol_id: row.get(0)?,
                name: row.get(1)?,
                label: row.get(2)?,
                file_path: row.get(3)?,
                start_line: row.get::<_, i64>(4)? as usize,
                end_line: row.get::<_, i64>(5)? as usize,
                is_exported: row.get::<_, i64>(6)? != 0,
                language: row.get(7)?,
            })
        })
        .map_err(|e| io::Error::other(format!("query list symbols by files: {e}")))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect list symbols by files: {e}")))
}

fn list_edges_with_params<'a>(
    conn: &Connection,
    sql: &str,
    params: impl IntoIterator<Item = &'a dyn ToSql>,
) -> io::Result<Vec<ResolvedSymbolEdge>> {
    let mut stmt = conn
        .prepare(sql)
        .map_err(|e| io::Error::other(format!("prepare list edges by files: {e}")))?;
    let rows = stmt
        .query_map(rusqlite::params_from_iter(params), |row| {
            Ok(ResolvedSymbolEdge {
                edge_id: row.get(0)?,
                source_id: row.get(1)?,
                target_id: row.get(2)?,
                edge_type: row.get(3)?,
                confidence: row.get(4)?,
                reason: row.get(5)?,
            })
        })
        .map_err(|e| io::Error::other(format!("query list edges by files: {e}")))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect list edges by files: {e}")))
}

/// 基于 `edges` 表递归追踪有限深度的调用链与影响范围。
pub fn trace_call_edges(
    repo_root: &Path,
    seed_symbol_ids: &[String],
    max_depth: usize,
    limit: usize,
) -> io::Result<Vec<GraphTraceEdgeHit>> {
    if seed_symbol_ids.is_empty() || max_depth == 0 || limit == 0 || !db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };

    trace_call_edges_in_conn(&conn, seed_symbol_ids, max_depth, limit)
}

fn trace_call_edges_in_conn(
    conn: &Connection,
    seed_symbol_ids: &[String],
    max_depth: usize,
    limit: usize,
) -> io::Result<Vec<GraphTraceEdgeHit>> {
    if seed_symbol_ids.is_empty()
        || max_depth == 0
        || limit == 0
        || !table_exists(conn, "edges")?
        || !table_exists(conn, "symbols")?
    {
        return Ok(Vec::new());
    }

    let seed_sql = (0..seed_symbol_ids.len())
        .map(|index| format!("SELECT ?{}", index + 1))
        .collect::<Vec<_>>()
        .join(" UNION ALL ");
    let depth_param = seed_symbol_ids.len() + 1;
    let limit_param = seed_symbol_ids.len() + 2;
    let sql = format!(
        "WITH RECURSIVE
            seed(symbol_id) AS ({seed_sql}),
            outbound(depth, edge_id, source_id, target_id, confidence, reason, visited) AS (
                SELECT
                    1,
                    e.id,
                    e.source_id,
                    e.target_id,
                    e.confidence,
                    COALESCE(e.reason, ''),
                    printf('|%s|%s|', e.source_id, e.target_id)
                FROM edges e
                JOIN seed s ON e.source_id = s.symbol_id
                WHERE e.edge_type = 'CALLS'
              UNION ALL
                SELECT
                    outbound.depth + 1,
                    e.id,
                    e.source_id,
                    e.target_id,
                    e.confidence,
                    COALESCE(e.reason, ''),
                    outbound.visited || e.target_id || '|'
                FROM edges e
                JOIN outbound ON e.source_id = outbound.target_id
                WHERE e.edge_type = 'CALLS'
                  AND outbound.depth < ?{depth_param}
                  AND instr(outbound.visited, printf('|%s|', e.target_id)) = 0
            ),
            inbound(depth, edge_id, source_id, target_id, confidence, reason, visited) AS (
                SELECT
                    1,
                    e.id,
                    e.source_id,
                    e.target_id,
                    e.confidence,
                    COALESCE(e.reason, ''),
                    printf('|%s|%s|', e.target_id, e.source_id)
                FROM edges e
                JOIN seed s ON e.target_id = s.symbol_id
                WHERE e.edge_type = 'CALLS'
              UNION ALL
                SELECT
                    inbound.depth + 1,
                    e.id,
                    e.source_id,
                    e.target_id,
                    e.confidence,
                    COALESCE(e.reason, ''),
                    inbound.visited || e.source_id || '|'
                FROM edges e
                JOIN inbound ON e.target_id = inbound.source_id
                WHERE e.edge_type = 'CALLS'
                  AND inbound.depth < ?{depth_param}
                  AND instr(inbound.visited, printf('|%s|', e.source_id)) = 0
            ),
            traces(direction, depth, edge_id, source_id, target_id, confidence, reason) AS (
                SELECT 'outbound', depth, edge_id, source_id, target_id, confidence, reason FROM outbound
                UNION ALL
                SELECT 'inbound', depth, edge_id, source_id, target_id, confidence, reason FROM inbound
            )
         SELECT
            direction,
            edge_id,
            source_id,
            target_id,
            MIN(depth) AS hop_distance,
            MAX(confidence) AS confidence,
            MAX(reason) AS reason
         FROM traces
         GROUP BY direction, edge_id, source_id, target_id
         ORDER BY hop_distance ASC, confidence DESC, edge_id ASC
         LIMIT ?{limit_param}"
    );

    let mut params = seed_symbol_ids
        .iter()
        .cloned()
        .map(Value::Text)
        .collect::<Vec<_>>();
    params.push(Value::Integer(max_depth as i64));
    params.push(Value::Integer(limit as i64));

    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| io::Error::other(format!("prepare trace_call_edges: {e}")))?;
    let rows = stmt
        .query_map(params_from_iter(params.iter()), |row| {
            Ok(GraphTraceEdgeHit {
                traversal_direction: row.get(0)?,
                edge_id: row.get(1)?,
                source_id: row.get(2)?,
                target_id: row.get(3)?,
                hop_distance: row.get::<_, i64>(4)? as usize,
                confidence: row.get(5)?,
                reason: row.get(6)?,
            })
        })
        .map_err(|e| io::Error::other(format!("query trace_call_edges: {e}")))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect trace_call_edges: {e}")))
}

/// 列出当前 DB 内全部 communities。
pub fn list_communities(repo_root: &Path) -> io::Result<Vec<CommunityNode>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };

    list_communities_in_conn(&conn)
}

fn list_communities_in_conn(conn: &Connection) -> io::Result<Vec<CommunityNode>> {
    if !table_exists(conn, "communities")? {
        return Ok(Vec::new());
    }

    let mut stmt = conn
        .prepare(
            "SELECT id, COALESCE(label, ''), COALESCE(cohesion, 0.0), COALESCE(symbol_count, 0)
             FROM communities
             ORDER BY id",
        )
        .map_err(|e| io::Error::other(format!("prepare list communities: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(CommunityNode {
                community_id: row.get(0)?,
                label: row.get(1)?,
                cohesion: row.get(2)?,
                symbol_count: row.get::<_, i64>(3)? as usize,
            })
        })
        .map_err(|e| io::Error::other(format!("query list communities: {e}")))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect list communities: {e}")))
}

/// 列出当前 DB 内全部 community 成员映射。
pub fn list_community_members(repo_root: &Path) -> io::Result<Vec<CommunityMember>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };

    list_community_members_in_conn(&conn)
}

fn list_community_members_in_conn(conn: &Connection) -> io::Result<Vec<CommunityMember>> {
    if !table_exists(conn, "community_members")? {
        return Ok(Vec::new());
    }

    let mut stmt = conn
        .prepare(
            "SELECT community_id, symbol_id
             FROM community_members
             ORDER BY community_id, symbol_id",
        )
        .map_err(|e| io::Error::other(format!("prepare list community members: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(CommunityMember {
                community_id: row.get(0)?,
                symbol_id: row.get(1)?,
            })
        })
        .map_err(|e| io::Error::other(format!("query list community members: {e}")))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect list community members: {e}")))
}

/// 列出当前 DB 内全部 processes。
pub fn list_processes(repo_root: &Path) -> io::Result<Vec<ProcessNode>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };

    list_processes_in_conn(&conn)
}

fn list_processes_in_conn(conn: &Connection) -> io::Result<Vec<ProcessNode>> {
    if !table_exists(conn, "processes")? {
        return Ok(Vec::new());
    }

    let mut stmt = conn
        .prepare(
            "SELECT id, COALESCE(label, ''), COALESCE(process_type, ''), COALESCE(step_count, 0),
                    entry_point_id, terminal_id
             FROM processes
             ORDER BY id",
        )
        .map_err(|e| io::Error::other(format!("prepare list processes: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(ProcessNode {
                process_id: row.get(0)?,
                label: row.get(1)?,
                process_type: row.get(2)?,
                step_count: row.get::<_, i64>(3)? as usize,
                entry_point_id: row.get(4)?,
                terminal_id: row.get(5)?,
            })
        })
        .map_err(|e| io::Error::other(format!("query list processes: {e}")))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect list processes: {e}")))
}

/// 列出当前 DB 内全部 process steps。
pub fn list_process_steps(repo_root: &Path) -> io::Result<Vec<ProcessStep>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };

    list_process_steps_in_conn(&conn)
}

fn list_process_steps_in_conn(conn: &Connection) -> io::Result<Vec<ProcessStep>> {
    if !table_exists(conn, "process_steps")? {
        return Ok(Vec::new());
    }

    let mut stmt = conn
        .prepare(
            "SELECT process_id, symbol_id, step_order
             FROM process_steps
             ORDER BY process_id, step_order",
        )
        .map_err(|e| io::Error::other(format!("prepare list process steps: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(ProcessStep {
                process_id: row.get(0)?,
                symbol_id: row.get(1)?,
                step_order: row.get::<_, i64>(2)? as usize,
            })
        })
        .map_err(|e| io::Error::other(format!("query list process steps: {e}")))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect list process steps: {e}")))
}

/// 用 `symbols_fts` 执行符号级 FTS/BM25 搜索。
/// 返回的是符号原始命中，页面/模块上下文会在 query workflow 再做回填。
pub fn search_symbols_fts(
    repo_root: &Path,
    term: &str,
    limit: usize,
) -> io::Result<Vec<FtsSymbolHit>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };

    search_symbols_fts_in_conn(&conn, term, limit)
}

fn search_symbols_fts_in_conn(
    conn: &Connection,
    term: &str,
    limit: usize,
) -> io::Result<Vec<FtsSymbolHit>> {
    if !table_exists(conn, "symbols_fts")? || !table_exists(conn, "symbols")? {
        return Ok(Vec::new());
    }

    let Some(query) = build_fts_query(term) else {
        return Ok(Vec::new());
    };

    let mut stmt = match conn.prepare(
        "SELECT s.id, s.name, s.label, s.file_path, COALESCE(s.language, ''), bm25(symbols_fts) AS score
         FROM symbols_fts
         JOIN symbols s ON s.id = symbols_fts.symbol_id
         WHERE symbols_fts MATCH ?1
         ORDER BY score
         LIMIT ?2",
    ) {
        Ok(stmt) => stmt,
        Err(_) => return Ok(Vec::new()),
    };

    let rows = match stmt.query_map(params![query, limit as i64], |row| {
        Ok(FtsSymbolHit {
            symbol_id: row.get(0)?,
            name: row.get(1)?,
            label: row.get(2)?,
            file_path: row.get(3)?,
            language: row.get(4)?,
            score: row.get(5)?,
        })
    }) {
        Ok(rows) => rows,
        Err(_) => return Ok(Vec::new()),
    };

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect symbols_fts hits: {e}")))
}

fn build_fts_query(term: &str) -> Option<String> {
    let trimmed = term.trim();
    if trimmed.is_empty() {
        return None;
    }

    let tokens = trimmed
        .split(|ch: char| ch.is_whitespace() || matches!(ch, '/' | '\\' | '.' | '-' | '_' | ':'))
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();

    if tokens.is_empty() {
        return None;
    }

    Some(
        tokens
            .into_iter()
            .map(|token| format!("\"{}\"*", token.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join(" OR "),
    )
}

// ---------------------------------------------------------------------------
// kv_store 兼容读写
// ---------------------------------------------------------------------------

/// 写入 kv_store 键值对（INSERT OR REPLACE）。
pub fn kv_set(conn: &Connection, key: &str, value: &str) -> io::Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO kv_store (key, value) VALUES (?1, ?2)",
        params![key, value],
    )
    .map_err(|e| io::Error::other(format!("kv_set({key}): {e}")))?;
    Ok(())
}

/// 读取 kv_store 键值对。键不存在时返回 None。
pub fn kv_get(conn: &Connection, key: &str) -> io::Result<Option<String>> {
    conn.query_row(
        "SELECT value FROM kv_store WHERE key = ?1",
        params![key],
        |row| row.get::<_, String>(0),
    )
    .optional()
    .map_err(|e| io::Error::other(format!("kv_get({key}): {e}")))
}

/// 检查 kv_store 中某个键是否存在。
pub fn kv_exists(conn: &Connection, key: &str) -> io::Result<bool> {
    let exists = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM kv_store WHERE key = ?1)",
            params![key],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|e| io::Error::other(format!("kv_exists({key}): {e}")))?;
    Ok(exists != 0)
}

// ---------------------------------------------------------------------------
// page_context_cache CRUD
// ---------------------------------------------------------------------------

pub fn write_page_context(
    conn: &Connection,
    page_id: &str,
    input_hash: &str,
    context_json: &str,
) -> io::Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO page_context_cache (page_id, input_hash, context)
         VALUES (?1, ?2, ?3)",
        params![page_id, input_hash, context_json],
    )
    .map_err(|e| io::Error::other(format!("write_page_context({page_id}): {e}")))?;
    Ok(())
}

pub fn read_page_context(conn: &Connection, page_id: &str) -> io::Result<Option<(String, String)>> {
    conn.query_row(
        "SELECT input_hash, context FROM page_context_cache WHERE page_id = ?1",
        params![page_id],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
    )
    .optional()
    .map_err(|e| io::Error::other(format!("read_page_context({page_id}): {e}")))
}

pub fn page_context_exists(conn: &Connection, page_id: &str) -> io::Result<bool> {
    let exists = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM page_context_cache WHERE page_id = ?1)",
            params![page_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|e| io::Error::other(format!("page_context_exists({page_id}): {e}")))?;
    Ok(exists != 0)
}

pub fn remove_page_context(conn: &Connection, page_id: &str) -> io::Result<()> {
    conn.execute(
        "DELETE FROM page_context_cache WHERE page_id = ?1",
        params![page_id],
    )
    .map_err(|e| io::Error::other(format!("remove_page_context({page_id}): {e}")))?;
    Ok(())
}

// ---------------------------------------------------------------------------
// page_generation_cache CRUD
// ---------------------------------------------------------------------------

pub fn write_page_generation(
    conn: &Connection,
    page_id: &str,
    input_hash: &str,
    content_hash: &str,
    sections_json: &str,
) -> io::Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO page_generation_cache
         (page_id, input_hash, content_hash, sections)
         VALUES (?1, ?2, ?3, ?4)",
        params![page_id, input_hash, content_hash, sections_json],
    )
    .map_err(|e| io::Error::other(format!("write_page_generation({page_id}): {e}")))?;
    Ok(())
}

pub fn read_page_generation(
    conn: &Connection,
    page_id: &str,
) -> io::Result<Option<(String, String, String)>> {
    conn.query_row(
        "SELECT input_hash, content_hash, sections
         FROM page_generation_cache
         WHERE page_id = ?1",
        params![page_id],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        },
    )
    .optional()
    .map_err(|e| io::Error::other(format!("read_page_generation({page_id}): {e}")))
}

pub fn page_generation_exists(conn: &Connection, page_id: &str) -> io::Result<bool> {
    let exists = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM page_generation_cache WHERE page_id = ?1)",
            params![page_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|e| io::Error::other(format!("page_generation_exists({page_id}): {e}")))?;
    Ok(exists != 0)
}

pub fn remove_page_generation(conn: &Connection, page_id: &str) -> io::Result<()> {
    conn.execute(
        "DELETE FROM page_generation_cache WHERE page_id = ?1",
        params![page_id],
    )
    .map_err(|e| io::Error::other(format!("remove_page_generation({page_id}): {e}")))?;
    Ok(())
}

pub fn remove_page_all(conn: &Connection, page_id: &str) -> io::Result<()> {
    remove_page_context(conn, page_id)?;
    remove_page_generation(conn, page_id)?;
    conn.execute(
        "DELETE FROM wiki_page_sections WHERE page_id = ?1",
        params![page_id],
    )
    .map_err(|e| io::Error::other(format!("remove page sections {page_id}: {e}")))?;
    conn.execute(
        "DELETE FROM wiki_pages_fts WHERE page_id = ?1",
        params![page_id],
    )
    .map_err(|e| io::Error::other(format!("remove page fts {page_id}: {e}")))?;
    Ok(())
}

// ---------------------------------------------------------------------------
// llm_cache CRUD
// ---------------------------------------------------------------------------

/// 写入 prompt 级 LLM 缓存。
pub fn write_llm_cache(repo_root: &Path, entry: &LlmCacheEntry) -> io::Result<()> {
    let conn = open_db(repo_root)?;
    write_llm_cache_in_conn(&conn, entry)
}

fn write_llm_cache_in_conn(conn: &Connection, entry: &LlmCacheEntry) -> io::Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO llm_cache
         (input_hash, prompt_type, prompt_version, response, model, created_at, ttl_seconds)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            entry.input_hash,
            entry.prompt_type,
            entry.prompt_version,
            entry.response,
            entry.model,
            entry.created_at,
            entry.ttl_seconds
        ],
    )
    .map_err(|e| io::Error::other(format!("write_llm_cache({}): {e}", entry.input_hash)))?;
    Ok(())
}

/// 读取仍然有效的 prompt 级 LLM 缓存。
pub fn read_llm_cache(
    repo_root: &Path,
    input_hash: &str,
    prompt_type: &str,
    prompt_version: &str,
    model: Option<&str>,
) -> io::Result<Option<LlmCacheEntry>> {
    if !db_exists(repo_root) {
        return Ok(None);
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(None),
    };

    read_llm_cache_in_conn(&conn, input_hash, prompt_type, prompt_version, model)
}

fn read_llm_cache_in_conn(
    conn: &Connection,
    input_hash: &str,
    prompt_type: &str,
    prompt_version: &str,
    model: Option<&str>,
) -> io::Result<Option<LlmCacheEntry>> {
    if !table_exists(conn, "llm_cache")? {
        return Ok(None);
    }

    let model = model.unwrap_or_default();
    let entry = conn
        .query_row(
        "SELECT input_hash, prompt_type, prompt_version, response, model, created_at, ttl_seconds
         FROM llm_cache
         WHERE input_hash = ?1
           AND prompt_type = ?2
           AND prompt_version = ?3
           AND (?4 = '' OR COALESCE(model, '') = ?4)",
        params![input_hash, prompt_type, prompt_version, model],
        |row| {
            Ok(LlmCacheEntry {
                input_hash: row.get(0)?,
                prompt_type: row.get(1)?,
                prompt_version: row.get(2)?,
                response: row.get(3)?,
                model: row.get(4)?,
                created_at: row.get(5)?,
                ttl_seconds: row.get(6)?,
            })
        },
    )
    .optional()
    .map_err(|e| io::Error::other(format!("read_llm_cache({input_hash}): {e}")))?;

    Ok(entry.filter(is_llm_cache_entry_fresh))
}

fn is_llm_cache_entry_fresh(entry: &LlmCacheEntry) -> bool {
    if entry.ttl_seconds <= 0 {
        return false;
    }

    let Some(created_at) = parse_llm_cache_timestamp(&entry.created_at) else {
        return false;
    };

    let expires_at = created_at + time::Duration::seconds(entry.ttl_seconds);
    expires_at >= OffsetDateTime::now_utc()
}

fn parse_llm_cache_timestamp(value: &str) -> Option<OffsetDateTime> {
    OffsetDateTime::parse(value, &Rfc3339).ok().or_else(|| {
        let normalized = value.trim().replace(' ', "T");
        let normalized = if normalized.ends_with('Z') {
            normalized
        } else {
            format!("{normalized}Z")
        };
        OffsetDateTime::parse(&normalized, &Rfc3339).ok()
    })
}
