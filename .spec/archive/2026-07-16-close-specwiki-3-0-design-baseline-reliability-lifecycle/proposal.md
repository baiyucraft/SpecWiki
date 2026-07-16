# close-specwiki-3-0-design-baseline-reliability-lifecycle

## 问题

### 背景

Repo Wiki 已经建立 minimal formal knowledge runtime、canonical query、declared lifecycle、projection/writeback ownership、两级 restore、health signals 和 compose checkpoint 等基础。前序 core-scenario change 也证明了 CS-05 的增量更新和 CS-09 的 level1 协作恢复。但这些证据分别针对局部对象或窄场景，尚不能回答整个 Runtime 在旧数据、部分失败、生命周期变化、页面治理和长流程中断时是否遵循一套唯一可靠性合同。

当前外部 state、分层 readiness、knowledge health、query trust、recommended action 和 pipeline gate 分散存在；相同 freshness 或 failure 事实可能在不同 workflow 中得到不同解释。扩展场景 A1 至 A10 仍是一组未分级草案，既把已经具备 substrate 的能力写成未来方向，也把没有正式模型的 PR ingestion、权限和故障复盘与当前 baseline 混在一起。

### 真实问题

本 change 要解决的真实问题是：在不重做完整 knowledge system 的前提下，为 A1 stale、A6 长期知识生命周期、A8 稳定页面投影治理和 A9 工具链失败降级建立可执行、跨 workflow 一致的可靠性闭环；同时诚实限定大仓 compose、KnowledgeUnit decomposition 和 provider session 的实际边界，并把 A1 至 A10 分为 baseline、next 和 non-goal，避免用历史名称、局部 helper 测试或 diagnostic 样本冒充完整能力。

## 目标

- 为 facts/index、declared knowledge、derived research、page projection、metadata/runtime mirror 和 local cache 固定 freshness authority、可消费条件、公开状态、trust、recommended action 与恢复终态。
- 闭合 A1 的 source/branch drift 生命周期，保证旧数据不会被 direct-trust，局部 update、full rebuild 与 formal restore 的状态变化可追溯、可重放、fail closed。
- 闭合 A6 的 declared lifecycle，明确 active、deprecated、superseded、replaced、零或多个 authoritative head、删除/失联、冲突、传播、query、status、update 与 restore 的一致行为。
- 建立 A8 的 page projection governance，使页面存在、保留、刷新、降级和回收均有正式 authority 与 reason，并与 ready/stale/blocked 技术 readiness 正交。
- 建立 A9 的 failure/degradation 分类，使 parser/index/graph/formal artifact/provider/research/compose/assemble 等失败的 remaining capability、decision、evidence 与恢复动作一致，不把 structural fallback 或 diagnostic 伪装成 production 成功。
- 固定大仓 compose 当前可承诺的恢复粒度：相同 workflow/facts 下可复用已完成 KnowledgeUnit 的 research/compose evidence，输入变化后旧 checkpoint/cache 必须失效；当前 unit 的 provider 调用可重跑，但不承诺 turn 级续跑。
- 校准 KnowledgeUnit decomposition 与 provider session 的术语边界：decomposition 是确定性信号/启发式 planner，provider session 是单次 page research 请求内的有界多轮状态；未证明的通用分解、跨 workflow session 和任意大仓 full compose 不得被写成 baseline。
- 将 A1 至 A10 逐项分类并记录已有 substrate、缺口、依赖和升级条件：A1/A4/A6/A8/A9 为 baseline，A2/A3/A5/A10 为 next，A7 为当前 non-goal。

## 非目标

- 不实现 PR/review connector、自动采纳审批或 merge decision ingestion；A2 保持 next。
- 不设计 onboarding 产品流程、角色化“先看/禁区/安全进入”体验；A3 保持 next。
- 不把多 package owner/entrypoint、完整 callers/callees/impact traversal 纳入当前 query；A5 保持 next，并服从已归档 Runtime Query 延期边界。
- 不实现用户身份、ACL、加密、私有笔记或远端 visibility authority；A7 不属于当前 repo-local baseline。
- 不引入 incident、timeline、recurrence、runbook 或故障模式独立模型；A10 保持 next。
- 不重定义 canonical query DTO、route-local ranking、query trust 或宿主消费合同，不扩大 host trigger 范围。
- 不开放自由文本自动写入 declared truth，不替用户做规则语义裁决，也不承诺自然语言规则与代码的完整冲突检测。
- 不重写整个 decomposition planner、research/compose orchestration 或 provider runtime，不承诺 unit 并行、provider turn 级 checkpoint、跨机器 working cache/session 恢复或任意规模仓库的 wall-clock SLA。
- 不把 development/test structural fallback 当作 production 正式成功，不把真实外部 reference 项目或网络 provider 设为普通离线单元门禁。
- 不执行 documentation-closure 负责的全库 capability Purpose、命名、roadmap、索引和历史 authority 迁移，不修改历史 archive。
- 不采用或迁移 upstream 实现。

