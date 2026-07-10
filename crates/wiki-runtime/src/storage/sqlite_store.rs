//! SQLite 统一缓存与状态存储层。
//! 迭代 6.5 起，SQLite 不再只是 `kv_store` 容器，而是 runtime 的事实主存储。

use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::{Path, PathBuf};

use rusqlite::{
    params, params_from_iter, types::Value, Connection, OpenFlags, OptionalExtension, ToSql,
    Transaction,
};
use std::time::Duration;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::domain::checkpoint::UnitRuntimeGate;
use crate::domain::metadata::DirtyState;
use crate::domain::module_tree::{ModuleNode, ModuleTree};
use crate::domain::relation::WikiRelation;
use crate::domain::state::{BuildState, SourceState, WikiPageState, WikiSectionState, WikiState};
use crate::storage::cache_store::{cache_dir, ensure_cache_dir};
use wiki_index::store::{
    FileSearchHit, FolderRecord, GraphPhaseStatus, GraphReadiness, GraphReadinessStatus,
    GraphSnapshot, ModuleRecord, ModuleSourceLink, SourceFileRecord,
};
use wiki_index::symbol_graph::{
    CommunityMember, CommunityNode, GraphAnalysisSnapshot, ProcessNode, ProcessStep,
    ResolvedGraphSnapshot, ResolvedSymbolEdge,
};
use wiki_index::symbols::{
    GraphPhase, RawCallCapture, RawCaptureBase, RawCaptureKind, RawHeritageCapture,
    RawImportCapture, ReferenceKind, SourceRange, SymbolNode, UnresolvedRef,
};

/// DB 文件名。
const DB_FILENAME: &str = "wiki-cache.db";
const SQLITE_BUSY_TIMEOUT_MS: u64 = 60_000;

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
    /// 命中符号。
    pub symbol: SymbolNode,
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

/// SQLite 中治理派生快照的序列化记录。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceCacheRecord {
    pub schema_version: String,
    pub policy_version: String,
    pub evidence_fingerprint: String,
    pub summary_json: String,
    pub change_jsons: Vec<String>,
    pub artifact_ref_jsons: Vec<String>,
    pub issue_jsons: Vec<String>,
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

    conn.busy_timeout(Duration::from_millis(SQLITE_BUSY_TIMEOUT_MS))
        .map_err(|e| io::Error::other(format!("sqlite busy_timeout: {e}")))?;
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

    let conn = Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|e| io::Error::other(format!("sqlite open readonly: {e}")))?;
    conn.busy_timeout(Duration::from_millis(SQLITE_BUSY_TIMEOUT_MS))
        .map_err(|e| io::Error::other(format!("sqlite readonly busy_timeout: {e}")))?;
    Ok(conn)
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
        "page_source_map",
        "page_module_map",
        "wiki_relations",
        "runtime_meta",
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
    init_index_tables(conn)?;
    init_knowledge_tables(conn)?;
    init_runtime_tables(conn)?;
    init_governance_tables(conn)?;
    Ok(())
}

fn init_governance_tables(conn: &Connection) -> io::Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS governance_snapshots (
            singleton_id         INTEGER PRIMARY KEY CHECK(singleton_id = 1),
            schema_version       TEXT NOT NULL,
            policy_version       TEXT NOT NULL,
            evidence_fingerprint TEXT NOT NULL,
            summary_json         TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS governance_changes (
            sort_order   INTEGER PRIMARY KEY,
            payload_json TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS governance_artifact_refs (
            sort_order   INTEGER PRIMARY KEY,
            payload_json TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS governance_issues (
            sort_order   INTEGER PRIMARY KEY,
            payload_json TEXT NOT NULL
        );",
    )
    .map_err(|error| io::Error::other(format!("governance schema init: {error}")))?;
    Ok(())
}

/// 在单个事务内替换完整 governance derived snapshot。
pub fn replace_governance_cache(
    conn: &mut Connection,
    record: &GovernanceCacheRecord,
) -> io::Result<()> {
    let tx = conn
        .transaction()
        .map_err(|error| io::Error::other(format!("begin governance cache tx: {error}")))?;
    tx.execute_batch(
        "DELETE FROM governance_changes;
         DELETE FROM governance_artifact_refs;
         DELETE FROM governance_issues;
         DELETE FROM governance_snapshots;",
    )
    .map_err(|error| io::Error::other(format!("clear governance cache: {error}")))?;
    tx.execute(
        "INSERT INTO governance_snapshots
         (singleton_id, schema_version, policy_version, evidence_fingerprint, summary_json)
         VALUES (1, ?1, ?2, ?3, ?4)",
        params![
            record.schema_version,
            record.policy_version,
            record.evidence_fingerprint,
            record.summary_json
        ],
    )
    .map_err(|error| io::Error::other(format!("insert governance snapshot: {error}")))?;
    insert_governance_payload_rows(&tx, "governance_changes", &record.change_jsons)?;
    insert_governance_payload_rows(&tx, "governance_artifact_refs", &record.artifact_ref_jsons)?;
    insert_governance_payload_rows(&tx, "governance_issues", &record.issue_jsons)?;
    tx.commit()
        .map_err(|error| io::Error::other(format!("commit governance cache tx: {error}")))
}

/// 清除所有 governance derived rows，不影响其它 SQLite 分层。
pub fn clear_governance_cache(conn: &mut Connection) -> io::Result<()> {
    let tx = conn
        .transaction()
        .map_err(|error| io::Error::other(format!("begin clear governance cache tx: {error}")))?;
    tx.execute_batch(
        "DELETE FROM governance_changes;
         DELETE FROM governance_artifact_refs;
         DELETE FROM governance_issues;
         DELETE FROM governance_snapshots;",
    )
    .map_err(|error| io::Error::other(format!("clear governance cache: {error}")))?;
    tx.commit()
        .map_err(|error| io::Error::other(format!("commit clear governance cache tx: {error}")))
}

/// 读取与指定 fingerprint、schema 和 policy 版本完全匹配的治理缓存。
pub fn read_governance_cache(
    conn: &Connection,
    expected_fingerprint: &str,
    schema_version: &str,
    policy_version: &str,
) -> io::Result<Option<GovernanceCacheRecord>> {
    let snapshot = conn
        .query_row(
            "SELECT schema_version, policy_version, evidence_fingerprint, summary_json
             FROM governance_snapshots
             WHERE singleton_id = 1
               AND evidence_fingerprint = ?1
               AND schema_version = ?2
               AND policy_version = ?3",
            params![expected_fingerprint, schema_version, policy_version],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            },
        )
        .optional()
        .map_err(|error| io::Error::other(format!("read governance snapshot: {error}")))?;
    let Some((schema_version, policy_version, evidence_fingerprint, summary_json)) = snapshot
    else {
        return Ok(None);
    };

    Ok(Some(GovernanceCacheRecord {
        schema_version,
        policy_version,
        evidence_fingerprint,
        summary_json,
        change_jsons: read_governance_payload_rows(conn, "governance_changes")?,
        artifact_ref_jsons: read_governance_payload_rows(conn, "governance_artifact_refs")?,
        issue_jsons: read_governance_payload_rows(conn, "governance_issues")?,
    }))
}

fn insert_governance_payload_rows(
    tx: &Transaction<'_>,
    table: &str,
    payloads: &[String],
) -> io::Result<()> {
    let sql = format!("INSERT INTO {table} (sort_order, payload_json) VALUES (?1, ?2)");
    for (index, payload) in payloads.iter().enumerate() {
        tx.execute(&sql, params![index as i64, payload])
            .map_err(|error| io::Error::other(format!("insert {table}: {error}")))?;
    }
    Ok(())
}

fn read_governance_payload_rows(conn: &Connection, table: &str) -> io::Result<Vec<String>> {
    let sql = format!("SELECT payload_json FROM {table} ORDER BY sort_order");
    let mut statement = conn
        .prepare(&sql)
        .map_err(|error| io::Error::other(format!("prepare read {table}: {error}")))?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|error| io::Error::other(format!("query {table}: {error}")))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| io::Error::other(format!("collect {table}: {error}")))
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

