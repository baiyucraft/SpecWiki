---
name: unispec-apply
description: 执行已确认 ready 的 tasks.md 并同步任务进度。当用户确认规划产物可进入实现时使用。
metadata:
  author: unispec
  generatedBy: "0.1.0"
---

根据已确认完成且 `implementation-ready: true` 的 `system-tests.md` 和 `tasks.md` 执行实现任务。按 `tasks.md` 中的小 task 顺序执行，完成并验证一个就勾选一个，直到 blocked、用户中断、用户只要求当前 task，或所有小 task 完成。

覆盖阶段：

```text
implementation
```

**重要：本 Skill 负责实现任务和同步 tasks.md 进度，不重新定义需求，不自行归档，不主动承担 archive-time wiki 沉淀或目录化拆分。除非 tasks 明确要求且用户确认，否则不更新 `.wiki`；长期知识沉淀和目录化拆分交给 `unispec-archive`。**

---

**输入**

```text
.spec/changes/<change-id>/proposal.md
.spec/changes/<change-id>/design.md
.spec/changes/<change-id>/system-tests.md
.spec/changes/<change-id>/tasks.md
.spec/changes/<change-id>/unit-tests.md（仅 TDD 模式）
.spec/changes/<change-id>/meta.yaml
相关代码、测试、配置和文档
```

当用户已确认 `system-tests.md` 和 `tasks.md` 完成，需要开始或继续实现时使用。如果当前对话或用户输入不能确认 `system-tests.md` 和 `tasks.md` 已完成审核，先询问用户；不要自行假设可以实现。

**步骤**

1. **选择 change 并确认可实现**

如果用户指定 change-id，使用该 change。否则：

- 从当前对话推断 change-id。
- 如果只有一个 active change，可以使用它。
- 如果存在多个 active changes 且无法判断，读取 `.spec/changes/` 并让用户选择。

确认以下文件存在且非空：

```text
.spec/changes/<change-id>/proposal.md
.spec/changes/<change-id>/design.md
.spec/changes/<change-id>/system-tests.md
.spec/changes/<change-id>/tasks.md
.spec/changes/<change-id>/meta.yaml
```

读取 `meta.yaml` 并确认这是可实现 change：

- `deliveryShape` 必须为 `single-change`。
- 如果 `multiChange.role` 为 `parent`，停止；parent 只承载 `split.md`，不能进入实现。
- `stage` 必须为 `tasks` 或 `implementation`。 `tasks` 只表示计划已生成；能否实现必须读取 `tasks.md` 的 `implementation-ready: true`。
- 不得从 `proposal`、`design`、`cases`、`review` 或 `verification` 直接跳入实现。若需要返工，应先确认 review 未推进到 `verification`，并让 change 回到 `implementation` 语义。

如果 `meta.yaml.artifacts` 缺失或与文件状态明显不一致，不要把它当作硬阻断；先提示并让用户确认是否修正 metadata。更新 `meta.yaml` 时必须保留已有 artifact 状态和未知字段。

2. **读取上下文和任务进度**

读取：

```text
proposal.md
design.md
system-tests.md
tasks.md
unit-tests.md（仅 TDD 模式）
meta.yaml
research/ 中被引用的报告
```

先读取 `tasks.md` 的 `## 实现模式`：

开始实现前必须确认 `tasks.md` 的 frontmatter 或受管区包含 `implementation-ready: true`。缺失、为 false 或无法解析时，暂停并要求回到 `unispec-plan` 补齐规划或等待用户确认，不得修改实现文件。

- `tdd`：必须读取 `unit-tests.md`；如果缺失、为空或没有对应 `UT-*` 编号，暂停并要求回到 `unispec-plan` 修正。
- `normal`：不要求 `unit-tests.md`；如存在也不把它当作事实来源，除非用户明确要求参考。不要在 normal 模式下更新历史残留的 `unit-tests.md`。

从 `tasks.md` 中统计：

- 大 task 列表。
- 小 task checkbox 完成情况。
- 每个大 task 的 `### CheckList` 完成情况。
- 下一个未完成小 task。
- TDD 模式下当前小 task 引用的 `UT-*` 编号。

从 `system-tests.md` 中读取 `ST-*` 系统测试用例、success criteria 覆盖矩阵和计划验证方式；系统测试用例 / task 映射以 `tasks.md` 的“用例到任务映射”为准。

3. **显示当前进度**

开始实现前，简要显示：

```text
change-id:
stage:
deliveryShape:
小 task 进度: <done>/<total>
实现模式: <tdd | normal>
当前大 task:
剩余小 task:
```

4. **选择下一个小 task**

默认按 `tasks.md` 顺序选择第一个未完成小 task。如果用户指定 task-id，执行指定小 task。

如果小 task 不清楚、无法验证，或和 `system-tests.md` 无法对应，暂停并建议回到 `unispec-plan` 修正文档。只有设计事实缺失、设计冲突或方案不可实现时，才回到 `unispec-design`。

5. **执行一个小 task**

每轮只处理一个小 task。通用要求：

- 读取相关代码、测试和配置。
- 保持改动最小，只做当前小 task 需要的修改。
- 不跨多个大 task 扩大范围。
- 不顺手实现未选中的任务。

TDD 模式按当前小 task 类型执行：

