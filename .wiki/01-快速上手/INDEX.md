---
title: 01-快速上手
description: SpecWiki Lite 的安装、初始化和日常使用入口
updated: 2026-07-29
owner: docs
---

# 01-快速上手

本栏目面向首次使用 SpecWiki Lite 的维护者和 Codex。目标是用最短路径建立可版本控制的 Wiki 和 change 工作流。

## 项目定位

- npm package 和可执行命令均为 `spec-wiki-lite`。
- 产品是纯 TypeScript 工具，只管理 `.wiki`、`.spec` 和 `.agents/skills`。
- `.wiki/` 保存正式项目文档；`.spec/` 保存 change 阶段与证据；`.agents/skills` 是唯一 Skills 落点。
- 产品只支持 Codex。Agent 在需要实现细节时直接读取当前源码，不维护代码扫描或隐藏数据层。

## 最短路径

```bash
npm install -g spec-wiki-lite
spec-wiki-lite init --host codex
spec-wiki-lite status
```

从源码参与开发时：

```bash
pnpm install
pnpm run test
pnpm run lint
pnpm run build
```

## CLI 范围

| 命令 | 用途 |
| --- | --- |
| `spec-wiki-lite init [path] [--host codex] [--language zh\|en] [--force] [--no-codegraph] [--no-aoci] [--json]` | 初始化 Wiki、change 与 Skills，并默认准备固定版本 CodeGraph/AOCI；延后时返回 not ready |
| `spec-wiki-lite update [--force] [--tools] [--json]` | 同步资产；仅 `--tools` 修复或升级外部工具 |
| `spec-wiki-lite status [--json]` | 检查 Wiki、Skills 和 active changes |
| `spec-wiki-lite show <change-id> [--artifact <artifact>] [--json]` | 查看 change 摘要或单个 artifact |
| `spec-wiki-lite validate <change-id> [--strict] [--json]` | 校验阶段、artifact 和归档证据 |
| `spec-wiki-lite update [--force] [--json]` | 同步 package 管理的基线与 Skills |
| `spec-wiki-lite archive <change-id>` | 将验证通过的 change 原子归档 |

## 外部认知工具

CodeGraph `1.6.0` 用于当前符号、调用链、影响范围与受影响测试；AOCI-CODE `0.1.0-rc12` 用于长期系统语义、职责、约束与跨会话认知。普通 init 默认准备两者，`--no-codegraph` / `--no-aoci` 只延后一次且项目不 ready。`status` 调用官方只读健康接口；声明数据库 source 后，AOCI Database Cognition 才成为门禁。Lite 不复制工具状态、数据库业务行或凭据，也不把外部 runtime 与本机状态打包。

## 推荐阅读路径

1. [00-环境准备](./00-环境准备.md)
2. [01-启动项目](./01-启动项目.md)
3. [02-构建项目](./02-构建项目.md)
4. [03-常见问题](./03-常见问题.md)
5. [02-开发指南](../02-开发指南/INDEX.md)
6. [03-模块指南](../03-模块指南/INDEX.md)