fn init_index_tables(conn: &Connection) -> io::Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS scan_cache (
            cache_key TEXT PRIMARY KEY,
            value     TEXT NOT NULL
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
            source_id TEXT NOT NULL,
            PRIMARY KEY (module_id, source_id)
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

        CREATE TABLE IF NOT EXISTS graph_snapshots (
            snapshot_id        TEXT PRIMARY KEY,
            source_fingerprint TEXT NOT NULL,
            status             TEXT NOT NULL,
            created_at         TEXT NOT NULL,
            is_current         INTEGER DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS files (
            file_id     TEXT PRIMARY KEY,
            path        TEXT NOT NULL UNIQUE,
            language    TEXT NOT NULL,
            kind        TEXT NOT NULL,
            fingerprint TEXT NOT NULL,
            size        INTEGER NOT NULL,
            indexed_at  TEXT NOT NULL,
            diagnostics TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS folders (
            folder_id TEXT PRIMARY KEY,
            path      TEXT NOT NULL UNIQUE,
            parent_id TEXT
        );

        CREATE TABLE IF NOT EXISTS raw_imports (
            capture_id       TEXT PRIMARY KEY,
            file_id          TEXT NOT NULL,
            file_path        TEXT NOT NULL,
            language         TEXT NOT NULL,
            source_symbol_id TEXT,
            raw_text         TEXT NOT NULL,
            target_hint      TEXT,
            raw_path         TEXT NOT NULL,
            imported_name    TEXT,
            alias            TEXT,
            line             INTEGER NOT NULL,
            parser_id        TEXT NOT NULL,
            parser_version   TEXT NOT NULL,
            diagnostics      TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS raw_calls (
            capture_id       TEXT PRIMARY KEY,
            file_id          TEXT NOT NULL,
            file_path        TEXT NOT NULL,
            language         TEXT NOT NULL,
            source_symbol_id TEXT,
            raw_text         TEXT NOT NULL,
            target_hint      TEXT,
            called_name      TEXT NOT NULL,
            receiver_text    TEXT,
            argument_shape   TEXT,
            line             INTEGER NOT NULL,
            parser_id        TEXT NOT NULL,
            parser_version   TEXT NOT NULL,
            diagnostics      TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS raw_heritage (
            capture_id       TEXT PRIMARY KEY,
            file_id          TEXT NOT NULL,
            file_path        TEXT NOT NULL,
            language         TEXT NOT NULL,
            source_symbol_id TEXT,
            raw_text         TEXT NOT NULL,
            target_hint      TEXT,
            owner_name       TEXT NOT NULL,
            owner_symbol_id  TEXT,
            target_name      TEXT NOT NULL,
            relation_kind    TEXT NOT NULL,
            line             INTEGER NOT NULL,
            parser_id        TEXT NOT NULL,
            parser_version   TEXT NOT NULL,
            diagnostics      TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS unresolved_refs (
            unresolved_ref_id TEXT PRIMARY KEY,
            capture_id        TEXT NOT NULL,
            file_id           TEXT NOT NULL,
            resolver_phase    TEXT NOT NULL,
            reference_kind    TEXT NOT NULL,
            reference_name    TEXT NOT NULL,
            target_hint       TEXT,
            path              TEXT NOT NULL,
            start_line        INTEGER NOT NULL,
            end_line          INTEGER NOT NULL,
            start_column      INTEGER NOT NULL,
            end_column        INTEGER NOT NULL,
            candidates        TEXT NOT NULL,
            reason            TEXT NOT NULL,
            diagnostics       TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS graph_phase_runs (
            phase              TEXT PRIMARY KEY,
            status             TEXT NOT NULL,
            input_fingerprint  TEXT,
            output_fingerprint TEXT,
            started_at         TEXT,
            completed_at       TEXT,
            diagnostics        TEXT NOT NULL
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS files_fts
        USING fts5(
            file_id UNINDEXED,
            path,
            language,
            content
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

        CREATE VIRTUAL TABLE IF NOT EXISTS symbols_fts
        USING fts5(
            symbol_id UNINDEXED,
            name,
            file_path,
            content
        );
        CREATE INDEX IF NOT EXISTS idx_modules_parent ON modules(parent_id);
        CREATE INDEX IF NOT EXISTS idx_symbols_file ON symbols(file_path);
        CREATE INDEX IF NOT EXISTS idx_symbols_name ON symbols(name);
        CREATE INDEX IF NOT EXISTS idx_symbols_label ON symbols(label);
        CREATE INDEX IF NOT EXISTS idx_edges_source ON edges(source_id);
        CREATE INDEX IF NOT EXISTS idx_edges_target ON edges(target_id);
        CREATE INDEX IF NOT EXISTS idx_edges_type ON edges(edge_type);
        CREATE INDEX IF NOT EXISTS idx_files_path ON files(path);
        CREATE INDEX IF NOT EXISTS idx_raw_imports_file ON raw_imports(file_path);
        CREATE INDEX IF NOT EXISTS idx_raw_calls_file ON raw_calls(file_path);
        CREATE INDEX IF NOT EXISTS idx_raw_heritage_file ON raw_heritage(file_path);
        CREATE INDEX IF NOT EXISTS idx_unresolved_refs_file ON unresolved_refs(file_id);",
    )
    .map_err(|e| io::Error::other(format!("index schema init: {e}")))?;
    Ok(())
}

fn init_knowledge_tables(conn: &Connection) -> io::Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS knowledge_domains (
            id             TEXT PRIMARY KEY,
            domain_type    TEXT NOT NULL,
            label          TEXT NOT NULL,
            evidence       TEXT,
            source_modules TEXT,
            source_files   TEXT
        );

        CREATE TABLE IF NOT EXISTS knowledge_units (
            id              TEXT PRIMARY KEY,
            unit_type       TEXT NOT NULL,
            title           TEXT NOT NULL,
            domain_id       TEXT REFERENCES knowledge_domains(id),
            parent_unit_id  TEXT REFERENCES knowledge_units(id),
            relative_path   TEXT NOT NULL,
            scope           TEXT,
            priority        REAL
        );

        CREATE TABLE IF NOT EXISTS research_cache (
            research_type  TEXT NOT NULL,
            target_id      TEXT NOT NULL,
            input_hash     TEXT NOT NULL,
            result         TEXT NOT NULL,
            model          TEXT,
            created_at     TEXT DEFAULT (datetime('now')),
            ttl_seconds    INTEGER DEFAULT 604800,
            PRIMARY KEY (research_type, target_id)
        );

        CREATE TABLE IF NOT EXISTS page_digests (
            unit_id      TEXT PRIMARY KEY REFERENCES knowledge_units(id),
            digest       TEXT NOT NULL,
            content_hash TEXT,
            created_at   TEXT DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS page_drafts (
            unit_id      TEXT PRIMARY KEY REFERENCES knowledge_units(id),
            draft        TEXT NOT NULL,
            content_hash TEXT,
            created_at   TEXT DEFAULT (datetime('now'))
        );

        CREATE INDEX IF NOT EXISTS idx_knowledge_units_domain ON knowledge_units(domain_id);
        CREATE INDEX IF NOT EXISTS idx_knowledge_units_parent ON knowledge_units(parent_unit_id);
        CREATE INDEX IF NOT EXISTS idx_research_cache_type ON research_cache(research_type);",
    )
    .map_err(|e| io::Error::other(format!("knowledge schema init: {e}")))?;
    Ok(())
}

fn init_runtime_tables(conn: &Connection) -> io::Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS runtime_meta (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
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

        CREATE TABLE IF NOT EXISTS page_source_map (
            page_id   TEXT NOT NULL REFERENCES wiki_pages(page_id) ON DELETE CASCADE,
            source_id TEXT NOT NULL REFERENCES source_states(source_id) ON DELETE CASCADE,
            PRIMARY KEY (page_id, source_id)
        );

        CREATE TABLE IF NOT EXISTS page_module_map (
            page_id    TEXT NOT NULL REFERENCES wiki_pages(page_id) ON DELETE CASCADE,
            module_id  TEXT NOT NULL,
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

        CREATE VIRTUAL TABLE IF NOT EXISTS wiki_pages_fts
        USING fts5(
            page_id UNINDEXED,
            title,
            path,
            summary
        );

        CREATE TABLE IF NOT EXISTS pipeline_checkpoint (
            checkpoint_id         TEXT PRIMARY KEY,
            facts_input_hash      TEXT NOT NULL,
            interrupted_stage     TEXT NOT NULL,
            interrupted_target_id TEXT,
            error_message         TEXT,
            created_at            TEXT DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS unit_runtime_gates (
            unit_id               TEXT PRIMARY KEY REFERENCES knowledge_units(id),
            unit_type             TEXT NOT NULL,
            research_status       TEXT NOT NULL,
            compose_status        TEXT NOT NULL,
            assemble_status       TEXT NOT NULL,
            last_ready_stage      TEXT,
            blocked_reason        TEXT,
            missing_dependencies  TEXT NOT NULL,
            updated_at            TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_wiki_pages_path ON wiki_pages(path);
        CREATE INDEX IF NOT EXISTS idx_sections_page ON wiki_page_sections(page_id, sort_order);
        CREATE INDEX IF NOT EXISTS idx_source_states_path ON source_states(path);
        CREATE INDEX IF NOT EXISTS idx_relations_source ON wiki_relations(source_id);
        CREATE INDEX IF NOT EXISTS idx_relations_target ON wiki_relations(target_id);
        CREATE INDEX IF NOT EXISTS idx_llm_cache_lookup
            ON llm_cache(prompt_type, prompt_version, model);
        CREATE INDEX IF NOT EXISTS idx_unit_runtime_gates_compose ON unit_runtime_gates(compose_status);",
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

fn replace_module_rows_tx(tx: &Transaction<'_>, modules: &[ModuleNode]) -> io::Result<()> {
    tx.execute_batch(
        "DELETE FROM module_source_map;
         DELETE FROM modules;",
    )
    .map_err(|e| io::Error::other(format!("clear module snapshot tables: {e}")))?;

    for (index, module) in modules.iter().enumerate() {
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

    Ok(())
}

fn replace_state_rows_tx(tx: &Transaction<'_>, state: &WikiState) -> io::Result<()> {
    tx.execute_batch(
        "DELETE FROM wiki_page_sections;
         DELETE FROM page_source_map;
         DELETE FROM page_module_map;
         DELETE FROM wiki_relations;
         DELETE FROM wiki_pages;
         DELETE FROM source_states;
         DELETE FROM runtime_meta WHERE key IN ('dirty_state', 'build_state');
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

    for (index, page) in state
        .pages
        .iter()
        .filter(|page| crate::storage::wiki_fs::is_official_page_path(&page.path))
        .enumerate()
    {
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

    for page in state
        .pages
        .iter()
        .filter(|page| crate::storage::wiki_fs::is_official_page_path(&page.path))
    {
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
        if is_spec_path(&symbol.file_path) {
            continue;
        }
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
         DELETE FROM edges;
         DELETE FROM raw_imports;
         DELETE FROM raw_calls;
         DELETE FROM raw_heritage;
         DELETE FROM unresolved_refs;
         DELETE FROM graph_phase_runs;
         DELETE FROM files_fts;
         DELETE FROM files;
         DELETE FROM folders;
         UPDATE graph_snapshots SET is_current = 0;",
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
    let symbol_ids = current_symbol_ids_tx(tx)?;
    for edge in &resolved_graph.edges {
        if edge.source_id.contains(".spec") || edge.target_id.contains(".spec") {
            continue;
        }
        if !symbol_ids.contains(&edge.source_id) || !symbol_ids.contains(&edge.target_id) {
            continue;
        }
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

fn current_symbol_ids_tx(tx: &Transaction<'_>) -> io::Result<BTreeSet<String>> {
    let mut stmt = tx
        .prepare("SELECT id FROM symbols")
        .map_err(|e| io::Error::other(format!("prepare current symbol ids: {e}")))?;
    let rows = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|e| io::Error::other(format!("query current symbol ids: {e}")))?;
    rows.collect::<Result<BTreeSet<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect current symbol ids: {e}")))
}

fn insert_graph_analysis_rows_tx(
    tx: &Transaction<'_>,
    analysis: &GraphAnalysisSnapshot,
) -> io::Result<()> {
    let symbol_ids = current_symbol_ids_tx(tx)?;
    let valid_community_ids = analysis
        .community_members
        .iter()
        .map(|member| member.community_id.clone())
        .collect::<BTreeSet<_>>();
    let valid_process_ids = analysis
        .process_steps
        .iter()
        .map(|step| step.process_id.clone())
        .collect::<BTreeSet<_>>();

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

    for member in analysis
        .community_members
        .iter()
        .filter(|member| symbol_ids.contains(&member.symbol_id))
        .filter(|member| valid_community_ids.contains(&member.community_id))
    {
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

    for step in analysis
        .process_steps
        .iter()
        .filter(|step| symbol_ids.contains(&step.symbol_id))
        .filter(|step| valid_process_ids.contains(&step.process_id))
    {
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

pub fn replace_graph_snapshot(conn: &mut Connection, snapshot: &GraphSnapshot) -> io::Result<()> {
    let tx = conn
        .transaction()
        .map_err(|e| io::Error::other(format!("begin replace_graph_snapshot tx: {e}")))?;
    clear_symbol_graph_rows_tx(&tx)?;
    clear_symbol_rows_tx(&tx)?;
    insert_source_rows_tx(&tx, snapshot)?;
    insert_symbol_rows_tx(&tx, &snapshot.symbols)?;
    let resolved_graph = ResolvedGraphSnapshot {
        edges: snapshot.edges.clone(),
        diagnostics: Vec::new(),
    };
    insert_edge_rows_tx(&tx, &resolved_graph)?;
    insert_raw_rows_tx(&tx, snapshot)?;
    insert_graph_analysis_rows_tx(&tx, &snapshot.analysis)?;
    insert_phase_rows_tx(&tx, &snapshot.phase_statuses)?;
    insert_graph_snapshot_row_tx(&tx, snapshot)?;
    tx.commit()
        .map_err(|e| io::Error::other(format!("commit replace_graph_snapshot tx: {e}")))
}

pub fn replace_graph_snapshot_for_files(
    conn: &mut Connection,
    file_paths: &[String],
    snapshot: &GraphSnapshot,
) -> io::Result<()> {
    let tx = conn
        .transaction()
        .map_err(|e| io::Error::other(format!("begin replace_graph_snapshot_for_files tx: {e}")))?;
    let stale_symbol_ids = select_symbol_ids_for_files_tx(&tx, file_paths)?;
    clear_graph_analysis_rows_tx(&tx)?;
    delete_edge_rows_for_symbol_ids_tx(&tx, &stale_symbol_ids)?;
    delete_graph_rows_for_files_tx(&tx, file_paths)?;
    insert_source_rows_tx(&tx, snapshot)?;
    insert_symbol_rows_tx(&tx, &snapshot.symbols)?;
    let resolved_graph = ResolvedGraphSnapshot {
        edges: snapshot.edges.clone(),
        diagnostics: Vec::new(),
    };
    insert_edge_rows_tx(&tx, &resolved_graph)?;
    insert_raw_rows_tx(&tx, snapshot)?;
    insert_graph_analysis_rows_tx(&tx, &snapshot.analysis)?;
    insert_phase_rows_tx(&tx, &snapshot.phase_statuses)?;
    insert_graph_snapshot_row_tx(&tx, snapshot)?;
    tx.commit()
        .map_err(|e| io::Error::other(format!("commit replace_graph_snapshot_for_files tx: {e}")))
}

fn delete_graph_rows_for_files_tx(tx: &Transaction<'_>, file_paths: &[String]) -> io::Result<()> {
    if file_paths.is_empty() {
        return Ok(());
    }
    let placeholders = sql_placeholders(file_paths.len());
    let params = file_paths.iter().map(|value| value as &dyn ToSql);
    tx.execute(
        &format!("DELETE FROM raw_imports WHERE file_path IN ({placeholders})"),
        params_from_iter(params),
    )
    .map_err(|e| io::Error::other(format!("delete raw_imports by file: {e}")))?;
    let params = file_paths.iter().map(|value| value as &dyn ToSql);
    tx.execute(
        &format!("DELETE FROM raw_calls WHERE file_path IN ({placeholders})"),
        params_from_iter(params),
    )
    .map_err(|e| io::Error::other(format!("delete raw_calls by file: {e}")))?;
    let params = file_paths.iter().map(|value| value as &dyn ToSql);
    tx.execute(
        &format!("DELETE FROM raw_heritage WHERE file_path IN ({placeholders})"),
        params_from_iter(params),
    )
    .map_err(|e| io::Error::other(format!("delete raw_heritage by file: {e}")))?;
    let params = file_paths.iter().map(|value| value as &dyn ToSql);
    tx.execute(
        &format!("DELETE FROM unresolved_refs WHERE path IN ({placeholders})"),
        params_from_iter(params),
    )
    .map_err(|e| io::Error::other(format!("delete unresolved_refs by file: {e}")))?;
    let params = file_paths.iter().map(|value| value as &dyn ToSql);
    tx.execute(
        &format!("DELETE FROM files_fts WHERE path IN ({placeholders})"),
        params_from_iter(params),
    )
    .map_err(|e| io::Error::other(format!("delete files_fts by file: {e}")))?;
    let params = file_paths.iter().map(|value| value as &dyn ToSql);
    tx.execute(
        &format!("DELETE FROM files WHERE path IN ({placeholders})"),
        params_from_iter(params),
    )
    .map_err(|e| io::Error::other(format!("delete files by path: {e}")))?;
    delete_symbol_rows_for_files_tx(tx, file_paths)
}

fn insert_source_rows_tx(tx: &Transaction<'_>, snapshot: &GraphSnapshot) -> io::Result<()> {
    for folder in &snapshot.folders {
        if is_spec_path(&folder.path) {
            continue;
        }
        tx.execute(
            "INSERT OR REPLACE INTO folders (folder_id, path, parent_id) VALUES (?1, ?2, ?3)",
            params![folder.folder_id, folder.path, folder.parent_id],
        )
        .map_err(|e| io::Error::other(format!("insert folder {}: {e}", folder.folder_id)))?;
    }

    for file in &snapshot.files {
        if is_spec_path(&file.path) {
            continue;
        }
        tx.execute(
            "INSERT OR REPLACE INTO files
             (file_id, path, language, kind, fingerprint, size, indexed_at, diagnostics)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                file.file_id,
                file.path,
                file.language,
                file.kind,
                file.fingerprint,
                file.size as i64,
                file.indexed_at,
                json_string(&file.diagnostics)?,
            ],
        )
        .map_err(|e| io::Error::other(format!("insert file {}: {e}", file.file_id)))?;
        tx.execute(
            "INSERT INTO files_fts (file_id, path, language, content) VALUES (?1, ?2, ?3, ?4)",
            params![file.file_id, file.path, file.language, file.path],
        )
        .map_err(|e| io::Error::other(format!("insert files_fts {}: {e}", file.file_id)))?;
    }

    Ok(())
}

fn insert_raw_rows_tx(tx: &Transaction<'_>, snapshot: &GraphSnapshot) -> io::Result<()> {
    for capture in &snapshot.raw_imports {
        if is_spec_path(&capture.file_path) {
            continue;
        }
        tx.execute(
            "INSERT INTO raw_imports
             (capture_id, file_id, file_path, language, source_symbol_id, raw_text, target_hint,
              raw_path, imported_name, alias, line, parser_id, parser_version, diagnostics)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                capture.base.capture_id,
                capture.base.file_id,
                capture.file_path,
                capture.language,
                capture.source_symbol_id,
                capture.source_text,
                capture.base.target_hint,
                capture.raw_path,
                capture.imported_name,
                capture.alias,
                capture.line as i64,
                capture.base.parser_id,
                capture.base.parser_version,
                json_string(&capture.base.diagnostics)?,
            ],
        )
        .map_err(|e| {
            io::Error::other(format!(
                "insert raw import {}: {e}",
                capture.base.capture_id
            ))
        })?;
    }

    for capture in &snapshot.raw_calls {
        if is_spec_path(&capture.file_path) {
            continue;
        }
        tx.execute(
            "INSERT INTO raw_calls
             (capture_id, file_id, file_path, language, source_symbol_id, raw_text, target_hint,
              called_name, receiver_text, argument_shape, line, parser_id, parser_version, diagnostics)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                capture.base.capture_id,
                capture.base.file_id,
                capture.file_path,
                capture.language,
                capture.source_symbol_id,
                capture.source_text,
                capture.base.target_hint,
                capture.called_name,
                capture.receiver_text,
                capture.argument_shape,
                capture.line as i64,
                capture.base.parser_id,
                capture.base.parser_version,
                json_string(&capture.base.diagnostics)?,
            ],
        )
        .map_err(|e| io::Error::other(format!("insert raw call {}: {e}", capture.base.capture_id)))?;
    }

    for capture in &snapshot.raw_heritage {
        if is_spec_path(&capture.file_path) {
            continue;
        }
        tx.execute(
            "INSERT INTO raw_heritage
             (capture_id, file_id, file_path, language, source_symbol_id, raw_text, target_hint,
              owner_name, owner_symbol_id, target_name, relation_kind, line, parser_id, parser_version, diagnostics)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                capture.base.capture_id,
                capture.base.file_id,
                capture.file_path,
                capture.language,
                capture.base.source_symbol_id,
                capture.source_text,
                capture.base.target_hint,
                capture.owner_name,
                capture.owner_symbol_id,
                capture.target_name,
                capture.relation_kind,
                capture.line as i64,
                capture.base.parser_id,
                capture.base.parser_version,
                json_string(&capture.base.diagnostics)?,
            ],
        )
        .map_err(|e| io::Error::other(format!("insert raw heritage {}: {e}", capture.base.capture_id)))?;
    }

    for unresolved in &snapshot.unresolved_refs {
        if is_spec_path(&unresolved.range.path) {
            continue;
        }
        tx.execute(
            "INSERT INTO unresolved_refs
             (unresolved_ref_id, capture_id, file_id, resolver_phase, reference_kind, reference_name,
              target_hint, path, start_line, end_line, start_column, end_column, candidates, reason, diagnostics)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                unresolved.unresolved_ref_id,
                unresolved.capture_id,
                unresolved.file_id,
                graph_phase_to_str(unresolved.resolver_phase),
                reference_kind_to_str(unresolved.reference_kind),
                unresolved.reference_name,
                unresolved.target_hint,
                unresolved.range.path,
                unresolved.range.start_line as i64,
                unresolved.range.end_line as i64,
                unresolved.range.start_column as i64,
                unresolved.range.end_column as i64,
                json_string(&unresolved.candidates)?,
                unresolved.reason,
                json_string(&unresolved.diagnostics)?,
            ],
        )
        .map_err(|e| io::Error::other(format!("insert unresolved ref {}: {e}", unresolved.unresolved_ref_id)))?;
    }

    Ok(())
}

fn insert_phase_rows_tx(tx: &Transaction<'_>, phases: &[GraphPhaseStatus]) -> io::Result<()> {
    for phase in phases {
        tx.execute(
            "INSERT OR REPLACE INTO graph_phase_runs
             (phase, status, input_fingerprint, output_fingerprint, started_at, completed_at, diagnostics)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                graph_phase_to_str(phase.phase),
                phase.status,
                phase.input_fingerprint,
                phase.output_fingerprint,
                phase.started_at,
                phase.completed_at,
                json_string(&phase.diagnostics)?,
            ],
        )
        .map_err(|e| io::Error::other(format!("insert graph phase {:?}: {e}", phase.phase)))?;
    }
    Ok(())
}

fn insert_graph_snapshot_row_tx(tx: &Transaction<'_>, snapshot: &GraphSnapshot) -> io::Result<()> {
    let now = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string());
    tx.execute("UPDATE graph_snapshots SET is_current = 0", [])
        .map_err(|e| io::Error::other(format!("clear current graph snapshot: {e}")))?;
    tx.execute(
        "INSERT INTO graph_snapshots (snapshot_id, source_fingerprint, status, created_at, is_current)
         VALUES (?1, ?2, 'ready', ?3, 1)
         ON CONFLICT(snapshot_id) DO UPDATE SET
             source_fingerprint = excluded.source_fingerprint,
             status = excluded.status,
             created_at = excluded.created_at,
             is_current = excluded.is_current",
        params![snapshot.snapshot_id, snapshot.source_fingerprint, now],
    )
    .map_err(|e| io::Error::other(format!("insert graph snapshot {}: {e}", snapshot.snapshot_id)))?;
    Ok(())
}

pub fn read_current_graph_snapshot(repo_root: &Path) -> io::Result<Option<GraphSnapshot>> {
    if !db_exists(repo_root) {
        return Ok(None);
    }
    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(None),
    };
    if !table_exists(&conn, "graph_snapshots")? {
        return Ok(None);
    }
    let current = conn
        .query_row(
            "SELECT snapshot_id, source_fingerprint FROM graph_snapshots WHERE is_current = 1 LIMIT 1",
            [],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()
        .map_err(|e| io::Error::other(format!("read current graph snapshot: {e}")))?;
    let Some((snapshot_id, source_fingerprint)) = current else {
        return Ok(None);
    };
    Ok(Some(GraphSnapshot {
        snapshot_id,
        source_fingerprint,
        files: list_files(repo_root)?,
        folders: list_folders(repo_root)?,
        symbols: list_symbols(repo_root)?,
        edges: list_edges(repo_root)?,
        raw_imports: list_raw_imports(repo_root)?,
        raw_calls: list_raw_calls(repo_root)?,
        raw_heritage: list_raw_heritage(repo_root)?,
        unresolved_refs: list_unresolved_refs(repo_root)?,
        analysis: GraphAnalysisSnapshot {
            communities: list_communities(repo_root)?,
            community_members: list_community_members(repo_root)?,
            processes: list_processes(repo_root)?,
            process_steps: list_process_steps(repo_root)?,
            cycles: Vec::new(),
            diagnostics: Vec::new(),
        },
        phase_statuses: list_graph_phase_runs(repo_root)?,
    }))
}

pub fn list_files(repo_root: &Path) -> io::Result<Vec<SourceFileRecord>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }
    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };
    if !table_exists(&conn, "files")? {
        return Ok(Vec::new());
    }
    let mut stmt = conn
        .prepare(
            "SELECT file_id, path, language, kind, fingerprint, size, indexed_at, diagnostics
             FROM files ORDER BY path",
        )
        .map_err(|e| io::Error::other(format!("prepare list files: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            let diagnostics: String = row.get(7)?;
            Ok(SourceFileRecord {
                file_id: row.get(0)?,
                path: row.get(1)?,
                language: row.get(2)?,
                kind: row.get(3)?,
                fingerprint: row.get(4)?,
                size: row.get::<_, i64>(5)? as u64,
                indexed_at: row.get(6)?,
                diagnostics: parse_string_list(&diagnostics).unwrap_or_default(),
            })
        })
        .map_err(|e| io::Error::other(format!("query list files: {e}")))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect list files: {e}")))
}

pub fn list_folders(repo_root: &Path) -> io::Result<Vec<FolderRecord>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }
    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };
    if !table_exists(&conn, "folders")? {
        return Ok(Vec::new());
    }
    let mut stmt = conn
        .prepare("SELECT folder_id, path, parent_id FROM folders ORDER BY path")
        .map_err(|e| io::Error::other(format!("prepare list folders: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(FolderRecord {
                folder_id: row.get(0)?,
                path: row.get(1)?,
                parent_id: row.get(2)?,
            })
        })
        .map_err(|e| io::Error::other(format!("query list folders: {e}")))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect list folders: {e}")))
}

pub fn search_files_fts(
    repo_root: &Path,
    term: &str,
    limit: usize,
) -> io::Result<Vec<FileSearchHit>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }
    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };
    if !table_exists(&conn, "files_fts")? || !table_exists(&conn, "files")? {
        return Ok(Vec::new());
    }
    let Some(query) = build_fts_query(term) else {
        return Ok(Vec::new());
    };
    let mut stmt = match conn.prepare(
        "SELECT f.file_id, f.path, f.language, f.kind, bm25(files_fts) AS score
         FROM files_fts
         JOIN files f ON f.file_id = files_fts.file_id
         WHERE files_fts MATCH ?1
         ORDER BY score
         LIMIT ?2",
    ) {
        Ok(stmt) => stmt,
        Err(_) => return Ok(Vec::new()),
    };
    let rows = match stmt.query_map(params![query, limit as i64], |row| {
        Ok(FileSearchHit {
            file_id: row.get(0)?,
            path: row.get(1)?,
            language: row.get(2)?,
            kind: row.get(3)?,
            score: row.get(4)?,
        })
    }) {
        Ok(rows) => rows,
        Err(_) => return Ok(Vec::new()),
    };
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect files_fts hits: {e}")))
}

pub fn list_raw_imports(repo_root: &Path) -> io::Result<Vec<RawImportCapture>> {
    list_raw_imports_like(repo_root)
}

pub fn list_raw_calls(repo_root: &Path) -> io::Result<Vec<RawCallCapture>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }
    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };
    if !table_exists(&conn, "raw_calls")? {
        return Ok(Vec::new());
    }
    let mut stmt = conn
        .prepare(
            "SELECT capture_id, file_id, file_path, language, source_symbol_id, raw_text, target_hint,
                    called_name, receiver_text, argument_shape, line, parser_id, parser_version, diagnostics
             FROM raw_calls ORDER BY file_path, line, capture_id",
        )
        .map_err(|e| io::Error::other(format!("prepare list raw calls: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            let line = row.get::<_, i64>(10)? as usize;
            let diagnostics_json: String = row.get(13)?;
            let base = RawCaptureBase {
                capture_id: row.get(0)?,
                file_id: row.get(1)?,
                language: row.get(3)?,
                capture_kind: RawCaptureKind::Call,
                source_symbol_id: row.get(4)?,
                raw_text: row.get(5)?,
                target_hint: row.get(6)?,
                range: SourceRange::new(
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    line,
                    line,
                    0,
                    0,
                ),
                parser_id: row.get(11)?,
                parser_version: row.get(12)?,
                diagnostics: parse_string_list(&diagnostics_json).unwrap_or_default(),
            };
            Ok(RawCallCapture {
                file_path: base.range.path.clone(),
                called_name: row.get(7)?,
                line,
                language: base.language.clone(),
                source_symbol_id: base.source_symbol_id.clone(),
                receiver_text: row.get(8)?,
                source_text: base.raw_text.clone(),
                argument_shape: row.get(9)?,
                base,
            })
        })
        .map_err(|e| io::Error::other(format!("query raw calls: {e}")))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect raw calls: {e}")))
}

