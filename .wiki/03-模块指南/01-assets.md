---
title: assets 模块
description: SpecWiki Lite 项目模板、Codex Skills 和资产所有权规则
updated: 2026-07-28
owner: docs
---

# assets 模块

## 职责

`core/assets` 维护一个封闭的项目资产 registry，并由同一个同步器服务 `init` 和 `update`。模板真相位于 `packages/spec-wiki-lite/assets/**`。

## 登记资产

| 类型 | 目标 | 更新规则 |
| --- | --- | --- |
| `scaffold` | `.wiki/INDEX.md`、项目/开发/架构/参考栏目与页面模板 | 缺失时创建；已有内容始终保留 |
| `managed` | `.wiki/00-conventions/INDEX.md` | 缺失时创建；仅 `--force` 覆盖 |
| `skill` | `.agents/skills/wiki-*/SKILL.md` | 缺失时创建；每次同步到当前包版本 |

初始 Wiki scaffold 是：

```text
.wiki/
├── INDEX.md
├── 00-conventions/INDEX.md
├── 00-conventions/00-page-template.md
├── 01-project/INDEX.md
├── 01-project/00-overview.md
├── 02-development/INDEX.md
├── 02-development/00-getting-started.md
├── 02-development/01-testing.md
├── 03-architecture/INDEX.md
├── 03-architecture/00-system-overview.md
└── 04-reference/INDEX.md
```

内容页只提供结构化填写提示，不推断仓库事实；Codex 后续按需读取源码并直接维护这些正式页面。

同步器同时确保以下目录存在：

```text
.wiki/
.spec/changes/
.spec/archive/
.agents/skills/
```

## Codex Skills

Registry 固定安装八个 Skills：

```text
wiki-continue
wiki-explore
wiki-propose
wiki-design
wiki-plan
wiki-apply
wiki-review
wiki-archive
```

Skills 只写入 `.agents/skills`。资产层不生成其他宿主入口，也不把项目业务判断写进安装逻辑。

## 写入安全

- 所有目标先经过项目根目录 containment 校验。
- 单文件写入使用同目录临时文件加 rename，避免留下半文件。
- 未登记的 `.wiki` 文件加入 `preserved`，不会被删除或覆盖。
- 报告按 `created / updated / unchanged / preserved` 分类并稳定排序。

## 维护入口

- Registry：`packages/spec-wiki-lite/src/core/assets/registry.ts`
- 资产实现：`packages/spec-wiki-lite/src/core/assets/**`
- 初始化编排：`packages/spec-wiki-lite/src/orchestration/init/runInit.ts`
- 模板：`packages/spec-wiki-lite/assets/wiki/**`、`packages/spec-wiki-lite/assets/skills/**`
