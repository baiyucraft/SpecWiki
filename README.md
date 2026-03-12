# spec-wiki

一个面向代码仓库的 Repo Wiki 原型项目。

它的目标是自动扫描本地代码目录，生成并持续更新 `.wiki/`，让人和 Agent 都能把这份 Wiki 当作项目知识层来使用。Git 只是一种可选元信息来源，不是核心功能的硬前置。当前阶段优先实现 Windows 下的 CodeBuddy Agent 接入。

## 当前状态

- 当前仅支持 Windows
- 当前仅支持 CodeBuddy Agent
- Rust `wiki-core` 负责扫描、生成、更新、查询和同步
- `agents/*` 负责将这些能力暴露成 Agent 工具

## 仓库结构

```text
.
├─ crates/wiki-core/      # Repo Wiki core
│  └─ package.json        # core build / test 脚本入口
├─ agents/codebuddy/      # CodeBuddy Agent
│  └─ src/*.test.ts       # Agent 自身测试
├─ scripts/               # 根级编排入口与测试脚本
│  ├─ build/              # 构建/发布共享路径解析
│  ├─ testing/            # 测试共享工具与 lifecycle phase wrapper
│  └─ tests/              # 根级整体测试：staging / e2e / 工作区检查
├─ openspec/              # proposal / design / tasks
├─ DESIGN.md              # 项目级设计
└─ DESIGN-CORE.md         # core 设计
```

## 运行产物

插件运行在目标仓库时，会写入：

```text
.wiki/
├─ 项目概述.md
├─ 系统架构.md
├─ 核心模块/
├─ wiki.metadata.json
└─ .cache/
```

- `.wiki/*.md` 是正式 Wiki 页面
- `wiki.metadata.json` 是正式索引
- `.wiki/.cache/` 是运行时缓存
- 当前生成会围绕模块层级组织总览页、架构页和模块页

## 编译

```bash
pnpm install
cargo build -p wiki-core --target-dir target
pnpm --dir agents/codebuddy build
pnpm run lint
```

这几组命令都从 workspace 根目录执行，职责不同：

- `cargo build -p wiki-core --target-dir target`
  - 只编译 `wiki-core`
- `pnpm --dir agents/codebuddy build`
  - 只编译 `codebuddy`
- `pnpm run lint`
  - 根级统一检查仓库文件，忽略 `dist/`、`target/` 等生成产物

各模块单独编译后的产物位置：

```text
target/debug/wiki-core.exe
agents/codebuddy/dist/
```

如果要准备发布产物，可以执行：

```bash
pnpm run build
```

根级 `build` 会把可发布内容整理到：

```text
dist/
├─ core/
└─ npm/
```

也就是说：

- 子包仍然自管 `build / test` 脚本
- 实际调用统一从 workspace 根发起
- `agents/codebuddy/dist/` 持有 Agent 自身 bundle
- 根级 `dist/` 只作为发布 staging，固定只保留 `core/` 和 `npm/`

## 开发调试

当前仓库没有单独的 dev server。日常开发主要是：

- 修改 `crates/wiki-core/` 后运行 core 自身测试
- 修改 `agents/codebuddy/` 后运行 Agent 自身的 build / test
- 根级整体测试负责 staging / e2e / 工作区级联验证

常用命令：

```bash
cargo test -p wiki-core --target-dir target
pnpm --dir agents/codebuddy build
pnpm --dir agents/codebuddy test
pnpm run test
```

测试也遵循同样的分层原则：

- `cargo test -p wiki-core --target-dir target`
  - 只跑 core 自身测试
- `pnpm --dir agents/codebuddy test`
  - 只跑 Agent 自身测试
- `pnpm run test`
  - 根级总入口，顺序执行 core 测试、Agent 测试和根级整体测试

说明：

- 当前仓库使用 Cargo workspace，Rust 产物统一输出到根目录 `target/`
- CodeBuddy Agent 默认会在 `target/debug/wiki-core.exe` 查找本地 binary
- 如需手动指定 binary，可设置环境变量 `CODEBUDDY_WIKI_CORE_BIN`
- `pnpm run test` 会依次执行：core 自测、codebuddy 自测、根级 Vitest 整体测试

## Baseline 验收

`Deterministic Structural Baseline` 这一轮的完成口径，不再只看“能不能生成 `.wiki/`”，而是看结构是否稳定：

- `ModuleTree` 需要支持递归层级，而不是只有一层子模块
- 模块页路径需要跟模块祖先链一致，例如 `核心模块/packages/domain/auth.md`
- `wiki.metadata.json` 需要导出页面父子关系、模块层级和页面 provenance
- `query` 需要优先返回页面、模块、源码、关系等结构化命中，Markdown 只作为回退

当前用于这一轮验收的中型 fixture 在 [crates/wiki-core/tests/fixtures/baseline-hierarchy-repo](E:/project/!byAI/spec-wiki/crates/wiki-core/tests/fixtures/baseline-hierarchy-repo)。它覆盖：

- nested workspace 模块：`apps/web`、`packages/domain/auth`、`packages/domain/shared`
- 非 workspace 模块：`spider`
- 基础设施目录：`infra/nginx`

如果你要手工验证 baseline，优先看这些测试：

- [crates/wiki-core/tests/hierarchy/hierarchy_planning.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/tests/hierarchy/hierarchy_planning.rs)
- [crates/wiki-core/tests/repo/repo_scan.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/tests/repo/repo_scan.rs)
- [crates/wiki-core/tests/acceptance/baseline_acceptance.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/tests/acceptance/baseline_acceptance.rs)
- [crates/wiki-core/tests/runtime/query_sync_rebuild.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/tests/runtime/query_sync_rebuild.rs)

# 一些想法

- [ ] 这个wiki是给agent用的 reference是不是应该更方便引用

- [ ] 包的体积小一点

- [ ] 参考Qoder的文件，查看中间产物，看看有什么值得借鉴的

  
