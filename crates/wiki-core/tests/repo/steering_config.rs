//! Steering 配置单元测试。
//! 覆盖新 `scan.ignore/include` schema、legacy ignore 兼容读取和默认值。

use std::fs;

use tempfile::TempDir;
use wiki_core::domain::steering::{load_steering_config, SteeringConfig};

fn make_repo() -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join(".wiki")).unwrap();
    dir
}

fn write_steering(repo: &TempDir, content: &str) {
    fs::write(
        repo.path().join(".wiki").join("wiki.steering.yaml"),
        content,
    )
    .unwrap();
}

#[test]
fn missing_file_returns_defaults() {
    let repo = make_repo();
    let config = load_steering_config(repo.path());

    assert_eq!(config.version, 1);
    assert_eq!(config.merge_threshold, 3);
    assert!(config.scan.ignore.is_empty());
    assert!(config.scan.include.is_empty());
    assert!(config.modules.promote.is_empty());
    assert!(config.modules.demote.is_empty());
}

#[test]
fn invalid_yaml_returns_defaults() {
    let repo = make_repo();
    write_steering(&repo, "{{{{not valid yaml");

    let config = load_steering_config(repo.path());
    assert_eq!(config.merge_threshold, 3);
    assert!(config.scan.ignore.is_empty());
}

#[test]
fn full_config_parses_new_scan_schema() {
    let repo = make_repo();
    write_steering(
        &repo,
        r#"
version: 1
scan:
  ignore:
    - "docs/**"
    - "examples/**"
  include:
    - "docs/keep.md"
modules:
  promote:
    - path: "internal/core"
      reason: "核心业务"
  demote:
    - path: "scripts"
      reason: "辅助脚本"
pages:
  priority:
    - path: "src/domain"
      boost: 2
  hints:
    - page_type: "overview"
      hint: "微服务架构"
merge_threshold: 5
"#,
    );

    let config = load_steering_config(repo.path());
    assert_eq!(config.version, 1);
    assert_eq!(config.merge_threshold, 5);
    assert_eq!(config.scan.ignore, vec!["docs/**", "examples/**"]);
    assert_eq!(config.scan.include, vec!["docs/keep.md"]);
    assert_eq!(config.modules.promote.len(), 1);
    assert_eq!(config.modules.promote[0].path, "internal/core");
    assert_eq!(config.modules.demote.len(), 1);
    assert_eq!(config.modules.demote[0].path, "scripts");
    assert_eq!(config.pages.priority.len(), 1);
    assert_eq!(config.pages.priority[0].boost, 2);
    assert_eq!(config.pages.hints.len(), 1);
}

#[test]
fn partial_config_fills_defaults() {
    let repo = make_repo();
    write_steering(&repo, "merge_threshold: 10\n");

    let config = load_steering_config(repo.path());
    assert_eq!(config.merge_threshold, 10);
    assert_eq!(config.version, 1);
    assert!(config.scan.ignore.is_empty());
    assert!(config.scan.include.is_empty());
    assert!(config.modules.promote.is_empty());
}

#[test]
fn effective_ignore_returns_scan_ignore() {
    let repo = make_repo();
    write_steering(
        &repo,
        r#"
scan:
  ignore:
    - "docs/**"
    - "target/**"
"#,
    );

    let config = load_steering_config(repo.path());
    let paths = config.effective_ignore_paths(Some("rust"));
    assert_eq!(paths, vec!["docs/**", "target/**"]);
}

#[test]
fn legacy_ignore_schema_migrates_into_scan_ignore() {
    let repo = make_repo();
    write_steering(
        &repo,
        r#"
ignore:
  global:
    - "docs/**"
  rust:
    - "target/**"
  python:
    - ".venv/**"
"#,
    );

    let config = load_steering_config(repo.path());
    assert_eq!(
        sorted(config.scan.ignore),
        sorted(vec![
            "docs/**".to_string(),
            "target/**".to_string(),
            ".venv/**".to_string(),
        ])
    );
    assert!(config.scan.include.is_empty());
}

#[test]
fn promote_demote_checks() {
    let repo = make_repo();
    write_steering(
        &repo,
        r#"
modules:
  promote:
    - path: "core"
  demote:
    - path: "scripts"
"#,
    );

    let config = load_steering_config(repo.path());
    assert!(config.is_promoted("core"));
    assert!(!config.is_promoted("scripts"));
    assert!(config.is_demoted("scripts"));
    assert!(!config.is_demoted("core"));
}

#[test]
fn default_config_is_sane() {
    let config = SteeringConfig::default();
    assert_eq!(config.version, 1);
    assert_eq!(config.merge_threshold, 3);
    assert!(config.scan.ignore.is_empty());
    assert!(config.scan.include.is_empty());
    assert!(config.modules.promote.is_empty());
    assert!(config.modules.demote.is_empty());
    assert!(config.pages.priority.is_empty());
    assert!(config.pages.hints.is_empty());
}

fn sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values
}