pub fn list_raw_heritage(repo_root: &Path) -> io::Result<Vec<RawHeritageCapture>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }
    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };
    if !table_exists(&conn, "raw_heritage")? {
        return Ok(Vec::new());
    }
    Ok(Vec::new())
}

pub fn list_unresolved_refs(repo_root: &Path) -> io::Result<Vec<UnresolvedRef>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }
    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };
    if !table_exists(&conn, "unresolved_refs")? {
        return Ok(Vec::new());
    }
    let mut stmt = conn
        .prepare(
            "SELECT unresolved_ref_id, capture_id, file_id, resolver_phase, reference_kind,
                    reference_name, target_hint, path, start_line, end_line, start_column, end_column,
                    candidates, reason, diagnostics
             FROM unresolved_refs ORDER BY file_id, start_line, unresolved_ref_id",
        )
        .map_err(|e| io::Error::other(format!("prepare unresolved refs: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            let candidates_json: String = row.get(12)?;
            let diagnostics_json: String = row.get(14)?;
            let file_id: String = row.get(2)?;
            Ok(UnresolvedRef {
                unresolved_ref_id: row.get(0)?,
                capture_id: row.get(1)?,
                file_id: file_id.clone(),
                resolver_phase: graph_phase_from_str(&row.get::<_, String>(3)?),
                reference_kind: reference_kind_from_str(&row.get::<_, String>(4)?),
                reference_name: row.get(5)?,
                target_hint: row.get(6)?,
                range: SourceRange::new(
                    file_id,
                    row.get::<_, String>(7)?,
                    row.get::<_, i64>(8)? as usize,
                    row.get::<_, i64>(9)? as usize,
                    row.get::<_, i64>(10)? as usize,
                    row.get::<_, i64>(11)? as usize,
                ),
                candidates: parse_string_list(&candidates_json).unwrap_or_default(),
                reason: row.get(13)?,
                diagnostics: parse_string_list(&diagnostics_json).unwrap_or_default(),
            })
        })
        .map_err(|e| io::Error::other(format!("query unresolved refs: {e}")))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect unresolved refs: {e}")))
}

