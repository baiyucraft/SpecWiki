//! JavaScript 语言处理器。
//! 这一层只负责从源码里提依赖目标，不负责更高层的模块或页面判断。

use tree_sitter::{Node, Parser};

use super::{extract_first_quoted_target, LanguageProcessor};

/// JavaScript 处理器使用 tree-sitter 先圈定依赖语法节点。
#[derive(Debug)]
pub struct JavaScriptProcessor;

impl JavaScriptProcessor {
    /// 构建 JavaScript 处理器。
    pub fn new() -> Self {
        Self
    }
}

impl LanguageProcessor for JavaScriptProcessor {
    fn supported_languages(&self) -> &'static [&'static str] {
        &["javascript"]
    }

    fn extract_dependency_targets(&self, content: &str) -> Vec<String> {
        let Some(tree) = parse_javascript_tree(content) else {
            return Vec::new();
        };

        let mut targets = Vec::new();
        collect_javascript_targets(tree.root_node(), content.as_bytes(), &mut targets);
        targets
    }
}

fn parse_javascript_tree(content: &str) -> Option<tree_sitter::Tree> {
    let mut parser = Parser::new();
    let language = tree_sitter_javascript::LANGUAGE.into();
    parser.set_language(&language).ok()?;
    parser.parse(content, None)
}

/// 递归遍历 JavaScript 语法树，收集 import/export/require/import() 目标。
/// 这里把“圈定语法节点”和“提取字符串目标”分开，便于后续继续扩语法覆盖面。
pub(crate) fn collect_javascript_targets(node: Node<'_>, source: &[u8], targets: &mut Vec<String>) {
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
        collect_javascript_targets(child, source, targets);
    }
}
