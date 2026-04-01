use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// `SymbolNode` 是当前迭代真正持久化的符号事实。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SymbolNode {
    /// 稳定符号 ID，供 SQLite、query 和后续关系解析复用。
    pub symbol_id: String,
    /// 符号展示名，也是 `SymbolTable` 的名称键。
    pub name: String,
    /// 定义标签，例如 `function`、`class`、`method`。
    pub label: String,
    /// 相对仓库根目录的源码路径。
    pub file_path: String,
    /// 定义起始行，便于 query 和后续关系定位。
    pub start_line: usize,
    /// 定义结束行。
    pub end_line: usize,
    /// 当前语言语义下是否可作为对外可见定义。
    pub is_exported: bool,
    /// parser registry 收口后的实际语言标签。
    pub language: String,
}

impl SymbolNode {
    /// symbols_fts 只需要简洁但稳定的检索文本。
    pub fn search_text(&self) -> String {
        format!(
            "{} {} {} {}",
            self.label, self.name, self.file_path, self.language
        )
    }
}

/// 原始 import capture 先保留模型，当前迭代暂不持久化。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct RawImportCapture {
    /// 捕获所在文件。
    pub file_path: String,
    /// 原始 import 路径或模块文本。
    pub raw_path: String,
    /// import 出现的原始行号。
    pub line: usize,
    /// 解析时实际使用的底层语言标签。
    pub language: String,
    /// 若 import 位于某个符号定义内部，则记录该源符号 ID。
    pub source_symbol_id: Option<String>,
    /// 原始 import 片段文本，供 diagnostics 与 resolution reason 复用。
    pub source_text: String,
}

/// 原始 call capture 先保留模型，当前迭代暂不持久化。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct RawCallCapture {
    /// 捕获所在文件。
    pub file_path: String,
    /// 被调用符号的原始名称。
    pub called_name: String,
    /// 调用发生行号。
    pub line: usize,
    /// 解析时实际使用的底层语言标签。
    pub language: String,
    /// 调用发生时所在的源符号 ID。
    pub source_symbol_id: Option<String>,
    /// 若调用带 receiver/member 语义，这里保留原始 receiver 文本。
    pub receiver_text: Option<String>,
    /// 原始调用片段文本。
    pub source_text: String,
}

/// 原始 heritage capture 先保留模型，当前迭代暂不持久化。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct RawHeritageCapture {
    /// 捕获所在文件。
    pub file_path: String,
    /// 关系出现的原始行号。
    pub line: usize,
    /// 解析时实际使用的底层语言标签。
    pub language: String,
    /// 关系拥有者，例如当前类或接口。
    pub owner_name: String,
    /// 关系拥有者对应的稳定 symbol ID。
    pub owner_symbol_id: Option<String>,
    /// 继承或实现目标。
    pub target_name: String,
    /// 关系类别，例如 extends / implements。
    pub relation_kind: String,
    /// 原始 heritage 片段文本。
    pub source_text: String,
}

/// 单文件符号解析结果。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ParsedFileSymbols {
    /// 当前结果对应的源码路径。
    pub file_path: String,
    /// 实际命中的 parser 语言。
    pub language: String,
    /// definition capture 提升后的稳定符号节点。
    pub symbols: Vec<SymbolNode>,
    /// 预留给后续迭代的 import 原始捕获。
    pub imports: Vec<RawImportCapture>,
    /// 预留给后续迭代的 call 原始捕获。
    pub calls: Vec<RawCallCapture>,
    /// 预留给后续迭代的 heritage 原始捕获。
    pub heritage: Vec<RawHeritageCapture>,
    /// fail-soft 诊断集合；当前文件失败不会阻断整个 workflow。
    pub diagnostics: Vec<SymbolParseDiagnostic>,
}

/// fail-soft 诊断，供 workflow 和测试查看。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct SymbolParseDiagnostic {
    /// 发生问题的源码路径。
    pub file_path: String,
    /// 出问题时尝试使用的语言标签。
    pub language: String,
    /// 诊断类别，例如 `parse_error`、`query_error`。
    pub kind: String,
    /// 面向日志和测试输出的可读错误文本。
    pub message: String,
}

/// 一轮扫描解析得到的完整 symbol snapshot。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ParsedSymbolsSnapshot {
    /// 按文件保存的解析结果，供增量更新按路径替换。
    pub files: BTreeMap<String, ParsedFileSymbols>,
    /// 当前轮次全部稳定符号节点。
    pub symbols: Vec<SymbolNode>,
    /// 本轮汇总诊断。
    pub diagnostics: Vec<SymbolParseDiagnostic>,
    /// 由 `symbols` 派生出的双索引查找表。
    pub symbol_table: SymbolTable,
}

/// `SymbolTable` 为后续跨文件查找保留双索引。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct SymbolTable {
    /// 文件内名称索引：`file_path -> name -> symbol_ids`。
    pub file_index: BTreeMap<String, BTreeMap<String, Vec<String>>>,
    /// 全局名称索引：`name -> symbol_ids`。
    pub global_index: BTreeMap<String, Vec<String>>,
}

impl SymbolTable {
    /// 基于稳定符号集合构建双索引，并对每个候选列表去重排序。
    pub fn from_symbols(symbols: &[SymbolNode]) -> Self {
        let mut table = Self::default();

        for symbol in symbols {
            table
                .file_index
                .entry(symbol.file_path.clone())
                .or_default()
                .entry(symbol.name.clone())
                .or_default()
                .push(symbol.symbol_id.clone());
            table
                .global_index
                .entry(symbol.name.clone())
                .or_default()
                .push(symbol.symbol_id.clone());
        }

        for file_symbols in table.file_index.values_mut() {
            for symbol_ids in file_symbols.values_mut() {
                symbol_ids.sort();
                symbol_ids.dedup();
            }
        }

        for symbol_ids in table.global_index.values_mut() {
            symbol_ids.sort();
            symbol_ids.dedup();
        }

        table
    }

    /// 返回同文件内同名定义候选。
    pub fn lookup_exact(&self, file_path: &str, name: &str) -> Vec<String> {
        self.file_index
            .get(file_path)
            .and_then(|symbols| symbols.get(name))
            .cloned()
            .unwrap_or_default()
    }

    /// 返回全局同名定义候选。
    pub fn lookup_global(&self, name: &str) -> Vec<String> {
        self.global_index.get(name).cloned().unwrap_or_default()
    }
}
