## ADDED Requirements

### Requirement: LLM 增强必须支持 leaf-first/source-fed/research-first compose
系统 MUST 让 LLM 增强正式支持 `leaf-first/source-fed/research-first compose` 路径。叶子 family 页、叶子模块页和高置信 topic 页 MUST 优先消费一手源码、API/config/docs surface；父页 research/compose MUST 优先消费子页结果。系统不得继续只依赖 page-level summary patch 作为主要增强模式。

#### Scenario: 叶子页优先消费一手材料
- **WHEN** 当前页面为叶子 family child、叶子模块页或高置信 topic 页
- **THEN** LLM 输入 MUST 优先包含一手源码、API/config/docs surface 和精准证据
- **THEN** 系统不得先只给模型父页摘要再反推叶子页内容

#### Scenario: 父页优先消费子页结果
- **WHEN** 当前页面为 family index、overview、architecture 或父模块页
- **THEN** LLM 输入 MUST 优先消费子页结构化结果和 digest
- **THEN** 系统不得把相同的一手材料再次作为父页主要输入

### Requirement: Research 和 Compose 层必须接入 LLM，不支持退化
系统 MUST 要求 Research 层（R1 system / R2 domain / R3 unit）和 Compose 层全部通过 LLM 生成结构化研究材料和页面内容。系统 MUST NOT 提供 deterministic fallback 路径（模板填充）。当 LLM 未配置时，pipeline MUST 返回错误而非退化到低质量模板输出。

#### Scenario: 无 LLM 配置时 pipeline 拒绝运行
- **WHEN** steering 中 `llm.enabled = false` 或未配置 provider
- **THEN** `run_compose_pipeline()` MUST 返回错误（`io::Error`）
- **THEN** 系统 MUST NOT 使用模板填充生成低质量页面

#### Scenario: LLM 调用失败时 pipeline 立即中断
- **WHEN** Research 或 Compose 阶段的 LLM 调用因网络错误、API 限流或超时而失败
- **THEN** pipeline MUST 立即中断，不继续后续步骤
- **THEN** 系统 MUST 保存已完成的中间结果到 `research_cache` / `page_drafts` 缓存
- **THEN** 系统 MUST 写入 `PipelineCheckpoint` 到 SQLite `pipeline_checkpoint` 表，记录中断位置和错误信息

### Requirement: pipeline 必须支持中断恢复
系统 MUST 在 init / rebuild / update workflow 入口处检查 `pipeline_checkpoint` 表。若存在有效检查点且 `facts_input_hash` 匹配当前 Facts 层输出，系统 MUST 从中断处恢复，跳过已缓存的 research / compose 结果。Pipeline 正常完成后 MUST 清除检查点。

#### Scenario: 从检查点恢复并跳过已完成步骤
- **WHEN** `pipeline_checkpoint` 存在且 `facts_input_hash` 匹配
- **THEN** pipeline MUST 从 `interrupted_stage` + `interrupted_target_id` 处恢复
- **THEN** 已缓存的 `research_cache` 和 `page_drafts` MUST 被直接复用

#### Scenario: Facts 输入变化时丢弃检查点
- **WHEN** `pipeline_checkpoint` 存在但 `facts_input_hash` 不匹配
- **THEN** 系统 MUST 丢弃检查点，从头开始整个 pipeline

#### Scenario: pipeline 正常完成后清除检查点
- **WHEN** pipeline 所有阶段正常完成
- **THEN** 系统 MUST 清除 `pipeline_checkpoint` 表中的记录

### Requirement: 清理旧 LLM 子开关和废弃参数
系统 MUST 移除 `LlmConfig` 中的 `content_enrichment_enabled` / `session_enabled` / `uncertainty_gate_enabled` / `page_enrichment_max_input_tokens` / `page_enrichment_parallel_requests` / `session_max_context_tokens` / `session_max_recent_turns` / `uncertainty_gate_max_input_tokens` / `uncertainty_gate_parallel_requests` 等废弃字段。LLM 调用限制 MUST 统一为 `max_research_calls` 和 `max_compose_calls`。

#### Scenario: Steering 配置只保留简化后的 LLM 字段
- **WHEN** 用户编写 `wiki.dev.yaml` 或 `wiki.steering.yaml`
- **THEN** LLM 配置 MUST 只包含 `enabled` / `model` / `max_research_calls` / `max_compose_calls` / `cache_ttl_seconds` / `cache_mode` / `allow_mermaid` / `providers` 等字段
- **THEN** 旧的 `content_enrichment_enabled` 等字段 MUST 被忽略或报 warning
