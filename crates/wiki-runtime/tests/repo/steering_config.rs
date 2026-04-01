//! Steering 配置单元测试。
//! 覆盖新 `scan.ignore/include` schema、legacy ignore 兼容读取和默认值。

use std::fs;
use std::path::Path;
use std::sync::Mutex;

use tempfile::TempDir;
use wiki_runtime::domain::steering::{
    check_user_config_file, ensure_default_user_config_file, load_steering_config,
    load_steering_config_with_mode, spec_wiki_user_config_path, LlmProviderConfig,
    LlmProviderRequestFormat, SteeringConfig, SteeringLoadMode,
};

static HOME_ENV_LOCK: Mutex<()> = Mutex::new(());

struct EnvVarGuard {
    key: &'static str,
    previous: Option<std::ffi::OsString>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: &Path) -> Self {
        let previous = std::env::var_os(key);
        std::env::set_var(key, value.as_os_str());
        Self { key, previous }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        if let Some(previous) = self.previous.as_ref() {
            std::env::set_var(self.key, previous);
        } else {
            std::env::remove_var(self.key);
        }
    }
}

fn make_repo() -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join(".wiki")).unwrap();
    dir
}

fn write_steering(repo: &TempDir, content: &str) {
    fs::write(repo.path().join(".wiki").join("config.yaml"), content).unwrap();
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
    assert_eq!(config.llm.parallel_requests, 3);
    assert_eq!(config.llm.max_calls, 48);
    assert_eq!(config.llm.max_research_calls, 256);
    assert_eq!(config.llm.max_compose_calls, 160);
    assert_eq!(config.llm.page_research_max_turns, 10);
}

#[test]
fn priority_is_user_then_repo_by_default() {
    let _home_lock = HOME_ENV_LOCK.lock().unwrap();
    let home = TempDir::new().unwrap();
    let _home_guard = EnvVarGuard::set("HOME", home.path());
    let _userprofile_guard = EnvVarGuard::set("USERPROFILE", home.path());
    fs::create_dir_all(home.path().join(".spec-wiki")).unwrap();
    fs::write(
        home.path().join(".spec-wiki").join("config.yaml"),
        r#"
llm:
  enabled: false
  model: "user/default-model"
"#,
    )
    .unwrap();

    let repo = make_repo();
    write_steering(
        &repo,
        r#"
llm:
  enabled: true
  model: "repo/shared-model"
"#,
    );
    write_dev_config(
        &repo,
        r#"
llm:
  model: "dev/override-model"
"#,
    );

    let config = load_steering_config(repo.path());
    assert!(config.llm.enabled);
    assert_eq!(config.llm.model, "repo/shared-model");
}

#[test]
fn priority_is_user_then_repo_then_dev_when_dev_mode_enabled() {
    let _home_lock = HOME_ENV_LOCK.lock().unwrap();
    let home = TempDir::new().unwrap();
    let _home_guard = EnvVarGuard::set("HOME", home.path());
    let _userprofile_guard = EnvVarGuard::set("USERPROFILE", home.path());
    fs::create_dir_all(home.path().join(".spec-wiki")).unwrap();
    fs::write(
        home.path().join(".spec-wiki").join("config.yaml"),
        r#"
llm:
  enabled: false
  model: "user/default-model"
"#,
    )
    .unwrap();

    let repo = make_repo();
    write_steering(
        &repo,
        r#"
llm:
  enabled: true
  model: "repo/shared-model"
"#,
    );
    write_dev_config(
        &repo,
        r#"
llm:
  model: "dev/override-model"
"#,
    );

    let config = load_steering_config_with_mode(repo.path(), SteeringLoadMode::Development);
    assert!(config.llm.enabled);
    assert_eq!(config.llm.model, "dev/override-model");
}

#[test]
fn ensure_default_user_config_creates_template() {
    let _home_lock = HOME_ENV_LOCK.lock().unwrap();
    let home = TempDir::new().unwrap();
    let _home_guard = EnvVarGuard::set("HOME", home.path());
    let _userprofile_guard = EnvVarGuard::set("USERPROFILE", home.path());

    let path = ensure_default_user_config_file()
        .unwrap()
        .expect("user config path should be available");
    let content = fs::read_to_string(&path).unwrap();

    assert_eq!(
        path,
        spec_wiki_user_config_path().expect("expected user config path")
    );
    assert!(content.contains("debug: {}"));
    assert!(content.contains("llm: {}"));
}

#[test]
fn check_user_config_reports_invalid_yaml() {
    let _home_lock = HOME_ENV_LOCK.lock().unwrap();
    let home = TempDir::new().unwrap();
    let _home_guard = EnvVarGuard::set("HOME", home.path());
    let _userprofile_guard = EnvVarGuard::set("USERPROFILE", home.path());
    fs::create_dir_all(home.path().join(".spec-wiki")).unwrap();
    fs::write(
        home.path().join(".spec-wiki").join("config.yaml"),
        "{{{{not valid yaml",
    )
    .unwrap();

    let error = check_user_config_file().expect_err("invalid yaml should fail");
    assert!(error.to_string().contains("failed to parse"));
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
    assert_eq!(config.llm.parallel_requests, 3);
    assert_eq!(config.llm.max_calls, 48);
    assert_eq!(config.llm.max_research_calls, 256);
    assert_eq!(config.llm.max_compose_calls, 160);
    assert_eq!(config.llm.page_research_max_turns, 10);
}

