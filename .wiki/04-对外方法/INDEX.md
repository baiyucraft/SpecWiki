---
title: 04-对外方法
description: SpecWiki Lite 的 CLI、项目产物和发布合同入口
updated: 2026-07-29
owner: docs
---

# 04-对外方法

SpecWiki Lite 面向 Codex 仓库提供一个 npm CLI 和八个 repo-local Skills。所有命令直接操作当前项目的 `.wiki`、`.spec` 与 `.agents/skills`。

## 公开命令

| 命令 | 用途 |
| --- | --- |
| `spec-wiki-lite init [path]` | 初始化默认中文或显式英文 Wiki、配置、change 目录和 Skills |
| `spec-wiki-lite status` | 检查 Wiki、Skills 和 active changes |
| `spec-wiki-lite show <change-id>` | 查看 change 状态或指定 artifact |
| `spec-wiki-lite validate <change-id>` | 校验 change metadata、stage 和 required artifacts |
| `spec-wiki-lite update` | 按资产所有权规则刷新登记资产 |
| `spec-wiki-lite archive <change-id>` | 严格校验并归档 change |

公开命令没有 namespace 或隐藏高级命令。完整参数见[CLI](./00-CLI.md)。

## 页面

| 页面 | 用途 |
| --- | --- |
| [00-CLI](./00-CLI.md) | 命令、参数、输出和退出码 |
| [01-配置与项目产物](./01-配置与项目产物.md) | Node.js 要求、目录边界、资产所有权和提交规则 |
| [02-v0.2.0发布合同](./02-v0.2.0发布合同.md) | 包身份、双工具认知门禁、制品和发布证据边界 |

## 事实来源

- CLI 与 API：`packages/spec-wiki-lite/src/**`
- 模板与 Skills：`packages/spec-wiki-lite/assets/**`
- 包身份：`packages/spec-wiki-lite/package.json`
- 模块职责：[03-模块指南](../03-模块指南/INDEX.md)

当前处于测试开发阶段，不提供旧命令、旧参数或旧数据格式的兼容别名。
