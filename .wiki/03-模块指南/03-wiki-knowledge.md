---
title: wiki-knowledge
description: KnowledgeUnit 主线上的 planning、research 和 compose 合同
updated: 2026-05-25
owner: docs
---

# wiki-knowledge

## KnowledgeDomain / KnowledgeUnit

| 项 | 内容 |
| --- | --- |
| KnowledgeDomain | knowledge planning / research / compose |
| KnowledgeUnit | knowledge tree / page research dossier / compose contract |
| 输出层 | knowledge 层 |

## 模块定位

`wiki-knowledge` 把 `wiki-index` 提供的事实组织成知识单元，并维护 Knowledge Planning、Research、Compose 相关合同。

## 不负责什么

- 不管理 `.wiki` 生命周期、cache、transport 或 CLI。
- 不直接承担宿主接入。
- 不以页面类型作为一等驱动；页面是 KnowledgeUnit 投影结果。

## 入口与目录

| 路径 | 用途 |
| --- | --- |
| `crates/wiki-knowledge/**` | knowledge planning / research / compose 逻辑 |
| `crates/wiki-knowledge/Cargo.toml` | crate 元数据和依赖声明 |

## 验证与排查

```bash
cargo test -p wiki-knowledge
```

新增页面语义前必须回答对应的 KnowledgeDomain / KnowledgeUnit，以及它属于哪一层输出。
