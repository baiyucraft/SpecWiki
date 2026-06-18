## Context

当前仓库已完成迭代 0-3，`Repository Scan -> ModuleTree -> Page Planner -> Wiki Assembler` 主路径稳定运行。但测试项目集分析（见 `.spec/archive/2026-03-08-iteration-3-incremental-runtime/test-project-analysis.md`）明确指出：当前主差异集中在 scanner / hierarchy / planner 质量，而非 runtime 主链错误。

具体问题：

- `crates/wiki-core/tests/fixtures/` 下的测试仓库被递归扫描并提升为正式模块，产生 20+ 个无意义模块页。
- `eslint.config.mjs`、`vitest.config.mjs` 各自成为独立模块页。
- `.spec` 变更管理产物目录被提升为模块。
- `wiki-core` 被标为 `infrastructure`（实际是 Rust 核心引擎），`codebuddy` 被标为 `frontend-app`（实际是 Agent 接入层）。
- wiki-core 模块页的关键源码全部来自 `tests/fixtures/`，真正的核心源码一个都没出现。

当前实现的排除规则覆盖面远不及参考实现：

- `deepwiki-rs`：140+ 条排除规则，25 种 CodePurpose 分类，显式排除 `__tests__`、`__mocks__`、`__fixtures__`，基于路径模式 + 文件名模式 + 扩展名的三层分类，importance scoring 综合 location / size / type / connectivity。
- `CodeWiki`：140+ 条 ignore pattern，显式排除 `tests`、`test`、`Tests`、`Test`、`examples`，基于 AST 的模块分类，entry point 检测覆盖 60+ 关键词。
- 当前 `wiki-core`：`should_ignore_dir()` 只有 16 条硬编码规则，`module_kind()` 只有 7 个分支且纯依赖标签，`key_source_score()` 对 fixture 路径的惩罚不够。

设计约束：

- 本迭代只修 scanner / hierarchy 事实层硬伤，不碰 steering 配置（迭代 5）、editable runtime（迭代 6）或 LLM 增强（迭代 7）。
- 遵守 `AGENTS.md` 分层规则：`Repository Scan -> ModuleTree -> Page Planner -> Wiki Assembler` 主路径不变。
- 不引入新的外部依赖。
- 修改必须向后兼容：已有的 `WikiState`、`MetadataMapper`、IPC 协议不做 breaking change。

## Goals / Non-Goals

**Goals:**

- 修复 scanner 层的噪声过滤缺陷：嵌套仓库、fixture 目录、非代码产物目录。
- 引入 test 路径文件降权标记，让 hierarchy 和 generation 层可以消费。
- 抑制单文件模块提升。
- 升级 module kind 分类为多维信号综合判断。
- 修正关键源码选择信号，确保核心实现文件优先。
- 测试项目集中高噪声问题显著收敛。

**Non-Goals:**

- 不引入 repo 级 steering 配置（迭代 5 范围）。
- 不修改 page_id / page_path 生成规则（迭代 5 范围）。
- 不修改 managed section、sync、editable runtime 相关逻辑（迭代 6 范围）。
- 不引入 LLM 辅助判断（迭代 7 范围）。
- 不修改 Agent 层、IPC 协议或外部 API。
- 不做 planner 的页面标题、父子关系规则收口（迭代 5 范围）。

## Decisions

### 决策 1：嵌套仓库检测采用 `.git` 目录 + workspace 成员白名单策略

在 `should_ignore_dir()` 阶段，对每个子目录检查是否包含 `.git` 目录。如果包含且该目录不属于当前仓库的 workspace 成员，则视为嵌套仓库并跳过。

对于 `tests/fixtures/` 下的测试仓库，它们通常包含 manifest 文件（如 `Cargo.toml` + `src/`）但不一定有 `.git`。因此补充一条规则：如果目录位于已知 fixture 路径模式下（`tests/fixtures/`、`test-data/` 等），直接跳过，不需要检测 `.git`。

备选方案：

- 方案 A：只检测 `.git` 目录
  - 否决原因：测试 fixture 仓库通常不含 `.git`，无法覆盖 `spec-wiki` 自身的核心问题。
- 方案 B：检测所有含 manifest 的子目录
  - 否决原因：会误排除 workspace 成员（如 `crates/wiki-core` 本身含 `Cargo.toml`）。

### 决策 2：fixture / test 目录排除采用路径模式匹配，而非 AST 分析

在 `should_ignore_dir()` 中新增 fixture 路径模式列表：`fixtures`、`__fixtures__`、`test-data`、`testdata`、`test_data`、`mock-data`、`mocks`、`__mocks__`。这些目录整体跳过。

对于 `tests/`、`test/`、`spec/`、`__tests__/` 等目录，不整体排除（其中可能有真实测试源码），但对其下的文件标记 `"test-file"` tag，供后续降权使用。

参考 `deepwiki-rs` 的做法：它在 config 中显式列出 `__tests__`、`__mocks__`、`__fixtures__` 为排除目录，同时通过 `is_test_file()` 和 `is_test_directory()` 对测试文件做标记。

备选方案：

- 方案 A：整体排除所有 `tests/` 目录
  - 否决原因：测试源码本身是有价值的仓库事实，不应完全排除。
- 方案 B：基于 AST 分析判断文件是否为测试
  - 否决原因：当前 tree-sitter 只覆盖 JS/TS/Python，覆盖面不够，且增加复杂度。

