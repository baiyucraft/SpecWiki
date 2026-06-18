## Context

迭代 2 已经把 `WikiState`、`MetadataMapper` 和 `query` 的状态消费路径落下来了，但 runtime 仍停在“全量构建 + 状态落盘”的阶段：

- `status.rs` 只比较源码路径和 fingerprint，`dirty_pages` 只是由页面 `source_paths` 粗粒度反推
- `update.rs` 遇到 `stale` 直接 `run_init()`，没有受影响页面计算，也不会复用 page context / generation 结果
- `.wiki/.cache/` 里目前只有 `repo-scan.json`、`module-tree.json`、`wiki-state.json` 三份文件，而且除了 `status` 的完整性检查外，几乎没有真正参与运行时决策
- `generation/sections.rs` 和 `generation/renderer.rs` 仍然把页面当成整页字符串处理，没有 section 级稳定身份

迭代 3 的设计约束来自三个方向：

- 本仓库边界：必须继续遵守 `Repository Scan -> ModuleTree -> Page Planner -> Wiki Assembler` 主路径，不能绕过 planner 直接拼页面
- 下一迭代前置：必须预埋 section-level 重生成接口，但不能提前实现迭代 4 的用户内容保留
- 开发参考：
  - `tmp/upstream-deepwiki-rs/src/cache/mod.rs`、`tmp/upstream-deepwiki-rs/src/generator/workflow.rs` 展示了 cache 与 workflow 阶段解耦、按类别复用的做法
  - `tmp/upstream-deepwiki-rs/src/generator/preprocess/extractors/structure_extractor.rs` 展示了把结构提取结果写入 cache 再被后续阶段消费的思路
  - `tmp/upstream-codewiki/codewiki/src/be/dependency_analyzer/dependency_graphs_builder.py`、`tmp/upstream-codewiki/codewiki/src/be/agent_tools/generate_sub_module_documentations.py` 展示了“先建立依赖/模块图，再递归处理受影响子树”的组织方式
  - `tmp/upstream-deepwiki-open` 只作为消费层和树形呈现参考，不作为 core runtime 的实现模板

## Goals / Non-Goals

**Goals:**

- 建立 `ChangeSet / AffectedSet` 内核，把源码变化稳定映射到模块、页面和 section
- 让 `status` 成为增量 runtime 的预判入口，而不是仅返回是否脏
- 让 `update` 在局部可修复时只重建受影响页面，并保持未受影响页面不重写
- 为 `init` 和 `rebuild` 补齐增量 runtime 所需的 page context / generation cache 初始化
- 为迭代 4 预留稳定 section ID、section hash 和页面组装接口
- 把测试收敛到“哪些页面应该被触达、哪些页面不该被重写、哪些情况必须升级为 full rebuild”

**Non-Goals:**

- 不实现用户自定义 section 保留、merge 或 managed/unmanaged 编辑协议
- 不引入 LLM、RAG、TOON 或 explanation layer 能力
- 不引入文件监听器、数据库或外部缓存服务
- 不改动 Agent 层 JSON IPC 的主结构
- 不承诺本迭代实现“零扫描”更新；首要目标是避免每次都重跑完整 planner/context/render 流程

## Decisions

### 决策 1：引入独立的 `ChangeSet / AffectedSet` 内核，而不是继续在 `DirtyState` 上堆字段

当前 `DirtyState` 只表达外部状态结论，无法承载“为什么这些页面受影响”以及“是否还能局部修复”。本迭代新增两层内部模型：

- `ChangeSet`
  - 表达 `added_sources / modified_sources / removed_sources`
  - 表达 `structural_sources`（manifest、workspace、入口、模块边界相关变更）
  - 表达 `requires_replan`、`requires_rebuild`
- `AffectedSet`
  - 表达 `affected_module_ids`
  - 表达 `affected_page_ids`
  - 表达 `affected_section_ids_by_page`
  - 表达 `removed_page_ids`

`DirtyState` 继续只作为外部可导出的状态摘要；`status` 和 `update` 内部统一先计算 `ChangeSet`，再得出 `AffectedSet`。

备选方案：

