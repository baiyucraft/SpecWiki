use std::io;

use serde::{Deserialize, Serialize};

use crate::scanner::FilePurpose;

/// 文件角色辅助判断输入。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilePurposeAssistInput {
    pub path: String,
    pub kind: String,
    pub language: String,
    pub file_size: usize,
    pub deterministic: String,
    pub preview: String,
}

/// 顶层目录晋升辅助判断输入。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopLevelPromotionAssistInput {
    pub root_path: String,
    pub score: usize,
    pub total_files: usize,
    pub source_files: usize,
    pub config_files: usize,
    pub entry_points: usize,
    pub has_subdirs: bool,
    pub languages: Vec<String>,
    pub tags: Vec<String>,
}

/// 模块类型辅助判断输入。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleKindAssistInput {
    pub root_path: String,
    pub tags: Vec<String>,
    pub explicit_root: bool,
    pub has_children: bool,
    pub has_main_entry: bool,
    pub has_app_source: bool,
    pub has_infra_files: bool,
    pub workspace_member: bool,
}

/// 低置信度跨模块关系辅助判断输入。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DependencyAssistInput {
    pub relation_type: String,
    pub source_path: String,
    pub target_path: String,
    pub source_module: String,
    pub target_module: String,
    pub confidence: String,
}

/// Facts/index 阶段可选的非确定性辅助合同。
pub trait FactsAssist {
    fn classify_file_purposes(
        &mut self,
        inputs: &[FilePurposeAssistInput],
    ) -> io::Result<Vec<Option<FilePurpose>>>;

    fn decide_top_level_promotions(
        &mut self,
        inputs: &[TopLevelPromotionAssistInput],
    ) -> io::Result<Vec<Option<bool>>>;

    fn classify_module_kind(
        &mut self,
        input: &ModuleKindAssistInput,
    ) -> io::Result<Option<String>>;

    fn keep_dependency_edges(
        &mut self,
        inputs: &[DependencyAssistInput],
    ) -> io::Result<Vec<Option<bool>>>;
}

