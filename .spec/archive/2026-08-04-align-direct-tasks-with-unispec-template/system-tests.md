# align-direct-tasks-with-unispec-template 系统测试

## 测试环境

- runtime/platform：Node.js >=20.19、Windows/PowerShell 或 CI
- fixture/data：当前 zh Wiki、双语 package assets、临时 Codex 项目
- 外部依赖：无；使用 Vitest、本地构建和 Lite CLI

## ST-01 direct 结构化任务模板

- 类型：normal
- 前置：选择 `direct`，读取双语 tasks template
- 操作：生成 direct tasks
- 断言：模板包含任务总览、实现模式、编号大 task、每个大 task 的 `### CheckList`、用例映射、执行顺序和暂缓事项；小 task 使用具体动作/模块/文件/接口/配置/数据流描述
- 证据：workflow contract test、模板文件

## ST-02 tdd 分支保持兼容

- 类型：normal
- 前置：选择 `tdd`
- 操作：读取同一双语模板并生成 TDD tasks
- 断言：仍有 Red → Green → Refactor 结构和 UT 映射，不要求 direct 的 Red 省略规则
- 证据：workflow contract test

## ST-03 禁止词与稳定字段

- 类型：failure
- 前置：扫描 package assets、repo-local `wiki-plan` 和 current docs
- 操作：执行内容合同扫描
- 断言：没有 readiness 门禁、`normal` 机器值、`unispec-*` 可执行命令；保留 `implementation-mode: <tdd|direct>`
- 证据：current-surface test、strict validate

## ST-04 package-owned 同步

- 类型：boundary
- 前置：修改 package assets 后执行 build/update
- 操作：同步 repo-local `wiki-plan` 与 tasks reference
- 断言：目标语言文件逐字一致，用户未登记文件保留，Wiki status 仍 ready
- 证据：`spec-wiki-lite update --json`、status、asset sync tests

## 成功标准映射

| 成功标准 | ST | 证据 |
| --- | --- | --- |
| direct 采用 UniSpec 结构化任务清单 | ST-01 | template contract |
| tdd 分支保持 Red/Green/Refactor | ST-02 | workflow contract |
| current surface 没有旧门禁/命令 | ST-03 | content scan |
| package/repo-local ownership 同步 | ST-04 | build/update/status |
