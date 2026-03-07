use std::path::Path;

pub fn looks_like_git_repo(path: &Path) -> bool {
    path.join(".git").exists()
}
