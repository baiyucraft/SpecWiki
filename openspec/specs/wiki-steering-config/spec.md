# wiki-steering-config Specification

## Purpose
定义 Repo Wiki steering 配置的 schema、默认值、本地 dev 覆盖关系与运行时消费边界。
## Requirements
### Requirement: 系统必须支持 repo 级 steering 配置文件
系统 MUST 支持从 `.wiki/wiki.steering.yaml` 读取 steering 配置。配置文件 MUST 使用 `version` 字段标识 schema 版本，并支持嵌套的 `scan` 配置块。`scan` 块 MUST 至少支持 `ignore` 和 `include` 两个字段：`scan.ignore` 用于追加忽略路径，`scan.include` 用于显式恢复原本会被忽略的路径。当配置文件不存在时，系统 MUST 使用合理默认值正常运行，不得阻塞 pipeline 或输出 warning。

#### Scenario: 读取存在的 steering 配置
- **WHEN** `.wiki/wiki.steering.yaml` 存在且格式合法
- **THEN** 系统 MUST 解析配置并在 scanner 与 planner 阶段消费
- **THEN** 配置中的 `scan.ignore` 和 `scan.include` 字段 MUST 按 schema 定义生效

#### Scenario: steering 配置不存在
- **WHEN** `.wiki/wiki.steering.yaml` 不存在
- **THEN** 系统 MUST 使用默认值（空忽略列表、空 include 列表、无提升/降级、默认合并阈值）
- **THEN** pipeline MUST 正常完成，行为与未引入 steering 时一致

#### Scenario: steering 配置格式非法
- **WHEN** `.wiki/wiki.steering.yaml` 存在但格式不合法（YAML 语法错误或字段类型不匹配）
- **THEN** 系统 MUST 输出明确的解析错误信息
- **THEN** 系统 MUST 回退到默认值继续运行，不得中断 pipeline

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

## REMOVED Requirements

### Requirement: steering 配置必须支持全局和按语言两层忽略路径
**Reason**: `DESIGN-CORE.md` 已将扫描侧 steering 结构收敛到 `scan.ignore` / `scan.include`，旧的 `ignore.global` 与 `ignore.<language>` 结构不再是长期事实模型。
**Migration**: 现有仓库应把旧的 `ignore.global` / `ignore.<language>` 配置迁移为 `scan.ignore`，需要强制纳入扫描的路径迁移为 `scan.include`。兼容读取仅作为过渡行为，不构成长期 contract。

## ADDED Requirements

### Requirement: steering 配置必须支持 scan.ignore 与 scan.include
系统 MUST 允许用户通过 steering 配置的 `scan.ignore` 与 `scan.include` 声明额外的扫描边界。`scan.ignore` MUST 追加到 scanner 内置忽略规则之上；`scan.include` MUST 作为白名单覆盖同轮扫描中的忽略结果。两者 MUST 在扫描阶段生效，并被 `init`、`update`、`sync`、`rebuild` 统一消费。

#### Scenario: 配置 scan.ignore
- **WHEN** steering 配置中声明 `scan.ignore: [\"docs/**\", \"examples/**\"]`
- **THEN** scanner MUST 在内置排除规则之外额外排除匹配这些 glob 的文件和目录
- **THEN** 被排除的文件不得出现在 `ScanReport.files` 中

#### Scenario: 配置 scan.include 恢复被忽略路径
- **WHEN** 某路径同时命中内置忽略规则或 `scan.ignore`
- **AND** steering 配置中声明 `scan.include` 包含该路径
- **THEN** scanner MUST 恢复该路径下的文件参与扫描
- **THEN** 这些文件 MUST 正常出现在 `ScanReport.files` 中

#### Scenario: init 与 update 共享同一 steering 扫描边界
- **WHEN** 用户修改 `scan.ignore` 或 `scan.include` 后执行 `init`、`update`、`sync` 或 `rebuild`
- **THEN** 系统 MUST 在该次 workflow 中按新配置重新执行扫描边界判断
- **THEN** 新旧扫描边界差异 MUST 能反映到后续的模块树、页面规划和 change_set 结果中

