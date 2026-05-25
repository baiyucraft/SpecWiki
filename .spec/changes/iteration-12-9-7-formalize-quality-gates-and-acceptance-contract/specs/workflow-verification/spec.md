## ADDED Requirements

### Requirement: 当前 acceptance harness 必须区分 primary gate 与 baseline guard
系统 MUST 将当前 acceptance harness 明确分为 `primary gate` 与 `baseline guard`。`storybook + dagger` MUST 作为当前 primary gate 样本；`run-test-projects.mjs` 的完整项目集 `init` 与 `test-wiki-lifecycle.mjs` MUST 作为 baseline regression guard。系统 MUST NOT 把这两类入口继续混成一个没有优先级的“大测试集合”。

#### Scenario: child change 进入正式验收
- **WHEN** 某个 child change 进入正式验收阶段
- **THEN** 系统 MUST 先给出 `storybook + dagger` 的 primary gate 结果
- **THEN** 系统 MUST 再给出完整项目集 `init` 与 lifecycle baseline 结果

#### Scenario: 批量项目集不等同于全量达标承诺
- **WHEN** 系统执行 `run-test-projects.mjs` 的完整项目集 `init`
- **THEN** 该结果 MUST 作为 baseline regression guard
- **THEN** 系统 MUST NOT 自动将其解释为“19 项目全部质量达标”的正式承诺

### Requirement: acceptance harness 必须固定正式命令与阻断语义
系统 MUST 为当前 acceptance harness 固定正式命令入口与阻断语义。当前至少 MUST 包含 `node scripts/run-test-projects.mjs`、`node scripts/test-wiki-lifecycle.mjs` 与对应 crate/workflow 自动化测试。若 primary gate 样本、formal gates 或 baseline guard 失败，系统 MUST 能明确指出是哪一类 gate 阻断了收口。

#### Scenario: primary gate 失败时输出阻断来源
- **WHEN** `storybook` 或 `dagger` 在当前 acceptance harness 中失败
- **THEN** 系统 MUST 将该结果明确标记为 `primary gate` 失败
- **THEN** 系统 MUST 不得仅以零散测试日志代替正式 gate 结论

#### Scenario: lifecycle baseline 失败时输出 baseline guard 结论
- **WHEN** `node scripts/test-wiki-lifecycle.mjs` 失败
- **THEN** 系统 MUST 将该结果明确标记为 `baseline guard` 失败
- **THEN** 系统 MUST 指出该失败与 primary gate 的区别，而不是混为单一质量结论
