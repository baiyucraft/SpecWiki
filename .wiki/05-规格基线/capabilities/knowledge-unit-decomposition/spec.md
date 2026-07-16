# knowledge-unit-decomposition Specification

## Purpose

定义当前 KnowledgeUnit 确定性启发式 decomposition 的真实边界。Planner 使用稳定代码、文档和 graph signals 选择受控 profile 与 parent/child 单元，但不承诺覆盖任意仓库的通用 typed surface 已完整实现。

```text
decomposition.kind = deterministic_heuristic
decomposition.generic_typed_surface = not_complete
large_repository.full_compose = bounded_fixture_only
```
## Requirements
### Requirement: planner 必须按稳定信号执行确定性启发式拆分
系统 MUST 基于当前已支持的代码结构、模块划分、调用关系、docs anchors、public API、config、testing/example/tutorial 与 graph signals 规划 KnowledgeUnit。Planner MUST 先判定受控 `DecompositionProfile`，再生成匹配的 leaf / parent KnowledgeUnit；未识别信号 MUST 保守落入现有 profile 或 knowledge-only unit，不得声称已完成通用 typed surface。

#### Scenario: docs-heavy 仓库按 API 与配置面拆出独立单元
- **WHEN** 某个 docs-heavy 仓库同时存在稳定的 docs anchors、public API exports 和配置入口
- **THEN** planner MUST 在已识别信号满足当前规则时将其拆分到不同的 `DecompositionProfile`
- **THEN** 未满足规则时 MUST 保持确定性且暴露当前启发式边界，不得用 provider 输出临时发明新类型

#### Scenario: runtime-heavy / compiler-heavy 仓库按运行时、编译链和测试面拆分
- **WHEN** 某个仓库同时存在 runtime 模块、compiler/codegen 模块、testing/example 目录和稳定的 process/community 信号
- **THEN** planner MUST 为当前已支持的稳定 signals 生成对应 profile 的 KnowledgeUnit
- **THEN** 自动化证据只证明受控 fixture，不构成任意大仓 full compose 或质量 SLA

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

### Requirement: 高层 parent KnowledgeUnit 必须保持 child-backed 聚合身份
系统 MUST 将 `Overview`、`Architecture`、`DomainIndex`，以及某个 domain 下 `decomposition_profile = config_surface` 的 parent `KnowledgeUnit`，作为显式的 parent `KnowledgeUnit` 聚合节点处理。系统 MUST 为这些节点维护稳定的 child 集合、逐层上卷输入身份、parent/child 关系以及后续 `UnitResearch` 输入身份，而不是把它们重新退化为仅由页面类型驱动的空壳总览页。系统 MUST NOT 为此引入基于仓库名、reference 标题或固定目录结构的专有 planner 分支。

#### Scenario: config surface 聚合仍属于 domain 内的 parent KnowledgeUnit
- **WHEN** planner 在某个 domain 下发现多个 `config_surface` 子单元，并需要生成汇总性的配置父页
- **THEN** 系统 MUST 将该父页对应到同一 domain 下的 parent `KnowledgeUnit`
- **THEN** 系统 MUST 通过 `decomposition_profile = config_surface` 与 child unit 集合表达其身份
- **THEN** 系统 MUST 让该 parent unit 继续作为后续 `UnitResearch` 与 child rollup 的稳定输入身份
- **THEN** 系统 MUST NOT 为该场景引入新的页面语义类型或样本仓库特判

#### Scenario: 高层 parent unit 身份与 child 边界重复运行保持稳定
- **WHEN** 同一仓库的高层 `Overview`、`Architecture`、`DomainIndex` 或 `config_surface` parent unit 在 child 集合未变化的情况下重复执行 `init` 或 `rebuild`
- **THEN** 这些 parent `KnowledgeUnit` 的稳定 id、relative_path 和 child 边界 MUST 保持一致
- **THEN** 系统 MUST 能把后续的 child rollup、`UnitResearch` 和 compose 输入继续映射回这些稳定身份
