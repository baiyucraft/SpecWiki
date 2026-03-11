## ADDED Requirements

### Requirement: `init`、`update` 与 `rebuild` 必须支持可回退的 LLM 增强阶段
系统 MUST 在保持现有 deterministic facts 主链的前提下，为 `init`、`update` 和 `rebuild` 增加可选的 `llm_uncertainty_gate` 与 `llm_enrichment` 阶段。`llm_uncertainty_gate` MUST 发生在 scanner / hierarchy / 低置信度依赖语义判定期间；`llm_enrichment` MUST 发生在 page context 已稳定、正式写盘之前。无论任一阶段是否启用、命中缓存或回退，workflow 的最终写盘结果都 MUST 保持可追溯且可落回 deterministic 内容。

#### Scenario: init 在 facts 稳定后执行页面增强
- **WHEN** 用户执行 `init`，且当前运行环境已协商开启 LLM 增强
- **THEN** 系统 MUST 先完成 deterministic 的扫描、symbol graph、module tree、page planning 和 page context 构建
- **THEN** 系统 MUST 在正式渲染和写盘前执行 `llm_enrichment`
- **THEN** 若增强阶段失败，系统 MUST 回退到 deterministic 页面内容继续完成 `init`

#### Scenario: update 只对受影响页面重新执行增强
- **WHEN** 用户执行 `update`，且变化范围只影响部分页面
- **THEN** 系统 MUST 只对受影响页面及其依赖的父页面重新执行页面增强
- **THEN** 未受影响页面不得因为 LLM 开启而被无谓重写

#### Scenario: rebuild 在未协商桥接时保持 deterministic
- **WHEN** 用户执行 `rebuild`，但当前 Agent 未声明 LLM 桥接能力或 steering 关闭了增强
- **THEN** 系统 MUST 跳过 LLM 阶段并走完整 deterministic 路径
- **THEN** rebuild 的最终状态与现有 deterministic 语义保持一致