#[test]
fn dev_config_overrides_shared_llm_provider_fields() {
    let _home_lock = HOME_ENV_LOCK.lock().unwrap();
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

    let config = load_steering_config_with_mode(repo.path(), SteeringLoadMode::Development);
    let selected = config.llm.resolve_selected_model().unwrap();

    assert!(config.llm.enabled);
    assert_eq!(config.llm.model, "dev/dev-model");
    assert_eq!(config.llm.parallel_requests, 5);
    assert_eq!(config.llm.max_calls, 9);
    assert!(!config.llm.allow_mermaid);
    assert_eq!(selected.provider_name, "dev");
    assert_eq!(selected.model_name, "dev-model");
    assert_eq!(selected.provider.api_base, "https://dev.example/v1/");
    assert_eq!(selected.provider.api_key_env, "DEV_PROVIDER_KEY");
    assert_eq!(selected.provider.timeout_seconds, 45);
    assert_eq!(selected.provider.max_retries, 3);
    assert_eq!(selected.provider.retry_backoff_ms, 400);
    assert_eq!(config.llm.provider_parallel_requests(), 5);
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
fn zero_max_research_calls_falls_back_to_default() {
    let repo = make_repo();
    write_steering(
        &repo,
        r#"
llm:
  enabled: true
  max_research_calls: 0
"#,
    );

    let config = load_steering_config(repo.path());

    assert_eq!(config.llm.max_research_calls, 256);
}

#[test]
fn zero_max_calls_and_page_turns_fall_back_to_defaults() {
    let repo = make_repo();
    write_steering(
        &repo,
        r#"
llm:
  enabled: true
  max_calls: 0
  page_research_max_turns: 0
"#,
    );

    let config = load_steering_config(repo.path());

    assert_eq!(config.llm.max_calls, 48);
    assert_eq!(config.llm.page_research_max_turns, 10);
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

    assert_eq!(config.llm.parallel_requests, 3);
    assert_eq!(config.llm.provider_parallel_requests(), 3);
}

#[test]
fn provider_max_retries_can_be_configured_and_zero_falls_back_to_default() {
    let _home_lock = HOME_ENV_LOCK.lock().unwrap();
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

    let config = load_steering_config_with_mode(repo.path(), SteeringLoadMode::Development);
    let selected = config.llm.resolve_selected_model().unwrap();

    assert_eq!(selected.provider.max_retries, 5);
}

#[test]
fn provider_retry_backoff_can_be_configured_and_zero_falls_back_to_default() {
    let _home_lock = HOME_ENV_LOCK.lock().unwrap();
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
      retry_backoff_ms: 0
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
      retry_backoff_ms: 250
"#,
    );

    let config = load_steering_config_with_mode(repo.path(), SteeringLoadMode::Development);
    let selected = config.llm.resolve_selected_model().unwrap();

    assert_eq!(selected.provider.retry_backoff_ms, 250);
}

#[test]
fn provider_request_format_defaults_to_chat_completions() {
    let config = LlmProviderConfig::default();
    let configured = LlmProviderConfig {
        api_base: "https://api.openai.com/v1".to_string(),
        ..LlmProviderConfig::default()
    };

    assert_eq!(
        config.request_format,
        LlmProviderRequestFormat::ChatCompletions
    );
    assert_eq!(config.endpoint_url(), None);
    assert_eq!(
        configured.endpoint_url(),
        Some("https://api.openai.com/v1/chat/completions".to_string())
    );
}

#[test]
fn provider_request_format_can_enable_responses_endpoint() {
    let repo = make_repo();
    write_steering(
        &repo,
        r#"
llm:
  enabled: true
  model: "proxy/provider-model"
  providers:
    proxy:
      api_base: "https://proxy.example/v1"
      request_format: responses
      models:
        provider-model: {}
"#,
    );

    let config = load_steering_config(repo.path());
    let selected = config.llm.resolve_selected_model().unwrap();

    assert_eq!(
        selected.provider.request_format,
        LlmProviderRequestFormat::Responses
    );
    assert_eq!(
        selected.provider.endpoint_url(),
        Some("https://proxy.example/v1/responses".to_string())
    );
}

#[test]
fn provider_request_format_preserves_explicit_endpoint_suffix() {
    let config = LlmProviderConfig {
        api_base: "https://proxy.example/v1/responses".to_string(),
        request_format: LlmProviderRequestFormat::ChatCompletions,
        ..LlmProviderConfig::default()
    };

    assert_eq!(
        config.endpoint_url(),
        Some("https://proxy.example/v1/responses".to_string())
    );
}

#[test]
fn dev_config_can_enable_debug_trace() {
    let _home_lock = HOME_ENV_LOCK.lock().unwrap();
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
  trace_dir: "logs/wiki-runtime"
  echo_to_stderr: true
"#,
    );

    let config = load_steering_config_with_mode(repo.path(), SteeringLoadMode::Development);

    assert!(config.debug.enabled);
    assert_eq!(config.debug.trace_dir, "logs/wiki-runtime");
    assert!(config.debug.echo_to_stderr);
}

fn sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values
}
