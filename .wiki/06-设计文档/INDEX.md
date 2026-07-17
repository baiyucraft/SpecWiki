---
title: 06-设计文档
description: spec-wiki 当前稳定设计、Runtime、Codex-first Agents 和场景边界入口
updated: 2026-07-17
owner: architecture
---

# 06-设计文档

本目录保存当前项目仍然有效的稳定设计 SSOT。根目录只保留产品和 Agent 入口，设计正文统一沉淀在这里。

这里的“稳定”表示页面是当前 `adopted authority`，不自动表示功能已实现、验证通过或发布。`implementationEvidence`、`verificationEvidence` 和 `releaseEvidence` 由各专题合同与可核验证据独立表达。

## 页面索引

| 页面 | 内容 | 状态 |
| --- | --- | --- |
| [00-总体设计](./00-总体设计.md) | Repo Wiki Core + Agents 的总体定位、四包架构和核心原则 | 当前稳定设计 |
| [01-Runtime设计](./01-Runtime设计.md) | runtime 主链、`.wiki/` 分层、query route、生命周期和恢复策略 | 当前稳定设计 |
| [02-Agents设计](./02-Agents设计.md) | Codex-first 宿主接入、bootstrap、trigger、runtime forwarding 和资产边界 | 当前稳定设计 |
| [03-核心场景](./03-核心场景.md) | 第一版 9 个核心用户故事 | 当前场景边界 |
| [04-扩展场景](./04-扩展场景.md) | baseline / next / non-goal 扩展场景分类 | 当前扩展场景分类 authority |
| [05-产品基线与设计治理](./05-产品基线与设计治理.md) | 3.0 产品范围、版本 authority、状态证据和设计完成规则 | 当前治理基线 |
| [06-Runtime查询合同](./06-Runtime查询合同.md) | query 输入、route groups、ranking、provenance、answer 与错误合同 | 当前稳定合同 |

## 维护规则

- 这里保存已采纳的设计 authority，不保存尚未采纳的阶段草案。
- 设计是否 implemented、verified 或 released 必须分别引用对应 evidence，不能从本目录存在性推导。
- 新的设计推衍通过 `.spec/changes/<change-id>/research/**` 与 `design.md` 推进；尚未采纳的内容不进入本目录。
- 设计正文被实现和验收后，长期稳定部分再迁入本目录。
- 历史 change artifact 仍保留在 `.spec/archive/**`，不复制到本目录。
