## ADDED Requirements

### Requirement: symbol FTS 查询链路必须保留定义行号
系统 MUST 让 `symbols_fts` 查询结果在 facts/query 链路中保留 symbol 定义行号，而不是在进入 runtime transport 之前丢失。

#### Scenario: symbol FTS 命中保留 start_line/end_line
- **WHEN** `symbols_fts` 命中了某个 symbol，且 `symbols` 表中存在 `start_line/end_line`
- **THEN** `SymbolSearchHit`、`SymbolHit` 与 runtime query symbol 投影 MUST 继续保留这些行号
