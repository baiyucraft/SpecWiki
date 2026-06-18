## Context

当前主链已经稳定收敛为 `scan -> parse_symbols -> resolve_symbol_graph -> analyze_symbol_graph -> module_tree -> build_contexts -> plan_pages -> render_pages`。从真实代码看：

- [`crates/wiki-core/src/generation/context.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/context.rs) 虽然已经能产出 `evidence_groups` 和 `diagram_inputs`，但页面增强仍主要消费二手摘要，而不是稳定 research object。
- [`crates/wiki-core/src/llm/mod.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/llm/mod.rs) 目前分成 `uncertainty_gate` 和 `page_enrichment` 两层，provider 路径和 Agent 路径也都还是“请求一次、返回一次”的模型。
- [`crates/wiki-core/src/transport/json_rpc.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/transport/json_rpc.rs) 与 [`agents/codebuddy/src/runtime/invokeCore.ts`](/E:/project/!byAI/spec-wiki/agents/codebuddy/src/runtime/invokeCore.ts) 只支持单轮 `llm_request / llm_response / llm_unavailable`，不支持会话状态、tool 调用和多轮推理。
- [`crates/wiki-core/src/workflows/init.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/init.rs) 和 [`crates/wiki-core/src/workflows/rebuild.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/rebuild.rs) 仍然把 LLM 看成“可选增强步骤”，而不是“结构化 research 阶段”。
- [`crates/wiki-core/src/debug_trace.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/debug_trace.rs) 已经能保留完整 JSON trace，但普通模式下还看不到实时 token usage，也没有 phase-specific budget guard。

对 `storybook-20260311-193106` 的真实 trace 说明：

- `file_purpose` 最长一次请求约 `140,370` 字符，约 `35k tokens`；
- `page_enrichment` 最长一次请求约 `13,678` 字符，约 `3.4k tokens`。

这说明 `uncertainty_gate` 和 `page_enrichment` 的输入规模不是一个量级，9.2 必须把 phase budget、批量化和实时 usage 独立建模，而不能只给一个总开关。

## Goals / Non-Goals

**Goals**

- 在不绕开当前 deterministic 主链的前提下，引入稳定 `TopicDossier / ModuleDossier / PageResearchResult` 中间层。
- 让父页明确消费 child rollup 和高价值源码片段，而不是重复消费扁平 facts。
- 把 Agent bridge 升级为有边界的 `agent_session_v1`，并和 provider tool-calling 共享同一套只读 tool schema。
- 让 `wiki.dev.yaml` / steering 支持 phase-specific 上下文上限、并行度、cache mode 和显式 cold-start。
- 在普通模式下实时输出 LLM usage，让长流程成本和热点阶段可观测。

**Non-Goals**

- 不把 `init` 整条链改成无限自由对话 agent。
- 不让 Agent 或 provider tool 直接落最终 Markdown。
- 不在 9.2 同轮引入完整 query/RAG UI 或新的 runtime 目录层。
- 不把所有 `uncertainty_gate` 跨类型揉成一个 mega prompt。
- 不在 9.2 同轮实现或验证 CodeBuddy Agent 侧的 bridge 细节；这轮先收 core/provider 路径。

## Decisions

### 1. 先引入 dossier 层，再谈多轮会话

决定：

- 在 `build_contexts` 之后、`plan_pages` 和 `render_pages` 之前引入稳定 dossier 对象；
- 首批对象至少包括：`ModuleDossier`、`TopicDossier`、`ChildPageRollup`、`PageResearchResult`；
- dossier 负责沉淀：
  - `key_sources`
  - `key_symbols`
  - `source_snippets`
  - `cross_module_edges`
  - `process_candidates`
  - `evidence_rollup`
  - `diagram_rollup`
  - `child_page_rollup`

原因：

- 9.1 之后，页面已经有 evidence layer，但 LLM 仍主要看到二手摘要。
- 参考 `deepwiki-rs` 和 `CodeWiki` 的真实实现，真正值钱的是“research 结果先沉淀，再进入 compose”，不是最后多写几句 prompt。

### 1.1 `PageResearchResult` 固定为结构化研究结果，不承载最终 Markdown

决定：

