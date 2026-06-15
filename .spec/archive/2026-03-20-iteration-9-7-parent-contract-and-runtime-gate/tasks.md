## 1. Parent Contract

- [x] 1.1 调整 `page_render`、`research_provider` 与 `compose_engine`，让 `Overview / Architecture / DomainIndex` 以及 `decomposition_profile = config_surface` 的 parent KnowledgeUnit 具备 unit-scoped parent research contract，而不是继续绕过到固定骨架 compose。
- [x] 1.2 为 parent compose 引入结构化 child rollup，覆盖 child digest、section/citation digest、diagram digest、key sources 与 readiness，避免 `config_surface` parent unit 和 overview/index 继续只消费 child summary。
- [x] 1.3 调整高层 parent 的 child 汇聚与 compose 输入上卷逻辑，确保证据通过 child rollup 逐层上卷，而不是让 `Overview / Architecture` 直接越过中间 parent 节点读取 deeper leaf 输入。

## 2. Runtime Gate

- [x] 2.1 在现有 runtime/state/cache 主链中持久化 workflow 级与 unit 级 compose-readiness gate，记录 blocked stage、blocked reason、missing dependencies 与恢复状态。
- [x] 2.2 升级 `page_context_cache` 或等价 runtime/cache 路径，为 parent KnowledgeUnit 持久化 child contract 摘要，而不是只留下最小 `source_ids`。
- [x] 2.3 调整 resume/rebuild 逻辑，使可恢复的半完成态不会因为缺少 hard checkpoint 被静默清空，并补对应的 Rust 测试。

## 3. Reporting And Tests

- [x] 3.1 更新 `reference-fidelity-reporting` 与项目分析脚本，使高复用页可以映射回具体高层 `KnowledgeUnit`，并输出 runtime readiness 级诊断。
- [x] 3.2 为高层 parent contract、compose-readiness gate、runtime incomplete 诊断补齐对应层级测试：Rust 测试放 `crates/*/tests/`，脚本/工作区测试放 `scripts/tests/*.test.ts`。
- [x] 3.3 作为独立 task，在本轮 UniSpec tasks 的设计与测试阶段单独执行注释规范检查，确认涉及代码与测试的新增/修改注释符合 [.wiki/02-开发指南/00-代码注释规范.md](E:/project/!byAI/spec-wiki/.wiki/02-开发指南/00-代码注释规范.md)。

## 4. Validation

- [x] 4.1 先跑 `storybook + dagger` 的 init、reference 报告与 lifecycle 专项验证，确认高层父页 contract 与 runtime gate 两条专项都能给出明确结论。
- [x] 4.2 运行 `node scripts/run-test-projects.mjs storybook dagger`，只对 `storybook + dagger` 执行 `init` 专项回归，并输出对应分析报告。
- [x] 4.3 运行 `node scripts/test-wiki-lifecycle.mjs storybook dagger` 完成 `storybook + dagger` 生命周期验证；将专项报告归档到 change 目录下的 `reference-project-reports/`。
