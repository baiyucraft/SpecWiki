## ADDED Requirements

### Requirement: reference 验证必须覆盖专题页、evidence 与图表达
系统 MUST 在 reference 项目验证中显式统计专题页覆盖率、evidence 落页情况和图表达覆盖率，而不是只汇总页面数、行数或段落密度。逐项目报告 MUST 能指出哪些专题未被规划、哪些关键来源未落页、哪些页面仍然缺图。

#### Scenario: 逐项目报告输出专题覆盖情况
- **WHEN** 测试脚本为带 reference 的项目生成对比报告
- **THEN** 报告 MUST 说明当前生成页覆盖了哪些专题类型
- **THEN** 报告 MUST 指出缺失的专题页类别或高频缺口

#### Scenario: 逐项目报告输出 evidence 与图覆盖
- **WHEN** 测试脚本对比 generated 页面和 reference 页面
- **THEN** 报告 MUST 说明关键 evidence 是否进入正文
- **THEN** 报告 MUST 说明 Mermaid 或等价图表达的覆盖情况
