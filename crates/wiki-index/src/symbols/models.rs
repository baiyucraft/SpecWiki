use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use wiki_model::domain::stable_id::stable_id;

/// 源码范围引用，统一供 symbol、raw capture、unresolved ref 和 query source ref 使用。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct SourceRange {
    pub file_id: String,
    pub path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub start_column: usize,
    pub end_column: usize,
}

impl SourceRange {
    pub fn new(
        file_id: impl Into<String>,
        path: impl Into<String>,
        start_line: usize,
        end_line: usize,
        start_column: usize,
        end_column: usize,
    ) -> Self {
        Self {
            file_id: file_id.into(),
            path: path.into(),
            start_line,
            end_line,
            start_column,
            end_column,
        }
    }
}

/// symbol 事实的来源类别。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum SymbolSourceKind {
    Parser,
    Heuristic,
    Recovered,
    #[default]
    Unknown,
}

/// symbol 事实的解析来源和诊断。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct SymbolProvenance {
    pub parser_id: String,
    pub parser_version: String,
    pub source_kind: SymbolSourceKind,
    pub confidence: f64,
    pub diagnostics: Vec<String>,
}

/// graph pipeline 的可诊断阶段。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum GraphPhase {
    Scan,
    Structure,
    #[default]
    Parse,
    ResolveImports,
    ResolveCalls,
    ResolveHeritage,
    AnalyzeCommunities,
    AnalyzeProcesses,
    BuildFts,
}

/// raw capture 的类别。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum RawCaptureKind {
    #[default]
    Import,
    Call,
    Heritage,
}

/// unresolved reference 的类别。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ReferenceKind {
    #[default]
    Import,
    Call,
    Heritage,
}

/// raw capture 共享 envelope。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct RawCaptureBase {
    pub capture_id: String,
    pub file_id: String,
    pub language: String,
    pub capture_kind: RawCaptureKind,
    pub source_symbol_id: Option<String>,
    pub raw_text: String,
    pub target_hint: Option<String>,
    pub range: SourceRange,
    pub parser_id: String,
    pub parser_version: String,
    pub diagnostics: Vec<String>,
}

impl RawCaptureBase {
    pub fn new(
        capture_kind: RawCaptureKind,
        file_path: impl Into<String>,
        language: impl Into<String>,
        line: usize,
        raw_text: impl Into<String>,
        target_hint: Option<String>,
        source_symbol_id: Option<String>,
    ) -> Self {
        let file_path = file_path.into();
        let language = language.into();
        let raw_text = raw_text.into();
        let file_id = stable_id("file", &file_path);
        let capture_id = stable_id(
            "capture",
            format!("{:?}:{}:{}:{}", capture_kind, file_path, line, raw_text),
        );
        Self {
            capture_id,
            file_id: file_id.clone(),
            language,
            capture_kind,
            source_symbol_id,
            raw_text,
            target_hint,
            range: SourceRange::new(file_id, file_path, line, line, 0, 0),
            parser_id: "tree-sitter".to_string(),
            parser_version: env!("CARGO_PKG_VERSION").to_string(),
            diagnostics: Vec::new(),
        }
    }
}

/// resolver 未能闭合的引用事实。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct UnresolvedRef {
    pub unresolved_ref_id: String,
    pub capture_id: String,
    pub file_id: String,
    pub resolver_phase: GraphPhase,
    pub reference_kind: ReferenceKind,
    pub reference_name: String,
    pub target_hint: Option<String>,
    pub range: SourceRange,
    pub candidates: Vec<String>,
    pub reason: String,
    pub diagnostics: Vec<String>,
}

