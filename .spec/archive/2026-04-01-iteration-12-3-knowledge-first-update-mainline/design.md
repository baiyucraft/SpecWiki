## Context

当前仓库在 `12.2` 已经把 `.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json + .wiki/.cache/**` 的正式分层和 cold restore 闭环扶正，但 `update` 主线仍然没有真正切到 knowledge-first：

- `crates/wiki-runtime/src/domain/change_set.rs` 当前把 `ChangeSet` 直接映射为 `AffectedSet { module/page/section }`，还没有独立的 affected knowledge scope
- `crates/wiki-runtime/src/workflows/update.rs` 虽然会对 symbols/edges 做局部 refresh，但后续仍调用 `run_compose_pipeline_with_action(...)` 生成整棵 `knowledge_tree`、`page_drafts` 与 `digests`，只是最终按 `input_hash` 跳过部分页面写盘
- 这意味着现在的增量行为更接近“全树 knowledge/research/compose + 局部页面写回”，而不是 `.wiki/06-设计文档/01-Runtime设计.md` 里定义的 `detect affected facts -> locate affected knowledge scope -> refresh derived knowledge -> refresh impacted projections`

`12.3` 的设计目标不是再补一个更复杂的 dirty-page planner，而是把 `KnowledgeUnit` 一等抽象真正接到 `update` 主线上。设计参考遵循本仓库边界：

- `tmp/upstream/deepwiki-rs`
  - 参考其显式 `preprocess -> research -> compose` 分段编排，支持把 scoped refresh 插在 research / compose 前，不照搬其产品形态
- `tmp/upstream/codewiki/codewiki`
  - 参考 leaf-first、child-first、parent-late 的汇总顺序，用来约束 parent unit 的增量传播，而不是沿用其模块页面语义
- `tmp/upstream/GitNexus`
  - 参考 detect-changes / affected-scope / impact analysis 的思路，帮助定义“变化先定位作用域，再执行刷新”的 contract；不把 execution-flow / PR review 产品形态照搬进 core
- `tmp/upstream/deepwiki-open`
  - 继续只作为 query / session / consumption 侧参考，不进入本轮 core update 设计

## Goals / Non-Goals

**Goals:**

- 把 `update` 的正式主线收口为 `ChangeSet -> affected knowledge scope -> refresh derived knowledge -> refresh impacted projections`
- 定义独立的 `AffectedKnowledgeScope`，覆盖直接受影响单元、父级传播单元、移除单元、受影响 domain 与 projection targets
- 让局部源码变化优先触发局部 knowledge refresh，而不是默认执行整棵 knowledge/research/compose 主链
- 明确 parent unit 的增量传播条件，避免高层 parent 在 child digest 未变化时被无谓重算
- 让 `.wiki/.knowledge/**`、`page-digests`、runtime gates 与页面投影在 update 后保持同一 snapshot 身份
- 让 `status / update` 的诊断语义能解释“为什么这些知识单元被刷新、为什么需要更大范围 refresh 或 rebuild”

**Non-Goals:**

- 不在本轮把 query 对外合同升级成 `v0.2.0` 正式发布面
- 不在本轮引入 declared knowledge 的 authoring / sync / conflict lifecycle
- 不重写 `KnowledgeUnit` 的拆分体系本身；`12.3` 消费既有稳定 unit 身份，而不是重开 decomposition 设计
- 不把 page projection 重新提升为 update 的主本体，也不以最终 Markdown fidelity 作为本轮首要门禁
- 不借机把 `sync / rebuild` 提升为新的公开 workflow surface

## Decisions

### 决策 1：在 `ChangeSet` 与 `AffectedSet` 之间新增一层 `AffectedKnowledgeScope`

`12.3` 引入独立的 `AffectedKnowledgeScope`，作为 `update` 的正式中间产物。它至少表达：

- `direct_unit_ids`
  - 由脏源码、graph 依赖扩散、docs/config/testing/example 信号变化直接命中的 leaf / parent `KnowledgeUnit`
- `propagated_parent_unit_ids`
  - 由于 child digest、child membership、parent contract 输入变化而需要重新汇总的祖先 unit
- `removed_unit_ids`
  - 上一轮存在、当前规划已消失的 unit
- `affected_domain_ids`
  - 因 unit 增删或 domain membership 变化而需要刷新摘要的 domain
- `projection_target_page_ids`
  - 需要新增、更新、删除的 page projection targets
- `escalation`
  - `local_refresh / subtree_replan / repo_replan / rebuild_recommended` 之一，以及对应 reason