- `PageResearchResult` 固定包含以下字段：

  ```json
  {
    "summary": "string",
    "key_points": ["string"],
    "evidence_rollup": [
      {
        "group_key": "string",
        "title": "string",
        "items": [
          {
            "source_id": "string",
            "path": "string",
            "start_line": 1,
            "end_line": 10,
            "note": "string"
          }
        ]
      }
    ],
    "diagram_rollup": [
      {
        "diagram_key": "string",
        "diagram_type": "module_dependency | hierarchy | process",
        "title": "string",
        "summary": "string"
      }
    ],
    "open_questions": ["string"]
  }
  ```

- 字段约束：
  - `summary`：1 段高密度摘要，不是长文
  - `key_points`：3-7 条
  - `evidence_rollup`：1-4 组，每组 1-6 条 evidence
  - `diagram_rollup`：0-3 条，只能引用 deterministic 已存在的 diagram inputs
  - `open_questions`：0-3 条，且必须是“待确认点”，不是自由发挥
- `PageResearchResult` 不包含最终 section markdown、页面标题重命名、任意新页面建议或无约束 Mermaid 文本。

原因：

- 这能把 research 结果稳定限制在“可被 renderer 消费的结构化中间产物”，避免模型直接篡改页面拓扑和 managed section。
- `summary + key_points + evidence_rollup + diagram_rollup + open_questions` 足够覆盖 9.2 的目标，又不会把结果对象扩得太松。

### 2. 父页消费 child rollup，不再重复重扫事实

决定：

- `overview / architecture / module / topic` 父页必须显式消费 child rollup；
- child rollup 除 `summary` 外，还至少包含：
  - `evidence_rollup`
  - `diagram_rollup`
  - `key_sources_rollup`
  - `open_questions`
- parent rebuild 传播链基于 child rollup hash，而不是只基于 child page markdown body。

原因：

- 当前页之间已经有父子关系，但父页仍偏向再次解释原始 facts。
- 参考 `CodeWiki`，leaf-first 价值在于“父页建立在子页结果之上”，而不是把叶子 facts 再重复说一遍。

### 3. research session 只做有边界的中间研究，不替代 renderer

决定：

- research session 只在 `module` / `topic` 页启用；
- 会话位置固定在：
  `facts/graph/planner -> dossier assembly -> research session -> structured PageResearchResult -> deterministic render`
- Agent 或 provider tool-calling 的最终产物必须是结构化 `PageResearchResult`，不是最终 Markdown。

原因：

- 当前仓库的 runtime、managed sections、page_id 和增量更新语义都已经稳定，不能被自由对话 agent 直接接管。
- 用户要的是更强的连续研究能力，不是丢掉当前 deterministic render contract。

### 4. `agent_session_v1` 先落 provider-tools，agent-bridge 后续跟进

决定：

- 研究协议统一为 `agent_session_v1`；
- provider 直连支持 tool-calling 时，走 provider-tools；
- 9.2 不要求未配置 provider 时立刻走 agent-bridge；
- 先把 provider-tools 路径跑通，并把 tool schema 与 structured result schema 设计成后续 agent-bridge 可复用。

首批事件：

- `agent_session_start`
- `agent_message`
- `agent_tool_call`
- `agent_tool_result`
- `agent_final`
- `agent_abort`

首批只读工具：

- `read_source_snippets(source_ids)`
- `get_module_context(module_ids)`
- `get_symbol_neighbors(symbol_ids)`
- `get_process_trace(process_id)`
- `get_page_children(page_id)`
- `get_evidence_group(group_id)`
- `search_topic_candidates(scope)`

首批工具的最小 contract 固定为：

- `read_source_snippets`
  - args: `{ "source_ids": ["string"] }`
  - result: `{ "snippets": [{ "source_id": "string", "path": "string", "start_line": 1, "end_line": 10, "content": "string" }] }`
- `get_module_context`
  - args: `{ "module_ids": ["string"] }`
  - result: `{ "modules": [{ "module_id": "string", "title": "string", "summary": "string", "key_sources": ["string"], "child_page_ids": ["string"] }] }`
- `get_symbol_neighbors`
  - args: `{ "symbol_ids": ["string"], "edge_types": ["string"] }`
  - result: `{ "neighbors": [{ "symbol_id": "string", "edges": [{ "edge_type": "string", "target_symbol_id": "string", "confidence": 0.9 }] }] }`
- `get_process_trace`
  - args: `{ "process_id": "string" }`
  - result: `{ "process": { "process_id": "string", "title": "string", "steps": [{ "step_key": "string", "label": "string", "source_id": "string" }] } }`
- `get_page_children`
  - args: `{ "page_id": "string" }`
  - result: `{ "children": [{ "page_id": "string", "title": "string", "summary": "string" }] }`
