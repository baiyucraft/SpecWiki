use std::collections::BTreeSet;
use std::path::Path;

pub fn detect_topics(paths: &[String]) -> Vec<String> {
    let mut topics = BTreeSet::new();

    for path in paths {
        let normalized = Path::new(path)
            .to_string_lossy()
            .replace('\\', "/")
            .to_lowercase();

        if normalized.ends_with("package.json")
            || normalized.contains("src/")
            || normalized.ends_with("vite.config.ts")
        {
            topics.insert("frontend".to_string());
        }

        if normalized.ends_with("pyproject.toml")
            || normalized.ends_with("requirements.txt")
            || normalized.ends_with("app.py")
        {
            topics.insert("backend".to_string());
        }
    }

    topics.into_iter().collect()
}
