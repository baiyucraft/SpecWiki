---
title: 01-快速上手
description: spec-wiki 的最短上手路径
updated: 2026-05-25
owner: docs
---

# 01-快速上手

本栏目面向首次接手项目的维护者和 Agent，目标是快速判断项目是什么、怎么准备环境、怎么运行验证。

## 项目定位

- `spec-wiki` 是 Repo Wiki Core + Agents 项目。
- 它提供 TS CLI 与 Rust runtime：CLI 负责宿主 bootstrap 和 runtime forwarding，Rust crates 负责事实扫描、知识组织、运行时生命周期和查询。
- 当前包名仍为 `spec-wiki`；UniSpec 只管理项目变更治理，不改变产品包名。

## 环境前提

- Node.js / pnpm：根级 `package.json` 声明 `pnpm@10.6.3`。
- Rust / Cargo：根级 `Cargo.toml` 管理 `wiki-model`、`wiki-index`、`wiki-knowledge`、`wiki-runtime` workspace。
- Windows x64：`packages/spec-wiki/package.json` 当前发布目标限制为 `win32` / `x64`。
- 本地 provider、timeout、retry 等调试配置优先参考 `wiki.dev.yaml`。

## 首次运行

```bash
pnpm install
pnpm run build
pnpm run lint
cargo test
```

## 常用命令

| 命令 | 用途 |
| --- | --- |
| `pnpm run build` | 构建发布产物与 CLI bundle |
| `pnpm run lint` | 运行 ESLint |
| `pnpm run test` | 运行工作区测试脚本；依赖本地共享 fixture 时可能需要先准备 `tmp/test/*` |
| `cargo test` | 运行 Rust workspace 测试 |
| `spec-wiki init --tool codex --repo-root .` | 安装宿主 bootstrap 资产 |
| `spec-wiki wiki init --repo-root .` | 初始化 repo-local knowledge runtime |

## 推荐阅读路径

1. [00-环境准备](./00-环境准备.md)
2. [01-启动项目](./01-启动项目.md)
3. [02-构建项目](./02-构建项目.md)
4. [03-常见问题](./03-常见问题.md)
5. [03-模块指南](../03-模块指南/INDEX.md)
