## MODIFIED Requirements

### Requirement: 项目集验证必须覆盖增强后的页面信息密度与 graph 落地
每轮与迭代 9 相关的 tasks 设计、实现或测试时，系统 MUST 对 `.wiki/06-设计文档/00-总体设计.md § 测试项目集` 的完整项目集执行 `init` 分析，并在 `test-project-analysis.md` 中按项目输出增强后的页面信息密度、graph facts 是否进入页面正文、workflow/architecture 页面表现，以及与 reference 的差异。9.3 还 MUST 逐项目统计 `section_plan` 覆盖率、overview/architecture research 命中和精准 evidence 引用密度；验证可以按 deterministic baseline 与增强模式做对照，但不得只给总表结论。

#### Scenario: 项目集分析逐项目输出增强表现
- **WHEN** 迭代 9 的 tasks 设计或测试阶段执行完整项目集 `init` 分析
- **THEN** 报告 MUST 按项目逐个说明 overview、architecture、module、workflow 页的增强表现
- **THEN** 报告 MUST 说明 communities / processes / cycle warnings 是否真正进入正文，而不只是底层 state
- **THEN** 报告 MUST 额外说明 `section_plan` 覆盖率、overview/architecture research 命中和 evidence 引用密度

#### Scenario: 有 reference 的项目继续进行结构对照
- **WHEN** 测试项目存在 `tmp/reference/*`
- **THEN** 项目集分析 MUST 继续对照 `.wiki/*.md` 和 `wiki.metadata.json`
- **THEN** 报告 MUST 明确记录增强后页面结构与 reference 的差异或“无显著差异”

### Requirement: reference 验证必须覆盖专题页、evidence 与图表达
系统 MUST 在 reference 项目验证中显式统计专题页覆盖率、evidence 落页情况和图表达覆盖率，而不是只汇总页面数、行数或段落密度。逐项目报告 MUST 能指出哪些专题未被规划、哪些关键来源未落页、哪些页面仍然缺图；9.3 还 MUST 指出 archetype 高频专题、section-scoped citation 密度和 section-plan 驱动覆盖。

#### Scenario: 逐项目报告输出专题覆盖情况
- **WHEN** 测试脚本为带 reference 的项目生成对比报告
- **THEN** 报告 MUST 说明当前生成页覆盖了哪些专题类型
- **THEN** 报告 MUST 指出缺失的专题页类别或高频缺口
- **THEN** 报告 MUST 说明 archetype 高频专题是否进入正式页面集合

#### Scenario: 逐项目报告输出 evidence、citation 与图覆盖
- **WHEN** 测试脚本对比 generated 页面和 reference 页面
- **THEN** 报告 MUST 说明关键 evidence 是否进入正文
- **THEN** 报告 MUST 说明页面中的 section-scoped citation 密度
- **THEN** 报告 MUST 说明 Mermaid 或等价图表达的覆盖情况

### Requirement: 验证必须覆盖 dossier、provider research session 与 cold/warm 对照
系统 MUST 提供自动化测试和项目集验证，覆盖 targeted dossier snippets、child rollup 稳定性、provider bounded research session、tool schema、phase budget 裁剪和 cold/warm run 差异。测试报告 MUST 明确区分 cold run 与 warm run，而不是把两者混在同一结论里。CodeBuddy Agent 侧验证不属于 9.3 的必做范围。

#### Scenario: 测试区分 cold run 与 warm run
- **WHEN** 验证脚本对同一项目执行两轮启用 LLM 的 workflow
- **THEN** 报告 MUST 明确标记哪一轮是 cold run、哪一轮是 warm run
- **THEN** 报告 MUST 能说明 cache mode 与真实请求数的差异

#### Scenario: 验证 provider bounded research session 与 section-plan 结果
- **WHEN** 测试通过 provider-tools 执行 `overview`、`architecture`、`module` 或 `topic` 页 research session
- **THEN** 测试 MUST 观察到 session 事件、tool 调用和结构化 `PageResearchResult`
- **THEN** 测试 MUST 观察到最终页面仍由 deterministic renderer 落盘
- **THEN** 测试 MUST 观察到结果包含 `section_plan`、精准 snippet 引用和 line-span evidence
