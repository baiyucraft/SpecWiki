use std::collections::BTreeSet;
use std::path::Path;

/// 根据路径集合粗略推断仓库技术栈标签。
/// 这里的输出主要服务概述页和模块评分，不追求完整技术画像。
///
/// # 参数
/// - `paths`：当前仓库内的文件路径集合。
///
/// # 返回
/// - 返回按字典序稳定输出的技术栈标签列表。
pub fn detect_tech_hints(paths: &[String]) -> Vec<String> {
    let mut topics = BTreeSet::new();

    for path in paths {
        let normalized = Path::new(path)
            .to_string_lossy()
            .replace('\\', "/")
            .to_lowercase();

        if normalized.ends_with("package.json")
            || normalized.contains("src/")
            || normalized.ends_with("vite.config.ts")
            || normalized.ends_with(".vue")
            || normalized.ends_with(".svelte")
            || normalized.ends_with(".jsx")
            || normalized.ends_with(".tsx")
        {
            topics.insert("frontend".to_string());
        }

        if normalized.ends_with("pyproject.toml")
            || normalized.ends_with("requirements.txt")
            || normalized.ends_with("app.py")
            || normalized.ends_with(".py")
            || normalized.ends_with(".rs")
            || normalized.ends_with(".java")
            || normalized.ends_with(".kt")
            || normalized.ends_with(".cs")
            || normalized.ends_with(".php")
            || normalized.ends_with(".swift")
        {
            topics.insert("backend".to_string());
        }

        if normalized.ends_with("nginx.conf") || normalized.contains("/conf/") {
            topics.insert("infrastructure".to_string());
        }
    }

    topics.into_iter().collect()
}
