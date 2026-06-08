---
title: wiki-model
description: 共享对象语言、正式状态和跨 crate DTO 的模型层
updated: 2026-05-25
owner: docs
---

# wiki-model

## KnowledgeDomain / KnowledgeUnit

| 项 | 内容 |
| --- | --- |
| KnowledgeDomain | shared model |
| KnowledgeUnit | formal state DTO / knowledge artifact DTO / cross-crate object language |
| 输出层 | model 层 |

## 模块定位

`wiki-model` 提供稳定共享对象语言。它应承载跨 crate 共享的正式 DTO、状态对象和元数据结构。

## 不负责什么

- 不做 IO、SQL、transport、prompt 或 renderer helper。
- 不依赖 `wiki-index`、`wiki-knowledge` 或 `wiki-runtime`。
- 不承载 workflow 编排和 runtime 生命周期。

## 入口与目录

| 路径 | 用途 |
| --- | --- |
| `crates/wiki-model/**` | 模型层源码 |
| `crates/wiki-model/Cargo.toml` | crate 元数据和依赖声明 |

## 验证与排查

```bash
cargo test -p wiki-model
```

如果新增 DTO 需要被其它 crate 消费，先确认它是稳定对象语言，而不是某个 runtime helper。
