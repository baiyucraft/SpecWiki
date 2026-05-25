## ADDED Requirements

### Requirement: runtime 必须对外投影 production research policy 状态
系统 MUST 将正式 workflow 的 provider policy 结果投影为稳定的 runtime 外部状态。`status`、runtime summary 和 gate/blocker 摘要 MUST 至少能区分 `needs_update`、`runtime_incomplete` 和 `blocker`，并提供足够的 blocker hint / recommended action，避免调用方继续依赖内部日志文本推断当前仓库到底是变脏、未完成还是被 provider 卡住。

#### Scenario: status 区分 runtime_incomplete 与 blocker
- **WHEN** 当前仓库 facts snapshot 已就绪，但 research/compose 尚未完成
- **THEN** `status` MUST 返回 `runtime_incomplete`
- **THEN** 若同时存在 provider blocker，`status` MUST 额外标记 blocker，而不是把两者折叠成统一成功态

#### Scenario: status 为 provider blocker 提供推荐动作
- **WHEN** 正式 workflow 因 provider 缺失、provider 不可用或 provider failure 被阻断
- **THEN** runtime summary MUST 提供 blocker 摘要与推荐动作
- **THEN** 调用方 MUST 不需要读取完整 debug trace 才能知道下一步应该补 provider、重试还是切到显式开发模式

## MODIFIED Requirements

### Requirement: runtime 必须持久化 pipeline 中断检查点
系统 MUST 在 SQLite state 中维护 `pipeline_checkpoint` 表，用于存储 pipeline 中断时的进度信息。检查点 MUST 包含 `checkpoint_id`、`facts_input_hash`、`interrupted_stage`、`interrupted_target_id`、`error_message` 和 `created_at`。当中断原因来自 provider 缺失、provider 不可用、provider research 失败或 compose 失败时，系统 MUST 将这些失败归入正式 checkpoint 语义，并与 runtime gate/blocker 保持可关联。Pipeline 正常完成后 MUST 清除检查点。

#### Scenario: provider blocker 写入检查点
- **WHEN** 正式 workflow 在进入 Research 或 Compose 前发现 provider 缺失或 provider 不可用
- **THEN** 系统 MUST 写入一条 `pipeline_checkpoint` 记录
- **THEN** 记录 MUST 包含当前 Facts 输入的哈希、中断阶段和错误摘要

#### Scenario: provider research 或 compose 失败时写入检查点
- **WHEN** Research 或 Compose 阶段因 provider 调用失败、中断或返回非法结构而终止
- **THEN** 系统 MUST 写入一条 `pipeline_checkpoint` 记录
- **THEN** 记录 MUST 能和对应 unit/runtime blocker 关联

#### Scenario: pipeline 完成时清除检查点
- **WHEN** pipeline 所有阶段正常完成
- **THEN** 系统 MUST 删除 `pipeline_checkpoint` 表中的所有记录