## 成功标准

- 存在唯一可靠性生命周期 authority，能对 facts/index、declared、derived、projection、metadata/mirror 和 cache 逐层说明 freshness source、可消费条件、状态、trust、recommended action、恢复入口和 fail-closed 条件。
- 自动化证据覆盖 A1 的 fresh -> needs_update/stale -> update/rebuild -> fresh 闭环，以及 branch/source/formal artifact drift；任何旧 snapshot、旧 checkpoint 或 stale route 都不会被报告为 direct ready/trusted。
- A6 的 lifecycle 状态、关系、合法迁移、删除/失联语义和 authoritative head 判定具有唯一合同；active、非 active、零 head、并行 head、关系错误和环均有确定的 status/query/sync/update/restore 行为。
- declared 变化在没有源码 dirty set 时也能确定性传播到相关 derived/projection/health scope；非法 page drift 不得覆盖 formal truth，冲突与解决条件必须保留可追溯 evidence 和单值 action。
- A8 的每个正式 page 都能追溯到 projection decision；结构性必保留页、knowledge-only unit、可长期投影页和不再允许存在的页具有可验证边界，默认策略不会随 KnowledgeUnit 数量无条件一对一增殖页面。
- projection promotion/retain/refresh/demotion/removal 与 technical readiness 正交；update/rebuild/sync/restore 对同一 decision 一致，并能安全回收 page、binding、digest、metadata/cache，同时不越权丢弃受保护的 manual/declared 内容。
- A9 的 failure matrix 至少覆盖 index unavailable、formal artifact drift、provider unavailable/transport error、timeout、turn/call budget、invalid output、no meaningful delta、runtime incomplete、compose/assemble interruption和 illegal page drift；每类都明确 blocker/degraded/diagnostic/accepted、remaining capability、evidence 与 action。
- production 模式不会仅因 Rust 路径返回 `Ok` 或存在 structural seed 就把无有效 provider evidence 的结果标成正式成功；status、runtime gate、query answer 与进程退出对同一 failure 结论一致。
- 大仓 compose 的自动化中断 fixture 证明：相同 action/facts 可复用已完成 KnowledgeUnit，恢复后计数不重复；facts/action 改变时旧 checkpoint/cache 被拒绝；完整公开 workflow 完成后不残留伪 interrupted/checkpoint 状态。
- provider session 和 decomposition 的公开说明与当前事实一致：session 不被描述为跨 workflow durable state，decomposition 不被描述为已完全摆脱启发式或已证明任意大仓 full compose。
- A1 至 A10 均具有唯一 `baseline/next/non-goal` 分类、当前 substrate、未覆盖边界、依赖和升级条件；A4 直接引用 canonical Runtime Query authority，不重新定义。
- proposal、design、system tests、实现、full review、verification 与相关 Wiki/capability authority 使用同一 lifecycle/scenario identity；测试必须证明行为，不以文档存在、历史 archive 名称或 diagnostic 报告代替通过证据。

## 影响范围

### 涉及角色 / 利益相关者

- 依赖 `status/query/update/sync/rebuild` 判断可用性和下一步动作的仓库用户与 Agent。
- 维护 KnowledgeUnit、declared/derived/projection、provider research、compose 与 Runtime workflow 的开发者。
- 负责 3.0 baseline、后续 host trigger 和 documentation closure 的维护者。

### 业务场景

- 用户切分支、拉取大改动或本地源码变化后判断旧 runtime 是否仍可消费。
- 规则被废弃、替代、删除或发生并行 authority 时进行同步、查询和恢复。
- KnowledgeUnit 变化后决定正式页面是否需要保留、刷新、降级或回收。
- parser/index/provider/research/compose/assemble 任一阶段失败后判断 remaining capability 和恢复动作。
- 长流程中断后复用已完成 KnowledgeUnit，并拒绝与新 facts/action 不一致的旧工作态。

### 功能范围

#### 包含内容

- Runtime reliability lifecycle、freshness/readiness/health/action 的跨层一致性。
- declared lifecycle authority、projection governance、failure/degradation matrix 和 unit 级 compose resume 验收。
- A1 至 A10 的稳定分类、依赖和升级条件。
- 与当前 authority 直接冲突的 Runtime/扩展场景/capability 文档同步，以及必要的 Rust/workspace 自动化。

#### 不包含内容

- PR/onboarding/ACL/incident/richer impact/host trigger 的新产品能力。
- 全 planner、LLM runtime 或完整 knowledge system 重构。
- documentation-closure 的全库文档迁移与历史清理。

### 业务规则

