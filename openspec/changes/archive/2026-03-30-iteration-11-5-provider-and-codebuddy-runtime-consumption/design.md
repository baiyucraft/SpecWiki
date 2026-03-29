## Context

当前仓库里，`wiki-runtime` 其实已经具备一组足以被宿主消费的真实能力，但这些能力还没有被收口成“当前阶段 runtime profile”。

从真实代码看：

- [`crates/wiki-runtime/src/transport/cli.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/transport/cli.rs) 已经把 `init / status / update / query / sync / rebuild` 暴露为统一 transport 入口。
- [`crates/wiki-runtime/src/transport/json_rpc.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/transport/json_rpc.rs) 已经支持长流程 NDJSON 事件流，包含 `progress / result / error / llm_request`，并保留了 session 路径。
- [`crates/wiki-runtime/src/workflows/status.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/workflows/status.rs) 当前只返回 `state / dirty_sources / dirty_pages / needs_rebuild_reason`，能回答“脏不脏”，但不能回答“当前 query 是否可用、推荐下一步是什么、为什么还没 ready”。
- [`crates/wiki-runtime/src/domain/checkpoint.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/domain/checkpoint.rs) 已经定义了 `PipelineRuntimeSummary` 与 `UnitRuntimeGate`，说明 runtime 内部已经有 workflow readiness 与 blocker 模型，但宿主消费面还没有正式收口。
- [`agents/codebuddy/src/runtime/invokeCore.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/runtime/invokeCore.ts) 已经能消费长流程 `progress` 和可选 `llm_request`，但 [`agents/codebuddy/src/runtime/parseResult.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/runtime/parseResult.ts) 只校验最薄的协议壳；对 `usage`、runtime summary、gate/readiness、provider-direct vs agent-bridge 没有正式消费语义。
- [`agents/codebuddy/src/tools/wikiStatus.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/tools/wikiStatus.ts) 与 [`agents/codebuddy/src/tools/wikiQuery.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/tools/wikiQuery.ts) 仍只是“把参数透传给 core”，还没有当前阶段场景所需的 preflight / trust / next action 轮廓。

从场景边界看：

- [`SCENE-1.md`](E:/project/!byAI/spec-wiki/SCENE-1.md) 场景 3、4 要求在改代码前先“找到入口、分析影响”。
- [`SCENE-1.md`](E:/project/!byAI/spec-wiki/SCENE-1.md) 场景 9 与 [`DESIGN-RUNTIME.md`](E:/project/!byAI/spec-wiki/DESIGN-RUNTIME.md) 又要求本地 runtime 恢复后能直接 `query`，并且 stale/missing 状态不能静默误导用户。
- [`SCENE-2.md`](E:/project/!byAI/spec-wiki/SCENE-2.md) 的 A1、A3、A9 也都在强调：新 Agent onboarding、索引/知识变旧、工具链失败时的降级状态，必须能被外部消费者稳定理解。

同时你已经明确把 knowledge/page 的正式消费留到后续迭代。因此 11.5 的设计边界只能是：让 Provider / CodeBuddy 能用“当前阶段的 `wiki-runtime`”，而不是提前把 `wiki-knowledge`、page projection 或完整 Agent consumption 一并定版。

## Goals / Non-Goals

**Goals:**

- 为当前阶段的 `wiki-runtime` 定义一份稳定的宿主消费轮廓，覆盖 `status`、`query`、长流程事件与 runtime readiness。
- 让 Provider / CodeBuddy 可以在不读取内部实现细节的前提下判断：runtime 是否可用、是否应先 `init/update/rebuild/sync`、当前 query 是否可信、workflow 正停在哪一层。
- 让 `CodeBuddy` 继续保持 thin boundary，但不再只消费 `unknown` 结果壳。
- 让 provider-direct、agent-bridge 与 deterministic-only 三种执行路径在真正跑过长流程后对宿主可观测，而不是靠“有没有收到 llm_request”间接猜测。
- 保持 index-first / 当前阶段 facts-first 的消费方向，为后续 knowledge/page 消费迭代留清晰边界。

