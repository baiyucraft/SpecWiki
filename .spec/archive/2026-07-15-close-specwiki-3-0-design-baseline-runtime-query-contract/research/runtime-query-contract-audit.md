# Runtime / Query 合同技术审计

## 调研目的

- 阶段：design
- 关联 change：close-specwiki-3-0-design-baseline-runtime-query-contract
- 服务边界：design
- 要回答的问题：现有 Runtime、index、transport、TS parser、CLI、Agents 与 Wiki 在 query 主合同、ranking、readiness、错误和延期能力上有哪些真实不一致，应如何收口。
- 停止条件：能够确定当前稳定接口、必须修复的代码偏差、明确延期能力和可自动化验证方向。

## 结论摘要

- 当前稳定外部输入继续使用非空 `term`；intent、focus、scope、traversal 不提升为公开输入。
- `route_groups` 是唯一主结果合同；扁平 `results` 会诱导跨 route 比较，和 route-local ranking 冲突，应与 `matched_pages`、`provenance_summary`、`summary`、`hits` 一并从公开 transport 与 TS 主类型删除。内部 rich report 可暂时保留供 workflow 装配。
- 当前路由是 index、knowledge、governance、projection 并行 fusion，rendered page 仅在正式层不足时作为条件 debug fallback；不能继续写成严格串行 cascade。
- SQLite FTS5 BM25 原值越小越相关，当前代码却按越大越可信推导 confidence，并对 source 结果按路径重排。公开 group 必须显式声明 ranking basis 与 score direction；result score 可选并保留 route-local 量纲，使用独立 `rank` 表达稳定次序。confidence 不再由 BM25 数值阈值推导；跨 route 不比较。
- raw Rust transport 与 TS parser 必须把 `route_groups` 作为同一个必备主字段，空结果也序列化为空数组，parser 缺字段必须报协议错误。
- 缺失或空白 term 必须在 Rust IPC 与 TS CLI 都返回 `invalid_argument`；index 未就绪必须返回稳定 `index_not_ready` 和 `init`/`rebuild` 恢复提示，不能压成通用 `workflow_failed`。
- owner 输出无稳定事实 owner；entrypoint 和 impact 仅存在 index 内部 substrate，process/community 也未形成 route/result schema，全部延期，不通过宿主私有字段模拟。

## 已读取资料

| 路径 / 来源 | 目的 | 关键发现 |
| --- | --- | --- |
| `proposal.md` | 固定目标和成功标准 | 要求跨 Runtime、CLI、Agents 统一合同，并明确 richer query 边界。 |
| `.wiki/06-设计文档/01-Runtime设计.md` | 核对当前 Runtime authority | 已声明 term-only 与 route/result 结构，但仍保留扁平结果副本，route 和 score 细节不足。 |
| `.wiki/06-设计文档/02-Agents设计.md` | 核对宿主消费 | 仍把 `matched_pages/provenance_summary` 列为稳定字段，和 Runtime 漂移。 |
| `.wiki/05-规格基线/capabilities/repo-wiki-runtime/spec.md` | 核对旧 capability 约束 | 仍写严格 `index -> knowledge -> page fallback` 和旧 `provenance_summary` 主要求。 |
| `crates/wiki-model/src/domain/query.rs` | 核对公开 DTO owner | 已定义 route/result/source ref 共享对象语言。 |
| `crates/wiki-index/src/query.rs` | 核对 index substrate | 内部已有 intent、entrypoint、impact slice、process/community，但外部 runtime 只用 Auto。source FTS 命中会被路径排序覆盖。 |
| `crates/wiki-runtime/src/storage/sqlite_store.rs` | 核对真实 score | SQLite FTS5 直接返回 `bm25()`，按升序排列。 |
| `crates/wiki-runtime/src/workflows/query.rs` | 核对 fusion、trust 和结果装配 | facts 与 knowledge 并行收集，page 条件 fallback；symbol/source/graph provenance state 不一致，score/confidence 量纲混用。 |
| `crates/wiki-runtime/src/transport/query_payload.rs` | 核对 raw JSON | route groups、扁平结果与多个旧派生视图并列；空 route groups 被省略。 |
| `crates/wiki-runtime/src/transport/cli.rs`、`dto.rs` | 核对 IPC 输入与错误 | Rust term 可缺失并变成空串；query 错误统一压成 workflow_failed。 |
| `packages/spec-wiki/src/runtime/parseResult.ts` | 核对 TS 类型和 parser | 类型要求主数组，但 parser 缺失时静默补空数组。 |
| `packages/spec-wiki/src/agents/shared/commandAssets.ts` | 核对宿主生成资产 | 仍引导宿主优先看旧 matched page/provenance summary。 |
| `.spec/archive/2026-06-18-refactor-specwiki-around-contract-closure-query-route-readiness/` | 核对历史证据 | 已落地 route DTO 与 readiness，但 archive 不是当前 authority，也未收口本轮发现的跨层偏差。 |

## 子代理调研

| 子代理 / 角色 | 调研主题 | 结论 | 主会话复核 |
| --- | --- | --- | --- |
| query source auditor | Runtime/index/transport/Agents 协议与实现 | 识别 BM25 方向、source 重排、宿主旧字段、term/error、字段必备性六类偏差。 | accepted；逐项用源码复核，并纳入 design。 |
| 主会话独立测试/文档审计 | 测试缺口与稳定文档迁移 | 当前测试固化旧派生字段，没有真实 BM25 排序、raw/parser parity、IPC invalid/index-not-ready 与 Agents 新字段断言。 | accepted；作为 plan 的 Red 测试方向。 |