/// `SymbolNode` 是当前迭代真正持久化的符号事实。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SymbolNode {
    /// 稳定符号 ID，供 SQLite、query 和后续关系解析复用。
    pub symbol_id: String,
    /// 稳定文件 ID，绑定 repo-root-relative path。
    pub file_id: String,
    /// 符号展示名，也是 `SymbolTable` 的名称键。
    pub name: String,
    /// 定义标签，例如 `function`、`class`、`method`。保留为过渡读取字段。
    pub label: String,
    /// 正式 symbol kind 合同。
    pub symbol_kind: String,
    /// 可选限定名。
    pub qualified_name: Option<String>,
    /// 可选签名。
    pub signature: Option<String>,
    /// 可选文档字符串。
    pub docstring: Option<String>,
    /// 可选可见性。
    pub visibility: Option<String>,
    /// 所属 owner symbol。
    pub owner_symbol_id: Option<String>,
    /// 相对仓库根目录的源码路径。
    pub file_path: String,
    /// 定义起始行，便于 query 和后续关系定位。
    pub start_line: usize,
    /// 定义结束行。
    pub end_line: usize,
    /// 正式源码范围合同。
    pub range: SourceRange,
    /// 当前语言语义下是否可作为对外可见定义。
    pub is_exported: bool,
    /// parser registry 收口后的实际语言标签。
    pub language: String,
    /// 解析来源和诊断。
    pub provenance: SymbolProvenance,
}

impl SymbolNode {
    pub fn from_definition(
        file_id: impl Into<String>,
        language: impl Into<String>,
        symbol_kind: impl Into<String>,
        name: impl Into<String>,
        qualified_name: Option<&str>,
        range: SourceRange,
        provenance: SymbolProvenance,
    ) -> Self {
        let file_id = file_id.into();
        let language = language.into();
        let symbol_kind = symbol_kind.into();
        let name = name.into();
        let qualified_name = qualified_name.map(str::to_string);
        let identity_name = qualified_name.as_deref().unwrap_or(&name);
        let symbol_id = stable_id(
            "symbol",
            format!(
                "{}:{}:{}:{}:{}",
                range.path, symbol_kind, identity_name, range.start_line, range.end_line
            ),
        );

        Self {
            symbol_id,
            file_id,
            name,
            label: symbol_kind.clone(),
            symbol_kind,
            qualified_name,
            signature: None,
            docstring: None,
            visibility: None,
            owner_symbol_id: None,
            file_path: range.path.clone(),
            start_line: range.start_line,
            end_line: range.end_line,
            range,
            is_exported: false,
            language,
            provenance,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn legacy(
        symbol_id: String,
        name: String,
        label: String,
        file_path: String,
        start_line: usize,
        end_line: usize,
        is_exported: bool,
        language: String,
    ) -> Self {
        let file_id = stable_id("file", &file_path);
        let range = SourceRange::new(
            file_id.clone(),
            file_path.clone(),
            start_line,
            end_line,
            0,
            0,
        );
        Self {
            symbol_id,
            file_id,
            name,
            label: label.clone(),
            symbol_kind: label,
            qualified_name: None,
            signature: None,
            docstring: None,
            visibility: None,
            owner_symbol_id: None,
            file_path,
            start_line,
            end_line,
            range,
            is_exported,
            language,
            provenance: SymbolProvenance {
                parser_id: "tree-sitter".to_string(),
                parser_version: env!("CARGO_PKG_VERSION").to_string(),
                source_kind: SymbolSourceKind::Parser,
                confidence: 1.0,
                diagnostics: Vec::new(),
            },
        }
    }

    pub fn with_symbol_id(mut self, symbol_id: impl Into<String>) -> Self {
        self.symbol_id = symbol_id.into();
        self
    }

    pub fn with_exported(mut self, is_exported: bool) -> Self {
        self.is_exported = is_exported;
        self
    }

    /// symbols_fts 只需要简洁但稳定的检索文本。
    pub fn search_text(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.symbol_kind,
            self.name,
            self.qualified_name.as_deref().unwrap_or_default(),
            self.file_path,
            self.language
        )
    }
}

/// 原始 import capture。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct RawImportCapture {
    pub base: RawCaptureBase,
    /// 捕获所在文件。
    pub file_path: String,
    /// 原始 import 路径或模块文本。
    pub raw_path: String,
    /// import 的具体名称。
    pub imported_name: Option<String>,
    /// import alias。
    pub alias: Option<String>,
    /// import 出现的原始行号。
    pub line: usize,
    /// 解析时实际使用的底层语言标签。
    pub language: String,
    /// 若 import 位于某个符号定义内部，则记录该源符号 ID。
    pub source_symbol_id: Option<String>,
    /// 原始 import 片段文本，供 diagnostics 与 resolution reason 复用。
    pub source_text: String,
}