- formal artifacts 和 committed snapshot 是共享 truth/recovery authority；`.cache`、checkpoint 和 provider session 是本地工作态，不得提升为 truth。
- page 是 projection/authoring surface，不是 derived knowledge truth；只有合法 declared-managed 输入可以回写 declared artifact。
- readiness、health、governance、query trust 与 workflow progress 是正交维度，不能用一个模糊状态覆盖全部事实。
- 任何降级都必须保留原因、remaining capability、evidence 和单值 recommended action；不可判断时 fail closed。
- 当前阶段不要求旧版本兼容；与确认合同冲突的旧字段、旧 fallback 和弱约定可以删除。

### 数据需求

- 验收需要消费 source/facts fingerprint、formal snapshot identity、declared relation/head、research/projection status、health signal、runtime gate/summary、checkpoint/cache identity、query readiness/trust/action 和 page binding evidence。
- 本 proposal 不规定具体 schema、存储路径变更、状态枚举或迁移步骤；这些 deferred 到 design。

### 非功能期望

- 状态和恢复判断必须确定性、可重放、fail closed，并可追溯到稳定 evidence。
- 离线自动化必须覆盖关键失败和中断路径；真实网络/大仓 evidence 只能作为显式计划的补充验证。
- lifecycle 与 projection policy 必须对相同输入产生稳定 decision，不依赖仓库名、reference 标题或不可复现的在线遥测。

## 交付形态

single-change

这是 parent `close-specwiki-3-0-design-baseline` 下顺序第 4 个 child，依赖已归档的 product-contract 和 runtime-query-contract，并消费已归档 core-scenario-acceptance 的场景/gate authority。它统一交付 reliability lifecycle 与扩展场景分级，不改变 parent 的 child 拆分。

## 风险

- freshness、readiness、health、query trust、governance 和 progress 多个正交状态若被错误压成单状态，可能让旧结果被信任或让可用层被不必要阻断。
- 收紧 declared 删除/head 语义会改变“删除 block 即物理删除 record”的现有行为和测试，需要避免误删可审计历史。
- 引入 page eligibility/retention 会改变页面数量、路径、metadata、digest、reference fixture 和外部链接；manual 内容的保留边界必须先明确。
- provider stop reason 当前可能以 `Ok` 但无有效 output 继续，收紧 production decision 后会暴露新的 blocker，这是预期行为变化而非兼容回归。
- 大仓中断测试如果依赖网络、真实 provider 或单纯延长 timeout，会产生慢且不稳定的门禁；必须区分确定性合同 evidence 与真实样本 evidence。
- 历史 capability 名称和 archive 可能高估 decomposition/session/compose 完成度；只改措辞而不修直接行为矛盾会留下新的双轨 authority。
- 本 child 同时触及 model、knowledge、runtime 和 Wiki，但不能顺手吞并 host trigger 或 documentation closure。

## 未知项

- source stale 时 `readiness.index` 应表达 stale 还是“结构存在但 result provenance stale”，以及 layer readiness、query trust 和 action 的最终优先级，留待 design 确定。
- declared block 消失应表示允许物理删除、失联、deprecated 还是非法迁移；零 authoritative head 是否属于合法无现行规则，留待 design 确定。
- conflict resolved history、resolution rationale 和 evidence 是否进入正式 artifact，及 conflict action 使用 `review` 还是 `review_governance`，留待 design 确定。
- page eligibility、结构性必保留集合、promotion/retention/demotion、manual 内容与外部链接处理方式，留待 design 确定；不默认引入在线使用频率。
- A9 各 stop/failure reason 的最终 blocker/degraded/diagnostic/accepted 映射与进程退出策略，留待 design 确定。
- compose resume 是否需要增强 init/rebuild/update 主路径、checkpoint journal 或只校准现有 unit 粒度合同，留待 design 基于 TDD 可行性决定。
- decomposition capability 中历史 typed surface 要求是需要恢复的实现缺口还是应正式撤回的过度承诺，留待 design 与 documentation-closure 边界共同判断。

## 参考资料

- [可靠性与生命周期边界调研](./research/reliability-lifecycle-audit.md)
- [parent 拆分方案](../close-specwiki-3-0-design-baseline/split.md)
- [产品基线与设计治理](../../archive/2026-07-15-close-specwiki-3-0-design-baseline-product-contract/design.md)
- [Runtime Query 合同设计](../../archive/2026-07-15-close-specwiki-3-0-design-baseline-runtime-query-contract/design.md)
- [核心场景验收设计](../../archive/2026-07-16-close-specwiki-3-0-design-baseline-core-scenario-acceptance/design.md)
- `.wiki/06-设计文档/01-Runtime设计.md`
- `.wiki/06-设计文档/04-扩展场景.md`
- `.wiki/06-设计文档/06-Runtime查询合同.md`
- `.wiki/05-规格基线/capabilities/knowledge-runtime-health-signals/spec.md`
- `.wiki/05-规格基线/capabilities/declared-knowledge-lifecycle/spec.md`
- `.wiki/05-规格基线/capabilities/knowledge-unit-decomposition/spec.md`
- 本 change 未使用 upstream；不存在直接迁移、改写或仅借鉴的外部实现。
