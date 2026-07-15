---
title: 06-设计文档
description: spec-wiki 当前稳定设计、runtime 设计、Agents 设计和场景边界入口
updated: 2026-07-15
owner: architecture
---

# 06-设计文档

本目录保存当前项目仍然有效的稳定设计 SSOT。根目录只保留产品和 Agent 入口，设计正文统一沉淀在这里。

## 页面索引

| 页面 | 内容 | 状态 |
| --- | --- | --- |
| [00-总体设计](./00-总体设计.md) | Repo Wiki Core + Agents 的总体定位、四包架构和核心原则 | 当前稳定设计 |
| [01-Runtime设计](./01-Runtime设计.md) | runtime 主链、`.wiki/` 分层、query route、生命周期和恢复策略 | 当前稳定设计 |
| [02-Agents设计](./02-Agents设计.md) | `spec-wiki` 宿主接入、bootstrap、runtime forwarding 和资产模型 | 当前稳定设计 |
| [03-核心场景](./03-核心场景.md) | 第一版 9 个核心用户故事 | 当前场景边界 |
| [04-扩展场景](./04-扩展场景.md) | 第一版之外的重要扩展场景 | 后续场景边界 |
| [05-产品基线与设计治理](./05-产品基线与设计治理.md) | 3.0 产品范围、版本 authority、状态证据和设计完成规则 | 当前治理基线 |
| [06-Runtime查询合同](./06-Runtime查询合同.md) | query 输入、route groups、ranking、provenance、answer 与错误合同 | 当前稳定合同 |

## 维护规则

- 这里保存已采纳的稳定设计，不保存未实现的阶段草案。
- 新的设计推衍先放入 `.docs/design/**`，采纳后再通过 `.spec/changes/**` 推进。
- 设计正文被实现和验收后，长期稳定部分再迁入本目录。
- 历史 change artifact 仍保留在 `.spec/archive/**`，不复制到本目录。
