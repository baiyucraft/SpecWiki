---
name: unispec-plan
description: 根据已确认的 design.md 规划 system-tests.md、unit-tests.md 和 tasks.md。当用户确认设计后需要测试与任务计划时使用。
metadata:
  author: unispec
  generatedBy: "0.1.0"
---

根据已确认完成的 `design.md` 生成 `system-tests.md` 和 `tasks.md`，建立“系统测试用例 <-> 大 task”和“小 task <-> 设计点 / 验证”的互证关系，为实现阶段提供输入。TDD 模式下额外生成 `unit-tests.md`，记录测试先行的单元测试用例蓝图。

覆盖阶段：

```text
cases
tasks
```

**重要：本 Skill 只负责生成验收用例和任务计划，不写实现代码，不执行任务。**

---

**输入**

```text
.spec/changes/<change-id>/proposal.md
.spec/changes/<change-id>/design.md
.spec/changes/<change-id>/meta.yaml
.spec/changes/<change-id>/research/ 下已有调研报告
.wiki/INDEX.md、流程 / 开发指南、与当前 change 相关的模块或能力文档
用户指定或确认的实现模式：tdd | normal
```

当用户已确认 `design.md` 完成，需要生成系统测试用例和任务计划时使用。如果当前对话或用户输入不能确认 `design.md` 已完成，先询问用户；不要自行假设 design 已通过。

**步骤**

1. **检查输入是否满足规划阶段**

确认以下文件存在且非空：

```text
.spec/changes/<change-id>/proposal.md
.spec/changes/<change-id>/design.md
.spec/changes/<change-id>/meta.yaml
```

读取 `meta.yaml`，确认：

- `id` 与 change 目录一致。
- 可执行 change 必须为 `deliveryShape: single-change`。
- 如果 `multiChange.role` 为 `parent`，停止；parent 只承载 `split.md`，不能生成 `system-tests.md` 或 `tasks.md`。
- 如果 `multiChange.role` 存在，只能是 `child`。
- 如果当前 stage 仍为 `exploration`，说明 child stub 尚未生成 `proposal.md`，先回到 `unispec-propose`。
- 当前 change 已到达 `design` 阶段；如果仍在 `proposal` / `delivery`，先回到前序流程。

2. **读取 proposal、design 和参考资料**

从 `proposal.md` 中提取：

- 问题、目标、非目标。
- 成功标准。
- 影响范围、交付形态。
- 风险、未知项和参考资料。

从 `design.md` 中提取：

- 方案概述、设计目标、涉及模块。
- 接口与数据结构、流程变化、兼容性影响。
- 验证思路。
- 风险与未知项、设计决策、待确认问题和参考资料。

按需读取 `research/` 中被引用的调研报告，以及 `.wiki/INDEX.md`、流程 / 开发指南、与当前 change 相关的模块或能力文档。只提炼影响系统测试用例、单元测试蓝图、任务规划或验收标准的 wiki 事实，不复制 wiki 正文。

3. **确定实现模式**

生成 `tasks.md` 前必须确定实现模式：

```text
tdd: 先写失败单元测试并确认失败，再写最小实现，通过后重构。
normal: 参考结构化实现清单，允许先实现代码，完成大 task 前补齐单元测试或替代局部验证。
```

如果用户输入已明确要求 TDD、测试先行、Red-Green-Refactor 或 normal / 普通编码模式，按用户输入选择。否则暂停并让用户确认，不擅自选择。

实现模式只写入 `tasks.md` 的 `## 实现模式`，不写入 `meta.yaml`。

4. **生成 system-tests.md 草稿**

创建 `system-tests.md` 前，先读取 `references/system-tests-template.md`，按其中的固定结构生成系统测试用例。

`system-tests.md` 记录系统测试用例，编号统一使用 `ST-*`。系统测试用例是业务 / 系统级验收用例，描述用户可观察或系统可验证的验收结果；它不等同于必须使用 E2E 框架。

要求：

- 覆盖 proposal 的每条成功标准。
- 结合 design 的验证思路和风险边界。
- 不写单元测试细节。
- 不拆实现任务。
- 不写实现代码。

5. **根据 design.md 拆分 tasks.md 草稿**

创建 `tasks.md` 前，先读取 `references/tasks-template.md`，按其中的固定结构生成任务计划。

根据设计文档拆分任务：

