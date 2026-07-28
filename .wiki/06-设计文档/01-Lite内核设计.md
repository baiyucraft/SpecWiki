---
title: Lite 内核设计
description: 纯 TypeScript 资产、Wiki、change 与路径安全设计
updated: 2026-07-29
owner: architecture
---

# Lite 内核设计

## 资产同步

registry 将文件声明为 `scaffold`、`managed` 或 `skill`。scaffold 与用户页面始终保留；managed 只在 `--force` 时更新；skill 每次同步到当前 package 版本。单文件写入使用同目录临时文件加 rename。

当前模板分别位于 `assets/wiki/zh/**` 与 `assets/wiki/en/**`。`.wiki/config.yaml` 选择目标语言，默认 `zh`；配置通过 YAML document API 更新，保留未知字段。语言迁移先验证来源/目标内容和路径，再以 rollback snapshot 执行登记文件替换。旧 `e830627` 英文 scaffold 由独立 migration baseline 识别。

## Wiki 检查

检查范围是 `.wiki/**/*.md`，要求根和每个 Markdown 目录存在 `INDEX.md`，页面 frontmatter 包含 `title/description/updated/owner`，相对链接不越界且目标存在，非 INDEX 页面从根导航可达，`source_of_truth` 不重复。报告同时包含配置语言和根 bootstrap marker；marker 不影响静态 `wiki.ready`，但会阻止项目级 ready。

## Change 工作流

stage 闭集为 `exploration/proposal/delivery/design/cases/tasks/implementation/review/verification/archive`。artifact 使用固定 registry；verification/archive 需要 full/pass 的 review 与 test 报告。archive 使用 dated target 和 rename，冲突不覆盖。child 归档同步 parent metadata 与 split marker，失败时回滚。

## 路径安全

change id 必须是 kebab-case。所有 I/O 先验证相对路径、拒绝绝对路径与 `..`，再校验 lexical containment；对最近存在祖先执行 realpath containment，阻止 symlink/junction 逃逸。
