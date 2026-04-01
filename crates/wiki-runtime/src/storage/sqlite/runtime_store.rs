//! runtime_store 聚合 runtime projection / lifecycle 自己拥有的 SQLite 入口。

use std::io;

use rusqlite::Connection;

use crate::domain::checkpoint::{PipelineCheckpoint, PipelineStage, UnitRuntimeGate};
use crate::storage::sqlite_store;

/// `SqliteRuntimeStore` 用于包装 runtime 自有表的访问。
pub struct SqliteRuntimeStore<'conn> {
    conn: &'conn Connection,
}

impl<'conn> SqliteRuntimeStore<'conn> {
    /// 绑定当前 workflow 已打开的 SQLite 连接。
    pub fn new(conn: &'conn Connection) -> Self {
        Self { conn }
    }

    pub fn runtime_meta_get(&self, key: &str) -> io::Result<Option<String>> {
        sqlite_store::runtime_meta_get(self.conn, key)
    }

    pub fn runtime_meta_set(&self, key: &str, value: &str) -> io::Result<()> {
        sqlite_store::runtime_meta_set(self.conn, key, value)
    }

    pub fn write_unit_runtime_gate(&self, gate: &UnitRuntimeGate) -> io::Result<()> {
        sqlite_store::write_unit_runtime_gate(self.conn, gate)
    }

    pub fn read_unit_runtime_gates(&self) -> io::Result<Vec<UnitRuntimeGate>> {
        sqlite_store::read_unit_runtime_gates(self.conn)
    }

    pub fn clear_unit_runtime_gates(&self) -> io::Result<()> {
        sqlite_store::clear_unit_runtime_gates(self.conn)
    }

    pub fn write_pipeline_checkpoint(&self, checkpoint: &PipelineCheckpoint) -> io::Result<()> {
        sqlite_store::write_pipeline_checkpoint(
            self.conn,
            &checkpoint.checkpoint_id,
            &checkpoint.facts_input_hash,
            checkpoint.interrupted_stage.as_str(),
            checkpoint.interrupted_target_id.as_deref(),
            checkpoint.error_message.as_deref(),
        )
    }

    pub fn read_pipeline_checkpoint(&self) -> io::Result<Option<PipelineCheckpoint>> {
        let Some((
            checkpoint_id,
            facts_input_hash,
            interrupted_stage,
            interrupted_target_id,
            error_message,
        )) = sqlite_store::read_pipeline_checkpoint(self.conn)?
        else {
            return Ok(None);
        };
        let stage = PipelineStage::from_str(&interrupted_stage).ok_or_else(|| {
            io::Error::other(format!(
                "unknown pipeline stage in checkpoint: {interrupted_stage}"
            ))
        })?;
        Ok(Some(PipelineCheckpoint {
            checkpoint_id,
            facts_input_hash,
            interrupted_stage: stage,
            interrupted_target_id,
            error_message,
        }))
    }

    pub fn clear_pipeline_checkpoint(&self) -> io::Result<()> {
        sqlite_store::clear_pipeline_checkpoint(self.conn)
    }
}
