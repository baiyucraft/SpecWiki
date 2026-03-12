# llm-budget-observability Specification

## Purpose
TBD - created by archiving change iteration-9-2-dossier-session-and-llm-budget-controls. Update Purpose after archive.
## Requirements
### Requirement: 系统必须支持按阶段配置的上下文预算与并行度
系统 MUST 为 `uncertainty_gate`、`page_enrichment` 和 research session 提供独立的上下文预算与并行度配置。系统 MUST 在真实发请求前先估算 prompt 大小，再决定压缩、裁剪或降级，而不是只依赖固定字符串截断。

#### Scenario: uncertainty gate 与 page enrichment 使用不同预算
- **WHEN** workflow 同时启用 `uncertainty_gate` 与 `page_enrichment`
- **THEN** 系统 MUST 允许分别为二者配置不同的最大输入 token 数与并行度
- **THEN** 系统不得只使用一个全局上限约束所有 prompt_type

#### Scenario: session 预算超限时压缩历史而不是无限扩张
- **WHEN** research session 的上下文接近或超过配置上限
- **THEN** 系统 MUST 优先压缩较早轮次为 summary，并保留最近有限轮和关键 tool artifact refs
- **THEN** 系统不得在超限后继续无限追加完整历史

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
- **WHEN** 某次 provider 或 agent-bridge 的真实 LLM 请求完成
- **THEN** 系统 MUST 在当前 workflow 的可观测输出中刷新 usage 累计值
- **THEN** 普通模式下调用方 MUST 能实时看到 token 成本增长

#### Scenario: provider 缺失 usage 时回退到本地估算
- **WHEN** provider 或 agent-bridge 未返回完整 usage 字段
- **THEN** 系统 MUST 回退到本地估算或等价安全策略
- **THEN** trace 或 summary MUST 标明该 usage 来源不是 provider 原始 usage

