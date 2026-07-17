---
title: spec-wiki CLI
description: Codex-first TypeScript CLI、宿主 bootstrap、trigger projection、runtime forwarding 和发布入口
updated: 2026-07-17
owner: docs
---

# spec-wiki CLI

## KnowledgeDomain / KnowledgeUnit

| 项 | 内容 |
| --- | --- |
| KnowledgeDomain | Agents integration |
| KnowledgeUnit | host bootstrap / command resolution / runtime forwarding / distribution |
| 输出层 | Agents 接入层 |

## 模块定位

`packages/spec-wiki` 是对外 npm 包和 CLI 入口。它负责宿主接入资产、命令解析、调用 Rust Runtime、解析结果和发布组装。宿主层采用 Codex-first：Codex 是唯一 reference host；Claude、CodeBuddy 是 compatible host。

## 不负责什么

- 不承载 Wiki 业务规则。
- 不绕过 `wiki-runtime` 直接拼装 `.wiki` 状态机。
- 不在宿主层复制 core 生成链逻辑。
- 不在 TypeScript parser 中复制治理 required artifact matrix 或 blocked/conflict 判定；只校验并透传 Rust governance DTO 闭集。
- 不让 CodeBuddy hooks/settings 反向定义共享 trigger 或 Runtime query 语义。
- 不把基础 bridge forwarding 描述为已启用的 production research bridge，也不持久化 provider session。

## 入口与目录

| 路径 | 用途 |
| --- | --- |
| `packages/spec-wiki/src/cli.ts` | CLI 入口 |
| `packages/spec-wiki/src/orchestration/**` | init 编排 |
| `packages/spec-wiki/src/runtime/**` | runtime forwarding 与结果解析 |
| `packages/spec-wiki/src/agents/**` | 宿主资产和公共 Agent 逻辑 |
| `scripts/build-dist.mjs` | 根级发布产物组装 |

真实宿主资产路径：

```text
.codex/skills/wiki-*/SKILL.md
.claude/skills/wiki-*/SKILL.md
.codebuddy/skills/wiki-*/SKILL.md + hooks/settings
```

## 验证与排查

```bash
pnpm --dir packages/spec-wiki run test
pnpm run build
```

公开命令和使用方式以 [04-对外方法/00-CLI](../04-对外方法/00-CLI.md) 为准；宿主角色与三层边界以 [06-设计文档/02-Agents设计](../06-设计文档/02-Agents设计.md) 为准。
