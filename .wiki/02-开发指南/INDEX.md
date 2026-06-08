---
title: 02-开发指南
description: spec-wiki 的通用开发流程、验证入口和文档维护规则
updated: 2026-05-25
owner: docs
---

# 02-开发指南

本栏目记录跨模块通用的开发约定。模块特有入口放在 [03-模块指南](../03-模块指南/INDEX.md)，公开使用方式放在 [04-对外方法](../04-对外方法/INDEX.md)。

## 开发流程

- 需求和设计变化先看 `.spec/changes/**` 当前 change；需要调整时先改 UniSpec change artifact，再改代码。
- 新实现应沿 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble` 主链推进。
- Rust 包边界以 `wiki-model / wiki-index / wiki-knowledge / wiki-runtime` 为准。
- 宿主接入命名统一使用 `Agents`，新增宿主优先走公共内核 + HostAdapter。
- 行为变化应补对应层级测试：Rust 测试放 `crates/*/tests/`，Agent 测试放 `agents/*/src/*.test.ts`，跨模块测试放 `scripts/tests/*.test.ts`。

## 验证命令

| 命令 | 用途 |
| --- | --- |
| `pnpm run lint` | TypeScript / Markdown 相关脚本的 lint gate |
| `pnpm run test` | 工作区综合测试，包含 Rust、包测试、构建和脚本测试 |
| `cargo test` | Rust workspace 全量测试 |
| `node scripts/run-test-projects.mjs storybook dagger` | 页面质量专项样本 init 验证 |
| `node scripts/test-wiki-lifecycle.mjs` | init / status / sync / query / update / rebuild 生命周期验证 |

## 页面

| 页面 | 用途 |
| --- | --- |
| [00-代码注释规范](./00-代码注释规范.md) | 代码注释、公开契约注释和关键流程注释的最低标准 |
| [01-测试与验收](./01-测试与验收.md) | 测试层级、验证入口、质量门禁和专项样本规则 |
| [02-脚本与工作流](./02-脚本与工作流.md) | 根级 scripts 编排入口、gate 语义和报告收集边界 |
| [03-参考实现边界](./03-参考实现边界.md) | upstream 本地参考仓库的使用方式、目标落点和采用方式 |

## 文档更新规则

- 长期稳定规则写入 `.wiki/`。
- 变更过程、阶段性设计、测试报告和 review 明细写入 `.spec/changes/**` 或 `.spec/archive/**`。
- `.docs/` 只保留阶段性设计稿、调研记录或迁移说明；实现完成后应删除或迁移到 `.wiki/`。
- 更新长期页面时同步栏目 `INDEX.md`，避免页面孤岛。
