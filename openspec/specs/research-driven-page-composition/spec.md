# research-driven-page-composition Specification

## Purpose
TBD - created by archiving change iteration-9-3-targeted-dossier-and-research-driven-pages. Update Purpose after archive.
## Requirements
### Requirement: research 结果必须驱动正式页面 section 结构
系统 MUST 让 `overview`、`architecture`、`module` 和 `topic` 页在进入 research 后由结构化 `section_plan` 驱动正式页面 section 结构，而不是继续仅把 research 结果当作摘要补丁。`section_plan` MUST 至少包含稳定 `section_key`、`section_title`、`section_summary`、`evidence_refs`、`diagram_refs` 和 `child_refs`，并继续禁止模型直接返回最终 Markdown 页面。

#### Scenario: research 结果为页面提供 section 计划
- **WHEN** 某个 `overview`、`architecture`、`module` 或 `topic` 页完成 research
- **THEN** 最终结果 MUST 包含稳定的 `section_plan`
- **THEN** renderer MUST 依据 `section_plan` 组织正文 section，而不是只把 `summary` 插入固定模板

#### Scenario: section 计划仍受 runtime contract 约束
- **WHEN** 系统把 research 结果转换为正式页面
- **THEN** 每个 section MUST 继续映射到稳定的 managed section identity
- **THEN** 模型不得直接返回最终 Markdown 文件或绕过现有 renderer

### Requirement: section 计划必须由受控证据与图输入支撑
系统 MUST 让 `section_plan` 只引用当前页面已有的 evidence、child rollup 和 deterministic diagram inputs。research 结果 MAY 重排章节顺序和表达重点，但不得凭空发明新来源、新子页或新结构图。

#### Scenario: section 计划引用现有 evidence 和子页结果
- **WHEN** `section_plan` 中某节声明 `evidence_refs` 或 `child_refs`
- **THEN** 这些引用 MUST 命中当前 dossier 或 child rollup 中已有的稳定标识
- **THEN** renderer MUST 能基于这些引用回溯到实际来源或子页摘要

#### Scenario: section 计划不得虚构结构图
- **WHEN** `section_plan` 中某节声明 `diagram_refs`
- **THEN** 每个 `diagram_ref` MUST 指向当前页面已有的 deterministic diagram input
- **THEN** 若页面不存在对应图输入，系统 MUST 回退到无图 section，而不是接受模型虚构的图结构

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

### Requirement: compose 主链必须支持 leaf-first page composition
系统 MUST 让 `family-leaf-doc`、叶子模块页和高置信专题页优先完成 research 与 compose，再由 `family-child`、`family-index`、`overview` 和 `architecture` 消费这些叶子结果。系统不得继续让父页主要依赖 surface 清单或扁平 facts 直接成页。

#### Scenario: family child 优先消费 leaf doc 结果
- **WHEN** 某个 `family-child` 下已经存在稳定的 `family-leaf-doc` 结果
- **THEN** `family-child` 的 compose MUST 优先消费这些 leaf doc 的 `compose plan / key sources / citation plan / child digest`
- **THEN** 父页不得回退为只消费 surface 路径列表

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

### Requirement: 高层父页必须具备 unit-scoped parent research contract
系统 MUST 让 `Overview`、`Architecture`、`DomainIndex` 与 `config_surface` parent unit 在进入正式 compose 前拥有自己的 unit-scoped parent research contract。该 contract MUST 至少表达稳定的 `section_plan`、evidence/diagram 引用和 child-backed research 输入。系统 MUST NOT 再允许这些高层父页仅依赖 `SystemResearch`、`DomainResearch` 或固定 section 模板直接成页；`SystemResearch / DomainResearch` 若存在，MUST 只作为 parent `UnitResearch` 的 seed / overlay 输入。

#### Scenario: overview 不再只靠 system research 直接成页
- **WHEN** 系统生成 `Overview` 页面
- **THEN** 该页面 MUST 先具备面向该 parent unit 的 `UnitResearch`
- **THEN** compose MUST 依据该 research contract 组织 section，而不是只把 `SystemResearch` 插入固定骨架

#### Scenario: domain index 不再只靠固定骨架消费 child 摘要
- **WHEN** 系统生成某个 domain 的 `DomainIndex`
- **THEN** 该页面 MUST 具备该 parent unit 自己的 `section_plan`
- **THEN** compose MUST 消费 child-backed evidence、citation 或 diagram 摘要，而不是只把 child summary 拼进固定 section

#### Scenario: system 和 domain research 只作为 overlay
- **WHEN** 系统已生成某个高层 parent unit 的 `UnitResearch`
- **THEN** `SystemResearch` 或 `DomainResearch` MUST 只作为该 `UnitResearch` 的 seed / overlay 输入
- **THEN** 系统 MUST NOT 让这些 seed 直接替代 parent unit 自己的 research contract

### Requirement: 父页 compose 输入必须按 child rollup 逐层上卷
系统 MUST 让 parent compose 输入以 child rollup 的形式逐层上卷。高层父页 MUST 通过直接 child unit 持久化出的 rollup 消费下层结果，而不是跨层直接抓取更深层 leaf 输入。上卷结果 MUST 至少包含 child digest、section-scoped citation digest、diagram digest、key sources 和 readiness 摘要。

#### Scenario: overview 通过 domain rollup 消费下层结果
- **WHEN** `Overview` 或 `Architecture` 需要消费 domain 下更深层 leaf 的研究结果
- **THEN** 系统 MUST 先由各 `DomainIndex` 或中间 parent unit 上卷出 child rollup
- **THEN** `Overview` 或 `Architecture` MUST 只消费这些逐层上卷结果
- **THEN** 系统 MUST NOT 让高层父页直接越过中间 parent 节点读取深层 leaf 输入

#### Scenario: config surface 父页消费 child rollup 而非子页摘要拼接
- **WHEN** 某个 `config_surface` parent unit 存在多个 child unit
- **THEN** parent compose 输入 MUST 包含这些 child unit 的 rollup 摘要
- **THEN** 系统 MUST NOT 仅通过 `PageDigest.summary` 或等价 child summary 直接拼出父页正文

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

