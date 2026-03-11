## Context

当前仓库的核心事实链已经基本成型。[`crates/wiki-core/src/workflows/init.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/init.rs) 真实执行的是 `scan -> parse_symbols -> resolve_symbol_graph -> analyze_symbol_graph -> build_module_tree -> build_contexts -> plan_pages -> render -> write_state -> write_metadata`；[`crates/wiki-core/src/repo/scanner.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/scanner.rs) 和 [`crates/wiki-core/src/repo/hierarchy.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/hierarchy.rs) 已经提供 deterministic 的 `FilePurpose`、顶层目录晋升和 `module_kind` 推断；[`crates/wiki-core/src/repo/symbol_graph/analyze/{communities,processes}.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/symbol_graph/analyze/) 已经能生成 communities、processes 和 cycle warnings。

但页面表达层仍然偏“骨架模板”。[`crates/wiki-core/src/domain/context.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/domain/context.rs) 只保留 `facts` 和 `summary_inputs` 两类字符串输入；[`crates/wiki-core/src/generation/sections.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/sections.rs) 仍按固定 section 标题把这些输入直接渲染成项目符号；[`crates/wiki-core/src/generation/renderer.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/renderer.rs) 只负责把 section body 串起来，并没有“高密度解释层”的插点。与此同时，SQLite 虽然已经在 [`crates/wiki-core/src/storage/sqlite_store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/storage/sqlite_store.rs) 里预留了 `llm_cache`，但还没有任何读写逻辑。

