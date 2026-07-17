# close-specwiki-3-0-design-baseline-host-trigger-contract

## 问题

Codex、Claude 和 CodeBuddy 已能通过统一 `spec-wiki` action skills 薄消费 Runtime，但“何时应触发 Wiki action”仍没有唯一、可执行的合同。当前 trigger 线索分散在 skill description、共享文案、CodeBuddy hook context 和宿主私有关键词数组中；只有 CodeBuddy 暴露可观测的 prompt hook，Codex 与 Claude 依赖宿主原生 skill discovery。项目因此无法用同一套语料比较三个宿主的语义决策，也无法区分“机制不支持”与“合同不一致”。

现有 query skill 已正确消费 Runtime canonical fields，但缺少 `should_trigger / should_not_trigger / ambiguous` 规范源、真实 trigger capability matrix、多语言正反例与模糊例、决策理由和 drift gate。CodeBuddy 的 substring 判断还可能被关键词近碰撞、malformed input 或上下文不足误触发；现有测试主要验证资产路径和生成字符串，不能证明跨宿主 trigger 语义一致。

同时，host trigger、已有基础 host-agent bridge transport、尚未启用的 production research bridge 与 provider research session 仍容易被混为一谈。Runtime Query 已明确公开输入仍是非空 term，richer intent 延期；reliability lifecycle 已明确 provider session 仅在单次 `research_page` 调用内有效且禁止持久化。本 change 必须在不扩展 Runtime 合同的前提下，建立可比较、可验证、诚实表达宿主能力差异的触发合同。

## 目标

- 将 Codex 定义为唯一 `reference host` 和首要兼容目标，以 repo-local `.codex/skills/wiki-*/SKILL.md`、native skill discovery 与共享 trigger guidance 作为其它宿主投影的参考合同。
- 将 Claude、CodeBuddy 定义为 `compatible host`；两者必须消费共享对象语言和语义合同，CodeBuddy hooks/settings 只作为次级适配能力，不得反向塑造公共架构。
- 建立唯一共享的宿主触发 taxonomy，至少表达 `should_trigger / should_not_trigger / ambiguous`、目标公开 action，以及可审计的 reason/evidence。
- 建立 Codex、Claude、CodeBuddy 的真实 compatibility role 与 trigger capability matrix，区分 reference priority、native skill discovery、prompt hook、session-start context、settings integration 和 deterministic classification 等能力，不把资产类型能力、兼容优先级与 trigger 能力混为一谈。
- 建立版本化、表驱动的跨宿主语料合同，覆盖中英文显式请求、隐式 repo map 需求、明确无需 Wiki、关键词近碰撞、上下文不足、malformed input 和 ambiguous 请求。
- 让三个宿主从同一 taxonomy 与 query consumption template 派生资产语义；允许 delivery mechanism 不同，但同一语料的 decision、target action 与 reason/evidence 可比较。
- 保持 Runtime Query 为唯一 query 业务 authority；宿主只薄消费 canonical DTO、typed error、readiness/trust/action 和 answer，不重建 route、ranking、状态机或结论。
- 明确 host trigger decision、已有 bridge transport / 尚未启用的 production research bridge 与 request-local provider research session 的正交边界。
- 建立自动化 conformance 与 drift gate，使新增宿主或修改宿主资产时必须声明真实能力并通过共享语料。

## 非目标

- 不修改 Runtime query request/response DTO、route/ref 闭集、ranking、readiness、trust、recommended action、answer 或 error policy。
- 不实现 intent/focus/scope/traversal、owner、entrypoint、impact、process/community 等 richer query，也不通过 prompt 或私有 payload 模拟它们。
- 不承诺控制 Codex、Claude 或 CodeBuddy 外部模型的实际自动选 skill 行为；验收对象是共享 decision、生成资产、能力声明与可观测 hook 行为。
- 不让 `ambiguous` 或 `should_not_trigger` 自动调用 CLI；`init / update / sync / rebuild` 等可能改变状态的 action 只允许显式触发。
- 不扩展现有 bridge transport，不启用 production research bridge 或多轮 agent-session bridge，不实现跨 workflow session resume、持久化 session history/recent turns/tool refs，也不重构 provider runtime。
- 不新增第四个宿主，不重构整个 bootstrap/orchestration，不清理无关 roadmap、历史术语或历史 archive；本 change 会同步所有直接依赖宿主与三层边界的当前有效 Wiki。
- 不修改历史 archive，不把历史宿主 conformance 中已删除的 query 字段重新提升为当前合同。

