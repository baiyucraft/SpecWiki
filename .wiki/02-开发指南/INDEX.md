---
title: 02-开发指南
description: SpecWiki Lite 的开发流程、验证入口和文档维护规则
updated: 2026-07-28
owner: docs
---

# 02-开发指南

本栏目记录跨模块通用的开发约定。模块职责见 [03-模块指南](../03-模块指南/INDEX.md)，公开使用方式见 [04-对外方法](../04-对外方法/INDEX.md)。

## 开发流程

1. 先读取 `.spec/changes/**` 当前 change，需求或设计变化先更新 change artifact。
2. 按 `tasks.md` 的 Red -> Green -> Refactor 顺序实现，真实测试文件在 apply 阶段创建。
3. 代码按 `core/assets`、`core/wiki`、`core/change`、`core/path` 和 CLI 边界组织。
4. 公开行为变化同步测试、README、Wiki 和 package assets。
5. 完整 review 与 verification 通过后，再由 CLI 原子归档 change。

SpecWiki Lite 只支持 Codex，Agent 资产统一放在 `.agents/skills`。不要新增宿主 adapter、hook、settings 或第二套 stage schema。

## 验证命令

| 命令 | 用途 |
| --- | --- |
| `pnpm --dir packages/spec-wiki-lite run test` | package 单元与 CLI 测试 |
| `pnpm --dir packages/spec-wiki-lite run build` | package ESM 构建 |
| `pnpm run test` | package、构建与根级合同测试 |
| `pnpm run lint` | ESLint 门禁 |
| `pnpm run build` | 分发 staging 验证 |
| `pnpm run pack` | tarball 打包验证 |
| `git diff --check` | 空白和补丁格式检查 |

## 页面

| 页面 | 用途 |
| --- | --- |
| [00-代码注释规范](./00-代码注释规范.md) | TypeScript 公开契约与关键文件操作注释标准 |
| [01-测试与验收](./01-测试与验收.md) | TDD、测试层级、分发 smoke 和自举验收 |
| [02-脚本与工作流](./02-脚本与工作流.md) | 根级脚本职责与 evidence 边界 |
| [03-参考实现边界](./03-参考实现边界.md) | UniSpec 来源、目标落点和改写边界 |

## 文档更新规则

- 长期稳定规则写入 `.wiki/`，每个含 Markdown 的目录必须有 `INDEX.md`。
- change 过程、一次性测试结果和 review 明细只写入 `.spec/changes/**` 或 `.spec/archive/**`。
- 页面 frontmatter 至少包含 `title`、`description`、`updated`、`owner`。
- 相对链接必须仍位于 `.wiki`，新增页面要从根索引可达，`source_of_truth` 不得重复。
- package 管理的模板真相位于 `packages/spec-wiki-lite/assets/**`；产品行为文档不得只修改安装后的副本。
