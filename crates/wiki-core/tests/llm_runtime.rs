use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use tempfile::tempdir;
use wiki_core::domain::steering::SteeringConfig;
use wiki_core::domain::stable_id::stable_id;
use wiki_core::domain::steering::{
    LlmCacheMode, LlmConfig, LlmProviderConfig, LlmProviderModelConfig,
};
use wiki_core::generation::context::{build_module_contexts, build_page_context, build_repo_context};
use wiki_core::generation::planner::plan_pages;
use wiki_core::llm::{
    FilePurposeAssistInput, LlmCompletion, LlmPromptRequest, LlmRuntime, LlmService,
    PageEnrichmentInput, PageResearchInput, PageResearchRuntimeContext, SelectedLlmPath,
};
use wiki_core::repo::hierarchy::build_module_tree;
use wiki_core::repo::scanner::scan_repo;
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

fn sample_page_input() -> PageEnrichmentInput {
    PageEnrichmentInput {
        page_id: "page-1".to_string(),
        page_type: "module".to_string(),
        title: "模块：demo".to_string(),
        scope: "module:demo".to_string(),
        section_titles: vec![
            "模块说明".to_string(),
            "关键源码".to_string(),
            "依赖关系".to_string(),
            "模块事实".to_string(),
            "子模块概述".to_string(),
        ],
        facts: vec!["模块名称：demo".to_string()],
        summary_inputs: vec!["源码：src/index.ts".to_string()],
        hints: vec!["重点说明入口职责".to_string()],
        child_summaries: vec!["child summary".to_string()],
        evidence_groups: vec![],
        diagram_inputs: vec![],
        allow_mermaid: true,
    }
}

