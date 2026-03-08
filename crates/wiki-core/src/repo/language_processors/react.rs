use regex::Regex;

use super::{LanguageProcessor, collect_regex_targets, extract_first_quoted_target};

/// React 处理器主要覆盖 JSX/TSX 文件中的 import / require / dynamic import。
#[derive(Debug)]
pub struct ReactProcessor {
    import_regex: Regex,
}

impl ReactProcessor {
    /// 构建 React 处理器。
    pub fn new() -> Self {
        Self {
            import_regex: Regex::new(r#"(?m)^\s*(?:import|export)\s+(?:.*\s+from\s+)?['"]([^'"]+)['"]"#).unwrap(),
        }
    }
}

impl LanguageProcessor for ReactProcessor {
    fn supported_languages(&self) -> &'static [&'static str] {
        &["react"]
    }

    fn extract_dependency_targets(&self, content: &str) -> Vec<String> {
        let mut targets = collect_regex_targets(content, &self.import_regex);

        for line in content.lines() {
            let normalized = line.trim();
            if normalized.contains("require(") || normalized.contains("import(") {
                if let Some(target) = extract_first_quoted_target(normalized) {
                    targets.push(target);
                }
            }
        }

        targets
    }
}
