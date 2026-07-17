# 宿主触发合同现状审计

## 调研目的

- 阶段：propose
- 关联 change：`close-specwiki-3-0-design-baseline-host-trigger-contract`
- 服务边界：proposal
- 要回答的问题：三个宿主当前如何暴露 Wiki action、哪些触发与 query 消费语义已经统一、哪些缺口必须进入本 change，以及 provider research session 与已有基础 / 未启用 production host-agent bridge 如何保持分层。
- 停止条件：能够形成可独立验收的 problem、goals、non-goals、success criteria、impact scope、risks 和 unknowns，且不提前决定接口或实现方案。

## 结论摘要

- Codex、Claude、CodeBuddy 已共享 action semantics 与 query skill renderer，现有 query 文案已经薄消费 Runtime canonical fields；本 change 应复用这一正确基线，不重新定义 Runtime DTO、ranking、readiness、trust 或 answer。
- 当前没有统一、机器可测的 `should_trigger / should_not_trigger / ambiguous` 对象语言，也没有多语言语料、决策理由或跨宿主 conformance 门禁。
- CodeBuddy 的 `UserPromptSubmit` hook 使用宿主私有中英关键词 substring 判断；Codex 与 Claude 只依赖宿主原生 skill discovery。机制差异是真实能力差异，合同应要求语义决策可比较，而不是伪造相同 hook 能力。
- 当前代码只有宿主 registry 与资产构建 switch，Wiki 中的 `HostAdapter / HostCapabilities` 仍是推荐抽象；需要一份由资产规划与测试共同消费的真实 trigger capability matrix，不能只写静态 Markdown 表。
- host trigger decision、host-agent bridge transport 与 request-local provider research session 是三个不同层次。基础 bridge forwarding 已存在，但 production research bridge 和多轮 agent-session bridge尚未启用；trigger 不得暗中扩展 richer query，bridge 不得被描述为 durable provider resume，provider session 不得持久化。

## 已读取资料

| 路径 / 来源 | 目的 | 关键发现 |
| --- | --- | --- |
| `../close-specwiki-3-0-design-baseline/split.md`、parent `meta.yaml` | 确认 child 目标、验收边界、顺序和依赖 | 本 child 必须覆盖 trigger taxonomy、capability matrix、query 消费模板与 session/bridge 分层；Runtime Query 依赖已归档 |
| `.wiki/06-设计文档/02-Agents设计.md` | 核对当前 Agents authority 与推荐抽象 | HostAdapter/HostCapabilities 已有推荐方向，但当前真实 trigger 机制与文档仍有漂移 |
| `.wiki/06-设计文档/06-Runtime查询合同.md` | 确认宿主可消费的 query authority | 公开输入仍是非空 term；Runtime 独占 route、ranking、readiness、trust、action、answer 与 error policy；richer input 延期 |
| `packages/spec-wiki/src/agents/shared/commandAssets.ts`、`workflowSemantics.ts` | 核对共享宿主资产语义 | 三宿主已共享 action/query 文案；query 已引用 canonical fields 并禁止宿主重建状态机 |
| `packages/spec-wiki/src/agents/shared/hosts.ts`、`hostAssets.ts` | 核对宿主 registry 与资产能力表达 | 当前只有检测信息和资产构建分支，没有可查询、可验证的 trigger capability matrix |
| `packages/spec-wiki/src/agents/codebuddy/assets.ts` 及相关测试 | 核对当前自动触发行为 | CodeBuddy 独有 SessionStart/UserPromptSubmit hook，prompt 判断仍是内嵌关键词 substring；测试主要验证生成字符串和资产路径 |
| `packages/spec-wiki/src/agents/codex/assets.ts`、`claude/assets.ts`、bootstrap tests | 核对 Codex/Claude 真实能力 | 两者生成 repo-local skills，没有与 CodeBuddy 等价的 prompt hook；现有测试验证资产落点和 thin query 文案 |
| `.spec/archive/2026-03-30-iteration-11-10-codebuddy-skill-conformance/`、`2026-03-30-iteration-11-11-codex-claude-host-conformance/` | 复核历史宿主收敛经验 | 可借鉴共享 renderer、skill 形态和 thin boundary；历史四 action 口径与旧 query 字段不能继续作为 authority |
| `.spec/archive/2026-07-15-close-specwiki-3-0-design-baseline-runtime-query-contract/`、`2026-07-16-close-specwiki-3-0-design-baseline-reliability-lifecycle/` | 核对当前 query/session 前置合同 | query canonical fields 已收口；provider session 是 request-local 且 persistence forbidden |

## 子代理调研

| 子代理 / 角色 | 调研主题 | 结论 | 主会话复核 |
| --- | --- | --- | --- |
| `host_trigger_audit` / proposal scope reviewer | 当前宿主资产、trigger 缺口、query authority 与 session/bridge 分层 | 缺三态 taxonomy、共享 corpus、真实 capability matrix 和跨宿主 parity evidence；不能把 CodeBuddy 私有关键词或历史旧字段提升为规范 | accepted；与主会话对 parent、代码、Wiki 和历史 archive 的核对一致 |

## 关键发现

### 共享 query 消费边界已经存在，但 trigger authority 不存在

- 证据：`packages/spec-wiki/src/agents/shared/commandAssets.ts`、`.wiki/06-设计文档/06-Runtime查询合同.md`。
- 说明：query skill 已读取 `readiness / query_mode / query_trust / recommended_action / governance / route_groups / answer`，并要求继续核验 source refs；但“何时建议或选择 wiki-query”仍分散在 description、hook context 和关键词表中。
- 影响：proposal 应保留 canonical query consumption 作为硬边界，并把新工作聚焦于 trigger decision 与 conformance，禁止重写 Runtime query 语义。

