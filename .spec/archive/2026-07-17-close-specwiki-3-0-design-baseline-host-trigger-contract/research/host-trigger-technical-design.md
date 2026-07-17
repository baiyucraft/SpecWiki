# 宿主触发合同技术设计调研

## 调研目的

- 阶段：design
- 关联 change：`close-specwiki-3-0-design-baseline-host-trigger-contract`
- 服务边界：design
- 要回答的问题：如何用当前 TypeScript/宿主资产架构实现单一 trigger taxonomy、确定性 evaluator、真实 capability matrix、跨宿主 corpus、CodeBuddy hook delivery，以及 trigger/bridge/provider session 分层。
- 停止条件：对象语言、优先级、模块落点、数据流、错误策略、迁移边界和验证方向足以支撑 design，不提前生成测试用例或任务清单。

## 结论摘要

- 新增共享纯函数 `evaluateHostTrigger`，输入只含未知类型的 prompt text，输出闭集 `decision / targetAction / reason / evidence`；evaluator 不接收 host id、不读文件、不调用 CLI，因此语义层天然跨宿主一致。
- query/status 采用 semantic-or-explicit policy；init/update/sync/rebuild 采用 explicit-only policy。决策 reducer 使用固定优先级，显式 opt-out、冲突、缺 term、mutation guard、语义命中和 no-match 不由宿主自行解释。
- 不补齐完整 HostAdapter。最小扩展现有 `HostDefinition`，要求每个宿主声明 `compatibilityRole` 与 trigger capabilities；Codex 是唯一 `reference`，Claude、CodeBuddy 是 `compatible`。资产构建完成后用公共 validator 核对声明与实际 asset kinds，形成机器可消费矩阵。
- Codex repo-local skills 是首要 reference projection；Claude 从同一 shared policy 生成兼容 skills。CodeBuddy `UserPromptSubmit` 再通过隔离的结构化事件 parser 调用共享 evaluator并映射为 context delivery；SessionStart 只输出固定 orientation context，不进入 evaluator。CodeBuddy 专属机制不反向决定 shared/Codex 合同。
- 语义 corpus 与宿主 adapter/event corpus 分区：前者对三个宿主运行同一 decision 断言，后者只验证真实可观测的 CodeBuddy 包络、可执行 hook 和 delivery。
- 仓库自身没有 CodeBuddy prompt event 字段 fixture；本机 CodeBuddy 官方插件开发资料明确 `UserPromptSubmit` 使用 `user_prompt`。实现阶段以该字段建立可执行 v1 fixture；解析失败始终 fail closed，禁止扫描 raw serialized JSON 或猜测多字段 fallback。

## 已读取资料

| 路径 / 来源 | 目的 | 关键发现 |
| --- | --- | --- |
| `proposal.md`、`research/host-trigger-contract-audit.md` | 固定目标、非目标和成功标准 | 需要三态 taxonomy、capability matrix、corpus、thin query consumption 与三层边界 |
| `packages/spec-wiki/src/wikiActions.ts` | 复用公开 action 闭集 | `PublicWikiAction` 已是唯一 action 类型，trigger 不应复制 action enum |
| `agents/shared/workflowSemantics.ts`、`commandAssets.ts` | 核对共享文案与现有 hook renderer | action semantics 已共享；query canonical fields 正确；CodeBuddy 仍扫描 raw stdin substring |
| `agents/shared/hosts.ts`、`hostAssets.ts` | 确认最小 compatibility/capability 落点 | `HostDefinition` 只有检测字段，资产由 switch 构建；可增量加入 required `compatibilityRole`、capabilities 与 post-build validator |
| 三宿主 `assets.ts`、`bootstrap.test.ts`、`assets.test.ts` | 确认真实 delivery mechanism | Codex/Claude 仅 skills；CodeBuddy 还有 hooks/settings；测试可扩展为资产/能力和可执行 hook 对拍 |
| `orchestration/init/plan.ts`、`runInit.ts` | 核对 bootstrap 数据流 | 现有 plan/write/report 无需重构，只需消费经过 capability 校验的 assets |
| `.wiki/06-设计文档/06-Runtime查询合同.md` | 固定 Runtime authority | trigger 不得增加 richer request 或重建 route/ranking/readiness/trust/action/answer |
| reliability lifecycle archive 与 runtime session tests | 固定 provider session 边界 | request 恢复必须从 `session=None` 开始，durable storage 禁止 session summary/turn/tool refs |
| 历史 host conformance archives | 复核资产 ownership 和 renderer 经验 | 只改写借鉴共享 renderer、受管资产和幂等刷新；旧 action/字段口径不迁移 |