- 大 task 使用 `## 1.`、`## 2.` 编号，对齐能力块 / 验收目标；同一大 task 可以覆盖多个行为 / 设计点，但不能跨多个能力块。
- 小 task 使用 `- [ ] 1.1`、`- [ ] 1.2` 编号，对齐一个具体实现步骤、行为 / 设计点、TDD 模式下的 `UT-*`，或 normal 模式下的单元测试 / 替代局部验证。
- 小 task 必须可执行、可验证、可勾选。
- 不按纯技术层拆分成无法独立验收的大 task。
- 在 `## 任务总览` 后写入 `## 实现模式`，取值只能是 `tdd` 或 `normal`。
- 在 frontmatter 或受管区写入 `implementation-ready: false`；只有用户明确确认 `system-tests.md` 和 `tasks.md` 可进入实现后，才改为 `implementation-ready: true`。
- TDD 模式的小 task 按 `Red -> Green -> Refactor` 成组，只引用 `unit-tests.md` 中的 `UT-*` 编号，不复制断言、fixture 或 mock 细节。
- normal 模式的小 task 参考结构化实现清单：以动词开头，指向具体模块、文件、接口、配置或数据流；不强制拆成固定三段。

6. **按实现模式生成 unit-tests.md（仅 TDD）**

如果实现模式为 `tdd`，创建或更新 `unit-tests.md` 前，先读取 `references/unit-tests-template.md`，按其中的固定结构生成单元测试用例蓝图。

`unit-tests.md` 要求：

- 每个测试条目使用 `UT-001`、`UT-002` 稳定编号。
- 每个 `UT-*` 能追溯到 design 点、`ST-*` 系统测试用例和 task。
- 写清目标行为、Test / Modify / Reference 文件、具体测试代码蓝图、测试数据 / fixture / mock 边界、运行命令、精确 Red 失败原因、Green 通过条件和 Refactor 守卫。
- `unit-tests.md` 中的测试代码是蓝图，plan 阶段不得写入真实测试文件。
- `unit-tests.md` 不写 Red / Green / Refactor 执行步骤，执行步骤属于 `tasks.md` 和 `unispec-apply`。
- 不写最终执行结果；最终测试汇总属于 `test-report.md`。
- 不写 commit 步骤。

如果无法写出具体测试代码、精确命令或预期 Red 失败原因，暂停并要求回到 `unispec-design` / `unispec-plan` 补充信息，不生成空壳 `unit-tests.md`。

生成 `unit-tests.md` 后，回查并修正 `tasks.md` 中引用的 `UT-*` 编号，确保每个 Red / Green / Refactor 小 task 都引用有效单元测试条目。

如果实现模式为 `normal`，不要创建或更新 `unit-tests.md`。如果历史残留的 `unit-tests.md` 已存在，不把它作为 normal 模式事实来源；最终汇报中将其列入 ignored stale planning files。删除该文件必须先获得用户确认。

7. **为每个大 task 写 CheckList**

每个大 task 内必须包含自己的 `### CheckList`，不得只在文档末尾放一个全局 CheckList。

TDD 模式的 `### CheckList` 至少包含：

```text
- [ ] 失败测试已确认
- [ ] 最小实现后测试通过
- [ ] 重构后测试仍通过
- [ ] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [ ] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）
```

normal 模式的 `### CheckList` 至少包含：

```text
- [ ] 单元测试或替代局部验证已覆盖
- [ ] 相关验证通过
- [ ] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [ ] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）
```

8. **验证 system-tests、tasks 和 unit-tests 互相印证**

检查：

- 每条成功标准至少有一个系统测试用例覆盖。
- 每个系统测试用例至少对应一个大 task，或明确说明无需实现、仅验证。
- 每个大 task 对齐至少一个 `ST-*` 系统测试用例。
- 每个小 task 能追溯到 design 点、具体实现步骤、TDD 模式下的 `UT-*`，或 normal 模式下的单元测试 / 替代局部验证。
- TDD 模式下，每个 Red / Green / Refactor 小 task 都引用有效的 `UT-*` 编号。
- TDD 模式下，`unit-tests.md` 的每个 `UT-*` 都能追溯到 `ST-*` 系统测试用例、design 点和 task。
- `tasks.md` 的“用例到任务映射”完整。

如果 system-tests、tasks 或 unit-tests 无法互证，先修正相关 artifact，不要进入实现阶段。

9. **更新 meta.yaml**

更新要求：