pub fn list_graph_phase_runs(repo_root: &Path) -> io::Result<Vec<GraphPhaseStatus>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }
    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };
    if !table_exists(&conn, "graph_phase_runs")? {
        return Ok(Vec::new());
    }
    let mut stmt = conn
        .prepare(
            "SELECT phase, status, input_fingerprint, output_fingerprint, started_at, completed_at, diagnostics
             FROM graph_phase_runs ORDER BY phase",
        )
        .map_err(|e| io::Error::other(format!("prepare graph phase runs: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            let diagnostics_json: String = row.get(6)?;
            Ok(GraphPhaseStatus {
                phase: graph_phase_from_str(&row.get::<_, String>(0)?),
                status: row.get(1)?,
                input_fingerprint: row.get(2)?,
                output_fingerprint: row.get(3)?,
                started_at: row.get(4)?,
                completed_at: row.get(5)?,
                diagnostics: parse_string_list(&diagnostics_json).unwrap_or_default(),
            })
        })
        .map_err(|e| io::Error::other(format!("query graph phase runs: {e}")))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect graph phase runs: {e}")))
}

fn list_raw_imports_like(repo_root: &Path) -> io::Result<Vec<RawImportCapture>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }
    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };
    if !table_exists(&conn, "raw_imports")? {
        return Ok(Vec::new());
    }
    let mut stmt = conn
        .prepare(
            "SELECT capture_id, file_id, file_path, language, source_symbol_id, raw_text, target_hint,
                    raw_path, imported_name, alias, line, parser_id, parser_version, diagnostics
             FROM raw_imports ORDER BY file_path, line, capture_id",
        )
        .map_err(|e| io::Error::other(format!("prepare list raw imports: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            let line = row.get::<_, i64>(10)? as usize;
            let diagnostics_json: String = row.get(13)?;
            let file_id: String = row.get(1)?;
            let file_path: String = row.get(2)?;
            let language: String = row.get(3)?;
            let source_symbol_id: Option<String> = row.get(4)?;
            let raw_text: String = row.get(5)?;
            let base = RawCaptureBase {
                capture_id: row.get(0)?,
                file_id: file_id.clone(),
                language: language.clone(),
                capture_kind: RawCaptureKind::Import,
                source_symbol_id: source_symbol_id.clone(),
                raw_text: raw_text.clone(),
                target_hint: row.get(6)?,
                range: SourceRange::new(file_id, file_path.clone(), line, line, 0, 0),
                parser_id: row.get(11)?,
                parser_version: row.get(12)?,
                diagnostics: parse_string_list(&diagnostics_json).unwrap_or_default(),
            };
            Ok(RawImportCapture {
                file_path,
                raw_path: row.get(7)?,
                imported_name: row.get(8)?,
                alias: row.get(9)?,
                line,
                language,
                source_symbol_id,
                source_text: raw_text,
                base,
            })
        })
        .map_err(|e| io::Error::other(format!("query raw imports: {e}")))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect raw imports: {e}")))
}