## 成功标准

- 存在单一 trigger taxonomy 与结构化 decision 对象语言；三个宿主的 adapter/renderer/hook 和测试不再维护相互独立的分类副本。
- 存在由实现、资产规划和测试共同消费的 HostAdapter trigger capability matrix；矩阵与 Codex、Claude、CodeBuddy 实际生成资产和可观测机制一致。
- 同一版本化语料在三个宿主上得到相同的语义 decision、target action 与 reason/evidence；不支持 prompt hook 的宿主通过 capability 显式表达 delivery 差异，而不是伪装机制 parity。
- 共享语料至少覆盖中文和英文的 should-trigger、should-not-trigger、ambiguous、关键词近碰撞、上下文不足和 malformed input，并具有确定性、表驱动断言。
- `should_not_trigger` 与 `ambiguous` 不执行 CLI；任何可能改变仓库或 runtime 状态的 action 只在用户明确请求时触发。
- query 宿主资产只引用 Runtime Query authority 的 canonical fields，禁止旧 `matched_pages / provenance_summary`、跨 route score 比较、宿主派生 answer 或 richer 私参。
- Runtime 返回 empty、stale、blocked、typed error 或 degraded fallback 时，宿主只呈现 Runtime conclusion 与 recommended action，不把失败或降级改写为可信成功。
- CodeBuddy prompt hook 不再拥有独立、无语料约束的关键词 authority；结构化事件解析、中英文本、大小写、标点、近碰撞和 malformed input 都有 fail-closed 自动化证据。SessionStart context 与 prompt decision 分开验收。
- host trigger、bridge transport 与 request-local provider session 在代码对象语言、文档和测试中不复用同一 identity 或 persistence 语义；production research bridge 当前保持 blocked，provider session 相关 durable state 仍为 forbidden。
- 新增宿主或改变宿主触发资产时，必须声明 capabilities 并运行同一 corpus conformance；自动化测试能阻止 capability、trigger taxonomy、生成资产与 Runtime query fields 再次漂移。

## 影响范围

### 涉及角色 / 系统

- 使用 Codex、Claude 或 CodeBuddy 理解代码仓库、查询 Wiki 状态或执行 Repo Wiki workflow 的用户与 Agent。
- `packages/spec-wiki` 的宿主 registry、共享 action assets、单宿主资产与 bootstrap orchestration。
- Runtime Query authority、Agents 设计和与三宿主接入直接相关的 capability 文档。

### 包含内容

- 共享 trigger taxonomy、decision/reason/evidence 业务边界与 action policy。
- 三宿主 trigger capability matrix 及其与实际 bootstrap assets 的一致性。
- query/status 语义触发、mutation action explicit-only 边界和 ambiguous fail-closed 行为。
- 跨宿主版本化 corpus、生成资产 conformance、可观测 hook 行为和 drift tests。
- 共享 query consumption template 对 canonical DTO 与进一步源码核验责任的约束。
- host trigger、已有基础 bridge transport / 未启用 production research bridge 与 provider request-local session 的分层说明。

### 不包含内容

- Runtime query 或 provider workflow 的新能力与协议实现。
- 外部宿主模型内部决策的黑盒准确率承诺。
- 与本 trigger/capability 合同无直接关系的 Agents 目录重构、全库文档清理或历史迁移。

### 业务规则

- 宿主兼容顺序固定为 shared kernel -> Codex reference projection -> Claude compatible projection -> CodeBuddy compatible projection；次级宿主的专属机制不得决定共享合同形态。
- Runtime 是 query 语义、质量状态、ranking、answer 和恢复动作的唯一规则所有者；宿主 trigger 只决定是否建议或选择已公开 action。
- query/status 可依据共享 taxonomy 进行语义建议；具有副作用或高成本的 action 必须保持 explicit-only。
- `ambiguous` 必须 fail closed：可以提示、建议显式 action 或请求补充非空 term，但不得自动执行 CLI。
- 机制 parity 不等于语义 parity。宿主不支持某种 hook 时必须在 capability matrix 中明确，而不是用无法验证的文案声称支持。
- 当前阶段不要求保留旧 trigger 关键词或旧 query 字段兼容层；确认共享 authority 后可以删除重复分类和过时 fallback。

