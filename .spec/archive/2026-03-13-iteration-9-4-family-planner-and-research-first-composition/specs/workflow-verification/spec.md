## ADDED Requirements

### Requirement: reference 与项目集验证必须覆盖 family coverage 和页面折叠度
系统 MUST 在项目集分析和 reference 报告中新增 family coverage、family index/child 数量、API/config/docs 命中率和页面折叠度指标。对 `storybook` 等 docs-heavy/platform 仓库，验证 MUST 能指出哪些 reference 页面仍被错误折叠进少数 overview/module/topic 页。

#### Scenario: reference 报告输出 family coverage 与 page collapse
- **WHEN** 脚本对带 reference 的 docs-heavy 或 platform 仓库生成报告
- **THEN** 报告 MUST 说明 family index/child 覆盖情况
- **THEN** 报告 MUST 说明 `pageCollapseRatio` 或等价折叠指标
- **THEN** 报告 MUST 指出仍然被错误折叠的主要页面家族

#### Scenario: 项目集分析输出 API/config/docs 命中率
- **WHEN** 运行完整项目集 `init` 分析
- **THEN** 报告 MUST 逐项目说明 API surface、config surface 和 docs anchors 是否进入正式页面
- **THEN** 对无 reference 项目，报告也 MUST 明确输出这些命中指标，而不是只写“无 reference”
### Requirement: storybook 专项验证必须覆盖 leaf doc 与 section citation 收敛
系统 MUST 在 `storybook` 专项验证中额外输出 `family-leaf-doc` 覆盖、leaf-first compose 命中和 section-scoped citation 命中情况，并把它们作为 9.4 后半程的核心验收指标。

#### Scenario: storybook 报告输出 leaf doc 覆盖与 section citation
- **WHEN** 系统对 `storybook` 生成专项 reference 报告
- **THEN** 报告 MUST 说明新增 leaf doc 页面数量与仍然折叠的主要 family
- **THEN** 报告 MUST 说明 citation 是否已从页面级附属块转为 section-scoped 命中

### Requirement: dagger 专项验证必须覆盖 runtime-heavy / compiler-heavy 的知识域拆分
系统 MUST 在 `dagger` 专项验证中输出 `CoreRuntime / Framework / PlatformBinding / CompilerToolchain / ApiReference / TestingInfra / ConceptGuide` 等知识域的发现情况，并记录 API/测试/教程页是否仍被折叠回大模块页。

#### Scenario: dagger 报告输出知识域发现与缺页分布
- **WHEN** 系统对 `dagger` 生成专项 reference 报告
- **THEN** 报告 MUST 说明 runtime-heavy / compiler-heavy 相关知识域是否被稳定发现
- **THEN** 报告 MUST 说明缺失 reference 页面主要集中在哪些 API / 框架 / 测试 / 教程主题

#### Scenario: storybook 与 dagger 的 lifecycle 验证都必须通过
- **WHEN** 运行 `storybook + dagger` 的 lifecycle 脚本
- **THEN** 两个样本的 `init → status → sync → update → rebuild` 断言都 MUST 通过
- **THEN** 报告 MUST 明确记录 token 统计、符号图稳定性和 parent rebuild 传播结果