### Requirement: steering 配置必须支持 LLM 增强控制项与页面提示
系统 MUST 允许用户通过 `.wiki/wiki.steering.yaml` 的 `llm` 配置块控制 LLM 增强行为。`llm` 配置块 MUST 至少支持是否启用内容增强、是否启用 Uncertainty Gate、单次 workflow 的最大真实调用次数，以及与图生成相关的开关或模式。系统还 MUST 允许通过 `pages.hints` 为不同页面类型追加提示语，并把这些提示作为页面增强输入的一部分。

#### Scenario: steering 显式关闭 LLM 增强
- **WHEN** steering 配置中声明关闭 LLM 内容增强或 Uncertainty Gate
- **THEN** 系统 MUST 跳过对应 LLM 阶段
- **THEN** workflow MUST 回退到纯 deterministic 行为，而不是尝试隐式调用 LLM

#### Scenario: steering 限制单次 workflow 的真实调用次数
- **WHEN** steering 配置中声明了最大真实调用次数
- **THEN** 系统 MUST 在达到该上限后停止发起新的真实 LLM 请求
- **THEN** 后续待处理项 MUST 回退到 deterministic 结果或缓存结果

#### Scenario: steering 配置 provider 直连并行度
- **WHEN** steering 或 `wiki.dev.yaml` 中声明了 `llm.parallel_requests`
- **THEN** 系统 MUST 仅把该值用作 provider 直连路径下的同层页面增强并行上限
- **THEN** 当值缺失、非法或小于 `1` 时，系统 MUST 回退到安全默认值而不是创建无上限并发

#### Scenario: pages.hints 参与页面增强输入
- **WHEN** steering 配置中为 `overview`、`architecture`、`module` 或 `workflow` 页面声明了提示项
- **THEN** 系统 MUST 把这些提示作为对应页面增强输入的一部分
- **THEN** 这些提示不得直接绕过事实层或改写页面身份

### Requirement: 本地 dev 配置必须允许覆盖 provider 直连参数
系统 MUST 允许通过 repo 根的 `wiki.dev.yaml` 为本地开发环境覆盖 LLM provider 直连参数。provider 配置 MUST 采用 `llm.providers.<provider>.models.<model>` 的两级结构，顶层 `llm.model` MUST 使用 `provider/model` 选择具体模型。`wiki.dev.yaml` 的加载优先级 MUST 高于 `.wiki/wiki.steering.yaml` 中对应的 `llm` 字段，但它只服务本地 dev 调试，不得被写入 runtime 页面、metadata 或 `.wiki/.cache` 之外的正式产物。`wiki.dev.yaml` 不得放在 `.wiki/` 目录下，以避免被 init/rebuild 的 runtime 清理删除。

#### Scenario: 本地 dev 配置覆盖共享 steering 的 provider 字段
- **WHEN** `.wiki/wiki.steering.yaml` 未声明 provider 直连参数，但 repo 根存在 `wiki.dev.yaml`
- **THEN** 系统 MUST 从 `wiki.dev.yaml` 读取并覆盖对应的 `llm.providers.*` 与 `llm.model` 配置
- **THEN** workflow MUST 使用覆盖后的本地 provider 配置参与 LLM 选择逻辑

#### Scenario: dev 配置缺失时保持共享 steering 行为
- **WHEN** repo 根不存在 `wiki.dev.yaml`
- **THEN** 系统 MUST 只使用 `.wiki/wiki.steering.yaml` 和默认值
- **THEN** workflow 行为不得因为缺少 dev 文件而报错

#### Scenario: 顶层 model 使用 provider/model 选择具体 provider 模型
- **WHEN** steering 或 `wiki.dev.yaml` 中声明 `llm.model = "proxy/gpt-5-mini"`
- **THEN** 系统 MUST 先解析出 `provider = proxy`、`model = gpt-5-mini`
- **THEN** 只有当 `llm.providers.proxy.models.gpt-5-mini` 存在时，core 才 MAY 直连该 provider
