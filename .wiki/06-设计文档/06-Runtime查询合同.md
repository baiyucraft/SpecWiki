---
title: Runtime 查询合同
description: Runtime、CLI 与 Agents 共同消费的 query 输入、路由、排序、状态和错误 authority
owner: architecture
updated: 2026-07-16
---

# Runtime 查询合同

## 文档定位

本页是 Repo Wiki Runtime query 对外合同的唯一 authority。`wiki-model` 定义跨 crate DTO 对象语言，`wiki-runtime` 独占 route、ranking、readiness、trust、recommended action 和 error policy 决策；TypeScript parser、CLI 与 Agents 只解析和呈现 Runtime 结论。

[01-Runtime设计](./01-Runtime设计.md) 只保留运行主链摘要，[02-Agents设计](./02-Agents设计.md) 只保留宿主消费责任。两者不重复定义本页的字段闭集或状态机。

## 稳定请求

公开 query 只接受非空 `term`：

```text
spec-wiki query <term...> [--json]
IPC: { action: "query", repoRoot, term }
```

- 入口必须在执行查询前验证 `term.trim()` 非空。
- 多个位置 token 由 CLI 合并为一个 term。
- `intent / focus / scope / traversal` 不是当前公开 request；宿主不得通过私有参数或 prompt 模拟。

## Canonical 响应

成功响应的稳定字段为：

| 字段 | 语义 |
| --- | --- |
| `term` | Runtime 实际执行的查询词 |
| `runtime_state` | 查询时的 runtime 总体状态 |
| `readiness` | index、knowledge 和 projection 的分层可用性 |
| `query_mode` | fusion 与 fallback 的顶层摘要，不表示排名 |
| `query_trust` | `ready / stale_but_queryable / blocked` |
| `recommended_action` | Runtime 给出的恢复或核验动作 |
| `governance` | 与 core query trust 正交的治理摘要 |
| `route_groups` | 按 route 组织的结果主合同 |
| `answer` | Runtime 基于实际 supporting refs 生成的结论包络 |

`route_groups` 是唯一结果 authority，即使无命中也必须序列化为空数组。顶层平铺 `results` 以及 `matched_pages / provenance_summary / summary / hits` 均已从公开 transport 删除，不是派生视图或兼容入口。Runtime 内部 rich report 可保留装配中间数据，但不得泄漏为宿主合同。

## Route 与 result

### Route 闭集

| Route tag | 对象层 |
| --- | --- |
| `index_symbol_hit` | 源码符号 |
| `index_path_hit` | 源码路径 |
| `index_module_hit` | 结构化模块，对应 `source_module` ref |
| `index_graph_hit` | 已稳定投影的 graph 关系 |
| `knowledge_declared_hit` | declared knowledge |
| `knowledge_derived_hit` | derived knowledge |
| `governance_evidence_ref` | 治理 evidence ref |
| `governance_summary_hit` | 治理摘要 |
| `projection_ref` | page / section projection |
| `rendered_page_debug_fallback` | 仅用于调试恢复的 rendered page fallback |

每条 result 必须包含 `route_tag / ref_kind / ref_id / label / rank / provenance / confidence / recommended_action / source_refs`；`score` 是可选的 route-local 原始值。`rank` 从 1 开始，是消费者应使用的组内次序。

每个 route group 必须包含 `route_tag`、`ranking_basis`、`score_direction`、`total_count`、`returned_count`、`truncated` 和 `results`。先按 route policy 排序，再以 `ref_kind / ref_id` 稳定 tie-break，然后去重、截断并分配 rank。

### Ranking 规则

| 来源 | `ranking_basis` | `score_direction` |
| --- | --- | --- |
| SQLite FTS | `bm25` | `lower_is_better` |
| exact/path/module 结构化命中 | `structural_match` | `none` |
| graph | `graph_confidence` | `higher_is_better` |
| knowledge/projection/fallback | `deterministic_match` | `none` |

