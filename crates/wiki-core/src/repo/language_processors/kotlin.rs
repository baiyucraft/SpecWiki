use regex::Regex;

use super::{LanguageProcessor, collect_regex_targets};

/// Kotlin 处理器主要依赖 package/import 两个结构信号。
#[derive(Debug)]
pub struct KotlinProcessor {
    import_regex: Regex,
    package_regex: Regex,
}

impl KotlinProcessor {
    /// 构建 Kotlin 处理器。
    pub fn new() -> Self {
        Self {
            import_regex: Regex::new(r"(?m)^\s*import\s+([^\s;]+)").unwrap(),
            package_regex: Regex::new(r"(?m)^\s*package\s+([^\s;]+)").unwrap(),
        }
    }
}

impl LanguageProcessor for KotlinProcessor {
    fn supported_languages(&self) -> &'static [&'static str] {
        &["kotlin"]
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
