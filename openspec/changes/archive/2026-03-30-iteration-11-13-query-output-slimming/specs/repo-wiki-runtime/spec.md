## ADDED Requirements

### Requirement: runtime transport 的默认 query 响应不得直接透传内部 rich query 结构
系统 MUST 把 runtime 内部 rich query 结构与默认外部 transport payload 分离。`run_query()` 内部 MAY 继续保留 richer 的查询对象以服务 runtime 内部逻辑、Rust 侧测试或后续 richer query 设计；但通过正式 transport 返回给外部调用方的默认 query 响应 MUST 经过显式映射，收敛到精简可读的公开视图。

#### Scenario: transport 对 query 结果执行显式映射
- **WHEN** 外部调用方通过正式 transport 调用 `query`
- **THEN** transport MUST 把内部 query 结果映射为默认公开 payload
- **THEN** transport MUST NOT 直接把内部 rich query 结构原样序列化给外部

#### Scenario: 稳定 query 字段保持不变
- **WHEN** 外部调用方消费默认 query 响应
- **THEN** `term`、`runtime_state`、`query_mode`、`query_trust`、`recommended_action`、`matched_pages` 和 `provenance_summary` MUST 保持可用
- **THEN** 本次收口 MUST NOT 通过删除这些稳定字段制造 breaking contract
