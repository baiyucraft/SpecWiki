use regex::Regex;

use super::{collect_regex_targets, extract_first_quoted_target, LanguageProcessor};

/// PHP 处理器覆盖 namespace、use 和 require/include 这几类最常见依赖。
#[derive(Debug)]
pub struct PhpProcessor {
    use_regex: Regex,
    namespace_regex: Regex,
}

impl PhpProcessor {
    /// 构建 PHP 处理器。
    pub fn new() -> Self {
        Self {
            use_regex: Regex::new(r"(?m)^\s*use\s+([^;]+);").unwrap(),
            namespace_regex: Regex::new(r"(?m)^\s*namespace\s+([^;]+);").unwrap(),
        }
    }
}

impl LanguageProcessor for PhpProcessor {
    fn supported_languages(&self) -> &'static [&'static str] {
        &["php"]
    }

    fn extract_dependency_targets(&self, content: &str) -> Vec<String> {
        let mut targets = collect_regex_targets(content, &self.use_regex)
            .into_iter()
            .map(|target| target.replace('\\', "/"))
            .collect::<Vec<_>>();

        for line in content.lines() {
            let normalized = line.trim();
            if normalized.starts_with("require")
                || normalized.starts_with("include")
                || normalized.starts_with("require_once")
                || normalized.starts_with("include_once")
            {
                if let Some(target) = extract_first_quoted_target(normalized) {
                    targets.push(target);
                }
            }
        }

        targets
    }

    fn extract_declared_aliases(&self, content: &str) -> Vec<String> {
        collect_regex_targets(content, &self.namespace_regex)
    }
}
