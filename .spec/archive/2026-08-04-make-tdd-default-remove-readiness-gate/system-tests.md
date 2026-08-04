# make-tdd-default-remove-readiness-gate 系统测试

## 测试环境

- runtime/platform：Node.js >=20.19、Windows/PowerShell 或 CI
- fixture/data：临时 Codex 项目、zh/en package-owned Skill assets
- 外部依赖：无；使用 Vitest、临时目录和本地 CLI

## ST-01 TDD 选择与记录

- 类型：normal
- 前置：change 处于 `design`，proposal/design 完整，用户在 plan 阶段选择 `tdd`
- 操作：生成 cases、unit-tests、tasks
- 断言：tasks 记录 `implementation-mode: tdd`；包含 Red → Green → Refactor；不包含 `implementation-ready`
- 证据：`.spec/changes/.../tasks.md`、workflow contract test

## ST-02 Direct 选择与记录

- 类型：normal
- 前置：用户在 plan 阶段明确选择 `direct`
- 操作：生成 tasks
- 断言：tasks 记录 `implementation-mode: direct`；使用 Implement → Verify → Refactor；不要求 Red 证据
- 证据：tasks 模板内容测试

## ST-03 合法 mode 直接路由 apply

- 类型：normal
- 前置：tasks、proposal、design、cases 完整且 `implementation-mode` 为 `tdd` 或 `direct`
- 操作：调用 `wiki-continue`
- 断言：目标 Skill 为 `wiki-apply`；不读取、不要求 `implementation-ready`
- 证据：routing contract test、Skill 正文

## ST-04 缺失或非法 mode 回到 plan

- 类型：failure
- 前置：tasks 缺少 `implementation-mode` 或值不在 `tdd|direct`
- 操作：调用 `wiki-continue`
- 断言：fail-closed 路由到 `wiki-plan`，不修改源码或 change stage
- 证据：routing contract test、strict validate 输出

## ST-05 无 readiness 字段仍可 apply

- 类型：normal
- 前置：tasks 完整、没有 `implementation-ready`，当前用户明确授权实现，strict validate 通过
- 操作：调用 `wiki-apply`
- 断言：进入 implementation；按 mode 执行相应证据流程
- 证据：apply contract test、tasks 证据

## ST-06 未授权仍暂停

- 类型：failure
- 前置：tasks 完整且 mode 合法，但当前请求只有查看/规划意图
- 操作：调用 `wiki-apply`
- 断言：暂停并报告需要实现授权，不修改实现文件
- 证据：apply authorization test

## ST-07 中英文资产一致

- 类型：boundary
- 前置：zh/en package assets 与 repo-local `.agents/skills` 均存在
- 操作：扫描 8 个 Skill、references、tasks 模板和旧门禁 token
- 断言：正文按语言本地化；机器字段稳定；current surface 不含 `implementation-ready`；历史 archive 可保留旧字段
- 证据：content contract、Wiki validate、`git diff --check`

## 成功标准映射

| 成功标准 | ST | 证据 |
| --- | --- | --- |
| plan 询问 tdd/direct，已有选择不重复询问 | ST-01 / ST-02 | wiki-plan Skill 与 plan artifact |
| tasks 只记录合法 implementation-mode | ST-01 / ST-02 / ST-04 | tasks template/content tests |
| 合法 mode 直接进入 apply | ST-03 / ST-05 | routing/apply contract tests |
| readiness 门禁删除但授权安全保留 | ST-05 / ST-06 | apply contract tests |
| 双语资产同步且 current surface 无旧 token | ST-07 | content scan/status/validate |
