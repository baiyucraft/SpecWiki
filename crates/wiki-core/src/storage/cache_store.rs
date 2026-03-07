use std::fs;
use std::io;
use std::path::Path;

pub fn ensure_cache_dir(repo_root: &Path) -> io::Result<()> {
    fs::create_dir_all(repo_root.join(".wiki/.cache"))
}
