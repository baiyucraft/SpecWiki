## ADDED Requirements

### Requirement: runtime 必须持久化 family 页面与 parent-child compose 结果
系统 MUST 在现有 runtime/state/cache 主链内持久化 family index、family child、child page digest 和 parent compose 输入。family 页 MUST 与其他正式页面共用同一套 `page_id`、managed section、cache 和增量更新 contract，而不是引入新的 sidecar 目录。

#### Scenario: family 页面进入正式 runtime
- **WHEN** planner 生成 family index 或 family child 页面
- **THEN** runtime MUST 为其写入正式页面状态、缓存和 managed sections
- **THEN** 这些页面 MUST 与 overview、module、topic 一样进入 `.wiki/*.md`

#### Scenario: parent-child compose 结果进入现有 cache/state
- **WHEN** 系统生成 child page digest 或 parent compose 输入
- **THEN** 系统 MUST 将这些对象写入现有 runtime/cache 主链
- **THEN** `update` 与 `rebuild` MUST 能基于这些结果进行增量复用

### Requirement: runtime 必须持久化 pipeline 中断检查点
系统 MUST 在 SQLite state 中维护 `pipeline_checkpoint` 表，用于存储 pipeline 中断时的进度信息。检查点 MUST 包含 `checkpoint_id`、`facts_input_hash`、`interrupted_stage`、`interrupted_target_id`、`error_message` 和 `created_at`。Pipeline 正常完成后 MUST 清除检查点。

#### Scenario: pipeline 中断时写入检查点
- **WHEN** Research 或 Compose 阶段因 LLM 调用失败而中断
- **THEN** 系统 MUST 写入一条 `pipeline_checkpoint` 记录
- **THEN** 记录 MUST 包含当前 Facts 输入的哈希、中断阶段和目标 ID

#### Scenario: pipeline 完成时清除检查点
- **WHEN** pipeline 所有阶段正常完成
- **THEN** 系统 MUST 删除 `pipeline_checkpoint` 表中的所有记录
