---
title: CLI
description: spec-wiki 对外 CLI 命令和使用边界
updated: 2026-05-25
owner: docs
---

# CLI

## 适用场景

使用者通过 `spec-wiki` CLI 完成两类事情：

- 初始化 SpecWiki，包括安装宿主 bootstrap 资产和建立 repo-local knowledge runtime。
- 在初始化后查询、同步和刷新 repo-local knowledge runtime。

## 包入口或命令

```text
spec-wiki
```

包入口以 `packages/spec-wiki/package.json` 的 `bin.spec-wiki` 为准。

## 初始化命令

```bash
spec-wiki init [--host <host> | --hosts <host1,host2>] [--repo-root <path>] [--no-interactive]
```

用途：初始化 SpecWiki。该入口统一负责 Codex、Claude、CodeBuddy 等宿主资产、repo-local knowledge runtime、必要的 `.wiki` runtime 产物和 Agent 入口。实现内部可以拆成 bootstrap、index、knowledge、projection 等阶段，但用户侧只需要记住这一条初始化命令。

## 一级命令

SpecWiki 对用户优先暴露一级命令。`wiki` 和 `governance` 是内部 runtime/domain，不作为普通用户命令 namespace。

```bash
spec-wiki status [--repo-root <path>]
spec-wiki update [--repo-root <path>] [--bridge-stdio]
spec-wiki query <term> [--repo-root <path>]
spec-wiki validate [change-id] [--repo-root <path>]
spec-wiki archive <change-id> [--repo-root <path>] [--dry-run] [--yes]
```

## 高级维护动作

`advanced` 是 help 分组，不是命令前缀。用户不需要输入 `spec-wiki advanced ...`。

```bash
spec-wiki sync [--repo-root <path>]
spec-wiki rebuild [--repo-root <path>] [--bridge-stdio]
spec-wiki changes [--repo-root <path>]
spec-wiki change <change-id> [--repo-root <path>]
spec-wiki doctor [--repo-root <path>]
spec-wiki repair [--repo-root <path>] [--dry-run] [--apply]
spec-wiki trace [--repo-root <path>]
```

## 参数或配置

| 名称 | 说明 |
| --- | --- |
| `--repo-root` | 目标仓库根目录 |
| `--host` / `--hosts` | bootstrap 目标宿主 |
| `--no-interactive` | 禁用交互选择 |
| `--bridge-stdio` | 长流程 runtime 命令的 stdio 桥接输出 |
| `--dry-run` | 预演将执行的写操作 |
| `--yes` | 跳过确认，执行已经通过验证的写操作 |
| `--apply` | 对 `repair` 执行实际修复；未提供时只报告计划 |

## 返回值或效果

- `init` 完成 SpecWiki 初始化，包含宿主资产和 repo-local runtime 基线。
- `status / query` 返回当前状态或查询结果。
- `update / sync / rebuild` 更新 repo-local runtime 产物。
- `validate` 无参数时验证 repo runtime；带 `change-id` 时验证指定 change。
- `archive` 只归档治理 change artifact 到 `.spec/archive/**`，必须先通过 validate；它不移动 `.wiki` runtime 产物。
- `repair` 默认只报告修复计划，显式传入 `--apply` 才修改文件。

## Query 稳定合同

`query` 的公开合同以 README、release 文档、CLI 源码和测试为事实来源。本页只记录长期使用边界：

- 查询词通过位置参数传入。
- 输出面向 Agent 快速定位文件、模块、符号、knowledge pages、projection refs、调用路径和 fallback 结果。
- JSON 主合同包含 `readiness`、`query_trust`、`recommended_action`、`route_groups`、`results`、`answer`、`summary` 和 `hits`。
- `results` 中每条结果携带 `route_tag / ref_kind / ref_id / label / score / provenance / confidence / recommended_action / source_refs`。
- `route_tag` 是闭集；宿主不得在 CLI/Skill 层自造私有 route tag。
- `provenance_summary` 是只读派生摘要，不能替代 `route_groups` / `results` 主合同。
- `governance_readiness` 当前为 `not_enabled` 占位；未启用治理 evidence index 不阻断普通 query。
- 查询不是通用聊天接口，也不替代 runtime 初始化、同步或更新。

## 限制与边界

- 当前正式发布目标是 Windows x64。
- `query` 需要位置参数查询词。
- `sync` 只同步受管 `.wiki` 页面编辑，不替代 `update`。
- `rebuild` 是显式全量重建，不是普通 `update` 的别名。
- `spec-wiki init` 是唯一对外初始化入口；runtime 初始化是该入口的内部阶段，不再作为另一套用户入口描述。
- `wiki` / `governance` namespace 只允许作为内部 runtime/domain 术语，不进入普通用户命令表。
- `advanced` 只作为 help 分组，不作为二级命令 namespace。
