## ADDED Requirements

### Requirement: 迭代 10 验证必须证明四 crate 边界成立且 workflow 不回退
系统 MUST 在 `迭代 10` 的验证中同时覆盖多 crate workspace build/test、crate 边界 smoke、正式 workflow smoke 与命名迁移面。通过条件 MUST 建立在“边界成立且 workflow 不回退”上，而不是页面质量提升或 runtime lifecycle 最终定型。

#### Scenario: 工作区验证四 crate 构建与边界
- **WHEN** 系统执行本 change 的 Rust/workspace 验证
- **THEN** 验证 MUST 观察到 `wiki-model`、`wiki-index`、`wiki-knowledge`、`wiki-runtime` 能完成 workspace build/test
- **THEN** 验证 MUST 检查 `wiki-index` 与 `wiki-knowledge` 不反向 import `wiki-runtime`
- **THEN** 验证 MUST 检查 `wiki-runtime` 不重新导出 `wiki-index` 或 `wiki-knowledge` 的内部实现

#### Scenario: 项目集与生命周期验证不回退
- **WHEN** 系统执行本 change 的脚本级验证
- **THEN** 系统 MUST 运行 `node scripts/test-wiki-lifecycle.mjs`
- **THEN** `storybook + dagger` MUST 继续作为专项守门样本，证明拆层没有让 runtime/workflow 进一步退化
- **THEN** 系统 MUST 单独执行一轮 `.wiki/02-开发指南/00-代码注释规范.md` 合规检查