`AffectedSet` 不删除，但退居 projection 层结果：它不再是 `ChangeSet` 的第一落点，而是由 `AffectedKnowledgeScope` 派生出的投影刷新集合。
`AffectedSet` MUST NOT 再与 `AffectedKnowledgeScope` 并列成为 update planner 的一级输出，更不能继续驱动 knowledge scope planning。

理由：

- 当前 `AffectedSet` 直接围绕 page/section，会把 `update` 错写成“页面 dirty repair”
- `.wiki/06-设计文档/01-Runtime设计.md` 已把“locate affected knowledge scope”写成正式主链；缺的只是具体 contract

备选方案：

- 继续扩展当前 `AffectedSet`
  - 否决：会把 KnowledgeUnit、domain、projection target 混进同一个 page-first DTO
- 让 `update` 直接比较前后 `page_digests`
  - 否决：`page_digests` 是 projection anchor，不是 knowledge scope 真相

### 决策 2：增量刷新顺序固定为“facts 先行，knowledge 决策，projection 末端提交”

`update` 的内部顺序固定为：

1. refresh facts/index substrate
2. 根据当前规划与上一轮正式 snapshot 计算 `AffectedKnowledgeScope`
3. 定向失效并重建受影响 unit 的 research / summary / digest
4. 仅对受影响 projection target 执行 page compose / assemble
5. 提交 `.wiki/.knowledge/**`、`.wiki/pages/**`、`wiki.metadata.json` 与 `.cache/**` 的同一 snapshot

其中：

- facts/index 仍允许沿用当前 scoped graph refresh 与必要时 full graph refresh 的实现
- knowledge refresh 不再默认对整棵 `knowledge_tree` 运行 research / compose
- parent unit 是否刷新，取决于 child digest 或 parent aggregate input 是否变化，而不是“任何 child touched 都重跑整条祖先链”

理由：

- `deepwiki-rs` 的显式 `research -> compose` 分段有利于在中间插入 scope-bound refresh
- `CodeWiki` 的 leaf-first / parent-late 顺序能约束 parent 传播只发生在 child contract 变化后

备选方案：

- 保留当前“全树 pipeline + 局部页面写盘”
  - 否决：语义上仍不是 knowledge-first，也会持续制造无意义的 provider/research 开销

### 决策 3：affected scope 基于“上一轮正式知识快照 + 当前规划”做 diff，而不是反推 Markdown

本轮 scope planning 的输入基线是：

- 上一轮正式 `.wiki/.knowledge/derived/knowledge-units.jsonl`
- `.wiki/.knowledge/derived/knowledge-tree.json`
- `.wiki/.knowledge/runtime/page-digests.jsonl`
- 恢复后的本地 mirror / cache（若存在）
- 当前 facts/index 驱动出的新规划结果

scope diff 优先比较：

- unit stable id
- unit source / module / docs anchor membership
- child membership
- parent aggregate input hash
- projection decision 与 target page id

系统 MUST NOT 通过最终 Markdown 反推 knowledge scope，也不能把页面存在与否当作 unit 变化的唯一依据。
系统 MUST 以 `knowledge_units / knowledge_tree` 这类 formal identity objects 作为 scope planning 主输入；`planned_pages`、`page_digests` 与最终 Markdown 只允许作为 projection 层派生输入，不得反向主导 unit/doman scope 计算。

理由：

- `12.2` 已经把 `.wiki/.knowledge/**` 扶正为 formal truth；`12.3` 应直接消费这层，而不是继续让页面反向主导知识
- Markdown 只适合做 projection merge，不适合做 affected knowledge planning

备选方案：

- 继续只读 SQLite mirror
  - 否决：会让正式 artifact 退化成旁路备份，而不是 update 真相输入

### 决策 4：整树 replan 仍属于 knowledge-first update，不等于 lifecycle rebuild

`12.3` 采用分级升级策略：

- `local_refresh`
  - 脏源码能稳定映射到既有 unit / parent aggregate，且 projection target 可局部确定
- `subtree_replan`
  - 存在结构变化，但变化被限制在某个 domain / subtree，能重算局部知识树
- `repo_replan`
  - workspace root、入口集、关键 config surface 或 decomposition signal 改变，必须重新规划整棵知识树，但仍属于 knowledge-first update 的范围，而不是 runtime lifecycle rebuild
- `rebuild_recommended`
  - 正式 artifact、cache mirror、projection/runtime state 之间一致性破坏，或 unit identity 无法可信恢复

