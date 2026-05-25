## ADDED Requirements

### Requirement: `PageDigest` 必须拥有正式 projection status contract
系统 MUST 将 `PageDigest` 视为正式 projection snapshot，而不是只够 parent compose 临时消费的弱摘要。该对象 MUST 使用稳定 typed `projection_status`，至少覆盖 `ready`、`stale` 与 `blocked` 三种状态。系统 MUST NOT 继续仅用 `readiness_stage` 自由字符串表达 projection 当前是否可正式消费。

#### Scenario: projection digest 以 typed status 落盘
- **WHEN** 某个页面 projection 被收敛为正式 `PageDigest`
- **THEN** 该对象 MUST 写出稳定 `projection_status`
- **THEN** `projection_status` MUST 只来自正式允许集合，而不是任意字符串

### Requirement: projection stale / blocked 必须拥有最小 reason contract
系统 MUST 为 `stale` 与 `blocked` projection digest 提供最小 machine-readable reason contract。该 contract MUST 至少允许表达 `reason_kind`、`reason_message` 与可选 `upstream_ref`。系统 MUST NOT 只靠页面文件、runtime gate 或 restore 失败旁路来推断 projection 当前为何 stale / blocked。

#### Scenario: stale projection digest 带正式原因
- **WHEN** 某个 projection digest 因页面输出缺失、快照不匹配或上游 knowledge 变化而被判为 `stale`
- **THEN** 该对象 MUST 同时携带至少一条 formal reason
- **THEN** 该 reason MUST 可被 artifact、status 或 health consumer 稳定读取

#### Scenario: blocked projection digest 带上游引用
- **WHEN** 某个 projection digest 因 research contract 无法满足而被判为 `blocked`
- **THEN** 该对象 MUST 能携带 `upstream_ref` 或等价稳定引用
- **THEN** 调用方 MUST 不需要反查 runtime gate 才能知道 projection 为何被阻断

### Requirement: projection digest snapshot 必须可校验
系统 MUST 为 projection digest snapshot 提供 canonicalize / validate contract，并在 artifact persist 与 restore 前执行。若 digest status、reason contract 或最小 identity 非法，系统 MUST 显式拒绝该 snapshot，而不是静默继续恢复或落盘。

#### Scenario: 非法 projection digest 在 persist 前被拒绝
- **WHEN** 某个 `PageDigest` 缺少 `unit_id / page_id`、携带非法 `projection_status`，或 `stale / blocked` 与 reason contract 不一致
- **THEN** artifact persist MUST 失败
- **THEN** 系统 MUST 返回显式错误，而不是写出半合法 snapshot
