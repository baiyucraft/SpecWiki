---
title: wiki 模块
description: SpecWiki Lite 对 .wiki Markdown 的确定性静态检查
updated: 2026-07-28
owner: docs
---

# wiki 模块

## 职责

`core/wiki` 只读取 `.wiki` 中的 Markdown 页面并返回静态检查报告。它不改写用户页面，也不从源码推导文档内容。

## 页面合同

- 根入口必须是 `.wiki/INDEX.md`。
- 每个包含 Markdown 的目录必须有 `INDEX.md`。
- 每页 YAML frontmatter 必须包含非空 `title`、`description`、`updated` 和 `owner`。
- Wiki 内相对链接不得越出 `.wiki`，并且目标文件必须存在。
- 非 `INDEX.md` 页面必须能从根入口沿 Wiki 链接到达。
- 同一个非空 `source_of_truth` 不得由多个页面重复声明。

## 报告

`inspectWiki(projectRoot)` 返回：

| 字段 | 含义 |
| --- | --- |
| `ready` | 没有任何 Wiki issue 时为 `true` |
| `pages` | 参与检查的项目相对 Markdown 路径 |
| `issues` | 按路径和 issue kind 稳定排序的问题列表 |

Issue kind 闭集为：

```text
missing_index
invalid_frontmatter
broken_link
orphan_page
duplicate_ssot
```

## 与 status 的关系

项目 `status` 直接组合 Wiki 检查、Skills 安装状态和 active changes 校验结果。Wiki 有任一 issue 时，项目级 `ready` 为 `false`；status 不自动修改页面。

## 维护入口

- Wiki 检查：`packages/spec-wiki-lite/src/core/wiki/inspect.ts`
- Frontmatter 解析：`packages/spec-wiki-lite/src/core/markdown/frontmatter.ts`
- 项目状态：`packages/spec-wiki-lite/src/core/status.ts`
- 页面结构约定：[配置与项目产物](../04-对外方法/01-配置与项目产物.md)
