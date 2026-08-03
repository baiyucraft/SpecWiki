---
name: wiki-plan
description: 把已接受的设计转换为 normal/failure/boundary 系统测试、可选 TDD 单元测试和可执行任务计划。
---

# Wiki Plan

只负责 `design`、`cases`、`tasks` 阶段。计划必须覆盖成功标准并允许实施者按 Red/Green/Refactor 执行。

## 前置条件

- proposal 与 design 完整、一致且 strict validate 无 blocker。
- change 是可执行 standalone/child，不是 parent。
- 每个成功标准都有可观察结果。

## 输入

- proposal、design、metadata、research。
- 项目现有测试约定、命令、受影响文件和可用运行环境。
- `references/system-tests-template.md`、`references/unit-tests-template.md`、`references/tasks-template.md`。
- 涉及浏览器交互时读取 `references/browser-automation.md`。

## System Tests

- 使用稳定 `ST-*` id，覆盖正常、失败、边界和回归场景。
- 写清前置环境、测试数据、动作、断言、失败关闭和证据形式。
- 系统测试不等于只写 E2E；CLI、API、文件检查、集成测试或人工验证都可作为合适证据。

## TDD Unit Tests

- 适合自动化时使用 `UT-*`，明确 Test/Modify、Red 原因、Green 最小行为和 Refactor 守护。
- 每个 UT 映射到 ST、成功标准或安全边界；不要只追求实现细节覆盖。

## Tasks

1. Red：先增加能因目标缺失而失败的相关测试，并记录失败证据。
2. Green：按依赖顺序实现最小完整主链。
3. Refactor：收口类型、错误、重复逻辑、文档和聚合验证。
4. 每个 task 映射 ST/UT、成功标准与验证命令。
5. 只有用户授权实现时写 `implementation-ready: true`；否则保持 false 并暂停。

## 浏览器自动化

浏览器工具是可选证据手段。只有项目可启动、测试数据可控、权限和副作用可接受时采用。优先使用项目已有工具；不可用时记录 fallback reason，并用组件测试、API/CLI 断言或有步骤的手工证据覆盖。不得仅凭截图判 pass。

## 输出

- `system-tests.md`
- TDD 适用时的 `unit-tests.md`
- `tasks.md` 与 `implementation-ready` signal
- planning 完成后 stage tasks，strict validate 通过

## 暂停条件

- 成功标准无法映射到证据。
- design 缺少接口、ownership、安全或回滚决定。
- 未获得实现授权。

## 下一阶段

只有 `implementation-ready: true` 时使用 `wiki-apply`。
