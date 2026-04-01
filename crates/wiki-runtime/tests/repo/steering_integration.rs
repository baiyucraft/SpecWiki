//! Steering 配置集成测试。
//! 验证 `scan.ignore/include`、legacy ignore 兼容读取、模块提升/降级和合并阈值。

use std::fs;

use tempfile::TempDir;

use wiki_index::scanner::{scan_repo, scan_repo_with_boundary};
use wiki_runtime::domain::steering::load_steering_config;

fn make_repo_with_steering(yaml: &str) -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"name":"test","version":"1.0.0"}"#,
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/index.ts"), "export const x = 1;\n").unwrap();
    fs::write(dir.path().join("src/app.ts"), "export function app() {}\n").unwrap();
    fs::write(dir.path().join("src/utils.ts"), "export function u() {}\n").unwrap();
    fs::write(dir.path().join("src/types.ts"), "export type T = string;\n").unwrap();

    fs::create_dir_all(dir.path().join("docs")).unwrap();
    fs::write(dir.path().join("docs/guide.md"), "# Guide\n").unwrap();
    fs::write(dir.path().join("docs/keep.md"), "# Keep\n").unwrap();
    fs::create_dir_all(dir.path().join("benchmarks")).unwrap();
    fs::write(dir.path().join("benchmarks/bench.ts"), "// bench\n").unwrap();

    fs::create_dir_all(dir.path().join("scripts")).unwrap();
    fs::write(dir.path().join("scripts/deploy.sh"), "#!/bin/bash\n").unwrap();

    fs::create_dir_all(dir.path().join(".wiki")).unwrap();
    fs::write(dir.path().join(".wiki/config.yaml"), yaml).unwrap();

    dir
}

#[test]
fn scan_ignore_excludes_directories() {
    let repo = make_repo_with_steering(
        r#"
version: 1
scan:
  ignore:
    - "benchmarks/**"
"#,
    );

    let steering = load_steering_config(repo.path());
    let (ignore_paths, include_paths) = steering.scan_boundary();
    let report = scan_repo_with_boundary(repo.path(), ignore_paths, include_paths).unwrap();

    assert!(
        !report
            .files
            .iter()
            .any(|f| f.path.starts_with("benchmarks/")),
        "benchmarks/ should be excluded by scan.ignore"
    );
    assert!(
        report.files.iter().any(|f| f.path.starts_with("src/")),
        "src/ should not be excluded"
    );
}

#[test]
fn scan_include_restores_ignored_paths() {
    let repo = make_repo_with_steering(
        r#"
version: 1
scan:
  ignore:
    - "docs/**"
  include:
    - "docs/keep.md"
"#,
    );

    let steering = load_steering_config(repo.path());
    let (ignore_paths, include_paths) = steering.scan_boundary();
    let report = scan_repo_with_boundary(repo.path(), ignore_paths, include_paths).unwrap();

    assert!(
        report.files.iter().any(|f| f.path == "docs/keep.md"),
        "scan.include should restore docs/keep.md"
    );
    assert!(
        !report.files.iter().any(|f| f.path == "docs/guide.md"),
        "docs/guide.md should remain excluded"
    );
}

#[test]
fn legacy_ignore_schema_is_still_respected() {
    let repo = make_repo_with_steering(
        r#"
version: 1
ignore:
  global:
    - "docs/**"
"#,
    );

    let steering = load_steering_config(repo.path());
    let (ignore_paths, include_paths) = steering.scan_boundary();
    let report = scan_repo_with_boundary(repo.path(), ignore_paths, include_paths).unwrap();

    assert!(
        !report.files.iter().any(|f| f.path.starts_with("docs/")),
        "legacy ignore.global should be normalized into scan.ignore"
    );
}

#[test]
fn builtin_ignore_rules_still_apply() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"name":"test","version":"1.0.0"}"#,
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/index.ts"), "export const x = 1;\n").unwrap();
    fs::create_dir_all(dir.path().join(".next/cache")).unwrap();
    fs::write(dir.path().join(".next/cache/data.json"), "{}").unwrap();
    fs::create_dir_all(dir.path().join("__pycache__")).unwrap();
    fs::write(dir.path().join("__pycache__/mod.pyc"), "bytes").unwrap();

    let report = scan_repo(dir.path(), &[]).unwrap();

    assert!(
        !report.files.iter().any(|f| f.path.contains(".next/")),
        ".next/ should be excluded by builtin rules"
    );
    assert!(
        !report.files.iter().any(|f| f.path.contains("__pycache__/")),
        "__pycache__/ should be excluded by builtin rules"
    );
}
