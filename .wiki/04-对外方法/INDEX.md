---
title: 04-对外方法
description: spec-wiki 的 CLI、配置、运行时产物和公开契约入口
updated: 2026-07-13
owner: docs
---

# 04-对外方法

本栏目面向使用者和宿主接入层，说明 `spec-wiki` 对外暴露的 CLI、配置和运行时产物边界。内部实现细节回链到 [03-模块指南](../03-模块指南/INDEX.md)。

## 初始化入口

| 命令 | 用途 |
| --- | --- |
| `spec-wiki init` | 初始化 SpecWiki，包括宿主 bootstrap、repo-local knowledge runtime 和 Agent 入口 |

## 一级命令

SpecWiki 对用户优先暴露一级命令。`wiki` 和 `governance` 是内部 runtime/domain，不作为普通用户命令 namespace。

| 命令 | 用途 |
| --- | --- |
| `spec-wiki status` | 查看 Wiki runtime、index、knowledge 和 governance readiness |
| `spec-wiki update` | 源码、文档或治理 artifact 变化后刷新 runtime |
| `spec-wiki query <term>` | 查询文件、模块、符号、knowledge pages、projection refs、fallback 结果和结构化治理 refs |
| `spec-wiki validate <change-id>` | 验证指定 change 的治理 artifact |

## 高级维护动作

这些命令保留为 advanced surface，不进入快速上手主路径。

`advanced` 是 help 分组，不是命令前缀；用户不需要输入 `spec-wiki advanced ...`。

| 命令 | 用途 |
| --- | --- |
| `spec-wiki sync` | 将受管 `.wiki` 页面编辑同步回 knowledge/runtime |
| `spec-wiki rebuild` | 显式全量重建 knowledge runtime |
| `spec-wiki changes` | 列出 active governance changes |
| `spec-wiki change <change-id>` | 查看单个 change 摘要 |
| `spec-wiki archive <change-id>` | 默认 dry-run；显式 `--apply` 写入，或用 `--resume <operation-id>` 恢复 |

workspace validate、`doctor`、`repair` 和 `trace` 尚未注册为当前 CLI 命令。

## API / 对外接口

当前没有独立 public SDK 页面。对外入口以 CLI 和 npm package `spec-wiki` 为主；包入口见 `packages/spec-wiki/package.json`。

## 配置

| 配置 | 用途 | 默认值或说明 |
| --- | --- | --- |
| `wiki.dev.yaml` | 本地 provider / retry / timeout / backoff 调试配置 | 测试开发时优先使用 |
| `.wiki/` runtime 目录 | repo-local knowledge runtime 与页面投影 | 初始构建由 `spec-wiki init` 内部完成，后续由 `update` / `rebuild` 刷新 |
| `.spec/` | UniSpec change artifact 与 archive | 由 UniSpec skills / CLI 维护 |

## 页面

| 页面 | 用途 |
| --- | --- |
| [00-CLI](./00-CLI.md) | CLI 命令、适用场景和常见用法 |
| [01-配置与运行时产物](./01-配置与运行时产物.md) | 配置文件、`.wiki` runtime 分层和 docs-only Wiki 的边界 |
| [02-v0.2.0发布合同](./02-v0.2.0发布合同.md) | v0.2.0 product-release identity、公开 surface、制品映射和 release evidence 边界 |

## 兼容性

当前处于测试开发阶段，不要求保留旧实现兼容层。公开契约变化应先进入 UniSpec change artifact，再同步本栏目。
