## ADDED Requirements

### Requirement: provider-backed research 路径必须为 system / domain / unit 输出 source-grounded fidelity contract
系统 MUST 在 runtime 已选择 provider-backed research 路径时，让 `SystemResearch`、`DomainResearch` 和 `UnitResearch` 都输出可直接服务最终页面 fidelity 的 source-grounded contract。该 contract MUST 至少覆盖 `section_plan`、`skeleton_profile`、`key_source_clusters`、`evidence_clusters` 和 diagram 输入，而不是继续只让 `unit research` 增强、把 `system/domain research` 长期停留在 structural summary。

#### Scenario: provider-backed 路径下 system / domain research 不再只有 structural summary
- **WHEN** 正式 workflow 已选择 provider-backed research 路径并生成 overview、architecture 或 domain index 相关页面
- **THEN** `SystemResearch` 和 `DomainResearch` MUST 产出可供 compose 直接消费的 `skeleton_profile`、`key_source_clusters` 或等价的 source-grounded 输入
- **THEN** 系统 MUST NOT 继续只把 `unit research` 视为 source grounding 的唯一承担者

#### Scenario: 非 provider-backed 路径不由 9.8 重新定义可用性政策
- **WHEN** runtime 仍走当前保留的 bridge、fallback 或显式开发/测试路径
- **THEN** 9.8 的 fidelity contract MAY 只约束已进入 provider-backed 主链的 research 输出
- **THEN** 本轮变更 MUST NOT 单独重写现有 provider availability 或 fallback 的全局可用性政策

### Requirement: 所有正式页面都必须消费统一的 ComposePageContract
系统 MUST 让 leaf 页和 parent 页统一消费 `ComposePageContract`。该 contract MUST 至少包含 `section_plan`、`skeleton_profile`、`section_grounding_refs`、`key_source_clusters`、`evidence_clusters`、`diagram_suggestions`、`child_digest_rollup`、`child_section_citation_digest`、`child_diagram_digest`、`child_key_sources` 与 `child_readiness`。`Overview`、`Architecture`、`DomainIndex` 与 `config_surface` parent unit MUST 一并使用这套 contract，不得继续依赖固定骨架或轻量 child summary 主导正文。

#### Scenario: 高层父页使用显式 child digest / citation / diagram / readiness 输入
- **WHEN** 系统生成 `Overview`、`Architecture`、`DomainIndex` 或 `config_surface` parent unit
- **THEN** 该页面的 compose 输入 MUST 显式包含 `child_digest_rollup`、`child_section_citation_digest`、`child_diagram_digest`、`child_key_sources` 和 `child_readiness`
- **THEN** 系统 MUST NOT 继续主要依赖 `render_child_digest_summary` 或固定 section 模板组织正文

#### Scenario: leaf 页也使用同一 ComposePageContract
- **WHEN** 系统生成 leaf `KnowledgeUnit`
- **THEN** leaf 页 MUST 继续通过同一 `ComposePageContract` 消费 `section_plan`、`skeleton_profile`、`section_grounding_refs`、`key_source_clusters` 和 `evidence_clusters`
- **THEN** child 相关字段 MAY 为空，但 contract 结构 MUST 保持一致
