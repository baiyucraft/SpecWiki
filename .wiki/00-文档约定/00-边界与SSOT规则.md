---
title: 边界与 SSOT 规则
description: 长期 Wiki、change artifact、临时材料和代码事实的边界
updated: 2026-07-29
owner: spec-wiki-lite
---

# 边界与 SSOT 规则

SpecWiki Lite 只管理可见 Markdown Wiki、分阶段 change artifact 和 Codex Skills。它不从源码生成持久化派生数据。

## 信息边界

| 类型 | 位置 | SSOT 规则 |
| --- | --- | --- |
| 长期项目知识 | `.wiki/**/*.md` | 保存稳定说明、设计、指南和公开合同 |
| Active change | `.spec/changes/<change-id>/` | 保存 proposal、design、tests、tasks 和验证证据 |
| 历史 change | `.spec/archive/<date>-<change-id>/` | 归档后只读，不反向覆盖当前 Wiki |
| Codex 工作流 | `.agents/skills/wiki-*/SKILL.md` | 唯一安装位置，由 package 资产同步 |
| 行为事实 | 源码、配置、测试 | CLI、schema、默认值和错误行为的最终事实来源 |
| 临时材料 | `.tmp/` 或登记后的 `.docs/` | 不作为当前产品 authority |

同一事实只维护一处。次级页面使用摘要和相对链接指向 authority，不复制容易漂移的命令、字段或测试结果。

## 正式 Wiki 树

```text
.wiki/INDEX.md
.wiki/<栏目>/INDEX.md
.wiki/<栏目>/NN-主题.md
.wiki/05-规格基线/capabilities/<capability>/spec.md
```

- 根和每级目录必须有 `INDEX.md`。
- 普通页面使用 `NN-主题.md`；capability 规格使用固定 `spec.md`。
- 页面必须包含 `title`、`description`、`updated`、`owner` frontmatter。
- 页面链接使用仓库内相对路径；不得用路径逃逸引用仓库外文件。
- 自定义页面由项目维护者拥有；`init/update` 不覆盖它们。

## 长文与导航

只有当一个页面同时承载多个稳定主题并影响导航或维护时，才拆为目录：

```text
.wiki/<栏目>/<主题>/
  INDEX.md
  01-<子主题>.md
  02-<子主题>.md
```

拆分、移动或新增页面时，必须同步更新父级 `INDEX.md` 和反向链接。行数只是整理信号，不是 CLI gate。

## 维护检查

1. 先判断信息属于 Wiki、active change、archive、代码事实还是临时材料。
2. 行为变化先更新 change artifact，再更新实现和受影响的当前 authority。
3. 新增页面后运行 `spec-wiki-lite status`，修复 frontmatter、链接、孤儿页和导航问题。
4. 无法确认事实来源时保留为 change 中的问题，不把推测写入长期 Wiki。
