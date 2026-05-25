## Why

当前仓库已经完成 `scan -> parse_symbols -> resolve/analyze symbol graph -> module_tree -> page_render` 的 deterministic 主链，但真实代码仍停在“事实已收集、表达仍偏骨架”的状态：[`crates/wiki-core/src/generation/context.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/context.rs) 主要把事实压成字符串列表，[`crates/wiki-core/src/generation/sections.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/sections.rs) 仍以固定模板把这些列表渲染成项目符号，[`crates/wiki-core/src/storage/sqlite_store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/storage/sqlite_store.rs) 虽已预留 `llm_cache`，但还没有任何实际消费链路。继续停在这一层，迭代 8/8.5 已经建立的 graph facts 很难真正转化成高信息密度 Wiki 页面，也无法达到“第一个可用版本”的目标。

上游真实源码已经证明“先确定事实、再做受控增强”的路径是可行的。deepwiki-rs 的 [`workflow.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/workflow.rs) 明确把生成链拆成 `preprocess -> research -> compose -> outlet`，并通过 [`context.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/context.rs) 与 memory 在阶段间传递上下文；CodeWiki 的 [`documentation_generator.py`](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/documentation_generator.py) 则按叶子模块优先、父模块消费子模块文档摘要的方式做自底向上生成；GitNexus 的 [`pipeline.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/pipeline.ts)、[`community-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/community-processor.ts) 与 [`process-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/process-processor.ts) 说明 graph facts 必须先稳定，再交给更高层消费。基于这些真实实现，迭代 9 最合理的目标不是把 LLM 塞进任意一层，而是把“可缓存、可回退、可追溯”的 LLM 增强接到现有 deterministic 主链之后，并补齐当前 binary/Agent 边界上缺失的可选 LLM 桥接。

## What Changes

- 引入新的 `wiki-llm-enhancement` 能力：在不破坏 deterministic 主链的前提下，为 scanner / hierarchy / graph summary / page generation 提供可缓存的 LLM 辅助判断与内容增强链路。
- 在 core 中定义统一的 LLM assist 契约，覆盖两类场景：
  - Uncertainty Gate：只在 deterministic 规则无法稳定收敛时触发，例如 `FilePurpose` 兜底、顶层目录晋升临界值、`module_kind` 兜底以及低置信度跨模块依赖语义。
  - Content Enrichment：在 `build_page_context` 之后、`render_page_bundle` 之前，对 overview / architecture / module / workflow 页面生成高信息密度说明、graph 叙述和 Mermaid 图草稿。
