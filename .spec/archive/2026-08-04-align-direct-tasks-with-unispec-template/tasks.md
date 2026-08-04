# align-direct-tasks-with-unispec-template 任务计划

## 任务总览

本 change 按 UniSpec tasks template 的能力块结构，更新 Lite 双语 `wiki-plan` Skill 与 tasks reference，并用合同测试验证 direct/tdd 分支、禁止词和资产同步。

## 实现模式

direct

参考 UniSpec 的结构化实现清单模式：先按具体模块、文件、接口、配置或数据流推进，完成大 task 前补齐自动化测试或替代局部验证；不要求 Red 失败证据。

## 1. 双语 tasks 模板结构

- [x] 1.1 重写 zh/en tasks template，加入任务总览、实现模式、编号大 task、direct 小 task 示例和 `### CheckList`。证据：双语模板内容合同通过。
- [x] 1.2 加入用例到任务映射、执行顺序、暂缓事项，并保留 tdd Red/Green/Refactor 分支。证据：focused workflow contract 5/5 passed。
- [x] 1.3 保留 `implementation-mode` 稳定字段，删除 readiness、`normal` 机器值和 UniSpec 命令。证据：current package/repo-local scan 通过。

### CheckList

- [x] direct 结构化清单可执行且每个小 task 指向具体对象
- [x] tdd 分支仍能表达 UT 与 Red/Green/Refactor
- [x] 模板内容合同测试通过
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 2. Skill 说明与 repo-local 同步

- [x] 2.1 更新 zh/en `wiki-plan/SKILL.md`，说明 direct 采用结构化清单、每个大 task 的 checklist 和映射要求。证据：双语 Skill 内容同步。
- [x] 2.2 执行 build 与 `spec-wiki-lite update`，同步 `.agents/skills/wiki-plan/**`。证据：update --json 更新 2 个登记文件。
- [x] 2.3 保留用户未登记文件，并确认 Wiki status 健康。证据：status ready=true、8 Skills installed。

### CheckList

- [x] package-owned 文件与 repo-local 文件逐字一致
- [x] status 报告 Wiki ready、Skills installed
- [x] asset sync 局部测试通过

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-01 | 1. 双语 tasks 模板结构 | 1.1 / 1.2 / workflow contract |
| ST-02 | 1. 双语 tasks 模板结构 | 1.2 / workflow contract |
| ST-03 | 1. 双语 tasks 模板结构 | 1.3 / content scan |
| ST-04 | 2. Skill 说明与同步 | 2.1 / 2.2 / 2.3 |

## 执行顺序

1. 先更新合同测试并取得 direct 结构缺失的 Red 结果。
2. 重写双语模板，再同步 Skill 和 repo-local 资产。
3. 执行 focused、aggregate、lint、typecheck、build、pack、Wiki 和 diff gates。

## 暂缓事项

- 不修改历史 archive，不复制 UniSpec readiness 或 `normal` 字段。