pub fn clear_scan_cache_for_test(conn: &Connection) -> io::Result<()> {
    conn.execute("DELETE FROM scan_cache", [])
        .map_err(|e| io::Error::other(format!("clear scan cache for test: {e}")))?;
    Ok(())
}

pub fn read_graph_readiness(repo_root: &Path) -> io::Result<GraphReadiness> {
    if !db_exists(repo_root) {
        return Ok(GraphReadiness {
            status: GraphReadinessStatus::Missing,
            reason: "graph cache is missing".to_string(),
            snapshot_id: None,
            source_fingerprint: None,
            required_tables: Vec::new(),
            diagnostics: Vec::new(),
        });
    }
    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => {
            return Ok(GraphReadiness {
                status: GraphReadinessStatus::Missing,
                reason: "graph cache cannot be opened".to_string(),
                snapshot_id: None,
                source_fingerprint: None,
                required_tables: Vec::new(),
                diagnostics: Vec::new(),
            });
        }
    };
    let required_tables = [
        "graph_snapshots",
        "files",
        "folders",
        "symbols",
        "edges",
        "raw_imports",
        "raw_calls",
        "raw_heritage",
        "unresolved_refs",
        "graph_phase_runs",
        "files_fts",
        "symbols_fts",
    ];
    let missing_tables = required_tables
        .iter()
        .filter_map(|table| match table_exists(&conn, table) {
            Ok(true) => None,
            Ok(false) => Some((*table).to_string()),
            Err(_) => Some((*table).to_string()),
        })
        .collect::<Vec<_>>();
    if !missing_tables.is_empty() {
        return Ok(GraphReadiness {
            status: GraphReadinessStatus::Blocked,
            reason: "required graph tables are missing".to_string(),
            snapshot_id: None,
            source_fingerprint: None,
            required_tables: missing_tables,
            diagnostics: Vec::new(),
        });
    }
    let Some(snapshot) = read_current_graph_snapshot(repo_root)? else {
        return Ok(GraphReadiness {
            status: GraphReadinessStatus::Missing,
            reason: "current graph snapshot is missing".to_string(),
            snapshot_id: None,
            source_fingerprint: None,
            required_tables: vec!["graph_snapshots".to_string()],
            diagnostics: Vec::new(),
        });
    };
    let phases = list_graph_phase_runs(repo_root)?;
    if let Some(phase) = phases.iter().find(|phase| phase.status == "blocked") {
        return Ok(GraphReadiness {
            status: GraphReadinessStatus::Blocked,
            reason: format!("graph phase {} is blocked", graph_phase_to_str(phase.phase)),
            snapshot_id: Some(snapshot.snapshot_id),
            source_fingerprint: Some(snapshot.source_fingerprint),
            required_tables: Vec::new(),
            diagnostics: phase.diagnostics.clone(),
        });
    }
    if phases.iter().any(|phase| phase.status == "rebuilding") {
        return Ok(GraphReadiness {
            status: GraphReadinessStatus::Rebuilding,
            reason: "graph rebuild is in progress".to_string(),
            snapshot_id: Some(snapshot.snapshot_id),
            source_fingerprint: Some(snapshot.source_fingerprint),
            required_tables: Vec::new(),
            diagnostics: Vec::new(),
        });
    }
    if phases.iter().any(|phase| phase.status == "stale") {
        return Ok(GraphReadiness {
            status: GraphReadinessStatus::Stale,
            reason: "graph source snapshot is stale".to_string(),
            snapshot_id: Some(snapshot.snapshot_id),
            source_fingerprint: Some(snapshot.source_fingerprint),
            required_tables: Vec::new(),
            diagnostics: Vec::new(),
        });
    }
    Ok(GraphReadiness {
        status: GraphReadinessStatus::Ready,
        reason: "current graph snapshot is queryable".to_string(),
        snapshot_id: Some(snapshot.snapshot_id),
        source_fingerprint: Some(snapshot.source_fingerprint),
        required_tables: Vec::new(),
        diagnostics: Vec::new(),
    })
}

