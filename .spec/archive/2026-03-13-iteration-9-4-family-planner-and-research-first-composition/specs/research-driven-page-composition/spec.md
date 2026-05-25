## ADDED Requirements

### Requirement: 页面生成必须采用 research-first compose，而不是模板后补
系统 MUST 在正式 renderer 落盘之前，先为 `overview`、`architecture`、`family-index`、`family-child`、`module` 和 `topic` 页生成结构化 compose 计划。compose 计划 MUST 至少包含页面定位、section 顺序、section 摘要、evidence refs、diagram refs 和 child refs。renderer MUST 基于 compose 计划写出正式页面，而不是继续把 research 结果当作固定模板的补丁。

#### Scenario: family 或 overview 页面先形成 compose 计划再落盘
- **WHEN** 某个 family、overview 或 architecture 页完成 research
- **THEN** 系统 MUST 先得到结构化 compose 计划
- **THEN** renderer MUST 依据该计划组织 section，而不是只把 summary 插入固定模板

#### Scenario: compose 计划仍受 runtime contract 约束
- **WHEN** 系统依据 compose 计划写出正式页面
- **THEN** managed section identity、marker 和增量更新边界 MUST 保持稳定
- **THEN** 系统不得让模型直接返回最终 Markdown 并绕过 renderer

### Requirement: 父页 compose 必须显式消费子页结果
系统 MUST 让 parent compose 显式消费子页的 `compose plan`、`evidence rollup`、`diagram rollup` 和 `key sources`，而不是只消费轻量 summary。系统 MAY 为子页结果生成压缩后的 `child_page_digest`，但父页不得回退为主要依赖扁平 facts。

#### Scenario: family index 或 overview 组合子页结果
- **WHEN** family index、overview 或 architecture 页存在多个已生成子页
- **THEN** 父页 compose MUST 显式吸收这些子页的结构化结果
- **THEN** 父页不得仅凭 child summaries 重新组织全部内容

#### Scenario: 子页结果可压缩但不可丢失结构
- **WHEN** 系统为父页构造 child page digest
- **THEN** digest MUST 保留子页的 section 结构、关键来源和主题定位
- **THEN** 系统不得把子页结果压缩到只剩一句 summary
### Requirement: compose 主链必须支持 leaf-first page composition
系统 MUST 让 `family-leaf-doc`、叶子模块页和高置信专题页优先完成 research 与 compose，再由 `family-child`、`family-index`、`overview` 和 `architecture` 消费这些叶子结果。系统不得继续让父页主要依赖 surface 清单或扁平 facts 直接成页。

#### Scenario: family child 优先消费 leaf doc 结果
- **WHEN** 某个 `family-child` 下已经存在稳定的 `family-leaf-doc` 结果
- **THEN** `family-child` 的 compose MUST 优先消费这些 leaf doc 的 `compose plan / key sources / citation plan / child digest`
- **THEN** 父页不得回退为只消费 surface 路径列表

### Requirement: compose 计划必须支持 section-scoped citation plan
系统 MUST 让 compose 计划中的每个 section 显式表达其 `evidence_refs`，并让 renderer 按 section 作用域落 citation / evidence block。系统不得只在页面级统一附加来源块。

#### Scenario: section 级 citation 随 compose plan 落盘
- **WHEN** 某个 section 的 compose 计划命中了明确的 `evidence_refs`
- **THEN** renderer MUST 在该 section 内或紧随该 section 输出相应 citation / evidence block
- **THEN** 其它 section 不得被动继承这组来源，除非它们显式复用同一组 `evidence_refs`

### Requirement: compose 层不支持 deterministic fallback
系统 MUST NOT 在 compose 阶段提供退化到模板填充的路径。compose 输出（`PageDraft` / `PageDigest`）MUST 全部由 LLM 驱动生成。系统 MUST 依赖 Research 层的 LLM 输出作为 compose 的唯一内容来源。

#### Scenario: compose 不退化为模板填充
- **WHEN** compose 层接收到 `UnitResearch` 输入
- **THEN** 段落展开、图表生成和 section 内容 MUST 由 LLM 产出
- **THEN** 系统 MUST NOT 使用硬编码模板或 Facts 字段直接拼接作为 compose 输出
