## ADDED Requirements

### Requirement: Runtime conflict snapshot MUST preserve open conflict truth and companion resolutions separately
系统 MUST 继续沿用既有 runtime snapshot layer 持久化 open conflict records，并在同层新增 companion resolution snapshot。任何 resolution 持久化 MUST NOT 改写 open conflict record 的 open-only truth boundary。

#### Scenario: runtime 持久化 conflict 与 resolution
- **WHEN** 系统为某个 conflict 写出 snapshot
- **THEN** open conflict record MUST 继续存放在既有 runtime conflict artifact 中
- **THEN** resolution state MUST 写入与之关联的 companion artifact，而不是覆写 open conflict record
