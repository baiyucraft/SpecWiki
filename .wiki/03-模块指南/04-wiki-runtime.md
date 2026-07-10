---
title: wiki-runtime
description: workflow orchestration、storage、transport、query route 和 .wiki 生命周期
updated: 2026-07-11
owner: docs
---

# wiki-runtime

## KnowledgeDomain / KnowledgeUnit

| 项 | 内容 |
| --- | --- |
| KnowledgeDomain | runtime lifecycle |
| KnowledgeUnit | workflow orchestration / storage state / query route / page projection |
| 输出层 | runtime 层 |

## 模块定位

`wiki-runtime` 承接 `wiki-model`、`wiki-index`、`wiki-knowledge`，负责 workflow orchestration、query route、storage、transport、`.wiki` 生命周期与恢复。它也负责只读读取 `.spec` evidence、执行唯一治理 policy，并把独立的 governance summary 组合进 status/query/update。

## 不负责什么

- 不重新导出 `wiki-index` 或 `wiki-knowledge` 的内部实现给外部直接使用。
- 不把宿主 bootstrap 资产写入自身业务逻辑。
- 不把长期文档整理等同于 runtime 初始化。
- 不把 `.spec` 纳入 code scanner、knowledge truth 或 Wiki 正文，也不让治理状态改写 core fusion readiness。

## 入口与目录

| 路径 | 用途 |
| --- | --- |
| `crates/wiki-runtime/src/**` | runtime 生命周期、workflow、storage、transport |
| `crates/wiki-runtime/src/domain/governance.rs` | required artifact、report gate、metadata 与 parent/child policy |
| `crates/wiki-runtime/src/storage/governance_fs.rs` | `.spec` evidence discovery、路径边界与 fingerprint |
| `crates/wiki-runtime/src/storage/sqlite/governance_store.rs` | fingerprint 绑定的可重建治理 cache wrapper |
| `crates/wiki-runtime/src/workflows/governance.rs` | status/list/inspect/validate/refresh/query refs 组合入口 |
| `crates/wiki-runtime/tests/**` | runtime、repo、hierarchy、symbol、lifecycle 测试 |
| `crates/wiki-runtime/Cargo.toml` | crate 元数据和依赖声明 |

## 验证与排查

```bash
cargo test -p wiki-runtime
node scripts/test-wiki-lifecycle.mjs
```

`.wiki/.knowledge/**`、正式可见 Wiki 页面树、`.wiki/wiki.metadata.json` 和 `.wiki/.cache/**` 是 runtime 分层，职责不能混用。正式可见 Wiki 页面树只包括 `.wiki/INDEX.md`、栏目 `INDEX.md` 和 `NN-主题.md`。

## 测试 Suite

`crates/wiki-runtime/tests/**` 按主题拆成 integration suites，顶层只保留 suite 入口，避免测试根目录失控增长。

| Suite | 用途 |
| --- | --- |
| `acceptance.rs` | CLI 合约、基础 acceptance、`init` 产物检查 |
| `hierarchy.rs` | module tree、hierarchy planning、page topology、merge strategy |
| `llm_runtime.rs` | LLM runtime contract 与降级路径验证 |
| `repo.rs` | repo scan、language parsing、noise filter、steering |
| `runtime.rs` | runtime / SQLite、query / sync / update / rebuild、markdown merge、metadata |
| `suite_env.rs` | integration suite 共享测试环境 |
| `symbols.rs` | symbol parsing、symbol resolution、symbol graph analysis |
