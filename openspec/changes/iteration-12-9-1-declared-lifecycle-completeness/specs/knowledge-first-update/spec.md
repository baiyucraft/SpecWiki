## ADDED Requirements

### Requirement: removed declared records 必须驱动 declared-only stale scope
系统 MUST 将 removed declared records 视为正式 declared lifecycle 变化，并将其传播为可被 `update` 消费的 stale scope。即使源码 dirty set 为空，只要本次 `sync` 合法移除了此前存在的 declared records，后续 `update` 仍 MUST 刷新受影响的 derived / projection snapshot。

#### Scenario: 无源码 dirty 但存在 removed declared records
- **WHEN** 某次 `sync` 合法移除了此前存在的 declared records，且当前源码层没有新增 dirty files
- **THEN** 后续 `update` MUST 继续命中 declared-only stale scope
- **THEN** 系统 MUST NOT 因源码 dirty set 为空就跳过这次 derived / projection refresh
