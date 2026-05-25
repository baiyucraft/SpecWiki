## ADDED Requirements

### Requirement: workflow 主链必须在 render 前引入 family planning、leaf-first research 与 compose
系统 MUST 在 `build_contexts -> plan_pages -> render_pages` 主链中加入 family planning、leaf-first research 与 compose 阶段。正式顺序 MUST 至少体现：先规划 family/module/topic 页面集合，再为叶子页组装 dossier 并执行 research，随后让父页消费子页结果完成 compose，最后再由 deterministic renderer 落盘。

#### Scenario: 叶子页先研究、父页后组合
- **WHEN** workflow 处理同时包含 family、module 和 topic 的页面树
- **THEN** 叶子 family child、叶子模块页和高置信 topic 页 MUST 先完成 research
- **THEN** 父页 MUST 在子页结果可用后再执行 compose

#### Scenario: workflow 不得绕过 planner 直接拼 family 页面
- **WHEN** 系统为 docs-heavy 或 platform 仓库生成 family 页面
- **THEN** 这些页面 MUST 由正式 planner 规划出来
- **THEN** render 阶段不得绕过 planner 临时创建额外页面

### Requirement: workflow 必须在 LLM 失败时中断并保存检查点
系统 MUST 在 Research 或 Compose 阶段的 LLM 调用失败时立即中断 pipeline，保存已完成的中间结果和中断位置到 `pipeline_checkpoint` 表。系统 MUST NOT 静默跳过失败的 LLM 调用或退化到模板填充。

#### Scenario: LLM 调用失败触发 pipeline 中断
- **WHEN** `research_system` / `research_domain` / `research_unit` 或 compose 阶段的 LLM 调用失败
- **THEN** workflow MUST 立即中断，不继续处理后续知识单元
- **THEN** 已完成的 research / compose 结果 MUST 已被写入缓存
- **THEN** `pipeline_checkpoint` MUST 被写入 SQLite，包含 `facts_input_hash`、中断阶段和目标 ID

### Requirement: workflow 必须在入口处检查并恢复检查点
系统 MUST 在 init / rebuild / update 的入口处检查 `pipeline_checkpoint` 表。若检查点有效（`facts_input_hash` 匹配），MUST 从中断处恢复而非从头开始。Pipeline 正常完成后 MUST 清除检查点记录。

#### Scenario: 从中断处恢复 pipeline
- **WHEN** workflow 入口检测到有效的 `pipeline_checkpoint` 且 `facts_input_hash` 匹配当前 Facts
- **THEN** pipeline MUST 跳过已缓存的 research / compose 步骤
- **THEN** pipeline MUST 从 `interrupted_stage` + `interrupted_target_id` 指定的位置继续执行

#### Scenario: Facts 变化导致检查点失效
- **WHEN** workflow 入口检测到 `pipeline_checkpoint` 但 `facts_input_hash` 不匹配
- **THEN** 系统 MUST 丢弃检查点记录
- **THEN** pipeline MUST 从头开始完整执行