score 的量纲和方向只在所属 group 内有意义。group 的固定输出顺序不表示全局 relevance，消费者禁止跨 route 比较 score 或自行合并排名。BM25 raw rank 不参与 high/medium/low confidence 阈值判断。

## Fusion、provenance 与 answer

Runtime 执行 formal layer fusion：index、knowledge、governance 和 projection 在各自 readiness 允许时并行贡献 route。只有 formal 结果不足时才启用 `rendered_page_debug_fallback`；fallback 的 provenance 必须是 `fallback / fallback`，confidence 必须是 low，且 query/answer trust 必须显式降级。

`provenance.layer` 闭集为 `index / knowledge / governance / projection / fallback`，`provenance.state` 闭集为 `ready / stale / missing / rebuilding / conflict / blocked / not_enabled / fallback`。declared/derived 是 route 类型，不是 readiness state。

- index stale 但仍可查询时，可返回真实 `stale` 命中，顶层 trust 为 `stale_but_queryable`。
- index missing/blocked 时禁止伪造 index route；knowledge/projection 或 fallback 可用时仍可显式降级返回。
- 无正式结果且无 fallback、core 又不可用时，trust 为 `blocked`，不生成伪命中。
- governance blocker 只影响 governance route/action，不降低已 ready 的 index/knowledge core trust。
- `answer.answer_mode / answer.answer_trust / supporting_refs` 由 Runtime 基于实际 route 和 core trust 生成；TS 和 Agents 不再推导。

## 错误、空结果与恢复

| 场景 | 合同 |
| --- | --- |
| term 缺失或 trim 后为空 | failure，`invalid_argument`，不执行 query |
| runtime/index 未初始化 | failure，`index_not_ready`，`recommended_action=init` |
| 已初始化但局部层可恢复 | success，通过 readiness/trust 和 `rebuild/update` 表达降级 |
| 合法 term 无命中 | success，`route_groups=[]`，返回 empty `answer` |
| 非预期 I/O 或协议错误 | `workflow_failed` 或 `protocol_error`，不伪装成空成功 |

TS parser 对缺失 `route_groups`、缺失 `answer`、未知闭集值或旧 query 字段 fail closed。CLI 与 Agents 应原样呈现 typed error 和 Runtime 的 recommended action。

## Agents 消费边界

宿主先读取 `readiness / query_mode / query_trust / recommended_action / governance / route_groups / answer`，按 Runtime 提供的组内 rank 和 supporting refs 回答。宿主不复制 route 枚举、排序算法、状态矩阵或 answer 推导。需要精确实现细节时，必须继续打开 source refs 并核验源码。

## Richer query 延期边界

| 能力 | 当前判定 | 升级条件 |
| --- | --- | --- |
| explicit intent/focus/scope/traversal | 延期 | request schema、默认规则、各入口 parity 与系统测试完成 |
| owner | 延期 | owner facts authority、confidence 与 source refs 完成 |
| entrypoint | 延期 | 稳定 route/ref、ranking 和宿主验收完成 |
| callers/callees/impact | 延期 | traversal/depth/truncation 及独立输出 schema 完成 |
| process/community | 延期 | workflow 实际填充、route/ref、ranking/provenance 与质量证据完成 |
| semantic/vector/LLM rerank | 非目标 | 另立 change，不得成为当前 deterministic 主链前提 |

`index_module_hit / source_module` 是已进入稳定合同的例外；其它内部 substrate 的存在不等于对外承诺。

## 维护规则

- 修改 query 公开字段、route/ref 闭集、ranking 或错误分类时，必须同步共享 DTO、Runtime transport、TS parser、Agents 资产和合同测试。
- proposal、design、review 和 archive 仅是 change artifact 或历史证据，不复制到本页，也不取代当前 authority。
- 本合同未使用 upstream；不存在直接迁移、改写或仅借鉴的外部实现。