- `get_evidence_group`
  - args: `{ "page_id": "string", "group_key": "string" }`
  - result: `{ "group": { "group_key": "string", "title": "string", "items": [{ "source_id": "string", "path": "string", "note": "string" }] } }`
- `search_topic_candidates`
  - args: `{ "scope": "repo | module", "scope_id": "string", "query": "string" }`
  - result: `{ "candidates": [{ "topic_key": "string", "title": "string", "reason": "string", "evidence_source_ids": ["string"] }] }`

原因：

- 用户要求“持续交流”和“tool 调用”，但我们不能开放泛化 shell。
- `CodeWiki`、`GitNexus`、`deepwiki-open` 的共同经验是：受控工具面比自由上下文更稳定。
- 当前用户已经明确要求先做供应商模型路径测试，因此本轮先把 provider 直连链跑通、量化和验证，避免同一轮同时摊开 Agent 侧协议与宿主测试。
- 先把 7 个工具的参数和结果固定下来，后续无论 provider-tools、emulated_tools 还是 agent-bridge 都能共享一套 DTO。

### 4.1 provider tool-calling 按当前通用字段约定实现，不自创嵌套格式

决定：

- provider 直连进入 tool-calling 时，HTTP body 继续以 `messages` 为主；
- `tools` 与 `tool_choice` 放在 provider 请求顶层，和 `messages` 同级，而不是塞进 `input` 或拼进 `user.content`；
- 工具调用结果以连续 turn 的方式回灌到 `messages`，遵循当前主流 chat-completions/tool-calling 约定，而不是把 tool 结果重新压回 `tools` 字段；
- `input` 只承载 dossier、evidence、rollup、budget 和当前任务范围，`response_schema` 只承载最终 `PageResearchResult` 的结构约束。

原因：

- 这和当前主流 provider 的 tool-calling 语义一致，后续兼容不同 openai-compatible provider 的成本最低。
- 如果把工具定义塞进 `input` 或 `user.content`，模型只能“看见工具描述”，不能真正进入 provider 原生的 tool-calling 模式。
- 这样也能把 provider 路径和后续 agent-bridge 路径的职责切干净：前者负责标准 `messages/tools/tool_choice`，后者复用同一套 tool schema 与结构化结果 schema。

### 4.2 provider tools 采用能力分层兼容，而不是假设所有接口都原生支持

决定：

- provider tools 能力分成三档：
  - `native_tools`：provider 原生支持顶层 `tools / tool_choice / tool_calls`
  - `emulated_tools`：provider 不支持原生 tools，但支持稳定的多轮 JSON 结构输出
  - `no_tools`：provider 不支持原生 tools，且不适合稳定做模拟 tools
- capability 可由 provider 配置显式声明，也可支持 `auto` 探测；
- `auto` 时优先尝试 `native_tools`，若接口显式报不支持 tools，再降级到 `emulated_tools`，仍失败则降到 `no_tools`；
- `no_tools` 下不进入 provider research session，直接回退到单轮增强或纯 deterministic。

兼容语义：

- `native_tools`
  - 顶层发送 `messages / tools / tool_choice`
  - provider 返回原生 tool call
- `emulated_tools`
  - 在消息内容里放 `available_tools` 与 tool protocol 说明
  - envelope 形状尽量贴近 `native_tools`，复用 `tool_calls / tool_call_id / function.name / function.arguments` 语义
  - assistant tool-call envelope 固定为：

    ```json
    {
      "type": "assistant",
      "content": null,
      "tool_calls": [
        {
          "id": "call_001",
          "type": "function",
          "function": {
            "name": "read_source_snippets",
            "arguments": "{\"source_ids\":[\"src:a\",\"src:b\"]}"
          }
        }
      ]
    }
    ```

  - core 执行工具后，回灌的 tool-result envelope 固定为：

    ```json
    {
      "type": "tool",
      "tool_call_id": "call_001",
      "name": "read_source_snippets",
      "content": "{\"snippets\":[{\"source_id\":\"src:a\",\"path\":\"src/router.ts\",\"content\":\"...\"}]}"
    }
    ```

  - 最终结果 envelope 固定为：

    ```json
    {
      "type": "final",
      "result": {
        "summary": "...",
        "evidence_rollup": [],
        "diagram_rollup": [],
        "open_questions": []
      }
    }
    ```

  - 其中：
    - `tool_calls[].function.arguments` 固定为 JSON 字符串，和当前主流 native tool-calling 保持一致
    - `tool.content` 固定为 JSON 字符串，避免 provider 对 object/array content 兼容不一致
    - `final.result` 固定满足 `PageResearchResult` schema
