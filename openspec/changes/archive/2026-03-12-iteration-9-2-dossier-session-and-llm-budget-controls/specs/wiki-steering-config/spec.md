## ADDED Requirements

### Requirement: steering 与本地 dev 配置必须支持 phase-specific LLM budget 和 cache mode
系统 MUST 允许通过 `.wiki/wiki.steering.yaml` 与 repo 根 `wiki.dev.yaml` 配置 phase-specific LLM budget、并行度、session 限制和 cache mode。`wiki.dev.yaml` MUST 继续优先覆盖共享 steering 中对应的 `llm` 字段。

#### Scenario: dev 配置覆盖 phase-specific budget
- **WHEN** `wiki.dev.yaml` 中声明 `uncertainty_gate_max_input_tokens`、`page_enrichment_max_input_tokens` 或 `session_max_context_tokens`
- **THEN** 系统 MUST 以本地 dev 配置为准
- **THEN** 对应 workflow MUST 使用这些预算参与请求裁剪和降级

#### Scenario: 配置 cache mode 控制 cold/warm 行为
- **WHEN** steering 或 `wiki.dev.yaml` 中声明 `llm.cache_mode`
- **THEN** 系统 MUST 按 `preserve`、`clear`、`refresh` 等模式控制 LLM cache 生命周期
- **THEN** 对应 workflow 的 trace 或 summary MUST 能反映本次是 cold run 还是 warm run

### Requirement: 系统必须支持 `~/.spec-wiki/` 用户级配置与 learned state
系统 MUST 支持在用户目录下使用 `~/.spec-wiki/config.yaml` 作为用户级显式配置，并使用 `~/.spec-wiki/state.yaml` 保存 learned state。用户级显式配置 MUST 参与常规配置优先级解析；learned state MUST 只在 capability 配为 `auto` 时生效，不得覆盖用户显式声明的能力模式。

#### Scenario: 用户级配置作为 repo 外默认值
- **WHEN** repo 内没有显式覆盖对应 provider、budget 或 cache mode 配置
- **THEN** 系统 MUST 允许从 `~/.spec-wiki/config.yaml` 读取这些默认值
- **THEN** repo 级配置仍 MUST 能覆盖用户级默认值

#### Scenario: learned state 只在 auto 模式下生效
- **WHEN** 某个 provider capability 在 `~/.spec-wiki/state.yaml` 中存在 learned 记录
- **THEN** 只有当当前 capability 配置为 `auto` 时，系统才 MAY 使用该 learned 结果
- **THEN** 若用户显式声明 `native_tools`、`emulated_tools` 或 `no_tools`，learned state 不得覆盖该显式配置

#### Scenario: provider capability 降级结果回写到 learned state
- **WHEN** 系统明确观测到某个 provider/model 对 tools 等 capability 的“不支持”错误
- **THEN** 系统 MUST 允许把该降级结果按 `api_base + provider + model_id` 维度回写到 `~/.spec-wiki/state.yaml`
- **THEN** 后续同一 provider/model 请求在 TTL 未过期时不得重复探测相同 capability

#### Scenario: 非 capability 错误不得回写为持久降级
- **WHEN** 当前请求失败原因属于认证错误、限流、服务器错误、超时、网络异常或一次性解析失败
- **THEN** 系统 MUST 不得把这类结果回写为 provider capability 的持久 learned 降级
- **THEN** 后续请求不得因为这类瞬时错误而长期卡在错误的降级模式

#### Scenario: 用户级配置与 learned state 使用固定最小字段集
- **WHEN** 系统读取 `~/.spec-wiki/config.yaml` 或 `~/.spec-wiki/state.yaml`
- **THEN** `config.yaml` MUST 至少支持 provider、budget、parallelism、cache mode 和 session 开关这些字段
- **THEN** `state.yaml` MUST 至少支持 `api_base`、`provider/model`、`tools_mode`、`detected_at`、`reason`、`ttl_hours`

### Requirement: steering 与本地 dev 配置必须拆分阶段开关和并行度
系统 MUST 允许分别配置 `uncertainty_gate`、`content_enrichment` 和 research session 的开关及并行度，而不是只保留一个总开关或单个 `parallel_requests`。

#### Scenario: 单独关闭 uncertainty gate
- **WHEN** 用户在 steering 或 `wiki.dev.yaml` 中关闭 `uncertainty_gate`
- **THEN** 系统 MUST 跳过对应阶段
- **THEN** `content_enrichment` 或 research session 仍 MAY 继续执行

#### Scenario: 分别配置 gate 和 page/session 并行度
- **WHEN** 配置中声明 `uncertainty_gate_parallel_requests`、`page_enrichment_parallel_requests` 或 session 对应并行限制
- **THEN** 系统 MUST 分别在对应阶段使用这些上限
- **THEN** 系统不得把所有阶段都套用同一个并行值
