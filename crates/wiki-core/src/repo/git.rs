use std::fs;
use std::path::Path;

pub fn looks_like_git_repo(path: &Path) -> bool {
    path.join(".git").exists()
}

pub fn current_branch(path: &Path) -> String {
    let head = read_head(path);

    match head {
        Some(value) if value.starts_with("ref: ") => value
            .trim_start_matches("ref: ")
            .rsplit('/')
            .next()
            .unwrap_or("unknown")
            .to_string(),
        Some(_) => "detached".to_string(),
        None => "unknown".to_string(),
    }
}

pub fn current_commit(path: &Path) -> String {
    let Some(head) = read_head(path) else {
        return "unindexed".to_string();
    };

    if let Some(reference) = head.strip_prefix("ref: ") {
        let ref_path = path.join(".git").join(reference);
        return fs::read_to_string(ref_path)
            .map(|value| value.trim().to_string())
            .ok()
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "unindexed".to_string());
    }

    if head.trim().is_empty() {
        return "unindexed".to_string();
    }

    head.trim().to_string()
}

fn read_head(path: &Path) -> Option<String> {
    fs::read_to_string(path.join(".git/HEAD"))
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
