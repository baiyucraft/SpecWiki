---
title: CLI
description: spec-wiki 对外 CLI 命令和使用边界
updated: 2026-05-25
owner: docs
---

# CLI

## 适用场景

使用者通过 `spec-wiki` CLI 完成两类事情：

- 安装宿主 bootstrap 资产。
- 初始化、查询、同步和刷新 repo-local knowledge runtime。

## 包入口或命令

```text
spec-wiki
```

包入口以 `packages/spec-wiki/package.json` 的 `bin.spec-wiki` 为准。

## Bootstrap 命令

```bash
spec-wiki init [--tool <host> | --tools <host1,host2>] [--repo-root <path>] [--no-interactive]
```

用途：安装 Codex、Claude、CodeBuddy 等宿主资产。它不构建 runtime。

## Runtime 命令

```bash
spec-wiki wiki init [--repo-root <path>] [--bridge-stdio]
spec-wiki wiki status [--repo-root <path>]
spec-wiki wiki update [--repo-root <path>] [--bridge-stdio]
spec-wiki wiki query [--repo-root <path>] --term <text>
spec-wiki wiki query [--repo-root <path>] <query text>
spec-wiki wiki sync [--repo-root <path>]
spec-wiki wiki rebuild [--repo-root <path>] [--bridge-stdio]
```

## 参数或配置

| 名称 | 说明 |
| --- | --- |
| `--repo-root` | 目标仓库根目录 |
| `--tool` / `--tools` | bootstrap 目标宿主 |
| `--no-interactive` | 禁用交互选择 |
| `--bridge-stdio` | 长流程 runtime 命令的 stdio 桥接输出 |
| `--term` | query 查询词 |

## 返回值或效果

- `init` 写入宿主资产。
- `wiki init / update / sync / rebuild` 更新 repo-local runtime 产物。
- `wiki status / query` 返回当前状态或查询结果。

## Query 稳定合同

`wiki query` 的公开合同以 README、release 文档、CLI 源码和测试为事实来源。本页只记录长期使用边界：

- 查询词通过 `--term <text>` 或位置参数传入。
- 输出面向 Agent 快速定位文件、模块、符号、knowledge pages 和调用路径。
- 查询结果应保留可解释的路径、标题、命中摘要和相关性线索。
- 查询不是通用聊天接口，也不替代 runtime 初始化、同步或更新。

## 限制与边界

- 当前正式发布目标是 Windows x64。
- `query` 需要 `--term` 或位置参数查询词。
- `sync` 只同步受管 `.wiki` 页面编辑，不替代 `update`。
- `rebuild` 是显式全量重建，不是普通 `update` 的别名。
- `spec-wiki init` 与 `spec-wiki wiki init` 是两个不同入口：前者安装宿主资产，后者构建 repo-local knowledge runtime。
