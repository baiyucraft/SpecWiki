## ADDED Requirements

### Requirement: `v0.2.0` 发布 gate 必须收口为固定 release evidence 集合
系统 MUST 将当前版本的发布 gate 收口为固定的 release evidence 集合，而不是每轮临时拼装放行条件。对 `v0.2.0` 而言，正式 release evidence MUST 至少覆盖 `packages/spec-wiki` 自动化测试、`storybook` primary gate、`chi + zustand` smoke，以及 `.wiki/02-开发指南/00-代码注释规范.md` 检查。系统 MUST NOT 将 `dagger`、19 项目全量回归或额外观察样本重新提升为本轮强制 release blocker。

#### Scenario: 生成 `v0.2.0` release evidence
- **WHEN** 系统为当前版本生成发布前验证记录
- **THEN** 记录 MUST 明确包含 `packages/spec-wiki` 测试、`storybook` primary gate、`chi + zustand` smoke 与 `.wiki/02-开发指南/00-代码注释规范.md` 检查
- **THEN** 记录 MUST 区分正式 release gate 与观察性样本，不得混淆

#### Scenario: 观察性样本不得被伪装为强制 blocker
- **WHEN** 当前版本额外执行 `dagger`、19 项目全量回归或其它观察性样本
- **THEN** 报告 MUST 明确标注这些结果属于参考观察或 baseline guard
- **THEN** 系统 MUST NOT 把它们回写成 `v0.2.0` 的正式放行前置条件
