# 可靠性与生命周期技术设计调研

## 调研目的

- 阶段：design
- 关联 change：close-specwiki-3-0-design-baseline-reliability-lifecycle
- 服务边界：design
- 要回答的问题：如何把跨层 freshness、declared authority、page projection、provider failure 和 KnowledgeUnit resume 收口到现有 Rust 分层，并形成可自动化验证的唯一合同。
- 停止条件：数据模型、决策优先级、workflow 落点、迁移边界和测试方向足以支撑 `design.md`，不提前生成 system tests、tasks 或实现代码。

## 结论摘要

- Runtime 应新增一个纯决策内核 `ReliabilityAssessment`，一次消费 live evidence 并统一投影 state、layer freshness、consumability、readiness、trust ceiling 和 recommended action；`status/query/update preflight` 不再各自二次推导。
- freshness 与 consumability 必须分轴。stale index 可以物理可读并提供受限结果，但不得显示为 direct-ready 或 grounded。
- declared semantic status、authoring presence 和 group authority 必须正交；active/replaced authority block 消失不得物理 prune，全部 deprecated 才能表达合法的零现行 authority。
- page 是否应存在由正式 `PageProjectionDecision` 决定，`ready/stale/conflict/blocked` 只描述技术 readiness；默认策略保留结构页，并对 leaf page 采用显式 include/exclude、稳定排序和 domain 预算。
- production provider 的硬不变量是“无有效 provider output 不得 accepted”。stop reason、provider failure kind 和 workflow decision 应由 exhaustive reducer 分开表达。
- compose resume 固定为 KnowledgeUnit 粒度。完整 research result 和 `PageDraft + PageDigest` pair 是提交点；provider session 保持 request-local，不持久化 turn/tool state。
- 当前 capability 应撤回“generic typed surface decomposition 已完成”的过度承诺，保留 deterministic heuristic planner、稳定 identity、parent/child 和 leaf-first 等已证明能力。
- 本次未使用 upstream；不存在直接迁移、改写或仅借鉴的外部实现。

## 已读取资料

| 路径 / 来源 | 目的 | 关键发现 |
| --- | --- | --- |
| `proposal.md`、`research/reliability-lifecycle-audit.md` | 核对已确认目标与 deferred-to-design 问题 | A1/A6/A8/A9 是实现主线，A4 只消费既有 query authority。 |
| `.wiki/06-设计文档/01-Runtime设计.md`、`04-扩展场景.md`、`06-Runtime查询合同.md` | 核对长期 authority 和 query 非目标 | query DTO 不应重定义；projection/declared 原则已有，但治理闭环不完整。 |
| `.wiki/05-规格基线/capabilities/knowledge-runtime-health-signals/spec.md` | 核对 health contract | health 与 readiness 已声明正交，但 action 合并仍依赖局部 rank。 |
| `.wiki/05-规格基线/capabilities/declared-knowledge-lifecycle/spec.md` | 核对 declared 状态和关系 | active/deprecated/superseded/replaced 已存在，删除、零 head 和历史事件缺失。 |
| `.wiki/05-规格基线/capabilities/knowledge-unit-decomposition/spec.md` | 核对 planner 承诺 | 当前代码不能证明 generic typed surface decomposition 已完成。 |
| `crates/wiki-runtime/src/domain/runtime_profile.rs`、`workflows/status.rs`、`workflows/query.rs` | 核对状态决策链 | state、readiness、health、governance 和 query trust 存在多次合并与局部特判。 |
| `crates/wiki-model/src/domain/knowledge_artifact.rs`、`projection.rs` | 核对 formal types | declared、conflict、health、projection digest 已有基础，可原地扩展。 |
| `crates/wiki-runtime/src/workflows/sync.rs`、`storage/knowledge_artifacts.rs` | 核对 declared commit/restore | block 消失会 prune；conflict 是当前 snapshot 的 open-only 派生视图。 |
| `crates/wiki-knowledge/src/projection.rs`、`crates/wiki-runtime/src/workflows/update.rs` | 核对 page planning/removal | 当前每个 unit 无条件生成 page，removed page 缺少 manual/link protection 与事务计划。 |
| `crates/wiki-runtime/src/workflows/research_provider.rs`、`llm/mod.rs`、`domain/research.rs` | 核对 provider outcome | 多个无 output stop reason 可经 structural seed 继续，production success 语义不闭合。 |
| `crates/wiki-runtime/src/domain/checkpoint.rs`、`workflows/page_render.rs` | 核对 resume | unit cache 可复用，但 action 未进入 research hash，scoped update 关闭 resume。 |
| 相关 Rust tests 与已归档 Runtime Query/core scenario design | 核对现有证据和稳定边界 | 已有窄场景测试可复用，但缺少 reducer table、atomic demotion 和大规模 deterministic interruption fixture。 |

