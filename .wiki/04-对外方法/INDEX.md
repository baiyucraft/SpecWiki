---
title: 04-对外方法
description: spec-wiki 的 CLI、配置、运行时产物和公开契约入口
updated: 2026-05-25
owner: docs
---

# 04-对外方法

本栏目面向使用者和宿主接入层，说明 `spec-wiki` 对外暴露的 CLI、配置和运行时产物边界。内部实现细节回链到 [03-模块指南](../03-模块指南/INDEX.md)。

## CLI / 命令

| 命令 | 用途 |
| --- | --- |
| `spec-wiki init` | 安装 Codex、Claude、CodeBuddy 等宿主 bootstrap 资产 |
| `spec-wiki wiki init` | 初始化 repo-local knowledge runtime |
| `spec-wiki wiki status` | 查看 runtime readiness、stale、blocker 和下一步动作 |
| `spec-wiki wiki update` | 源码变更后增量刷新 runtime |
| `spec-wiki wiki query` | 查询文件、模块、符号、knowledge pages 和调用路径 |
| `spec-wiki wiki sync` | 将受管 `.wiki` 页面编辑同步回 runtime state、metadata 与 cache |
| `spec-wiki wiki rebuild` | 显式全量重建 knowledge runtime |

## API / 对外接口

当前没有独立 public SDK 页面。对外入口以 CLI 和 npm package `spec-wiki` 为主；包入口见 `packages/spec-wiki/package.json`。

## 配置

| 配置 | 用途 | 默认值或说明 |
| --- | --- | --- |
| `wiki.dev.yaml` | 本地 provider / retry / timeout / backoff 调试配置 | 测试开发时优先使用 |
| `.wiki/` runtime 目录 | repo-local knowledge runtime 与页面投影 | 由 `spec-wiki wiki init` / `update` / `rebuild` 生成 |
| `.spec/` | UniSpec change artifact 与 archive | 由 UniSpec skills / CLI 维护 |

## 页面

| 页面 | 用途 |
| --- | --- |
| [00-CLI](./00-CLI.md) | CLI 命令、适用场景和常见用法 |
| [01-配置与运行时产物](./01-配置与运行时产物.md) | 配置文件、`.wiki` runtime 分层和 docs-only Wiki 的边界 |

## 兼容性

当前处于测试开发阶段，不要求保留旧实现兼容层。公开契约变化应先进入 UniSpec change artifact，再同步本栏目。