fn graph_phase_to_str(phase: GraphPhase) -> &'static str {
    match phase {
        GraphPhase::Scan => "scan",
        GraphPhase::Structure => "structure",
        GraphPhase::Parse => "parse",
        GraphPhase::ResolveImports => "resolve_imports",
        GraphPhase::ResolveCalls => "resolve_calls",
        GraphPhase::ResolveHeritage => "resolve_heritage",
        GraphPhase::AnalyzeCommunities => "analyze_communities",
        GraphPhase::AnalyzeProcesses => "analyze_processes",
        GraphPhase::BuildFts => "build_fts",
    }
}

fn graph_phase_from_str(value: &str) -> GraphPhase {
    match value {
        "scan" => GraphPhase::Scan,
        "structure" => GraphPhase::Structure,
        "parse" => GraphPhase::Parse,
        "resolve_imports" => GraphPhase::ResolveImports,
        "resolve_calls" => GraphPhase::ResolveCalls,
        "resolve_heritage" => GraphPhase::ResolveHeritage,
        "analyze_communities" => GraphPhase::AnalyzeCommunities,
        "analyze_processes" => GraphPhase::AnalyzeProcesses,
        "build_fts" => GraphPhase::BuildFts,
        _ => GraphPhase::Parse,
    }
}

fn reference_kind_to_str(kind: ReferenceKind) -> &'static str {
    match kind {
        ReferenceKind::Import => "import",
        ReferenceKind::Call => "call",
        ReferenceKind::Heritage => "heritage",
    }
}

fn reference_kind_from_str(value: &str) -> ReferenceKind {
    match value {
        "call" => ReferenceKind::Call,
        "heritage" => ReferenceKind::Heritage,
        _ => ReferenceKind::Import,
    }
}

fn is_spec_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    let trimmed = normalized.trim_start_matches("./");
    trimmed == ".spec" || trimmed.starts_with(".spec/")
}

/// 只替换 facts/index 侧的 symbols / edges / graph analysis 快照。
pub fn replace_symbol_graph(
    repo_root: &Path,
    symbols: &[SymbolNode],
    edges: &[ResolvedSymbolEdge],
    analysis: &GraphAnalysisSnapshot,
    resolved_graph: &ResolvedGraphSnapshot,
) -> io::Result<()> {
    let mut conn = open_db(repo_root)?;
    let tx = conn
        .transaction()
        .map_err(|e| io::Error::other(format!("begin replace_symbol_graph tx: {e}")))?;

    clear_symbol_graph_rows_tx(&tx)?;
    clear_symbol_rows_tx(&tx)?;
    insert_symbol_rows_tx(&tx, symbols)?;

    let mut graph_snapshot = resolved_graph.clone();
    graph_snapshot.edges = edges.to_vec();
    insert_edge_rows_tx(&tx, &graph_snapshot)?;
    insert_graph_analysis_rows_tx(&tx, analysis)?;

    tx.commit()
        .map_err(|e| io::Error::other(format!("commit replace_symbol_graph tx: {e}")))
}

pub fn replace_symbol_graph_for_files(
    repo_root: &Path,
    file_paths: &[String],
    symbols: &[SymbolNode],
    edges: &[ResolvedSymbolEdge],
    analysis: &GraphAnalysisSnapshot,
    resolved_graph: &ResolvedGraphSnapshot,
) -> io::Result<()> {
    let mut conn = open_db(repo_root)?;
    let tx = conn
        .transaction()
        .map_err(|e| io::Error::other(format!("begin replace_symbol_graph_for_files tx: {e}")))?;
    let stale_symbol_ids = select_symbol_ids_for_files_tx(&tx, file_paths)?;
    clear_graph_analysis_rows_tx(&tx)?;
    delete_edge_rows_for_symbol_ids_tx(&tx, &stale_symbol_ids)?;
    delete_symbol_rows_for_files_tx(&tx, file_paths)?;
    insert_symbol_rows_tx(&tx, symbols)?;
    let mut graph_snapshot = resolved_graph.clone();
    graph_snapshot.edges = edges.to_vec();
    insert_edge_rows_tx(&tx, &graph_snapshot)?;
    insert_graph_analysis_rows_tx(&tx, analysis)?;
    tx.commit()
        .map_err(|e| io::Error::other(format!("commit replace_symbol_graph_for_files tx: {e}")))
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

/// 把 `ModuleTree` 投影成模块与源码的关联索引。
fn module_membership_from_tree(
    module_tree: &ModuleTree,
) -> (Vec<ModuleNode>, BTreeMap<String, Vec<String>>) {
    let mut source_to_module_ids = BTreeMap::<String, Vec<String>>::new();
    for module in &module_tree.modules {
        for source_id in &module.source_ids {
            source_to_module_ids
                .entry(source_id.clone())
                .or_default()
                .push(module.id.clone());
        }
    }

    (module_tree.modules.clone(), source_to_module_ids)
}

/// 用 index 提供的模块树补全运行时状态表，恢复完整 WikiState。
pub fn load_state_rows(conn: &Connection, module_tree: &ModuleTree) -> io::Result<WikiState> {
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
                    owner_kind: Some(if row.get::<_, i64>(3)? != 0 {
                        wiki_model::domain::projection::SectionOwnership::DerivedManaged
                    } else {
                        wiki_model::domain::projection::SectionOwnership::ManualUnmanaged
                    }),
                    knowledge_refs: Vec::new(),
                    content_hash: row.get(4)?,
                    input_hash: String::new(),
                    generated_content_hash: row.get(5)?,
                    projection_digest_ref: None,
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

    let (modules, mut source_to_module_ids) = module_membership_from_tree(module_tree);

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

/// 用 index snapshot 刷新 `modules / module_source_map` 真相表。
pub fn replace_module_tree_snapshot(conn: &Connection, module_tree: &ModuleTree) -> io::Result<()> {
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| io::Error::other(format!("begin replace_module_tree_snapshot tx: {e}")))?;
    replace_module_rows_tx(&tx, &module_tree.modules)?;
    tx.commit()
        .map_err(|e| io::Error::other(format!("commit replace_module_tree_snapshot tx: {e}")))
}

/// 列出当前 DB 内的模块快照视图。
pub fn list_modules_snapshot(repo_root: &Path) -> io::Result<Vec<ModuleRecord>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };

    list_modules_snapshot_in_conn(&conn)
}

