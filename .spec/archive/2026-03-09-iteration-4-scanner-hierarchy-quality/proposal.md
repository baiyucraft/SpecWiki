## Why

当前 scanner / hierarchy 层存在多个 deterministic 事实硬伤，导致模块树无法真实反映仓库结构。测试项目集分析已经把主差异收敛到五类问题：fixture / nested repo 误提升、单文件配置被提升为模块、非代码目录进入模块树、kind 分类过度依赖文件扩展名、关键源码被低信号文件淹没。这些问题直接影响后续迭代（页面拓扑稳定、steering、editable runtime、LLM 增强）的基础质量，必须在进入迭代 5 之前修复。

参考实现 `deepwiki-rs` 使用 140+ 条排除规则和 25 种 CodePurpose 分类，`CodeWiki` 使用 140+ 条 ignore pattern 和基于 AST 的模块分类，两者都对 test / fixture / build artifact 做了显式排除。当前 `wiki-core` 的排除规则和分类逻辑远不及参考实现的覆盖面。

## What Changes

- 增强 scanner 的目录/文件排除规则：新增 nested repo 检测（子目录含 `.git` / `Cargo.toml` + `src/` 等 manifest 组合时识别为嵌套仓库）、fixture / test 目录排除（`tests/fixtures/`、`test-data/`、`__fixtures__/` 等）、非代码产物目录排除（`.spec/`、`.github/`、`examples/` 等可配置）。
- 引入单文件模块抑制逻辑：hierarchy 层在模块提升评分中，对只含单个文件且无子模块的候选节点施加惩罚，阻止 `eslint.config.mjs`、`vitest.config.mjs` 等配置文件独立成为模块页。
- 升级 module kind 分类启发式：从纯文件扩展名判断升级为结合 manifest 类型、目录结构模式、入口文件和模块标签的综合判断，对齐 `deepwiki-rs` 的 CodePurpose 分类思路。
- 修正关键源码选择信号：在 `key_source_score()` 中对 fixture / test 路径施加显式惩罚，确保 `src/domain/`、`src/workflows/` 等核心实现目录下的文件不被 `tests/fixtures/` 淹没。

## Capabilities

### New Capabilities

- `scanner-noise-filter`: 覆盖 nested repo 检测、fixture/test 目录排除、非代码产物目录排除等 scanner 层噪声过滤增强。

### Modified Capabilities

- `repo-hierarchy-model`: 新增单文件模块抑制规则、module kind 分类启发式升级、关键源码选择信号修正。
- `workflow-verification`: 新增 scanner 质量验证场景，覆盖 fixture 排除、单文件模块抑制、kind 分类准确性和关键源码选择的测试项目集分析。

## Impact

- `crates/wiki-core/src/repo/scanner.rs`：`should_ignore_dir()` 和 `should_ignore_file()` 扩展排除规则，新增 `is_nested_repo()` 检测。
- `crates/wiki-core/src/repo/hierarchy.rs`：`module_kind()` 分类逻辑重写，`discover_meaningful_top_level_roots()` 评分调整，新增单文件模块抑制。
- `crates/wiki-core/src/generation/context.rs`：`key_source_score()` 信号修正，fixture / test 路径惩罚。
- `crates/wiki-core/tests/`：新增 scanner 质量测试，扩展 hierarchy 测试覆盖。
- 不涉及 Agent 层、IPC 协议或外部 API 变更。