当前 JSON IPC 也还不具备 LLM 桥接能力。[`crates/wiki-core/src/main.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/main.rs) 在 `--json` 模式下会一次性读完 stdin；[`crates/wiki-core/src/transport/dto.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/transport/dto.rs) 只定义了 `progress / result / error` 三类事件；[`agents/codebuddy/src/runtime/invokeCore.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/runtime/invokeCore.ts) 也只会消费这些事件。因此，如果迭代 9 仍坚持“core 不直连 LLM、Agent 不承载 Wiki 业务规则”，就必须同时补齐一个可选的 LLM 协议桥，而不是只在 Rust 里加一个空 trait。与此同时，新的实现要求还需要允许 core 在本地直接调用供应商 API，并在 dev 测试时优先使用本地 dev 配置，而不是每次都依赖 Agent bridge。

上游真实实现给出了三个明确借鉴点：

- deepwiki-rs 的 [`workflow.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/workflow.rs)、[`preprocess/mod.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/preprocess/mod.rs) 和 [`compose/mod.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/compose/mod.rs) 证明，内容生成应该建立在稳定事实和中间 memory 之上，而不是让 LLM 直接支配文件扫描或页面树。
- CodeWiki 的 [`documentation_generator.py`](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/documentation_generator.py) 证明，父模块/总览页质量取决于“叶子页先生成、父页消费子页摘要”的自底向上顺序，而不是对整个仓库做一次大 prompt。
- GitNexus 的 [`pipeline.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/pipeline.ts)、[`community-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/community-processor.ts) 和 [`process-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/process-processor.ts) 证明，graph facts 必须先稳定落库，再交给上层解释/搜索/展示消费。

## Goals / Non-Goals

**Goals:**

- 在现有 deterministic pipeline 之上增加可选的 LLM 辅助判断和页面内容增强，而不是推翻当前 scanner / symbol graph / planner 主链。
- 保持 page_id、section_id、managed marker、user section 保留、增量 update 和 SQLite runtime 的稳定语义。
- 为当前 binary + CodeBuddy Agent 架构提供一个可选的、thin Agent 风格的 LLM 桥接协议，使“第一个可用版本”可以真正跑通。
- 让迭代 8 的 graph facts 真正进入 overview / architecture / module / workflow 页面正文，而不是继续停留在 `summary_inputs` 的项目符号层。
- 通过 `llm_cache`、输入哈希、调用预算和 steering 开关，控制成本、稳定性和可回退性。

**Non-Goals:**

- 不在本迭代引入 TOON、RAG、向量检索或 deepwiki-open 式 request-scoped retrieval 主链；这些仍归迭代 10。
- 不让 core 直接调用外部 LLM API，也不把 prompt 逻辑或 Wiki 业务规则下沉到 Agent。
- 不重写 ModuleTree、PagePlanner、Query 契约；本迭代只在既有事实模型和页面渲染链上增加增强层。
- 不把 Mermaid 图升级成独立持久化表；本迭代只生成受控的 Mermaid fenced block，并作为 managed section 内容写入现有 runtime。

## Decisions

### 1. LLM 能力采用“provider 直连优先，Agent bridge 兜底”的双入口

本迭代不再把 LLM 接入限定为 Agent bridge 单一路径，而是明确分成两个入口，并规定优先级：

- 若当前 repo 的 LLM 配置已经提供可用的供应商 API 参数，则 `wiki-core` 直接完成 HTTP 调用。
- 只有在 core 侧没有可用 provider 配置时，才退回到现有 `wiki-core --json` 长流程协议上的可协商 LLM 会话扩展，由 Agent 代调用。

Agent bridge 的协议形态保持不变，具体做法是：

- `CoreCommand` 增加可选的 LLM 协商字段；只有当 Agent 显式声明支持时，core 才进入增强会话模式。
- 长流程 stdout 除了现有 `progress / result / error` 外，可额外输出 `llm_request` 事件。
- Agent 保持 stdin 打开，并回写 `llm_response` 或 `llm_unavailable` 事件；core 负责 prompt 组装、响应解析和 fallback。
- 未协商或 Agent 不支持时，且 core 侧也没有 provider 配置，workflow 完全沿用 deterministic 路径，协议仍只有 `progress / result / error`。

这样做的原因是当前部署接口就是 binary：[`main.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/main.rs) 与 [`invokeCore.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/runtime/invokeCore.ts) 已经围绕 NDJSON 长流程建立了最小事件流，只缺少双向会话。相比之下：

- 仍然让 Agent 先读仓库、再自己拼 prompt 会复制 core 里的 scanner / hierarchy / graph / page context 逻辑。
- 两阶段“先 plan prompt、后 resume workflow”的方案虽然可行，但需要新增中间状态、resume token 和额外 workflow action，侵入面并不比扩展会话更小。
- 对开发调试来说，只靠 Agent bridge 会让本地验证依赖宿主环境，无法直接用一个本地 dev 配置完成 provider 验证。

### 1.1 本地 dev 配置不放进 `.wiki/`

provider API 往往带有密钥、代理地址或私有网关配置，不适合放进会被 runtime 清理的 `.wiki/` 目录。当前 [`remove_runtime()`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/storage/wiki_fs.rs) 会删除整个 `.wiki/`，因此本迭代将本地 dev 配置固定放在 repo 根的 `wiki.dev.yaml`：

- `.wiki/wiki.steering.yaml` 继续承载团队共享的 steering。
- `wiki.dev.yaml` 只承载本机开发调试覆盖项，主要是 `llm.providers.*` 与可选的 `llm.enabled/model`。
- 加载顺序是 `wiki.steering.yaml` 在前，`wiki.dev.yaml` 在后；dev 文件只做本地覆盖，不参与正式 runtime 输出。

### 1.2 provider 配置采用“provider registry + model registry”两级结构

为了同时支持一个 provider 下挂多个可选模型，并让缓存键、日志和 steering 选择语义保持稳定，本迭代把 provider 直连配置收敛为：

- `llm.providers.<provider_name>`：描述一个 provider 端点、鉴权和超时配置；
- `llm.providers.<provider_name>.models.<model_name>`：描述该 provider 下可选的模型条目；
- `llm.model = "<provider_name>/<model_name>"`：统一选择本轮 workflow 使用的模型。

workflow 内部据此做两层解析：

- 先把 `llm.model` 解析成 `provider_name + model_name`；
- 再到 `llm.providers` 中找到对应 provider 和模型配置，得到真正用于 provider API 请求的 `model_id`。

这样做的原因是当前单一 `llm.provider + llm.model` 结构只能表达“一个 provider + 一个模型字符串”，不适合本地 dev 在同一代理商下快速切换多个模型，也不利于把缓存键稳定收敛到 `provider/model` 这一层。相比之下，provider registry 结构更接近真实宿主配置，也让 Agent bridge 在未直连时继续看到稳定的模型选择标识。

### 2. LLM 只分两层使用：Uncertainty Gate 和 Content Enrichment

LLM 不直接改写事实层，而是分成两个受控层次：

- Uncertainty Gate：只在当前 deterministic 逻辑的模糊边界触发，主要插在 [`classify_file_purpose`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/scanner.rs)、`TopLevelRootStats::should_promote()`、[`module_kind`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/hierarchy.rs) 兜底分支，以及 `confidence = "heuristic"` 的跨模块依赖语义判断处。
- Content Enrichment：只消费已经稳定的 `PageContext`、`ModuleContext`、`RepoContext`、`GraphSummary` 和子页面摘要，负责把固定 bullet list 转成高信息密度文字和受控 Mermaid 图。

这样能保持“deterministic facts 先于解释层”的边界，和 deepwiki-rs 的 `preprocess -> compose`、GitNexus 的 `facts first` 路线一致。替代方案是让 LLM 参与模块树或页面规划，但这会让 page identity、update 粒度和 runtime state 一起漂移，不适合当前仓库。

### 3. 页面增强插在 `build_page_context` 之后、`build_section_drafts` 之前

当前页面输入和写盘链路已经比较稳定：

- `build_page_context()` 负责把事实转成页面输入；
- `build_section_drafts()` 负责稳定 section 标题与 `section_id`；
- `render_page_bundle()` 负责 managed marker 和 Markdown 组装。

因此本迭代不改动这三个层级的职责，而是在中间新增一个轻量的 `PageEnrichmentInput` / `PageEnrichmentResult` 层：

- `PageEnrichmentInput` 从 `PageContext`、`ModuleTree`、`GraphSummary`、page hints 和子页面摘要计算而来；
- `PageEnrichmentResult` 返回“某个 section 的增强正文”“可选 Mermaid fenced block”“使用了哪些 child summaries / graph facts / hints”；
- `build_section_drafts()` 仍然控制 section 标题和稳定 `section_id`，只是 section body 先看 LLM 结果，失败时再回退到 deterministic 模板。

这样做比“让 LLM 直接生成整页 Markdown”更适合当前 runtime，因为 [`SectionDraft`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/sections.rs) 已经是 `update` 和 section-level cache 的稳定粒度。

### 4. 增强生成采用叶子优先顺序，并复用当前页面树而不是重建新 DAG

当前 `plan_pages_with_graph()` 已经基于 `ModuleTree` 生成 overview / architecture / module / workflow 页面，且 parent-child 关系稳定。因此迭代 9 不再重新定义新的“文档生成图”，而是按现有页面树和模块树执行：

- 先生成所有 module 页的增强摘要；
- 父 module 页消费子 module 页增强摘要；
- overview / architecture / workflow 页消费顶层 module 摘要、`GraphSummary.detected_processes`、communities 和 cycle warnings。

这借鉴 CodeWiki 的 leaf-first 动态规划，但避免把 spec-wiki 拉向 CodeWiki 那种“模块文档就是唯一中间产物”的形态。对 workflow 页，本仓库已经有 [`detect_processes`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/symbol_graph/analyze/processes.rs) 结果，所以直接消费 graph summary 比重建文档树更合适。

### 5. 复用现有缓存与哈希模型，只把 LLM 输出和 prompt 结果追加到现有链路

本迭代不新增新的主状态表，而是复用现有持久化链：

- `llm_cache` 负责缓存 prompt 级结果，键由 `prompt_type + prompt_version + model_id + normalized_input_hash` 组成；provider 直连和 Agent bridge 只要模型标识相同，就共享同一层 prompt cache。
- `page_generation_cache` 继续缓存最终 `SectionDraft[]`，因此 LLM 增强后的 section body 仍走现有页级缓存。
- `compute_page_input_hash()` 需要把 `generation_mode`、prompt 版本、page hints 哈希和模型标识纳入增强模式下的输入摘要；deterministic 模式保持原样。

替代方案是增加独立的“页面增强表”或“摘要表”，但这会扩大状态层改动，并让 `update` 需要维护更多映射。现有 `llm_cache + page_generation_cache` 足以承载本轮变化。

### 6. Mermaid 图采用“受控提示 + 轻量语法守卫”，不引入外部 fixer 作为硬依赖

deepwiki-rs 的 [`fixer.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/outlet/fixer.rs) 采用外部 `mermaid-fixer` 二进制自动修复图，但当前 spec-wiki 的分发目标仍是 Windows + CodeBuddy，额外引入发布依赖会抬高打包和验收复杂度。因此本迭代先采用更保守的做法：

