# spec-wiki

`spec-wiki` 用来给代码仓库提供一层轻量、可本地运行的 Repo Wiki，方便 Agent 先拿到结构化地图，再去深入读代码。

在 `v0.1.0` 里，它主要做两件事：

- 给 `Codex`、`Claude`、`CodeBuddy` 做 repo 内 bootstrap
- 构建 index-first 的本地 runtime，用来查询文件、模块、符号和调用路径

## 为什么用它

Agent 直接进入大仓库时，通常会先盲搜、盲读，token 花得多，路径也容易走偏。

`spec-wiki` 的价值是先给它一份结构地图：

- 关键代码大概在哪
- 哪些文件和符号彼此相关
- 当前本地 runtime 是 ready、stale，还是需要重新初始化

`v0.1.0` 的目标不是完整生成整套仓库文档，而是先把“第一层仓库地图”做稳定。

## 当前范围

`v0.1.0` 当前正式保证的边界：

- Windows runtime 支持
- 对外公开 CLI：`init`、`status`、`update`、`query`
- `init`、`update` 走 index-only
- `query` 走 index-first

完整 knowledge/page 生成目前还不是 `v0.1.0` 的正式合同。

## 快速开始

### 1. 给宿主做 bootstrap

```bash
spec-wiki init --tool codex --repo-root .
```

这个命令会往仓库里写入宿主相关的受管资产。

当前支持：

- `codex`
- `claude`
- `codebuddy`

### 2. 初始化本地 repo index

```bash
spec-wiki wiki init --repo-root .
```

这个命令会扫描仓库并初始化本地 wiki cache。

### 3. 查看 runtime 是否可用

```bash
spec-wiki wiki status --repo-root .
```

在让 Agent 依赖 repo wiki 之前，先用它看当前状态。

### 4. 查询仓库地图

```bash
spec-wiki wiki query --repo-root . --term "payment flow"
```

也支持位置参数：

```bash
spec-wiki wiki query payment flow
```

### 5. 在源码变更后刷新

```bash
spec-wiki wiki update --repo-root .
```

## 两个不同的 `init`

这是最容易混淆的地方。

### `spec-wiki init`

这是 bootstrap 命令。

它负责安装 repo 内的宿主资产，比如 skill、hook、settings。

它不会构建 wiki runtime。

### `spec-wiki wiki init`

这是 runtime 命令。

它负责扫描仓库并初始化本地 repo wiki cache。

它不会安装宿主 bootstrap 资产。

## 安装或从源码运行

如果你是在当前仓库里直接运行：

```bash
pnpm install
pnpm build
node dist/spec-wiki/bin/spec-wiki.js --help
```

安装后可以直接用：

```bash
spec-wiki --help
```

## 命令说明

### Bootstrap

```bash
spec-wiki init [--tool <host> | --tools <host1,host2>] [--repo-root <path>] [--no-interactive]
```

### Runtime

```bash
spec-wiki wiki init [--repo-root <path>] [--bridge-stdio]
spec-wiki wiki status [--repo-root <path>]
spec-wiki wiki update [--repo-root <path>] [--bridge-stdio]
spec-wiki wiki query [--repo-root <path>] --term <text>
spec-wiki wiki query [--repo-root <path>] <query text>
```

补充：

- `--bridge-stdio` 只适用于 `init`、`update` 这类长流程
- `query` 必须提供 `--term` 或位置参数查询词

## 会生成什么

runtime 会在仓库里写入本地 `.wiki/` 目录。

对 `v0.1.0`，当前最应该依赖的核心产物是：

```text
.wiki/
└─ .cache/
   └─ wiki-cache.db
```

## `v0.1.0` 里 `query` 的稳定合同

对 `query`，建议只稳定依赖这些字段：

- `query_mode`
- `query_trust`
- `recommended_action`
- `matched_pages`
- `provenance_summary`

推荐使用方式：

1. 先用 `query` 缩小搜索空间
2. 找到最相关的文件、模块或符号
3. 需要精确实现细节时，再直接读代码

`query` 的目标是降低搜索成本，不是替代读代码。

## 它适合什么场景

- 给 Agent 一份快速的仓库结构地图
- 减少盲目读文件和 token 浪费
- 判断当前仓库应该先 `init` 还是 `update`
- 帮助定位下一步该读哪些文件和符号

## 协议

本项目采用 `GNU GPL v3.0` 协议。

## TODO

- 正式支持 knowledge/page runtime
- 增强 research 和结果组装
- 扩展更多平台支持
