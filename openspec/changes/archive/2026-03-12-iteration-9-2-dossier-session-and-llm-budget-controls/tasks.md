## 1. Dossier 与 Rollup

- [x] 1.1 在 `crates/wiki-core/src/domain/context.rs`、`crates/wiki-core/src/generation/context.rs` 中引入 `ModuleDossier`、`TopicDossier`、`ChildPageRollup`、`PageResearchResult` 等稳定对象，并为它们补齐 identity/hash 规则；`PageResearchResult` 固定为 `summary / key_points / evidence_rollup / diagram_rollup / open_questions`。
- [x] 1.2 让 `module`、`topic` 页优先消费 dossier 中的 `key_sources / key_symbols / source_snippets / evidence_rollup / diagram_rollup / child_page_rollup`，而不是继续只吃扁平 `facts / child_summaries`。
- [x] 1.3 调整父页上下文与增量传播链，让 `overview / architecture / module / topic` 父页基于 child rollup 变化触发重建，而不是重复重扫底层 facts。

## 2. Research Session 与 Tool Schema

- [x] 2.1 在 `crates/wiki-core/src/transport/json_rpc.rs` 和 `crates/wiki-core/src/llm/**` 中定义 `agent_session_v1` 的 core/provider contract，首批支持 `agent_session_start / agent_message / agent_tool_call / agent_tool_result / agent_final / agent_abort`，但 9.2 只要求 provider 直连路径先跑通。
- [x] 2.2 为 provider-tools 收敛统一的只读 tool schema，首批实现 `read_source_snippets`、`get_module_context`、`get_symbol_neighbors`、`get_process_trace`、`get_page_children`、`get_evidence_group`、`search_topic_candidates`，并保证后续 agent-bridge 可复用该 schema。
- [x] 2.3 在 `module` / `topic` 页试点 bounded research session，并确保最终只回填结构化 `PageResearchResult`，不让 provider session 直接落最终 Markdown。
- [x] 2.4 为 session state 增加 `session_id / session_summary / recent_turns / tool_artifact_refs`，并把默认 `recent_turns` 控制在 4-8 轮窗口内。
- [x] 2.5 为 provider tools 实现 capability 分层与降级链：`native_tools -> emulated_tools -> no_tools -> deterministic`，并支持显式配置或 `auto` 探测。
- [x] 2.6 为 provider 请求默认发送顶层 `response_format`，并继续在 prompt 中保留 `response_schema`；若接口因该字段本身报 transport-level 错误，仅允许做一次去字段重试。
- [x] 2.7 为 `emulated_tools` 定义固定 envelope：assistant 消息使用 `type/content/tool_calls`，tool 消息使用 `type/tool_call_id/name/content`，final 消息使用 `type/result`，并保证字段语义尽量对齐 `native_tools`。

## 3. Budget、并行度与 Cache Mode

- [x] 3.1 在 `crates/wiki-core/src/domain/steering.rs` 与 `wiki.dev.yaml` 覆盖逻辑中新增 phase-specific LLM 配置：`uncertainty_gate_max_input_tokens`、`page_enrichment_max_input_tokens`、`session_max_context_tokens`、`session_max_recent_turns`、阶段开关与并行度；同时引入 `~/.spec-wiki/config.yaml` 用户级默认配置。
- [x] 3.2 将 `uncertainty_gate` 收敛为“同类型批量 + 有限并行”，优先落 `file_purpose`，并把 `top_level_promotion`、`dependency_edge` 纳入相同批量 contract。
- [x] 3.3 将 `init / rebuild` 的 runtime 清理与 LLM cache 解耦，支持 `preserve / clear / refresh` 三种 cache mode，并新增显式 cold-start 路径用于性能测试。
- [x] 3.4 引入 `~/.spec-wiki/state.yaml` learned state，按 `api_base + provider + model_id` 缓存 provider capability 探测/降级结果；仅在 `auto` 模式生效，并带 TTL、原子写入和错误类型过滤。

## 4. Usage 与 Progress

- [x] 4.1 在 `crates/wiki-core/src/workflows/**`、`crates/wiki-core/src/llm/mod.rs`、`crates/wiki-core/src/transport/json_rpc.rs` 中实现普通模式实时 usage 统计，每次真实请求完成后更新 `request_count / input_tokens / output_tokens / total_tokens`。
- [x] 4.2 让 progress/event stream 输出 workflow 总计、按 `prompt_type` 分组和 provider/model 维度的 usage snapshot，并保持终态唯一性。
- [x] 4.3 扩展 debug trace，补充 dossier 输入摘要、session turns、tool artifact refs、budget 裁剪信息和 usage 来源（provider usage / local estimate）。

## 5. 验证与脚本

- [x] 5.1 为 Rust core 增加自动化测试，覆盖 dossier/rollup 稳定性、provider session、tool schema、phase budget 裁剪、同类型批量 gate 和 cache mode。
- [x] 5.2 升级 `scripts/run-test-projects.mjs`、`scripts/test-wiki-lifecycle.mjs`、`scripts/collect-reference-project-reports.mjs`，明确输出 cold run / warm run 区分、实时 token usage 和 dossier/provider-session 相关指标。
- [x] 5.3 使用供应商模型路径运行 `node scripts/run-test-projects.mjs` 全量 19 项目，运行 `node scripts/test-wiki-lifecycle.mjs` 全链路验证，并对有 reference 的项目生成新的逐项目报告。

## 6. 注释与收尾

- [x] 6.1 按 `COMMENTING.md` 单独检查 `crates/wiki-core/**`、`scripts/**` 中 dossier、session、budget 和 usage 相关新增注释。
- [x] 6.2 在当前 9.2 change 目录中沉淀一份“cold/warm 对照与 session trace 解读”说明，避免关键判断只存在对话上下文里。
