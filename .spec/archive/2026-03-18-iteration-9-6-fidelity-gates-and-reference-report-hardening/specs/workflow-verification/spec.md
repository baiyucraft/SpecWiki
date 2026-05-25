## ADDED Requirements

### Requirement: 9.6 的专项验收必须把 reuse、skeleton fidelity 与 key source coverage 纳入正式门槛
系统 MUST 将 `reuse`、`skeleton fidelity` 与 `key source coverage` 作为 `storybook + dagger` 专项验收的正式门槛，与 `overall_match_rate` 一起回答“页数是否接近 reference、是否存在 coarse page reuse、docs-backed 页面是否具备 reference 式骨架、正文是否真正覆盖关键文件”这四个问题。`overall_match_rate` MUST 仅作为一级门槛，不能单独决定通过。

#### Scenario: 9.6 专项报告生成最终门禁结论
- **WHEN** 系统生成 `storybook` 或 `dagger` 的 9.6 专项 reference 报告
- **THEN** 报告 MUST 同时给出 `overall_match_rate`、`reuse`、`skeleton fidelity` 与 `key source coverage`
- **THEN** 报告 MUST 明确回答页数、reuse、章节骨架与关键文件覆盖这四个专项问题
- **THEN** 系统 MUST 不得仅凭 `overall_match_rate >= 95%` 就判定通过

### Requirement: 9.6 的项目分析脚本必须消费 2.0 的 KnowledgeUnit/Research 数据面
系统 MUST 让项目分析脚本基于 `knowledge_units`、`knowledge_domains`、`research_cache`、`wiki_pages`、`page_digests` 等 2.0 runtime 数据进行统计，而不能继续依赖旧的 `page_context_cache.context.research_result`、`topic_dossier` 或旧 `page_type` 语义来判断 research 命中和页面类型。

#### Scenario: 项目分析脚本统计 storybook 或 dagger 的 research 命中
- **WHEN** 系统执行 `collect-test-project-analysis` 或等价项目分析脚本
- **THEN** 脚本 MUST 从 `knowledge_units / research_cache / wiki_pages` 读取研究与页面统计
- **THEN** 脚本 MUST 不得再因为旧 `PageContext` 字段缺失而把已完成的 research 统计为 0
- **THEN** 输出的页面类型与 decomposition 统计 MUST 与 2.0 的 KnowledgeUnit 主线一致

### Requirement: 9.6 的专项报告必须验证 warm report 稳定性
系统 MUST 对同一项目至少执行两次 warm report，并比较 `reuse_overage`、`median skeleton fidelity` 与 `median key source coverage` 的波动。若指标抖动超出允许范围，报告 MUST 标记该项目当前口径不稳定，不能作为后续迭代基线。

#### Scenario: warm report 重跑同一项目
- **WHEN** 系统对 `storybook` 或 `dagger` 连续执行至少两次 warm report
- **THEN** 报告 MUST 输出两次结果的 `reuse_overage`、`median skeleton fidelity` 与 `median key source coverage`
- **THEN** 报告 MUST 判断这些指标的波动是否处于允许范围
- **THEN** 若波动超出范围，系统 MUST 将该快照标记为不稳定，而不是直接作为 9.7-9.9 的验收基线