## 子代理调研

| 子代理 / 角色 | 调研主题 | 结论 | 主会话复核 |
| --- | --- | --- | --- |
| `trigger_model_design` / trigger contract designer | taxonomy、decision schema、reducer precedence、corpus 和 adapter delivery | 推荐共享纯 evaluator、闭集 reason/evidence、单 target、mutation explicit-only、语义与包络 corpus 分区；不补齐完整 HostAdapter | accepted；主会话结合 host asset/build/test 调用链调整为扩展 `HostDefinition` + post-build capability validator |
| `bridge_session_audit` / boundary reviewer | trigger、host-agent bridge 与 provider session 边界 | 基础 bridge forwarding 已实现，production research/agent-session bridge 未启用；provider session 限于一次 `research_page` 调用且不可持久化；trigger 不携带 transport/session identity | accepted；用于同步 capability 与 Agents authority，不改变 Runtime DTO |

## 关键发现

### Codex reference 与兼容投影必须有明确顺序

- 证据：当前 Codex/Claude 都生成 repo-local action skills，CodeBuddy 额外拥有 hooks/settings；机制更丰富不代表架构优先级更高。
- 说明：若以 CodeBuddy hook 反推公共合同，公共对象语言会被单一宿主事件包络绑死，也会让 Codex 的 native skill discovery 退化为次级兼容。
- 影响：规划与验收顺序固定为 shared kernel -> Codex reference -> Claude compatible -> CodeBuddy compatible；`HostDefinition.compatibilityRole` 使该顺序成为显式合同。

### 纯 evaluator 与宿主 delivery 必须分层

- 证据：Codex/Claude 没有 prompt hook，CodeBuddy 有 UserPromptSubmit；proposal 要求语义 parity 而非机制 parity。
- 说明：如果 evaluator 接收 host id，机制差异会污染 decision identity；如果 hook 直接执行 CLI，则 ambiguous/negative 和 mutation guard 无法 fail closed。
- 影响：共享 evaluator 只产生不可变 decision；宿主 adapter 在其后选择 `action_context / clarification_context / orientation_context / none`，任何 delivery 都不直接执行 action。

### 最小 capability 扩展比完整 HostAdapter 重构更合适

- 证据：当前 orchestration 只依赖 `SupportedHost` 与 `buildHostBootstrapAssets`；proposal 明确不重构整个 bootstrap。
- 说明：完整迁移到 Wiki 推荐的 HostAdapter 会同时改变 registry、detect、context、assets、ownership 和 orchestration，超出 trigger contract。
- 影响：在 `HostDefinition` 增加 required `triggerCapabilities`，保留现有 build switch；公共 validator 对构建结果执行 drift check。未来完整 adapter 可另立 change。

### 当前 raw stdin substring 必须删除

- 证据：`renderCodeBuddyHookScript` 对完整 stdin lower-case 后搜索裸关键词。
- 说明：字段名、路径、代码片段和 JSON 其它属性都可能误命中；无法判断 malformed envelope，也无法提供稳定 evidence。
- 影响：CodeBuddy adapter 先 JSON parse 和 schema extract，再调用 evaluator；未知包络、缺字段、非字符串、null 和空白全部产生 `invalid_input` 且 delivery 为 none。

### Corpus 需要区分合同层与机制层

- 证据：外部宿主模型 skill selection 不可自动观测，而 CodeBuddy hook 可以在 Node 子进程中执行。
- 说明：强迫所有 host 跑 hook fixture 会伪造能力；只断言 skill 字符串又不能证明 decision reducer。
- 影响：semantic cases 对共享 evaluator 和三宿主 contract projection 运行；adapter cases 只对 capabilities 声明的真实机制运行。Codex/Claude 的验收是 policy-derived guidance，不是模型准确率。

### Bridge/session Wiki 存在历史过度承诺

