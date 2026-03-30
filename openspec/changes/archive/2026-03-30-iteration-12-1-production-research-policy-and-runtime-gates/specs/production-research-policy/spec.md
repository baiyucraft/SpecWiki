## ADDED Requirements

### Requirement: 正式 workflow 必须以 provider-backed research 作为唯一成功语义
系统 MUST 将正式 `init`、`update`、`rebuild` 的 research 成功语义收敛到 provider-backed research。若当前 workflow 处于正式模式，则 `StructuralResearchProvider` MUST NOT 被视为正式成功路径，也不得继续作为默认 fallback。正式模式下只允许两类终态：provider-backed research 成功，或带 blocker/checkpoint 的显式失败。

#### Scenario: 正式 init 只能以 provider-backed research 成功
- **WHEN** 系统在正式模式下执行 `init`
- **THEN** workflow MUST 通过 provider-backed research 进入后续 compose
- **THEN** 系统 MUST NOT 因 provider 不可用而自动切换到 `StructuralResearchProvider` 并返回成功

#### Scenario: 正式 update 不能把 structural baseline 伪装成成功
- **WHEN** 系统在正式模式下执行 `update`
- **THEN** 若 provider research 未成功，workflow MUST 返回失败或 blocker 终态
- **THEN** 系统 MUST NOT 仅凭 structural baseline 或已有 page cache 就宣称本轮正式 update 成功

### Requirement: 结构型 research 只能用于测试、fixture 与显式开发模式
系统 MUST 把 `StructuralResearchProvider` 收敛为测试、fixture 和显式开发模式专用实现。显式开发模式的启用方式 MUST 可被配置和验证；未处于这些模式时，系统 MUST NOT 选择结构型 provider。

#### Scenario: 测试或 fixture 允许结构型 provider
- **WHEN** Rust 测试、fixture 或等价受控环境执行 workflow
- **THEN** 系统 MAY 使用 `StructuralResearchProvider`
- **THEN** 该路径 MUST NOT 被记为正式 production success

#### Scenario: 显式开发模式允许结构型 provider
- **WHEN** 用户通过显式开发模式配置运行 workflow
- **THEN** 系统 MAY 使用 `StructuralResearchProvider` 作为开发调试路径
- **THEN** 结果 MUST 被标记为开发模式，而不是正式 runtime ready

### Requirement: provider 不可用或 research 失败时必须留下 blocker 诊断
系统 MUST 在 provider 缺失、provider 不可用、provider research 失败或 provider 输出非法结构时留下可恢复的 blocker 诊断。该诊断 MUST 同时进入 workflow checkpoint 与 runtime gate/readiness 主链，以支持 `status`、恢复和专项报告复用。

#### Scenario: provider 缺失时写入 blocker 与 checkpoint
- **WHEN** 正式 workflow 启动后发现不存在可用 provider
- **THEN** 系统 MUST 写入 checkpoint，并记录 provider 缺失对应的 blocker
- **THEN** `status` MUST 能区分这是 blocker，而不是普通的 `needs_update`

#### Scenario: provider research 失败时保留失败上下文
- **WHEN** provider research 请求返回错误、超时或非法结构
- **THEN** 系统 MUST 保留失败目标、失败阶段与错误摘要
- **THEN** 后续 `status`、报告或恢复逻辑 MUST 能直接读取这些 blocker 线索

### Requirement: runtime 状态必须区分 `runtime_incomplete`、`blocker` 与 `needs_update`
系统 MUST 对外区分三类 knowledge runtime 状态：`needs_update`、`runtime_incomplete` 和 `blocker`。`needs_update` 表示 facts 或输入已变脏；`runtime_incomplete` 表示 facts 已就绪但 research/compose 尚未完成；`blocker` 表示正式 workflow 被 provider 缺失、provider 不可用或 provider failure 等硬问题阻断。系统 MUST NOT 再把这些状态折叠成统一的成功态或 `missing`。

#### Scenario: facts 已就绪但 compose 未完成时标记为 runtime_incomplete
- **WHEN** facts snapshot 已存在，且 workflow 已进入 research/compose，但尚未达到可交付终态
- **THEN** `status` MUST 返回 `runtime_incomplete`
- **THEN** 系统 MUST NOT 把该状态伪装成 ready 或 `needs_update`

#### Scenario: provider blocker 与源码变脏分离表达
- **WHEN** 仓库同时存在 provider blocker 和源码脏变更
- **THEN** 系统 MUST 能分别表达 blocker 与 `needs_update`
- **THEN** 调用方 MUST 不需要依赖日志文本猜测哪一个才是主阻断