- 方案 A：继续只保留 `dirty_sources + dirty_pages`
  - 否决原因：无法区分局部可修复与必须重建，也无法承载 section 级影响面
- 方案 B：把所有中间字段都塞进 `DirtyState`
  - 否决原因：`DirtyState` 既会导出到 metadata，又会出现在 workflow 响应里，内部运算字段会污染外部协议

### 决策 2：扩展 `WikiState` 为“增量状态内核”，并把 section 状态纳入页面状态

`WikiState` 会从“页面/源码/模块/关系快照”扩展为“可驱动增量更新的内核”：

- `WikiPageState`
  - 新增 `input_hash`：当前页面上下文输入的稳定指纹
  - 新增 `sections: Vec<WikiSectionState>`
  - 保留 `content_hash`，作为整页装配后的落盘摘要
- `WikiSectionState`
  - `section_id`
  - `title`
  - `managed`
  - `content_hash`
  - `source_ids`
  - `relation_ids`
- `SourceState`
  - 保持 `source -> page/module` 映射
  - 新增或补齐局部重建所需的结构标签时，只用于内部状态，不导出到 metadata

这样 `WikiState` 就能在不读取整页 Markdown 的情况下判断：

- 某个页面的输入是否真的变化
- 页面里哪些 section 的输入发生了变化
- 哪些页面只是受模块关系/架构摘要牵连，而不是直接受源码牵连

备选方案：

- 方案 A：section 状态只放 generation cache，不进 `WikiState`
  - 否决原因：`status` 和 `update` 的核心判断会依赖一份“可丢弃的缓存”，违背状态内核边界
- 方案 B：等迭代 4 再引入 section 状态
  - 否决原因：届时会迫使 update 从整页覆盖重构为 section 替换，代价更高

### 决策 3：把 cache 切成“仓库级快照 + 每页缓存”，而不是继续只有三份整仓缓存

保留现有三份仓库级缓存：

- `.wiki/.cache/repo-scan.json`
- `.wiki/.cache/module-tree.json`
- `.wiki/.cache/wiki-state.json`

新增两类每页缓存：

- `.wiki/.cache/page-contexts/<page-id>.json`
  - 保存 `PageContext` 和 `input_hash`
- `.wiki/.cache/page-generation/<page-id>.json`
  - 保存 section 级渲染产物、section hash、装配前草稿

cache invalidation 规则按层处理：

- 仅源码内容变更且模块树不变：
  - 只失效受影响页面的 page context / generation cache
- 模块树变化但页面集合仍可重算：
  - 重建 `module-tree.json`
  - 失效受影响页面及其祖先页面的 per-page cache
- 页面缺失、cache schema 不兼容、状态文件损坏：
  - 直接提升为 `needs_rebuild`

这里仍允许 `status`/`update` 做目录遍历；但必须避免“任意一个源码变化就整仓重建 planner/context/render”的行为。

备选方案：

- 方案 A：只保留单个 `generation-cache.json`
  - 否决原因：难以按页面精确失效，也不利于测试验证“未触达页面不重写”
- 方案 B：完全不新增 cache，只靠 `WikiState`
  - 否决原因：无法复用 page context 和 section 渲染结果，`update` 仍会退化成半全量重建

### 决策 4：`status` 和 `update` 共享同一套 change planning 内核

本迭代新增统一入口，例如 `plan_runtime_changes(repo_root, runtime_bundle) -> ChangePlan`。`ChangePlan` 内含：

- `change_set`
- `affected_set`
- `next_scan_report`
- `next_module_tree`（仅在需要时构建）
- `next_page_plan`（仅在需要时构建）
- `fallback_mode`（`none / init / rebuild`）

工作流分工：

- `status`
  - 只计算 `ChangePlan`
  - 返回 `fresh / stale / missing / needs_rebuild`
  - `dirty_pages` 直接来自 `affected_set`
- `update`
  - 复用 `ChangePlan`
  - `fresh` 时直接 no-op
  - `stale` 时只重建受影响页面
  - `missing` 时走 `init`
  - `needs_rebuild` 时走 `rebuild`
- `rebuild`
  - 显式忽略增量 cache，重新生成整套 runtime