fn list_modules_snapshot_in_conn(conn: &Connection) -> io::Result<Vec<ModuleRecord>> {
    if !table_exists(conn, "modules")? {
        return Ok(Vec::new());
    }

    let mut stmt = conn
        .prepare(
            "SELECT module_id, name, kind, root_paths, parent_id, child_ids, entry_points, tags
             FROM modules
             ORDER BY sort_order, module_id",
        )
        .map_err(|e| io::Error::other(format!("prepare list modules snapshot: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(ModuleRecord {
                module_id: row.get(0)?,
                name: row.get(1)?,
                kind: row.get(2)?,
                root_paths: parse_string_list(&row.get::<_, String>(3)?)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                parent_id: row.get(4)?,
                child_ids: parse_string_list(&row.get::<_, String>(5)?)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                entry_points: parse_string_list(&row.get::<_, String>(6)?)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                tags: parse_string_list(&row.get::<_, String>(7)?)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
            })
        })
        .map_err(|e| io::Error::other(format!("query list modules snapshot: {e}")))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect list modules snapshot: {e}")))
}

/// 列出当前 DB 内的 `module_source_map` 关联记录。
pub fn list_module_source_links(repo_root: &Path) -> io::Result<Vec<ModuleSourceLink>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };

    if !table_exists(&conn, "module_source_map")? {
        return Ok(Vec::new());
    }

    let mut stmt = conn
        .prepare(
            "SELECT module_id, source_id
             FROM module_source_map
             ORDER BY module_id, source_id",
        )
        .map_err(|e| io::Error::other(format!("prepare list module_source links: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(ModuleSourceLink {
                module_id: row.get(0)?,
                source_id: row.get(1)?,
            })
        })
        .map_err(|e| io::Error::other(format!("query list module_source links: {e}")))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect list module_source links: {e}")))
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
    if !table_exists(conn, "scan_cache")? {
        return Ok(None);
    }
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
            Ok(symbol_from_row_fields(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get::<_, i64>(4)? as usize,
                row.get::<_, i64>(5)? as usize,
                row.get::<_, i64>(6)? != 0,
                row.get(7)?,
            ))
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
            Ok(symbol_from_row_fields(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get::<_, i64>(4)? as usize,
                row.get::<_, i64>(5)? as usize,
                row.get::<_, i64>(6)? != 0,
                row.get(7)?,
            ))
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
        "SELECT s.id, s.name, s.label, s.file_path, COALESCE(s.start_line, 0), COALESCE(s.end_line, 0), COALESCE(s.language, ''), bm25(symbols_fts) AS score
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
            symbol: symbol_from_row_fields(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get::<_, i64>(4)? as usize,
                row.get::<_, i64>(5)? as usize,
                false,
                row.get(6)?,
            ),
            score: row.get(7)?,
        })
    }) {
        Ok(rows) => rows,
        Err(_) => return Ok(Vec::new()),
    };

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("collect symbols_fts hits: {e}")))
}

fn symbol_from_row_fields(
    symbol_id: String,
    name: String,
    label: String,
    file_path: String,
    start_line: usize,
    end_line: usize,
    is_exported: bool,
    language: String,
) -> SymbolNode {
    SymbolNode::legacy(
        symbol_id,
        name,
        label,
        file_path,
        start_line,
        end_line,
        is_exported,
        language,
    )
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

/// 清空 prompt 级 LLM 缓存。
pub fn clear_llm_cache(repo_root: &Path) -> io::Result<()> {
    let conn = open_db(repo_root)?;
    if !table_exists(&conn, "llm_cache")? {
        return Ok(());
    }
    conn.execute("DELETE FROM llm_cache", [])
        .map_err(|error| io::Error::other(format!("clear_llm_cache: {error}")))?;
    Ok(())
}

/// 读取全部 prompt 级 LLM 缓存，用于 runtime 清理前保留。
pub fn load_all_llm_cache(repo_root: &Path) -> io::Result<Vec<LlmCacheEntry>> {
    if !db_exists(repo_root) {
        return Ok(Vec::new());
    }
    let conn = match open_db_readonly(repo_root) {
        Ok(conn) => conn,
        Err(_) => return Ok(Vec::new()),
    };
    if !table_exists(&conn, "llm_cache")? {
        return Ok(Vec::new());
    }
    let mut stmt = conn
        .prepare(
            "SELECT input_hash, prompt_type, prompt_version, response, model, created_at, ttl_seconds
             FROM llm_cache",
        )
        .map_err(|error| io::Error::other(format!("load_all_llm_cache prepare: {error}")))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(LlmCacheEntry {
                input_hash: row.get(0)?,
                prompt_type: row.get(1)?,
                prompt_version: row.get(2)?,
                response: row.get(3)?,
                model: row.get(4)?,
                created_at: row.get(5)?,
                ttl_seconds: row.get(6)?,
            })
        })
        .map_err(|error| io::Error::other(format!("load_all_llm_cache query: {error}")))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| io::Error::other(format!("load_all_llm_cache rows: {error}")))
}

/// 恢复一组 prompt 级 LLM 缓存。
pub fn restore_llm_cache(repo_root: &Path, entries: &[LlmCacheEntry]) -> io::Result<()> {
    if entries.is_empty() {
        return Ok(());
    }
    let conn = open_db(repo_root)?;
    for entry in entries {
        write_llm_cache_in_conn(&conn, entry)?;
    }
    Ok(())
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

// ── knowledge_domains / knowledge_units CRUD ────────────────────────

pub fn write_knowledge_domains(
    conn: &Connection,
    domains: &[crate::domain::knowledge::KnowledgeDomain],
) -> io::Result<()> {
    for domain in domains {
        let evidence = serde_json::to_string(&domain.evidence).unwrap_or_default();
        let modules = serde_json::to_string(&domain.source_modules).unwrap_or_default();
        let files = serde_json::to_string(&domain.source_files).unwrap_or_default();
        conn.execute(
            "INSERT OR REPLACE INTO knowledge_domains
             (id, domain_type, label, evidence, source_modules, source_files)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                domain.id,
                format!("{:?}", domain.domain_type),
                domain.label,
                evidence,
                modules,
                files,
            ],
        )
        .map_err(|e| io::Error::other(format!("write_knowledge_domain({}): {e}", domain.id)))?;
    }
    Ok(())
}

pub fn write_knowledge_units(
    conn: &Connection,
    units: &[crate::domain::knowledge::KnowledgeUnit],
) -> io::Result<()> {
    let mut remaining = units.iter().cloned().collect::<Vec<_>>();
    let mut inserted = BTreeSet::new();

    while !remaining.is_empty() {
        let mut progressed = false;
        let mut deferred = Vec::new();

        for unit in remaining {
            if let Some(parent_unit_id) = &unit.parent_unit_id {
                if !inserted.contains(parent_unit_id) {
                    deferred.push(unit);
                    continue;
                }
            }

            let scope = serde_json::to_string(&unit.scope).unwrap_or_default();
            let domain_id = if unit.domain_id.is_empty()
                || matches!(
                    unit.unit_type,
                    crate::domain::knowledge::UnitType::Overview
                        | crate::domain::knowledge::UnitType::Architecture
                ) {
                None
            } else {
                Some(unit.domain_id.as_str())
            };
            conn.execute(
                "INSERT OR REPLACE INTO knowledge_units
                 (id, unit_type, title, domain_id, parent_unit_id, relative_path, scope, priority)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    unit.id,
                    format!("{:?}", unit.unit_type),
                    unit.title,
                    domain_id,
                    unit.parent_unit_id,
                    unit.relative_path,
                    scope,
                    unit.priority,
                ],
            )
            .map_err(|e| io::Error::other(format!("write_knowledge_unit({}): {e}", unit.id)))?;
            inserted.insert(unit.id);
            progressed = true;
        }

        if !progressed {
            let blocked = deferred
                .iter()
                .map(|unit| unit.id.clone())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(io::Error::other(format!(
                "write_knowledge_units: unresolved parent/domain dependencies for [{blocked}]"
            )));
        }

        remaining = deferred;
    }
    Ok(())
}

pub fn replace_knowledge_snapshot(
    conn: &Connection,
    domains: &[crate::domain::knowledge::KnowledgeDomain],
    units: &[crate::domain::knowledge::KnowledgeUnit],
) -> io::Result<()> {
    clear_unit_runtime_gates(conn)?;
    clear_page_digests(conn)?;
    clear_page_drafts(conn)?;
    conn.execute("DELETE FROM knowledge_units", [])
        .map_err(|e| io::Error::other(format!("clear knowledge_units: {e}")))?;
    conn.execute("DELETE FROM knowledge_domains", [])
        .map_err(|e| io::Error::other(format!("clear knowledge_domains: {e}")))?;
    write_knowledge_domains(conn, domains)?;
    write_knowledge_units(conn, units)
}

// ── research_cache CRUD ─────────────────────────────────────────────

pub fn write_research_cache(
    conn: &Connection,
    research_type: &str,
    target_id: &str,
    input_hash: &str,
    result_json: &str,
    model: Option<&str>,
) -> io::Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO research_cache
         (research_type, target_id, input_hash, result, model)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![research_type, target_id, input_hash, result_json, model],
    )
    .map_err(|e| {
        io::Error::other(format!(
            "write_research_cache({research_type}, {target_id}): {e}"
        ))
    })?;
    Ok(())
}

