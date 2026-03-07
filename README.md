# codebuddy-wiki

一个面向代码仓库的 Repo Wiki 原型项目。

它的目标是自动扫描 Git 仓库，生成并持续更新 `.wiki/`，让人和 Agent 都能把这份 Wiki 当作项目知识层来使用。当前阶段优先实现 Windows 下的 CodeBuddy Agent 接入。

## 当前状态

- 当前仅支持 Windows
- 当前仅支持 CodeBuddy Agent
- Rust `wiki-core` 负责扫描、生成、更新、查询和同步
- `packages/codebuddy` 负责将这些能力暴露成 CodeBuddy 工具

## 仓库结构

```text
.
├─ crates/wiki-core/      # Repo Wiki core
├─ packages/codebuddy/    # CodeBuddy Agent
├─ tests/                 # integration / e2e tests
├─ openspec/              # proposal / design / tasks
├─ DESIGN.md              # 项目级设计
└─ DESIGN-CORE.md         # core 设计
```

## 运行产物

插件运行在目标仓库时，会写入：

```text
.wiki/
├─ 项目概述.md
├─ wiki.metadata.json
└─ .cache/
```

- `.wiki/*.md` 是正式 Wiki 页面
- `wiki.metadata.json` 是正式索引
- `.wiki/.cache/` 是运行时缓存

## 本地开发

```bash
pnpm install
cargo test -p wiki-core
pnpm run test:adapter
pnpm run test:integration
pnpm run test:e2e
```
