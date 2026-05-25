## ADDED Requirements

### Requirement: Workflow surfaces MUST consume governance resolution without redefining conflict detection
`sync / status / update / query` MUST 消费 companion resolution artifact，并据此更新 blocker、degraded 与 refresh-needed 语义；它们 MUST NOT 在本轮重新定义 deterministic conflict detection 或修改 open conflict artifact 的 formal boundary。

#### Scenario: update 遇到已决议但待恢复的 conflict
- **WHEN** `update` 读取到某个 conflict 的 resolution state 为 `resolved` 且 refresh state 为 `pending`
- **THEN** workflow MUST 将其视为需要恢复链继续推进的对象
- **THEN** workflow MUST NOT 将其误判为仍处于未决 review conflict
