//! `store` 定义 knowledge 层自己的持久化合同。
//! 这些合同只描述知识快照和 compose 产物，不绑定具体 SQLite 实现。

use std::io;

use wiki_model::domain::knowledge::{KnowledgeDomain, KnowledgeUnit};

/// `KnowledgeSnapshotStore` 负责知识快照与 research workflow cache 的持久化。
pub trait KnowledgeSnapshotStore {
    /// 写入当前轮次确认的知识域快照。
    fn write_knowledge_domains(&self, domains: &[KnowledgeDomain]) -> io::Result<()>;
    /// 写入当前轮次确认的知识单元快照。
    fn write_knowledge_units(&self, units: &[KnowledgeUnit]) -> io::Result<()>;
    /// 写入 research 阶段的缓存结果。
    fn write_research_cache(
        &self,
        research_type: &str,
        target_id: &str,
        input_hash: &str,
        result: &str,
    ) -> io::Result<()>;
    /// 读取 research 阶段缓存结果。
    fn read_research_cache(
        &self,
        research_type: &str,
        target_id: &str,
        input_hash: &str,
    ) -> io::Result<Option<String>>;
    /// 清空 research 阶段缓存。
    fn clear_research_cache(&self) -> io::Result<()>;
}

/// `KnowledgeArtifactStore` 负责 compose 阶段产生的稳定产物缓存。
pub trait KnowledgeArtifactStore {
    /// 写入单个知识单元的 page digest。
    fn write_page_digest(
        &self,
        unit_id: &str,
        digest: &str,
        content_hash: Option<&str>,
    ) -> io::Result<()>;
    /// 读取单个知识单元的 page digest。
    fn read_page_digest(&self, unit_id: &str) -> io::Result<Option<String>>;
    /// 清空全部 page digest。
    fn clear_page_digests(&self) -> io::Result<()>;
    /// 写入单个知识单元的 page draft。
    fn write_page_draft(
        &self,
        unit_id: &str,
        draft: &str,
        content_hash: Option<&str>,
    ) -> io::Result<()>;
    /// 读取单个知识单元的 page draft。
    fn read_page_draft(&self, unit_id: &str) -> io::Result<Option<String>>;
    /// 清空全部 page draft。
    fn clear_page_drafts(&self) -> io::Result<()>;
}