这样可以避免 `status` 和 `update` 各自维护一套不同的脏判断逻辑。

备选方案：

- 方案 A：`status` 只做简单比较，`update` 自己重算受影响页面
  - 否决原因：逻辑容易漂移，测试也会分裂

### 决策 5：页面内部改成 section 级渲染与组装，但本迭代仍按整页落盘

`generation/sections.rs` 会从“返回 `Vec<String>`”改成“返回稳定 section 结构”，例如：

- `SectionDraft`
  - `section_id`
  - `title`
  - `managed`
  - `source_ids`
  - `relation_ids`
  - `content`

`renderer.rs` 负责渲染 section 内容，`Wiki Assembler` 负责把 section 草稿组装成 Markdown。`update` 的最小重建单元是 section，但落盘仍以页面文件为单位：

- 如果页面的 section 输入没有变化，则不重写该页面文件
- 如果页面里只有部分 section 变化，则只重渲染对应 section，再重新组装该页面

这满足迭代 3 的“section-level 设计预埋”，但不提前实现迭代 4 的用户内容保留。

备选方案：

- 方案 A：继续用整页字符串渲染，到迭代 4 再拆
  - 否决原因：会把增量更新和可编辑 runtime 的重构成本推迟到下一轮
- 方案 B：现在就上 managed/unmanaged merge
  - 否决原因：超出当前迭代边界，会混入用户内容同步规则

### 决策 6：结构变化优先触发“局部 replan”，只有关键运行时不一致才触发 full rebuild

不是所有结构变化都必须删除 `.wiki/` 重来。区分三类：

- 局部源码变化
  - 直接走增量 update
- 局部结构变化
  - 例如新增源码、删除源码、manifest 改动导致模块树变化
  - 允许重建 `ModuleTree` 和 `PagePlan`
  - 只对新增、删除、受影响祖先链页面做写盘
- 关键运行时不一致
  - 页面文件缺失但 metadata/state 仍声明存在
  - cache schema 版本不兼容
  - `WikiState` 与 `repo-scan/module-tree` 无法建立一致映射
  - 这类场景直接升级为 `needs_rebuild`

这样 `update` 不会因为“新增一个模块页”就粗暴删除所有页面，但也不会在半损坏 runtime 上继续局部拼补。

## Risks / Trade-offs

- [增量路径明显变复杂] → 通过 `ChangePlan` 统一 `status/update/rebuild` 的决策，避免判断逻辑散落
- [WikiState 体积上升] → 只把 section 摘要、hash 和映射放进状态层，正文内容继续留在 generation cache
- [结构变化边界容易误判] → 先把 manifest/workspace/entry-point/module-tree diff 列为显式 structural signal，并为新增/删除源码补专门测试
- [per-page cache 数量增多] → 使用稳定 page id 作为文件名，避免路径漂移；同时在 `rebuild` 时统一清理
- [metadata 回退能力与增量更新能力不再等价] → 明确区分“能理解正式索引”和“能执行增量更新”两种能力；前者可由 metadata 支撑，后者要求完整 runtime cache
- [本迭代仍可能保留一次目录遍历] → 接受目录遍历成本，但避免全量 context/render/rewrite；优先解决重生成范围，而不是先做文件监听器级优化

## Migration Plan

1. 先扩展 generation 层为 section 结构，补齐 page context / generation cache 读写
2. 再引入 `ChangeSet / AffectedSet / ChangePlan`，让 `status` 使用统一变化规划
3. 然后把 `update` 从 `run_init()` 替换为增量 apply 路径，并补齐页面新增/删除清理
4. 最后更新测试：单文件修改、结构变化、cache 缺失回退，以及测试项目集 `init` 分析

回滚策略：

- 如果增量路径不稳定，可以保留 `rebuild` 和 `init` 作为兜底
- 在实现阶段应当先让 `update` 在不满足增量前提时自动回退，而不是在半成品状态上尝试局部修复

## Open Questions

- 当前无必须在 proposal 阶段阻断实现的开放问题；实现时若发现 `status` 的快速扫描还需要额外 stat 字段，可在同一 change 内补充到 `ScanReport` 的内部 cache 字段，但不改变外部协议
