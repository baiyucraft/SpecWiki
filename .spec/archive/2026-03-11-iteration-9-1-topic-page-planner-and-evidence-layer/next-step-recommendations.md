# 9.1 之后的下一步建议

这份结论来自对当前仓库和四个参考项目真实源码的交叉阅读，不是从 README 或产品说明倒推出来的。

补充对照报告见：

- `.spec/changes/iteration-9-1-topic-page-planner-and-evidence-layer/llm-reference-code-report.md`

## 结论

下一步不该继续主要调 prompt，而应该优先做三件事：

1. 引入 `Topic Insight / TopicDossier` 中间层。
2. 让父页真实消费子页结果，而不是重复消费底层 facts。
3. 先精炼 graph-derived inputs，再继续扩 planner 和 Mermaid。

## 依据

### 1. `deepwiki-rs` 说明专题页应该先有独立 insight，再进入 compose

参考代码：

- `tmp/upstream/deepwiki-rs/src/generator/workflow.rs`
- `tmp/upstream/deepwiki-rs/src/generator/compose/mod.rs`
- `tmp/upstream/deepwiki-rs/src/generator/compose/agents/key_modules_insight_editor.rs`

观察：

- 它的生成链不是 `scan -> write pages` 一步到位，而是 `preprocess -> research -> compose -> output`。
- `KeyModulesInsightEditor` 会先产出多个 insight report，再把它们编进 `DocTree`。

对当前仓库的含义：

- 现在 `planner.rs` 里的 `topic` 还是从 seed 直接升页，缺少一个“专题 dossier / insight”层。
- 应该在 `RepoContext / ModuleContext -> PlannedPage` 之间补一个稳定中间对象，承载 source cluster、process、cross-module edges、核心 symbol 和 summary facts。

### 2. `CodeWiki` 说明父页应该消费子页产物，而不是重扫底层事实

参考代码：

- `tmp/upstream/codewiki/codewiki/src/be/documentation_generator.py`
- `tmp/upstream/codewiki/codewiki/src/be/cluster_modules.py`
- `tmp/upstream/codewiki/codewiki/src/be/agent_orchestrator.py`

观察：

- `documentation_generator.py` 明确是 leaf-first。
- 父模块文档建立在子模块文档结果之上，而不是简单把源码再解释一遍。
- `cluster_modules.py` 的拆分粒度也不是按目录死拆，而是受体量和复杂度约束。

对当前仓库的含义：

- `overview / architecture / module / topic` 父页需要显式消费 child page 的 summary、evidence rollup 和 diagram inputs。
- planner 也应该引入复杂度驱动的拆分阈值，而不是只靠固定 topic seed。

### 3. `deepwiki-open` 说明 evidence layer 应该升级成可引用出处层

参考代码：

- `tmp/upstream/deepwiki-open/api/rag.py`
- `tmp/upstream/deepwiki-open/api/websocket_wiki.py`
- `tmp/upstream/deepwiki-open/api/api.py`

观察：

- 它的消费层不是只读 Markdown，而是依赖带稳定来源的 retrieval context。
- `websocket_wiki.py` 会按 `file_path` 组织 retrieved documents，并明确约束“不要编造 citation”。

对当前仓库的含义：

- 9.1 的 `evidence_groups` 已经有雏形，但还需要继续升级成可引用结构。
- 后续应为 evidence 补齐 `source_path`、`symbol/line span`、`evidence_type`、`topic_key/diagram_key` 这类稳定字段。

### 4. `GitNexus` 说明 planner 上限取决于图事实质量，不是模板本身

参考代码：

- `tmp/upstream/GitNexus/gitnexus/src/core/ingestion/pipeline.ts`
- `tmp/upstream/GitNexus/gitnexus/src/core/ingestion/process-processor.ts`
- `tmp/upstream/GitNexus/gitnexus/src/core/ingestion/community-processor.ts`
- `tmp/upstream/GitNexus/gitnexus/src/core/ingestion/import-processor.ts`
- `tmp/upstream/GitNexus/gitnexus/src/core/ingestion/call-processor.ts`

观察：

