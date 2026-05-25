## MODIFIED Requirements

### Requirement: 页面生成必须采用 research-first compose，而不是模板后补
系统 MUST 在正式 renderer 落盘之前，先为 `system / domain / unit` 三级研究对象生成 provider-backed research 结果，再由 compose 计划生成正式页面。`init / update / rebuild` 的正式 runtime MUST 使用 provider-backed `ResearchProvider`，不得继续默认绑定 `StructuralResearchProvider`。`StructuralResearchProvider` 仅 MAY 用于测试、fixture 和显式开发调试场景。

#### Scenario: 正式 workflow 先形成 provider-backed research 结果再落盘
- **WHEN** 系统执行正式的 `init`、`update` 或 `rebuild`
- **THEN** workflow MUST 先生成 provider-backed 的 `SystemResearch / DomainResearch / UnitResearch`
- **THEN** renderer MUST 依据这些 research 结果驱动 compose，而不是只把 summary 插入固定模板

#### Scenario: StructuralResearchProvider 不再作为正式 runtime 默认实现
- **WHEN** 系统执行正式 workflow 且存在可用 provider 配置
- **THEN** workflow MUST 不得默认选择 `StructuralResearchProvider`
- **THEN** 结构型 provider 仅可在测试或显式开发模式下使用

### Requirement: 父页 compose 必须显式消费子页结果
系统 MUST 让 parent compose 显式消费子页的 `PageDigest`、section-scoped citation digest、diagram digest 和 key sources，而不是只消费轻量 summary。父页不得回退为主要依赖扁平 facts 或 surface 列表重做整页内容。

#### Scenario: 父页消费子页结构化结果而不是扁平 summary
- **WHEN** domain index、overview、architecture 或其它 parent unit 存在已完成的 leaf/child 页面
- **THEN** 父页 compose 输入 MUST 包含这些子页的 digest、citation digest 或 diagram digest
- **THEN** 父页不得只消费一句 child summary 后重新组织全部内容

### Requirement: compose 计划必须支持 section-scoped citation plan
系统 MUST 让 compose 计划中的每个 section 显式表达其 `evidence_refs`，并让 renderer 把 citation / evidence 以最终 Markdown 可识别形式落在该 section 的作用域内。citation 不得继续只停留在 `PageDraft`、`PageContext` 或 cache 表中。

#### Scenario: section 级 citation 正式进入最终页面
- **WHEN** 某个 compose section 命中了明确的 `evidence_refs`
- **THEN** renderer MUST 在该 section 内或紧随该 section 写出可统计的 citation / evidence block
- **THEN** reference 报告脚本必须能从最终 `.wiki/*.md` 识别这些 citation

### Requirement: compose 层不支持 deterministic fallback
系统 MUST NOT 在 compose 阶段提供退化到模板填充的正式 runtime 路径。Research 与 Compose 的正式 workflow 输出 MUST 由 provider-backed LLM 生成；若 provider 不可用且当前不是测试/显式开发模式，workflow MUST 失败并保存检查点，而不是回退为低质量模板输出。

#### Scenario: 正式 runtime 无 provider 时拒绝退化执行
- **WHEN** 正式 workflow 进入 Research 或 Compose 阶段但不存在可用 provider
- **THEN** pipeline MUST 返回错误并保存检查点
- **THEN** 系统 MUST NOT 使用模板填充继续生成页面
