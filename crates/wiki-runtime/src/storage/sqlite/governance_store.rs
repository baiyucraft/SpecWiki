//! governance_store 为只读治理内核保存可重建的 SQLite 派生快照。
//! `.spec` 始终是 evidence truth；本模块只负责 fingerprint 绑定的查询加速数据。

use std::io;

use rusqlite::Connection;
use serde_json::Value;

use crate::storage::sqlite_store::{self, GovernanceCacheRecord};

/// 一次 governance refresh 产生的完整可重建缓存快照。
#[derive(Debug, Clone, PartialEq)]
pub struct GovernanceCacheSnapshot {
    /// 缓存表结构版本。
    pub schema_version: String,
    /// 生成本快照的治理规则版本。
    pub policy_version: String,
    /// 当前 `.spec` evidence 的内容指纹。
    pub evidence_fingerprint: String,
    /// Repo 级治理摘要，不包含 artifact 正文。
    pub summary: Value,
    /// 稳定排序后的 change summaries。
    pub changes: Vec<Value>,
    /// 稳定排序后的结构化 artifact refs。
    pub artifact_refs: Vec<Value>,
    /// 稳定排序后的治理诊断。
    pub issues: Vec<Value>,
}

/// governance SQLite wrapper；所有快照替换必须在单个事务中完成。
pub struct SqliteGovernanceCache<'conn> {
    conn: &'conn mut Connection,
}

impl<'conn> SqliteGovernanceCache<'conn> {
    /// 绑定已有 SQLite connection。
    pub fn new(conn: &'conn mut Connection) -> Self {
        Self { conn }
    }

    /// 原子替换当前 governance derived snapshot。
    pub fn replace_snapshot(&mut self, snapshot: &GovernanceCacheSnapshot) -> io::Result<()> {
        let record = GovernanceCacheRecord {
            schema_version: snapshot.schema_version.clone(),
            policy_version: snapshot.policy_version.clone(),
            evidence_fingerprint: snapshot.evidence_fingerprint.clone(),
            summary_json: serde_json::to_string(&snapshot.summary).map_err(|error| {
                io::Error::other(format!("serialize governance summary: {error}"))
            })?,
            change_jsons: serialize_values("change", &snapshot.changes)?,
            artifact_ref_jsons: serialize_values("artifact ref", &snapshot.artifact_refs)?,
            issue_jsons: serialize_values("issue", &snapshot.issues)?,
        };
        sqlite_store::replace_governance_cache(self.conn, &record)
    }

    /// 清除可重建 governance cache；不会触碰其它 runtime tables。
    pub fn clear(&mut self) -> io::Result<()> {
        sqlite_store::clear_governance_cache(self.conn)
    }

    /// 仅在 fingerprint、schema 和 policy 版本全部匹配时读取快照。
    pub fn read_snapshot(
        &mut self,
        expected_fingerprint: &str,
        schema_version: &str,
        policy_version: &str,
    ) -> io::Result<Option<GovernanceCacheSnapshot>> {
        let Some(record) = sqlite_store::read_governance_cache(
            self.conn,
            expected_fingerprint,
            schema_version,
            policy_version,
        )?
        else {
            return Ok(None);
        };

        Ok(Some(GovernanceCacheSnapshot {
            schema_version: record.schema_version,
            policy_version: record.policy_version,
            evidence_fingerprint: record.evidence_fingerprint,
            summary: parse_value("summary", &record.summary_json)?,
            changes: parse_values("change", &record.change_jsons)?,
            artifact_refs: parse_values("artifact ref", &record.artifact_ref_jsons)?,
            issues: parse_values("issue", &record.issue_jsons)?,
        }))
    }
}

fn serialize_values(kind: &str, values: &[Value]) -> io::Result<Vec<String>> {
    values
        .iter()
        .map(|value| {
            serde_json::to_string(value)
                .map_err(|error| io::Error::other(format!("serialize governance {kind}: {error}")))
        })
        .collect()
}

fn parse_values(kind: &str, values: &[String]) -> io::Result<Vec<Value>> {
    values
        .iter()
        .map(|value| parse_value(kind, value))
        .collect()
}

fn parse_value(kind: &str, value: &str) -> io::Result<Value> {
    serde_json::from_str(value)
        .map_err(|error| io::Error::other(format!("parse governance {kind}: {error}")))
}
