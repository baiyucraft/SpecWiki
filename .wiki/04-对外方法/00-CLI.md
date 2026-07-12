---
title: CLI
description: spec-wiki 一级命令、机器协议和退出码合同
updated: 2026-07-13
owner: docs
---

# CLI

## 主路径

`spec-wiki` 使用一级命令。默认 help 只突出初始化、状态、查询和更新：

```bash
spec-wiki init [--host <host> | --hosts <host,host>] [--repo-root <path>] [--no-interactive]
spec-wiki status [--repo-root <path>]
spec-wiki query <term...> [--repo-root <path>]
spec-wiki update [--repo-root <path>] [--bridge-stdio]
```

`init` 是唯一对外初始化入口，依次完成宿主 bootstrap、repo-local runtime 初始化和 landing status。机器模式不进入交互选择。

## 高级命令

`spec-wiki --help-all` 列出已经实现的高级命令；`advanced` 只是 help 分组，不是 namespace。

```bash
spec-wiki sync [--repo-root <path>]
spec-wiki rebuild [--repo-root <path>] [--bridge-stdio]
spec-wiki changes [--repo-root <path>]
spec-wiki change <change-id> [--repo-root <path>]
spec-wiki validate <change-id> [--repo-root <path>]
```

本阶段不注册 `archive`、workspace validate、`doctor`、`repair` 或 `trace`。

## 输出模式

- 默认输出 human 文本，只翻译 Rust DTO，不重新计算 readiness、outcome 或 recommended action。
- `--json` 输出 JSON 或 NDJSON。
- `--bridge-stdio` 强制机器模式，仅适用于 `init/update/rebuild`。
- 长流程事件流必须按完整 NDJSON 行解析，并且恰好包含一个 terminal event。

## 退出码

| 退出码 | 含义 |
| --- | --- |
| `0` | 成功 |
| `2` | unified init partial，或 `validate` 返回 `valid=false` |
| `64` | 缺参、未知命令/参数、非法组合、host 选择失败 |
| `1` | domain、workflow、protocol 或 internal failure |

## 稳定边界

- `query` 只接受一个或多个位置 token，并合并为查询词。
- `--host` 可重复，`--hosts` 接受逗号列表，两者互斥。
- `changes/change/validate` 返回基于单次 live governance evaluation 的结构化 envelope。
- JavaScript API 保留 `wikiInit/wikiStatus/wikiQuery/wikiUpdate/wikiSync/wikiRebuild`。
- 宿主资产保留 `wiki-*` skill identity 和 Claude `/wiki:*` identity，但执行一级 CLI。
