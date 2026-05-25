## 1. Scanner 噪声过滤增强

- [x] 1.1 在 `crates/wiki-core/src/repo/scanner.rs` 的 `should_ignore_dir()` 中新增 fixture / test-data 目录排除规则，覆盖 `fixtures`、`__fixtures__`、`test-data`、`testdata`、`test_data`、`mock-data`、`mocks`、`__mocks__` 以及 `tests/fixtures/`、`test/fixtures/`、`spec/fixtures/` 路径模式
- [x] 1.2 在 `should_ignore_dir()` 中新增嵌套仓库检测：子目录包含 `.git` 目录且不属于当前仓库 workspace 成员时跳过；需要将 workspace_roots 信息传入 `visit_dir()` 调用链
- [x] 1.3 在 `should_ignore_dir()` 中新增非代码产物目录排除：`.spec`、`.github`、`.gitlab`、`.circleci`、`.husky`、`coverage`、`.nyc_output`、`examples`；`vendor` 目录在存在 `go.mod` 时保留
- [x] 1.4 在 `detect_tags()` 中新增 test 路径文件降权标记：位于 `tests/`、`test/`、`spec/`、`__tests__/`、`__test__/` 路径下的源码文件添加 `"test-file"` tag

## 2. 单文件模块抑制

- [x] 2.1 在 `crates/wiki-core/src/repo/hierarchy.rs` 的 `discover_meaningful_top_level_roots()` 评分逻辑中，新增单文件模块抑制：候选模块根路径下只含 1 个文件且无子目录时，跳过提升，该文件归入父模块
- [x] 2.2 在模块提升评分中，对候选路径下带有 `"test-file"` tag 的文件施加降权（如 entry_points 加分减半、source_files 计数排除 test-file），避免 test / fixture 文件数量主导提升评分

## 3. Module Kind 分类升级

- [x] 3.1 重写 `crates/wiki-core/src/repo/hierarchy.rs` 的 `module_kind()` 函数，从纯标签分支升级为多维信号综合判断：manifest 类型（`Cargo.toml` 的 `[lib]`/`[[bin]]`、`package.json` 的 `main`/`bin`、`go.mod`）→ 入口文件 → 目录结构 → 模块标签 → 目录名/manifest 关键词 → 默认值
- [x] 3.2 新增 kind 值 `library` 和 `cli-tool`，补充到 `module_kind()` 的返回值集合中；确保现有 kind 值（`application`、`backend-service`、`frontend-app`、`infrastructure`、`workspace-member`、`module-group`、`module`）的判断逻辑在新分支中保持兼容
- [x] 3.3 为 manifest 类型检查实现轻量级解析辅助函数（如 `has_cargo_lib_section()`、`has_cargo_bin_section()`、`has_package_json_bin()`），放在 scanner 或 hierarchy 的辅助模块中

## 4. 关键源码选择信号修正

- [x] 4.1 在 `crates/wiki-core/src/generation/context.rs` 的 `key_source_score()` 中新增 fixture 路径惩罚：路径包含 `fixtures/`、`test-data/`、`testdata/`、`mock-data/` 时施加 -1000
- [x] 4.2 在 `key_source_score()` 中新增 test 路径降权：路径包含 `tests/`、`test/`、`spec/`、`__tests__/` 时施加 -50
- [x] 4.3 在 `key_source_score()` 中消费 scanner 新增的 `"test-file"` tag：带有该 tag 的文件额外施加 -30 惩罚

## 5. 自动化验证

- [x] 5.1 新增 scanner 噪声过滤测试（`crates/wiki-core/tests/`）：构造含嵌套仓库（子目录有 `.git`）、fixture 目录、非代码产物目录的测试仓库，验证 `scan_repo()` 的 `RepoFacts.files` 不包含被排除目录下的文件
- [x] 5.2 新增单文件模块抑制测试：构造含单文件候选模块的测试仓库，验证 `build_module_tree()` 不将其提升为独立模块
- [x] 5.3 新增 module kind 分类测试：构造含 `Cargo.toml [lib]`、`Cargo.toml [[bin]]`、`package.json bin`、纯基础设施目录等场景的测试仓库，验证 `module_kind()` 返回正确的 kind 值
- [x] 5.4 新增关键源码选择测试：构造含大量 fixture 文件和少量核心源码的模块，验证 `select_key_sources()` 优先选择核心源码
- [x] 5.5 跑通仓库自动化测试入口（`pnpm test`），修复本轮变更引入的回归

## 6. 测试项目集分析

- [x] 6.1 对当前仓库 `E:\project\!byAI\spec-wiki` 执行 `init`，重点验证：fixture 目录不再被提升为模块、`eslint.config.mjs` / `vitest.config.mjs` 不再生成独立模块页、`.spec` 不再被提升为模块、`wiki-core` kind 不再为 `infrastructure`、`codebuddy` kind 不再为 `frontend-app`、关键源码列表包含 `src/domain/` 或 `src/workflows/` 下的文件
- [x] 6.2 对 `E:\project\aLocal` 执行 `init`，将结果与 `tmp/reference-aLocal` 对照，确认页面结构和 metadata 字段无退化
- [x] 6.3 对其余测试项目集全量执行 `init` 并记录差异，重点关注：Go 扁平仓库的 `vendor` 保留、大型 monorepo（storybook、dagger）的模块数量变化、有 reference 的项目（pinia、restaurant-app）与参考产物的对照

## 7. 注释合规

- [x] 7.1 按 `COMMENTING.md` 检查本轮新增或修改文件的注释，覆盖公共接口、核心类型、关键流程和测试场景注释，并修正不符合规范的注释