- 只允许生成 `flowchart` / `graph TD` 这类受控 Mermaid 形态；
- core 在写入前做轻量语法守卫，例如 fenced block、首行类型、非法字符和空图检测；
- 守卫失败时直接回退为普通说明文字，而不是让整页失败。

这比外接 fixer 更弱，但与当前 runtime 和打包边界更匹配；若后续确实需要高级修复，再在迭代 10+ 引入可选 fixer。

### 7. LLM 调用采用“按深度分组的有限并行 + 明确预算”，而不是无上限 page 级并发

当前 [`prepare_page_artifacts()`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/page_render.rs) 已经支持 bounded parallel page pre-render，但这套并行模型默认适用于纯 CPU deterministic 任务。LLM 调用如果完全复用该策略，会带来两个问题：

- 父页可能先于子页请求，拿不到子摘要；
- 调用顺序和响应时间会影响缓存命中与页面抖动。

因此本迭代改为：

- Uncertainty Gate 严格串行或极低并发执行，并受 `llm.max_calls` 限制；
- Content Enrichment 按模块树深度分组：同深度页可有限并行，不同深度保持父页晚于子页；
- 新增 `llm.parallel_requests` 配置项，默认只影响 provider 直连路径；同一深度内最多并发这么多真实 provider 请求；
- Agent bridge 在当前迭代继续保持串行请求。原因不是“不想并行”，而是当前 [`json_rpc.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/transport/json_rpc.rs) 的 session 实现本质上仍是“一问一答”流控：core 发出一个 `llm_request` 后会同步等待同 `request_id` 的响应，[`invokeCore.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/runtime/invokeCore.ts) 也按事件顺序串行消费。若在这一层强行并发，会把协议、回写顺序和错误收口一起复杂化。
- progress 新增 `llm_uncertainty_gate` 和 `llm_enrichment` 两个稳定 phase，便于观察成本和回退路径。