/// 原始 call capture。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct RawCallCapture {
    pub base: RawCaptureBase,
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
    /// 参数形态的轻量描述。
    pub argument_shape: Option<String>,
}

/// 原始 heritage capture。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct RawHeritageCapture {
    pub base: RawCaptureBase,
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
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
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
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_node_contract_preserves_identity_range_and_provenance() {
        let range = SourceRange {
            file_id: "file:src/service.ts".to_string(),
            path: "src/service.ts".to_string(),
            start_line: 3,
            end_line: 8,
            start_column: 1,
            end_column: 2,
        };
        let provenance = SymbolProvenance {
            parser_id: "tree-sitter-typescript".to_string(),
            parser_version: "test-version".to_string(),
            source_kind: SymbolSourceKind::Parser,
            confidence: 1.0,
            diagnostics: Vec::new(),
        };

        let symbol = SymbolNode::from_definition(
            "file:src/service.ts",
            "typescript",
            "method",
            "run",
            Some("PaymentService.run"),
            range.clone(),
            provenance.clone(),
        )
        .with_exported(true);

        assert_eq!(symbol.file_id, "file:src/service.ts");
        assert_eq!(symbol.symbol_kind, "method");
        assert_eq!(symbol.range, range);
        assert_eq!(symbol.provenance, provenance);
        assert!(symbol.search_text().contains("PaymentService.run"));

        let rebuilt = SymbolNode::from_definition(
            "file:src/service.ts",
            "typescript",
            "method",
            "run",
            Some("PaymentService.run"),
            range,
            provenance,
        );
        assert_eq!(rebuilt.symbol_id, symbol.symbol_id);
    }

    #[test]
    fn raw_capture_dtos_preserve_source_identity() {
        let range = SourceRange::new("file:src/service.ts", "src/service.ts", 1, 1, 0, 31);
        let base = RawCaptureBase {
            capture_id: "capture:import:src/service.ts:1".to_string(),
            file_id: "file:src/service.ts".to_string(),
            language: "typescript".to_string(),
            capture_kind: RawCaptureKind::Import,
            source_symbol_id: None,
            raw_text: "import { run } from './runner'".to_string(),
            target_hint: Some("./runner".to_string()),
            range,
            parser_id: "tree-sitter-typescript".to_string(),
            parser_version: "test-version".to_string(),
            diagnostics: Vec::new(),
        };
        let import = RawImportCapture {
            base: base.clone(),
            file_path: "src/service.ts".to_string(),
            raw_path: "./runner".to_string(),
            imported_name: Some("run".to_string()),
            alias: None,
            line: 1,
            language: "typescript".to_string(),
            source_symbol_id: None,
            source_text: "import { run } from './runner'".to_string(),
        };
        let unresolved = UnresolvedRef {
            unresolved_ref_id: "unresolved:capture:import:src/service.ts:1".to_string(),
            capture_id: base.capture_id.clone(),
            file_id: base.file_id.clone(),
            resolver_phase: GraphPhase::ResolveImports,
            reference_kind: ReferenceKind::Import,
            reference_name: "run".to_string(),
            target_hint: Some("./runner".to_string()),
            range: base.range.clone(),
            candidates: Vec::new(),
            reason: "target_not_found".to_string(),
            diagnostics: Vec::new(),
        };

        assert_eq!(import.base.capture_id, unresolved.capture_id);
        assert_eq!(unresolved.resolver_phase, GraphPhase::ResolveImports);
        assert_eq!(unresolved.reference_kind, ReferenceKind::Import);
    }
}
