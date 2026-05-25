## ADDED Requirements

### Requirement: Post-12-9 roadmap MUST adopt existing declared authoring prerequisite instead of duplicating it
`iteration-12-10-knowledge-system-remaining-gaps` MUST 将 `formalize-declared-authoring-contract` 视为 adopted existing prerequisite，而不是再创建一个同义 child。roadmap MUST 明确该 prerequisite 负责 `DeclaredRecord / typed scope / supersede / authoring truth / declared-driven stale propagation`，后续 child MUST 以此为前置。

#### Scenario: 规划 post-12-9 remaining gaps program
- **WHEN** 团队创建 `12-10` umbrella 来组织 remaining gaps
- **THEN** roadmap MUST 列出 `formalize-declared-authoring-contract` 为 adopted existing prerequisite
- **THEN** roadmap MUST NOT 再新建一个语义重复的 declared authoring child

### Requirement: Post-12-9 roadmap MUST freeze exactly three new child boundaries
`iteration-12-10-knowledge-system-remaining-gaps` MUST 只新增三个 child boundary：`extend-governance-conflict-resolution-contract`、`harden-answer-assembly-compose-runtime`、`formalize-knowledge-governance-metrics-and-release-gates`。每个 child MUST 说明它是在扩展 `12-9` 哪个已 formalize capability，而不是重做 formalization。

#### Scenario: 团队为 remaining gaps umbrella 列出子 change
- **WHEN** umbrella 为 `12-10` program 列出 child changes
- **THEN** 系统 MUST 只出现上述三个新 child
- **THEN** 每个 child MUST 说明自己是 `extends / hardens / aggregates`，而不是重新 `formalize` 已在 `12-9` 冻结过的能力

### Requirement: Roadmap MUST freeze dependency order across prerequisite and child changes
roadmap MUST 固定 `formalize-declared-authoring-contract -> extend-governance-conflict-resolution-contract -> harden-answer-assembly-compose-runtime -> formalize-knowledge-governance-metrics-and-release-gates` 的依赖顺序。任何后置 child 若缺少前置 contract，MUST NOT 被视为可独立闭环的优先项。

#### Scenario: 团队尝试跳过 governance resolution 直接推进 answer hardening
- **WHEN** 某个 proposal 计划直接实施 `harden-answer-assembly-compose-runtime`
- **THEN** roadmap MUST 先检查 `formalize-declared-authoring-contract` 与 `extend-governance-conflict-resolution-contract` 是否已成立
- **THEN** 若前置未成立，系统 MUST NOT 将 answer hardening 视为独立闭环项

### Requirement: Roadmap MUST define program-level release evidence matrix without owning runtime code work
umbrella MUST 只定义 adopted prerequisite、child matrix、dependency order 与 release evidence matrix。它 MUST NOT 在 tasks 中承诺 runtime code 直接落地。release evidence matrix MUST 至少覆盖 `storybook + dagger` primary sample、`19` 项目 baseline batch，以及 knowledge governance summary。

#### Scenario: 团队为 umbrella 编写 tasks
- **WHEN** 团队为 `12-10` umbrella 编写 tasks
- **THEN** tasks MUST 只包含 program 文档、矩阵、依赖和 evidence 组织工作
- **THEN** tasks MUST NOT 混入 runtime code、artifact schema 或脚本实现项

### Requirement: Roadmap MUST keep remaining-gaps non-goals explicit
`12-10` roadmap MUST 显式写出非目标：不宣称完整 knowledge system 已 closure、不回到 page-first、不新增 dashboard、多 repo 编排或宿主 UI 扩张。后续 child 若出现这些方向，MUST 被视为 scope drift。

#### Scenario: 后续 child proposal 出现平台化或 page-first 表述
- **WHEN** 某个 child proposal 出现 `dashboard`、`多 repo 编排`、`page-first planner` 或“本轮完成完整 knowledge system”等表述
- **THEN** roadmap MUST 将其标记为超出 `12-10` remaining gaps 边界
- **THEN** 系统 MUST 要求将该内容收回非目标或另开 change
