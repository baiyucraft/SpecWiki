## ADDED Requirements

### Requirement: knowledge system 完整化计划必须拆分为 capability-scoped child changes
系统 MUST 将“完善 knowledge system”拆分为 capability-scoped child changes，而不是把 `formal completeness`、`governance completeness`、`query / answer completeness` 与 `engineering hardening` 混成一个直接实施的 change。每个 child change MUST 拥有清晰的 formal object 边界与独立验收面。

#### Scenario: 当前 runtime 已进入 minimal formal knowledge runtime，准备继续推进完整化
- **WHEN** 仓库已经建立 `minimal formal knowledge runtime`，并准备继续推进完整 knowledge system
- **THEN** 系统 MUST 先产出 capability map 与 child change 列表
- **THEN** 系统 MUST NOT 直接把所有后续能力写成一个“一次性补完”的实施 change

### Requirement: umbrella roadmap change 不得伪装成 runtime 实施承诺
`iteration-12-9-knowledge-system-completeness` 这类 umbrella / roadmap change MUST 明确只负责 program-level 冻结，不直接承诺 runtime 代码落地。它至少 MUST 固定 capability map、spec impact matrix、依赖顺序与 acceptance matrix。系统 MUST NOT 把 umbrella 文档写成“本轮已经实现全部能力”的实施承诺。

#### Scenario: 团队为 umbrella change 编写 proposal / design
- **WHEN** 某个 umbrella change 负责后续多个 formal contract 的 program-level 组织
- **THEN** proposal / design MUST 明确该 change 不直接承诺 runtime 代码落地
- **THEN** proposal / design MUST 把 capability map、spec impact matrix 与 acceptance matrix 作为正式输出，而不是可有可无的背景说明

### Requirement: 完整化计划必须冻结 7 个 capability 与明确非目标
knowledge system completeness roadmap MUST 将“完善 knowledge system”冻结为 7 个 capability：`declared lifecycle completeness`、`derived research contract completeness`、`projection / readiness / recovery completeness`、`governance conflict artifacts`、`query route completeness`、`answer assembly contract`、`engineering hardening / quality gates`。roadmap MUST 同时显式写出非目标：不回到 page-first、不做治理平台空话、不做多 repo 编排、不做宿主 UI 扩张。

#### Scenario: 后续团队想把 roadmap 再泛化成模糊愿景
- **WHEN** roadmap 文档出现“更多页面”“更聪明 query”“更强治理平台”这类模糊表述
- **THEN** 系统 MUST 将这些目标收回到 7 个 capability 或显式非目标中
- **THEN** 系统 MUST NOT 让 roadmap 漂移回 page-first 或平台化空话

### Requirement: child change 必须按 formal object 与 contract 描述，而不是按 page-first 功能描述
每个 child change MUST 显式说明其新增或强化的 formal object、truth source、workflow consumers、artifact layer 与 failure / degraded / recovery contract。系统 MUST NOT 以页面类型、页面排版或样本仓库特例作为 capability 的主拆分维度。

#### Scenario: 为某个完整化 capability 编写 proposal / design
- **WHEN** 系统为某个 knowledge completeness child change 编写 proposal 或 design
- **THEN** 文档 MUST 明确写出 formal object、truth source、workflow consumers、artifact layer 与 failure / degraded / recovery 语义
- **THEN** 文档 MUST NOT 只描述某类页面长什么样或某个样本仓库想要什么效果

### Requirement: 完整化计划必须固定 capability 依赖顺序
系统 MUST 为 knowledge completeness child changes 固定依赖顺序，使 governance、query、answer 与 engineering hardening 建立在前置 formal contracts 之上，而不是并行无序推进。至少 MUST 显式区分：`declared lifecycle`、`derived research contract`、`projection / readiness / recovery`、`governance conflict artifacts`、`query route`、`answer assembly`、`engineering hardening / quality gates` 七类 capability。

