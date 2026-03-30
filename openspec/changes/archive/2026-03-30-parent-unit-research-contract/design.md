## Context

当前仓库已经具备 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble`
主链骨架，`wiki-knowledge` 也已经提供了 `KnowledgeDomain / KnowledgeUnit / KnowledgeTree`
以及 `ResearchProvider -> compose_contract_page_for_unit()` 的正式入口。但高层父页仍然保留了一个明显的过渡期短路：

- `page_render.rs` 里的 `unit_uses_seed_backed_research()` 仍会让
  `Overview / Architecture / DomainIndex` 跳过自己的 `UnitResearch`
- `compose.rs` 里的 `compose_contract_page_for_unit()` 仍允许这些高层父页在没有
  `UnitResearch` 时直接用 `SystemResearch / DomainResearch` seed 成页

这与当前已有 spec 已经不一致：

- `knowledge-unit-decomposition` 已要求高层 parent `KnowledgeUnit`
  保持 child-backed 聚合身份
- `research-driven-page-composition` 已要求高层父页必须具备
  unit-scoped parent research contract，并按 child rollup 逐层上卷
- `repo-wiki-runtime` 已要求 parent contract 摘要进入 runtime/cache 主链

当前 change 只解决这一条主线：

- 高层 parent unit 必须拥有自己的 `UnitResearch`
- parent compose 输入必须围绕逐层 child rollup 组织

不在本 change 内同时解决：

- 正式 workflow 的 no-default-structural-fallback
- `.wiki/.knowledge/**` 的完整正式落盘
- query route 扩张
- declared knowledge 生命周期

本设计参考以下本地实现思路，但不照搬其页面语义：

- `CodeWiki`
  - `documentation_generator.py:74-89` 体现 leaf-first 顺序
  - `documentation_generator.py:99-122` 体现父页消费 child docs 的输入纪律
- `deepwiki-rs`
  - `workflow.rs:89-107` 体现 `research -> compose` 分段
  - `compose/agents/overview_editor.rs:26-37` 体现 compose 只消费 research result

## Goals / Non-Goals

**Goals:**

- 让 `Overview / Architecture / DomainIndex / config-surface parent` 都成为显式 parent `KnowledgeUnit` 研究对象
- 去掉高层父页对 seed-only compose 的正式依赖
- 让 parent compose 输入统一围绕 child rollup 组织，而不是越层抓 leaf 或只拼 summary
- 让 runtime/cache 至少能回溯 parent contract 的最小 child-backed 摘要，便于测试、诊断和后续增量复用
- 让 `storybook` 的专项验收可以直接验证 parent contract 是否成立

**Non-Goals:**

- 不在本 change 内移除 production structural fallback
- 不在本 change 内完成 `.wiki/.knowledge/**` 最小正式对象集落盘
- 不在本 change 内改写 query 对外合同
- 不在本 change 内重构完整 dossier 体系
- 不在本 change 内处理完整的 knowledge-first update

## Decisions

### 决策 1：高层 parent unit 一律进入 `UnitResearch` 主线

实现上不再允许 `Overview / Architecture / DomainIndex / config-surface parent`
仅依赖 `SystemResearch / DomainResearch` 直接成页。

具体收口：

- `SystemResearch / DomainResearch`
  - 继续保留
  - 角色降为 parent unit 的 seed / overlay 输入
- `UnitResearch`
  - 成为所有正式页面唯一的 research contract
  - 高层 parent unit 也必须产出自己的 `section_plan / evidence / diagram / child-backed input`

理由：

- 这与现有 spec 一致
- 能把高层父页重新拉回 `KnowledgeUnit` 主线
- 能避免父页长期停留在“模板页 + child summary”状态

备选方案：

- 保留 `Overview / Architecture / DomainIndex` 的 seed-only research
  - 否决：这正是当前 many-to-one reuse 的主要来源

### 决策 2：parent compose 只消费逐层 child rollup

高层父页输入统一收敛为 child-backed compose contract，不允许越层抓取更深层 leaf。

最小输入集：

- `child digest`
- `section-scoped citation digest`
- `diagram digest`
- `child key sources`
- `child readiness`

其中：

- `Overview / Architecture`
  - 只消费 `DomainIndex` 或其他中间 parent unit 上卷结果
- `DomainIndex`
  - 只消费其直接 child unit
- `config-surface parent`
  - 只消费其直接 config 子单元

理由：

- 保持 leaf-first / parent-consume-child 的执行纪律
- 避免高层页再次吞掉深层 leaf 的职责
- 让 runtime 诊断和后续增量刷新都能围绕稳定 parent-child 边界展开

备选方案：

- 高层父页允许缺 digest 时回退到深层 leaf 直读
  - 否决：会重新长回跨层聚合和 many-to-one reuse

### 决策 3：`SystemResearch / DomainResearch` 通过 overlay 合并到 parent `UnitResearch`

本 change 不删除 `SystemResearch / DomainResearch`，而是把它们明确降为：

- 章节 seed
- 高层定位和摘要 seed
- skeleton / section grounding 的 overlay 输入

最终 compose 使用的仍然是 parent 自己的 `UnitResearch` 合同。

理由：

- 可复用现有 research 结果，避免一次性大改
- 能保持高层页已有的一部分稳定输入
- 变更面比直接删除 system/domain research 更小

备选方案：

- 直接删除 `SystemResearch / DomainResearch`
  - 否决：过大，且会把本 change 从 parent contract 扩成 research 架构重写

### 决策 4：runtime 只补最小 parent contract 摘要，不重做完整 lifecycle

本 change 内 runtime 侧只做最小补充：

- parent page context / cache 中必须能回溯
  - `child_unit_ids`
  - `child_page_ids`
  - child digest 引用
  - citation / diagram 摘要引用
  - readiness

不在本 change 内同时重做：

- 完整 checkpoint 语义
- 完整 `.wiki/.knowledge/**` 文件系统产物
- 完整 status 生命周期矩阵

理由：

- parent contract 成立需要最小可观测面
- 但完整 lifecycle 重做属于后续 change

备选方案：

- 在本 change 内把 runtime 生命周期一起收完
  - 否决：明显范围漂移

### 决策 5：本轮专项验收先盯住 `storybook` 的 parent contract，而不是页面数量

`storybook` 的验收口径在本 change 内只回答四件事：

- 高层 parent unit 是否真实产出自己的 `UnitResearch`
- 父页是否只消费直接 child rollup
- runtime/cache 是否能回溯 parent contract 摘要
- 高层 parent unit 的 reuse 是否开始收敛

不把以下内容作为本 change 的通过前置：

- no-default-structural-fallback
- `.wiki/.knowledge/**` 全面落盘
- query 路由升级

## Risks / Trade-offs

- [高层父页切到 `UnitResearch` 后页面内容短期波动] → 先锁稳定 `unit id / relative_path / child boundary`，避免语义变化演变成身份抖动
- [parent rollup 输入收紧后，部分高层页信息密度暂时下降] → 允许通过 seed overlay 保留高层摘要，但禁止再次绕过 parent `UnitResearch`
- [runtime 只补最小摘要，后续仍需继续补生命周期] → 在 spec 中明确本 change 只要求最小可回溯 contract，避免误以为 lifecycle 已收口
- [测试只盯 `storybook` 容易被误解为样本特化] → 验收结论只允许沉淀为通用 parent contract 规则，不允许新增样本专用分支

## Migration Plan

1. 先调整 planning / tree build，确认高层 parent unit 边界与 child 集合稳定
2. 再修改 compose pipeline，移除高层 parent unit 的 seed-only research 短路
3. 让高层 parent unit 统一进入 `UnitResearch`，并通过 overlay 合并 system/domain seed
4. 收紧 parent compose 输入，只允许消费直接 child rollup
5. 补 runtime/cache 的最小 parent contract 摘要
6. 先跑 `storybook` 专项，再补相应 Rust / script 测试；`dagger` 留到后续 change 再补

## Open Questions

- `config-surface parent` 的识别是否完全复用现有 planner 输出，还是需要在本 change 内补一个更显式的 parent 标记
- parent `UnitResearch` 的 input hash 是否需要显式纳入 child digest 引用集合，以便后续增量复用更稳定
- 最小 parent contract 摘要应继续复用现有 page context/cache 字段，还是在 runtime cache 中增加更明确的字段名