### 决策 3：非代码产物目录排除采用硬编码列表 + 后续 steering 扩展预留

在 `should_ignore_dir()` 中新增非代码产物目录列表：`.spec`、`.github`、`.gitlab`、`.circleci`、`.husky`、`coverage`、`.nyc_output`、`examples`。

`vendor` 目录特殊处理：如果当前仓库存在 `go.mod`（Go 仓库），则保留 `vendor/`；否则排除。

硬编码列表在本迭代足够，steering 配置在迭代 5 引入后可以覆盖和扩展。

备选方案：

- 方案 A：现在就引入配置文件
  - 否决原因：steering 配置是迭代 5 的范围，本迭代不应提前引入。

### 决策 4：单文件模块抑制在 hierarchy 层的模块提升评分中实现

在 `discover_meaningful_top_level_roots()` 的评分逻辑中，新增规则：如果候选模块根路径下只包含 1 个文件且没有子目录，则直接跳过提升，该文件归入父模块。

这比在 planner 层过滤更合理，因为问题的根源在 hierarchy 层的模块发现，而不是页面规划。

备选方案：

- 方案 A：在 planner 层过滤单文件模块
  - 否决原因：模块树中仍然存在噪声节点，影响 query、metadata 和后续迭代。

### 决策 5：module kind 分类升级为多维信号综合判断

重写 `module_kind()` 函数，从当前的 7 个纯标签分支升级为综合判断：

1. 检查 manifest 类型：`Cargo.toml` 中的 `[lib]` / `[[bin]]`、`package.json` 中的 `main` / `bin`、`go.mod` 等。
2. 检查目录结构模式：是否有 `src/`、`cmd/`、`lib/` 等标准源码目录。
3. 检查入口文件：`main.rs`、`main.go`、`index.ts` 等。
4. 检查模块标签：frontend / backend / infrastructure 等已有标签。
5. 检查目录名和 manifest 关键词：`agent`、`adapter`、`plugin`、`cli`、`tool` 等。

优先级：manifest 声明 > 入口文件 > 目录结构 > 标签 > 默认值。

新增 kind 值：`library`（有 `[lib]` 声明或纯库结构）、`cli-tool`（有 `[[bin]]` 或 `bin` 字段 + main 入口）。

备选方案：

- 方案 A：引入 LLM 辅助分类
  - 否决原因：迭代 7 范围，本迭代只做 deterministic 规则。
- 方案 B：保持当前标签分支，只调整优先级
  - 否决原因：当前分支逻辑无法区分 library / cli-tool / application，信号维度不够。

### 决策 6：关键源码选择信号修正在 `key_source_score()` 中实现

在现有评分函数中新增：

- fixture 路径惩罚：路径包含 `fixtures/`、`test-data/`、`testdata/`、`mock-data/` 的文件，施加 -1000 惩罚（与现有 low-signal 惩罚对齐）。
- test 路径降权：路径包含 `tests/`、`test/`、`spec/`、`__tests__/` 的文件，施加 -50 惩罚（不完全排除，但显著降低优先级）。
- 消费 scanner 新增的 `"test-file"` tag：如果文件带有该 tag，额外施加 -30 惩罚。

这样 `src/domain/*.rs` 的基础分 40（source kind）+ 25（language）+ 15（path signal）= 80，而 `tests/fixtures/**/*.rs` 的分数为 40 + 25 - 1000 = -935，确保核心源码优先。

备选方案：

- 方案 A：在 scanner 层直接排除 test 文件
  - 否决原因：测试文件仍是有价值的仓库事实，应该出现在 `RepoFacts.files` 中，只是不应主导关键源码选择。

## Risks / Trade-offs

- [硬编码排除列表可能遗漏某些项目的特殊目录] → 迭代 5 的 steering 配置将提供用户自定义扩展能力。本迭代的硬编码列表覆盖最常见的模式。
- [嵌套仓库检测可能误排除合法的 workspace 成员] → 通过 workspace 成员白名单保护，只有不在 workspace 中的含 `.git` 子目录才被排除。
- [单文件模块抑制可能在极端情况下排除有价值的独立模块] → 阈值设为"只含 1 个文件且无子目录"，这是一个非常保守的条件。如果一个目录只有一个文件，它几乎不可能是一个有意义的独立模块。
- [module kind 分类的 manifest 解析增加了 scanner 的复杂度] → 只做轻量级的 manifest 字段检查（检查 `[lib]` / `[[bin]]` 是否存在），不做完整的 TOML/JSON 解析。
- [test 路径降权可能在测试密集型项目中过度抑制测试模块] → 降权值 -50 是温和的，不会完全排除测试文件，只是确保核心实现文件优先。fixture 路径的 -1000 惩罚更激进，但 fixture 本身就不应出现在关键源码中。

## Open Questions

- `examples/` 目录是否应该被完全排除？某些项目的 examples 目录包含有价值的示例代码。当前决策是排除，但可以在迭代 5 的 steering 配置中允许用户覆盖。
- 是否需要对 `docs/` 目录做特殊处理？当前 scanner 已经将 `.md` 文件标记为 `"docs"` kind，但 `docs/` 目录本身没有被排除。如果 `docs/` 下有源码文件（如文档站点的构建脚本），它们仍会被扫描。
