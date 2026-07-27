---
title: Asset Ownership Sync
description: Wiki 基线与 Codex Skills 的幂等 ownership 同步合同
updated: 2026-07-28
owner: product
---

# Asset Ownership Sync

## Purpose

让空仓库获得可用骨架，让已有仓库安全更新 package-owned 资产，同时保留项目自定义内容。

## Requirements

### Requirement: Ownership 决定覆盖策略

#### Scenario: 默认同步

- **WHEN** 执行 `init` 或 `update`
- **THEN** 缺失 scaffold 被创建，已有 scaffold 被保留，Skills 同步到 package 版本

#### Scenario: 强制同步

- **WHEN** 使用 `--force`
- **THEN** managed baseline 更新，scaffold 与用户页面仍不覆盖

### Requirement: 同步失败关闭

#### Scenario: 文件写入

- **WHEN** 创建或更新 package-owned 资产
- **THEN** 内容先写入同目录临时文件，再原子替换目标

### Requirement: 写入范围受限

#### Scenario: 初始化项目

- **WHEN** CLI 同步资产
- **THEN** 只写 `.wiki`、`.spec` 和 `.agents/skills` 范围，并只接受 `--host codex`
