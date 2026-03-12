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

fn write_dev_config(repo: &TempDir, content: &str) {
    fs::write(repo.path().join("wiki.dev.yaml"), content).unwrap();
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
    assert!(!config.debug.enabled);
    assert!(config.debug.trace_dir.is_empty());
    assert!(!config.debug.echo_to_stderr);
    assert!(config.llm.providers.is_empty());
    assert_eq!(config.llm.page_enrichment_parallel_requests, 3);
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
    assert!(!config.debug.enabled);
    assert!(config.debug.trace_dir.is_empty());
    assert!(!config.debug.echo_to_stderr);
    assert!(config.llm.providers.is_empty());
    assert_eq!(config.llm.page_enrichment_parallel_requests, 3);
}

#[test]
fn dev_config_overrides_shared_llm_provider_fields() {
    let repo = make_repo();
    write_steering(
        &repo,
        r#"
llm:
  enabled: false
  model: "shared/shared-model"
  max_calls: 9
  parallel_requests: 2
  allow_mermaid: false
  providers:
    shared:
      api_base: "https://shared.example/v1"
      timeout_seconds: 30
      models:
        shared-model:
          model_id: "gpt-shared"
"#,
    );
    write_dev_config(
        &repo,
        r#"
llm:
  enabled: true
  model: "dev/dev-model"
  parallel_requests: 5
  providers:
    dev:
      api_base: "https://dev.example/v1/"
      api_key_env: "DEV_PROVIDER_KEY"
      timeout_seconds: 45
      models:
        dev-model:
          model_id: "gpt-dev"
"#,
    );

    let config = load_steering_config(repo.path());
    let selected = config.llm.resolve_selected_model().unwrap();

    assert!(config.llm.enabled);
    assert_eq!(config.llm.model, "dev/dev-model");
    assert_eq!(config.llm.max_calls, 9);
    assert_eq!(config.llm.page_enrichment_parallel_requests, 5);
    assert!(!config.llm.allow_mermaid);
    assert_eq!(selected.provider_name, "dev");
    assert_eq!(selected.model_name, "dev-model");
    assert_eq!(selected.provider.api_base, "https://dev.example/v1/");
    assert_eq!(selected.provider.api_key_env, "DEV_PROVIDER_KEY");
    assert_eq!(selected.provider.timeout_seconds, 45);
    assert_eq!(selected.provider.max_retries, 3);
    assert_eq!(
        selected.model.resolved_model_id(selected.model_name),
        "gpt-dev"
    );
}

#[test]
fn missing_dev_config_keeps_shared_llm_provider_fields() {
    let repo = make_repo();
    write_steering(
        &repo,
        r#"
llm:
  enabled: true
  model: "shared/shared-model"
  providers:
    shared:
      api_base: "https://shared.example/v1"
      api_key: "shared-key"
      models:
        shared-model: {}
"#,
    );

    let config = load_steering_config(repo.path());
    let selected = config.llm.resolve_selected_model().unwrap();

    assert!(config.llm.enabled);
    assert_eq!(config.llm.model, "shared/shared-model");
    assert_eq!(selected.provider.api_base, "https://shared.example/v1");
    assert_eq!(selected.provider.api_key, "shared-key");
    assert_eq!(
        selected.model.resolved_model_id(selected.model_name),
        "shared-model"
    );
}

#[test]
fn zero_parallel_requests_falls_back_to_default() {
    let repo = make_repo();
    write_steering(
        &repo,
        r#"
llm:
  enabled: true
  parallel_requests: 0
"#,
    );

    let config = load_steering_config(repo.path());

    assert_eq!(config.llm.page_enrichment_parallel_requests, 3);
}

#[test]
fn provider_max_retries_can_be_configured_and_zero_falls_back_to_default() {
    let repo = make_repo();
    write_steering(
        &repo,
        r#"
llm:
  enabled: true
  model: "shared/shared-model"
  providers:
    shared:
      api_base: "https://shared.example/v1"
      max_retries: 0
      models:
        shared-model: {}
"#,
    );
    write_dev_config(
        &repo,
        r#"
llm:
  providers:
    shared:
      max_retries: 5
"#,
    );

    let config = load_steering_config(repo.path());
    let selected = config.llm.resolve_selected_model().unwrap();

    assert_eq!(selected.provider.max_retries, 5);
}

#[test]
fn dev_config_can_enable_debug_trace() {
    let repo = make_repo();
    write_steering(
        &repo,
        r#"
debug:
  enabled: false
  trace_dir: ".debug/shared"
"#,
    );
    write_dev_config(
        &repo,
        r#"
debug:
  enabled: true
  trace_dir: "logs/wiki-core"
  echo_to_stderr: true
"#,
    );

    let config = load_steering_config(repo.path());

    assert!(config.debug.enabled);
    assert_eq!(config.debug.trace_dir, "logs/wiki-core");
    assert!(config.debug.echo_to_stderr);
}

fn sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values
}
