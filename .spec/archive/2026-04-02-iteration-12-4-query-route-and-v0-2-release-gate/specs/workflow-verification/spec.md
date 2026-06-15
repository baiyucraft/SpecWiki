## ADDED Requirements

### Requirement: `v0.2.0` 的专项验收必须以 knowledge runtime first-class release 为门禁
系统 MUST 在本 change 的专项验证中直接检查 `v0.2.0` 公开合同是否已经从 `index-only` 切换到 knowledge runtime first-class release，而不是继续把 facts/index 可查询当成通过。验证 MUST 同时回答：

- `init / update` 是否只在最小正式 knowledge runtime 形成后才算成功
- `query` 是否已经按 `index -> knowledge -> page fallback` 路由
- `provenance_summary` 是否稳定区分 `index_hit / knowledge_hit / page_fallback`
- `status` 是否不再投影 `index_only`
- `README`、帮助文本与命令合同测试是否已经切换到 `v0.2.0` 语义

#### Scenario: `storybook` 成为 `v0.2.0` primary gate
- **WHEN** 系统执行本 change 的专项验证
- **THEN** `storybook` MUST 作为 `v0.2.0` knowledge runtime first-class release 的 primary gate
- **THEN** 报告 MUST 明确指出它是否真正形成最小正式 knowledge runtime，而不是只证明 facts/index 可查询
- **THEN** `dagger` MAY 作为后续参考项单独观察，但 MUST NOT 再作为本轮强制收口条件

#### Scenario: `chi + zustand` 补充 release 广度验证
- **WHEN** 系统执行本 change 的 release gate
- **THEN** 除 `storybook` 外，系统 MUST 再对 `chi + zustand` 执行 smoke 验证
- **THEN** 报告 MUST 证明 `v0.2.0` 语义不是只在两个专项样本上成立

#### Scenario: 完整项目集 baseline guard 不再属于本轮强制验收
- **WHEN** 系统执行本 change 的测试收口
- **THEN** 系统 MUST NOT 把完整项目集 `init` baseline guard 作为 `12.4` 本轮强制通过条件
- **THEN** 若后续单独执行 `node scripts/run-test-projects.mjs`，报告 MUST 明确区分该结果只是参考观察面，而不是本轮 primary 或 smoke gate

#### Scenario: query route 通过稳定 route tags 可观测
- **WHEN** 验证脚本执行代表性的 query 断言
- **THEN** 验证 MUST 能观察到 `provenance_summary` 对 `index_hit`、`knowledge_hit` 与 `page_fallback` 的稳定区分
- **THEN** 验证 MUST 证明系统没有把 page fallback 伪装成 facts 或 knowledge 命中

### Requirement: 本 change 的验收必须证明 `v0.1.0 index-only` 公开合同已经移除
系统 MUST 通过自动化测试、脚本验证或等价方式证明 `v0.1.0 index-only` 的公开合同已经被移除。验证 MUST 同时覆盖 runtime 行为、命令合同与公开文档，而不是只改一层。

#### Scenario: runtime 行为不再以 facts/index-only 作为成功初始化
- **WHEN** 验证脚本对目标仓库执行 `init` 或 `update`
- **THEN** 测试 MUST 观察到系统不会在仅形成 facts/index snapshot 时把本次 workflow 判为公开成功
- **THEN** 若 workflow 仍处于 `runtime_incomplete` 或等价诊断态，报告 MUST 明确记录该状态而不是把它算作通过

#### Scenario: README 与帮助文本不再宣传 `v0.1.0 index-only`
- **WHEN** 系统执行本 change 的合同验证
- **THEN** 验证 MUST 检查 `README`、帮助文本或等价公开入口中不再把 `v0.1.0 index-only` 描述为当前正式发布合同
- **THEN** 验证 MUST 记录 `v0.2.0 knowledge runtime` 语义已经与 runtime 实现保持一致

#### Scenario: 本 change 继续执行注释合规检查
- **WHEN** 本 change 进入测试收口阶段
- **THEN** 系统 MUST 单独执行一次 `.wiki/02-开发指南/00-代码注释规范.md` 合规检查
- **THEN** 该检查结果 MUST 进入本 change 的验收记录
