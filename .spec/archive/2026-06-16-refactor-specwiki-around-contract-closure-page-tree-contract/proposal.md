# refactor-specwiki-around-contract-closure-page-tree-contract

## 问题

新版 SpecWiki 的设计草案已经确定用户可见 Wiki 页面树应采用 `.wiki/INDEX.md`、栏目 `INDEX.md` 和 `NN-主题.md` 的单一结构，但当前稳定设计、历史 capability baseline 和阶段草案中仍存在 `.wiki/pages/**` 作为 runtime page projection 目标的旧口径。

如果不先收口页面树，后续 truth kind、restore、projection、query、governance 和 CLI 变更都会继续面对两套 page truth：

- 新版用户可见页面树：`.wiki/**/*.md`，排除 `.wiki/.knowledge/**`、`.wiki/.cache/**` 和 `.wiki/pages/**`。
- runtime surface 外的旧目录：`.wiki/pages/**`。

这会导致 metadata、query、update 和 rebuild 行为互相猜测，后续实现容易重新长出兼容层。

## 目标

- 将新版用户可见 Wiki 页面树确认为唯一 runtime 目标页面树。
- 停止把 `.wiki/pages/**` 作为 `init / update / rebuild` 的写入目标。
- 让默认 `query` 不读取 `.wiki/pages/**`。
- 让 `update` 不刷新 `.wiki/pages/**`，也不把它纳入 `AffectedProjectionScope`。
- 让 `rebuild / restore / metadata` 只消费正式页面树。
- 为后续 truth / restore / projection / query child changes 提供稳定页面树前提。

## 非目标

- 不实现 truth kind、两级 restore、snapshot manifest 或 readiness 状态；这些属于 `refactor-specwiki-around-contract-closure-truth-restore-snapshot`。
- 不实现 declared authoring writeback、managed section merge 或 projection ownership 重构；这些属于 `refactor-specwiki-around-contract-closure-projection-writeback-boundaries`。
- 不实现完整 query route DTO、ranking 或 human output；这些属于 `refactor-specwiki-around-contract-closure-query-route-readiness`。
- 不实现 code graph schema、raw captures 或 phase DAG；这些属于 `refactor-specwiki-around-contract-closure-code-graph-index`。
- 不实现 governance isolation、archive dry-run 或 operation manifest。
- 不迁移、不清理、不诊断 `.wiki/pages/**`；旧目录不属于新版 SpecWiki runtime surface。

## 成功标准

- `init / update / rebuild` 的 runtime 写入目标不再包含 `.wiki/pages/**`。
- metadata、state、SQLite `wiki_pages / wiki_pages_fts` 只记录正式页面树。
- 默认 `query` 不读取 `.wiki/pages/**`，不返回旧目录内容作为正式结果。
- `restore / rebuild` 不从 `.wiki/pages/**` 重建正式 metadata 或 runtime state。
- `update` 不刷新 `.wiki/pages/**`，也不把旧目录纳入 `AffectedProjectionScope`。
- `status` 不为 `.wiki/pages/**` 提供专门状态、诊断字段或清理建议。
- 文档、测试和 fixture 不再把 `.wiki/pages/**` 写作新版目标目录。

## 影响范围

- Runtime workflow：`init / update / rebuild / query / restore` 的页面树读写边界。
- Runtime metadata：page path、page id、reverse refs 和正式页面树过滤口径。
- Wiki 文档规范：`.wiki/INDEX.md`、栏目 `INDEX.md`、`NN-主题.md` 的唯一页面树规则。
- Tests / fixtures：涉及 `.wiki/pages/**` 的 runtime、query、restore、status 断言。
- 设计文档：需要把仍然把 `.wiki/pages/**` 表述为目标目录的草案改为 runtime surface 外目录口径。

## 交付形态

single-change

这是 `refactor-specwiki-around-contract-closure` parent 下的第 1 个 child change。它为后续 truth / restore / projection / query / graph / CLI / governance 重构建立页面树前提。

## 风险

- 旧测试或 fixture 可能默认断言 `.wiki/pages/**` 存在，需要集中更新。
- 现有 runtime 可能把 metadata 恢复逻辑和任意 page path 耦合较深，实际设计阶段需要识别拆分点。
- 如果正式页面树 predicate 不够集中，下游仍可能各自接受非正式路径。

## 未知项

- 当前代码中页面写入、读取、metadata 导出、restore 和 query fallback 的完整路径链需要在 design 阶段确认。
- 是否已有 fixture 依赖 `.wiki/pages/**` 作为正式页面树，需要在 design 阶段盘点。
- 正式页面树的目录深度是否需要在 planner 中进一步收紧到某种固定层级，需要在 design 阶段判断。

## 参考资料

- `.docs/design/specwiki-contract-closure.md`
- `.docs/design/governance-runtime-integration.md`
- `.docs/design/knowledge-to-wiki-projection-contract.md`
- `.wiki/00-文档约定/00-边界与SSOT规则.md`
- `.wiki/00-文档约定/01-页面模板.md`
- `.spec/changes/refactor-specwiki-around-contract-closure/split.md`
