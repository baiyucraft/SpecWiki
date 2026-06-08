---
title: 00-文档约定
description: spec-wiki Wiki 的边界、命名、模板和 UniSpec 规范入口
updated: 2026-05-25
owner: docs
---

# 00-文档约定

本栏目定义 `spec-wiki` 长期文档如何组织、命名和维护。它只约束 Wiki 与 change artifact 的边界，不替代源码、测试、配置或正式规格基线。

## 页面

| 页面 | 用途 |
| --- | --- |
| [00-边界与SSOT规则](./00-边界与SSOT规则.md) | 长期文档、临时材料、change artifact、runtime 产物和代码事实的边界 |
| [01-页面模板](./01-页面模板.md) | 新增模块指南页、对外方法页和目录型页面时的基础结构 |
| [02-UniSpec开发规范](./02-UniSpec开发规范.md) | UniSpec change artifact、stage、multi-change 和 archive 规范 |
| [03-Agent协作入口](./03-Agent协作入口.md) | AGENTS.md 与 .wiki 长期知识之间的职责边界 |
| [04-文档盘点与沉淀规则](./04-文档盘点与沉淀规则.md) | 当前 Markdown 文档的 SSOT 分类、Wiki 沉淀方式和保留边界 |

## 当前约定

- 每个一级栏目必须有 `INDEX.md`。
- 普通长期页面命名为 `NN-主题.md`，例如 `00-环境准备.md`。
- 目录型主题命名为 `NN-主题/INDEX.md + NN-子页.md`，只在单页过长或多个稳定主题需要拆分时使用。
- 结构性例外：根入口和栏目入口使用 `INDEX.md`；规格基线 capability 保留 `capabilities/<capability>/spec.md`。
- frontmatter 至少包含 `title`、`description`、`updated`、`owner`。
- `owner: docs` 表示项目长期文档维护；`owner: unispec` 只用于 UniSpec 内置基线页。

## 维护流程

1. 新增长期页面前，先判断信息应进入 `.wiki/`、`.spec/changes/**`、`.spec/archive/**` 还是源码 / 测试。
2. 新增、移动或重命名页面后，同步更新所在栏目 `INDEX.md`，必要时同步 `.wiki/INDEX.md`。
3. 修改 CLI、配置、runtime 产物、模块职责或开发流程后，检查 `01-快速上手`、`03-模块指南` 和 `04-对外方法` 是否需要同步。
4. 不能确认事实来源时，写成待确认项，不把推测写成结论。