#### Scenario: 后续团队准备直接推进 answer assembly
- **WHEN** 某个后续 change 计划直接推进 answer assembly 或 host-facing knowledge answer
- **THEN** 完整化计划 MUST 先检查 `declared lifecycle`、`derived research contract`、`projection / readiness / recovery` 与 `query route` 是否已有前置 contract
- **THEN** 若这些前置 contract 尚未成立，系统 MUST NOT 将 answer assembly 视为可独立闭环的优先项

### Requirement: 完整化计划必须存在 spec impact matrix 与 per-capability contract matrix
knowledge completeness roadmap MUST 为全部 7 个 capability 提供正式矩阵，至少覆盖：它会修改哪些既有 specs、需要新增哪些 specs、主要 owner crate 是什么，以及该 capability 的 formal object、truth source、workflow consumers、artifact layer、failure / degraded / recovery contract。系统 MUST NOT 只保留 capability 名称而缺少这些 program-level 约束。

#### Scenario: 团队准备为某个 capability 开新 child change
- **WHEN** 团队根据 umbrella roadmap 继续拆分或实施某个 capability
- **THEN** roadmap MUST 能回答该 capability 修改哪些既有 specs、需要哪些新 specs、owner crate 是谁
- **THEN** roadmap MUST 能回答它的 formal object、truth source、workflow consumers、artifact layer 与 failure / degraded / recovery contract

### Requirement: engineering hardening 必须与 formal contract 变更分离验收
系统 MUST 将 engineering hardening / quality gates 与 formal contract 变更分离验收。formal contract change 的验收 MUST 关注 schema、state machine、route contract、artifact 与 recovery；engineering hardening 的验收 MUST 关注 `storybook + dagger`、全量 `19` 项目、长流程稳定性与质量指标。两者 MUST NOT 在同一个 child change 中被混写成单一完成标准。

#### Scenario: 某个 child change 同时宣称修改 schema 并完成大样本稳定性收口
- **WHEN** 某个 child change 同时包含 formal object / lifecycle 变更与大样本稳定性承诺
- **THEN** 完整化计划 MUST 将其拆分为 formal contract change 与 engineering hardening change
- **THEN** 系统 MUST NOT 用批量样本通过来掩盖 formal contract 边界仍未收稳的问题

### Requirement: 每个 child change 必须遵守统一验收矩阵
每个 knowledge completeness child change MUST 至少定义以下验收层：`formal object / schema`、`workflow consumption`、`artifact / recovery`、`storybook + dagger` 样本验证，以及在适用阶段进入全量 `19` 项目批量验证。任何 child change 若缺失这些层中的适用验收项，MUST NOT 被视为完成。

#### Scenario: 某个 child change 完成 formal schema 编写后准备关闭
- **WHEN** 某个 child change 已写完 formal objects 与 spec，但尚未定义 workflow、artifact 或样本验收
- **THEN** 系统 MUST 将其视为未完成
- **THEN** 系统 MUST 要求继续补齐统一验收矩阵中的适用项

### Requirement: 统一验收矩阵必须显式包含 primary sample、batch baseline 与 COMMENTING 检查
knowledge completeness roadmap 的 acceptance matrix MUST 显式包含：`storybook + dagger` 专项验收与报告要求、全量 `19` 项目的批量验收入口与质量指标，以及 [COMMENTING.md](/E:/project/!byAI/spec-wiki/COMMENTING.md) 与 OpenSpec 产物边界一致性检查。系统 MUST NOT 只定义 schema 或 workflow 验收，而遗漏 program-level closeout 所需的样本、批量和注释一致性检查。

#### Scenario: 某个 child change 准备收口
- **WHEN** 团队准备关闭某个 knowledge completeness child change
- **THEN** acceptance matrix MUST 能指出它要跑的 `storybook + dagger` 专项验收、`19` 项目批量入口与质量指标
- **THEN** acceptance matrix MUST 能指出 COMMENTING 与 OpenSpec 术语边界是否仍一致
