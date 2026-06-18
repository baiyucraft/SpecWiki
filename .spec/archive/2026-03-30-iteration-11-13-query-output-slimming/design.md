## Context

当前 `run_query()` 在 runtime 内部返回的是偏 rich 的 `QueryReport`，其中同时包含：

- 宿主稳定字段：`term / runtime_state / query_mode / query_trust / recommended_action / matched_pages / provenance_summary`
- 结构化命中：`matched_modules / matched_sources / matched_symbols / matched_symbol_edges / matches`
- 一批偏内部字段：`edge_id / symbol_id / source_symbol_id / target_symbol_id / page_ids / module_ids / score / confidence / hop_distance` 等

目前 transport 层在 [crates/wiki-runtime/src/transport/cli.rs](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/transport/cli.rs) 里对 `query` 直接 `as_json(run_query(...))`，因此 `spec-wiki wiki query` 会把内部 rich 结构原样对外输出。用户实际看到的是 runtime 内部对象，而不是经过收口的外部协议。

参考边界：

- [.wiki/06-设计文档/02-Agents设计.md](E:/project/!byAI/spec-wiki/.wiki/06-设计文档/02-Agents设计.md) 已明确 `v0.1.0` 宿主稳定消费应收敛在 `term / runtime_state / query_mode / query_trust / recommended_action / matched_pages / provenance_summary`
- `GitNexus` 的参考价值在于更厚的 query substrate，不在于默认把所有 machine-oriented graph 元数据直接吐给外部 CLI

## Goals / Non-Goals

**Goals:**

- 让 `spec-wiki wiki query` 的默认 JSON 明显变短、可读、可消费
- 保留当前 `v0.1.0` 正式稳定字段不变
- 把默认外部 payload 和内部 rich query 结构解耦，避免后续 richer query 设计被当前 CLI 输出绑定死
- 尽量不影响 runtime 内部测试和未来 richer query 演进空间

**Non-Goals:**

- 不引入新的 `intent-aware query` 外部输入
- 不在这轮新增 `--verbose`、`--debug-query` 或第二套 query action
- 不移除 runtime 内部需要的 rich query 结构
- 不重做 query ranking、graph 扩展或结果截断策略

## Decisions

### 1. 默认只瘦外部 transport 视图，不直接砍掉内部 `QueryReport`

保留 `run_query()` 的内部 rich 结果给 runtime 自己和 Rust 侧测试使用；在 transport 层为 `query` 增加一个显式 mapper，把它映射成默认外部精简 JSON。

原因：

- 用户抱怨的是 `spec-wiki wiki query` 输出过长，不是内部 Rust API 不能存在 rich 结构
- 这样能避免一轮“为 CLI 瘦身”顺手把内部调试与后续 richer query 演进一起砍掉
- `v0.1.0` 的稳定宿主合同本来就比内部实现字段更窄，transport mapper 更符合当前分层

备选方案：

- 直接改 `QueryReport` 字段本体。问题是会把内部测试、未来 richer query 设计和外部默认视图绑死在一起。

### 2. 精简视图保留可读命中，不保留默认机器标识和评分字段

默认外部 payload 保留这些层：

- 稳定顶层字段：`term / runtime_state / query_mode / query_trust / recommended_action / matched_pages / provenance_summary`
- `matched_modules`：保留 `name / kind / root_paths / tags / reasons`
- `matched_sources`：保留 `path / reasons`
- `matched_symbols`：保留 `name / label / file_path / language / reasons`
- `matched_symbol_edges`：保留 `edge_type / source_symbol / target_symbol / traversal_modes / reason`
- `matches`：保留 `title / path / item_type / source_files / reasons / summary / match_mode / provenance`

默认移除这些字段：

- 标识符：`page_id / module_id / source_id / symbol_id / edge_id / source_symbol_id / target_symbol_id`
- 关联回填字段：`page_ids / module_ids`
- 评分与调试字段：`score / confidence / hop_distance` 以及 edge/symbol 级重复理由列表

原因：

- 这些字段对当前默认宿主薄消费不是正式必需
- 绝大多数对人读输出没有帮助，反而把 payload 拉长
- 一旦继续默认暴露，后续就容易被宿主误当成稳定合同

### 3. 这轮不引入 verbose/debug 外部协议

这轮先只做默认外部 payload 瘦身，不新增 `--verbose` 或 debug query 视图。

原因：

- 用户当前痛点是默认输出过长，先把默认路径收好最重要
- 一旦本轮补 `--verbose`，会把 `spec-wiki` CLI、runtime `CoreCommand`、宿主 description 和测试面一起扩大
- rich 字段仍留在 runtime 内部，需要详细信息时，后续可以单独起 iteration 设计 debug/verbose contract

### 4. 测试要同时覆盖“保留什么”和“不得再返回什么”

这轮测试除了保留既有 query 稳定字段，还必须显式断言默认外部 payload 不再包含：

- `edge_id / source_symbol_id / target_symbol_id`
- `symbol_id / page_ids / module_ids / score`
- `confidence / hop_distance`

原因：

- 只有做负向断言，才能防止后续又把内部字段顺手带回外部 transport

## Risks / Trade-offs

- [默认外部 payload 比内部 rich 结构更窄] → 通过显式 transport mapper 保留内部 rich 结构，避免影响 runtime 内部演进
- [部分机器侧消费者可能已经隐式依赖某些内部字段] → 本轮只收紧 `spec-wiki wiki query` / runtime transport 默认输出，并补 acceptance/e2e 测试把正式合同钉住
- [不做 verbose/debug 可能让调试体验暂时变差] → rich 结构仍保留在 runtime 内部，后续若确有需要，再单独为 debug/verbose 起正式 iteration
- [精简过度，误砍掉真实有用信息] → 保留可读的 `matched_modules / matched_sources / matched_symbols / matched_symbol_edges / matches`，只去掉 machine-only 标识和评分字段

## Migration Plan

1. 先在 UniSpec 中定义默认 query 外部 payload 的精简 contract
2. 在 transport 层增加 `QueryReport -> external query payload` 映射
3. 更新 acceptance/runtime/e2e 测试，锁定默认输出边界
4. 保持 `parseResult.ts` 的稳定字段 contract 不变，不扩 `spec-wiki` CLI 参数面

## Open Questions

- `matched_symbol_edges.edge_type` 是否也应进一步收口；当前只投影 `CALLS`，保留它主要是为后续 richer graph output 留接口
- 后续如果要支持 debug/verbose query，应复用现有 action 加参数，还是新增独立调试入口