- 证据：基础 `--bridge-stdio` 与 `llm_request/llm_response` forwarding 已存在，但 active bridge 拒绝 agent-session events，production `research_page` 会 block agent bridge；部分 capability 却仍描述 durable stable session 与自动 bridge fallback。
- 说明：本 change 不能重构 provider，但必须阻止 trigger contract 复用这些字段或承诺。
- 影响：trigger DTO、corpus、hook output 和 generated context 禁止 session identity/persistence 字段；当前 authority 必须区分已实现 transport、未启用 production bridge 与 request-local provider session。

## 方案与取舍

| 选项 | 优点 | 缺点 | 结论 |
| --- | --- | --- | --- |
| 完整实现 Wiki 推荐 HostAdapter | 抽象整齐 | 扩大到 detect/context/ownership/orchestration 重构 | 不采用；另立 change |
| 扩展 HostDefinition compatibility role/capabilities + asset validator | 最小、编译期强制新宿主声明、运行/测试可核对 | 现有 build switch 仍保留 | 采用；Codex 唯一 reference |
| 让每个宿主各自判断 trigger | 宿主自由度高 | taxonomy、优先级和 evidence 必然漂移 | 不采用 |
| 共享 evaluator + 宿主 delivery adapter | 语义唯一、机制诚实、可独立测试 | CodeBuddy 生成脚本需对拍防止实现漂移 | 采用 |
| Hook 调用新的 CLI evaluator 命令 | 避免脚本算法复制 | 每次 prompt 启进程，扩大公开 CLI 与失败面 | 不采用 |
| 由共享规则生成自包含 hook，并与库 evaluator 全 corpus 对拍 | 保持 repo-local hook，无新 CLI | renderer 需要稳定序列化规则和小型 interpreter | 采用 |
| Corpus 只用 TypeScript inline cases | 类型方便 | 难以作为版本化、宿主无关 fixture 审计 | 不采用 |
| JSON v1 corpus + 严格 parser | 可版本化、可由 package/adapter tests 共同读取 | 需要 schema validation | 采用 |

## 风险与未知项

| 项目 | 影响 | 处理方式 |
| --- | --- | --- |
| CodeBuddy prompt event schema 后续变化 | hook 可能无法提取 prompt | v1 fixture 只接受官方资料已证明的 `user_prompt`；其余 fail closed，后续变化显式升级 fixture/version |
| 规则表演变为通用 NLU | 范围和维护成本失控 | 只承诺 v1 corpus 闭集、词边界和请求短语；no-match fail closed |
| Generated hook 与库 evaluator 漂移 | CodeBuddy 决策偏离共享合同 | 全 semantic + adapter corpus 对拍，生成脚本作为可执行 artifact 测试 |
| Evidence 断言过细 | 新增等价规则导致语料脆弱 | 所有 case 断言 decision/action/reason；只在边界 case 固定 ruleId |
| Capability 仅被测试读取 | matrix 仍可能成为旁路文档 | buildHostBootstrapAssets 在返回前执行公共 validator；测试覆盖所有 HOSTS |
| 旧受管 hook 仍包含关键词逻辑 | init 前旧仓库行为不变 | 不保留兼容 fallback；下一次 `spec-wiki init` 按 ownership 覆盖受管 hook |

## 对当前 artifact 的影响

- 应写入：`design.md`
- 影响内容：
  - 明确共享类型、policy、reducer precedence、normalization、decision invariants 和 delivery 闭集。
  - 明确最小 capability 扩展、asset validator、CodeBuddy parser/hook、corpus schema 与测试分层。
  - 明确无新 CLI/Runtime DTO/持久化，旧 hook 通过现有受管覆盖迁移。
  - 明确直接 Wiki 同步落点与 documentation-closure 的剩余边界。
- 后续阶段处理：
  - system-tests/tasks 规划具体 corpus case、Red-Green-Refactor 顺序和全量 gate。

## 未采纳内容

- 不直接迁移 `.wiki/06-设计文档/02-Agents设计.md` 中完整 HostAdapter 代码片段；只借鉴公共内核与宿主差异分层。
- 不沿用历史 archive 的四 action、`matched_pages`、`provenance_summary` 或 durable session 表述。
- 不采用外部 NLU、正则库或 upstream classifier；本调研未使用 upstream。
