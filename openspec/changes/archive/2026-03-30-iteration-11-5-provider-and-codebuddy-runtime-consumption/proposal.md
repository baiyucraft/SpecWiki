## Why

`wiki-runtime` 在当前阶段已经不只是一个“能跑命令的 binary”：[`crates/wiki-runtime/src/transport/cli.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/transport/cli.rs) 已经暴露 `init / status / update / query / sync / rebuild`，[`crates/wiki-runtime/src/transport/json_rpc.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/transport/json_rpc.rs) 已经支持 `progress / result / error / llm_request` 事件流，[`crates/wiki-runtime/src/domain/checkpoint.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/domain/checkpoint.rs) 还定义了 `PipelineRuntimeSummary` 与 `UnitRuntimeGate`。但对 `Provider / CodeBuddy` 来说，这些能力目前仍是“存在于代码里”，而不是一个可稳定消费的当前阶段 runtime profile；现有 CodeBuddy 侧 [`invokeCore.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/runtime/invokeCore.ts) 与 [`parseResult.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/runtime/parseResult.ts) 也只消费最薄的协议壳。

这轮要现在做，因为 11 已经把焦点放在 `wiki-index` 初步可用，而你又明确把 knowledge/page 的消费留到后续迭代。于是 11.5 最合理的任务不是提前做完整 runtime/agent 收口，而是先回答：在不触碰 knowledge/page 正式消费面的前提下，`Provider / CodeBuddy` 如何基于当前阶段的 `wiki-runtime` 满足场景里的新需求，例如 runtime 是否可用、是否需要先恢复/刷新、当前 query 结果是否可信、provider 直连还是 agent bridge 正在生效，以及在长流程中如何消费进度、usage 和 gate 状态。

## What Changes

- 为当前阶段的 `wiki-runtime` 定义一份“宿主可消费 runtime profile”，覆盖 `status / query / init / update / rebuild / sync` 的正式消费轮廓，而不是让 Provider / CodeBuddy 继续只拿到 `CoreResponse.data: unknown`。
- 强化 `status` 与相关 runtime 输出，使宿主能明确知道当前 runtime 是 `fresh / stale / missing / needs_rebuild`，以及 facts 是否可查、推荐下一步动作是什么；`runtime_summary / gate` 这类摘要只在当前 runtime 已持久化相关状态时作为可选投影暴露。
- 强化 `query` 的当前阶段消费语义，让宿主能依赖 index-first 结果、page fallback provenance、runtime readiness 和可信度边界来满足“找入口 / 看影响 / 先定位再改”的场景，而不要求后续 knowledge/page 消费面先成熟。
- 收口长流程事件流的宿主消费要求：Provider / CodeBuddy 需要稳定消费 `progress`、usage、runtime summary、gate/readiness 以及真实生效的 `provider_direct / agent_bridge / deterministic_only` 执行路径，而不是自己猜测 workflow 正停在哪一层。
- 保持 Agent thin boundary：CodeBuddy 仍只做参数收集、binary 调用、结果解析和事件透传；新的“当前阶段使用方式”必须通过 `wiki-runtime` 合同表达，而不是把 Wiki 业务逻辑搬到 TS 层。
- **BREAKING**：当前阶段的宿主消费规范将不再允许把 `wiki-runtime` 当成“只有 action+term 的黑盒命令”；Provider / CodeBuddy 需要开始按正式 runtime profile 消费结果与状态，但 knowledge/page 的正式消费仍明确留到后续迭代。

## Capabilities

### New Capabilities
- `runtime-consumption-profile`: 定义 Provider / CodeBuddy 在当前阶段可稳定消费的 `wiki-runtime` profile，包括 readiness、推荐动作、index-first query 信号、长流程事件与执行路径信号。

### Modified Capabilities
- `repo-wiki-runtime`: 补齐当前阶段 `status / query / lifecycle` 输出的宿主可消费语义，使 runtime 不只是对内可用，而是对 Provider / CodeBuddy 可解释、可路由。
- `workflow-progress-streaming`: 补齐长流程事件流里对 runtime summary、usage、gate/readiness 与 provider-direct / agent-bridge 路径的宿主消费约束。
- `codebuddy-agent-integration`: 收口 CodeBuddy 如何薄消费当前阶段 `wiki-runtime`，包括 preflight、query、progress、provider bridge 与错误透传语义。

## Impact

- 重点影响 runtime 合同与输出面：[`crates/wiki-runtime/src/transport/dto.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/transport/dto.rs)、[`crates/wiki-runtime/src/transport/json_rpc.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/transport/json_rpc.rs)、[`crates/wiki-runtime/src/transport/cli.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/transport/cli.rs)、[`crates/wiki-runtime/src/workflows/status.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/workflows/status.rs)、[`crates/wiki-runtime/src/workflows/query.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/workflows/query.rs)、[`crates/wiki-runtime/src/domain/checkpoint.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/domain/checkpoint.rs)。
- 重点影响 CodeBuddy 宿主接入：[`agents/codebuddy/src/runtime/invokeCore.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/runtime/invokeCore.ts)、[`agents/codebuddy/src/runtime/parseResult.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/runtime/parseResult.ts)、[`agents/codebuddy/src/tools/wikiQuery.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/tools/wikiQuery.ts)、[`agents/codebuddy/src/tools/wikiStatus.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/tools/wikiStatus.ts)。
- 重点影响已有规范：[`openspec/specs/repo-wiki-runtime/spec.md`](E:/project/!byAI/spec-wiki/openspec/specs/repo-wiki-runtime/spec.md)、[`openspec/specs/workflow-progress-streaming/spec.md`](E:/project/!byAI/spec-wiki/openspec/specs/workflow-progress-streaming/spec.md)、[`openspec/specs/codebuddy-agent-integration/spec.md`](E:/project/!byAI/spec-wiki/openspec/specs/codebuddy-agent-integration/spec.md)。
- 本轮明确不收 knowledge/page 正式消费面，不提前替代后续 `wiki-knowledge`、page projection 或更完整的 AGENT/provider 消费迭代。
