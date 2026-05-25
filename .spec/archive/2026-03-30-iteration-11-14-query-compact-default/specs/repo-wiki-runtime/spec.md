## ADDED Requirements

### Requirement: runtime 默认 query transport 必须输出 compact summary 与统一 hits
系统 MUST 将默认 query transport 收敛为 compact payload。默认外部结果 MUST 保留 `term`、`runtime_state`、`query_mode`、`query_trust`、`recommended_action`、`matched_pages` 和 `provenance_summary`，并新增统一的 `summary` 与 `hits` 视图。

#### Scenario: 默认 query transport 返回 compact payload
- **WHEN** 外部调用方通过正式 transport 调用 `query`
- **THEN** 返回结果 MUST 包含 `summary`
- **THEN** 返回结果 MUST 包含统一的 `hits`
- **THEN** `hits` MUST 可表达 page、symbol、source、module 和 call edge 线索

#### Scenario: 内部 rich query 结构不直接透传
- **WHEN** 外部调用方消费默认 query transport
- **THEN** 默认 payload MUST NOT 直接返回 `matched_modules`
- **THEN** 默认 payload MUST NOT 直接返回 `matched_sources`
- **THEN** 默认 payload MUST NOT 直接返回 `matched_symbols`
- **THEN** 默认 payload MUST NOT 直接返回 `matched_symbol_edges`
- **THEN** 默认 payload MUST NOT 直接返回 `matches`