## 关键发现

### Score 和 confidence 语义当前不可成立

- 证据：`sqlite_store.rs` 使用 FTS5 `bm25()`；`wiki-index/query.rs` 对 source hits 再按 path 排序；`wiki-runtime/query.rs` 使用 `>= 0.8` / `>= 0.5`。
- 说明：FTS5 原始 rank 越小越相关，且可能为负值；当前阈值会把高相关命中错误标为 low。
- 影响：FTS route 保留 lower-is-better score 但在 group 中明确方向，结构化 route 可无 score；所有 group 都生成稳定 rank。confidence 按来源 authority、diagnostics、layer readiness 和 fallback 状态计算，不读取 BM25 阈值。

### 主合同与派生视图形成双轨

- 证据：Rust model/transport 与 Runtime Wiki已有 route groups；Agents Wiki/资产仍列 matched pages/provenance summary，E2E 也只断言旧字段。
- 说明：宿主会继续按旧 summary 推断结果来源，绕过逐结果 provenance。
- 影响：公开 payload 删除旧派生字段和扁平结果副本，宿主资产和 E2E 迁移到 route groups；内部 rich workflow 字段暂不在本 change 全量删除，避免扩大内部重构。

### Route 实际是 fusion，不是严格 cascade

- 证据：workflow 无论 index 是否命中都会读取 knowledge；只有 rendered page fallback 受条件控制。
- 说明：旧 `index -> knowledge -> page fallback` 表述会误导后续场景和宿主把 knowledge 当成 index miss 才启用。
- 影响：canonical 规则写为 `index + knowledge + governance + projection fusion -> conditional rendered fallback`。

### 输入、字段存在性和错误分类跨入口不一致

- 证据：TS CLI 拒绝空 term，Rust IPC 接受；raw JSON 省略空数组，TS parser 补默认；index NotFound 被压成 workflow_failed。
- 说明：raw CLI、bridge 和 npm API 观察到不同协议。
- 影响：transport 先做 term 校验，空数组必序列化，parser fail closed；新增 `index_not_ready` error kind 与结构化恢复数据。

### Richer query 事实不足以提升为公开合同

- 证据：index 内部已有 `IndexQueryIntent`、entrypoint、impact slice、process/community，但 runtime request 只传 Auto，公开 route/ref kind 无 owner/process/community/impact 结果模型。
- 说明：直接提升会扩大为 schema 与算法建设，并超出 proposal 非目标。
- 影响：本 change 明确延期，并要求未来 change 先补稳定事实、route schema、ranking/provenance 和验证证据。

## 方案与取舍

| 选项 | 优点 | 缺点 | 结论 |
| --- | --- | --- | --- |
| 仅更新文档以描述现状 | 改动小 | 已证实的排序、错误和协议差异继续存在 | 不采用 |
| 保留所有旧字段为兼容层 | 迁移压力低 | 项目明确无需兼容，双轨合同无法结束 | 不采用 |
| 删除公开派生视图并修复跨层主合同 | authority 唯一、可测试、宿主薄消费 | 需要同步 Rust/TS/tests/docs | 采用 |
| 直接公开全部 richer query | 能力面更大 | 无稳定 schema 和验收证据，超出范围 | 不采用，延期 |
| 暴露 SQLite raw BM25 | 保留底层原值 | 方向和量纲不适合跨 adapter 消费 | 不采用 |
| group 显式 ranking basis/direction，result 使用 rank + optional score | 不伪造统一量纲，可稳定排序和测试 | DTO 改动更明确 | 采用 |

## 风险与未知项

| 项目 | 影响 | 处理方式 |
| --- | --- | --- |
| 内部 rich `QueryReport` 仍保留旧字段 | 可能被误当公开合同 | 只让 transport/model/TS/Agents/Wiki 成为外部 authority；内部字段标注非公开并由后续清理。 |
| 固定结构化分数可能过度表达精度 | confidence 被误读 | score basis 明确区分 normalized FTS、structural exact 和 graph confidence。 |
| 旧 capability 文档仍有冲突 | 全库 authority 不一致 | 本 change 只同步直接拥有的 Runtime/Agents 设计；capability 全量迁移归 documentation-closure，但增加合同测试防止主入口回退。 |
| index not ready 的恢复动作依上下文不同 | 单一动作可能不精确 | 未初始化返回 `init`；facts snapshot/graph 缺失场景由 runtime readiness 决定 `rebuild`，协议携带明确 action。 |

## 对当前 artifact 的影响

- 应写入：`design.md`
- 影响内容：
  - stable request/response contract、fusion/fallback、score/ranking、state matrix 和 error contract。
  - Rust model/index/runtime、TS parser/CLI、Agents assets、Wiki 与测试的具体落点。
  - richer query 延期清单与升级条件。
- 后续阶段处理：
  - plan 阶段把真实 SQLite、transport/parser parity、error taxonomy、degraded provenance 与宿主字段迁移转换为 TDD 用例和 tasks。

## 未采纳内容

- 不使用 upstream；本次没有直接迁移、改写或仅借鉴的外部实现。
- 不在本 change 新增 semantic search、owner 推断、process/community route 或公开 intent payload。
- 不回写历史 archive，也不把历史 capability 冲突全部提前清理。