- 它把 structure/import/call/community/process 拆成多阶段处理。
- 对 process、community 和 edge confidence 都有明显的降噪与去重步骤。

对当前仓库的含义：

- 现在 topic/evidence/diagram 的上限，主要仍受 `GraphSummary` 粗糙度限制。
- 更值得优先补的是 entry-point scoring、process dedupe、community 降噪、import/call confidence 分层。

## 建议顺序

1. `Topic Insight / TopicDossier`
2. parent page consume child page
3. graph summary refinement
4. evidence citation strengthening

## 下一步清单

下面这份清单按“下一轮最值得做什么”收敛，优先级从高到低排序。

### P0：先改生成链位置，不再主要调 prompt

- [ ] 在 `facts -> planned page` 之间新增 `TopicDossier / ModuleDossier` 中间层。
- [ ] dossier 至少承载：`key_sources`、`key_symbols`、`cross_module_edges`、`process_candidates`、`evidence_rollup`、`child_page_rollup`。
- [ ] `page_enrichment` 改为优先消费 dossier，而不是直接消费当前 `PageContext` 的轻量摘要。
- [ ] 保留现有 `prompt_type / input_hash / response_schema / cache` contract，不改掉当前结构化调用方式。

### P1：让父页真正建立在子页之上

- [ ] 扩展 child page 产物，除了 `summary` 之外，再稳定输出 `evidence_rollup`、`diagram_rollup`、`key_sources_rollup`。
- [ ] `overview / architecture / module / topic` 父页上下文显式吸收这些 child rollup，而不是只拼 `child_summaries`。
- [ ] planner 增加“父页覆盖阈值”，避免父页和子页重复写同一批 facts。
- [ ] parent page rebuild 传播链同步基于 child rollup 变化触发。

### P2：把 LLM 输入从二手摘要升级到有限的一手材料

- [ ] 为 dossier 增加“高价值源码片段”输入，而不是只给 `源码：...` 这种路径摘要。
- [ ] 优先选入：入口文件、关键调度点、跨模块边界文件、流程终点文件、核心接口/实现对。
- [ ] 增加“依赖代码片段”或“关键 symbol 摘要”，参考 deepwiki-rs 的 `dependency component code snippets` 做法。
- [ ] 控制 token 预算时优先裁掉低价值标签噪音，不优先裁掉关键源码片段。

### P3：继续提高 graph summary 质量，而不是先扩模板

- [ ] entry-point scoring 再细化，减少把普通 helper 当流程入口。
- [ ] process dedupe 再增强，避免同一 entry/terminal 反复产出近似流程。
- [ ] community label 降噪，减少目录名和泛词主导主题页。
- [ ] import/call confidence 分层进入 summary，页面只默认消费高置信度关系。
- [ ] 根级核心源码簇晋升逻辑再加强，避免核心单文件长期埋在 overview 里。

### P3.5：把 `uncertainty_gate` 从串行补判改成“同类型批量 + 有限并行”

- [ ] 不要把所有 gate 跨类型揉成一个 mega prompt，而是按阶段、按同类型批量。
- [ ] `file_purpose` 继续优先做批量化，这是当前最主要的请求大头。
- [ ] `top_level_promotion`、`dependency_edge` 也应支持同类型批量判断。
- [ ] `module_kind` 视候选规模决定是否批量，不强行并到所有类型里。
- [ ] provider 路径下支持有限并行批次，不要求所有批次完全串行。
- [ ] 失败回退和 cache 粒度仍保持单条判断单元，不因为批量化丢失增量复用能力。

### P4：把 evidence layer 升成真正可引用层

- [ ] `PageEvidenceItem` 补齐 `source_path`、`symbol_id`、`start_line`、`end_line`、`evidence_type`。
- [ ] evidence 与 `topic_key / diagram_key / section_title` 建稳定关联。
- [ ] 页面正文、Mermaid、后续 query 都统一复用同一套 evidence identity。
- [ ] 后续如果做 query/RAG，优先复用这层，不另起一套 citation 数据结构。

### P5：验证方式也要跟着改

