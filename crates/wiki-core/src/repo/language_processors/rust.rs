//! Rust 语言处理器。
//! 这一层优先使用 `syn` 做语法级提取，避免继续靠字符串匹配 `use` 和 `mod`。

use syn::{Item, UseTree};

use super::LanguageProcessor;

/// Rust 处理器优先使用 `syn`，避免继续用字符串猜 `use` / `mod`。
#[derive(Debug)]
pub struct RustProcessor;

impl RustProcessor {
    /// 构建 Rust 处理器。
    pub fn new() -> Self {
        Self
    }
}

impl LanguageProcessor for RustProcessor {
    fn supported_languages(&self) -> &'static [&'static str] {
        &["rust"]
    }

    fn extract_dependency_targets(&self, content: &str) -> Vec<String> {
        let Ok(file) = syn::parse_file(content) else {
            return Vec::new();
        };

        let mut targets = Vec::new();
        collect_rust_targets_from_items(&file.items, &mut targets);
        targets
    }
}

fn collect_rust_targets_from_items(items: &[Item], targets: &mut Vec<String>) {
    for item in items {
        match item {
            Item::Use(item_use) => collect_use_tree_targets(String::new(), &item_use.tree, targets),
            Item::Mod(item_mod) => {
                targets.push(item_mod.ident.to_string());

                if let Some((_, nested_items)) = &item_mod.content {
                    collect_rust_targets_from_items(nested_items, targets);
                }
            }
            Item::ExternCrate(item_extern_crate) => {
                targets.push(item_extern_crate.ident.to_string());
            }
            _ => {}
        }
    }
}

/// 递归展开 Rust 的 `use` 树，把嵌套导入折叠成统一的路径字符串。
/// 例如 `use crate::a::{b, c};` 会被拆成 `crate/a/b`、`crate/a/c`。
fn collect_use_tree_targets(prefix: String, tree: &UseTree, targets: &mut Vec<String>) {
    match tree {
        UseTree::Path(path) => {
            let next_prefix = if prefix.is_empty() {
                path.ident.to_string()
            } else {
                format!("{prefix}/{}", path.ident)
            };
            collect_use_tree_targets(next_prefix, &path.tree, targets);
        }
        UseTree::Name(name) => {
            let full_target = if prefix.is_empty() {
                name.ident.to_string()
            } else {
                format!("{prefix}/{}", name.ident)
            };
            targets.push(full_target);
        }
        UseTree::Rename(rename) => {
            let full_target = if prefix.is_empty() {
                rename.ident.to_string()
            } else {
                format!("{prefix}/{}", rename.ident)
            };
            targets.push(full_target);
        }
        UseTree::Group(group) => {
            for item in &group.items {
                collect_use_tree_targets(prefix.clone(), item, targets);
            }
        }
        UseTree::Glob(_) => {
            if !prefix.is_empty() {
                targets.push(prefix);
            }
        }
    }
}
