# codebuddy-wiki

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
├─ scripts/tests/         # 根级整体测试：staging / e2e / 工作区检查
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
pnpm --dir crates/wiki-core run build
pnpm --dir agents/codebuddy run build
pnpm run lint
```

这几组命令的职责不同：

- `pnpm --dir crates/wiki-core run build`
  - 只编译 `wiki-core`
- `pnpm --dir agents/codebuddy run build`
  - 只编译 `codebuddy`
- `pnpm run lint`
  - 根级统一检查仓库文件，忽略 `dist/`、`target/` 等生成产物

各模块单独编译后的产物位置：

```text
crates/wiki-core/target/debug/wiki-core.exe
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

- 子包负责“把自己编译好”
- 根级负责“编排全部模块并整理发布产物”

## 开发调试

当前仓库没有单独的 dev server。日常开发主要是：

- 修改 `crates/wiki-core/` 后运行 core 自身测试
- 修改 `agents/codebuddy/` 后运行 Agent 自身的 build / test
- 根级整体测试负责 staging / e2e / 工作区级联验证

常用命令：

```bash
pnpm --dir crates/wiki-core run test
pnpm --dir agents/codebuddy run build
pnpm --dir agents/codebuddy run test
pnpm run test
```

测试也遵循同样的分层原则：

- `pnpm --dir crates/wiki-core run test`
  - 只跑 core 自身测试
- `pnpm --dir agents/codebuddy run test`
  - 只跑 Agent 自身测试
- `pnpm run test`
  - 根级总入口，顺序执行 core 测试、Agent 测试和根级整体测试

说明：

- CodeBuddy Agent 默认会在 `crates/wiki-core/target/debug/wiki-core.exe` 查找本地 binary
- 如需手动指定 binary，可设置环境变量 `CODEBUDDY_WIKI_CORE_BIN`
- `pnpm run test` 会依次执行：core 自测、codebuddy 自测、根级 Vitest 整体测试
