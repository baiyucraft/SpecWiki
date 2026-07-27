---
title: Codex Skill Distribution
description: 八个阶段 Skill 的唯一资产位置和路由合同
updated: 2026-07-28
owner: product
---

# Codex Skill Distribution

## Purpose

为 Codex 提供可发现、可更新且与 `.spec` stage 一致的 Wiki 工作流。

## Requirements

### Requirement: Skill 集合固定

#### Scenario: Init 或 update

- **WHEN** 同步 package 资产
- **THEN** `.agents/skills` 包含 `wiki-continue/explore/propose/design/plan/apply/review/archive`

### Requirement: Continue 只做阶段路由

#### Scenario: 继续 active change

- **WHEN** 调用 `wiki-continue`
- **THEN** 读取 status 和 change metadata，选择一个匹配 stage 的专业 Skill，不跳过 required artifact

### Requirement: Agent 按需读取事实

#### Scenario: 处理 Wiki change

- **WHEN** Skill 需要仓库上下文
- **THEN** 可按需读取源码、测试和 Wiki，但不创建源码派生数据库或隐藏文档层

### Requirement: 安装位置唯一

#### Scenario: 同步 Skills

- **WHEN** init 或 update 完成
- **THEN** Skills 只安装到 `.agents/skills/wiki-*`
