# knowledge-unit-decomposition Specification

## Purpose
TBD - created by archiving change iteration-9-5-provider-first-research-evidence-and-unit-decomposition. Update Purpose after archive.
## Requirements
### Requirement: planner 必须按真实信号执行通用 KnowledgeUnit 中粒度拆分
系统 MUST 基于真实代码结构、模块划分、调用关系、docs anchors、public API surface、config surface、testing/example/tutorial 目录与 graph facts 规划 KnowledgeUnit 中粒度拆分，而不是继续主要依赖 `module / topic / family` 旧页面语义。planner MUST 先判定 `DecompositionProfile`，再生成与之匹配的 leaf / parent KnowledgeUnit。

#### Scenario: docs-heavy 仓库按 API 与配置面拆出独立单元
- **WHEN** 某个 docs-heavy 仓库同时存在稳定的 docs anchors、public API exports 和配置入口
- **THEN** planner MUST 将这些信号拆分到不同的 `DecompositionProfile`
- **THEN** 系统 MUST 生成独立的 API、配置、指南类 KnowledgeUnit，而不是把它们合并进单一 `ConceptGuide`

#### Scenario: runtime-heavy / compiler-heavy 仓库按运行时、编译链和测试面拆分
- **WHEN** 某个仓库同时存在 runtime 模块、compiler/codegen 模块、testing/example 目录和稳定的 process/community 信号
- **THEN** planner MUST 生成对应的 runtime、compiler、testing、example/tutorial 类 KnowledgeUnit
- **THEN** 这些单元不得被默认折叠回少数大模块页

### Requirement: KnowledgeUnit 拆分必须保持 deterministic 且禁止样本硬编码
系统 MUST 让 `DecompositionProfile`、unit id、相对路径和父子关系仅由稳定 signal 与 planner 规则决定。系统 MUST NOT 基于仓库名、reference 标题、固定目录名白名单或 storybook/dagger 样本特判生成页面集合。

#### Scenario: 相同输入重复运行时单元身份稳定
- **WHEN** 同一仓库在 source graph、docs/API/config/testing/example 信号未变化的情况下重复执行 `init` 或 `rebuild`
- **THEN** 相同 KnowledgeUnit 的 id、relative_path 和 parent_unit_id MUST 保持稳定
- **THEN** 页面集合不得因模型输出差异而抖动

#### Scenario: 样本仓库结论被收敛为通用规则
- **WHEN** 系统使用 storybook 或 dagger 暴露出的缺口来优化 planner
- **THEN** 这些结论 MUST 以 `DecompositionProfile`、signal rule 或 collapse guard 的形式沉淀
- **THEN** core 中不得出现基于仓库名或 reference 标题的专用 planner 分支

