## MODIFIED Requirements

### Requirement: 系统必须提供端到端验证适配层到 Wiki 产物的主链路
系统 MUST 提供自动化测试，验证适配层调用 Rust core 后能够在目标仓库完成 Wiki 初始化、手工编辑同步、增量更新、强制重建并执行查询，且生成的 `.wiki/` 产物与返回结果符合预期。针对迭代 5 收口，验证 MUST 覆盖 managed marker 写盘、legacy 页面迁移、`sync` 回写 section / summary / metadata、`update / rebuild` 保留同页 user sections，以及测试项目集 `init` 分析。每轮与迭代 5 相关的 tasks 设计、实现或测试时，还 MUST 对 `DESIGN.md § 测试项目集` 的完整项目集执行 `init` 分析；如果目标仓库存在 reference，则必须对照 `.wiki/*.md` 与 `wiki.metadata.json`。

#### Scenario: 初始化后页面包含 managed marker
- **WHEN** 端到端或集成测试在临时仓库中执行 `init`
- **THEN** 测试必须观察到 `.wiki/` 目录和 `wiki.metadata.json` 已生成
- **THEN** 测试必须观察到页面中的 runtime 托管区段带有 managed marker

#### Scenario: 手工新增区段后执行 sync
- **WHEN** 测试在初始化后向页面插入新的用户区段并执行 `sync`
- **THEN** 测试必须观察到对应页面被加入 `synced_pages`
- **THEN** 测试必须观察到 WikiState 中新增了 `managed = false` 的 section 状态
- **THEN** 测试必须观察到 metadata summary 或 page content hash 已被刷新

#### Scenario: 手工区段在 update 后仍被保留
- **WHEN** 测试在 `sync` 之后修改相关源码并执行 `update`
- **THEN** 测试必须观察到 `updated_pages` 只包含受影响页面
- **THEN** 测试必须观察到同页 user section 在最终 Markdown 中仍然存在
- **THEN** 测试必须观察到未受影响页面未被无条件重写

#### Scenario: legacy 页面迁移
- **WHEN** 测试准备一份无 managed marker 的 legacy 页面并执行 `sync` 或 `update`
- **THEN** 测试必须观察到系统能够按已知 section 标题恢复 managed/user section 边界
- **THEN** 若迁移成功，后续 runtime 写盘必须输出带 marker 的页面格式

#### Scenario: rebuild 保留同页 user section
- **WHEN** 测试在同步手工区段后执行 `rebuild`
- **THEN** 测试必须观察到系统重新生成 managed sections
- **THEN** 测试必须观察到仍然存在的同一 `page_id` 页面保留了原有 user sections

#### Scenario: 对照测试项目集与参考产物
- **WHEN** 开发者为迭代 5 相关 change 设计 tasks、执行实现或做回归验证
- **THEN** 系统必须能够对 `DESIGN.md § 测试项目集` 的完整项目集执行 `init`
- **THEN** 开发者必须固定分析 `E:\\project\\aLocal` 和 `E:\\project\\!byAI\\spec-wiki`，且不得省略其余样本仓库
- **THEN** 如果目标仓库存在 reference 参考，则必须将生成结果与 reference 的页面结构和 metadata 字段做对照
