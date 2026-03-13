use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use tempfile::tempdir;
use wiki_core::domain::stable_id::stable_id;
use wiki_core::domain::steering::SteeringConfig;
use wiki_core::domain::steering::{
    LlmCacheMode, LlmConfig, LlmProviderConfig, LlmProviderModelConfig,
};
use wiki_core::generation::context::{
    build_module_contexts, build_page_context, build_repo_context,
};
use wiki_core::generation::planner::plan_pages;
use wiki_core::llm::{
    FilePurposeAssistInput, LlmCompletion, LlmPromptRequest, LlmRuntime, LlmService,
    PageResearchInput, PageResearchRuntimeContext,
};
use wiki_core::repo::hierarchy::build_module_tree;
use wiki_core::repo::scanner::scan_repo;
use wiki_core::repo::symbol_graph::{GraphAnalysisSnapshot, ResolvedGraphSnapshot};
use wiki_core::repo::symbols::ParsedSymbolsSnapshot;
use wiki_core::storage::sqlite_store::{
    load_all_llm_cache, read_llm_cache, write_llm_cache, LlmCacheEntry,
};
use wiki_core::storage::wiki_fs::remove_runtime_with_cache_mode;
use wiki_core::workflows::init::run_init_with_progress_and_llm_as;
use wiki_core::workflows::progress::NoopProgressSink;

