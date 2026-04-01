//! knowledge_store 聚合 knowledge planning / research / compose 的 SQLite 入口。

use std::io;

use rusqlite::Connection;
use wiki_knowledge::store::{KnowledgeArtifactStore, KnowledgeSnapshotStore};
use wiki_model::domain::knowledge::{KnowledgeDomain, KnowledgeUnit};

use crate::storage::sqlite_store;

/// `SqliteKnowledgeStore` 用于在 runtime 中实现 wiki-knowledge 的 store trait。
pub struct SqliteKnowledgeStore<'conn> {
    conn: &'conn Connection,
}

impl<'conn> SqliteKnowledgeStore<'conn> {
    /// 绑定当前 workflow 已打开的 SQLite 连接。
    pub fn new(conn: &'conn Connection) -> Self {
        Self { conn }
    }
}

impl KnowledgeSnapshotStore for SqliteKnowledgeStore<'_> {
    fn write_knowledge_domains(&self, domains: &[KnowledgeDomain]) -> io::Result<()> {
        sqlite_store::write_knowledge_domains(self.conn, domains)
    }

    fn write_knowledge_units(&self, units: &[KnowledgeUnit]) -> io::Result<()> {
        sqlite_store::write_knowledge_units(self.conn, units)
    }

    fn write_research_cache(
        &self,
        research_type: &str,
        target_id: &str,
        input_hash: &str,
        result: &str,
    ) -> io::Result<()> {
        sqlite_store::write_research_cache(
            self.conn,
            research_type,
            target_id,
            input_hash,
            result,
            None,
        )
    }

    fn read_research_cache(
        &self,
        research_type: &str,
        target_id: &str,
        input_hash: &str,
    ) -> io::Result<Option<String>> {
        sqlite_store::read_research_cache(self.conn, research_type, target_id, input_hash)
    }

    fn clear_research_cache(&self) -> io::Result<()> {
        sqlite_store::clear_research_cache(self.conn)
    }
}

impl KnowledgeArtifactStore for SqliteKnowledgeStore<'_> {
    fn write_page_digest(
        &self,
        unit_id: &str,
        digest: &str,
        content_hash: Option<&str>,
    ) -> io::Result<()> {
        sqlite_store::write_page_digest(self.conn, unit_id, digest, content_hash)
    }

    fn read_page_digest(&self, unit_id: &str) -> io::Result<Option<String>> {
        sqlite_store::read_page_digest(self.conn, unit_id)
    }

    fn clear_page_digests(&self) -> io::Result<()> {
        sqlite_store::clear_page_digests(self.conn)
    }

    fn write_page_draft(
        &self,
        unit_id: &str,
        draft: &str,
        content_hash: Option<&str>,
    ) -> io::Result<()> {
        sqlite_store::write_page_draft(self.conn, unit_id, draft, content_hash)
    }

    fn read_page_draft(&self, unit_id: &str) -> io::Result<Option<String>> {
        sqlite_store::read_page_draft(self.conn, unit_id)
    }

    fn clear_page_drafts(&self) -> io::Result<()> {
        sqlite_store::clear_page_drafts(self.conn)
    }
}
