use regex::Regex;

use super::{LanguageProcessor, collect_regex_targets};

/// C# 处理器先基于 `using` 和 `namespace` 建立最小依赖与别名信息。
#[derive(Debug)]
pub struct CSharpProcessor {
    using_regex: Regex,
    namespace_regex: Regex,
}

impl CSharpProcessor {
    /// 构建 C# 处理器。
    pub fn new() -> Self {
        Self {
            using_regex: Regex::new(r"(?m)^\s*using\s+([^;=]+);").unwrap(),
            namespace_regex: Regex::new(r"(?m)^\s*namespace\s+([^\s\{;]+)").unwrap(),
        }
    }
}

impl LanguageProcessor for CSharpProcessor {
    fn supported_languages(&self) -> &'static [&'static str] {
        &["csharp"]
    }

    fn extract_dependency_targets(&self, content: &str) -> Vec<String> {
        collect_regex_targets(content, &self.using_regex)
            .into_iter()
            .map(|target| target.replace('.', "/"))
            .collect()
    }

    fn extract_declared_aliases(&self, content: &str) -> Vec<String> {
        collect_regex_targets(content, &self.namespace_regex)
    }
}
