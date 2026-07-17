---
title: CLI
description: spec-wiki 一级命令、Codex-first 宿主入口、机器协议和退出码合同
updated: 2026-07-17
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

Codex 是唯一 reference host，默认文档与兼容验证优先使用 `--host codex`。Claude、CodeBuddy 是 compatible hosts；显式选择任一宿主的 CLI 行为不变。

## 高级命令

`spec-wiki --help-all` 列出已经实现的高级命令；`advanced` 只是 help 分组，不是 namespace。

```bash
spec-wiki sync [--repo-root <path>]
spec-wiki rebuild [--repo-root <path>] [--bridge-stdio]
spec-wiki changes [--repo-root <path>]
spec-wiki change <change-id> [--repo-root <path>]
spec-wiki validate <change-id> [--repo-root <path>]
spec-wiki archive <change-id> [--dry-run | --apply | --resume <operation-id>] [--repo-root <path>]
```

`archive` 默认等同于 `--dry-run`，只执行 validate、readiness 评估并返回 operation manifest，不移动 change，也不创建 durable operation。`--apply` 才执行归档写入；部分完成后必须使用报告给出的 `operation-id` 配合 `--resume` 恢复。三种模式互斥，archive 始终是短流程命令。

archive 只修改 `.spec` 的 change、parent marker 和 `.spec/.runtime/archive-operations` 操作证据，不调用 Wiki sync/update/rebuild，也不写 `.wiki/**`；Wiki 相关结果仅作为 `wiki_sync_issues` 和 `evidence_refs` 返回。

本阶段不注册 workspace validate、`doctor`、`repair` 或 `trace`。

## 输出模式

- 默认输出 human 文本，只翻译 Rust DTO，不重新计算 readiness、outcome 或 recommended action。
- `--json` 输出 JSON 或 NDJSON。
- `--bridge-stdio` 强制机器模式，仅适用于 `init/update/rebuild`。
- 长流程事件流必须按完整 NDJSON 行解析，并且恰好包含一个 terminal event。

`--bridge-stdio` 表示基础 JSON/NDJSON/stdin-stdout forwarding。它不证明 production `research_page` bridge 或多轮 agent-session bridge 已启用，也不授权 durable provider session。

## 退出码

| 退出码 | 含义 |
| --- | --- |
| `0` | 成功 |
| `2` | unified init partial、`validate` 返回 `valid=false`，或 archive 返回 not-ready、precondition changed、conflict、locked、recovery-required |
| `64` | 缺参、未知命令/参数、非法组合、host 选择失败 |
| `1` | archive manifest invalid，或其他 domain、workflow、protocol、I/O、internal failure |

## 稳定边界

- `query` 只接受一个或多个位置 token，并合并为查询词。
- `--host` 可重复，`--hosts` 接受逗号列表，两者互斥。
- `changes/change/validate` 返回基于单次 live governance evaluation 的结构化 envelope。
- `archive` 转发 `archiveMode`；仅 resume 额外转发 `archiveOperationId`，human/JSON 输出均直接翻译 Rust archive DTO。
- JavaScript API 保留 `wikiInit/wikiStatus/wikiQuery/wikiUpdate/wikiSync/wikiRebuild`。
- 三宿主资产统一保留 repo-local `wiki-*` skill identity 并执行一级 CLI；Claude 旧 `/wiki:*` command 路径不属于当前合同。
