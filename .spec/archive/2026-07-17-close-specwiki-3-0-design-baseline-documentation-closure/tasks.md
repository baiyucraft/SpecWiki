---
implementation-ready: true
---

# close-specwiki-3-0-design-baseline-documentation-closure 任务计划

## 任务总览

任务按 design 的六个可独立验证能力块拆分。用户已授权完整自动执行、无需逐阶段审核，因此规划产物生成后直接标记 implementation-ready；实现仍严格按每组 Red、Green、Refactor 和局部质量检查推进。

## 实现模式

tdd

先写失败合同测试并确认失败原因，再执行最小文档迁移，通过后收敛 helper 和措辞，最后运行专题及全量验证。

## 1. Capability Purpose、inventory 与 KnowledgeUnit-first 收口

- [x] 1.1 Red: UT-001 写入 Purpose、inventory 和 family capability 失败测试，并确认 4 个占位与平行 capability 触发预期失败
- [x] 1.2 Green: UT-001 补齐/合并 capability Purpose 与 requirements，删除 `content-family-planner` 并同步 INDEX，使测试通过
- [x] 1.3 Refactor: UT-001 收敛 Purpose/INDEX parser 和 family 限定措辞，保持测试通过

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（root Vitest 定向测试与 `git diff --check`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 2. Canonical query 与 v0.2.0 release authority 收口

- [x] 2.1 Red: UT-002 写入 query projection、release authority 和 evidence 正交失败测试，确认旧 matched 字段与缺失 authority 触发预期失败
- [x] 2.2 Green: UT-002 重写 query/release capability 投影并创建 v0.2.0 Wiki release contract，使测试通过
- [x] 2.3 Refactor: UT-002 统一版本域、staging 和 evidence 术语，复跑 Runtime query/product baseline tests

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（documentation closure + Runtime query + product baseline tests）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 3. 设计状态、场景和 Codex-first 当前事实收口

- [x] 3.1 Red: UT-003 写入 adopted/evidence、host facts 和 roadmap 导航失败测试，确认过时状态触发预期失败
- [x] 3.2 Green: UT-003 更新设计 INDEX、总体设计、Agents、核心场景与产品基线，使测试通过
- [x] 3.3 Refactor: UT-003 统一 authority 状态词并复跑 core scenario/host trigger/product baseline tests

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（相关 root contract tests 与 `git diff --check`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 4. `.docs` reference inventory 收口

- [x] 4.1 Red: UT-004 写入 survivor inventory/front matter 失败测试，确认 design/roadmap/release/quality 残留触发预期失败
- [x] 4.2 Green: UT-004 迁移稳定内容、删除已迁移/已完成材料、重写 `.docs/INDEX.md` 并标记保留 research，使测试通过
- [x] 4.3 Refactor: UT-004 收敛显式 survivor inventory，检查删除路径无 current 引用

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（documentation closure test 与 `git diff --check`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 5. README、CLI 与 release 公开投影收口

- [x] 5.1 Red: UT-005 写入中英文 README 的 CLI/host/release parity 失败测试，确认旧 `/wiki:*`、缺 archive 和非 Codex-first 触发预期失败
- [x] 5.2 Green: UT-005 更新 README EN/CN、对外方法 INDEX 和 Wiki 根索引，使测试通过
- [x] 5.3 Refactor: UT-005 收敛共享结构 marker，复跑 CLI surface/distribution/package build

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（CLI/distribution tests、package build、`git diff --check`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 6. Current links、full verification 与归档准备

- [x] 6.1 Red: UT-006 写入 current link/active pointer 失败测试，确认删除材料链接或已归档 active pointer触发预期失败
- [x] 6.2 Green: UT-006 修正 current authority 链接与具体 change pointer，使测试通过
- [x] 6.3 Refactor: UT-006 限定 scan roots、排除 archive/generic paths，并执行全量测试、review/verification 准备
- [x] 6.4 Review Red: UT-004/UT-006 增加 adoptedRefs 无效目标和 Wiki runtime path 反例，确认门禁漏报/误纳入风险
- [x] 6.5 Review Green: UT-004/UT-006 解析并验证 adoptedRefs，限定 current Wiki 长期页面扫描范围
- [x] 6.6 Review Refactor: UT-004/UT-006 收敛 front matter/path helper，重跑全量测试与 full review/verification
- [x] 6.7 Review Red: UT-004 增加 `.wiki/../` authority 逃逸反例并确认 current Wiki 判定漏报
- [x] 6.8 Review Green/Refactor: UT-004 要求 canonical Wiki path，重跑验证与 full review/verification

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（全量 `pnpm test`、UniSpec validate、`git diff --check`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

### Review Remediation CheckList

- [x] review 反例 Red 已确认
- [x] 最小修复后专题测试通过
- [x] 重构后全量测试、lint、build、UniSpec validate 与 `git diff --check` 通过
- [x] full review 与 full verification 重新执行

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 | 1. Capability 收口 | 1.1-1.3 / UT-001 |
| ST-002 | 2. Query 与 release authority | 2.1-2.3 / UT-002 |
| ST-003 | 3. 设计状态与宿主事实 | 3.1-3.3 / UT-003 |
| ST-004 | 4. `.docs` inventory | 4.1-4.3 / UT-004 |
| ST-005 | 5. README/CLI/release 投影 | 5.1-5.3 / UT-005 |
| ST-006 | 6. 链接、verification 与归档准备 | 6.1-6.3 / UT-006 |

## 执行顺序

- 先创建同一 root 合同测试的六个 Red，并逐组确认失败与待实现行为对应。
- Capability 与 query/release authority 先完成，作为设计/README/`.docs` 投影输入。
- Wiki 状态和 `.docs` 迁移完成后再修 README 与全局链接，避免重复改链。
- 最后执行专题测试、全量测试、full review/verification 和 archive checks。

## 暂缓事项

- Registry/tag/checksum/publish evidence 采集不在本 change；未来由独立 release change 实现。
- 完整 HostAdapter、task-aware trigger scope 和 richer public query intents 保持各自已确认延期边界。
