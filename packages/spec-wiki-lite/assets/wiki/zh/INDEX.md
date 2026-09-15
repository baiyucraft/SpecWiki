---
title: Wiki 初始化任务
description: 指导 Codex 根据当前仓库事实建立正式项目 Wiki
updated: 2026-09-15
owner: project
---

<!-- spec-wiki-lite:bootstrap-pending -->

# Wiki 初始化任务

本页是一次性初始化任务，不是最终 Wiki 首页。Codex 应按需探索当前仓库、完善栏目和页面，完成后用正式项目首页整体替换本页并移除 bootstrap marker。

## 已有栏目

- [文档约定](./00-文档约定/INDEX.md)
- [快速上手](./01-快速上手/INDEX.md)
- [开发指南](./02-开发指南/INDEX.md)
- [模块指南](./03-模块指南/INDEX.md)
- [对外方法](./04-对外方法/INDEX.md)

## 初始化步骤

1. 先读取 AOCI 官方 Guide 与完整 Overview；若 Code Cognition 未对齐，按官方 Maintain/Recovery 完成治理后再继续。AOCI 提供长期职责、强关系和约束。
2. 使用 CodeGraph 检查当前源码入口、符号、调用链和影响范围，再用 README、manifest、构建/测试配置和定向源码核验。CodeGraph 与 AOCI 证据必须分开记录。
3. 判断证据是否足以说明项目定位、技术栈与运行方式；存在阻塞信息时先向用户确认，不做无差别全量扫描。
4. 用交叉验证的项目事实替换各栏目 `INDEX.md` 中的占位内容；不适用的栏目说明原因，不虚构功能。
5. 按需新增快速上手、模块、CLI、API、配置、部署或排障页面；每个 Markdown 目录保留 `INDEX.md`。
6. 将本页替换为正式首页。正式首页至少包含一级目录、SSOT 规则、按任务导航、按模块或包导航和文档约定。

## 输出

- 正式 `.wiki/INDEX.md`
- 已填充的栏目索引和必要项目页面
- 仍需用户确认的问题
