## ADDED Requirements

### Requirement: planner 必须先构建 typed surface bundles 再决定 KnowledgeUnit 拆分
系统 MUST 先把 `docs anchors`、`public api surfaces`、`config surfaces`、`runtime/compiler/testing/example/troubleshooting/integration` 信号、`graph/process/community` 事实和 `repo_archetype_signals` 收敛为稳定的 typed surface bundles，再据此决定 leaf / parent `KnowledgeUnit` 的拆分、合并与父子边界。系统 MUST NOT 再让离散关键词表、样本目录名或仓库私有命名直接主导页面集合。

#### Scenario: docs-heavy 仓库先按 typed bundles 拆出不同知识面
- **WHEN** 某个 docs-heavy 仓库同时存在稳定的 docs anchors、公开 API 表面、配置入口和 troubleshooting/example 材料
- **THEN** planner MUST 先把这些输入收敛为不同的 typed surface bundles
- **THEN** 系统 MUST 基于这些 bundles 生成独立的 API、配置、指南、故障排查或示例类 `KnowledgeUnit`
- **THEN** 系统 MUST NOT 继续主要依赖离散关键词把这些知识面折叠回单一大页

#### Scenario: runtime-heavy / compiler-heavy 仓库先按 typed bundles 拆出运行时与编译链
- **WHEN** 某个仓库同时存在 runtime 模块、compiler/codegen 模块、测试目录、示例目录和稳定 graph/process 信号
- **THEN** planner MUST 先形成对应的 runtime、compiler、testing、example 或 troubleshooting bundles
- **THEN** 系统 MUST 基于这些 bundles 生成中粒度 `KnowledgeUnit`
- **THEN** 系统 MUST NOT 让单一模块名或样本关键词直接决定页面边界

### Requirement: planner 必须用 collapse guard 抑制 missing page 与 coarse reuse
系统 MUST 为 `KnowledgeUnit` 规划引入显式 `collapse guard`，并让它直接回答“当前候选是否必须独立成页、是否允许合并回父页、是否存在 severe reuse 风险”。`collapse guard` MUST 建立在 typed surface bundles、child density、关键源码簇和 evidence 稠密度之上，而不是建立在仓库名、reference 标题或固定样本规则之上。

#### Scenario: collapse guard 阻止高密度 API / config / troubleshooting 候选被吞并
- **WHEN** 某个 parent unit 下同时出现多个高密度的 API、配置或 troubleshooting surface bundles
- **THEN** planner MUST 通过 `collapse guard` 判定这些候选是否必须独立成页
- **THEN** 系统 MUST NOT 在存在 severe reuse 风险时继续把这些候选默认折叠回 parent unit

#### Scenario: collapse guard 结论可映射回 gap ledger
- **WHEN** `storybook`、`dagger` 或其它样本在 reference/gap ledger 中暴露 missing page 或 many-to-one reuse
- **THEN** 系统 MUST 能把这些缺口映射回对应的 `collapse guard` 或 typed surface bundle 规则
- **THEN** 这些结论 MUST 继续沉淀为通用 planner 规则，而不是样本仓库特判
