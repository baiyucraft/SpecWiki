## ADDED Requirements

### Requirement: Query, rebuild, and restore MUST surface answer substrate drift as degraded or refuse
`query / rebuild / restore` MUST 在 answer substrate 漂移、缺 provenance、缺 supporting refs 或 governance blocker 存在时，输出一致的 `degraded` 或 `refuse` 语义。系统 MUST NOT 在这些场景中静默维持正常 answer mode。

#### Scenario: rebuild 后 formal substrate 无法完整恢复
- **WHEN** `rebuild` 或 `restore` 后某个 answer target 的 substrate 不再完整
- **THEN** workflow MUST 将后续 answer surface 标记为 `degraded` 或 `refuse`
- **THEN** workflow MUST NOT 继续返回看似 `direct` 的 answer mode
