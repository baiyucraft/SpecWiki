## ADDED Requirements

### Requirement: evidence layer 必须支持 family-scoped provenance 与非代码来源
系统 MUST 让 evidence layer 支持 family-scoped provenance，并允许 docs anchors、public API surface、config surface 和 type surface 进入正式 evidence 对象。每条 evidence 除现有代码行段外，还 MUST 能标记其所属 family/page scope 和来源类别。

#### Scenario: family 页面引用 docs/API/config 证据
- **WHEN** family 页主要由 docs、API 或配置入口支撑
- **THEN** evidence layer MUST 能表达这些来源
- **THEN** 页面 evidence block MUST 保留可追溯的路径、锚点或 surface 标识

#### Scenario: family-scoped provenance 在重复生成时稳定
- **WHEN** 同一 family 页的 evidence 集合未变化
- **THEN** 对应 evidence identity 与 family scope MUST 保持稳定
- **THEN** runtime 不得因正文调整而重建无关的 evidence 身份
### Requirement: evidence layer 必须支持 section-scoped citation
系统 MUST 让 evidence layer 可以按 section 作用域被 compose / renderer 精确引用，而不是只按页面作用域附着。evidence 对象除 `page scope` 外，还 MUST 支持稳定的 `section refs` 与 `child digest refs`。

#### Scenario: section 只消费自己的 evidence
- **WHEN** compose 计划中某个 section 只命中了部分 evidence groups
- **THEN** renderer MUST 只在该 section 输出这些 evidence
- **THEN** 未命中的 section 不得自动复用同组 evidence block