fn write_repo_file(repo_root: &Path, relative_path: &str, content: &str) {
    let path = repo_root.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

fn make_storybook_family_repo() -> tempfile::TempDir {
    let repo = tempdir().unwrap();
    write_repo_file(
        repo.path(),
        "package.json",
        r#"{"name":"storybook-like","private":true}"#,
    );
    write_repo_file(
        repo.path(),
        "code/frameworks/react-vite/src/index.ts",
        "export const reactVite = true;\n",
    );
    write_repo_file(
        repo.path(),
        "code/builders/builder-vite/src/index.ts",
        "export const builderVite = true;\n",
    );
    write_repo_file(
        repo.path(),
        "docs/get-started/index.md",
        "# Get Started\n",
    );
    write_repo_file(
        repo.path(),
        "docs/configure/index.md",
        "# Configure\n",
    );
    write_repo_file(
        repo.path(),
        "code/addons/a11y/package.json",
        r#"{"name":"@storybook/addon-a11y"}"#,
    );
    write_repo_file(
        repo.path(),
        "code/addons/a11y/src/index.ts",
        "export const addonA11y = true;\n",
    );
    write_repo_file(
        repo.path(),
        "code/addons/a11y/src/types.ts",
        "export type A11yOptions = { enabled: boolean };\n",
    );
    write_repo_file(
        repo.path(),
        "code/core/src/main.ts",
        "export const main = () => true;\n",
    );
    write_repo_file(
        repo.path(),
        "code/core/src/preview.ts",
        "export const preview = () => true;\n",
    );
    write_repo_file(repo.path(), "docs/addons/index.md", "# Addons\n");
    repo
}

fn llm_config(model: &str) -> LlmConfig {
    LlmConfig {
        enabled: true,
        model: model.to_string(),
        max_calls: 16,
        cache_ttl_seconds: 60 * 60,
        allow_mermaid: true,
        providers: BTreeMap::new(),
        ..LlmConfig::default()
    }
}

fn mock_completion(output: serde_json::Value, model: &str) -> LlmCompletion {
    LlmCompletion {
        output,
        model: Some(model.to_string()),
        usage: None,
    }
}

fn provider_config(api_base: &str, models: &[(&str, &str)]) -> LlmProviderConfig {
    let mut config = LlmProviderConfig {
        api_base: api_base.to_string(),
        ..LlmProviderConfig::default()
    };
    for (model_name, model_id) in models {
        config.models.insert(
            (*model_name).to_string(),
            LlmProviderModelConfig {
                model_id: (*model_id).to_string(),
            },
        );
    }
    config
}

#[test]
fn page_research_input_from_page_populates_basic_fields_for_family_page() {
    let repo = make_storybook_family_repo();
    let report = scan_repo(repo.path(), &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(
        &report,
        &tree,
        &repo_ctx,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    let family_page = pages
        .iter()
        .find(|page| page.page_type == "family-index")
        .expect("storybook family index page should exist");
    let page_context = build_page_context(family_page, &report, &tree, &repo_ctx, &mod_ctxs);
    let research_input = PageResearchInput::from_page(family_page, &page_context);

    assert_eq!(research_input.page_type, "family-index");
    assert!(!research_input.facts.is_empty());
}

#[derive(Default)]
struct BatchFilePurposeLlmService {
    calls: usize,
}

impl LlmService for BatchFilePurposeLlmService {
    fn request(&mut self, request: &LlmPromptRequest) -> io::Result<LlmCompletion> {
        self.calls += 1;

        let output = match request.prompt_type.as_str() {
            "file_purpose" => {
                let items = request
                    .input
                    .get("items")
                    .and_then(serde_json::Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                serde_json::json!({
                    "items": items.into_iter().filter_map(|item| {
                        let path = item.get("path")?.as_str()?;
                        let purpose = if path.contains("middleware") {
                            "实现 HTTP 请求日志记录的中间件"
                        } else if path.contains("helper") {
                            "提供字符串拼接辅助函数"
                        } else {
                            "提供处理器链式组合的核心工具函数"
                        };
                        Some(serde_json::json!({
                            "path": path,
                            "purpose": purpose,
                        }))
                    }).collect::<Vec<_>>()
                })
            }
            _ => serde_json::json!({}),
        };

        Ok(mock_completion(output, "batch-model"))
    }
}

#[derive(Default)]
struct BatchUncertaintyLlmService {
    calls: usize,
    prompt_types: Vec<String>,
}

impl LlmService for BatchUncertaintyLlmService {
    fn request(&mut self, request: &LlmPromptRequest) -> io::Result<LlmCompletion> {
        self.calls += 1;
        self.prompt_types.push(request.prompt_type.clone());
        let output = match request.prompt_type.as_str() {
            "top_level_promotion" => {
                let items = request
                    .input
                    .get("items")
                    .and_then(serde_json::Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                serde_json::json!({
                    "items": items.into_iter().filter_map(|item| {
                        let root_path = item.get("root_path")?.as_str()?;
                        Some(serde_json::json!({
                            "root_path": root_path,
                            "promote": root_path.contains("apps"),
                        }))
                    }).collect::<Vec<_>>()
                })
            }
            "dependency_edge" => {
                let items = request
                    .input
                    .get("items")
                    .and_then(serde_json::Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                serde_json::json!({
                    "items": items.into_iter().filter_map(|item| {
                        let relation_type = item.get("relation_type")?.as_str()?;
                        let source_path = item.get("source_path")?.as_str()?;
                        let target_path = item.get("target_path")?.as_str()?;
                        Some(serde_json::json!({
                            "key": format!("{relation_type}|{source_path}|{target_path}"),
                            "keep": !target_path.contains("discard"),
                        }))
                    }).collect::<Vec<_>>()
                })
            }
            _ => serde_json::json!({}),
        };

        Ok(mock_completion(output, "uncertainty-batch-model"))
    }
}

#[allow(dead_code)]
struct FakeProviderServer {
    api_base: String,
    calls: Arc<AtomicUsize>,
    active_calls: Arc<AtomicUsize>,
    max_active_calls: Arc<AtomicUsize>,
    response_delay: Duration,
    requests: Arc<Mutex<Vec<String>>>,
    request_instants: Arc<Mutex<Vec<Instant>>>,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

#[derive(Clone)]
struct FakeProviderHttpResponse {
    status_code: u16,
    body: String,
}

impl FakeProviderServer {
    fn start(content: &str) -> Self {
        Self::start_with_delay(content, Duration::ZERO)
    }

    fn start_with_delay(content: &str, response_delay: Duration) -> Self {
        Self::start_with_raw_responses(
            vec![serde_json::json!({
                "model": "provider-model",
                "choices": [
                    {
                        "message": {
                            "content": format!("```json\n{content}\n```")
                        }
                    }
                ]
            })
            .to_string()],
            response_delay,
        )
    }

    fn start_with_raw_responses(response_bodies: Vec<String>, response_delay: Duration) -> Self {
        let responses = response_bodies
            .into_iter()
            .map(|body| FakeProviderHttpResponse {
                status_code: 200,
                body,
            })
            .collect::<Vec<_>>();
        Self::start_with_http_responses(responses, response_delay)
    }

    fn start_with_http_responses(
        responses: Vec<FakeProviderHttpResponse>,
        response_delay: Duration,
    ) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let api_base = format!("http://{}/v1", listener.local_addr().unwrap());
        let calls = Arc::new(AtomicUsize::new(0));
        let active_calls = Arc::new(AtomicUsize::new(0));
        let max_active_calls = Arc::new(AtomicUsize::new(0));
        let requests = Arc::new(Mutex::new(Vec::new()));
        let request_instants = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let responses = Arc::new(responses);

        let calls_ref = Arc::clone(&calls);
        let active_calls_ref = Arc::clone(&active_calls);
        let max_active_calls_ref = Arc::clone(&max_active_calls);
        let responses_ref = Arc::clone(&responses);
        let requests_ref = Arc::clone(&requests);
        let request_instants_ref = Arc::clone(&request_instants);
        let stop_ref = Arc::clone(&stop);
        let response_delay_for_workers = response_delay;
        let handle = thread::spawn(move || {
            while !stop_ref.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let calls_ref = Arc::clone(&calls_ref);
                        let active_calls_ref = Arc::clone(&active_calls_ref);
                        let max_active_calls_ref = Arc::clone(&max_active_calls_ref);
                        let responses_ref = Arc::clone(&responses_ref);
                        let requests_ref = Arc::clone(&requests_ref);
                        let request_instants_ref = Arc::clone(&request_instants_ref);
                        thread::spawn(move || {
                            let active = active_calls_ref.fetch_add(1, Ordering::SeqCst) + 1;
                            update_max_concurrency(&max_active_calls_ref, active);
                            let body = read_http_request_body(&mut stream);
                            let call_index = calls_ref.fetch_add(1, Ordering::SeqCst);
                            requests_ref.lock().unwrap().push(body);
                            request_instants_ref.lock().unwrap().push(Instant::now());
                            if !response_delay_for_workers.is_zero() {
                                thread::sleep(response_delay_for_workers);
                            }
                            let response = responses_ref
                                .get(call_index)
                                .cloned()
                                .or_else(|| responses_ref.last().cloned())
                                .unwrap_or_else(|| FakeProviderHttpResponse {
                                    status_code: 200,
                                    body: serde_json::json!({
                                        "model": "provider-model",
                                        "choices": [
                                            {
                                                "message": {
                                                    "content": "```json\n{}\n```"
                                                }
                                            }
                                        ]
                                    })
                                    .to_string(),
                                });
                            write_http_response(&mut stream, response.status_code, &response.body);
                            active_calls_ref.fetch_sub(1, Ordering::SeqCst);
                        });
                    }
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(_) => break,
                }
            }
        });

        Self {
            api_base,
            calls,
            active_calls,
            max_active_calls,
            response_delay,
            requests,
            request_instants,
            stop,
            handle: Some(handle),
        }
    }

    fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }

    fn requests(&self) -> Vec<String> {
        self.requests.lock().unwrap().clone()
    }
}

impl Drop for FakeProviderServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = std::net::TcpStream::connect(
            self.api_base
                .trim_start_matches("http://")
                .trim_end_matches("/v1"),
        );
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        if !self.response_delay.is_zero() {
            thread::sleep(self.response_delay);
        }
        while self.active_calls.load(Ordering::SeqCst) > 0 {
            thread::sleep(Duration::from_millis(10));
        }
    }
}

