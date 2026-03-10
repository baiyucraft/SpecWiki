use regex::Regex;

use super::{collect_regex_targets, LanguageProcessor};

/// Swift 处理器当前先提取模块 import。
#[derive(Debug)]
pub struct SwiftProcessor {
    import_regex: Regex,
}

impl SwiftProcessor {
    /// 构建 Swift 处理器。
    pub fn new() -> Self {
        Self {
            import_regex: Regex::new(r"(?m)^\s*import\s+([^\s]+)").unwrap(),
        }
    }
}

impl LanguageProcessor for SwiftProcessor {
    fn supported_languages(&self) -> &'static [&'static str] {
        &["swift"]
    }

    fn extract_dependency_targets(&self, content: &str) -> Vec<String> {
        collect_regex_targets(content, &self.import_regex)
    }
}
