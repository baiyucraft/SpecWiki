//! Python 语言处理器。
//! 它先用语法树圈定 import 语句，再把目标转换成统一的路径风格。

use tree_sitter::{Node, Parser};

use super::LanguageProcessor;

/// Python 处理器使用 tree-sitter 定位 import 语句，避免注释和普通字符串干扰。
#[derive(Debug)]
pub struct PythonProcessor;

impl PythonProcessor {
    /// 构建 Python 处理器。
    pub fn new() -> Self {
        Self
    }
}

impl LanguageProcessor for PythonProcessor {
    fn supported_languages(&self) -> &'static [&'static str] {
        &["python"]
    }

    fn extract_dependency_targets(&self, content: &str) -> Vec<String> {
        let Some(tree) = parse_python_tree(content) else {
            return Vec::new();
        };

        let mut targets = Vec::new();
        collect_python_targets(tree.root_node(), content.as_bytes(), &mut targets);
        targets
    }
}

fn parse_python_tree(content: &str) -> Option<tree_sitter::Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_python::language())
        .ok()?;
    parser.parse(content, None)
}

/// 递归遍历 Python 语法树，收集 `import` 与 `from ... import ...` 目标。
/// 这一步只提“依赖谁”，不尝试还原完整符号关系。
fn collect_python_targets(node: Node<'_>, source: &[u8], targets: &mut Vec<String>) {
    match node.kind() {
        "import_statement" => {
            if let Ok(text) = node.utf8_text(source) {
                if let Some(rest) = text.trim().strip_prefix("import ") {
                    for target in rest.split(',') {
                        let dependency = target
                            .split(" as ")
                            .next()
                            .map(str::trim)
                            .filter(|value| !value.is_empty());

                        if let Some(dependency) = dependency {
                            targets.push(dependency.replace('.', "/"));
                        }
                    }
                }
            }
        }
        "import_from_statement" => {
            if let Ok(text) = node.utf8_text(source) {
                if let Some(rest) = text.trim().strip_prefix("from ") {
                    if let Some(module_name) = rest.split(" import ").next() {
                        targets.push(normalize_python_import_target(module_name.trim()));
                    }
                }
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_python_targets(child, source, targets);
    }
}

/// 把 Python 的导入目标归一化成扫描层统一使用的路径风格。
/// 相对导入会保留 `./`、`../` 语义，绝对导入则统一改成 `/` 分隔。
fn normalize_python_import_target(module_name: &str) -> String {
    if let Some(stripped) = module_name.strip_prefix("..") {
        return format!("../{}", stripped.trim_start_matches('.').replace('.', "/"));
    }

    if let Some(stripped) = module_name.strip_prefix('.') {
        return format!("./{}", stripped.trim_start_matches('.').replace('.', "/"));
    }

    module_name.replace('.', "/")
}