- [ ] reference 对比不只看页数和标题，还要看父子页是否真正分工。
- [ ] 单独统计“父页重复率”“child rollup 使用率”“Mermaid 命中率”“evidence 覆盖率”。
- [ ] 对 4 个重点 reference 项目单独出差距报告，不和 19 项目基线测试混在一起。
- [ ] debug trace 里补 dossier 输入摘要，便于核对 LLM 到底看到了什么。

### P6：引入 Agent / tool 连续研究会话，但不要直接让 Agent 写最终页

- [ ] 明确保留当前 `scan -> module_tree -> planner -> render` 主链，不要把 `init` 直接改成无限自由对话 agent。
- [ ] Agent session 只作为 `facts/graph/planner -> research session -> structured dossier/result -> deterministic render` 中间研究阶段插入。
- [ ] 在现有单轮 `llm_request -> llm_response` 之上设计 `agent_session_v1`，支持连续多轮会话。
- [ ] 协议建议至少包含：`agent_session_start`、`agent_message`、`agent_tool_call`、`agent_tool_result`、`agent_final`、`agent_abort`。
- [ ] 每个研究会话都应带稳定 `session_id`，便于串联 progress、tool artifact、token usage 和缓存。
- [ ] Agent 的最终产物应是结构化 `TopicDossier / ModuleDossier / PageResearchResult`，而不是直接返回最终 Markdown。
- [ ] 页面落盘仍由 core 的 deterministic renderer 负责，避免把 runtime 完全交给自由对话 agent。
- [ ] 第一轮只在 `topic`、`module` 页试点，不先扩到 `overview / architecture / init` 全链路。

### P7：为 Agent 和 tool 调用定义受控的只读工具面

- [ ] 不先开放泛化 shell/tool 调用，先开放和 core facts 强绑定的只读工具。
- [ ] 第一批工具建议包括：`read_source_snippets(source_ids)`、`get_module_context(module_ids)`、`get_symbol_neighbors(symbol_ids)`、`get_process_trace(process_id)`、`get_page_children(page_id)`、`get_evidence_group(group_id)`、`search_topic_candidates(scope)`。
- [ ] tool 返回应优先复用现有 state/cache/context 数据结构，不平行复制一套 DTO。
- [ ] provider tool-calling 路径与 agent-bridge 路径要共享同一套 tool schema，不要分裂成两套 agent 架构。

### P8：对话上下文应该复用，但复用“结构化沉淀”，不是原始聊天流水

- [ ] 引入可复用上下文层：`repo_context_cache`、`module_dossier_cache`、`page_rollup_cache`、`session_summary_cache`。
- [ ] 同仓库跨页面优先复用 `repo_context` 和 `module_dossier`，不要每页都从零研究。
- [ ] 父页生成时优先复用 child rollup，而不是把子页相关原始对话重新灌给模型。
- [ ] 同一页面多轮 Agent 会话时，保留 `session_summary + recent_turns + tool_artifact_refs`，而不是保留整段历史全文。
- [ ] `recent_turns` 的默认保留窗口可先按 4-8 轮收敛，再结合真实 trace 和模型上下文上限调优。
- [ ] 对话里可复用的应该是：稳定事实、模块 dossier、child rollup、tool 结果引用、已验证结论与排除项。
- [ ] 不应直接复用的包括：原始长对话全文、临时推理过程、试探性假设、已过期的代码细节判断。

### P9：支持可配置的最长上下文上限，并写进 `wiki.dev.yaml`

- [ ] 在 `llm` 配置下新增上下文上限相关字段，优先支持写进 repo 根 `wiki.dev.yaml`。
- [ ] 不建议只配置一个全局上限，建议至少拆成：`uncertainty_gate_max_input_tokens` 和 `page_enrichment_max_input_tokens`。
- [ ] 在真正发请求前先估算当前 prompt 大小，再决定是否压缩、裁剪或降级，而不是固定截断。
- [ ] 裁剪策略应优先保留高价值源码片段、child rollup、关键 evidence，不优先保留低价值标签噪音。
- [ ] 如果后续引入 agent session，再追加 `session_max_context_tokens` 与 `session_max_recent_turns`。

### P9.5：把开关、并行度和缓存策略也拆开配置