## 子代理调研

| 子代理 / 角色 | 调研主题 | 结论 | 主会话复核 |
| --- | --- | --- | --- |
| reliability runtime auditor | canonical state/action 与 A9 failure matrix | 建议单一 assessment、双轴 layer state、按前置条件短路 action；production 无 output 一律 blocked。 | accepted；保留 query governance 正交合同，不新增公开 resume action。 |
| declared/projection auditor | declared authority 与 page governance | 建议 semantic/authoring/authority 三轴、保留冲突事件、projection decision、manual/link protection 和事务提交。 | accepted；replacement head 可 query，hard purge/redirect 继续非目标。 |
| compose/extended auditor | unit resume、session、decomposition 和大仓 fixture | 建议 `PipelineResumeIdentity`、完整 cache pair 提交点、request-local session 和 32-64 unit 离线 fixture。 | accepted；不新增重型 checkpoint journal。 |

## 关键发现

### 状态冲突来自多次投影而非缺少 enum

- 证据：`runtime_profile.rs` 的 `preflight_for_state/query_trust_for/merge_recommended_action`，以及 `status.rs/query.rs` 的二次推导。
- 说明：同一 stale 或 blocker evidence 会先变成 state，再被 readiness、health、governance 和 route logic 重新解释，导致 index readiness 与 result provenance 可能不一致。
- 影响：设计采用 evidence-first reducer；现有公开 `RuntimeReadiness` 继续作为粗粒度输出，不成为内部 truth source。

### declared replacement graph 需要 group-level authority

- 证据：`DeclaredKnowledgeRecordStatus`、双向 relation 语法、active-only query 和 open-only conflict 派生。
- 说明：多代 `A -> B -> C` 无法仅靠逐 record 校验稳定表达；active/replaced 是可成为 authority 的 head，deprecated 是显式无现行 authority，superseded 是历史节点。
- 影响：设计新增 canonical replacement edge、authority decision 和 append-only governance event，并禁止 block 删除绕过状态迁移。

### page ownership 不能回答 page 是否应存在

- 证据：`plan_pages_from_knowledge_tree` 对所有 unit 直接生成 `PagePlan`；`SectionOwnership` 只约束 section 修改权。
- 说明：technical readiness 和 page eligibility 是两组正交问题。安全 demotion 还依赖 manual/declared protection、managed/manual link 和同一 commit 的 metadata/digest/cache 回收。
- 影响：设计新增 projection policy、decision artifact、link ref 和 runtime commit plan。

### provider `Ok` 不是 success authority

- 证据：`PageResearchSessionResult` 对 `not_run/no_meaningful_delta/turn_budget_exhausted/call_budget_rejected/invalid_output` 返回无 output result；provider wrapper 可继续保留 structural seed。
- 说明：transport、session stop、evidence quality、workflow decision 和 remaining capability 当前混在一起。
- 影响：设计引入 `ResearchOutcomeDecision` reducer；development structural fallback 只能是 diagnostic，不能写 production-ready formal artifact。

### resume identity 目前不完整

