use regex::Regex;

use super::{LanguageProcessor, collect_regex_targets};

/// Java 处理器沿用 `deepwiki-rs` 当前代码里的思路，以 package/import 为主。
#[derive(Debug)]
pub struct JavaProcessor {
    import_regex: Regex,
    package_regex: Regex,
}

impl JavaProcessor {
    /// 构建 Java 处理器。
    pub fn new() -> Self {
        Self {
            import_regex: Regex::new(r"(?m)^\s*import\s+([^;]+);").unwrap(),
            package_regex: Regex::new(r"(?m)^\s*package\s+([^;]+);").unwrap(),
        }
    }
}

impl LanguageProcessor for JavaProcessor {
    fn supported_languages(&self) -> &'static [&'static str] {
        &["java"]
    }

    fn extract_dependency_targets(&self, content: &str) -> Vec<String> {
        collect_regex_targets(content, &self.import_regex)
            .into_iter()
            .map(|target| target.replace('.', "/"))
            .collect()
    }

    fn extract_declared_aliases(&self, content: &str) -> Vec<String> {
        collect_regex_targets(content, &self.package_regex)
    }
}