fn update_max_concurrency(target: &AtomicUsize, current: usize) {
    let mut seen = target.load(Ordering::SeqCst);
    while current > seen {
        match target.compare_exchange(seen, current, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => break,
            Err(actual) => seen = actual,
        }
    }
}

fn read_http_request_body(stream: &mut std::net::TcpStream) -> String {
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 1024];
    let header_end;

    loop {
        let read = match stream.read(&mut chunk) {
            Ok(read) => read,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(10));
                continue;
            }
            Err(_) => return String::new(),
        };
        if read == 0 {
            return String::new();
        }
        buffer.extend_from_slice(&chunk[..read]);
        if let Some(index) = buffer.windows(4).position(|window| window == b"\r\n\r\n") {
            header_end = index + 4;
            break;
        }
    }

    let header_text = String::from_utf8_lossy(&buffer[..header_end]);
    let content_length = header_text
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().ok())
                .flatten()
        })
        .unwrap_or(0);

    while buffer.len() < header_end + content_length {
        let read = match stream.read(&mut chunk) {
            Ok(read) => read,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(10));
                continue;
            }
            Err(_) => break,
        };
        if read == 0 {
            break;
        }
        buffer.extend_from_slice(&chunk[..read]);
    }

    String::from_utf8_lossy(&buffer[header_end..header_end + content_length]).to_string()
}