只有 `rebuild_recommended` 才允许把建议动作提升到 rebuild。整树 knowledge replan 仍然是 `update` 主线的一部分，结构变化本身不自动等于 rebuild。

理由：

- 当前实现把缺页/缺 cache 之类 runtime 破坏与普通结构变化混在同一套回退感觉里，粒度太粗
- `v0.2.0` 的 D 段需要的是 knowledge-first update，不是“更多 full rebuild”

备选方案：

- 结构变化统一 full rebuild
  - 否决：会让 knowledge-first update 对真实仓库几乎没有可用价值

### 决策 5：formal artifact 在 update 中按 scope 定向刷新，并以 snapshot commit 结束

`.wiki/.knowledge/**` 的 formal artifact 继续是正式真相，但 `update` 时必须支持：

- 仅刷新受影响 unit / domain / parent aggregate 对应的 summary 与 digest
- 删除 `removed_unit_ids`、`removed_page_ids` 对应的 formal records / anchors
- 重新生成 `recovery-manifest` 与 metadata snapshot 锚点
- 保持未受影响 formal records 的稳定身份与内容不漂移

实现上允许文件级重写，但语义上必须是 scope-bound refresh，而不是“因为文件格式方便所以整仓知识都算 touched”。

理由：

- `12.2` 回答了“写什么”；`12.3` 必须补上“什么时候重写、什么时候保持稳定”

备选方案：

- artifact 文件每次全量重写且不承诺稳定性
  - 否决：会破坏 update 可解释性，也会让 snapshot diff 噪声过大

### 决策 6：验证以 `storybook + dagger` 的增量解释性为主

本轮验证重点切到：

- 小范围源码改动是否只命中局部 knowledge scope
- parent propagation 是否只在 child contract 变化时发生
- formal artifact 与 projection anchors 是否按 scope 刷新
- `status / update` 是否给出清晰的 escalation reason

专项门禁仍优先 `storybook + dagger`。完整项目集 `init` 分析与 `.wiki/02-开发指南/00-代码注释规范.md` 合规检查继续保留为 tasks/verification 约束，而不是与主专项门禁同权的设计目标。

理由：

- `storybook + dagger` 能验证 docs-heavy / runtime-heavy 两端的知识范围传播
- 全量 19 项目在本轮仍是 baseline guard，但不是 primary product gate

## Risks / Trade-offs

- [affected scope 算错会导致漏刷或过刷] → 用稳定 `unit_id`、child membership 与 parent aggregate hash 做双重判定，并在无法可信定位时显式升级 escalation
- [parent propagation 过于保守会漏掉高层汇总页] → parent refresh 判定同时比较 child digest 集合与 parent own research input，而不是只看直接源码命中
- [formal artifact 与 SQLite mirror 双写期间可能短暂不一致] → 以 snapshot commit 为最终提交边界；只有 metadata / recovery-manifest 一起更新后才视为当前 update 成功
- [storyboard / dagger 上局部改动仍可能触发较大范围 provider 开销] → 允许 `subtree_replan / repo_replan`，但必须把原因暴露出来，不能伪装成局部刷新成功
- [引入新 scope DTO 会扩大 runtime/knowledge 边界] → 把 scope contract 放在 KnowledgeUnit 主线，不让 page/section 语义重新长成核心抽象

## Migration Plan

1. 先为 `ChangeSet -> AffectedKnowledgeScope -> AffectedSet` 建立新 contract，并保留当前 `AffectedSet` 作为 projection 层结果。
2. 让 `update` 先基于正式 artifacts 与当前规划计算 scope diff，再把 research / compose 收窄到受影响 unit 集合。
3. 调整 formal artifact 与 projection anchor 的 update 提交策略，补齐删除和 snapshot 锚点更新。
4. 调整 `status / update` 的推荐动作与错误摘要，使其能表达 `local_refresh / subtree_replan / repo_replan / rebuild_recommended` 的原因。
5. 先跑 `storybook + dagger` 的增量专项，再补完整项目集 `init` 分析和生命周期回归。

## Open Questions

- `AffectedKnowledgeScope` 是否需要直接暴露到外部 `status` 详细字段，还是仅先用于内部诊断与测试报告；当前倾向先内部化，仅对外暴露 escalation reason 与 touched summary
- formal artifact 文件格式是否需要为将来的更细粒度 patch 做准备；当前倾向保持现有文件布局不变，只先承诺 scope-bound 语义而不承诺 patch-level 写盘实现
