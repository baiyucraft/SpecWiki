---
title: spec-wiki CLI
description: TypeScript CLI、宿主 bootstrap、runtime forwarding 和发布入口
updated: 2026-05-25
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

`packages/spec-wiki` 是对外 npm 包和 CLI 入口。它负责宿主接入资产、命令解析、调用 Rust runtime、解析结果和发布组装。

## 不负责什么

- 不承载 Wiki 业务规则。
- 不绕过 `wiki-runtime` 直接拼装 `.wiki` 状态机。
- 不在宿主层复制 core 生成链逻辑。

## 入口与目录

| 路径 | 用途 |
| --- | --- |
| `packages/spec-wiki/src/cli.ts` | CLI 入口 |
| `packages/spec-wiki/src/orchestration/**` | init 编排 |
| `packages/spec-wiki/src/runtime/**` | runtime forwarding 与结果解析 |
| `packages/spec-wiki/src/agents/**` | 宿主资产和公共 Agent 逻辑 |
| `scripts/build-dist.mjs` | 根级发布产物组装 |

## 验证与排查

```bash
pnpm --dir packages/spec-wiki run test
pnpm run build
```

公开命令和使用方式以 [04-对外方法/00-CLI](../04-对外方法/00-CLI.md) 为准。
