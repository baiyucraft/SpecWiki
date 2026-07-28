---
title: Agents 设计
description: Codex-only Skills、阶段路由和资产边界
updated: 2026-07-29
owner: architecture
---

# Agents 设计

SpecWiki Lite 只支持 Codex。八个 Skills 安装到 `.agents/skills`：

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

`wiki-continue` 读取 `spec-wiki-lite status`：有 active change 时按 stage 路由；没有 active change但 `wiki.bootstrapPending` 为 true 时路由 `wiki-explore`，由后续 change 完成项目 Wiki 初始化。其余 Skills 各自负责一个阶段。Skills 可以按需读取源码与 Wiki，但不得建立持久化代码索引或第二套 metadata schema。

资产真相位于 `packages/spec-wiki-lite/assets/skills/**`。`init/update` 通过同一同步器写入 repo-local `.agents/skills`，不生成 `.codex`、hooks、settings 或其他宿主投影。
