use wiki_core::transport::dto::{CoreCommand, CoreResponse};

#[test]
fn parses_init_command() {
    let cmd: CoreCommand = serde_json::from_str(r#"{"action":"init","repoRoot":"."}"#).unwrap();

    assert_eq!(cmd.action, "init");
    assert_eq!(cmd.repo_root.as_deref(), Some("."));
}

#[test]
fn serializes_error_response() {
    let response = CoreResponse::error("not_git_repo");
    let json = serde_json::to_string(&response).unwrap();

    assert!(json.contains("not_git_repo"));
}

#[test]
fn handles_json_command_and_returns_error_payload() {
    let response =
        wiki_core::transport::json_rpc::handle_json(r#"{"action":"unknown-action"}"#).unwrap();

    assert!(!response.ok);
    assert_eq!(
        response.error.as_deref(),
        Some("unsupported_action:unknown-action")
    );
}
