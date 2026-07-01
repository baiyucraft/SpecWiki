---
title: wiki-index
description: 代码事实、扫描、符号、graph substrate 和索引查询层
updated: 2026-07-01
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

`wiki-index` 负责从仓库中提取代码事实，包括文件扫描、噪声过滤、符号解析、code graph substrate、FTS 和 facts-only 查询底座。

## Code Graph 合同

`wiki-index` 的 graph substrate 是 facts 层 authority，不从 `.wiki` 页面或 runtime fallback 反推事实。

稳定合同包括：

- source authority: `files / folders`
- symbol facts: `SymbolNode / SourceRange / SymbolProvenance`
- graph facts: `edges / raw_imports / raw_calls / raw_heritage / unresolved_refs`
- phase diagnostics: `GraphPhaseStatus / GraphReadiness`
- query hits: symbol、path、graph 三类 facts-only 命中

`scan_cache` 可以保存扫描原始快照，但正式 source record 读取以 graph `files / folders` 为准。

`.spec` 目录不进入 code graph facts、raw captures、unresolved refs 或 FTS；治理 artifact 由治理层单独处理。

## Query 边界

`wiki-index::query` 只消费 `IndexSnapshotStore / IndexQueryStore` traits，不读取 runtime state、Markdown page fallback 或 `.wiki` 页面。

当前稳定输出：

- symbol hit 携带 `file_id / range / provenance`
- path hit 来自 graph source authority 与 `files_fts`
- graph hit 携带 hop distance、confidence、reason、provenance 和 diagnostics

## 不负责什么

- 不负责 declared knowledge、page projection 或 `.wiki` 生命周期。
- 不生成长期 Wiki 文档。
- 不把样本仓库经验硬编码成专有 planner 分支。

## 入口与目录

| 路径 | 用途 |
| --- | --- |
| `crates/wiki-index/src/scanner.rs` | repo scan 与噪声过滤入口 |
| `crates/wiki-index/src/store.rs` | index snapshot/query traits 与 graph DTO |
| `crates/wiki-index/src/symbols/**` | symbol、raw capture、range、provenance 合同 |
| `crates/wiki-index/src/symbol_graph/**` | imports/calls/heritage resolve 与 graph analysis |
| `crates/wiki-index/src/query.rs` | facts-only index query adapter |
| `crates/wiki-index/**` | facts / index 层源码 |
| `crates/wiki-index/Cargo.toml` | crate 元数据和依赖声明 |

## 验证与排查

```bash
cargo test -p wiki-index
cargo test -p wiki-runtime scanner_excludes_non_code_artifact_directories
cargo test -p wiki-runtime --test runtime
```

扫描过滤应排除 `.spec`、`.wiki` runtime 产物、fixture、构建产物等非业务源码目录。
