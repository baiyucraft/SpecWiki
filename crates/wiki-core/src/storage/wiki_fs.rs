use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn write_page(repo_root: &Path, relative_path: &str, content: &str) -> io::Result<()> {
    let target = wiki_root(repo_root).join(relative_path);

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(target, content)
}

pub fn wiki_root(repo_root: &Path) -> PathBuf {
    repo_root.join(".wiki")
}

pub fn resolve_page_path(repo_root: &Path, page_path: &str) -> PathBuf {
    wiki_root(repo_root).join(page_path.trim_start_matches(".wiki/"))
}

pub fn page_exists(repo_root: &Path, page_path: &str) -> bool {
    resolve_page_path(repo_root, page_path).exists()
}

pub fn remove_runtime(repo_root: &Path) -> io::Result<()> {
    let wiki_root = wiki_root(repo_root);

    if wiki_root.exists() {
        fs::remove_dir_all(wiki_root)?;
    }

    Ok(())
}