### 三宿主的触发机制能力不对称

- 证据：Codex/Claude 只生成 repo-local skills；CodeBuddy 额外生成 SessionStart/UserPromptSubmit hooks 和 settings patch。
- 说明：无法用相同运行机制观测三个宿主的真实模型选 skill 行为，也不应声称能够控制外部宿主模型决策。
- 影响：跨宿主 parity 应定义为共享 taxonomy、target action、reason/evidence 与生成资产语义一致；delivery mechanism 必须由 capability matrix 显式区分。

### CodeBuddy 私有 substring 判断会产生不可审计误触发

- 证据：`renderCodeBuddyHookScript` 对 stdin lowercase 后执行内嵌关键词 `includes`。
- 说明：当前没有 negative、ambiguous、近碰撞、malformed input 或上下文不足语料，也没有共享 classifier authority。
- 影响：proposal 必须要求版本化、表驱动、多语言 corpus，以及 `should_not_trigger`、`ambiguous` 的 fail-closed 行为；具体 evaluator/schema 留到 design。

### trigger、bridge 与 provider session 必须正交

- 证据：Runtime Query authority 延期 richer input；reliability lifecycle 已固定 `provider_session.scope=request_local` 与 persistence forbidden；历史 agent bridge 材料仍存在过时 session 表述。
- 说明：host trigger 只决定建议或选择哪个公开 action；基础 host-agent bridge transport 已存在，但 production research bridge 与 agent-session event path 当前未启用；provider research session 是单次 `research_page` 调用内的 bounded state，内部可包含多次模型/tool turn。
- 影响：proposal 明确三层边界，不扩展 bridge、不启用 production research bridge、不引入 durable session、不把 trigger decision 写入 Runtime request 私有字段。

## 方案与取舍

| 选项 | 优点 | 缺点 | 结论 |
| --- | --- | --- | --- |
| 只补文档 taxonomy 和矩阵 | 范围小 | 无法阻止宿主资产和 hook 再次漂移，也不能证明同一语料决策可比较 | 不采用 |
| 建立共享三态合同、真实 capability matrix、语料与自动化 conformance | 能形成唯一规范源和可执行证据，同时允许机制差异 | design 需要谨慎定义 evaluator、corpus 与 adapter 消费边界 | 采用 |
| 强制三宿主实现相同 prompt hook/classifier | 表面机制一致 | Codex/Claude 当前无等价 hook，会伪造能力并扩大宿主集成范围 | 不采用 |
| 同时实现 richer query 或 host-agent bridge | 可一次覆盖更多意图 | 破坏已归档 Runtime Query/session 边界并使 change 不可独立验收 | 不采用 |

## 风险与未知项

| 项目 | 影响 | 处理方式 |
| --- | --- | --- |
| taxonomy 覆盖全部 action 可能扩大范围 | init/update/sync/rebuild 具有副作用，模糊触发风险高 | proposal 固定 query/status 为主要语义触发面，mutation actions explicit-only；具体 precedence 留到 design |
| ambiguous 的呈现方式尚未统一 | 不同宿主可能只能注入提示、显示 skill 或请求用户补 term | 写入 proposal 成功标准；具体 delivery policy deferred-to-design |
| Codex/Claude 无 prompt hook | 无法把外部模型实际选 skill 作为可重复测试对象 | 将验收限定为共享 decision、资产语义、capability 声明和可观测 hook，不虚构模型控制能力 |
| 多语言与近碰撞语料边界 | 语料过窄会漏判，过宽会膨胀为通用 NLU | proposal 要求中英 baseline 与负例/模糊例；locale 扩展、优先级和多 target 留到 design |
| 推荐 HostAdapter 抽象与当前 switch 实现不一致 | 全量 bootstrap 重构会扩大 change | 只要求可验证的 trigger capabilities 与资产消费合同；是否补齐完整 adapter 接口 deferred-to-design |

## 对当前 artifact 的影响

- 应写入：`proposal.md`
- 影响内容：
  - 问题聚焦于 trigger authority、跨宿主可比性、能力事实与三层边界，而不是宿主资产形态重做。
  - goals 包含三态 taxonomy、真实 capability matrix、版本化 corpus、共享 query consumption 与 conformance evidence。
  - non-goals 明确排除 Runtime DTO/richer query、外部模型控制、自动副作用 action、bridge 扩展/production research bridge、durable session、新宿主和无关全库文档收口。
  - success criteria 要求同语料语义一致、delivery 差异显式、ambiguous/negative fail closed、canonical fields 单一、capability drift 可测试。
- 后续阶段处理：
  - taxonomy schema、reason/evidence 闭集、corpus 格式、precedence、ambiguous delivery policy、capability 类型和 adapter 接口落点 deferred-to-design。

## 未采纳内容

- 未采用历史 `matched_pages / provenance_summary` 或四 action 口径；它们已被当前 Runtime Query authority 与公开 action 集替代。
- 未把 `.wiki/06-设计文档/02-Agents设计.md` 中的推荐 HostAdapter 代码片段直接迁移为实现方案；proposal 只记录需满足的可验证能力边界。
- 未修改历史 archive，也未使用 upstream；不存在外部实现的直接迁移、改写或仅借鉴。
