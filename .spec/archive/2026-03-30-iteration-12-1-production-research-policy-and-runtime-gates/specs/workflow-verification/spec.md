## MODIFIED Requirements

### Requirement: 验证必须覆盖 LLM 开关、缓存命中与桥接回退
系统 MUST 提供自动化测试，验证在 LLM 关闭、显式开发模式、provider 直连可用和 provider/blocker 失败四种典型路径下，`init`、`update` 和 `rebuild` 都能保持正确的 production policy 语义。验证 MUST 同时覆盖 core 直接执行和宿主桥接执行，不得只测 happy path。对于正式模式，系统 MUST 不再把 Agent bridge 不可用或 provider 不可用视为 deterministic/structural success。

#### Scenario: 显式开发模式允许结构型 provider
- **WHEN** 测试在显式开发模式下执行启用 research 的 workflow
- **THEN** 测试 MAY 观察到 workflow 使用结构型 provider 完成调试路径
- **THEN** 报告 MUST 明确该结果属于开发模式，而不是正式 runtime success

#### Scenario: provider 直连优先于 Agent bridge
- **WHEN** 测试同时提供可用的 provider 直连配置和 Agent bridge
- **THEN** 测试 MUST 观察到 core 优先使用 provider 直连
- **THEN** Agent bridge 不得收到同一请求对应的 `llm_request`

#### Scenario: 正式模式下 provider 不可用时返回 blocker
- **WHEN** 测试以正式模式执行启用 research 的 workflow，但不存在可用 provider
- **THEN** 测试 MUST 观察到 workflow 返回显式失败、checkpoint 或 blocker 终态
- **THEN** 测试 MUST NOT 观察到系统自动回退为 deterministic/structural success

#### Scenario: provider 失败时同时留下 checkpoint 与 gate
- **WHEN** 测试在正式模式下让 provider research 请求报错、超时或返回非法结构
- **THEN** 测试 MUST 观察到 `pipeline_checkpoint` 被写入
- **THEN** 测试 MUST 观察到 runtime gate/blocker 摘要可被 `status` 或报告脚本读取

### Requirement: 本 change 的专项验证必须覆盖 storybook 的 production research policy 和 runtime gates
系统 MUST 在本 change 的专项验证中验证 `storybook` 的 production research policy、runtime readiness 和 blocker 诊断，而不是继续只看最终 Markdown 匹配率或单样本通过。验证 MUST 直接读取 `status`、runtime SQLite 状态、checkpoint、gate/blocker 摘要与专项报告，确认正式模式下是否真实执行 provider-backed research、provider 失败时是否转为显式 blocker、以及 `runtime_incomplete / blocker / needs_update` 是否能被稳定区分。`dagger` MAY 作为观察样本记录，但 MUST NOT 成为本 change 的专项输出要求。

#### Scenario: storybook 专项验证 production research policy
- **WHEN** 系统对 `storybook` 运行本 change 的专项验证
- **THEN** 报告 MUST 指出正式 workflow 是否真实走了 provider-backed research
- **THEN** 若 provider 路径失败，报告 MUST 输出对应 checkpoint 与 blocker 摘要
- **THEN** 系统 MUST NOT 仅凭页面数量或最终 query 成功判断通过

#### Scenario: 本 change 只输出 storybook 专项结论
- **WHEN** 系统生成本 change 的专项验证结果
- **THEN** 报告 MUST 只输出 `storybook` 的 runtime 状态、blocker hint 和 recommended action
- **THEN** 报告 MUST 不得把这类诊断留在非结构化日志中
- **THEN** 若记录 `dagger` 观察结果，系统 MUST 明确标记其不属于本 change 的正式专项结论