**Non-Goals:**

- 不在本轮定版 knowledge/page 的正式宿主消费 contract。
- 不在本轮新增高层“Agent 业务工具”，例如让 CodeBuddy 在 TS 层实现任务分解、改动入口推理或页面语义路由。
- 不在本轮重做 `query` 的完整自然语言协议或最终 runtime query route 收口。
- 不在本轮推翻 provider 直连、agent-bridge 或现有六个 CodeBuddy 工具集合。
- 不把 `wiki-runtime` 变成新的知识真相层；它仍然只是当前阶段的 orchestration、transport、storage 和 lifecycle 壳。

## Decisions

### 决策 1：把宿主消费面收成 `preflight -> action -> query` 三段，而不是新增高层业务工具

当前阶段对 Provider / CodeBuddy 最有价值的不是再长一组新工具，而是把现有 `wikiStatus / wikiInit / wikiUpdate / wikiQuery / wikiSync / wikiRebuild` 的使用方式收成稳定轮廓：

1. `preflight`
   - 通过 `status` 判断 runtime 是否 `fresh / stale / missing / needs_rebuild`
   - 读取 facts/query readiness、推荐下一步动作，以及可用时的 runtime 摘要投影
2. `action`
   - 通过 `init / update / rebuild / sync` 推进 runtime
   - 通过事件流观察 progress、usage、执行路径和 blocker
3. `query`
   - 在 runtime 足够 ready 时执行 index-first `query`
   - 从返回结果里区分 facts/index 命中与 page fallback

这里的 `preflight -> action -> query` 只是推荐消费轮廓，不是要求 CodeBuddy 在 TS 层实现一套新的状态机。runtime 仍然是语义 owner，宿主只消费其结果。

这样做的理由：

- 当前 CodeBuddy 只有薄工具包装，继续加宿主业务工具只会把 Wiki 语义抬到 TS 层，违背 `Agents` 边界。
- 当前阶段真正缺的是“怎么用现有 runtime”，而不是“再加一层 Agent-side orchestration”。

备选方案：

- 新增 `wikiPrepareTask`、`wikiAnalyzeImpact` 之类高层工具。
  - 否决原因：这些名字看起来方便，但会把 query route、状态判断和 Wiki 业务规则带进 Agent 层。
- 保持六个工具不变，但不定义正式消费顺序。
  - 否决原因：宿主仍只能靠猜测决定什么时候该 `status`、什么时候该 `update`、什么时候 query 结果可信。

### 决策 2：`status` 必须升级为正式 preflight 结果，而不是只返回脏文件列表

本轮把 `status` 从“脏判断”升级为“宿主 preflight”。

在保留现有 `state / dirty_sources / dirty_pages / needs_rebuild_reason` 的同时，新增当前阶段宿主可消费的字段：

- `query_readiness`
  - `ready`
  - `needs_init`
  - `needs_update`
  - `blocked`
- `recommended_action`
  - `none`
  - `init`
  - `update`
  - `rebuild`
  - `sync`
- `facts_ready`
  - 是否已经具备当前阶段可查询的 facts/runtime snapshot
- `runtime_summary`
  - 在当前 runtime 已持久化 `PipelineRuntimeSummary` 时的可选投影
- `gate_summary`
  - 在当前 runtime 已持久化 `UnitRuntimeGate` 时的可选投影，而不是整库直出
- `llm_mode_hint`
  - 仅表示配置/可用性层面的预判，例如 `provider_configured / bridge_available / deterministic_default`
  - 不表示某次 workflow 已真实采用的执行路径

这里的关键约束是：`status` 必须能回答“现在能不能安全 query / 开工”，而不是只回答“有几个脏文件”。

这样做的理由：

- 真实场景里，Provider / CodeBuddy 最需要的不是脏文件列表，而是下一步动作和可信度边界。
- 当前 runtime 已经有 `PipelineRuntimeSummary / UnitRuntimeGate` 这类内部状态对象；但 `status` 本体并不依赖它们，所以只有在这些对象已落盘时才应作为可选投影暴露。