fn write_http_response(stream: &mut std::net::TcpStream, status_code: u16, body: &str) {
    let reason = match status_code {
        200 => "OK",
        408 => "Request Timeout",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        _ => "Response",
    };
    let response = format!(
        "HTTP/1.1 {} {}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
        status_code,
        reason,
        body.len(),
        body
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[test]
fn llm_runtime_batches_file_purpose_requests() {
    let fixture = tempdir().unwrap();
    let config = llm_config("bridge/mock-model");
    let inputs = vec![
        FilePurposeAssistInput {
            path: "src/middleware/logger.go".to_string(),
            kind: "source".to_string(),
            language: "go".to_string(),
            file_size: 64,
            deterministic: "utility".to_string(),
            preview: "package middleware\nfunc Logger(next Handler) {}\n".to_string(),
        },
        FilePurposeAssistInput {
            path: "src/helper/strings.go".to_string(),
            kind: "source".to_string(),
            language: "go".to_string(),
            file_size: 48,
            deterministic: "utility".to_string(),
            preview: "package helper\nfunc Join(parts []string) string {}\n".to_string(),
        },
        FilePurposeAssistInput {
            path: "src/middleware/auth.go".to_string(),
            kind: "source".to_string(),
            language: "go".to_string(),
            file_size: 72,
            deterministic: "utility".to_string(),
            preview: "package middleware\nfunc RequireAuth(next Handler) {}\n".to_string(),
        },
    ];

    let mut service = BatchFilePurposeLlmService::default();
    let mut runtime = LlmRuntime::new(fixture.path(), &config, Some(&mut service));
    let results = runtime.classify_file_purposes(&inputs).unwrap();

    drop(runtime);
    assert_eq!(service.calls, 1);
    assert_eq!(
        results[0],
        Some(wiki_core::repo::scanner::FilePurpose::Middleware)
    );
    assert_eq!(
        results[1],
        Some(wiki_core::repo::scanner::FilePurpose::Helper)
    );
    assert_eq!(
        results[2],
        Some(wiki_core::repo::scanner::FilePurpose::Middleware)
    );
}

#[test]
fn llm_runtime_batches_top_level_promotion_requests() {
    let fixture = tempdir().unwrap();
    let config = llm_config("bridge/mock-model");
    let inputs = vec![
        wiki_core::llm::TopLevelPromotionAssistInput {
            root_path: "apps/web".to_string(),
            score: 2,
            total_files: 5,
            source_files: 3,
            config_files: 1,
            entry_points: 1,
            has_subdirs: true,
            languages: vec!["typescript".to_string()],
            tags: vec!["frontend".to_string()],
        },
        wiki_core::llm::TopLevelPromotionAssistInput {
            root_path: "docs".to_string(),
            score: 2,
            total_files: 3,
            source_files: 1,
            config_files: 0,
            entry_points: 0,
            has_subdirs: true,
            languages: vec!["markdown".to_string()],
            tags: vec![],
        },
    ];

    let mut service = BatchUncertaintyLlmService::default();
    let mut runtime = LlmRuntime::new(fixture.path(), &config, Some(&mut service));
    let results = runtime.decide_top_level_promotions(&inputs).unwrap();

    drop(runtime);
    assert_eq!(service.calls, 1);
    assert_eq!(results, vec![Some(true), Some(false)]);
}

#[test]
fn llm_runtime_batches_dependency_edge_reviews() {
    let fixture = tempdir().unwrap();
    let config = llm_config("bridge/mock-model");
    let inputs = vec![
        wiki_core::llm::DependencyAssistInput {
            relation_type: "DEPENDS_ON".to_string(),
            source_path: "apps/web/src/main.ts".to_string(),
            target_path: "packages/shared/src/util.ts".to_string(),
            source_module: "web".to_string(),
            target_module: "shared".to_string(),
            confidence: "heuristic".to_string(),
        },
        wiki_core::llm::DependencyAssistInput {
            relation_type: "DEPENDS_ON".to_string(),
            source_path: "apps/web/src/main.ts".to_string(),
            target_path: "discard/module.ts".to_string(),
            source_module: "web".to_string(),
            target_module: "discard".to_string(),
            confidence: "heuristic".to_string(),
        },
    ];

    let mut service = BatchUncertaintyLlmService::default();
    let mut runtime = LlmRuntime::new(fixture.path(), &config, Some(&mut service));
    let results = runtime.keep_dependency_edges(&inputs).unwrap();

    drop(runtime);
    assert_eq!(service.calls, 1);
    assert_eq!(results, vec![Some(true), Some(false)]);
}

#[test]
fn remove_runtime_preserves_llm_cache_in_preserve_mode() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    fs::create_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    let entry = LlmCacheEntry {
        input_hash: "hash-1".to_string(),
        prompt_type: "page_research".to_string(),
        prompt_version: "page-research/v1".to_string(),
        response: "{\"summary\":\"cached\"}".to_string(),
        model: Some("proxy/provider-model".to_string()),
        created_at: "2099-03-11T20:00:00Z".to_string(),
        ttl_seconds: 3_600,
    };
    write_llm_cache(repo_root, &entry).unwrap();

    remove_runtime_with_cache_mode(repo_root, LlmCacheMode::Preserve).unwrap();
    let preserved = read_llm_cache(
        repo_root,
        &entry.input_hash,
        &entry.prompt_type,
        &entry.prompt_version,
        entry.model.as_deref(),
    )
    .unwrap();
    assert!(preserved.is_some());

    remove_runtime_with_cache_mode(repo_root, LlmCacheMode::Clear).unwrap();
    let cleared = read_llm_cache(
        repo_root,
        &entry.input_hash,
        &entry.prompt_type,
        &entry.prompt_version,
        entry.model.as_deref(),
    )
    .unwrap();
    assert!(cleared.is_none());
}

#[test]
fn init_uses_provider_research_session_with_tools() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    let source_id = stable_id("source", "packages/app/src/index.ts");
    let server = FakeProviderServer::start_with_raw_responses(
        vec![
            serde_json::json!({
                "model": "provider-model",
                "choices": [
                    {
                        "message": {
                            "tool_calls": [
                                {
                                    "id": "call_001",
                                    "type": "function",
                                    "function": {
                                        "name": "read_source_snippets",
                                        "arguments": format!("{{\"source_ids\":[\"{source_id}\"]}}"),
                                    }
                                }
                            ]
                        }
                    }
                ],
                "usage": {
                    "prompt_tokens": 100,
                    "completion_tokens": 20,
                    "total_tokens": 120
                }
            })
            .to_string(),
            serde_json::json!({
                "model": "provider-model",
                "choices": [
                    {
                        "message": {
                            "content": "```json\n{\"summary\":\"research 摘要\",\"page_positioning\":\"该页定位为模块研究结果页。\",\"section_plan\":[{\"section_key\":\"module-intro\",\"section_title\":\"模块说明\",\"section_summary\":\"通过 tool 读取关键源码\",\"evidence_refs\":[],\"diagram_refs\":[],\"child_refs\":[]}],\"evidence_rollup\":[],\"diagram_rollup\":[],\"open_questions\":[]}\n```",
                            "tool_calls": null
                        }
                    }
                ],
                "usage": {
                    "prompt_tokens": 120,
                    "completion_tokens": 40,
                    "total_tokens": 160
                }
            })
            .to_string(),
        ],
        Duration::ZERO,
    );

    write_repo_file(
        repo_root,
        "package.json",
        r#"{"name":"research-demo","private":true,"workspaces":["packages/*"]}"#,
    );
    write_repo_file(
        repo_root,
        "packages/app/package.json",
        r#"{"name":"app","version":"1.0.0"}"#,
    );
    write_repo_file(
        repo_root,
        "packages/app/src/index.ts",
        "export function run() { return true; }\n",
    );
    let report = scan_repo(repo_root, &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(
        &report,
        &tree,
        &repo_ctx,
        &mod_ctxs,
        &SteeringConfig::default(),
    );
    let module_page = pages
        .iter()
        .find(|page| page.relative_path.ends_with("核心模块/packages/app.md"))
        .expect("module page should exist");
    let page_context = build_page_context(module_page, &report, &tree, &repo_ctx, &mod_ctxs);
    let research_input = PageResearchInput::from_page(module_page, &page_context);
    let symbol_snapshot = ParsedSymbolsSnapshot::default();
    let resolved_graph = ResolvedGraphSnapshot::default();
    let graph_analysis = GraphAnalysisSnapshot::default();
    let research_runtime = PageResearchRuntimeContext {
        page: module_page,
        page_context: &page_context,
        scan_report: &report,
        module_tree: &tree,
        repo_context: &repo_ctx,
        module_contexts: &mod_ctxs,
        symbol_snapshot: &symbol_snapshot,
        resolved_graph: &resolved_graph,
        graph_analysis: &graph_analysis,
    };
    let mut config = llm_config("proxy/provider-model");
    config.providers.insert(
        "proxy".to_string(),
        provider_config(&server.api_base, &[("provider-model", "provider-model")]),
    );
    let mut runtime = LlmRuntime::new(repo_root, &config, None);
    let output = runtime
        .research_page(&research_input, &research_runtime)
        .unwrap()
        .expect("research session should succeed");
    let requests = server.requests();
    let cache_entries = load_all_llm_cache(repo_root).unwrap();

    assert_eq!(output.result.summary, "research 摘要");
    assert!(output
        .result
        .section_plan
        .iter()
        .any(|item| item.section_summary.contains("通过 tool 读取关键源码")));
    assert!(server.calls() >= 2);
    assert!(!cache_entries.is_empty());
    assert!(requests.iter().any(|request| request.contains("\"tools\"")));
    assert!(requests
        .iter()
        .any(|request| request.contains("\"response_format\"")));
    assert!(requests
        .iter()
        .any(|request| request.contains("page_research")));
    assert!(output
        .session
        .recent_turns
        .iter()
        .any(|turn| turn.role == "assistant"));
    assert!(output
        .session
        .tool_artifact_refs
        .iter()
        .any(|artifact| artifact.tool_name == "read_source_snippets"));

    let first_request: serde_json::Value = serde_json::from_str(&requests[0]).unwrap();
    assert_eq!(
        first_request
            .get("response_format")
            .and_then(|value| value.get("json_schema"))
            .and_then(|value| value.get("schema"))
            .and_then(|value| value.get("additionalProperties"))
            .and_then(serde_json::Value::as_bool),
        Some(false)
    );
    assert_eq!(
        first_request
            .get("response_format")
            .and_then(|value| value.get("json_schema"))
            .and_then(|value| value.get("schema"))
            .and_then(|value| value.get("properties"))
            .and_then(|value| value.get("evidence_rollup"))
            .and_then(|value| value.get("items"))
            .and_then(|value| value.get("additionalProperties"))
            .and_then(serde_json::Value::as_bool),
        Some(false)
    );
    let mut nested_required = first_request
        .get("response_format")
        .and_then(|value| value.get("json_schema"))
        .and_then(|value| value.get("schema"))
        .and_then(|value| value.get("properties"))
        .and_then(|value| value.get("evidence_rollup"))
        .and_then(|value| value.get("items"))
        .and_then(|value| value.get("properties"))
        .and_then(|value| value.get("items"))
        .and_then(|value| value.get("items"))
        .and_then(|value| value.get("required"))
        .and_then(serde_json::Value::as_array)
        .map(|required| {
            required
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .expect("nested required should exist");
    nested_required.sort();
    assert_eq!(
        nested_required,
        vec![
            "coarse_span".to_string(),
            "end_line".to_string(),
            "evidence_type".to_string(),
            "note".to_string(),
            "path".to_string(),
            "section_refs".to_string(),
            "source_id".to_string(),
            "start_line".to_string(),
        ]
    );
    assert_eq!(
        first_request
            .get("response_format")
            .and_then(|value| value.get("json_schema"))
            .and_then(|value| value.get("schema"))
            .and_then(|value| value.get("properties"))
            .and_then(|value| value.get("evidence_rollup"))
            .and_then(|value| value.get("items"))
            .and_then(|value| value.get("properties"))
            .and_then(|value| value.get("items"))
            .and_then(|value| value.get("items"))
            .and_then(|value| value.get("properties"))
            .and_then(|value| value.get("source_id"))
            .and_then(|value| value.get("type"))
            .and_then(serde_json::Value::as_array)
            .map(|types| types
                .iter()
                .filter_map(serde_json::Value::as_str)
                .collect::<Vec<_>>()),
        Some(vec!["string", "null"])
    );
    assert_eq!(
        first_request
            .get("tools")
            .and_then(serde_json::Value::as_array)
            .and_then(|tools| tools.first())
            .and_then(|tool| tool.get("function"))
            .and_then(|value| value.get("parameters"))
            .and_then(|value| value.get("additionalProperties"))
            .and_then(serde_json::Value::as_bool),
        Some(false)
    );
}

#[test]
fn storybook_family_page_provider_research_drives_compose_rendering() {
    let repo = make_storybook_family_repo();
    let report = scan_repo(repo.path(), &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(
        &report,
        &tree,
        &repo_ctx,
        &mod_ctxs,
        &SteeringConfig::default(),
    );
    let family_page = pages
        .iter()
        .find(|page| page.page_type == "family-index")
        .expect("storybook family index page should exist");
    let page_context = build_page_context(family_page, &report, &tree, &repo_ctx, &mod_ctxs);
    let source_id = report
        .files
        .iter()
        .find(|file| file.path == "code/addons/a11y/src/index.ts")
        .map(|file| file.id.clone())
        .expect("addon source id should exist");
    let server = FakeProviderServer::start_with_raw_responses(
        vec![
            serde_json::json!({
                "model": "provider-model",
                "choices": [
                    {
                        "message": {
                            "tool_calls": [
                                {
                                    "id": "call_001",
                                    "type": "function",
                                    "function": {
                                        "name": "read_source_snippets",
                                        "arguments": format!("{{\"source_ids\":[\"{source_id}\"]}}"),
                                    }
                                }
                            ]
                        }
                    }
                ],
                "usage": {
                    "prompt_tokens": 120,
                    "completion_tokens": 20,
                    "total_tokens": 140
                }
            })
            .to_string(),
            serde_json::json!({
                "model": "provider-model",
                "choices": [
                    {
                        "message": {
                            "content": "```json\n{\"summary\":\"addon family research summary\",\"page_positioning\":\"该页负责收拢 addon 家族的公共入口、文档锚点和配置面。\",\"section_plan\":[{\"section_key\":\"family-intro\",\"section_title\":\"知识域概览\",\"section_summary\":\"先解释 addon 家族的职责和覆盖范围。\",\"evidence_refs\":[],\"diagram_refs\":[],\"child_refs\":[]},{\"section_key\":\"family-surfaces\",\"section_title\":\"Docs / API / 配置面\",\"section_summary\":\"再按 docs anchor、public API 和配置入口组织材料。\",\"evidence_refs\":[],\"diagram_refs\":[],\"child_refs\":[]}],\"evidence_rollup\":[],\"diagram_rollup\":[],\"open_questions\":[]}\n```",
                            "tool_calls": null
                        }
                    }
                ],
                "usage": {
                    "prompt_tokens": 160,
                    "completion_tokens": 60,
                    "total_tokens": 220
                }
            })
            .to_string(),
        ],
        Duration::ZERO,
    );
    let research_input = PageResearchInput::from_page(family_page, &page_context);
    let symbol_snapshot = ParsedSymbolsSnapshot::default();
    let resolved_graph = ResolvedGraphSnapshot::default();
    let graph_analysis = GraphAnalysisSnapshot::default();
    let research_runtime = PageResearchRuntimeContext {
        page: family_page,
        page_context: &page_context,
        scan_report: &report,
        module_tree: &tree,
        repo_context: &repo_ctx,
        module_contexts: &mod_ctxs,
        symbol_snapshot: &symbol_snapshot,
        resolved_graph: &resolved_graph,
        graph_analysis: &graph_analysis,
    };
    let mut config = llm_config("proxy/provider-model");
    config.providers.insert(
        "proxy".to_string(),
        provider_config(&server.api_base, &[("provider-model", "provider-model")]),
    );
    let mut runtime = LlmRuntime::new(repo.path(), &config, None);
    let output = runtime
        .research_page(&research_input, &research_runtime)
        .unwrap()
        .expect("family research should succeed");

    let requests = server.requests();

    assert_eq!(output.result.summary, "addon family research summary");
    assert!(output
        .result
        .page_positioning
        .contains("addon 家族的公共入口"));
    assert!(output
        .session
        .tool_artifact_refs
        .iter()
        .any(|artifact| artifact.tool_name == "read_source_snippets"));
    assert!(server.calls() >= 2);
    assert!(requests.iter().any(|request| request.contains("\"tools\"")));
    assert!(requests
        .iter()
        .any(|request| request.contains("\"response_format\"")));
}

#[test]
fn core_page_research_budget_is_reserved_before_non_core_pages() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    let generic_response = serde_json::json!({
        "model": "provider-model",
        "choices": [
            {
                "message": {
                    "content": "```json\n{\"summary\":\"research 摘要\",\"page_positioning\":\"该页定位为核心研究页。\",\"section_plan\":[],\"evidence_rollup\":[],\"diagram_rollup\":[],\"open_questions\":[]}\n```",
                    "tool_calls": null
                }
            }
        ],
        "usage": {
            "prompt_tokens": 60,
            "completion_tokens": 20,
            "total_tokens": 80
        }
    })
    .to_string();
    let server =
        FakeProviderServer::start_with_raw_responses(vec![generic_response; 6], Duration::ZERO);

    write_repo_file(
        repo_root,
        "package.json",
        r#"{"name":"budget-demo","private":true,"workspaces":["packages/*"]}"#,
    );
    write_repo_file(
        repo_root,
        "packages/app/package.json",
        r#"{"name":"app","version":"1.0.0"}"#,
    );
    write_repo_file(
        repo_root,
        "packages/app/src/index.ts",
        "export function run() { return true; }\n",
    );

    let report = scan_repo(repo_root, &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &SteeringConfig::default());
    let module_page = pages
        .iter()
        .find(|page| page.page_type == "module")
        .expect("module page should exist");
    let overview_page = pages
        .iter()
        .find(|page| page.page_type == "overview")
        .expect("overview page should exist");
    let architecture_page = pages
        .iter()
        .find(|page| page.page_type == "architecture")
        .expect("architecture page should exist");

    let module_context = build_page_context(module_page, &report, &tree, &repo_ctx, &mod_ctxs);
    let overview_context = build_page_context(overview_page, &report, &tree, &repo_ctx, &mod_ctxs);
    let architecture_context =
        build_page_context(architecture_page, &report, &tree, &repo_ctx, &mod_ctxs);
    let symbol_snapshot = ParsedSymbolsSnapshot::default();
    let resolved_graph = ResolvedGraphSnapshot::default();
    let graph_analysis = GraphAnalysisSnapshot::default();

    let module_runtime = PageResearchRuntimeContext {
        page: module_page,
        page_context: &module_context,
        scan_report: &report,
        module_tree: &tree,
        repo_context: &repo_ctx,
        module_contexts: &mod_ctxs,
        symbol_snapshot: &symbol_snapshot,
        resolved_graph: &resolved_graph,
        graph_analysis: &graph_analysis,
    };
    let overview_runtime = PageResearchRuntimeContext {
        page: overview_page,
        page_context: &overview_context,
        scan_report: &report,
        module_tree: &tree,
        repo_context: &repo_ctx,
        module_contexts: &mod_ctxs,
        symbol_snapshot: &symbol_snapshot,
        resolved_graph: &resolved_graph,
        graph_analysis: &graph_analysis,
    };
    let architecture_runtime = PageResearchRuntimeContext {
        page: architecture_page,
        page_context: &architecture_context,
        scan_report: &report,
        module_tree: &tree,
        repo_context: &repo_ctx,
        module_contexts: &mod_ctxs,
        symbol_snapshot: &symbol_snapshot,
        resolved_graph: &resolved_graph,
        graph_analysis: &graph_analysis,
    };

    let mut config = llm_config("proxy/provider-model");
    config.max_calls = 12;
    config.providers.insert(
        "proxy".to_string(),
        provider_config(&server.api_base, &[("provider-model", "provider-model")]),
    );
    let mut runtime = LlmRuntime::new(repo_root, &config, None);

    let module_input = PageResearchInput::from_page(module_page, &module_context);
    let overview_input = PageResearchInput::from_page(overview_page, &overview_context);
    let architecture_input = PageResearchInput::from_page(architecture_page, &architecture_context);

    assert!(runtime
        .research_page(&module_input, &module_runtime)
        .unwrap()
        .is_some());
    assert!(runtime
        .research_page(&module_input, &module_runtime)
        .unwrap()
        .is_some());
    assert!(runtime
        .research_page(&module_input, &module_runtime)
        .unwrap()
        .is_none());
    assert!(runtime
        .research_page(&overview_input, &overview_runtime)
        .unwrap()
        .is_some());
    assert!(runtime
        .research_page(&architecture_input, &architecture_runtime)
        .unwrap()
        .is_some());
    assert_eq!(server.calls(), 3);
}

#[test]
fn init_uses_wiki_dev_yaml_provider_without_agent_bridge() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    let server = FakeProviderServer::start(
        r#"{"summary":"provider 摘要","section_overrides":{"模块说明":"通过 wiki.dev.yaml 走 provider。"},"mermaid_blocks":{"依赖关系":"graph TD\nRepo-->Provider"}}"#,
    );

    write_repo_file(repo_root, "package.json", r#"{"name":"llm-dev-demo"}"#);
    write_repo_file(
        repo_root,
        "src/index.ts",
        "export function handleCheckout() { return true; }\n",
    );
    write_repo_file(
        repo_root,
        ".wiki/wiki.steering.yaml",
        concat!(
            "llm:\n",
            "  enabled: false\n",
            "  model: shared/shared-model\n",
            "  providers:\n",
            "    shared:\n",
            "      models:\n",
            "        shared-model: {}\n",
        ),
    );
    write_repo_file(
        repo_root,
        "wiki.dev.yaml",
        &format!(
            concat!(
                "llm:\n",
                "  enabled: true\n",
                "  model: proxy/provider-model\n",
                "  providers:\n",
                "    proxy:\n",
                "      api_base: \"{}\"\n",
                "      models:\n",
                "        provider-model:\n",
                "          model_id: provider-model\n",
            ),
            server.api_base
        ),
    );

    let mut sink = NoopProgressSink;
    let report = run_init_with_progress_and_llm_as("init", repo_root, &mut sink, None).unwrap();

    assert!(
        !report.generated_pages.is_empty(),
        "init with provider should generate pages"
    );
    let has_module_page = report
        .generated_pages
        .iter()
        .any(|path| !path.ends_with("项目概述.md") && !path.ends_with("系统架构.md"));
    assert!(has_module_page, "should generate at least one module-level page");
}

#[test]
fn invalid_page_research_output_is_negative_cached() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    let source_id = stable_id("source", "packages/app/src/index.ts");
    let server = FakeProviderServer::start_with_raw_responses(
        vec![
            serde_json::json!({
                "model": "provider-model",
                "choices": [
                    {
                        "message": {
                            "tool_calls": [
                                {
                                    "id": "call_001",
                                    "type": "function",
                                    "function": {
                                        "name": "read_source_snippets",
                                        "arguments": format!("{{\"source_ids\":[\"{source_id}\"]}}"),
                                    }
                                }
                            ]
                        }
                    }
                ]
            })
            .to_string(),
            serde_json::json!({
                "model": "provider-model",
                "choices": [
                    {
                        "message": {
                            "content": "provider returned invalid final content"
                        }
                    }
                ]
            })
            .to_string(),
        ],
        Duration::ZERO,
    );

    write_repo_file(
        repo_root,
        "package.json",
        r#"{"name":"research-demo","private":true,"workspaces":["packages/*"]}"#,
    );
    write_repo_file(
        repo_root,
        "packages/app/package.json",
        r#"{"name":"app","version":"1.0.0"}"#,
    );
    write_repo_file(
        repo_root,
        "packages/app/src/index.ts",
        "export function run() { return true; }\n",
    );
    let report = scan_repo(repo_root, &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(
        &report,
        &tree,
        &repo_ctx,
        &mod_ctxs,
        &SteeringConfig::default(),
    );
    let module_page = pages
        .iter()
        .find(|page| page.relative_path.ends_with("核心模块/packages/app.md"))
        .expect("module page should exist");
    let page_context = build_page_context(module_page, &report, &tree, &repo_ctx, &mod_ctxs);
    let research_input = PageResearchInput::from_page(module_page, &page_context);
    let symbol_snapshot = ParsedSymbolsSnapshot::default();
    let resolved_graph = ResolvedGraphSnapshot::default();
    let graph_analysis = GraphAnalysisSnapshot::default();
    let research_runtime = PageResearchRuntimeContext {
        page: module_page,
        page_context: &page_context,
        scan_report: &report,
        module_tree: &tree,
        repo_context: &repo_ctx,
        module_contexts: &mod_ctxs,
        symbol_snapshot: &symbol_snapshot,
        resolved_graph: &resolved_graph,
        graph_analysis: &graph_analysis,
    };
    let input_hash = wiki_core::llm::build_prompt_input_hash(
        wiki_core::llm::PromptType::PageResearch,
        Some("proxy/provider-model"),
        &serde_json::to_value(&research_input).unwrap(),
    );
    let mut config = llm_config("proxy/provider-model");
    config.providers.insert(
        "proxy".to_string(),
        provider_config(&server.api_base, &[("provider-model", "provider-model")]),
    );
    let mut runtime = LlmRuntime::new(repo_root, &config, None);

    let first = runtime
        .research_page(&research_input, &research_runtime)
        .unwrap();
    let cached = read_llm_cache(
        repo_root,
        &input_hash,
        "page_research",
        "page-research/v1",
        Some("proxy/provider-model"),
    )
    .unwrap()
    .expect("negative page research cache should exist");
    let second = runtime
        .research_page(&research_input, &research_runtime)
        .unwrap();

    assert!(first.is_none());
    assert!(second.is_none());
    assert!(cached.response.contains("\"_cache_status\":\"negative\""));
    assert_eq!(server.calls(), 2);
}
