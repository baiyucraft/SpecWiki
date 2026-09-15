---
title: CLI 模块
description: SpecWiki Lite 命令解析、输出、退出码与 TypeScript API
updated: 2026-07-28
owner: docs
---

# CLI 模块

## 职责

`cli.ts` 定义唯一命令面，解析参数后直接调用 TypeScript core。`bin.ts` 只绑定进程输入输出，`index.ts` 汇总公共 API。

## 命令分发

| 命令 | 调用模块 |
| --- | --- |
| `init` | `orchestration/init`，同步资产后默认准备 CodeGraph/AOCI 并聚合官方 readiness |
| `status` | `core/status` |
| `show` | `core/change/show` |
| `validate` | `core/change/validate` |
| `update` | `core/assets` |
| `archive` | `core/change/archive` |

CLI 不启动子进程，不维护后台状态，也不根据输出重新推导 core 结论。

## 输出

- 默认输出标题和格式化 JSON 数据。
- `status/show/validate/update --json` 输出 `{ ok, data?, error? }` 单行 JSON。
- `init --json` 也输出稳定 JSON，包含 CodeGraph CLI、Codex MCP、项目索引和 warnings。
- `init` 默认准备固定版本 CodeGraph/AOCI；`--no-codegraph` / `--no-aoci` 只延后一次，外部失败不回滚 Wiki/.spec 核心资产但返回 exit 2。
- `update --tools` 才修复或升级工具；普通 update 不产生外部工具副作用。
- 帮助文本由同一命令清单生成。

## 退出码

| 代码 | 含义 |
| --- | --- |
| `0` | 命令成功 |
| `1` | 文件系统、解析或其他执行失败 |
| `2` | Change 校验未就绪或归档前置条件未满足 |
| `64` | 未知命令、缺少参数或非法参数组合 |

## TypeScript API

包入口导出 CLI、assets、wiki、change、path safety 和 init orchestration 的稳定函数与类型。具体导出以 `packages/spec-wiki-lite/src/index.ts` 为唯一事实来源；使用者不应从 `dist` 内部路径深链导入。

完整命令参数见[公开 CLI](../04-对外方法/00-CLI.md)。