备选方案：

- 继续只返回 `fresh/stale/missing/needs_rebuild`。
  - 否决原因：状态过粗，无法支持“query 是否可信”“为什么 blocked”“现在该先做什么”。
- 直接把 `UnitRuntimeGate[]` 全量暴露给宿主，或要求 `status` 无条件返回这些对象。
  - 否决原因：这会把 runtime 内部模型原样泄漏出去，宿主依赖会过深，也会把缺 checkpoint 错写成协议失败。

### 决策 3：`query` 的当前阶段消费语义必须显式表达 trust，而不是只给结果列表

本轮不重做 query 输入协议，但要明确宿主消费输出时必须看到的边界：

- `runtime_state`
  - 当前 query 时 runtime 处于什么 readiness
- `query_mode`
  - `index_first`
  - `page_fallback`
  - `mixed`
- `query_trust`
  - `ready`
  - `stale_but_queryable`
  - `blocked`
- `recommended_action`
  - 如果结果不可信或 runtime 未 ready，明确告诉宿主下一步
- `provenance_summary`
  - 继续保留，但不再是唯一 trust 信号

这里不改变“当前 query 仍以 `term` 为外部入口”的事实，但要求结果必须告诉宿主：

- 这是事实层命中，还是 page fallback
- 当前 runtime 是 ready，还是只是暂时可查
- 是否建议先 `update / sync / rebuild`

这样做的理由：

- `SCENE-1` 的找入口、看影响，都要求“结果可信且带理由”，不是只给一串 matches。
- 在 knowledge/page 消费推迟的前提下，当前阶段唯一可依赖的是 facts/index 与 runtime readiness 的组合。

备选方案：

- 继续只返回 `QueryReport`，让宿主从 `matched_*` 和 `provenance_summary` 自己猜可信度。
  - 否决原因：宿主层很容易把 page fallback 当成 facts 结果，或者在 stale runtime 上盲目信任旧结果。

### 决策 4：长流程事件流继续复用 NDJSON，但要补齐宿主真正需要消费的信号

本轮不新增另一套 transport，而是在现有 `progress / result / error / llm_request` NDJSON 上补齐宿主当前阶段真正需要的字段：

- `progress`
  - 继续保留 `action / phase / message / elapsed_ms / processed / total`
  - 正式要求宿主可消费 `usage`
  - 允许在关键阶段附带 `runtime_state` 或等价摘要引用
- `result`
  - 对真正执行 workflow 的长流程 action，最终 `data` 必须包含稳定的 `runtime_summary`
  - 对真正执行 LLM/runtime 路径的 action，最终 `data` 必须包含真实 `llm_execution_mode`
- `error`
  - 对失败的长流程路径，必须尽量包含 `runtime_summary` 或 blocker 线索，而不是只有裸错误文本
- `llm_request`
  - 继续只在 `agent_bridge` 路径下出现
  - 宿主不能再用“有没有 llm_request”作为唯一模式判断，而应以实际 workflow 终态里的 `llm_execution_mode` 为准

CodeBuddy 侧对应变化是：

- `parseResult.ts` 必须从“只验证最薄壳”升级到“能消费 usage、runtime summary 和执行路径”
- `invokeCore.ts` 仍然只负责进程/事件桥接，不解释 Wiki 业务语义

这样做的理由：

- 当前 `WorkflowProgressEvent` 已经有 `usage`，但 CodeBuddy parser 还没正式消费。
- provider direct path 下不会出现 `llm_request`；只有实际跑过长流程之后，runtime 自己才能给出真实执行路径，宿主不应再靠推断。

备选方案：

- 新增 `runtime_summary` 专用事件类型。
  - 暂不采用：当前阶段先用现有 `result` 与可选 `progress` 字段就够，避免 transport 扩散过快。

### 决策 5：Provider 与 CodeBuddy 共享同一 runtime profile，但消费位置不同

