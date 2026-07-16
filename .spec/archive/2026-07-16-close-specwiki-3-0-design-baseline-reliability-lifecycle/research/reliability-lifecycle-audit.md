# 可靠性与生命周期边界调研

## 调研目的

- 阶段：propose
- 关联 change：close-specwiki-3-0-design-baseline-reliability-lifecycle
- 服务边界：proposal
- 要回答的问题：当前 Runtime stale/降级、declared lifecycle、page projection、大仓 compose、provider session 和扩展场景分别已实现到哪里，哪些缺口应进入本 change。
- 停止条件：能够形成可独立验收的 problem、goals、non-goals、success criteria、impact、risks 和 unknowns，不进入架构或实现设计。

## 结论摘要

- 当前已有 minimal reliability substrate，但 freshness、readiness、health、query trust、recommended action 与失败恢复尚未形成覆盖所有 truth layer 的唯一生命周期合同。
- A1、A4、A6、A8、A9 可进入 3.0 baseline；A2、A3、A5、A10 属于 next；A7 权限/可见性不属于当前 repo-local baseline。
- declared lifecycle、projection binding、两级 restore 和 KnowledgeUnit 粒度 checkpoint 已有实现，本 change 不应重复发明这些对象，而应闭合语义矛盾与端到端证据。
- 当前大仓 compose 只能承诺已完成 KnowledgeUnit 的缓存/门禁可复用，不能承诺 provider turn 级断点恢复或任意大仓稳定完成。
- `PageResearchSessionState` 当前只服务单次 page research 调用内的有界多轮状态，不是跨 workflow、跨进程或跨宿主持久 session。
- KnowledgeUnit decomposition 是确定性 profile/信号/父子关系 planner，但仍含启发式关键词和一 unit 一 projection 的强耦合，不能由历史 capability 名称推导为通用分解已完成。
- 本次未使用 upstream；不存在直接迁移、改写或仅借鉴的外部实现。

## 已读取资料

| 路径 / 来源 | 目的 | 关键发现 |
| --- | --- | --- |
| `.spec/changes/close-specwiki-3-0-design-baseline/split.md`、`meta.yaml` | 核对 child 边界、顺序和依赖 | 本 child 固定为 order 4，依赖 product-contract 与 runtime-query-contract，范围包含 reliability lifecycle 与扩展场景分类。 |
| `.spec/changes/close-specwiki-3-0-design-baseline/research/design-debt-and-delivery-boundary.md` | 核对 parent 调研 | 大仓 compose 暴露真实实现缺口；扩展场景必须强制 baseline/next/non-goal 三分。 |
| `.wiki/06-设计文档/01-Runtime设计.md` | 核对长期 Runtime authority | 已定义 truth layers、restore、projection、workflow 与 declared lifecycle 原则，但部分叙述超出当前可验证行为。 |
| `.wiki/06-设计文档/04-扩展场景.md` | 核对 A1..A10 | 仍是未分级草案，没有 dependency、upgrade condition 或可验收边界。 |
| `.wiki/06-设计文档/06-Runtime查询合同.md` | 核对 A4 与 query 边界 | A4 已有 canonical query authority；richer impact/owner/entrypoint 继续延期。 |
| `.wiki/05-规格基线/capabilities/knowledge-runtime-health-signals/spec.md` | 核对 health contract | 已定义 orphan、missing provenance、derived/projection stale、declared divergence 与 status summary。 |
| `.wiki/05-规格基线/capabilities/declared-knowledge-lifecycle/spec.md` | 核对 A6 | declared 状态、关系、scope 与失效传播已有正式基础。 |
| `.wiki/05-规格基线/capabilities/knowledge-unit-decomposition/spec.md` | 核对 decomposition | 稳定 profile/identity/parent-child 有要求，但不能证明历史 typed surface decomposition 已完整实现。 |
| `.spec/archive/2026-04-14-iteration-12-8-stabilize-knowledge-runtime-contract/**` | 核对历史 reliability 合同 | 已建立 minimal knowledge contract 和 health/state 原语，不等于完整可靠性闭环。 |
| `.spec/archive/2026-06-16-formalize-declared-authoring-contract/**` | 核对 declared lifecycle | active/deprecated/superseded/replaced 与 relation 校验已进入正式模型。 |
| `.spec/archive/2026-06-17-refactor-specwiki-around-contract-closure-projection-writeback-boundaries/**` | 核对 projection/writeback ownership | section ownership、marker、binding、drift 与 restore 边界已经通过 full review。 |
| `.spec/archive/2026-07-15-close-specwiki-3-0-design-baseline-runtime-query-contract/**` | 核对 canonical query | 本 child 必须消费，不得重新定义 query DTO、ranking、trust 或 action 语义。 |
| `.spec/archive/2026-07-16-close-specwiki-3-0-design-baseline-core-scenario-acceptance/**` | 核对核心场景证据 | CS-05/CS-09 证明窄闭环，但 proposal 明确把完整 reliability 留给本 child。 |
| `crates/wiki-model/src/domain/knowledge_artifact.rs`、`projection.rs`、`knowledge.rs` | 核对正式对象 | health、declared lifecycle、projection status/ownership、KnowledgeUnit identity 已存在。 |
| `crates/wiki-knowledge/src/planning.rs`、`projection.rs` | 核对 decomposition/page planning | planner 确定性但仍有关键词启发式；当前 projection 基本按每个 KnowledgeUnit 生成页面。 |
| `crates/wiki-runtime/src/workflows/status.rs`、`query.rs`、`sync.rs`、`update.rs` | 核对状态与传播 | status/readiness/health/query 的局部规则存在，但跨层 freshness 和 action 仍有不一致风险。 |
| `crates/wiki-runtime/src/workflows/page_render.rs`、`research_provider.rs`、`llm/mod.rs` | 核对 compose/session | checkpoint/cache 可按 unit 续跑；provider session 不持久化；部分 stop reason 可能以 structural output 继续。 |
| `crates/wiki-runtime/tests/runtime/**`、`tests/llm_runtime.rs` | 核对自动化证据 | stale/update、restore、declared、projection、provider blocker 和 compose resume 有分散测试，缺少统一 lifecycle matrix。 |

