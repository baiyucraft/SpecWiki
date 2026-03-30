## MODIFIED Requirements

### Requirement: 页面生成必须采用 research-first compose，而不是模板后补
系统 MUST 在正式 renderer 落盘之前，先为 `system / domain / unit` 三级研究对象生成 provider-backed research 结果，再由 compose 计划生成正式页面。正式 `init / update / rebuild` 的 runtime MUST 使用 provider-backed `ResearchProvider`；`StructuralResearchProvider` 仅 MAY 用于测试、fixture 和显式开发调试场景，不得继续作为正式 runtime 的默认成功路径。

#### Scenario: 正式 workflow 先形成 provider-backed research 结果再落盘
- **WHEN** 系统执行正式的 `init`、`update` 或 `rebuild`
- **THEN** workflow MUST 先生成 provider-backed 的 `SystemResearch / DomainResearch / UnitResearch`
- **THEN** renderer MUST 依据这些 research 结果驱动 compose，而不是只把 summary 插入固定模板

#### Scenario: 正式 runtime 不再接受默认 structural provider
- **WHEN** 系统执行正式 workflow
- **THEN** workflow MUST NOT 默认选择 `StructuralResearchProvider`
- **THEN** 若 provider 不可用，系统 MUST 进入显式失败或 blocker 语义，而不是把 structural baseline 当成成功

#### Scenario: 结构型 provider 只保留给测试或显式开发模式
- **WHEN** 系统处于测试、fixture 或显式开发调试模式
- **THEN** workflow MAY 使用 `StructuralResearchProvider`
- **THEN** 该路径 MUST NOT 被当作正式 production ready

### Requirement: compose 层不支持 deterministic fallback
系统 MUST NOT 在 compose 阶段提供退化到模板填充的正式 runtime 路径。Research 与 Compose 的正式 workflow 输出 MUST 由 provider-backed LLM 生成；若 provider 不可用、provider 失败且当前不是测试/fixture/显式开发模式，workflow MUST 失败并保存 checkpoint，同时留下 gate / blocker 诊断，而不是回退为低质量模板输出。

#### Scenario: 正式 runtime 无 provider 时拒绝退化执行
- **WHEN** 正式 workflow 进入 Research 或 Compose 阶段但不存在可用 provider
- **THEN** pipeline MUST 返回错误并保存 checkpoint
- **THEN** 系统 MUST 同时记录对应的 runtime blocker
- **THEN** 系统 MUST NOT 使用模板填充继续生成页面

#### Scenario: provider research 失败时停止正式 compose
- **WHEN** provider research 请求超时、报错或返回非法结构
- **THEN** workflow MUST 停止正式 compose
- **THEN** 系统 MUST 保留失败阶段、失败目标和错误摘要
- **THEN** 系统 MUST NOT 回退到 deterministic compose 伪装成功