- Red：按对应 `UT-*` 的测试代码蓝图写入失败测试，只创建或修改测试文件、fixture、mock 或必要测试辅助代码；在 Red 完成前，不修改生产代码。按 `unit-tests.md` 的运行命令执行，确认失败测试名、关键错误信息 / 断言差异和失败原因符合“预期 Red 失败”，且不是语法错误、导入错误、环境错误或无关回归。
- Green：只写让对应 `UT-*` 满足 Green 通过条件的最小生产代码，不扩展为完整实现计划之外的行为。按 `unit-tests.md` 的运行命令确认测试通过。
- Refactor：按对应 `UT-*` 的 Refactor 守卫，在测试保持通过的前提下清理实现，不引入新行为。

normal 模式按当前小 task 执行：

- 允许先实现代码。
- 必须补充或更新对应单元测试，或明确不可单测原因和替代局部验证。
- 大 task 完成前必须运行相关单元测试或替代局部验证，并用结果支撑 `### CheckList` 的“单元测试或替代局部验证已覆盖”和“相关验证通过”。

如果实现暴露 `proposal.md`、`design.md`、`system-tests.md` 或 `tasks.md` 不准确，暂停并建议回到对应 Skill 更新 artifacts，不要在实现阶段悄悄改变需求或设计方向。

6. **运行相关验证**

- TDD Red 小 task 的预期失败是有效验证；只有失败测试名、关键错误信息 / 断言差异和原因解释都匹配 `unit-tests.md` 才能勾选。
- TDD Green / Refactor 小 task 必须保持对应单元测试通过。
- normal 小 task 优先运行对应单元测试或局部验证。
- 当一个大 task 下的小 task 都完成时，运行与该大 task 关联的 `ST-*` 系统测试用例，或 `system-tests.md` 覆盖矩阵中的计划验证方式。
- 完成大 task 前，运行项目可用的局部质量检查命令，例如 lint、typecheck、static analysis、formatter check、compiler check、语言或框架自带检查；如果项目没有对应命令，说明原因。
- 如果存在用户给定的验证命令，优先运行相关命令。
- 如果无法运行验证，说明原因和剩余风险，不勾选未验证 task。TDD Red 失败原因不匹配时，不能勾选，应修正测试或回到 `unispec-plan`。

7. **更新 tasks.md**

当前小 task 验证通过后，立即将对应 checkbox 从 `- [ ]` 改为 `- [x]`。

TDD 模式下：

- Red task 只有在“预期失败验证通过”后才能勾选。
- Green task 只有在对应测试通过后才能勾选。
- Refactor task 只有在重构后测试仍通过后才能勾选。

只有当当前大 task 下的相关小 task 和验证结果足以支撑某个 `### CheckList` 项时，才能勾选该项。不得因为完成一个小 task 就机械勾完整个 CheckList。

TDD 模式下，如果一个大 task 包含多个 `UT-*`，只有该大 task 下所有关联 `UT-*` 的 Red / Green / Refactor 任务都满足对应条件后，才能勾选 TDD 相关 CheckList 项。

本大 task 的局部质量检查只覆盖该大 task，不代表整个 change 已通过 `unispec-review`。

不把未完成、未验证或仍有阻塞的 task 勾选完成。

8. **更新 meta.yaml**

开始实现或完成至少一个小 task 后，将 `meta.yaml.stage` 设置为 `implementation`。

更新要求：

- 保留已有 `id`、`deliveryShape`、`createdAt`、已有 artifact 状态和未知字段。
- 不把 implementation 标成完成。
- 如果已有 `updatedAt` 惯例，按项目惯例更新；否则不要新增格式不明的字段。

`stage: implementation` 表示实现进行中，不表示实现完成。实现完成由后续 `review` / `verification` 阶段判断。

9. **循环或暂停**

完成、验证并勾选当前小 task 后：

- 如果用户只要求当前 task，汇报本轮结果并停止。
- 如果还有 pending 小 task，继续选择下一个小 task。
- 如果遇到 blocked、错误、任务不清、验证无法完成或用户中断，暂停并汇报原因。
- 如果所有小 task 和每个大 task 的 `### CheckList` 都完成，汇报实现完成，并建议进入 `unispec-review`。

**输出**

实现中输出：

```text
正在实现：<change-id>
当前大 task：<major-task>
当前小 task：<task-id> <task description>
涉及文件：
- ...
验证方式：
- ...
```

暂停时输出：

```text
暂停原因：
当前进度：
已完成小 task：
阻塞问题：
可选下一步：
```

完成时输出：

```text
实现完成：<change-id>
小 task 进度：<done>/<total>
本轮完成：
- ...
验证结果：
- ...
修改文件：
- ...
下一步：使用 `unispec-review`
```

**实现指南**

当前执行者始终负责：

```text
选择任务
复核执行结果
运行或确认验证
更新 tasks.md
更新 meta.yaml
```

默认循环执行，直到 blocked、用户中断、用户只要求当前 task，或所有小 task 完成。每轮执行边界始终是一个小 task。

---

**暂停条件**

遇到以下情况必须暂停：

- 必需 artifacts 缺失、为空或 `meta.yaml` 不可解析。
- `deliveryShape` 不是 `single-change`，或当前 change 是 parent。
- `meta.yaml.stage` 不是 `tasks` 或 `implementation`。
- `tasks.md` 缺少 `implementation-ready: true`。
- 小 task 不清楚或无法验证。
- TDD 模式下 `unit-tests.md` 缺失、为空、没有对应 `UT-*` 编号，或 Red 失败原因无法判断。
- 实现发现 proposal、design、system-tests 或 tasks 不准确。
- 需要改变设计方向或成功标准。
- 测试或验证失败且无法在当前小 task 范围内修复。
- 用户中断或要求暂停。

**约束**

- 不重新解释需求。
- 不修改设计方向。
- 不跨多个大 task 扩大范围。
- 不跳过测试。
- 不勾选未验证 task。
- 不自行归档。
