## Why

当前默认 `spec-wiki wiki query` 虽然已经去掉了一批内部 ID 和评分字段，但外部 payload 仍然保留了多套并列数组，Agent 仍要为 `matched_modules / matched_sources / matched_symbols / matched_symbol_edges / matches` 付出额外 token 和解析成本。这和当前 query 的目标相悖：让宿主以尽量低的 token 成本快速定位页面、源码、符号和调用线索。

这轮继续保持内部 `run_query()` rich report 不动，只收紧默认 transport 视图，把它变成真正的 `summary + hits` 结构。

## What Changes

- 默认 query transport 改成 `summary + hits` 的 compact payload
- 继续保留稳定顶层字段：`term / runtime_state / query_mode / query_trust / recommended_action / matched_pages / provenance_summary`
- 默认不再返回 `matched_modules / matched_sources / matched_symbols / matched_symbol_edges / matches`
- 将页面、符号、源码、模块、调用边压平成统一 `hits`
- 更新 CLI / acceptance / e2e / lifecycle 脚本断言，锁住新的默认形状

## Capabilities

### Modified Capabilities

- `repo-wiki-runtime`
- `wiki-bm25-query`
