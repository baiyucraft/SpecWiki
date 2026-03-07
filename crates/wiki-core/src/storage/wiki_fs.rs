use std::fs;
use std::io;
use std::path::Path;

pub fn write_page(repo_root: &Path, relative_path: &str, content: &str) -> io::Result<()> {
    let target = repo_root.join(".wiki").join(relative_path);

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(target, content)
}
