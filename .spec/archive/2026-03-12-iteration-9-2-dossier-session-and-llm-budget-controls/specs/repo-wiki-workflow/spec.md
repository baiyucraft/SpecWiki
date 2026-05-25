## ADDED Requirements

### Requirement: workflow 主链必须在正式渲染前组装 dossier 并按需执行 bounded research session
系统 MUST 在保持 deterministic 主链的前提下，在正式渲染前组装 dossier，并按页面类型决定是否执行 bounded research session。research session 的输入 MUST 来自 dossier，而不是绕过主链重新扫描仓库。9.2 首先要求 provider 直连路径下的 bounded research session 可运行、可回退、可观测。

#### Scenario: module 或 topic 页在 render 前执行 research session
- **WHEN** 用户执行 `init`、`update` 或 `rebuild`，且当前页面为 `module` 或 `topic`
- **THEN** 系统 MUST 先完成 dossier 组装
- **THEN** 若 research session 开启，系统 MUST 在 render 前执行该 session 并消费结构化结果

#### Scenario: update 只重建受影响 dossier 与父页 rollup
- **WHEN** 变化范围只影响部分 dossier、child rollup 或 session 结果
- **THEN** 系统 MUST 只重建这些页面及其受影响父页
- **THEN** 未受影响页面不得因为 dossier/session 引入而被无谓重写

### Requirement: uncertainty gate 必须按同类型批量和有限并行执行
系统 MUST 让 `uncertainty_gate` 优先按同类型候选进行批量判断，并允许在阶段内有限并行。不同阶段、不同 schema 的候选不得被揉成一个跨阶段 mega prompt。

#### Scenario: file_purpose 批量判断
- **WHEN** 同一阶段内存在多条 `file_purpose` 候选
- **THEN** 系统 MUST 优先按批次请求这些候选
- **THEN** 系统不得默认逐条串行请求每个文件角色判断

#### Scenario: 不同 schema 的 gate 保持分阶段
- **WHEN** workflow 同时存在 `file_purpose`、`top_level_promotion`、`dependency_edge` 等不同类型 gate
- **THEN** 系统 MUST 允许它们分别批量
- **THEN** 系统不得把不同 schema 混成一个总 prompt
