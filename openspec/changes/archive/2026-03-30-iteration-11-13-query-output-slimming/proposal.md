## Why

当前 `spec-wiki wiki query` 直接把 runtime 内部的结构化查询对象原样暴露给外部 CLI，导致默认 JSON 过长、可读性差，也把 `edge_id / symbol_id / score / confidence` 这类偏内部排序与调试用途的字段误暴露成了看起来像正式合同的输出。继续在这套默认 payload 上迭代，只会让宿主和用户更难分辨哪些字段是真正稳定可消费的，哪些只是当前实现细节。

这轮不扩 query 能力上限，也不把 `intent-aware query` 提前做成新协议。目标只是在 `v0.1.0` 已经收敛好的 query contract 之上，把默认外部产物收瘦到“人和宿主都更容易消费”的精简视图，同时保留内部 rich 结构给 runtime 内部与后续 richer query 设计使用。

## What Changes

- 为 `spec-wiki wiki query` 对应的 runtime 外部 payload 增补默认精简视图，去掉默认不该外露的内部标识、评分和调试字段
- 保留 `term / runtime_state / query_mode / query_trust / recommended_action / matched_pages / provenance_summary` 作为稳定核心字段
- 将 `matched_modules / matched_sources / matched_symbols / matched_symbol_edges / matches` 收敛为更可读的默认外部结构，避免继续暴露 `edge_id / symbol_id / page_ids / module_ids / score / confidence / hop_distance` 一类机器侧细节
- 明确这轮不新增 `verbose/debug` 外部 query 协议；内部 rich query 结构继续保留在 runtime 内部边界
- 补充 CLI / transport / e2e 测试，锁住“默认 query 输出应为精简外部视图”的 contract

## Capabilities

### New Capabilities

- 无

### Modified Capabilities

- `repo-wiki-runtime`: query 的正式外部 transport contract 从“直接透传内部 rich 结构”收紧为“默认返回精简视图”
- `wiki-bm25-query`: 默认页面 / symbol / graph query 命中结果的外部字段集收敛为可读视图

## Impact

- Rust: `crates/wiki-runtime/src/transport/cli.rs`、`crates/wiki-runtime/src/workflows/query.rs`、可能新增 query transport mapper
- 测试: `crates/wiki-runtime/tests/acceptance/command_contract.rs`、runtime query 相关测试、`scripts/tests/e2e.test.ts`
- OpenSpec: `specs/repo-wiki-runtime/spec.md` 与 `specs/wiki-bm25-query/spec.md`
