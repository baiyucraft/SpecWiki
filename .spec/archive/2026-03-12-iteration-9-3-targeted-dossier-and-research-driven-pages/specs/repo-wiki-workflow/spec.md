## MODIFIED Requirements

### Requirement: workflow 主链必须在正式渲染前组装 dossier 并按需执行 bounded research session
系统 MUST 在保持 deterministic 主链的前提下，在正式渲染前组装 dossier，并按页面类型决定是否执行 bounded research session。research session 的输入 MUST 来自 dossier、targeted snippets、child rollup 和显式 session state，而不是绕过主链重新扫描仓库。9.3 要求 `overview`、`architecture`、`module` 和 `topic` 页都可进入 bounded research session；`workflow` 页继续保持 deterministic + facts-driven diagram。

#### Scenario: overview、architecture、module 或 topic 页在 render 前执行 research session
- **WHEN** 用户执行 `init`、`update` 或 `rebuild`，且当前页面为 `overview`、`architecture`、`module` 或 `topic`
- **THEN** 系统 MUST 先完成 dossier 组装
- **THEN** 若 research session 开启，系统 MUST 在 render 前执行该 session 并消费结构化结果

#### Scenario: update 只重建受影响 dossier、section plan 与父页 rollup
- **WHEN** 变化范围只影响部分 dossier、child rollup、section plan 或 session 结果
- **THEN** 系统 MUST 只重建这些页面及其受影响父页
- **THEN** 未受影响页面不得因为 dossier/session 引入而被无谓重写

### Requirement: `init`、`update` 与 `rebuild` 必须支持可回退的 LLM 增强阶段
系统 MUST 在保持现有 deterministic facts 主链的前提下，为 `init`、`update` 和 `rebuild` 增加可选的 `llm_uncertainty_gate` 与 `llm_enrichment` 阶段。`llm_uncertainty_gate` MUST 发生在 scanner / hierarchy / 低置信度依赖语义判定期间；`llm_enrichment` MUST 发生在 page context 已稳定、正式写盘之前。无论任一阶段是否启用、命中缓存或回退，workflow 的最终写盘结果都 MUST 保持可追溯且可落回 deterministic 内容。

#### Scenario: init 在 facts 稳定后执行 research-driven 页面增强
- **WHEN** 用户执行 `init`，且当前运行环境已协商开启 LLM 增强
- **THEN** 系统 MUST 先完成 deterministic 的扫描、symbol graph、module tree、page planning 和 page context 构建
- **THEN** 系统 MUST 在正式渲染和写盘前对 `overview`、`architecture`、`module` 和 `topic` 页执行 research-driven `llm_enrichment`
- **THEN** 若增强阶段失败，系统 MUST 回退到 deterministic 页面内容继续完成 `init`

#### Scenario: rebuild 在 research 不可用时保持 deterministic
- **WHEN** 用户执行 `rebuild`，但当前 provider 不可用、预算关闭或 research contract 校验失败
- **THEN** 系统 MUST 跳过 research-driven 结果并走完整 deterministic 路径
- **THEN** rebuild 的最终状态与现有 deterministic 语义保持一致