## 交付形态

single-change

这是 parent `close-specwiki-3-0-design-baseline` 下顺序第 5 个 child，依赖已归档的 `close-specwiki-3-0-design-baseline-runtime-query-contract`。它只收口三宿主触发与薄消费合同，为最后的 `documentation-closure` 提供稳定输入，不重新拆分 parent scope。

## 风险

- 三宿主的可观测能力天然不对称；若把 parity 错定义为相同 hook 机制，会伪造 Codex/Claude 能力并扩大宿主集成范围。
- trigger taxonomy 若一次覆盖所有 action 和复杂意图，容易演变为通用 NLU 或 richer query change；mutation action 必须保持 explicit-only。
- 多语言语料过窄会漏掉真实表达，过宽则可能把 change 扩大为准确率优化项目；本 change 应验证合同闭集与关键边界，不承诺开放域语言理解。
- 现有 Agents Wiki、当前 capability 与代码仍有路径和 session 表述漂移；本 change 修正所有直接依赖 trigger/capability/session 分层的当前 authority 与入口，其余无关 roadmap/历史术语由 documentation-closure 处理。
- 若 capability matrix 只存在于 Markdown，它会再次与生成资产漂移；必须有机器可消费与测试证据。

## 未知项

- trigger decision 的最小 reason/evidence 闭集、优先级、冲突解析和多 target 表达方式。
- shared corpus 的具体文件格式、版本策略、locale 扩展方式，以及如何区分 prompt 文本与宿主事件包络。
- `ambiguous` 在不同 capability 下应注入 context、建议 `wiki-query` 还是请求用户提供 term；所有方式都必须保持不自动执行。
- 如何在当前 registry/asset planner 上建立最小 `compatibilityRole + triggerCapabilities` 合同，同时把完整 `HostAdapter` 重构留给独立 change。
- SessionStart 固定 context 与 UserPromptSubmit decision 是否共用部分 taxonomy，以及如何避免 session context 被误读为已经执行 action。

## 参考资料

- [宿主触发合同现状审计](./research/host-trigger-contract-audit.md)：来源为本 change 的定向 proposal 调研；目标落点是 problem、goals、non-goals、success criteria、risks 和 unknowns；采用方式为改写提炼。
- `../close-specwiki-3-0-design-baseline/split.md`：来源为当前 parent；目标落点是 child 目标、验收边界、顺序和依赖；采用方式为直接约束。
- `../../../.wiki/06-设计文档/06-Runtime查询合同.md`：来源为已归档 Runtime Query child 的 canonical authority；目标落点是稳定 query fields、term-only 输入和宿主 thin-consumption 边界；采用方式为直接约束。
- `../../../.wiki/06-设计文档/02-Agents设计.md`：来源为当前 Agents 设计；目标落点是宿主职责、HostAdapter 方向、资产机制和 bridge/session 边界；采用方式为现状核对与冲突识别，不直接复制推荐代码。
- `../../../packages/spec-wiki/src/agents/shared/commandAssets.ts`、`shared/hosts.ts`、`shared/hostAssets.ts` 与三个宿主 `assets.ts`：来源为当前代码事实；目标落点是共享 query renderer、真实 trigger mechanism、capability 缺口和影响范围；采用方式为事实核对。
- `../../archive/2026-03-30-iteration-11-10-codebuddy-skill-conformance/`、`../../archive/2026-03-30-iteration-11-11-codex-claude-host-conformance/`：来源为历史宿主 conformance；目标落点是共享 skill 形态、thin boundary 和资产 ownership 经验；采用方式为改写借鉴，不沿用旧 action/字段口径，不作为当前 authority。
- `../../archive/2026-07-16-close-specwiki-3-0-design-baseline-reliability-lifecycle/`：来源为已归档 reliability lifecycle；目标落点是 request-local provider session 与 persistence forbidden 边界；采用方式为直接约束。
- 本 change 未使用 upstream；不存在外部实现的直接迁移、改写或仅借鉴。