并行实现边界收敛为：

- 先按深度分组并顺序处理深度层；
- 每层先构造稳定 `PageContext` / `PageEnrichmentInput`；
- 已命中 `llm_cache` 的页面直接短路，不占并行槽；
- 仅对未命中缓存且预算允许的页面发起真实 provider 请求；
- 真实请求完成后再串行写回 `llm_cache` 与页面产物，避免 SQLite 写竞争和 page artifact 抖动。

### 7.1 Uncertainty Gate 优先做“同类型批量判断”，而不是跨阶段 mega prompt

`uncertainty_gate` 当前有四类输入：

- 文件角色兜底：[`scanner.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/scanner.rs)
- 顶层目录晋升候选：[`hierarchy.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/hierarchy.rs)
- `module_kind` 兜底：[`hierarchy.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/hierarchy.rs)
- 低置信度依赖边：[`hierarchy.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/hierarchy.rs)

这些判断虽然都属于“不确定性补判”，但它们并不共享同一时点的数据：

- `file_purpose` 发生在 scan 阶段，只有文件路径、语言和源码预览；
- 顶层目录晋升要等扫描统计聚合完成；
- `module_kind` 要等模块候选已经形成；
- `dependency_edge` 要等模块映射和依赖线索已经可用。

因此不适合把四类判断揉成一个跨阶段 mega prompt。更合理的方式是：

- 保持分阶段执行；
- 在每个阶段内部，把同 schema 的候选项批量送进 LLM；
- 仍按单条候选写入/读取 `llm_cache`，避免“一项变化打爆整批缓存”。

本轮优先优化顺序是：

- 先把 `file_purpose` 从逐条请求改成批量判断，因为它是 scan 阶段最容易形成高频小请求的热点；
- 再考虑把顶层目录晋升候选和低置信度 dependency edge 做成同样的批量模式；
- `module_kind` 保留为可选后续项，因为候选量通常远少于 `file_purpose`。

### 8. provider 直连优先级在 workflow 内部选择，而不是让 transport 决定

provider 直连和 Agent bridge 都服务同一组 prompt，因此优先级判断放在 workflow 内部最合适：

