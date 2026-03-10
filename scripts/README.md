# Scripts Layout

根级 `scripts/*.mjs` 只保留可直接调用的编排入口：

- `build-dist.mjs`
- `publish-packages.mjs`
- `run-tests.mjs`
- `run-test-projects.mjs`
- `test-wiki-lifecycle.mjs`
- `collect-test-project-analysis.mjs`

其中 `run-test-projects.mjs` 和 `test-wiki-lifecycle.mjs` 支持 `--jobs <N>`，默认按 8 个项目并行执行。

内部实现按职责下沉到子目录：

- `build/`
  - 构建和发布共享的路径解析
- `testing/`
  - 测试共享工具和 lifecycle phase wrapper
- `tests/`
  - 根级 Vitest 集成测试
- `templates/`
  - 发布包模板
