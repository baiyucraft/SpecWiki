# knowledge-runtime-health-signals Specification

## Purpose

定义 Runtime 跨层 reliability 与 health signal 合同。Health、freshness、consumability、governance 和 workflow progress 是正交维度；公开 workflow 必须消费同一 reliability assessment，不能用单个模糊状态替代。
## Requirements
### Requirement: Runtime 必须使用统一 reliability assessment

系统 MUST 基于 live facts、formal artifacts、runtime gates、governance evidence 与 local working state 生成唯一 reliability assessment。该 assessment MUST 分离 freshness 与 consumability，并为每个公开 workflow 投影一致的 readiness、trust、reason 与单值 recommended action。Cache 或局部 `Ok` MUST NOT 单独构成正式成功证据。

#### Scenario: stale route 只在真实 provenance 下受限消费
- **WHEN** formal result 仍存在但其 source/facts fingerprint 已落后于 live repository
- **THEN** 系统 MUST 将对应 route 标为 stale，而不是 ready/direct-trust
- **THEN** 只有保留真实 stale provenance 时才可继续返回，并提供 update 或 rebuild 动作

#### Scenario: governance review 不降低无关 ready route
- **WHEN** declared authority scope 需要治理复核，但 index route 仍有 fresh evidence
- **THEN** 系统 MUST 保持 index route 的 core trust
- **THEN** 系统 MUST 以正交 governance action 暴露复核需求

### Requirement: runtime 必须持久化最小 knowledge health signals
系统 MUST 为 knowledge runtime 持久化最小 health signals，而不只是报告 runtime 是否存在。最小 health signal 集 MUST 至少覆盖：`orphan_unit`、`missing_provenance`、`derived_stale`、`projection_stale` 与 `declared_derived_divergence`。每条 signal MUST 至少包含稳定 `signal_id`、`signal_kind`、`severity`、`target_ref`、`recommended_action` 与 `reason`。

#### Scenario: orphan unit 被识别为正式 health signal
- **WHEN** 某个 `KnowledgeUnit` 已存在 formal identity，但缺失有效 domain、source 或 projection 绑定
- **THEN** 系统 MUST 生成 `orphan_unit` signal
- **THEN** 该 signal MUST 指向具体 `unit_id` 并提供稳定推荐动作

#### Scenario: declared 与 derived 不一致进入 health layer
- **WHEN** 某条 declared record 已更新，但其关联的 derived summary 或 projection 仍停留在旧快照
- **THEN** 系统 MUST 生成 `declared_derived_divergence` signal
- **THEN** 系统 MUST NOT 仅以页面 fallback 成功就隐藏该不一致

### Requirement: `status` 必须聚合 knowledge health 摘要
系统 MUST 让 `status` 除 readiness 之外返回最小 knowledge health 摘要。该摘要 MUST 至少表达当前存在的 health signal 总数、按 kind 或 severity 聚合后的计数，以及面向调用方的单值 `recommended_action`。`status` MUST 能区分“runtime ready 但 health degraded”与“runtime blocker”。

#### Scenario: runtime 可用但 health degraded
- **WHEN** 当前 formal artifacts 可被 `query` 或恢复链消费，但存在 `derived_stale` 或 `projection_stale` 等非 blocker health signal
- **THEN** `status` MUST 继续返回可消费的 readiness
- **THEN** `status` MUST 同时返回非空 health 摘要与对应推荐动作

#### Scenario: 严重 health signal 触发 rebuild 推荐
- **WHEN** 当前存在无法可信修复的一致性问题，例如 projection 锚点与 declared/derived 快照严重冲突
- **THEN** `status` MUST 将其投影为 blocker 或 `rebuild` 推荐
- **THEN** 系统 MUST 保留导致该建议的 signal 摘要，而不是只返回模糊错误

