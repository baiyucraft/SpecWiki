---
title: wiki-index
description: 代码事实、扫描、符号和索引层
updated: 2026-05-25
owner: docs
---

# wiki-index

## KnowledgeDomain / KnowledgeUnit

| 项 | 内容 |
| --- | --- |
| KnowledgeDomain | facts / index |
| KnowledgeUnit | repo scan facts / file purpose / symbol index / graph substrate |
| 输出层 | facts / index 层 |

## 模块定位

`wiki-index` 负责从仓库中提取代码事实，包括文件扫描、噪声过滤、符号解析、索引和查询底座。

## 不负责什么

- 不负责 declared knowledge、page projection 或 `.wiki` 生命周期。
- 不生成长期 Wiki 文档。
- 不把样本仓库经验硬编码成专有 planner 分支。

## 入口与目录

| 路径 | 用途 |
| --- | --- |
| `crates/wiki-index/src/scanner.rs` | repo scan 与噪声过滤入口 |
| `crates/wiki-index/**` | facts / index 层源码 |
| `crates/wiki-index/Cargo.toml` | crate 元数据和依赖声明 |

## 验证与排查

```bash
cargo test -p wiki-index
cargo test -p wiki-runtime scanner_excludes_non_code_artifact_directories
```

扫描过滤应排除 `.spec`、`.wiki` runtime 产物、fixture、构建产物等非业务源码目录。
