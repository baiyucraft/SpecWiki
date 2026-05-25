## ADDED Requirements

### Requirement: `KnowledgeResearchSummary` 必须拥有正式 status contract
系统 MUST 将 `KnowledgeResearchSummary` 视为正式 derived object，而不是只够 compose 临时消费的弱摘要。该对象 MUST 使用稳定 typed `summary_status`，至少覆盖 `ready`、`degraded` 与 `blocked` 三种状态。系统 MUST NOT 继续使用自由字符串表达 research summary 状态。

#### Scenario: research summary 以 typed status 落盘
- **WHEN** 某个 `KnowledgeUnit` 被收敛为正式 `KnowledgeResearchSummary`
- **THEN** 该对象 MUST 写出稳定 `summary_status`
- **THEN** `summary_status` MUST 只来自正式允许集合，而不是任意字符串

### Requirement: `degraded / blocked` 必须拥有最小 reason contract
系统 MUST 为 `degraded` 与 `blocked` research summary 提供最小 machine-readable reason contract。该 contract MUST 至少允许表达 `reason_kind`、`reason_message` 与可选 `upstream_ref`。系统 MUST NOT 只靠自由文本摘要、runtime gate 旁路字段或 provider 日志推断 research summary 当前为何 degraded / blocked。

#### Scenario: degraded research summary 带正式原因
- **WHEN** 某个 research summary 因缺失关键研究内容而被判为 `degraded`
- **THEN** 该对象 MUST 同时携带至少一条 formal reason
- **THEN** 该 reason MUST 可被 artifact、status 或 health consumer 稳定读取

#### Scenario: blocked research summary 带正式上游引用
- **WHEN** 某个 research summary 因 provider 或上游 contract 无法完成而被判为 `blocked`
- **THEN** 该对象 MUST 能携带 `upstream_ref` 或等价稳定引用
- **THEN** 调用方 MUST 不需要反查 runtime gate 才能知道阻塞来源

### Requirement: research summary snapshot 必须可校验
系统 MUST 为 research summary snapshot 提供 canonicalize / validate contract，并在 artifact persist 与 restore 前执行。若 summary status、reason contract 或最小 identity 非法，系统 MUST 显式拒绝该 snapshot，而不是静默继续恢复或落盘。

#### Scenario: 非法 research summary 在 persist 前被拒绝
- **WHEN** 某个 `KnowledgeResearchSummary` 缺少 `unit_id`、携带非法 `summary_status`，或 `blocked / degraded` 与 reason contract 不一致
- **THEN** artifact persist MUST 失败
- **THEN** 系统 MUST 返回显式错误，而不是写出半合法 snapshot
