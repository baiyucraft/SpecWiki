## Why

默认 compact query payload 现在已经足够薄，但 `symbol` 命中没有任何行号信息，Agent 很难直接定位到定义位置。当前行号并不是底层没有，而是在 `symbols_fts -> SymbolSearchHit -> SymbolHit -> QuerySymbolMatch -> transport` 的链路里被裁掉了。

## What Changes

- 为默认 query 的 `symbol` hit 补回 `start_line/end_line`
- 将 compact symbol hit 的 `location` 收口为 `path:line`
- 同时保留 `line_start/line_end` 供机器消费
- 不扩 `source / module / page / call_edge` 的行号合同

## Capabilities

### Modified Capabilities

- `repo-wiki-runtime`
- `wiki-bm25-query`
