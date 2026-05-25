## Why

当前仓库已经有 `index -> knowledge -> page fallback` 的 query 骨架，也已经把 `declared / derived / projection / health` 收进 formal runtime artifacts，但“knowledge query 到底怎么参与路由”仍没有被正式写死。若继续只靠 `query_mode`、临时 provenance tag 和实现内排序维持行为，后续很容易重新滑回 page-first，或把 answer assembly、UI 展示、复杂 ranking 一起混进 query change。

因此这轮需要单独收敛一个更窄的 child change：只把 `knowledge query routing contract` 正式化，固定 `symbol -> graph -> declared knowledge -> derived knowledge -> page` 的路由、优先级、降级语义与 provenance 合同，为后续 answer assembly 建立稳定前置，而不是提前把 answer 本身做掉。

## What Changes

- 新增 `knowledge-query-routing` capability，正式定义 `symbol -> graph -> declared knowledge -> derived knowledge -> page` 的 route contract、truth boundary、route tags 与 degraded policy。
- 修改 `repo-wiki-workflow`，把 `query` 的正式稳定合同从“有 query_mode 即可”收紧为 knowledge-first route contract，明确各层参与条件、fallback 边界与宿主可依赖字段。
- 保持 `wiki-index-query-surface` 与 `wiki-bm25-query` 作为下层前置 substrate 参考，本轮不重写其 taxonomy 或 BM25 contract，只在新的 route contract 中正式消费它们。
- 固定本轮非目标：不做 answer assembly，不做 host/UI 展示，不做 intent-aware 外部输入，不做复杂 cross-layer rerank，不把 governance decision workflow 混入 query route。

## Capabilities

### New Capabilities
- `knowledge-query-routing`: 定义 knowledge-first query routing 的正式对象、优先级、route tags、degraded policy 与 fallback 边界。

### Modified Capabilities
- `repo-wiki-workflow`: 收紧 `query` 的正式稳定合同，明确 knowledge route、trust、recommended_action 与 provenance 的职责边界。

## Impact

- 主要影响 [query.rs](/E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/workflows/query.rs)、`wiki-runtime` 的 query transport / DTO，以及 `wiki-index` 暴露给 runtime 的 query substrate 边界。
- 影响 `.wiki/.knowledge/**` 中 `declared`、`derived`、`runtime` artifacts 如何被 `query` 正式消费，但本轮不新增 answer artifact。
- 影响 `query` 相关测试与后续 `storybook + dagger` / 全量项目的 query 路由验收口径。