- 升级页面生成链：从“`PageContext -> 固定 section 模板`”升级为“`PageContext -> EnrichmentInput -> LLM/Deterministic SectionDraft`”，保留现有稳定 `page_id`、`section_id`、managed marker、user section 保留与增量刷新语义。
- 引入叶子优先的增强生成顺序：模块页先生成，父模块页和 overview / architecture / workflow 页消费子模块摘要、graph summary 与 repo facts，而不是对整个仓库一次性生成大页。
- 把 `llm_cache` 从预留表升级为真实缓存层，缓存 prompt 输入哈希、prompt 类型、模型标识、响应内容、TTL 与命中状态；相同输入重复运行时不得重复调用 LLM。
- 扩展 steering 配置，新增 `llm.*` 控制项和页面增强提示项，使仓库可以显式控制是否启用 LLM、调用上限、页面增强强度与补充提示。
- 在 provider 直连路径上补齐可配置的 LLM 并行度控制：同深度独立页面允许有限并行增强，请求并行度由 steering / `wiki.dev.yaml` 显式配置；Agent bridge 继续作为串行兜底，避免把当前 NDJSON session 强行扩成无界多路复用。
- 把 `uncertainty_gate` 的高频同类型判断从“逐条请求”升级为“同类型批量判断优先”：特别是 `file_purpose`、顶层目录晋升候选和低置信度依赖边，应优先按批次送进 LLM，以减少 provider 往返次数；不同阶段、不同类型的判断仍保持分阶段执行，不合并成跨阶段 mega prompt。
- 把 LLM 调用入口扩展成“两级优先级”模型：如果 core 侧已配置可用的供应商 API，则由 `wiki-core` 直接调用；若未配置供应商 API，再退回现有 JSON IPC / Agent bridge 由 CodeBuddy 等宿主代调用。Agent 仍只做协议桥接和 provider 调用，不承载 Wiki 业务规则。
- 为开发调试新增 repo 根本地 `wiki.dev.yaml` 配置入口。该文件只服务本地 dev/provider 配置，加载时覆盖 `.wiki/wiki.steering.yaml` 中对应的 LLM 字段，并且不得放在 `.wiki/` 下面，以避免被 init/rebuild 的 runtime 清理误删。provider 配置采用 `llm.providers.<provider>.models.<model>` 结构，顶层 `llm.model` 统一使用 `provider/model` 选择具体模型。
- 补齐页面图表达：让迭代 8 的 communities / processes / cycle warnings 真正进入正文 section 和 workflow / architecture 页面，同时增加基础 Mermaid 图草稿与 fail-soft 修复路径。
- 扩展验证链路，覆盖 LLM 开/关、缓存命中、调用上限、provider 有限并行、回退路径、页面稳定性、IPC 桥接、项目集全量 `init` 分析，以及一次单独的注释规范检查任务。
- 升级 reference 项目报告脚本：执行过程中必须能看到项目级与 phase 级进度，并支持通过 `--jobs` 调整项目并行度，避免长时间黑盒等待。

## Capabilities

### New Capabilities
- `wiki-llm-enhancement`: 定义 Uncertainty Gate、页面内容增强、Mermaid 草图生成、缓存与 deterministic fallback 的统一契约。

### Modified Capabilities
- `repo-wiki-workflow`: workflow 从纯 deterministic render 升级为“deterministic facts + optional LLM enhancement + stable write-back”的可回退主链。
- `repo-wiki-runtime`: 正式 Wiki 页面需要支持 LLM 增强 section 与 diagram，同时保持稳定 page/section identity、managed marker 和 user section 保留。
- `sqlite-cache-storage`: `llm_cache` 从预留 schema 升级为真实缓存能力，并参与 init/update/rebuild 的一致性收口。
- `wiki-steering-config`: steering 需要新增 `llm` 配置块、provider 直连配置和 dev 覆盖入口，而不是只控制扫描和页面合并。
- `workflow-progress-streaming`: 长流程 JSON IPC 需要支持可选的 LLM 协商/事件扩展，同时保持现有 progress/result/error 兼容语义。
- `codebuddy-agent-integration`: CodeBuddy Agent 需要在保持 thin Agent 边界的前提下桥接可选 LLM 请求，而不是只消费 progress 与最终结果。
- `workflow-verification`: 验证必须覆盖 LLM 开关、缓存、回退、IPC 桥接、页面稳定性、项目集分析和注释规范检查。

## Impact

- 主要影响 `crates/wiki-core/src/generation/*`、`crates/wiki-core/src/workflows/{init,update,rebuild}.rs`、`crates/wiki-core/src/domain/{context,steering}.rs`、`crates/wiki-core/src/repo/{scanner,hierarchy}.rs`、`crates/wiki-core/src/storage/{sqlite_store,state_store}.rs`、`crates/wiki-core/src/transport/{dto,json_rpc}.rs` 与 `agents/codebuddy/src/runtime/*`。
- 需要新增 LLM assist 输入/输出模型、页面增强缓存与可选协议桥接，但不会把 Wiki 业务规则迁移到 Agent，也不会引入 deepwiki-open 式的 request-scoped RAG 主链。
- 页面正文和长流程协议会增加新的增强能力与可观测事件，但必须保持“未启用 LLM 时现有 deterministic 行为不退化”的兼容约束。