- `no_tools`
  - 直接跳过 session tool loop
  - 回退到单轮 `page_enrichment` 或 deterministic

原因：

- 现在的 provider 生态不稳定，不能把 9.2 建在“所有 openai-compatible 接口都完整支持 tools”这个假设上。
- `native_tools` 是首选，因为最省 token、最稳；`emulated_tools` 是兼容路径；`no_tools` 是必须有的保底路径。
- 这样设计后，provider 路径和后续 agent-bridge 路径都可以复用同一套 canonical tool schema，而 capability 只影响适配方式，不影响上层 dossier / result contract。
- `emulated_tools` 若尽量贴近 `native_tools`，后续 provider 适配层、trace 和测试都更容易共享解析逻辑，不需要维护两套完全不同的心智模型。
- 把 envelope 定死为三种消息形状后，session parser、debug trace、回放测试和 future agent-bridge 都可以直接复用同一套断言。

### 4.3 `response_format` 不做能力分层，统一采用“默认传 + prompt 双保险”

决定：

- `response_format` 不参与 `native / emulated / none` 这类 capability 分层；
- provider 请求默认在顶层发送 `response_format`，并继续在 prompt 中保留 `response_schema` 说明；
- 优先使用能表达最终结构约束的 `json_schema` 形态，而不是只依赖松散的文本约定；
- 即使顶层已经传了 `response_format`，也不移除 prompt 内的 schema 提示；
- 若某些代理或兼容接口对该字段本身直接报 transport-level 错误，允许做一次“仅移除该字段”的传输层重试，但这不构成上层 contract 的语义降级。

原因：

- `response_format` 和 `tools` 不同。`tools` 涉及 provider 是否具备原生 tool-calling 能力，确实需要能力分层；`response_format` 更适合作为默认结构约束。
- “顶层 `response_format` + prompt 内 `response_schema`”是当前最稳的双保险：支持原生结构约束的 provider 能直接受益，不完整兼容的代理仍可退回到 prompt 里的结构要求。
- 把“字段被网关拒绝时移除一次再试”限定在 transport 层，可以避免把这个问题误建模成上层语义能力分层。

### 5. 会话复用的是结构化沉淀，不是整段聊天历史

决定：

- 会话状态分成三层：
  - `session_summary`
  - `recent_turns`
  - `tool_artifact_refs`
- 仓库和页面级复用对象分成四层：
  - `repo_context_cache`
  - `module_dossier_cache`
  - `page_rollup_cache`
  - `session_summary_cache`
- 默认只保留最近 `4-8` 轮原文；更长历史压缩成 summary。

原因：

- 能复用的是结构化知识对象，不是原始聊天流水。
- 这样才能既支持连续交流，又不让 token 规模持续失控。

### 6. budget、并行度和 cache mode 都必须按阶段配置

决定：

- 用户级目录固定为 `~/.spec-wiki/`；
- 主配置放 `~/.spec-wiki/config.yaml`，承载用户显式声明的 provider、budget、并行度和默认模式；
- 学习态与运行态能力探测结果放 `~/.spec-wiki/state.yaml`，避免反复污染用户手写主配置；
- steering / `wiki.dev.yaml` 至少支持：
  - `llm.uncertainty_gate.enabled`
  - `llm.content_enrichment.enabled`
  - `llm.uncertainty_gate_max_input_tokens`
  - `llm.page_enrichment_max_input_tokens`
  - `llm.session_max_context_tokens`
  - `llm.session_max_recent_turns`
  - `llm.uncertainty_gate_parallel_requests`
  - `llm.page_enrichment_parallel_requests`
  - `llm.cache_mode`
- `cache_mode` 支持 `preserve | clear | refresh`；
- provider capability 的 learned state 只在上层配置为 `auto` 时生效；
- learned state 按 `api_base + provider + model_id` 维度缓存，并带 TTL；
- 只有明确属于 provider capability 不支持的错误，才允许回写到 learned state；401/403、429、5xx、timeout、网络错误和一次性解析失败不得回写为持久降级；
- `init / rebuild` 默认不隐式清空 LLM cache，但必须支持显式 cold-start。

用户级配置最小结构固定为：

