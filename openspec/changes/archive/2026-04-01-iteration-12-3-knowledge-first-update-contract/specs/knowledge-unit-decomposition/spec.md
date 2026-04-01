## MODIFIED Requirements

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