- [ ] 将 `uncertainty_gate` 与 `content_enrichment` 拆成独立开关，便于性能对比和 reference 对照。
- [ ] `wiki.dev.yaml` 中支持分别配置不同阶段的并行度，不只保留一个总的 `parallel_requests`。
- [ ] 至少考虑：`uncertainty_gate_parallel_requests` 与 `page_enrichment_parallel_requests`。
- [ ] 在 reference 对比或开发调优时，可以只开 `content_enrichment`，先关 `uncertainty_gate`。
- [ ] provider 路径与 agent session 路径都要遵守同一套上限/并行配置语义。

### P9.6：`init/rebuild` 默认不要隐式清 LLM cache，但必须支持显式 cold-start

- [ ] 当前 runtime 清理策略应与 LLM prompt cache 解耦，避免每次 `init` 默认都冷启动重打所有请求。
- [ ] 如果 `.wiki/` 需要重建，优先保留独立的 LLM cache 或把 cache 放到不随 runtime 一起删除的位置。
- [ ] cache 失效应优先基于 `prompt_type + input_hash + model` 控制，不靠“整目录删除”。
- [ ] 同时必须支持显式 cold-start 模式，用于性能测试、回归测试和 provider 成本观测。
- [ ] 建议通过启动参数或 `wiki.dev.yaml` 配置选择 `preserve / clear / refresh` 之类的 cache 模式。
- [ ] 测试报告应明确区分 cold run 和 warm run，避免把两种结果混在一起比较。

### P10：非 debug 模式下也要持续输出 LLM 成本统计

- [ ] 非 debug 模式下也要统计并输出每次 workflow 的 LLM 输入 token、输出 token、总 token。
- [ ] 统计应支持实时刷新，而不是只在 workflow 结束时打印最终汇总。
- [ ] 最少要在每次真实 LLM 请求完成后实时更新累计值：`input_tokens`、`output_tokens`、`total_tokens`、`request_count`。
- [ ] 如果 workflow 本身有进度流，建议把实时 token 统计挂到同一条 progress/event stream，而不是另开一套不可见日志。
- [ ] 优先消费 provider 返回的 `usage.prompt_tokens / completion_tokens / total_tokens`；缺失时再回退到本地估算。
- [ ] 统计维度至少包括：workflow 总计、按 `prompt_type` 分组、provider/model 维度。
- [ ] 长流程结束时输出总览，避免只有 debug trace 才能看见成本。
- [ ] debug trace 继续保留单请求明细，但普通模式下至少应有 summary。
- [ ] 如果走 agent bridge，也应把宿主最终使用的 token usage 回传给 core，而不是只回 `output` 和 `model`。

### P11：基于真实 trace 反推默认值

- [ ] 以上下文上限默认值为例，优先参考真实 trace，而不是拍脑袋定阈值。
- [ ] 当前 `storybook-20260311-193106` 的真实 provider 请求里，`file_purpose` 最长一次输入约 `140,370` 字符，约 `35k tokens`。
- [ ] 同一次 trace 里，`page_enrichment` 最长一次输入约 `13,678` 字符，约 `3.4k tokens`。
- [ ] 这说明 `uncertainty_gate` 与 `page_enrichment` 的上下文规模不是一个量级，默认配置必须分开。

## 推荐的 9.2 收敛范围

如果下一轮只做一个务实版本，建议 scope 收到下面 3 块：

1. `TopicDossier / ModuleDossier`
2. parent page consume child rollup
3. graph summary refinement（只收 entry/process/community/call-confidence）

这一轮先不要把 query/RAG、全文 citation UI、更多 page type 一起并进来。

如果 9.2 允许多收一点，我建议增加一个小的配套包，但仍然不要扩得太大：

4. `llm context budget + token usage summary`

`agent session` 和 `tool schema` 可以先写设计，不一定在 9.2 同轮完整实现。

## 明确不建议

- 不建议继续主要靠 prompt 微调拉齐 reference。
- 不建议先把 query/RAG 大范围并回 core 主链。
- 不建议仅通过放宽 planner 阈值追求“页数变多”。
