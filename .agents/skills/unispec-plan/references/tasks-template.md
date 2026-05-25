---
implementation-ready: false
---

# Tasks 模板

用于 `unispec-plan` 生成 `.spec/changes/<change-id>/tasks.md`。

Tasks 根据已确认的 `design.md` 和 `system-tests.md` 拆分实现任务。大 task 对齐能力块或验收目标；小 task 对齐具体实现步骤、行为 / 设计点、TDD 模式下的 `UT-*`，或 normal 模式下的单元测试 / 替代局部验证。每个大 task 都必须包含自己的 `### CheckList`。

## 使用规则

- `tasks.md` 必须使用本文档中的固定二级标题。
- 大 task 使用 `## 1.`、`## 2.` 编号。
- 小 task 使用 `- [ ] 1.1`、`- [ ] 1.2` checkbox 编号。
- `implementation-ready` 默认为 `false`；只有用户确认 `system-tests.md` 和 `tasks.md` 可进入实现后，才改为 `true`。
- `## 实现模式` 必须位于 `## 任务总览` 之后、首个大 task 之前，取值只能是 `tdd` 或 `normal`。
- `tdd` 模式的小 task 必须按 `Red -> Green -> Refactor` 成组，并引用 `unit-tests.md` 中的 `UT-*` 编号。
- `normal` 模式的小 task 参考结构化实现清单：以动词开头，指向具体模块、文件、接口、配置或数据流；不强制拆成固定三段。
- 每个大 task 必须包含自己的 `### CheckList`，不得只在文档末尾放一个全局 CheckList。
- 一个大 task 可以覆盖多个行为 / 设计点，但这些行为 / 设计点必须围绕同一能力块或验收目标；不同能力块应拆成不同大 task。
- 小 task 必须可执行、可验证、可勾选。
- 不生成无法独立验证的大 task。

## 固定结构

~~~md
---
implementation-ready: false
---

# <change-id> 任务计划

## 任务总览

<说明任务拆分依据：来自 design.md 的能力块 / 验收目标。>

## 实现模式

tdd | normal

<tdd：先写失败单元测试并确认失败，再写最小实现，通过后重构。normal：参考结构化实现模式，先按具体实现清单推进，完成大 task 前补齐单元测试或替代局部验证。>

## 1. <大 task：能力块 / 验收目标>

TDD 模式：

- [ ] 1.1 Red: UT-001 编写 <行为 / 设计点 1> 的失败单元测试，并确认失败原因符合预期
- [ ] 1.2 Green: UT-001 编写最小实现，使该测试通过
- [ ] 1.3 Refactor: UT-001 在测试保持通过的前提下清理实现
- [ ] 1.4 Red: UT-002 编写 <行为 / 设计点 2> 的失败单元测试，并确认失败原因符合预期
- [ ] 1.5 Green: UT-002 编写最小实现，使该测试通过
- [ ] 1.6 Refactor: UT-002 在测试保持通过的前提下清理实现

normal 模式（结构化实现示例，按实际任务增删，不固定条数）：

- [ ] 1.1 创建/修改 <具体模块 / 文件 / 接口> 以支持 <行为 / 设计点 1>
- [ ] 1.2 接入 <调用路径 / 配置 / 数据流> 到 <行为 / 设计点 2> 的使用场景
- [ ] 1.3 更新 <配置 / 数据结构 / 文档> 以匹配 <行为 / 设计点 3>

如测试或验证需要独立执行，可新增类似任务：

- [ ] 1.4 添加/调整 <测试文件 / 验证命令> 覆盖 <行为 / 设计点 1>

### CheckList

TDD 模式：

- [ ] 失败测试已确认
- [ ] 最小实现后测试通过
- [ ] 重构后测试仍通过
- [ ] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [ ] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

normal 模式：

- [ ] 单元测试或替代局部验证已覆盖
- [ ] 相关验证通过
- [ ] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [ ] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 | 1. <大 task> | 1.1 / 1.2 / UT-001 |

## 执行顺序

- <任务依赖顺序和执行建议>

## 暂缓事项

- <暂不执行的任务、原因和触发条件；没有则写“无”>
~~~

## 写作边界

- 大 task 面向一个可验证能力块或验收目标，不按纯技术层拆分；一个大 task 可以覆盖多个行为 / 设计点，但不能跨多个能力块。
- 小 task 写具体动作，不写泛泛的“完善逻辑”“处理边界”；每个小 task 对齐一个具体行为 / 设计点、实现步骤、TDD 模式下的 `UT-*`，或 normal 模式下的单元测试 / 替代局部验证。
- 单元测试或替代局部验证可以作为小 task，但不能替代系统测试用例。
- TDD 模式下，单元测试用例蓝图写入 `unit-tests.md`，`tasks.md` 只引用 `UT-*` 编号并负责 Red / Green / Refactor 执行任务。
- TDD 模式下，一个大 task 包含多个 `UT-*` 时，`### CheckList` 项必须覆盖该大 task 下所有关联 `UT-*` 后才能勾选。
- CheckList 是本大 task 的小 review，不是整个 change 的最终 review。
- `用例到任务映射` 必须能说明每个 `ST-*` 系统测试用例如何被实现任务覆盖。
