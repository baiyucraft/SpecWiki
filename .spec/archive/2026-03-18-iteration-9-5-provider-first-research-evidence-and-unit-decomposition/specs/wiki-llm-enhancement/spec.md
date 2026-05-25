## MODIFIED Requirements

### Requirement: Research 和 Compose 层必须接入 LLM，不支持退化
系统 MUST 要求正式 runtime 的 Research 层（R1 system / R2 domain / R3 unit）和 Compose 层全部通过 provider-backed LLM 生成结构化研究材料和页面内容。系统 MUST NOT 让 `StructuralResearchProvider` 继续作为正式 `init / update / rebuild` 的默认实现。结构型 provider 仅 MAY 在测试、fixture 或显式开发模式下使用。若正式 runtime 无可用 provider，pipeline MUST 返回错误并保存检查点，而不是退化到模板填充。

#### Scenario: 正式 workflow 使用 provider-backed LLM
- **WHEN** 系统执行正式的 `init`、`update` 或 `rebuild`
- **THEN** Research 与 Compose 阶段 MUST 使用 provider-backed LLM
- **THEN** 正式 workflow 不得默认绑定 `StructuralResearchProvider`

#### Scenario: 测试或显式开发模式允许结构型 provider
- **WHEN** 系统处于测试、fixture 或显式开发调试模式
- **THEN** workflow MAY 使用 `StructuralResearchProvider`
- **THEN** 该路径不得被当作正式 runtime 成功口径

## ADDED Requirements

### Requirement: provider 选择必须在 workflow 入口统一决策
系统 MUST 在 workflow 入口统一选择正式 runtime 使用的 provider-backed `ResearchProvider / ComposeProvider`，并把该选择传入整条 `run_compose_pipeline()`。provider 选择、cache key、checkpoint 与 debug trace 语义 MUST 对 `init / update / rebuild` 保持一致。

#### Scenario: init / update / rebuild 共享同一 provider 选择语义
- **WHEN** 用户分别执行 `init`、`update` 和 `rebuild`
- **THEN** 这三个 workflow MUST 通过统一的 provider 选择逻辑决定 runtime provider
- **THEN** 系统不得在某个 workflow 中偷偷回退到结构型 provider 而其它 workflow 不回退