pub fn read_research_cache(
    conn: &Connection,
    research_type: &str,
    target_id: &str,
    input_hash: &str,
) -> io::Result<Option<String>> {
    if !table_exists(conn, "research_cache")? {
        return Ok(None);
    }
    let result: Option<String> = conn
        .query_row(
            "SELECT result FROM research_cache
             WHERE research_type = ?1 AND target_id = ?2 AND input_hash = ?3
               AND (ttl_seconds <= 0
                    OR datetime(created_at, '+' || ttl_seconds || ' seconds') > datetime('now'))",
            params![research_type, target_id, input_hash],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| {
            io::Error::other(format!(
                "read_research_cache({research_type}, {target_id}): {e}"
            ))
        })?;
    Ok(result)
}

pub fn clear_research_cache(conn: &Connection) -> io::Result<()> {
    if !table_exists(conn, "research_cache")? {
        return Ok(());
    }
    conn.execute("DELETE FROM research_cache", [])
        .map_err(|e| io::Error::other(format!("clear_research_cache: {e}")))?;
    Ok(())
}

// ── page_digests CRUD ───────────────────────────────────────────────

pub fn write_page_digest(
    conn: &Connection,
    unit_id: &str,
    digest_json: &str,
    content_hash: Option<&str>,
) -> io::Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO page_digests (unit_id, digest, content_hash) VALUES (?1, ?2, ?3)",
        params![unit_id, digest_json, content_hash],
    )
    .map_err(|e| io::Error::other(format!("write_page_digest({unit_id}): {e}")))?;
    Ok(())
}

pub fn read_page_digest(conn: &Connection, unit_id: &str) -> io::Result<Option<String>> {
    if !table_exists(conn, "page_digests")? {
        return Ok(None);
    }
    conn.query_row(
        "SELECT digest FROM page_digests WHERE unit_id = ?1",
        params![unit_id],
        |row| row.get(0),
    )
    .optional()
    .map_err(|e| io::Error::other(format!("read_page_digest({unit_id}): {e}")))
}

pub fn clear_page_digests(conn: &Connection) -> io::Result<()> {
    if !table_exists(conn, "page_digests")? {
        return Ok(());
    }
    conn.execute("DELETE FROM page_digests", [])
        .map_err(|e| io::Error::other(format!("clear_page_digests: {e}")))?;
    Ok(())
}

pub fn remove_page_digest(conn: &Connection, unit_id: &str) -> io::Result<()> {
    if !table_exists(conn, "page_digests")? {
        return Ok(());
    }
    conn.execute(
        "DELETE FROM page_digests WHERE unit_id = ?1",
        params![unit_id],
    )
    .map_err(|e| io::Error::other(format!("remove_page_digest({unit_id}): {e}")))?;
    Ok(())
}

// ── page_drafts CRUD ────────────────────────────────────────────────

pub fn write_page_draft(
    conn: &Connection,
    unit_id: &str,
    draft_json: &str,
    content_hash: Option<&str>,
) -> io::Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO page_drafts (unit_id, draft, content_hash) VALUES (?1, ?2, ?3)",
        params![unit_id, draft_json, content_hash],
    )
    .map_err(|e| io::Error::other(format!("write_page_draft({unit_id}): {e}")))?;
    Ok(())
}

pub fn read_page_draft(conn: &Connection, unit_id: &str) -> io::Result<Option<String>> {
    if !table_exists(conn, "page_drafts")? {
        return Ok(None);
    }
    conn.query_row(
        "SELECT draft FROM page_drafts WHERE unit_id = ?1",
        params![unit_id],
        |row| row.get(0),
    )
    .optional()
    .map_err(|e| io::Error::other(format!("read_page_draft({unit_id}): {e}")))
}

pub fn clear_page_drafts(conn: &Connection) -> io::Result<()> {
    if !table_exists(conn, "page_drafts")? {
        return Ok(());
    }
    conn.execute("DELETE FROM page_drafts", [])
        .map_err(|e| io::Error::other(format!("clear_page_drafts: {e}")))?;
    Ok(())
}

pub fn remove_page_draft(conn: &Connection, unit_id: &str) -> io::Result<()> {
    if !table_exists(conn, "page_drafts")? {
        return Ok(());
    }
    conn.execute(
        "DELETE FROM page_drafts WHERE unit_id = ?1",
        params![unit_id],
    )
    .map_err(|e| io::Error::other(format!("remove_page_draft({unit_id}): {e}")))?;
    Ok(())
}

// ── unit_runtime_gates CRUD ────────────────────────────────────────

pub fn write_unit_runtime_gate(conn: &Connection, gate: &UnitRuntimeGate) -> io::Result<()> {
    let missing_dependencies = serde_json::to_string(&gate.missing_dependencies)
        .map_err(|e| io::Error::other(format!("serialize unit_runtime_gate deps: {e}")))?;
    conn.execute(
        "INSERT OR REPLACE INTO unit_runtime_gates
         (unit_id, unit_type, research_status, compose_status, assemble_status,
          last_ready_stage, blocked_reason, missing_dependencies, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            gate.unit_id,
            gate.unit_type,
            gate.research_status,
            gate.compose_status,
            gate.assemble_status,
            gate.last_ready_stage,
            gate.blocked_reason,
            missing_dependencies,
            gate.updated_at,
        ],
    )
    .map_err(|e| io::Error::other(format!("write_unit_runtime_gate({}): {e}", gate.unit_id)))?;
    Ok(())
}

pub fn read_unit_runtime_gates(conn: &Connection) -> io::Result<Vec<UnitRuntimeGate>> {
    if !table_exists(conn, "unit_runtime_gates")? {
        return Ok(Vec::new());
    }
    let mut stmt = conn
        .prepare(
            "SELECT unit_id, unit_type, research_status, compose_status, assemble_status,
                    last_ready_stage, blocked_reason, missing_dependencies, updated_at
             FROM unit_runtime_gates
             ORDER BY unit_id",
        )
        .map_err(|e| io::Error::other(format!("prepare read_unit_runtime_gates: {e}")))?;
    let rows = stmt
        .query_map([], |row| {
            let missing_dependencies: String = row.get(7)?;
            let missing_dependencies =
                serde_json::from_str::<Vec<String>>(&missing_dependencies).unwrap_or_default();
            Ok(UnitRuntimeGate {
                unit_id: row.get(0)?,
                unit_type: row.get(1)?,
                research_status: row.get(2)?,
                compose_status: row.get(3)?,
                assemble_status: row.get(4)?,
                last_ready_stage: row.get(5)?,
                blocked_reason: row.get(6)?,
                missing_dependencies,
                updated_at: row.get(8)?,
            })
        })
        .map_err(|e| io::Error::other(format!("query read_unit_runtime_gates: {e}")))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(format!("rows read_unit_runtime_gates: {e}")))
}

pub fn clear_unit_runtime_gates(conn: &Connection) -> io::Result<()> {
    if !table_exists(conn, "unit_runtime_gates")? {
        return Ok(());
    }
    conn.execute("DELETE FROM unit_runtime_gates", [])
        .map_err(|e| io::Error::other(format!("clear_unit_runtime_gates: {e}")))?;
    Ok(())
}

// ── pipeline_checkpoint CRUD ────────────────────────────────────────

pub fn write_pipeline_checkpoint(
    conn: &Connection,
    checkpoint_id: &str,
    facts_input_hash: &str,
    interrupted_stage: &str,
    interrupted_target_id: Option<&str>,
    error_message: Option<&str>,
) -> io::Result<()> {
    conn.execute("DELETE FROM pipeline_checkpoint", [])
        .map_err(|e| io::Error::other(format!("clear checkpoint before write: {e}")))?;
    conn.execute(
        "INSERT INTO pipeline_checkpoint
         (checkpoint_id, facts_input_hash, interrupted_stage, interrupted_target_id, error_message)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            checkpoint_id,
            facts_input_hash,
            interrupted_stage,
            interrupted_target_id,
            error_message,
        ],
    )
    .map_err(|e| io::Error::other(format!("write_pipeline_checkpoint: {e}")))?;
    Ok(())
}

pub fn read_pipeline_checkpoint(
    conn: &Connection,
) -> io::Result<Option<(String, String, String, Option<String>, Option<String>)>> {
    if !table_exists(conn, "pipeline_checkpoint")? {
        return Ok(None);
    }
    conn.query_row(
        "SELECT checkpoint_id, facts_input_hash, interrupted_stage,
                interrupted_target_id, error_message
         FROM pipeline_checkpoint LIMIT 1",
        [],
        |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        },
    )
    .optional()
    .map_err(|e| io::Error::other(format!("read_pipeline_checkpoint: {e}")))
}

pub fn clear_pipeline_checkpoint(conn: &Connection) -> io::Result<()> {
    if !table_exists(conn, "pipeline_checkpoint")? {
        return Ok(());
    }
    conn.execute("DELETE FROM pipeline_checkpoint", [])
        .map_err(|e| io::Error::other(format!("clear_pipeline_checkpoint: {e}")))?;
    Ok(())
}