```yaml
version: 1

llm:
  providers:
    proxy:
      api_base: http://host/v1
      api_key_env: SPEC_WIKI_PROXY_KEY
      default_model: gpt-5.2
      capabilities:
        tools: auto
        response_format: true
  uncertainty_gate_enabled: true
  content_enrichment_enabled: true
  session_enabled: true
  uncertainty_gate_max_input_tokens: 12000
  page_enrichment_max_input_tokens: 8000
  session_max_context_tokens: 16000
  session_max_recent_turns: 6
  uncertainty_gate_parallel_requests: 3
  page_enrichment_parallel_requests: 3
  cache_mode: preserve
```

learned state 最小结构固定为：

```yaml
version: 1

learned:
  providers:
    proxy:
      models:
        gpt-5.2:
          api_base: http://host/v1
          tools_mode: emulated
          detected_at: "2026-03-11T21:30:00+08:00"
          reason: provider_rejected_tools
          ttl_hours: 168
```

其中：

- `config.yaml` 只承载用户显式配置；
- `state.yaml` 只承载 learned capability 结果；
- `tools_mode` 仅在上层 capability 为 `auto` 时生效。

原因：

- `uncertainty_gate` 和 `page_enrichment` 的输入规模差异非常大；
- 开发阶段需要默认 warm cache，性能测试又必须支持 cold-start。
- 把“用户显式配置”和“运行时学习结果”分开，可以避免系统反复改写用户手工维护的配置文件。
- provider capability 一旦已经明确探测过，就不应该在后续每次请求里重新试探一遍。

### 7. `uncertainty_gate` 只做“同类型批量 + 有限并行”

决定：

- `file_purpose` 优先改为同类型批量；
- `top_level_promotion`、`dependency_edge` 也支持批量；
- `module_kind` 视候选规模决定是否批量；
- 不做跨阶段、跨 schema 的 mega prompt。

原因：

- 这样既能显著减少请求数，又不会损失 cache 粒度、失败隔离和增量复用。

### 8. 普通模式下实时输出 usage，debug trace 继续保留明细

决定：

- 每次真实 LLM 请求完成后，都在正常 progress/event stream 中更新 usage snapshot；
- 最少实时输出：
  - `request_count`
  - `input_tokens`
  - `output_tokens`
  - `total_tokens`
- 维度至少包括：
  - workflow 总计
  - `prompt_type`
  - provider/model
- debug trace 继续保留单请求 JSON、session 细节和原始响应。

原因：

- 用户在普通跑数和调试里都需要实时知道“现在花了多少 token、慢在哪个阶段”，不能只有 trace 文件。

## Risks / Trade-offs

- [dossier 增加复杂度] → 先只覆盖 `module/topic` 两类页，保持其他页仍可走现有 deterministic 路径。
- [agent session 扩大会话成本] → 通过 phase budget、最近轮次上限和只读工具面收口。
- [provider tool-calling 与 agent-bridge 行为漂移] → 两条路径共用 tool schema 和 structured result schema。
- [cache 保留掩盖性能问题] → 明确区分 `preserve / clear / refresh`，并要求测试报告区分 cold/warm。
- [usage 统计不完整] → provider 有 usage 就优先信 provider；缺失时再回退本地估算，并在 trace 里标明来源。

## Migration Plan

1. 先落 dossier 数据模型、child rollup 和 parent rollup 消费，不启用多轮会话。
2. 再引入 `agent_session_v1` 的 provider-tools 路径，只在 `module/topic` 页试点。
3. 同步落 phase budget、cache mode 和实时 usage 输出。
4. 最后补 uncertainty gate 批量化和 cold/warm 报告口径，跑 provider 路径的 reference 项目与 19 项目集对照。

## Deferred

- 这三项统一后置到迭代 11（CodeBuddy Agent 可用化）：
  - CodeBuddy Agent 对 `agent_session_v1` 的 bridge 实现
  - Agent 侧 tool schema 执行与 usage 回传
  - CodeBuddy Agent 相关自动化测试与 e2e 验证
- 跨平台与跨宿主扩展后置到迭代 12（Distribution, Hosts & Platform Expansion）

## Open Questions

- provider tool-calling 路径是否需要在 core 内部维护中间 turns，还是只在 trace/runtime 中保留压缩后的 session state。
- `PageResearchResult` 是否需要区分“模型建议但未采纳”的 rejected notes。
- `overview / architecture` 页何时切到 dossier/session 路线，还是继续以 child rollup 为主即可。
