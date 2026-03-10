## ADDED Requirements

### Requirement: 系统必须基于 symbol graph 检测代码 communities
系统 MUST 基于解析后的 `CALLS`、`EXTENDS` 和 `IMPLEMENTS` edges 检测代码 communities，并把结果写入 `communities` 与 `community_members` 表。community 检测 MUST 对大型图提供降噪策略，至少允许过滤低置信度边与高度孤立节点；当首选算法不可用或超时，系统 MUST 回退到 deterministic 的近似聚类结果，而不是让整轮 workflow 失败。

#### Scenario: 解析后生成 communities
- **WHEN** 仓库的 symbol graph 中存在多组相互协作的代码簇
- **THEN** 系统 MUST 生成至少一组 community 结果
- **THEN** 每个 community MUST 记录稳定 ID、标签、内聚度和成员数量

#### Scenario: 大型图启用降噪与 fallback
- **WHEN** symbol graph 规模较大或首选 community 检测算法超时/不可用
- **THEN** 系统 MUST 过滤低质量边或孤立节点以控制分析成本
- **THEN** 系统 MUST 回退到 deterministic 的近似聚类结果，而不是终止 workflow

### Requirement: 系统必须从 CALLS 图中检测执行流 processes
系统 MUST 基于 CALLS 图检测执行流 `processes`，至少包含入口点评分、正向 trace、子集去重和端点去重。每个 process MUST 记录稳定 ID、标签、`process_type`、步骤数量、入口 symbol、终点 symbol，并写入 `processes` 与 `process_steps` 表。process trace 只应消费中高置信度 CALLS edges，不得让低质量 fuzzy-global 边主导执行流。

#### Scenario: 从入口点追踪执行流
- **WHEN** symbol graph 中存在可识别的入口点和多跳调用链
- **THEN** 系统 MUST 能从入口点追踪出至少一个多步 process
- **THEN** 该 process MUST 写入 `processes` 和对应的 `process_steps`

#### Scenario: 低质量 CALLS 边不主导 process trace
- **WHEN** 某些 CALLS edge 的置信度过低或来源不可靠
- **THEN** 系统 MUST 不让这些边主导 process trace 的生成
- **THEN** 生成的 process MUST 优先反映中高置信度的真实调用链

### Requirement: 系统必须检测 symbol graph 中的调用环并生成拓扑提示
系统 MUST 对 symbol graph 执行 SCC/cycle detection，并在检测到调用环时生成可供 planner、query 或后续 LLM 流程消费的拓扑提示。系统 MUST 在存在环时采用稳定的断边或降级策略，避免后续流程因为循环依赖无法继续。

#### Scenario: 检测并处理调用环
- **WHEN** symbol graph 中存在强连通调用环
- **THEN** 系统 MUST 能识别这些 SCC/cycle
- **THEN** 系统 MUST 生成稳定的断边或降级结果，使后续流程仍能继续执行

#### Scenario: 无环时输出稳定拓扑顺序
- **WHEN** symbol graph 中不存在强连通调用环
- **THEN** 系统 MUST 能为后续 planner 或 analysis 导出稳定的拓扑提示
- **THEN** 重复分析同一图时该提示顺序 MUST 保持 deterministic