## 子代理调研

| 子代理 / 角色 | 调研主题 | 结论 | 主会话复核 |
| --- | --- | --- | --- |
| reliability runtime auditor | stale/readiness/health/recovery | 已有状态原语，但 A1/A9 跨层语义与 init/rebuild resume 边界不完整。 | accepted；纳入 problem、goals 和 risks。 |
| declared/projection auditor | A6 declared 与 A8 page governance | declared 数据合同较完整；删除/head/conflict 历史仍有缺口；page eligibility/retention/demotion 基本缺失。 | accepted；A6/A8 分别设可验收目标。 |
| compose/extended auditor | compose、provider session、decomposition、A1..A10 | unit 级 resume 成立，turn/session 级不成立；扩展场景建议 5 baseline、4 next、1 non-goal。 | accepted；用于 success criteria 与明确非目标。 |

## 关键发现

### Runtime 状态原语存在，但 freshness authority 未完全一致

- 证据：`crates/wiki-runtime/src/workflows/status.rs`、`query.rs`、`domain/runtime_profile.rs`。
- 说明：外部 state、layer readiness、health summary、query trust 和 action 分别存在；源码变更可使 state/query stale，但某些 layer readiness 仍可能因本地 row 存在而显示 ready。
- 影响：proposal 要求 A1 固定每层 freshness authority、可消费条件和恢复终态，design 再决定状态优先级。

### A9 的失败分类仍依赖局部 Err/Ok 路径

- 证据：`crates/wiki-runtime/src/workflows/research_provider.rs`、`llm/mod.rs`、`tests/runtime/status_and_update.rs`。
- 说明：provider transport error 在 production 可成为 blocker，但 turn budget、call budget、invalid output、no meaningful delta 等 stop result 可能继续输出 structural seed；其它 parser/graph/storage/assemble 失败也没有统一矩阵。
- 影响：proposal 要求 failure class、remaining capability、decision、action 和 evidence 一致，禁止静默成功。

### compose resume 是 KnowledgeUnit 粒度，不是 durable provider session

- 证据：`crates/wiki-runtime/src/workflows/page_render.rs`、`domain/checkpoint.rs`、`domain/context.rs`。
- 说明：相同 facts/action 可复用 research cache、draft、digest 和 unit gate；当前 unit 的 provider 多轮调用中断后仍需整 unit 重跑，session state 未跨 workflow 持久化。
- 影响：proposal 只承诺有界 unit 级恢复，并要求纠正历史命名高估。

### declared lifecycle 的删除、authority head 与治理历史仍不闭合