本轮不为 Provider 和 CodeBuddy 分别设计两套 contract。

统一原则是：

- `wiki-runtime` 提供同一份当前阶段 profile
- `CodeBuddy` 作为外部宿主，消费 `status / query / progress / result / error`
- provider 直连或 agent bridge 作为 workflow 内执行路径，消费同样的 readiness、usage 和执行路径信号

这意味着：

- `runtime_summary`、`recommended_action` 是跨消费者共用的
- `llm_mode_hint` 是 preflight 预判信号，`llm_execution_mode` 只在实际 workflow 终态里出现
- CodeBuddy 不重写 runtime 语义
- provider 相关逻辑也不绕过 runtime 自己定义另一套状态判断

这样做的理由：

- 当前设计边界里，`wiki-runtime` 就是 orchestration / transport / lifecycle 壳；消费 contract 应该统一在这里生长。
- 如果 Provider 和 CodeBuddy 各长一套，后续 13/15 很难收回。

备选方案：

- CodeBuddy 消费一套外部 contract，provider 再消费一套 workflow 内 contract。
  - 否决原因：重复造两层语义，后面只会越来越不一致。

## Risks / Trade-offs

- [风险：11.5 会提前吞掉 13/15 的完整 runtime/agent 收口] → 本轮只定义当前阶段的 runtime consumption profile，不定版 knowledge/page 消费，不新增高层 Agent 业务工具。
- [风险：`status` 被误用成“真实执行路径探针”] → `status` 只允许给预判型 `llm_mode_hint`，真实 `llm_execution_mode` 只能由实际跑过的 workflow 终态给出。
- [风险：`status` 塞太多字段后重新变成内部状态直出] → 只暴露宿主决策必需的 `readiness / recommended_action / 可选 summary`，不把整个 SQLite/runtime 内部对象原样公开，也不要求无条件存在。
- [风险：CodeBuddy parser 变厚，重新承担 Wiki 语义] → parser 只负责类型化和协议校验，不做状态推理和业务决策。
- [风险：provider-direct vs agent-bridge 的执行路径信号与真实执行不一致] → 由 runtime 自己在 workflow 终态里产出 `llm_execution_mode`，宿主只消费，不推断。
- [风险：当前 query 结果增加 trust 字段后与后续 knowledge/page query 语义冲突] → 字段命名明确限定为“当前阶段 runtime trust/profile”，后续 knowledge/page 消费迭代可以在更高层扩展，不覆盖本轮含义。

## Migration Plan

1. 先扩展 `wiki-runtime` 的 `status`、长流程终态与必要的 query 输出，形成当前阶段宿主可消费 profile。
2. 在 `wiki-runtime` 中统一生成 `runtime_summary / recommended_action / llm_mode_hint / query_trust` 等投影字段，并只在实际 workflow 终态里生成 `llm_execution_mode`，而不是让各宿主自己拼。
3. 更新 CodeBuddy 的 `parseResult.ts`、`invokeCore.ts` 与相关工具类型，只消费这套正式 profile，不引入新的业务推理。
4. 为 provider direct、agent bridge、deterministic-only 三种路径补测试，确保执行路径与事件/终态信号一致。
5. 补充 spec 与 tests，确认知识/页面消费面仍明确留在后续迭代。

回滚策略：

- 若 profile 设计不稳定，应回滚新增字段与 CodeBuddy typed parser，保留现有最薄协议壳。
- 不允许通过在 Agent 层增加业务逻辑来“绕过” runtime profile 不完整的问题。

## Open Questions

- `gate_summary` 是返回聚合计数 + top blockers，还是允许返回少量稳定 `unit_id` 明细；当前倾向只返回聚合 + 少量 blocker 摘要，避免宿主绑死内部结构。
- `query_trust = stale_but_queryable` 的边界是否只取决于 `status.state=stale`，还是还要结合 facts snapshot 的时间戳；当前倾向先基于 runtime state 与 facts readiness 做最小版本，不提前把时间戳策略做复杂。