- workflow 先读取 steering + `wiki.dev.yaml` 合成后的 LLM 配置，并把 `llm.model` 解析成 `provider/model`；
- 若解析出的 provider/model 在本地 `llm.providers` 中完整可用，则构造 provider-backed `LlmService`；
- 否则再消费 transport 注入的 Agent bridge service；
- 两者都不可用时，`LlmRuntime` 自动退化成 deterministic。

这样 transport 仍然只负责“有无 Agent bridge”，不会反向承担 provider 配置解析，也避免 TS 层和 Rust 层重复实现优先级规则。

### 9. reference 报告脚本必须消费现有 progress 事件，而不是继续黑盒等待

当前 [`scripts/collect-reference-project-reports.mjs`](E:/project/!byAI/spec-wiki/scripts/collect-reference-project-reports.mjs) 直接 `execFileSync` 调 `wiki-core --json`，这会导致两个问题：

- 用户在长时间的 provider 增强阶段完全看不到 `scan -> parse_symbols -> llm_enrichment -> render_pages` 的真实进度；
- 即使底层 workflow 已经通过 [`WorkflowReporter`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/progress.rs) 输出稳定 phase，测试脚本也没有消费这些事件，导致可观测性丢失。

因此脚本侧改为：

- 对 reference 项目集使用流式 NDJSON 调用，实时打印项目名、phase、耗时与关键计数；
- 支持 `--jobs <N>`，允许多个项目并行收集报告，同时每条日志必须带项目前缀，避免输出不可读；
- 默认仍保持保守项目并行度，避免和 `llm.parallel_requests` 叠加后把 provider 打满；
- 报告 Markdown 仍按项目单独写入当前 change 目录，日志可见性只是执行期能力，不改变最终报告结构。

## Risks / Trade-offs

- [Risk] JSON IPC 从单向输出扩到可选双向会话，transport 实现复杂度上升  
  → Mitigation：把新协议做成显式协商模式；未协商时沿用当前一发一收行为，避免一次性打破现有 Agent 和脚本。

- [Risk] LLM 输出不稳定，可能导致页面内容抖动或 hash 频繁变化  
  → Mitigation：所有 prompt 输入做稳定排序；响应要求结构化 JSON；解析失败回退 deterministic；哈希中显式纳入 prompt version 与 model id。

- [Risk] Uncertainty Gate 过度触发会把扫描/建树变成高成本链路  
  → Mitigation：只在现有 deterministic 临界点触发；缓存命中优先；`llm.max_calls` 达上限后强制回退。

- [Risk] LLM 增强后页面更丰富，但 update 可能放大受影响范围  
  → Mitigation：增强输入仍以 page/module scope 为边界；只有事实哈希变化的页面才重新请求；父页只依赖子摘要而不依赖子页完整正文。

- [Risk] Mermaid 图质量不稳定，容易污染正式页面  
  → Mitigation：把 Mermaid 当作可选增强；守卫失败就降级成普通 section 文本，不影响最终页面写盘。

## Migration Plan

1. 先在 core 中引入 `llm_cache` 读写、steering 配置、Uncertainty Gate / Content Enrichment 输入模型和 deterministic fallback，但默认关闭。
2. 扩展 JSON IPC 协商字段与 `llm_request / llm_response` 会话能力；未启用该字段时保持现有 `progress / result / error` 兼容。
3. 在 CodeBuddy Agent 中新增可选 LLM bridge，默认允许返回 `llm_unavailable`，确保旧宿主或未配置 provider 时仍能完成 deterministic workflow。
4. 把 overview / architecture / module / workflow 页逐步接到增强层，并同步补齐 SQLite 缓存、项目集验证和生命周期验证。
5. 若线上或验收发现增强链路不稳定，可通过 steering 关闭 `llm` 开关或由 Agent 不声明 bridge capability，立即回退到现有 deterministic 行为。

## Open Questions

- CodeBuddy 宿主最终提供的 LLM 调用能力是“Agent 自带 provider 配置”还是“宿主暴露统一 callback”？这会影响 `llm_request` 事件的最小字段集。
- `pages.hints` 是否只按 `page_type` 生效，还是需要进一步支持 `page_id` / `module_id` 定位？当前实现更适合先按 `page_type` 收口。
- 对于 update 中“子页摘要变了但父页 deterministic facts 未变”的场景，是否要把父页自动纳入 affected set？当前设计倾向于纳入，否则父页增强正文会滞后。
