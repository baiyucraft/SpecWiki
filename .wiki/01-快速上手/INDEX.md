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
| `spec-wiki-lite init [path] [--host codex] [--language zh\|en] [--force] [--no-codegraph] [--json]` | 初始化所选语言 Wiki、配置、change 目录、八个 Skills，并默认准备外部 CodeGraph |
| `spec-wiki-lite status [--json]` | 检查 Wiki、Skills 和 active changes |
| `spec-wiki-lite show <change-id> [--artifact <artifact>] [--json]` | 查看 change 摘要或单个 artifact |
| `spec-wiki-lite validate <change-id> [--strict] [--json]` | 校验阶段、artifact 和归档证据 |
| `spec-wiki-lite update [--force] [--json]` | 同步 package 管理的基线与 Skills |
| `spec-wiki-lite archive <change-id>` | 将验证通过的 change 原子归档 |

## CodeGraph

初始化默认尝试执行全局 `@colbymchenry/codegraph` 安装、Codex 用户级 MCP 配置和项目级 `codegraph init`。CodeGraph 失败只返回 warning，核心 Wiki/.spec 初始化仍然成功；离线或 CI 环境使用 `--no-codegraph`。`status` 只检查 `.codegraph` 是否存在，不执行外部命令。CodeGraph 是外部只读分析工具，不是 Lite 自己的索引或知识图谱，数据库不会进入发布包。

## 推荐阅读路径

1. [00-环境准备](./00-环境准备.md)
2. [01-启动项目](./01-启动项目.md)
3. [02-构建项目](./02-构建项目.md)
4. [03-常见问题](./03-常见问题.md)
5. [02-开发指南](../02-开发指南/INDEX.md)
6. [03-模块指南](../03-模块指南/INDEX.md)
