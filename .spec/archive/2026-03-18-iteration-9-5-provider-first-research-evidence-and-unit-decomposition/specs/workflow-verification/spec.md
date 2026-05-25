## MODIFIED Requirements

### Requirement: reference 验证必须覆盖专题页、evidence 与图表达
系统 MUST 在 reference 项目验证中显式统计最终 `.wiki/*.md` 中的专题页覆盖率、citation / evidence 落页情况和图表达覆盖率，而不是只统计页面数、行数或 cache 中间态。逐项目报告 MUST 能指出哪些主题未被规划、哪些关键来源未落页、哪些页面仍然缺图，并明确这些统计来自最终 Markdown contract。

#### Scenario: 报告从最终 Markdown 读取 citation / evidence / mermaid
- **WHEN** 测试脚本为带 reference 的项目生成对比报告
- **THEN** 报告 MUST 直接从最终 `.wiki/*.md` 统计 citation、evidence block 和 Mermaid block
- **THEN** 报告不得只依赖 `page_drafts`、`page_context_cache` 或其它中间缓存表给出最终结论

### Requirement: reference 与项目集验证必须覆盖 family coverage 和页面折叠度
系统 MUST 在项目集分析和 reference 报告中把 family coverage 扩展为 `knowledge-unit decomposition` 命中指标，并继续统计 page collapse。报告 MUST 说明 docs/API/config/runtime/testing/example/tutorial 等单元类型是否进入正式页面，以及哪些 reference 页面仍被错误折叠进 overview 或大模块页。

#### Scenario: 报告输出 decomposition 命中与 page collapse
- **WHEN** 脚本对带 reference 的 docs-heavy 或 runtime-heavy 仓库生成报告
- **THEN** 报告 MUST 输出各类 KnowledgeUnit 的命中分布和 page collapse 指标
- **THEN** 报告 MUST 指出仍然被错误折叠的主要主题家族或知识单元

## ADDED Requirements

### Requirement: 9.5 的专项验证范围必须固定为 storybook 与 dagger
系统 MUST 将 `9.5` 的实现验证、reference 对比和 lifecycle 验收范围固定为 `storybook` 与 `dagger` 两个样本。当前 change 的通过条件 MUST 只依赖这两个样本的专项结果，不得再要求同步完成 19 项目集全量回归。

#### Scenario: 9.5 验收不再绑定全量项目集
- **WHEN** 系统执行 `9.5` 的测试、专项分析和验收报告生成
- **THEN** 验收范围 MUST 只包含 `storybook` 与 `dagger`
- **THEN** 报告不得把完整项目集回归结果当作本轮通过前置条件

### Requirement: 9.5 的专项验收必须把 storybook 与 dagger 都收敛到至少 95%
系统 MUST 把 `storybook` 与 `dagger` 对各自 reference 的总体对齐率收敛到 `>=95%`，并把这一结果建立在最终 `.wiki/*.md` 的对比结果上。系统 MUST 同时报告 `missing pages`、`collapsed pages` 与 `low-fidelity matched pages`，避免只靠增页堆高表面命中率。

#### Scenario: storybook 与 dagger 的总体对齐率都达到 95%
- **WHEN** 系统生成 `storybook` 与 `dagger` 的最终专项 reference 报告
- **THEN** 报告 MUST 明确给出两者各自的总体对齐率
- **THEN** `storybook` 的总体对齐率 MUST 大于等于 `95%`
- **THEN** `dagger` 的总体对齐率 MUST 大于等于 `95%`

#### Scenario: 95% 不得通过单纯增页获得
- **WHEN** 某轮专项收敛后总体对齐率提升
- **THEN** 报告 MUST 同时给出 `missing pages`、`collapsed pages`、`extra generated pages` 与 `low-fidelity matched pages`
- **THEN** 系统不得仅通过显著增加额外页面而忽略高频 collapse 或低质量命中问题

### Requirement: storybook 与 dagger 专项验证必须同时覆盖 provider、citation 与 decomposition 收敛
系统 MUST 在 `storybook` 与 `dagger` 专项验证中同时输出 provider-backed research 执行情况、最终 markdown citation / diagram 命中，以及 KnowledgeUnit decomposition 命中情况。专项报告 MUST 直接暴露“provider 已执行但 citation 未落页”或“citation 已落页但 unit 仍被错误折叠”这类链路断点。

#### Scenario: storybook 与 dagger 报告输出链路断点
- **WHEN** 系统为 `storybook` 或 `dagger` 生成专项 reference 报告
- **THEN** 报告 MUST 同时说明 provider research 是否生效、最终 citation/diagram 是否落页，以及 decomposition 是否命中目标主题
- **THEN** 报告 MUST 能明确指出主断点位于 provider、renderer 还是 planner

### Requirement: docs-backed 页面验证必须覆盖本地化命名优先级
系统 MUST 在 docs-heavy 样本验证中检查最终 `.wiki/` 的 docs-backed 页面是否优先采用本地化/派生 docs corpus 的命名与路径。若仓库同时存在英文 raw docs 与本地化/派生 docs 语料，专项报告 MUST 能指出英文 raw docs 文件名是否仍大批残留。

#### Scenario: 仓库同时存在英文 raw docs 与本地化/派生 docs 语料
- **WHEN** 系统为 `storybook` 或 `dagger` 生成最终专项报告
- **THEN** 报告 MUST 指出最终 `.wiki/` 是否仍大量保留英文 raw docs 文件名
- **THEN** 若本地化/派生 docs 语料已存在，系统 MUST 优先以该语料生成 docs-backed 页面路径与标题

### Requirement: docs-backed 页面验证必须覆盖主章节骨架对齐
系统 MUST 在 docs-heavy 样本验证中检查 docs-backed 页面是否稳定保留或收敛到 reference 的主章节骨架，而不是退回英文原始 docs 标题或泛化模板章节。

#### Scenario: reference 页面具有稳定主章节序列
- **WHEN** 某个 docs-backed 页面在 reference 中已经表现为稳定章节序列
- **THEN** 最终 `.wiki/*.md` MUST 优先保留或收敛到对应的主章节骨架
- **THEN** 报告 MUST 能指出该页是否偏离了 `cite / 目录 / 简介 / 项目结构 / 核心组件 / 架构总览 / 详细组件分析 / 依赖关系分析 / 性能考量 / 故障排查指南 / 结论 / 附录` 这类 reference 主结构
