---
title: Codex Skill Distribution
description: 八个双语阶段 Skill、references、唯一资产位置和 readiness 合同
updated: 2026-08-03
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

### Requirement: Skill 正文跟随 Wiki 语言

#### Scenario: 选择 zh 或 en

- **WHEN** `.wiki/config.yaml` 的 `wiki.language` 为 `zh|en`
- **THEN** 8 个 `SKILL.md` 与 16 个登记 references 使用对应语言正文
- **AND** Skill id、目录、reference 文件名、CLI 和 `.spec` 机器字段保持英文稳定

### Requirement: 登记 Skill 文件受管且完整

#### Scenario: Update 修复 Skill

- **WHEN** 任一登记主文件或 reference 缺失、被修改、旧版本或语言不一致
- **THEN** 普通 update 将其恢复为目标语言的 package 版本
- **AND** Skill 目录中用户新增的未登记文件保留

#### Scenario: Status 检查 Skill

- **WHEN** status 计算 `skills[].installed`
- **THEN** 只有该 Skill 的所有目标语言登记文件存在且内容匹配时为 true

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

### Requirement: 模板与 review standards 闭包

#### Scenario: 使用阶段 Skill

- **WHEN** Skill 引用 `references/<file>.md`
- **THEN** package 与安装目录包含对应本地化文件
- **AND** plan 的 browser automation 为工具中立可选指南
- **AND** review 提供通用、frontend、Go、Java、Python standards 及正式报告模板
