//! TypeScript 语言处理器。
//! 它和 JavaScript 共享同类目标，但使用 TypeScript 语法树避免 TS 特性漏判。

use tree_sitter::{Node, Parser};

use super::{extract_first_quoted_target, LanguageProcessor};

/// TypeScript 处理器和 JavaScript 一样先依赖语法树，但使用 TypeScript 语法。
#[derive(Debug)]
pub struct TypeScriptProcessor;

impl TypeScriptProcessor {
    /// 构建 TypeScript 处理器。
    pub fn new() -> Self {
        Self
    }
}

impl LanguageProcessor for TypeScriptProcessor {
    fn supported_languages(&self) -> &'static [&'static str] {
        &["typescript"]
    }

    fn extract_dependency_targets(&self, content: &str) -> Vec<String> {
        let Some(tree) = parse_typescript_tree(content) else {
            return Vec::new();
        };

        let mut targets = Vec::new();
        collect_typescript_targets(tree.root_node(), content.as_bytes(), &mut targets);
        targets
    }
}

fn parse_typescript_tree(content: &str) -> Option<tree_sitter::Tree> {
    let mut parser = Parser::new();
    let language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
    parser.set_language(&language).ok()?;
    parser.parse(content, None)
}

/// 递归遍历 TypeScript 语法树，收集 import/export/require/import() 目标。
/// 这里不做类型级分析，只提取模块依赖线索给扫描层使用。
fn collect_typescript_targets(node: Node<'_>, source: &[u8], targets: &mut Vec<String>) {
    match node.kind() {
        "import_statement" | "export_statement" => {
            if let Ok(text) = node.utf8_text(source) {
                if let Some(target) = extract_first_quoted_target(text) {
                    targets.push(target);
                }
            }
        }
        "call_expression" => {
            if let Ok(text) = node.utf8_text(source) {
                let normalized = text.trim();
                let is_supported_call = normalized.starts_with("require(")
                    || normalized.starts_with("import(")
                    || normalized.contains("=require(")
                    || normalized.contains("= require(");

                if is_supported_call {
                    if let Some(target) = extract_first_quoted_target(normalized) {
                        targets.push(target);
                    }
                }
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_typescript_targets(child, source, targets);
    }
}

