use regex::Regex;

use super::{collect_regex_targets, extract_script_blocks, LanguageProcessor};

/// Vue 处理器先提取 `<script>` 片段，再按 JS/TS 风格提 import。
#[derive(Debug)]
pub struct VueProcessor {
    import_regex: Regex,
}

impl VueProcessor {
    /// 构建 Vue 处理器。
    pub fn new() -> Self {
        Self {
            import_regex: Regex::new(
                r#"(?m)^\s*(?:import|export)\s+(?:.*\s+from\s+)?['"]([^'"]+)['"]"#,
            )
            .unwrap(),
        }
    }
}

impl LanguageProcessor for VueProcessor {
    fn supported_languages(&self) -> &'static [&'static str] {
        &["vue"]
    }

    fn extract_dependency_targets(&self, content: &str) -> Vec<String> {
        extract_script_blocks(content)
            .into_iter()
            .flat_map(|script| collect_regex_targets(&script, &self.import_regex))
            .collect()
    }
}

