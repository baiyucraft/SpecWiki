## ADDED Requirements

### Requirement: research-driven compose 结果必须沉淀为最小正式知识摘要与恢复锚点
系统 MUST 让正式 workflow 产出的 provider-backed `SystemResearch / DomainResearch / UnitResearch` 与 compose 结果，沉淀为 `.wiki/.knowledge/**` 中的最小正式对象，而不是继续只停留在 `research_cache`、`page_digests`、`page_drafts` 或其它运行期缓存。沉淀范围 MUST 收敛为：

- `knowledge_domains / knowledge_units / knowledge_tree` 对应的 identity snapshot
- parent / unit research 摘要
- `page_digests`
- runtime gates / readiness 摘要

该 requirement 只约束“结果如何沉淀为正式 artifact”，MUST NOT 借机改写 leaf-first、parent rollup、section plan、citation policy 或 provider policy 的既有 compose contract。

#### Scenario: unit research 与 parent research 形成正式 summary artifact
- **WHEN** 正式 workflow 已生成某个 `KnowledgeUnit` 的 provider-backed research 结果
- **THEN** 系统 MUST 将该结果的最小 summary 写入 `.wiki/.knowledge/derived/**`
- **THEN** 系统 MUST NOT 继续只把该结果留在运行期 `research_cache`

#### Scenario: compose 结果形成 projection / recovery anchor
- **WHEN** 系统已完成某个页面的 compose 与 projection 绑定
- **THEN** 系统 MUST 将 `page_digests` 与对应 runtime gates / readiness 摘要写入 `.wiki/.knowledge/runtime/**`
- **THEN** 这些对象 MUST 被标记为 projection / recovery anchor，而不是 knowledge identity 本体