- 证据：`crates/wiki-model/src/domain/knowledge_artifact.rs`、`crates/wiki-runtime/src/workflows/sync.rs`、`tests/runtime/editable_runtime.rs`。
- 说明：关系合法性、无环、active-only query 和 stale propagation 已成立，但删除 authoring block 会物理 prune；零 authoritative head、冲突 resolved history 与删除/废弃边界没有唯一合同。
- 影响：proposal 要求 A6 的迁移、删除、head、冲突和跨 workflow 行为可验证。

### section ownership 不等于 page projection governance

- 证据：`crates/wiki-knowledge/src/projection.rs`、`.wiki/06-设计文档/01-Runtime设计.md`。
- 说明：现有 ownership/marker/binding 解决谁能写和如何恢复，未回答页面为何存在、何时保留/降级/回收；每个 unit 默认产生 page 与“少量稳定页面”原则冲突。
- 影响：proposal 要求 A8 建立与 readiness 正交的 projection decision 和有界页面策略。

### decomposition 与 provider session 的历史名称高于实际能力

- 证据：`crates/wiki-knowledge/src/planning.rs`、`crates/wiki-runtime/src/workflows/research_provider.rs`、历史 9.8 design。
- 说明：decomposition 是确定性启发式 planner；provider session 是 request-local bounded loop；真实大仓 full compose 仍只有 diagnostic evidence。
- 影响：proposal 要求明确当前可信部分、未证明部分和升级条件，不把名称当成完成证据。

## 方案与取舍

| 选项 | 优点 | 缺点 | 结论 |
| --- | --- | --- | --- |
| 只整理扩展场景文档 | 改动小 | 无法消解 Runtime 状态、provider stop reason、declared 删除和 page 增殖的行为矛盾 | 不采用 |
| 把完整 knowledge system、大仓性能、session 和 planner 全部补完 | 表面上范围完整 | 不可独立验收，超出 parent child 边界，并会吞并 host/documentation 后续 change | 不采用 |
| 收口 reliability lifecycle 合同并修正必要主链缺口 | 能复用已有 substrate，以 TDD 证明 A1/A6/A8/A9，同时诚实分类未来场景 | design 需要处理多组正交状态和迁移风险 | 采用 |

## 风险与未知项

| 项目 | 影响 | 处理方式 |
| --- | --- | --- |
| freshness/readiness/trust 多状态可能互相覆盖 | 宿主可能直接信任旧层 | 写入 proposal 成功标准；具体优先级 deferred-to-design。 |
| declared block 删除与可审计生命周期冲突 | 可能丢失长期治理历史 | 写入 proposal 目标；删除/head 语义 deferred-to-design。 |
| page eligibility 收紧改变页面集合和链接 | 可能影响 metadata、digest、fixture 与用户手写内容 | 写入 proposal 风险；promotion/demotion/manual/link 策略 deferred-to-design。 |
| provider stop reason 当前决策不统一 | production 可能静默接受无有效 provider output | 写入 proposal 成功标准；分类矩阵 deferred-to-design。 |
| 大仓/网络测试成本高且不确定 | 不适合作为普通离线门禁 | 使用确定性中断/规模 fixture 验证合同；真实样本只作为明确计划的额外 evidence。 |
| 历史 capability 名称高估能力 | documentation closure 可能继续传播错误结论 | 本 child 校准直接相关 authority；全库 Purpose/命名迁移留给 documentation-closure。 |

## 对当前 artifact 的影响

- 应写入：`proposal.md`
- 影响内容：
  - A1/A4/A6/A8/A9 为 baseline，A2/A3/A5/A10 为 next，A7 为 non-goal。
  - 目标覆盖跨层 freshness、declared lifecycle、projection governance、failure degradation 和 unit 级 compose resume。
  - 明确 provider session、decomposition 和大仓 full compose 的真实边界。
  - 非目标排除 PR/onboarding/ACL/incident/richer impact/turn-level resume 和全 planner 重写。
- 后续阶段处理：
  - 状态/动作优先级、projection decision、declared 删除/head、A9 failure matrix、compose fixture 与迁移策略 deferred-to-design。

## 未采纳内容

- 不采用或迁移 upstream；本次调研未读取 upstream 作为事实来源。
- 不把历史 archive 中“typed surface decomposition”“provider session”或“大仓 compose”名称直接提升为当前能力。
- 不把真实外部大仓与网络 provider 设为普通离线单元门禁。
- 不提前执行 documentation-closure 对全库 capability Purpose、命名、roadmap 和索引的迁移。
