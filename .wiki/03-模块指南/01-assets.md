---
title: assets 模块
description: SpecWiki Lite 项目模板、Codex Skills 和资产所有权规则
updated: 2026-07-29
owner: docs
---

# assets 模块

## 职责

`core/assets` 维护一个封闭的项目资产 registry，并由同一个同步器服务 `init` 和 `update`。模板真相位于 `packages/spec-wiki-lite/assets/**`。

## 登记资产

| 类型 | 目标 | 更新规则 |
| --- | --- | --- |
| `scaffold` | 根 bootstrap 页、五个栏目索引和代码注释页 | 缺失时创建；已有内容始终保留 |
| `managed` | SSOT、页面模板和 Lite 工作流页 | 缺失时创建；仅 `--force` 覆盖 |
| `skill` | `.agents/skills/wiki-*/SKILL.md` | 缺失时创建；每次同步到当前包版本 |

中文默认 scaffold 是：

```text
.wiki/
├── INDEX.md
├── config.yaml
├── 00-文档约定/
│   ├── INDEX.md
│   ├── 00-边界与SSOT规则.md
│   ├── 01-页面模板.md
│   └── 02-SpecWiki-Lite工作流.md
├── 01-快速上手/INDEX.md
├── 02-开发指南/
│   ├── INDEX.md
│   └── 00-代码注释规范.md
├── 03-模块指南/INDEX.md
└── 04-对外方法/INDEX.md
```

`assets/wiki/zh/**` 与 `assets/wiki/en/**` 提供对称结构。根页包含 bootstrap marker；Codex 后续按需读取仓库并整体替换为正式首页。

## 配置与迁移

- `.wiki/config.yaml` 的 `wiki.language` 是目标语言 SSOT，默认 `zh`。
- 同语言同步遵守原 ownership；语言变化时先验证来源内容仍等于 package 模板。
- `assets/migrations/wiki-en-v0/**` 只用于识别提交 `e830627` 的旧英文 scaffold。
- scaffold 修改、目标冲突或路径逃逸使迁移在写入前失败。
- 修改过的旧英文 scaffold 可通过显式 `wiki.language: en` 保持英文；此时旧路径按用户内容保留，并安装不冲突的当前英文资产。
- 迁移执行中断时按 snapshot 回滚登记文件；未登记页面不参与事务。

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
- 报告按 `created / updated / unchanged / preserved / removed` 分类并稳定排序。

## 维护入口

- Registry：`packages/spec-wiki-lite/src/core/assets/registry.ts`
- 资产实现：`packages/spec-wiki-lite/src/core/assets/**`
- 初始化编排：`packages/spec-wiki-lite/src/orchestration/init/runInit.ts`
- 模板：`packages/spec-wiki-lite/assets/wiki/**`、`packages/spec-wiki-lite/assets/skills/**`
