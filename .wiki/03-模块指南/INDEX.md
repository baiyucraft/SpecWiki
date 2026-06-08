---
title: 03-模块指南
description: spec-wiki 的模块边界、KnowledgeUnit 映射和维护入口
updated: 2026-05-25
owner: docs
---

# 03-模块指南

本栏目不是包目录说明书。每个模块页必须说明它对应的 KnowledgeDomain / KnowledgeUnit、事实来源和输出层，避免回退到纯 `module/topic/family` 驱动。

## 模块总览

| 模块 | KnowledgeDomain / KnowledgeUnit | 输出层 | 事实来源 |
| --- | --- | --- | --- |
| [00-工作区总览](./00-工作区总览.md) | workspace boundary / package dependency map | 项目级导航 | `Cargo.toml`、`pnpm-workspace.yaml`、`package.json` |
| [01-wiki-model](./01-wiki-model.md) | shared model / formal state DTO | model 层 | `crates/wiki-model/**` |
| [02-wiki-index](./02-wiki-index.md) | facts / scanner / symbol index | facts / index 层 | `crates/wiki-index/**` |
| [03-wiki-knowledge](./03-wiki-knowledge.md) | knowledge planning / research / compose contract | knowledge 层 | `crates/wiki-knowledge/**` |
| [04-wiki-runtime](./04-wiki-runtime.md) | workflow orchestration / lifecycle / storage / query route | runtime 层 | `crates/wiki-runtime/**` |
| [05-spec-wiki-cli](./05-spec-wiki-cli.md) | host bootstrap / runtime forwarding / distribution | Agents 接入层 | `packages/spec-wiki/**` |

## 维护建议

- 新增模块页时，先写 KnowledgeDomain / KnowledgeUnit，再写路径。
- 模块页只写稳定职责和维护入口，不复制实现细节。
- 公开 CLI、配置和使用方式回链到 [04-对外方法](../04-对外方法/INDEX.md)。
- 通用测试和注释规则回链到 [02-开发指南](../02-开发指南/INDEX.md)。