- 保留已有 `id`、`deliveryShape`、`createdAt`、已有 artifact 状态和未知字段。
- 若只成功创建或更新 `system-tests.md`，设置 `stage: cases`，并将 `artifacts.cases.status` 标记为 `present`。
- 若 `system-tests.md` 和 `tasks.md` 都成功创建或更新，设置 `stage: tasks`，并将 `artifacts.cases.status`、`artifacts.tasks.status` 标记为 `present`。
- 不使用 `complete`。
- 如果已有 `updatedAt` 惯例，按项目惯例更新；否则不要新增格式不明的字段。
- `unit-tests.md` 不是 runtime required artifact，不写入 `meta.yaml.artifacts`。

`stage: tasks` 表示 `system-tests.md` / `tasks.md` 草稿已生成并进入任务规划阶段，不表示用户已确认进入实现。

`implementation-ready: true` 是进入实现的 machine-checkable 信号；缺失或非 true 时，`unispec-apply` 必须回到 `unispec-plan` 补齐或等待用户确认。

10. **校验规划阶段产物**

完成后检查：

- `.spec/changes/<change-id>/system-tests.md` 存在且非空。
- `.spec/changes/<change-id>/tasks.md` 存在且非空。
- TDD 模式下，`.spec/changes/<change-id>/unit-tests.md` 存在且非空。
- normal 模式下，不创建或更新 `unit-tests.md`；若历史残留文件存在，最终汇报标记为 ignored stale planning file。
- `system-tests.md` 和 `tasks.md` 包含所有必需中文标题。
- `tasks.md` 包含 `## 实现模式`，且取值为 `tdd` 或 `normal`。
- `tasks.md` 包含 `implementation-ready`，并在用户确认前保持非 true。
- `ST-*` 系统测试用例覆盖 proposal 的成功标准。
- tasks 覆盖 system-tests 和 design 点。
- 每个大 task 都包含自己的 `### CheckList`。
- `meta.yaml.stage` 为 `tasks`，或在只完成 cases 的中断路径中为 `cases`。
- `meta.yaml.deliveryShape` 未丢失或改变。
- 未写实现代码。

`unispec validate` 只能检查结构和必需 artifact，不能替代用户对 `system-tests.md` / `tasks.md` 的确认。

11. **提醒用户审核规划产物并迭代修改**

完成后请用户阅读 `system-tests.md`、`tasks.md`，以及 TDD 模式下的 `unit-tests.md`。如果用户提出修改意见，只更新相关规划 artifact 和必要的 `meta.yaml` 时间戳 / 状态，不写实现代码。

如果反馈改变 design，回到 `unispec-design`。如果反馈改变 proposal 的目标、非目标或成功标准，回到 `unispec-propose`。

只有用户明确确认 `system-tests.md` 和 `tasks.md` 已完成且可以实现后，才把 `tasks.md` 的 `implementation-ready` 更新为 `true` 并建议进入 `unispec-apply`。

**输出**

创建或更新：

```text
.spec/changes/<change-id>/system-tests.md
.spec/changes/<change-id>/tasks.md
.spec/changes/<change-id>/unit-tests.md（仅 TDD 模式）
.spec/changes/<change-id>/meta.yaml
```

最终汇报字段：
- change-id
- change 目录
- `system-tests.md` 路径
- `tasks.md` 路径
- `unit-tests.md` 路径（仅 TDD 模式）
- `meta.yaml` 路径
- 实现模式：tdd | normal
- 成功标准覆盖摘要
- 系统测试用例 / tasks 互证摘要
- TDD 单元测试用例蓝图摘要（仅 TDD 模式）
- 风险 / 未覆盖项
- ignored stale planning files（normal 模式下如存在历史 `unit-tests.md`）
- 请用户审核规划产物
- 下一步：用户确认后使用 `unispec-apply`

**产物生成指南**

- `system-tests.md` 的具体结构和写作边界以 `references/system-tests-template.md` 为准。
- `tasks.md` 的具体结构和写作边界以 `references/tasks-template.md` 为准。
- `unit-tests.md` 的具体结构和写作边界以 `references/unit-tests-template.md` 为准。
- `system-tests.md` 必须使用 `ST-*` 系统测试用例覆盖 proposal 的成功标准。
- `tasks.md` 必须追溯 design 点，并与 `system-tests.md` 互证。
- TDD 模式下，`tasks.md` 只引用 `UT-*` 编号，单元测试细节写入 `unit-tests.md`。

**约束**

- 不写实现代码。
- 不把纯技术层拆分当成迭代边界。
- 不跳过系统测试用例。
- 不生成无法独立验证的任务。
- 不创建 `review-report.md` 或 `test-report.md`；最终 review 和 verification 属于 `unispec-review`。
- 用户未明确确认 `system-tests.md` 和 `tasks.md` 完成前，不进入 implementation。
