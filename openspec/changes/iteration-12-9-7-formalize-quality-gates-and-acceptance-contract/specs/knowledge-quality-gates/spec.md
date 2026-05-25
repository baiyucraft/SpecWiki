## ADDED Requirements

### Requirement: 系统必须定义最小正式质量闸门
系统 MUST 为 knowledge system 定义最小正式质量闸门，而不是继续依赖分散的脚本约定。当前版本的最小 gates MUST 至少覆盖 `artifact validity`、`restore validity`、`query route contract` 与 `status/recommended_action stability`。任一 gate 失败时，系统 MUST 能将该失败明确归因到具体 gate，而不是只给出模糊“测试失败”。

#### Scenario: formal artifact 不合法时阻断收口
- **WHEN** 当前 runtime artifacts 不满足正式 schema 或快照一致性
- **THEN** 系统 MUST 将该结果判定为 `artifact validity` gate 失败
- **THEN** 系统 MUST 不得继续把该 change 视为可通过

#### Scenario: restore contract 漂移时阻断收口
- **WHEN** cache restore、cold rebuild 或等价恢复路径不再满足正式 contract
- **THEN** 系统 MUST 将该结果判定为 `restore validity` gate 失败
- **THEN** 系统 MUST 明确输出该 gate 的阻断结论

### Requirement: 每个 capability 必须映射到统一测试矩阵
系统 MUST 为后续 capability 定义统一的 capability-to-test matrix。每个 capability 至少 MUST 映射到 `model/schema tests`、`artifact/recovery tests`、`workflow tests` 与 `sample gates` 四类测试面；必要时 MAY 再映射到 batch baseline 或 reference reporting，但 MUST 先覆盖这四类核心测试面。

#### Scenario: 某个 capability 进入实现前验收规划
- **WHEN** 团队为某个 capability 编写 tasks 或验收方案
- **THEN** 方案 MUST 明确列出该 capability 对应的 `model/schema`、`artifact/recovery`、`workflow` 与 `sample` 测试面
- **THEN** 系统 MUST 不得只靠单一脚本或单一仓库样本作为唯一验收依据

### Requirement: quality gates 必须区分 blocker 与 diagnostic
系统 MUST 将 gate 结果显式区分为 `blocker` 与 `diagnostic` 两类。`blocker` 表示该 change 当前不得收口；`diagnostic` 表示系统需要保留结构化观察结果，但它本身不单独阻断收口。系统 MUST NOT 把所有脚本输出、样本观察或 fidelity 指标统一压成同一等级的失败。

#### Scenario: 正式 gate 失败时进入 blocker
- **WHEN** `artifact validity`、`restore validity`、`query route contract` 或 `status/recommended_action stability` 任一正式 gate 失败
- **THEN** 系统 MUST 将该结果标记为 `blocker`
- **THEN** 系统 MUST 不得继续将当前 change 判定为通过

#### Scenario: 观察性输出不得自动升级为 blocker
- **WHEN** 某个脚本输出仅提供观测信息、趋势或补充诊断，但未命中正式 gate 失败条件
- **THEN** 系统 MUST 将其标记为 `diagnostic`
- **THEN** 系统 MUST 不得把该类输出自动提升为正式阻断结论
