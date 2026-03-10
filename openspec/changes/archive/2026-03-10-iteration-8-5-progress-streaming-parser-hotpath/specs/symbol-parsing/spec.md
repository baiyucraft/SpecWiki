## ADDED Requirements

### Requirement: 同一 parse unit 的 definitions 与 raw captures 必须复用解析工件
系统 MUST 在单次 workflow 的 symbol parsing 中，对同一 `ParseUnit` 只构建一次 syntax tree 和一次对应 query，并在此基础上同时提取 definitions 与 raw `import / call / heritage` captures。热路径优化不得改变既有 symbol ID、raw capture 归属关系、诊断隔离行为或 fail-soft 语义。

#### Scenario: 同一文件同时包含 definitions 与 raw captures
- **WHEN** 某个源码文件或包装脚本块同时需要提取 definitions 和 raw relation captures
- **THEN** 系统 MUST 复用该 parse unit 的同一份 syntax tree 与 query 工件
- **THEN** 最终产出的 symbols、imports、calls 和 heritage 结果 MUST 与未优化前保持语义一致

#### Scenario: 单文件解析失败仍保持隔离
- **WHEN** 某个 parse unit 在复用工件构建前或构建过程中出现 parse/query 错误
- **THEN** 系统 MUST 仍只把该文件视为失败并记录诊断
- **THEN** 其他文件的 symbol parsing 结果 MUST 不受影响

### Requirement: symbol parsing 必须支持受字节预算约束的确定性并行执行
在待解析源码规模较大时，系统 MUST 保留字节预算分块约束，并允许在单个 chunk 内以文件为工作单元执行有限并行解析。无论 worker 数量如何，最终 `symbols`、raw captures 和诊断集合的内容与排序语义 MUST 保持 deterministic，不得因为并行度不同而改变稳定 ID 或结果顺序。

#### Scenario: 不同并行度下结果保持一致
- **WHEN** 同一仓库分别以单工作单元和默认并行度执行 symbol parsing
- **THEN** 系统 MUST 产出相同的 symbol IDs 与 raw capture 结果
- **THEN** 结果排序语义 MUST 保持一致

#### Scenario: 大仓库并行执行仍受 chunk 预算约束
- **WHEN** 仓库中的待解析源码总量超过单批字节预算
- **THEN** 系统 MUST 继续按稳定的 chunk 边界切分工作
- **THEN** chunk 内并行执行不得突破既定的内存预算边界
