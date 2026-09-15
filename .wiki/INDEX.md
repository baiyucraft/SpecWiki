---
title: SpecWiki Lite Wiki
description: SpecWiki Lite 的长期项目知识与维护入口
updated: 2026-07-29
owner: docs
---

# SpecWiki Lite Wiki

SpecWiki Lite 是纯 TypeScript 的 `.wiki + .spec + Codex Skills` 工具。它同步受管模板、检查 Markdown 结构并管理 change 生命周期；Agent 按需读取仓库文件并编辑正式 Wiki 页面。

## 一级目录

| 目录 | 用途 |
| --- | --- |
| [00-文档约定](./00-文档约定/INDEX.md) | Wiki 边界、SSOT、页面模板和 change 约定 |
| [01-快速上手](./01-快速上手/INDEX.md) | 安装、初始化、构建和常见问题 |
| [02-开发指南](./02-开发指南/INDEX.md) | 测试、脚本、注释和参考边界 |
| [03-模块指南](./03-模块指南/INDEX.md) | TypeScript 模块职责与维护入口 |
| [04-对外方法](./04-对外方法/INDEX.md) | CLI、目录合同与发布合同 |
| [05-规格基线](./05-规格基线/INDEX.md) | Lite 当前 capability 基线 |
| [06-设计文档](./06-设计文档/INDEX.md) | 总体、内核、Agents 和场景设计 |

## SSOT

- 源码、配置和测试是行为事实来源。
- `.wiki/` 只保存稳定项目知识和导航。
- `.spec/changes/**` 与 `.spec/archive/**` 保存 change artifact，不复制进 Wiki。
- `.agents/skills/wiki-*` 是 Codex 工作流资产的唯一安装位置。
- `.wiki/config.yaml` 是 Wiki 语言和未来 LLM 项目配置的唯一入口。
- 同一事实只维护一处，其他页面通过摘要和相对链接引用。

## 关键入口

- [总体设计](./06-设计文档/00-总体设计.md)
- [Lite 内核设计](./06-设计文档/01-Lite内核设计.md)
- [Agents 设计](./06-设计文档/02-Agents设计.md)
- [CLI](./04-对外方法/00-CLI.md)
- [v0.2.0 发布合同](./04-对外方法/02-v0.2.0发布合同.md)
- [测试与验收](./02-开发指南/01-测试与验收.md)
