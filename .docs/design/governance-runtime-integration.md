---
title: SpecWiki 治理运行时融合设计（已迁移）
status: migrated
updated: 2026-07-13
---

# SpecWiki 治理运行时融合设计（已迁移）

Governance evidence、policy、query refs 与 recoverable archive 已进入 `wiki-runtime`，本文件只保留历史入口。

- 稳定治理边界：[Runtime 设计](../../.wiki/06-设计文档/01-Runtime设计.md#governance-runtime-边界)
- Runtime 模块入口：[wiki-runtime](../../.wiki/03-模块指南/04-wiki-runtime.md)
- 对外命令：[CLI](../../.wiki/04-对外方法/00-CLI.md)
- 原始草稿快照：[parent archive source design](../../.spec/archive/2026-07-13-refactor-specwiki-around-contract-closure/research/source-designs/governance-runtime-integration.md)

Archive 不再强制调用 wiki-sync；Wiki 只通过 issues/refs 报告。
