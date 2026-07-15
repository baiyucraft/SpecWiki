---
implementation-ready: true
---

# close-specwiki-3-0-design-baseline-product-contract 任务计划

## 任务总览

任务按四个可验收能力块拆分：canonical baseline 可达性、版本 truth、状态与完成判定、材料与迁移边界。用户已明确授权完整 TDD 实现、自动验证、review 和归档，因此本计划直接标记为可实现。

## 实现模式

tdd

先按 `unit-tests.md` 写入单个失败测试并确认预期 Red，再做最小 Markdown 合同实现，通过后重构测试 helper 或文档结构。每个能力块结束时运行局部 Vitest、ESLint、UniSpec 校验和注释规范检查。

## 1. 建立唯一可达的 canonical baseline

- [x] 1.1 Red: UT-001 编写 canonical 页面存在、双入口可达和职责分离的失败测试，并确认缺页诊断符合预期
- [x] 1.2 Green: UT-001 创建 `05-产品基线与设计治理.md` 最小骨架，并更新设计 INDEX 与总体设计回链使测试通过
- [x] 1.3 Refactor: UT-001 清理固定路径和必要标记 helper，在测试保持通过时确保无递归扫描或全文快照

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 2. 固化独立版本域与显式映射

- [x] 2.1 Red: UT-002 编写三类版本域、authority、scope、显式 relation 和 manifest 对照的失败测试
- [x] 2.2 Green: UT-002 在 canonical 页面补齐版本 truth model、当前显式映射和数值独立规则使测试通过
- [x] 2.3 Refactor: UT-002 收敛 manifest 读取和诊断 helper，保持固定清单且不引入版本 lockstep

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 3. 分离设计决策、交付证据和完成判定

- [x] 3.1 Red: UT-003 编写四维正交状态合同的失败测试
- [x] 3.2 Green: UT-003 在 canonical 页面实现 decision / implementation / verification / release 证据模型和合法组合
- [x] 3.3 Refactor: UT-003 清理状态断言，在保持通过时避免将状态收缩为单一完成值
- [x] 3.4 Red: UT-004 编写单域与全项目两级完成规则的失败测试
- [x] 3.5 Green: UT-004 在 canonical 页面实现单域清单、六 child 与 documentation-closure 门禁
- [x] 3.6 Refactor: UT-004 收敛完成规则断言，并保持设计完成与实现 / 发布完成解耦

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 4. 固化有界材料分类并完成整体合同验证

- [x] 4.1 Red: UT-005 编写材料类别、只读 archive 和 documentation-closure 延期边界的失败测试
- [x] 4.2 Green: UT-005 在 canonical 页面补齐稳定材料分类、下游输入和不执行全库迁移的边界
- [x] 4.3 Refactor: UT-005 清理材料边界断言，保持固定输入并避免复制一次性迁移任务

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 | 1. 建立唯一可达的 canonical baseline | 1.1-1.3 / UT-001 |
| ST-002 | 2. 固化独立版本域与显式映射 | 2.1-2.3 / UT-002 |
| ST-003 | 3. 分离设计决策、交付证据和完成判定 | 3.1-3.3 / UT-003 |
| ST-004 | 3. 分离设计决策、交付证据和完成判定 | 3.4-3.6 / UT-004 |
| ST-005 | 4. 固化有界材料分类并完成整体合同验证 | 4.1-4.3 / UT-005 |

## 执行顺序

- 依次执行 1 -> 2 -> 3 -> 4；后续测试复用 canonical 页面和前序 helper。
- 每个 Red 只新增当前 UT 测试，确认预期失败后再修改 Markdown 生产合同。
- 每个大 task 完成时运行关联 `ST-*`、单文件 ESLint 和 `unispec validate`。
- 所有 task 完成后运行根 Vitest、`pnpm run lint` 和 `pnpm test`，再进入 `unispec-review`。

## 暂缓事项

- 全库状态、INDEX 标签、capability Purpose、roadmap 和旧 authority 指针迁移由 `close-specwiki-3-0-design-baseline-documentation-closure` 执行。
- registry、Git tag、binary checksum 和 staged publish evidence 的长期落点由后续 release / documentation-closure change 决定。
