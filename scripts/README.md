# Scripts Layout

根级 `scripts/*.mjs` 只保留可直接调用的编排入口：

- `build-dist.mjs`
- `publish-packages.mjs`
- `run-tests.mjs`
- `run-test-projects.mjs`
- `test-wiki-lifecycle.mjs`
- `collect-reference-project-reports.mjs`
- `collect-test-project-analysis.mjs`

其中 `run-test-projects.mjs` 和 `test-wiki-lifecycle.mjs` 支持 `--jobs <N>`，默认按 8 个项目并行执行。

当前验收 contract 约定：

- `node scripts/collect-reference-project-reports.mjs storybook dagger`
  - `primary gate`
  - reference fidelity 是正式 gate 输入，但不替代 formal artifact / restore / query / status gates
- `node scripts/run-test-projects.mjs`
  - `baseline guard`
  - 批量项目集 `init` 回归，不等同于 19 项目全量质量达标承诺
- `node scripts/test-wiki-lifecycle.mjs`
  - `baseline guard`
  - 生命周期链路回归

`run-test-projects.mjs` 与 `test-wiki-lifecycle.mjs` 额外支持 `--json-summary`，用于输出结构化 gate summary。

内部实现按职责下沉到子目录：

- `build/`
  - 构建和发布共享的路径解析
- `testing/`
  - 测试共享工具和 lifecycle phase wrapper
- `tests/`
  - 根级 Vitest 集成测试
- `templates/`
  - 发布包模板