- 证据：`prepare_resume_state`、research input hash 和 `run_scoped_compose_pipeline_for_update`。
- 说明：action 改变仍可能命中旧 research cache；scoped update 明确关闭 resume；gate/summary 与 cache 之间缺少单一提交点说明。
- 影响：设计将 action、facts、knowledge tree 和 contract version 纳入 resume key，并用完整 research 与 draft/digest pair 判定已完成 unit。

## 方案与取舍

| 选项 | 优点 | 缺点 | 结论 |
| --- | --- | --- | --- |
| 继续补充 state/action 特判 | 改动局部 | 多套规则继续漂移，无法 exhaustive 验收 | 不采用 |
| 新增统一 reliability reducer，现有 DTO 只做投影 | 单一 authority、可做 table tests、不扩大 query payload | 需要迁移多个现有 helper | 采用 |
| block 删除即 prune declared record | 操作简单 | 丢失 authority 与审计历史 | 删除该行为 |
| 新增 hard purge/redirect 产品能力 | 可处理隐私删除和旧链接 | 超出 proposal，需独立 authority/UX | deferred |
| 按每个 unit 继续生成 page | 无迁移成本 | 与 A8 和有界页面目标直接冲突 | 删除该默认 |
| 引入通用在线 usage ranking | 可动态选页 | 没有稳定遥测 authority，测试不确定 | 不采用 |
| 使用 deterministic policy + 显式 include/exclude + domain budget | 可重放、可配置、默认有界 | 会改变当前 page 集合 | 采用 |
| provider turn/session 持久化 | 恢复更细 | 扩大安全、schema 和 provider 语义 | 非目标 |
| KnowledgeUnit 完整提交点 resume | 复用现有 cache，风险可控 | 当前 unit 仍需重跑 | 采用 |

## 风险与未知项

| 项目 | 影响 | 处理方式 |
| --- | --- | --- |
| projection 收紧改变页面路径和数量 | fixtures、metadata 和内部链接会变化 | 当前无兼容要求；采用 staging commit 和全量 managed-link 校验。 |
| manual 内容阻止 demotion | 页面可能停在 retiring | fail closed，保留页面并给出 `review_governance`，不自动搬运文本。 |
| snapshot identity 当前只含 facts | same-facts sync/demotion 无法区分提交 | 新 committed snapshot identity 纳入 declared/projection/page hashes。 |
| provider stop 收紧暴露更多 blocker | 现有 production 流程通过率下降 | 这是 proposal 要求；development fixture 使用显式 diagnostic policy。 |
| action 纳入 research hash降低跨 action cache 命中 | 增加重复计算 | 可靠性隔离优先；未来 content-addressed verified cache 另立 change。 |
| runtime commit journal 扩大实现面 | update/sync/rebuild 都需迁移 | 只实现 repo-local lock/staging/rollback/roll-forward，不引入服务或分布式事务。 |

## 对当前 artifact 的影响

- 应写入：`design.md`
- 影响内容：
  - `ReliabilityEvidence -> ReliabilityAssessment -> public projection` 单一决策链。
  - declared authority、authoring presence、conflict history 和无 hard purge 的状态机。
  - projection eligibility/lifecycle/action、policy、保护规则与事务提交。
  - A9 exhaustive matrix、production evidence gate 与 diagnostic fallback。
  - `PipelineResumeIdentity`、unit 提交点、request-local session 与 deterministic interruption fixture。
  - A1 至 A10 分类和当前术语承诺。
- 后续阶段处理：system tests、unit tests 和逐文件任务由 `unispec-plan` 生成。

## 未采纳内容

- 不采用 upstream；本次调研没有外部迁移、改写或借鉴来源。
- 不引入 provider turn 级 checkpoint、跨 workflow session、真实网络 required gate 或任意大仓 wall-clock SLA。
- 不在本 child 完成全 planner 重写、全库 capability Purpose/命名迁移、hard purge、redirect 或在线 usage ranking。
