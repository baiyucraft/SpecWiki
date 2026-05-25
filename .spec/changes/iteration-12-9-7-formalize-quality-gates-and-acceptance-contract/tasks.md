## 1. Gate Contract 收口

- [x] 1.1 定义最小正式质量闸门，固定 `artifact validity / restore validity / query route contract / status-recommended-action stability`。
- [x] 1.2 定义 failure-to-gate 映射，明确什么失败会阻断 child change 收口。
- [x] 1.3 收紧本轮非目标：不做性能优化专项、不做 provider 稳定性攻坚、不承诺 19 项目全量达标、不做 dashboard 或平台化测试治理系统。

## 2. Acceptance Harness 设计

- [x] 2.1 固定当前 `primary gate` 与 `baseline guard` 的角色：`storybook + dagger` 为 primary gate，完整项目集 `init` 与 lifecycle 为 baseline guard。
- [x] 2.2 固定正式命令入口与输出口径，至少覆盖 `run-test-projects.mjs`、`test-wiki-lifecycle.mjs` 与 reference fidelity 报告。
- [x] 2.3 定义 capability-to-test matrix，要求每个 capability 至少映射到 `model/schema`、`artifact/recovery`、`workflow` 与 `sample` 测试面。

## 3. Spec 与脚本对齐

- [x] 3.1 让 `workflow-verification` 与 `reference-fidelity-reporting` 对齐新的 gate contract 与 acceptance harness 边界。
- [x] 3.2 为后续脚本实现列出需要补的结构化输出、阻断语义与 gate 汇总变更点。
- [x] 3.3 单独检查 [COMMENTING.md](/E:/project/!byAI/spec-wiki/COMMENTING.md) 对脚本、测试与 gate 诊断注释的要求。

## 4. 验收规划

- [x] 4.1 规划 `storybook + dagger` 的 primary gate 验收脚本与通过条件。
- [x] 4.2 规划完整项目集 `init` 与 lifecycle baseline guard 的最小阻断条件。
- [x] 4.3 规划后续 child changes 如何复用这套 quality gates 与 acceptance contract。