fn llm_config(model: &str) -> LlmConfig {
    LlmConfig {
        enabled: true,
        model: model.to_string(),
        max_calls: 16,
        page_enrichment_parallel_requests: 3,
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

enum FakeMode {
    ValidEnhancement,
    InvalidEnhancement,
}

struct FakeLlmService {
    calls: usize,
    mode: FakeMode,
}

impl FakeLlmService {
    fn valid() -> Self {
        Self {
            calls: 0,
            mode: FakeMode::ValidEnhancement,
        }
    }

    fn invalid() -> Self {
        Self {
            calls: 0,
            mode: FakeMode::InvalidEnhancement,
        }
    }
}

impl LlmService for FakeLlmService {
    fn request(&mut self, request: &LlmPromptRequest) -> io::Result<LlmCompletion> {
        self.calls += 1;

        let output = match self.mode {
            FakeMode::ValidEnhancement => serde_json::json!({
                "summary": "增强摘要",
                "section_overrides": {
                    "模块说明": "这是增强后的模块说明。"
                },
                "mermaid_blocks": {
                    "依赖关系": "graph TD\nA-->B"
                },
                "consumed_hints": ["重点说明入口职责"],
                "consumed_child_summaries": ["child summary"]
            }),
            FakeMode::InvalidEnhancement => serde_json::json!({
                "summary": "",
                "section_overrides": {},
                "mermaid_blocks": {
                    "模块说明": "sequenceDiagram\nA->>B: invalid"
                }
            }),
        };

        Ok(mock_completion(
            match request.prompt_type.as_str() {
                "page_enrichment" => output,
                _ => serde_json::json!({}),
            },
            "mock-model",
        ))
    }
}

#[derive(Default)]
struct BudgetTrackingLlmService {
    prompt_types: Vec<String>,
}

impl LlmService for BudgetTrackingLlmService {
    fn request(&mut self, request: &LlmPromptRequest) -> io::Result<LlmCompletion> {
        self.prompt_types.push(request.prompt_type.clone());
        let output = match request.prompt_type.as_str() {
            "file_purpose" => serde_json::json!({
                "purpose": "utility"
            }),
            "page_enrichment" => serde_json::json!({
                "summary": "budget summary",
                "section_overrides": {
                    "模块说明": "预算保留后仍然生成了模块说明。"
                },
                "mermaid_blocks": {}
            }),
            _ => serde_json::json!({}),
        };

        Ok(mock_completion(output, "budget-model"))
    }
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

#[derive(Default)]
struct InputCaptureLlmService {
    last_request: Option<LlmPromptRequest>,
}

impl LlmService for InputCaptureLlmService {
    fn request(&mut self, request: &LlmPromptRequest) -> io::Result<LlmCompletion> {
        self.last_request = Some(request.clone());
        Ok(mock_completion(
            serde_json::json!({
                "summary": "budget summary",
                "section_overrides": {
                    "模块说明": "budget-clipped"
                },
                "mermaid_blocks": {}
            }),
            "capture-model",
        ))
    }
}

struct FakeProviderServer {
    api_base: String,
    calls: Arc<AtomicUsize>,
    active_calls: Arc<AtomicUsize>,
    max_active_calls: Arc<AtomicUsize>,
    response_delay: Duration,
    requests: Arc<Mutex<Vec<String>>>,
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
            vec![
                serde_json::json!({
                    "model": "provider-model",
                    "choices": [
                        {
                            "message": {
                                "content": format!("```json\n{content}\n```")
                            }
                        }
                    ]
                })
                .to_string(),
            ],
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
        let stop = Arc::new(AtomicBool::new(false));
        let responses = Arc::new(responses);

        let calls_ref = Arc::clone(&calls);
        let active_calls_ref = Arc::clone(&active_calls);
        let max_active_calls_ref = Arc::clone(&max_active_calls);
        let responses_ref = Arc::clone(&responses);
        let requests_ref = Arc::clone(&requests);
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
                        thread::spawn(move || {
                            let active = active_calls_ref.fetch_add(1, Ordering::SeqCst) + 1;
                            update_max_concurrency(&max_active_calls_ref, active);
                            let body = read_http_request_body(&mut stream);
                            let call_index = calls_ref.fetch_add(1, Ordering::SeqCst);
                            requests_ref.lock().unwrap().push(body);
                            if !response_delay_for_workers.is_zero() {
                                thread::sleep(response_delay_for_workers);
                            }
                            let response = responses_ref
                                .get(call_index)
                                .cloned()
                                .or_else(|| responses_ref.last().cloned())
                                .unwrap_or_else(|| {
                                    FakeProviderHttpResponse {
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
                                    }
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

    fn max_active_calls(&self) -> usize {
        self.max_active_calls.load(Ordering::SeqCst)
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
fn llm_runtime_hits_prompt_cache_on_second_request() {
    let fixture = tempdir().unwrap();
    let input = sample_page_input();
    let config = llm_config("bridge/mock-model");

    let mut first_service = FakeLlmService::valid();
    let mut first_runtime = LlmRuntime::new(fixture.path(), &config, Some(&mut first_service));
    let first = first_runtime.enrich_page(&input).unwrap().unwrap();

    let mut second_service = FakeLlmService::valid();
    let mut second_runtime = LlmRuntime::new(fixture.path(), &config, Some(&mut second_service));
    let second = second_runtime.enrich_page(&input).unwrap().unwrap();

    drop(first_runtime);
    drop(second_runtime);
    assert_eq!(first.summary, second.summary);
    assert_eq!(first_service.calls, 1);
    assert_eq!(second_service.calls, 0);
}

#[test]
fn llm_runtime_invalidates_cache_when_model_changes() {
    let fixture = tempdir().unwrap();
    let input = sample_page_input();
    let first_config = llm_config("bridge/mock-model-a");
    let second_config = llm_config("bridge/mock-model-b");

    let mut first_service = FakeLlmService::valid();
    let mut first_runtime =
        LlmRuntime::new(fixture.path(), &first_config, Some(&mut first_service));
    first_runtime.enrich_page(&input).unwrap().unwrap();

    let mut second_service = FakeLlmService::valid();
    let mut second_runtime =
        LlmRuntime::new(fixture.path(), &second_config, Some(&mut second_service));
    second_runtime.enrich_page(&input).unwrap().unwrap();

    drop(first_runtime);
    drop(second_runtime);
    assert_eq!(first_service.calls, 1);
    assert_eq!(second_service.calls, 1);
}

#[test]
fn llm_runtime_reserves_budget_for_page_enrichment() {
    let fixture = tempdir().unwrap();
    let input = sample_page_input();
    let mut config = llm_config("bridge/mock-model");
    config.max_calls = 6;
    let mut service = BudgetTrackingLlmService::default();
    let mut runtime = LlmRuntime::new(fixture.path(), &config, Some(&mut service));

    for index in 0..8 {
        let _ = runtime.classify_file_purpose(&FilePurposeAssistInput {
            path: format!("src/file_{index}.txt"),
            kind: "source".to_string(),
            language: "text".to_string(),
            file_size: 32,
            deterministic: "utility".to_string(),
            preview: "export const value = true;".to_string(),
        });
    }

    let result = runtime.enrich_page(&input).unwrap().unwrap();
    drop(runtime);
    let file_purpose_calls = service
        .prompt_types
        .iter()
        .filter(|prompt_type| prompt_type.as_str() == "file_purpose")
        .count();
    let page_enrichment_calls = service
        .prompt_types
        .iter()
        .filter(|prompt_type| prompt_type.as_str() == "page_enrichment")
        .count();

    assert_eq!(file_purpose_calls, 4);
    assert_eq!(page_enrichment_calls, 1);
    assert_eq!(
        result.section_overrides.get("模块说明").map(String::as_str),
        Some("预算保留后仍然生成了模块说明。")
    );
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
fn page_enrichment_budget_trims_large_inputs_before_request() {
    let fixture = tempdir().unwrap();
    let mut config = llm_config("bridge/mock-model");
    config.page_enrichment_max_input_tokens = 80;
    let mut input = sample_page_input();
    input.facts = (0..10)
        .map(|index| format!("事实 {index}: {}", "A".repeat(240)))
        .collect();
    input.hints = (0..8)
        .map(|index| format!("hint-{index}: {}", "B".repeat(200)))
        .collect();
    input.child_summaries = (0..8)
        .map(|index| format!("child-{index}: {}", "C".repeat(220)))
        .collect();

    let mut service = InputCaptureLlmService::default();
    let mut runtime = LlmRuntime::new(fixture.path(), &config, Some(&mut service));
    let _ = runtime.enrich_page(&input).unwrap();
    drop(runtime);

    let captured = service.last_request.expect("request should be captured");
    let original_input_len = serde_json::to_string(&input).unwrap().chars().count();
    let input_text = captured.input.to_string();
    assert!(input_text.chars().count() < original_input_len);
    assert!(input_text.contains("..."));
    assert!(
        captured
            .input
            .get("child_summaries")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|items| items.len() <= 8)
    );
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
        created_at: "2026-03-11T20:00:00Z".to_string(),
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
fn provider_direct_call_is_preferred_over_agent_bridge() {
    let fixture = tempdir().unwrap();
    let input = sample_page_input();
    let server = FakeProviderServer::start(
        r#"{"summary":"provider 摘要","section_overrides":{"模块说明":"provider 直连正文。"},"mermaid_blocks":{"依赖关系":"graph TD\nProvider-->Core"}}"#,
    );
    let mut config = llm_config("proxy/provider-model");
    config.providers.insert(
        "proxy".to_string(),
        provider_config(&server.api_base, &[("provider-model", "provider-model")]),
    );

    let mut agent_service = FakeLlmService::valid();
    let mut runtime = LlmRuntime::new(fixture.path(), &config, Some(&mut agent_service));
    let result = runtime.enrich_page(&input).unwrap().unwrap();
    let selected_path = runtime.selected_path();

    drop(runtime);
    assert_eq!(selected_path, Some(SelectedLlmPath::ProviderApi));
    assert_eq!(result.summary, "provider 摘要");
    assert_eq!(
        result.section_overrides.get("模块说明").map(String::as_str),
        Some("provider 直连正文。")
    );
    assert_eq!(server.calls(), 1);
    assert_eq!(agent_service.calls, 0);
    assert!(server
        .requests()
        .iter()
        .any(|request| request.contains("page_enrichment")));
}

#[test]
fn provider_direct_call_retries_retryable_status_three_times_by_default() {
    let fixture = tempdir().unwrap();
    let server = FakeProviderServer::start_with_http_responses(
        vec![
            FakeProviderHttpResponse {
                status_code: 500,
                body: r#"{"error":{"message":"temporary upstream error"}}"#.to_string(),
            },
            FakeProviderHttpResponse {
                status_code: 502,
                body: r#"{"error":{"message":"temporary gateway error"}}"#.to_string(),
            },
            FakeProviderHttpResponse {
                status_code: 200,
                body: serde_json::json!({
                    "model": "provider-model",
                    "choices": [
                        {
                            "message": {
                                "content": "```json\n{\"summary\":\"provider 摘要\",\"section_overrides\":{\"模块说明\":\"重试后成功。\"},\"mermaid_blocks\":{}}\n```"
                            }
                        }
                    ]
                })
                .to_string(),
            },
        ],
        Duration::ZERO,
    );
    let input = sample_page_input();
    let mut config = llm_config("proxy/provider-model");
    config.providers.insert(
        "proxy".to_string(),
        provider_config(&server.api_base, &[("provider-model", "provider-model")]),
    );

    let mut runtime = LlmRuntime::new(fixture.path(), &config, None);
    let result = runtime.enrich_page(&input).unwrap().unwrap();

    assert_eq!(server.calls(), 3);
    assert_eq!(
        result.section_overrides.get("模块说明").map(String::as_str),
        Some("重试后成功。")
    );
}

#[test]
fn provider_direct_call_honors_configured_max_retries() {
    let fixture = tempdir().unwrap();
    let server = FakeProviderServer::start_with_http_responses(
        vec![
            FakeProviderHttpResponse {
                status_code: 500,
                body: r#"{"error":{"message":"temporary upstream error"}}"#.to_string(),
            },
            FakeProviderHttpResponse {
                status_code: 200,
                body: serde_json::json!({
                    "model": "provider-model",
                    "choices": [
                        {
                            "message": {
                                "content": "```json\n{\"summary\":\"provider 摘要\",\"section_overrides\":{\"模块说明\":\"第二次才成功。\"},\"mermaid_blocks\":{}}\n```"
                            }
                        }
                    ]
                })
                .to_string(),
            },
        ],
        Duration::ZERO,
    );
    let input = sample_page_input();
    let mut config = llm_config("proxy/provider-model");
    let mut provider = provider_config(&server.api_base, &[("provider-model", "provider-model")]);
    provider.max_retries = 1;
    config.providers.insert("proxy".to_string(), provider);

    let mut runtime = LlmRuntime::new(fixture.path(), &config, None);
    let result = runtime.enrich_page(&input).unwrap();

    assert!(result.is_none());
    assert_eq!(server.calls(), 1);
}

#[test]
fn provider_direct_batch_enrichment_uses_configured_parallelism() {
    let fixture = tempdir().unwrap();
    let server = FakeProviderServer::start_with_delay(
        r#"{"summary":"provider 摘要","section_overrides":{"模块说明":"provider 直连正文。"},"mermaid_blocks":{}}"#,
        Duration::from_millis(120),
    );
    let mut config = llm_config("proxy/provider-model");
    config.page_enrichment_parallel_requests = 2;
    config.providers.insert(
        "proxy".to_string(),
        provider_config(&server.api_base, &[("provider-model", "provider-model")]),
    );

    let mut runtime = LlmRuntime::new(fixture.path(), &config, None);
    let inputs = (0..3)
        .map(|index| {
            let mut input = sample_page_input();
            input.page_id = format!("page-{index}");
            input.title = format!("模块：demo-{index}");
            input.scope = format!("module:demo-{index}");
            input.summary_inputs = vec![format!("源码：src/{index}.ts")];
            input.child_summaries = vec![format!("child summary {index}")];
            input
        })
        .collect::<Vec<_>>();

    let results = runtime.enrich_pages(&inputs).unwrap();

    assert_eq!(runtime.selected_path(), Some(SelectedLlmPath::ProviderApi));
    assert_eq!(runtime.provider_parallel_requests(), 2);
    assert_eq!(server.calls(), 3);
    assert!(server.max_active_calls() >= 2);
    assert!(results.iter().all(|result| {
        result
            .as_ref()
            .and_then(|item| item.section_overrides.get("模块说明"))
            .is_some_and(|section| section == "provider 直连正文。")
    }));
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
                            "content": "```json\n{\"summary\":\"research 摘要\",\"key_points\":[\"通过 tool 读取关键源码\"],\"evidence_rollup\":[],\"diagram_rollup\":[],\"open_questions\":[]}\n```",
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
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &SteeringConfig::default());
    let module_page = pages
        .iter()
        .find(|page| page.relative_path.ends_with("核心模块/packages/app.md"))
        .expect("module page should exist");
    let page_context = build_page_context(module_page, &report, &tree, &repo_ctx, &mod_ctxs);
    let research_input = PageResearchInput::from_page(module_page, &page_context);
    let research_runtime = PageResearchRuntimeContext {
        page: module_page,
        page_context: &page_context,
        scan_report: &report,
        module_tree: &tree,
        repo_context: &repo_ctx,
        module_contexts: &mod_ctxs,
    };
    let mut config = llm_config("proxy/provider-model");
    config.uncertainty_gate_enabled = false;
    config.content_enrichment_enabled = false;
    config.session_enabled = true;
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
    assert!(
        output
            .result
            .key_points
            .iter()
            .any(|item| item.contains("通过 tool 读取关键源码"))
    );
    assert_eq!(server.calls(), 2);
    assert!(!cache_entries.is_empty());
    assert!(requests.iter().any(|request| request.contains("\"tools\"")));
    assert!(requests
        .iter()
        .any(|request| request.contains("\"response_format\"")));
    assert!(requests
        .iter()
        .any(|request| request.contains("page_research")));
    assert!(
        output
            .session
            .recent_turns
            .iter()
            .any(|turn| turn.role == "assistant")
    );
    assert!(
        output
            .session
            .tool_artifact_refs
            .iter()
            .any(|artifact| artifact.tool_name == "read_source_snippets")
    );

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
            "end_line".to_string(),
            "note".to_string(),
            "path".to_string(),
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
            .map(|types| types.iter().filter_map(serde_json::Value::as_str).collect::<Vec<_>>()),
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
fn init_with_llm_enrichment_writes_enhanced_module_sections() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(repo_root, "package.json", r#"{"name":"llm-demo"}"#);
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
            "  enabled: true\n",
            "  model: bridge/mock-model\n",
            "pages:\n",
            "  hints:\n",
            "    - page_type: module\n",
            "      hint: 重点说明入口职责\n",
        ),
    );

    let mut sink = NoopProgressSink;
    let mut llm_service = FakeLlmService::valid();
    let report =
        run_init_with_progress_and_llm_as("init", repo_root, &mut sink, Some(&mut llm_service))
            .unwrap();

    let module_page = report
        .generated_pages
        .iter()
        .find(|path| path.contains("核心模块"))
        .expect("module page should exist");
    let content = fs::read_to_string(repo_root.join(module_page)).unwrap();

    assert!(content.contains("这是增强后的模块说明。"));
    assert!(!content.contains("```mermaid"));
    assert!(llm_service.calls > 0);
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

    let module_page = report
        .generated_pages
        .iter()
        .find(|path| path.contains("核心模块"))
        .expect("module page should exist");
    let content = fs::read_to_string(repo_root.join(module_page)).unwrap();
    let cache_entries = load_all_llm_cache(repo_root).unwrap();

    assert!(content.contains("通过 wiki.dev.yaml 走 provider。"));
    assert!(server.calls() > 0);
    assert!(!cache_entries.is_empty());
}

#[test]
fn provider_invalid_json_output_still_records_usage() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    let server = FakeProviderServer::start_with_raw_responses(
        vec![
            serde_json::json!({
                "model": "provider-model",
                "choices": [
                    {
                        "message": {
                            "content": "provider returned plain text instead of json"
                        }
                    }
                ]
            })
            .to_string(),
        ],
        Duration::ZERO,
    );

    let mut config = llm_config("proxy/provider-model");
    config.uncertainty_gate_enabled = false;
    config.content_enrichment_enabled = true;
    config.session_enabled = false;
    config.providers.insert(
        "proxy".to_string(),
        provider_config(&server.api_base, &[("provider-model", "provider-model")]),
    );

    let mut runtime = LlmRuntime::new(repo_root, &config, None);
    let input = sample_page_input();
    let input_hash = wiki_core::llm::build_prompt_input_hash(
        wiki_core::llm::PromptType::PageEnrichment,
        Some("proxy/provider-model"),
        &input,
    );
    let output = runtime.enrich_page(&input).unwrap();
    let cached = read_llm_cache(
        repo_root,
        &input_hash,
        "page_enrichment",
        "page-enrichment/v2",
        Some("proxy/provider-model"),
    )
    .unwrap()
    .expect("negative cache should be written");
    let usage = runtime.usage_snapshot();

    assert!(output.is_none());
    assert!(!cached.response.is_empty());
    assert_eq!(usage.request_count, 1);
    assert!(usage.input_tokens > 0);
    assert!(usage.output_tokens > 0);
    assert!(usage.total_tokens >= usage.input_tokens + usage.output_tokens);
    assert_eq!(
        usage.by_prompt_type
            .iter()
            .find(|bucket| bucket.key == "page_enrichment")
            .map(|bucket| bucket.request_count),
        Some(1)
    );

    let second = runtime.enrich_page(&input).unwrap();
    let second_usage = runtime.usage_snapshot();
    assert!(second.is_none());
    assert_eq!(server.calls(), 1);
    assert_eq!(second_usage.request_count, 1);
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
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &SteeringConfig::default());
    let module_page = pages
        .iter()
        .find(|page| page.relative_path.ends_with("核心模块/packages/app.md"))
        .expect("module page should exist");
    let page_context = build_page_context(module_page, &report, &tree, &repo_ctx, &mod_ctxs);
    let research_input = PageResearchInput::from_page(module_page, &page_context);
    let research_runtime = PageResearchRuntimeContext {
        page: module_page,
        page_context: &page_context,
        scan_report: &report,
        module_tree: &tree,
        repo_context: &repo_ctx,
        module_contexts: &mod_ctxs,
    };
    let input_hash = wiki_core::llm::build_prompt_input_hash(
        wiki_core::llm::PromptType::PageResearch,
        Some("proxy/provider-model"),
        &serde_json::to_value(&research_input).unwrap(),
    );
    let mut config = llm_config("proxy/provider-model");
    config.uncertainty_gate_enabled = false;
    config.content_enrichment_enabled = false;
    config.session_enabled = true;
    config.providers.insert(
        "proxy".to_string(),
        provider_config(&server.api_base, &[("provider-model", "provider-model")]),
    );
    let mut runtime = LlmRuntime::new(repo_root, &config, None);

    let first = runtime.research_page(&research_input, &research_runtime).unwrap();
    let cached = read_llm_cache(
        repo_root,
        &input_hash,
        "page_research",
        "page-research/v1",
        Some("proxy/provider-model"),
    )
    .unwrap()
    .expect("negative page research cache should exist");
    let second = runtime.research_page(&research_input, &research_runtime).unwrap();

    assert!(first.is_none());
    assert!(second.is_none());
    assert!(cached.response.contains("\"_cache_status\":\"negative\""));
    assert_eq!(server.calls(), 2);
}

#[test]
fn init_falls_back_to_deterministic_content_when_enhancement_invalid() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(repo_root, "package.json", r#"{"name":"llm-demo"}"#);
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
            "  enabled: true\n",
            "  model: bridge/mock-model\n",
        ),
    );

    let mut sink = NoopProgressSink;
    let mut llm_service = FakeLlmService::invalid();
    let report =
        run_init_with_progress_and_llm_as("init", repo_root, &mut sink, Some(&mut llm_service))
            .unwrap();

    let module_page = report
        .generated_pages
        .iter()
        .find(|path| path.contains("核心模块"))
        .expect("module page should exist");
    let content = fs::read_to_string(repo_root.join(module_page)).unwrap();

    assert!(content.contains("根路径位于"));
    assert!(!content.contains("```mermaid"));
}
