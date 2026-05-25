# Unit Tests 模板

用于 `unispec-plan` 在 TDD 模式下生成 `.spec/changes/<change-id>/unit-tests.md`。

Unit Tests 是 TDD 模式的单元测试用例蓝图，用来定义每个 `UT-*` 应该测试什么、测试代码应如何书写、Red 应如何失败，以及 Green / Refactor 的判定条件。它不是执行计划，不包含 Red / Green / Refactor 步骤；执行计划属于 `tasks.md`，执行过程属于 `unispec-apply`。它不是 runtime required artifact，不写入 `meta.yaml.artifacts`，不替代 `system-tests.md`，也不替代最终 `test-report.md`。

## 使用规则

- 仅当 `tasks.md` 的 `## 实现模式` 为 `tdd` 时生成或更新。
- 每个测试用例块使用稳定编号 `UT-001`、`UT-002`。
- 每个 `UT-*` 必须能追溯到 design 点、tasks 小 task 和至少一个 `ST-*` 系统测试用例。
- 每个 `UT-*` 必须写出目标行为、Test / Modify / Reference 文件、具体测试代码蓝图、测试数据 / fixture / mock 边界、运行命令、精确 Red 失败原因、Green 通过条件和 Refactor 守卫。
- `unit-tests.md` 中的测试代码是蓝图，`unispec-plan` 不把它写入真实测试文件；实际写入由 `unispec-apply` 在 Red task 执行。
- Green 通过条件和 Refactor 守卫只写判定条件，不记录执行结果。
- fixture、mock 和测试辅助代码边界必须写清楚；避免测试 mock 行为而不是真实行为。
- 如果无法写出具体测试代码、命令或预期 Red 失败原因，暂停并回到 design / plan 补充信息，不生成空壳。
- 不包含 commit 步骤；提交由用户明确触发。
- 没有内容的小节写 `无`，不要保留占位文本。

## 固定结构

~~~md
# <change-id> 单元测试设计

## 测试总览

<说明本文件服务的 TDD 范围，以及与 design.md / system-tests.md / tasks.md 的关系。>

## 单元测试用例

### UT-001 <具体行为名称>

**目标行为**

<一句话说明该单元测试验证的单一行为。>

**关联**

- Design: <design.md 中的标题 / 决策点>
- 系统测试用例: ST-001
- Tasks: 1.1 Red / 1.2 Green / 1.3 Refactor

**Files**

- Test: `<测试文件路径>`
- Modify: `<生产代码文件路径>`
- Reference: `<相关 design / existing test / helper 路径；没有则写“无”>`

**测试代码蓝图**

~~~typescript
test('<清晰描述一个行为>', async () => {
  // 写出计划中的测试代码。这里是蓝图，不在 plan 阶段写入真实文件。
});
~~~

**测试数据 / Fixture / Mock 边界**

- <允许的测试数据、fixture、helper、mock；没有则写“无”>

**运行命令**

`<精确测试命令，例如 pnpm vitest run src/path/file.test.ts -t "行为名">`

**预期 Red 失败**

- 失败测试名: `<测试名>`
- 关键错误 / 断言差异: `<错误信息 / 断言差异>`
- 失败原因: <待实现行为缺失；不是语法错误、导入错误、环境错误或无关回归>

**Green 通过条件**

- <该测试通过时必须满足的条件；不记录执行结果>

**Refactor 守卫**

- <重构后必须继续成立的测试或行为约束；不得新增行为，不得扩大 mock，不得让 ST-001 的验证路径失效>

### UT-002 <具体行为名称>

**目标行为**

<一句话说明该单元测试验证的单一行为。>

**关联**

- Design: <design.md 中的标题 / 决策点>
- 系统测试用例: ST-002
- Tasks: 1.4 Red / 1.5 Green / 1.6 Refactor

**Files**

- Test: `<测试文件路径>`
- Modify: `<生产代码文件路径>`
- Reference: `<相关 design / existing test / helper 路径；没有则写“无”>`

**测试代码蓝图**

~~~typescript
test('<清晰描述一个行为>', async () => {
  // 写出计划中的测试代码。这里是蓝图，不在 plan 阶段写入真实文件。
});
~~~

**测试数据 / Fixture / Mock 边界**

- <允许的测试数据、fixture、helper、mock；没有则写“无”>

**运行命令**

`<精确测试命令>`

**预期 Red 失败**

- 失败测试名: `<测试名>`
- 关键错误 / 断言差异: `<错误信息 / 断言差异>`
- 失败原因: <待实现行为缺失；不是语法错误、导入错误、环境错误或无关回归>

**Green 通过条件**

- <该测试通过时必须满足的条件；不记录执行结果>

**Refactor 守卫**

- <重构后必须继续成立的测试或行为约束；不得新增行为，不得扩大 mock，不得让 ST-002 的验证路径失效>

## 测试辅助边界

- <允许新增或修改的 fixture、helper、mock、测试数据；没有则写“无”>

## 不纳入单元测试的内容

- <不适合单元测试覆盖的内容、原因和后续验证方式；没有则写“无”>
~~~

## 写作边界

- `unit-tests.md` 只描述计划中的单元测试用例蓝图，不记录执行步骤或执行后的最终测试报告。
- Red 阶段的失败必须来自待实现行为缺失；语法错误、环境错误或无关回归不算有效 Red。
- Red 的 Expected 不能只写 FAIL，必须写失败测试名、关键错误信息 / 断言差异和原因解释。
- Green 通过条件和 Refactor 守卫只是判定条件，不是执行记录。
- 不写 Red / Green / Refactor 执行步骤，不替代 `tasks.md` 或 `unispec-apply`。
- 不写 commit 步骤。
- 最终测试执行汇总写入 `test-report.md`，不回填到本文件。
