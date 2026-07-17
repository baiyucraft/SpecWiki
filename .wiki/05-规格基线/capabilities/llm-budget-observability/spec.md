# llm-budget-observability Specification

## Purpose
定义 provider-backed Research/Compose 的调用预算、单次 `research_page` 轮次上限、cache mode 与 usage 可观测合同。
## Requirements
### Requirement: 系统必须支持 Research/Compose 调用预算与 request-local 轮次上限
系统 MUST 使用 `max_research_calls`、`max_compose_calls`、`parallel_requests` 与 `page_research_max_turns` 约束 provider-backed workflow。系统 MUST 在真实发请求前检查对应预算，不得无上限发起 provider 请求。

#### Scenario: Research 与 Compose 使用独立调用上限
- **WHEN** workflow 同时执行 Research 与 Compose
- **THEN** 系统 MUST 分别消费 `max_research_calls` 与 `max_compose_calls`
- **THEN** 任一阶段不得借用另一阶段预算绕过上限

#### Scenario: 单次 research_page 达到轮次上限
- **WHEN** request-local provider session 达到 `page_research_max_turns`
- **THEN** Runtime MUST 停止继续追加模型/tool turn，并按结构化结果或失败策略结束当前调用
- **THEN** 临时 summary、turns 和 tool refs MUST 在调用结束后丢弃，不得持久化

### Requirement: 系统必须支持可配置的 cache mode 与显式 cold-start
系统 MUST 让 LLM cache 生命周期与 runtime 清理解耦。`init` / `rebuild` 默认不得隐式清空 LLM cache，但系统 MUST 支持显式 cold-start，并允许通过配置或启动参数选择 `preserve`、`clear`、`refresh` 等 cache mode。

#### Scenario: 默认 init 保留 LLM cache
- **WHEN** 用户执行普通 `init` 或 `rebuild`，且未显式要求 cold-start
- **THEN** 系统 MUST 默认保留可复用的 LLM cache
- **THEN** runtime 目录清理不得顺手删掉全部 prompt cache

#### Scenario: 显式 cold-start 清空 LLM cache
- **WHEN** 用户通过配置或启动参数显式要求 cold-start
- **THEN** 系统 MUST 清空或绕过既有 LLM cache
- **THEN** 该次 workflow 的测试或报告 MUST 能明确标记为 cold run

### Requirement: 非 debug 模式下也必须实时输出 LLM usage 统计
系统 MUST 在普通 workflow 模式下实时输出 LLM usage 统计，而不是只在 debug trace 中可见。每次真实请求完成后，系统 MUST 更新累计 `request_count`、`input_tokens`、`output_tokens`、`total_tokens`，并允许按 `prompt_type`、provider 和 model 维度聚合。

#### Scenario: 每次真实请求完成后刷新 usage
- **WHEN** 某次 provider 请求，或未来已验证并实际启用的 agent-bridge 请求完成
- **THEN** 系统 MUST 在当前 workflow 的可观测输出中刷新 usage 累计值
- **THEN** 普通模式下调用方 MUST 能实时看到 token 成本增长

#### Scenario: provider 缺失 usage 时回退到本地估算
- **WHEN** provider 或未来已验证并实际启用的 agent-bridge 未返回完整 usage 字段
- **THEN** 系统 MUST 回退到本地估算或等价安全策略
- **THEN** trace 或 summary MUST 标明该 usage 来源不是 provider 原始 usage

