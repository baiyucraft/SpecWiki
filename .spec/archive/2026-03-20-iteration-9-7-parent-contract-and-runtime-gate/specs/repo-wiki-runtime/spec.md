## ADDED Requirements

### Requirement: runtime 必须持久化 parent compose readiness 与 gate 状态
系统 MUST 在正式 runtime/state/cache 主链内持久化 parent compose readiness 与 gate 状态，用于表达某个 unit 当前处于 `research-ready`、`compose-ready`、`assemble-ready` 还是被 soft blocker 卡住。该持久化 contract MUST 至少覆盖 `unit_id`、`unit_type`、当前阶段、缺失依赖、阻塞原因、更新时间和 workflow 级摘要。系统 MUST NOT 再把“research 已存在但 compose/assemble 未完成”的状态仅留给临时内存或通过 cache 缺失间接猜测。

#### Scenario: research 完成但 compose 不满足时仍有正式 gate 状态
- **WHEN** 某个 parent unit 已完成 research，但 child rollup、provider 输出或 compose 输入尚未满足
- **THEN** runtime MUST 持久化该 unit 当前不是 `compose-ready`
- **THEN** runtime MUST 记录该 unit 的阻塞原因与缺失依赖
- **THEN** 系统 MUST NOT 只留下 `research_cache` 而没有任何 gate/readiness 说明

#### Scenario: hard interruption 与 readiness 状态分离持久化
- **WHEN** workflow 因 provider 调用失败、compose 失败或写盘失败而中断
- **THEN** 系统 MUST 继续持久化 hard interruption checkpoint
- **THEN** 系统 MUST 同时保留当前 workflow 或 unit 的 readiness/gate 摘要
- **THEN** 系统 MUST 能区分“发生错误中断”和“尚未达到 compose-ready”这两类状态

### Requirement: parent contract 摘要必须进入可复用 runtime/cache 主链
系统 MUST 将 parent compose 需要的最小 contract 摘要写入现有 runtime/cache 主链，以支持 `update`、`rebuild`、reference 报告与 runtime 诊断复用。对 parent unit 而言，摘要 MUST 至少覆盖 `child_unit_ids`、`child_page_ids`、child digest 引用、citation/diagram 摘要引用与 readiness 状态。系统 MUST NOT 继续只为这类页面写入最小 `source_ids`。

#### Scenario: parent page context 能回溯 child contract 输入
- **WHEN** 系统为某个 parent unit 写入正式 runtime cache
- **THEN** 后续读取方 MUST 能从 cache/state 中回溯该页面对应的 child unit 集合与最小 compose contract 摘要
- **THEN** 报告、诊断和增量更新 MUST 不需要依赖最终 Markdown 反推这些输入
