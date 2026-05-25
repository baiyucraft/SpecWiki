## ADDED Requirements

### Requirement: answer assembly 必须只消费 formal knowledge inputs
系统 MUST 让 answer assembly 只消费正式 substrate，而不是继续依赖页面正文或临时 cache。正式输入 MUST 仅来自 `index / graph hits`、`declared records`、`derived summaries`、`projection/page refs` 与 `health / readiness`。系统 MUST NOT 把页面 Markdown、临时 prompt 输出或其它非正式对象直接视为 answer 主真相。

#### Scenario: answer 主要依赖 knowledge route 结果
- **WHEN** 某次 answer 需要基于 knowledge route 返回结果
- **THEN** 系统 MUST 优先消费 `declared records`、`derived summaries` 与 `projection/page refs`
- **THEN** 系统 MUST NOT 继续把页面正文或临时 cache 当作唯一 answer substrate

#### Scenario: answer 消费 facts 与 graph 命中
- **WHEN** 某次 answer 主要基于 `index / graph hits` 回答定位、调用链或影响范围问题
- **THEN** 系统 MUST 允许这些 formal facts 作为 answer 输入
- **THEN** 系统 MUST 保留这些输入的 provenance，而不是伪装成 knowledge summary

### Requirement: 系统必须定义最小 AnswerEnvelope
系统 MUST 为 answer 定义最小正式输出壳 `AnswerEnvelope` 或等价对象。该输出 MUST 至少包含 `answer_mode`、`answer_trust`、`recommended_action`、`provenance` 与 `supporting_refs`，使宿主与 Agent 能稳定消费 answer，而不必重新推断其来源与可靠性。

#### Scenario: direct answer 返回正式 envelope
- **WHEN** 当前 formal substrate 足以直接回答问题
- **THEN** 系统 MUST 返回包含 `answer_mode`、`answer_trust`、`recommended_action`、`provenance` 与 `supporting_refs` 的正式 envelope
- **THEN** 调用方 MUST 不需要自行回推这次 answer 的 route 或下一步动作

#### Scenario: answer 输出 supporting refs
- **WHEN** 某次 answer 引用了 declared、derived、projection 或 facts substrate
- **THEN** `supporting_refs` MUST 指向这些正式对象或其稳定引用
- **THEN** 系统 MUST 不得只返回一段无法追溯 formal source 的自由文本

### Requirement: answer assembly policy 必须区分 direct、degraded 与 refuse
系统 MUST 为 answer assembly 定义稳定的装配策略，至少区分 `direct`、`degraded` 与 `refuse` 三类模式。`direct` 表示当前 substrate 足以支持正式回答；`degraded` 表示当前仍可回答，但必须显式暴露风险与后续动作；`refuse` 表示当前 formal substrate 不足或风险不可接受，不能继续组装 answer。

#### Scenario: substrate 完整时返回 direct
- **WHEN** query route、knowledge artifacts 与 readiness 均满足正式回答前提
- **THEN** answer assembly MUST 返回 `direct` 模式
- **THEN** 系统 MUST 不把该场景错误压成 degraded 或 refuse

#### Scenario: substrate 不足时返回 refuse
- **WHEN** 当前 formal substrate 无法支持可追溯 answer，或当前风险不允许继续回答
- **THEN** answer assembly MUST 返回 `refuse` 模式
- **THEN** 系统 MUST 不得偷偷用页面正文或临时 cache 兜出看似正常的答案

### Requirement: direct、degraded 与 refuse 的触发条件必须形成稳定判定矩阵
系统 MUST 为 `direct`、`degraded` 与 `refuse` 定义稳定的触发条件矩阵。至少 MUST 明确：formal inputs 是否完整、route 是否跌入 `page_fallback`、runtime 是否 stale、health 是否 degraded、是否存在 open governance conflict 这几类条件如何影响 answer 模式。系统 MUST NOT 让宿主或实现代码在没有正式矩阵的情况下自行猜测 answer mode。

#### Scenario: formal inputs 完整且无显著风险时进入 direct
- **WHEN** formal inputs 完整、route 未跌入 `page_fallback`，且当前不存在 stale runtime、health degradation 或 open governance conflict
- **THEN** answer assembly MUST 进入 `direct`
- **THEN** 系统 MUST 不得把该场景误判为 `degraded` 或 `refuse`

#### Scenario: formal inputs 仍可消费但存在显著风险时进入 degraded
- **WHEN** formal inputs 仍足以支撑可追溯 answer，但 route 已跌入 `page_fallback`、runtime stale、health degraded 或 open governance conflict 至少命中其一
- **THEN** answer assembly MUST 进入 `degraded`
- **THEN** 系统 MUST 明确输出导致降级的 provenance 与推荐动作

#### Scenario: formal inputs 不足或风险不可接受时进入 refuse
- **WHEN** formal inputs 不足以支持可追溯 answer，或当前风险策略要求停止回答
- **THEN** answer assembly MUST 进入 `refuse`
- **THEN** 系统 MUST 不得让宿主再根据页面正文、自由文本或旧 cache 自行补出“正常答案”
