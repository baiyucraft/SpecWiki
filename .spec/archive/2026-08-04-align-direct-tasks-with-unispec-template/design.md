# align-direct-tasks-with-unispec-template 设计

## 方案概述

将现有双语 Lite tasks 模板改为一份可分支使用的固定结构。所有模式共享 `任务总览`、`实现模式`、大 task、checklist、映射、执行顺序和暂缓事项；`tdd` 的大 task 小 task 使用 Red → Green → Refactor，`direct` 的小 task 使用 UniSpec 改写后的结构化实现清单。

## 模板合同

- 首行机器字段：`implementation-mode: <tdd|direct>`。
- 固定标题：`## 任务总览`、`## 实现模式`、`## 1.`、`### CheckList`、`## 用例到任务映射`、`## 执行顺序`、`## 暂缓事项`。
- direct 小 task：动词开头，明确模块/文件/接口/配置/数据流；可加入单元测试或替代局部验证任务，但不要求 Red。
- tdd 小 task：按 UT 编号成组写 Red、Green、Refactor，并要求 Red 失败证据。
- 每个大 task 的 checklist 至少覆盖局部验证、质量检查和注释规范；全局 review/verification/archive 仍由后续 Skills 负责。
- 不写 readiness 字段，不写 `normal` 机器值，不写 `unispec-*` 命令。

## Ownership、路径与回滚

- package assets 是唯一模板 SSOT；`.agents/skills/wiki-plan/**` 由 `init/update` 同步。
- 用户未登记的 Skill 文件保留；模板登记文件按既有 ownership 策略更新。
- 只修改模板、Skill 正文和合同测试；失败时可由资产同步器原子恢复旧文件。
- 历史 `.spec/archive/**` 不扫描、不改写。

## 验证设计

- 内容测试逐语言检查固定标题、direct 结构、tdd 结构、稳定字段和禁止词。
- package/root 聚合测试确认同步、tarball、Wiki 健康和无旧 readiness/UniSpec 可执行表述。
- 来源记录：UniSpec 0.1.0 tasks template；目标落点是 Lite `wiki-plan` 双语 assets；采用方式为结构与行为改写。

## 回滚

若模板合同测试失败，回退 package-owned 双语模板和 Skill 后重新执行 `spec-wiki-lite update`；不移动或改写历史 change。
