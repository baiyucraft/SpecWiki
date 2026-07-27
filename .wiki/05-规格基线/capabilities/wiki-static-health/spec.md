---
title: Wiki Static Health
description: 可见 Markdown Wiki 的确定性结构健康合同
updated: 2026-07-28
owner: product
---

# Wiki Static Health

## Purpose

在不解析源码的前提下，给项目维护者和 Codex 一份可定位、可自动化的 Wiki 结构报告。

## Requirements

### Requirement: 页面结构可验证

#### Scenario: 健康 Wiki

- **WHEN** 所有目录都有 `INDEX.md`，页面具有完整 frontmatter，链接有效且页面可从根导航到达
- **THEN** `spec-wiki-lite status` 报告 Wiki ready

#### Scenario: 结构损坏

- **WHEN** 缺少目录入口、frontmatter、链接目标或导航路径
- **THEN** status 返回包含 kind、path 和 message 的稳定 issue

### Requirement: SSOT 不重复

#### Scenario: 重复 authority

- **WHEN** 多个当前页面声明相同 `source_of_truth`
- **THEN** status 报告 `duplicate_ssot`

### Requirement: 检查边界固定

#### Scenario: 运行静态检查

- **WHEN** 执行 status
- **THEN** 只读取可见 Wiki Markdown、Skills 和 active change 元数据，不推断源码内容的新鲜度
