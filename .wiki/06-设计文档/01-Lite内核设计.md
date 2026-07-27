---
title: Lite 内核设计
description: 纯 TypeScript 资产、Wiki、change 与路径安全设计
updated: 2026-07-28
owner: architecture
---

# Lite 内核设计

## 资产同步

registry 将文件声明为 `scaffold`、`managed` 或 `skill`。scaffold 与用户页面始终保留；managed 只在 `--force` 时更新；skill 每次同步到当前 package 版本。单文件写入使用同目录临时文件加 rename。

## Wiki 检查

检查范围是 `.wiki/**/*.md`，要求根和每个 Markdown 目录存在 `INDEX.md`，页面 frontmatter 包含 `title/description/updated/owner`，相对链接不越界且目标存在，非 INDEX 页面从根导航可达，`source_of_truth` 不重复。

## Change 工作流

stage 闭集为 `exploration/proposal/delivery/design/cases/tasks/implementation/review/verification/archive`。artifact 使用固定 registry；verification/archive 需要 full/pass 的 review 与 test 报告。archive 使用 dated target 和 rename，冲突不覆盖。child 归档同步 parent metadata 与 split marker，失败时回滚。

## 路径安全

change id 必须是 kebab-case。所有 I/O 先验证相对路径、拒绝绝对路径与 `..`，再校验 lexical containment；对最近存在祖先执行 realpath containment，阻止 symlink/junction 逃逸。
