---
title: SpecWiki CLI 一级命令统一设计
description: 将 SpecWiki 用户侧 CLI 收敛为一级命令，隐藏 wiki / governance runtime namespace
updated: 2026-06-13
owner: architecture
status: draft
---

# SpecWiki CLI 一级命令统一设计

## 文档定位

本文是阶段性设计稿，用于统一 SpecWiki 的 CLI 产品面。它不表示当前代码已经完成迁移，也不要求保留旧二级命令兼容层。

当前原则：

- 用户侧只暴露一级命令。
- `wiki` 和 `governance` 是 runtime/domain 内部概念，不再作为普通用户命令 namespace。
- `spec-wiki init` 是唯一初始化入口。
- 当前处于测试开发阶段，旧命令、旧参数和旧 wrapper 可以在迁移完成后删除。

## 背景问题

现有设计曾把命令分成：

```text
spec-wiki init
spec-wiki wiki <action>
spec-wiki governance <action>
```

这会带来三个问题：

- 用户需要先理解 `wiki` / `governance` 两套 namespace，再决定命令入口。
- `init` 容易被拆成宿主 bootstrap 与 runtime init 两条路径。
- Agent / Skill 文案会继续把内部 runtime 分层暴露给用户，形成多套心智。

目标不是减少 runtime 能力，而是把能力重新映射到更自然的用户意图。

## 设计原则

### 1. 用户意图优先

CLI 命令表达用户想做什么，而不是内部模块在哪里。

```text
初始化 -> init
看状态 -> status
刷新 -> update
查询 -> query
验证 -> validate
归档 -> archive
```

### 2. 一级命令优先

默认 help、快速上手、Agent 入口和宿主 skill 只展示一级命令。

### 3. runtime namespace 内部化

`wiki`、`governance`、`index`、`knowledge`、`runtime` 都是内部领域词。它们可以存在于代码模块、DTO、trace 和 debug 输出中，但不作为普通用户命令前缀。

`advanced` 也只是 help 分组，不是命令前缀。系统不得把旧的二级 namespace 换成新的 `advanced` 二级 namespace。

### 4. 初始化只保留一个入口

`spec-wiki init` 统一完成：

- host bootstrap
- index / graph 初始构建
- knowledge runtime 初始构建
- `.wiki` 页面投影
- governance discovery
- Agent 入口安装

实现内部可以拆 phase，但用户侧不暴露第二个 init。

### 5. 高级维护不进入主路径

`sync / rebuild / doctor / repair / trace` 等能力保留，但放在 advanced help，不进入 Quick Start 主路径。

## 目标命令面

### Public Commands

```text
spec-wiki init
spec-wiki status
spec-wiki update
spec-wiki query <term>
spec-wiki validate [change-id]
spec-wiki archive <change-id>
```

含义：

| 命令 | 用户意图 | 内部路由 |
| --- | --- | --- |
| `init` | 初始化 SpecWiki | bootstrap + runtime init + governance discovery |
| `status` | 查看仓库知识与治理状态 | wiki status + governance status |
| `update` | 源码、文档或治理 artifact 变化后刷新 | index update + knowledge update + governance derived state |
| `query <term>` | 查询代码、知识、页面和治理证据 | index query + knowledge query + governance refs |
| `validate [change-id]` | 验证 runtime 或指定 change | repo validation 或 governance validate |
| `archive <change-id>` | 归档治理 change | governance archive workflow |

`status / update / query` 默认融合 Wiki runtime 与 governance runtime。没有 `.spec` 或未启用治理的仓库，治理部分应返回 `not_enabled`，不得阻断普通 Wiki 能力。

### Advanced Commands

```text
spec-wiki sync
spec-wiki rebuild
spec-wiki changes
spec-wiki change <change-id>
spec-wiki doctor
spec-wiki repair
spec-wiki trace
```

含义：

| 命令 | 用户意图 | 说明 |
| --- | --- | --- |
| `sync` | 将合法页面编辑同步回 knowledge | 高级维护，不替代 `update` |
| `rebuild` | 显式全量重建 runtime | 高成本动作，需清楚提示 |
| `changes` | 列出 active governance changes | 只读 |
| `change <change-id>` | 查看单个 change 摘要 | 等价于旧 show 语义 |
| `doctor` | 诊断配置、产物、缓存、宿主资产 | 不修改文件 |
| `repair` | 修复可自动恢复的问题 | 默认只报告计划；必须显式 `--apply` 才修改 |
| `trace` | 输出最近 workflow trace | debug / support |

Advanced commands 可以出现在完整 help 中，但 Quick Start 只链接到 CLI 页，不展开这些命令。

`changes` 与 `change <change-id>` 只能承担 list / inspect 语义，不得扩展成新的隐形治理 namespace。

## 命令语义细节

### `init`

```text
spec-wiki init [--host <host> | --hosts <host1,host2>] [--repo-root <path>] [--no-interactive] [--dry-run]
```

`--host` 取代 `--tool`，因为 Codex、Claude、CodeBuddy 是宿主，而不是普通工具。

`init` 的输出应说明每个阶段状态：

```text
host bootstrap: ready / skipped / blocked
index: ready / blocked
knowledge: ready / blocked
wiki projection: ready / skipped / blocked
governance: ready / not_enabled / blocked
next action: none / update / validate / doctor
```

### `status`

```text
spec-wiki status [--repo-root <path>] [--json]
```

返回统一状态，不要求用户自己分别执行 Wiki 状态和治理状态。

### `update`

```text
spec-wiki update [--repo-root <path>] [--dry-run] [--bridge-stdio]
```

用于日常变化后的增量刷新。它不负责 archive，也不默认执行 destructive repair。

### `query`

```text
spec-wiki query <term> [--repo-root <path>] [--json]
```

查询结果必须显式区分来源：

```text
index_graph
declared_knowledge
derived_knowledge
wiki_projection
governance_evidence_ref
```

### `validate`

```text
spec-wiki validate [change-id] [--repo-root <path>] [--json]
```

双语义：

- 无 `change-id`：验证当前 workspace 的 SpecWiki runtime、配置、产物、公开文档合同和宿主资产。
- 有 `change-id`：验证指定 change 的 artifact、stage、review gate 和 archive readiness。

为了降低歧义，输出第一行必须明确 validation mode：

```text
mode: workspace
```

或：

```text
mode: change
change: <change-id>
```

无参数时不得隐式选择某个 active change。即使当前只有一个 active change，也只能提示用户显式执行 `spec-wiki validate <change-id>`。

如果后续发现双语义在实现或用户测试中不够清晰，可拆成：

```text
spec-wiki validate
spec-wiki validate-change <change-id>
```

但第一版优先保留 `validate [change-id]`，避免命令膨胀。

### `archive`

```text
spec-wiki archive <change-id> [--repo-root <path>] [--dry-run] [--yes]
```

默认先执行 validate。没有通过 validate 时不得移动目录。

`archive` 只归档 governance change artifact 到 `.spec/archive/**`。它不得移动 `.wiki` runtime 产物、不得归档整个项目、不得清理 release 文件。

`archive` 必须输出 operation manifest，包括：

- source path
- target path
- artifact hash summary
- validation result
- wiki-sync issue summary
- executed steps
- recovery hint

## 内部路由模型

用户命令不直接等于 runtime 模块名。

```text
spec-wiki <command>
  -> CLI command router
  -> IntentCommand
  -> RuntimeAction
  -> wiki-runtime / governance service
  -> unified response DTO
```

示例映射：

| 用户命令 | RuntimeAction |
| --- | --- |
| `init` | `runtime.init` + `governance.discover` |
| `status` | `runtime.status` + `governance.status` |
| `update` | `runtime.update` + `governance.refresh` |
| `query` | `runtime.query` + `governance.query_refs` |
| `validate` | `runtime.validate` |
| `validate <change-id>` | `governance.validate` |
| `archive <change-id>` | `governance.archive` |
| `sync` | `runtime.sync` |
| `rebuild` | `runtime.rebuild` |
| `doctor` | `runtime.doctor` + `host.doctor` |
| `repair` | `runtime.repair` + `host.repair` |
| `trace` | `runtime.trace` |

## Help 信息分层

默认 help：

```text
Usage:
  spec-wiki init [options]
  spec-wiki status [options]
  spec-wiki update [options]
  spec-wiki query <term> [options]
  spec-wiki validate [change-id] [options]
  spec-wiki archive <change-id> [options]

Advanced:
  spec-wiki sync [options]
  spec-wiki rebuild [options]
  spec-wiki changes [options]
  spec-wiki change <change-id> [options]
  spec-wiki doctor [options]
  spec-wiki repair [options]
  spec-wiki trace [options]
```

Quick Start 只展示：

```text
spec-wiki init
spec-wiki status
spec-wiki query <term>
spec-wiki update
```

治理场景页再展示：

```text
spec-wiki changes
spec-wiki change <change-id>
spec-wiki validate <change-id>
spec-wiki archive <change-id>
```

## 参数统一

通用参数：

```text
--repo-root <path>
--json
--verbose
--quiet
```

确认和预演：

```text
--dry-run
--yes
--apply
```

宿主初始化：

```text
--host <host>
--hosts <host1,host2>
--no-interactive
```

长流程：

```text
--bridge-stdio
```

废弃：

```text
--tool
--tools
```

测试开发阶段不保留长期兼容 alias。迁移期如果需要临时接受旧参数，必须输出 deprecation warning，并在同一 change 中删除。

`repair` 默认不修改文件；只有显式传入 `--apply` 才执行实际修复。`archive` 使用 `--yes` 跳过确认，但仍必须先通过 validate。

## 文档迁移规则

必须删除用户侧二级命令示例：

```text
spec-wiki wiki ...
spec-wiki governance ...
```

替换为一级命令。

允许保留的内部表述：

```text
runtime action
runtime namespace
governance service
internal forwarding
expert/debug trace
```

但这些表述不得出现在 Quick Start 的命令表里。

## Agent / Skill 迁移规则

Agent / Skill 入口统一调用一级命令：

```text
spec-wiki status
spec-wiki query <term>
spec-wiki update
spec-wiki validate <change-id>
```

Skill 不再写：

```text
spec-wiki wiki status
spec-wiki governance validate
```

Skill 可以在说明中提到 runtime / governance 作为响应中的状态字段，但不能把它们暴露成用户要选择的命令 namespace。

## 实施计划

### Phase 1: 文档合同

- 新增本文。
- 同步 `.wiki/04-对外方法/00-CLI.md`。
- 同步 `.wiki/06-设计文档/02-Agents设计.md`。
- 同步 `.docs/design/governance-runtime-integration.md`。
- 快速上手只保留一级命令。

### Phase 2: TS CLI Router

- 将 `cli.ts` 改为一级命令 router。
- `init / status / update / query / validate / archive / sync / rebuild / changes / change / doctor / repair / trace` 都在顶层解析。
- 删除或迁移 `wiki <action>` 用户入口。
- 删除或迁移 `governance <action>` 用户入口。

### Phase 3: Runtime DTO

- 新增 IntentCommand 或等价 command mapping。
- `status / update / query` 聚合 wiki runtime 与 governance runtime。
- `validate` 支持 repo scope 与 change scope。

### Phase 4: Host Assets

- 更新 `.agents` / `.codex` / 其它宿主资产。
- Skill 只调用一级命令。
- Host bootstrap 生成的 command / prompt 不再出现二级命令。

### Phase 5: Tests and Cleanup

- 更新 CLI tests。
- 更新 scripts 中的命令调用。
- 更新 release / docs / wiki。
- 全仓扫描：

```text
rg "spec-wiki wiki|spec-wiki governance"
```

除历史 release 记录或明确标注的 migration note 外，不应再有用户侧命令示例。

## 风险与控制

| 风险 | 后果 | 控制 |
| --- | --- | --- |
| `validate` 双语义不清晰 | 用户不知道验证 workspace 还是 change | 输出 `mode: workspace/change`，help 中明确 `[change-id]` |
| `archive` 过早暴露 | 误移动 change artifact | 默认 validate，支持 dry-run，需确认 |
| `archive` 语义过宽 | 用户误以为会归档 `.wiki` 或整个项目 | help 中写死只归档 `.spec` change artifact |
| 删除二级命令影响宿主资产 | Agent 调用失败 | 同步更新 host assets 和 tests |
| `sync / rebuild` 被误当日常主命令 | 用户过度使用重操作 | 放入 advanced help |
| `wiki` namespace 残留在 docs | 双入口心智复发 | 全仓扫描和 reviewer gate |
| `advanced` 变成新 namespace | 又形成二级命令心智 | advanced 只作为 help 分组，不作为命令前缀 |
| `--host` 替换 `--tool` 造成测试漂移 | CLI tests 失败 | 同 change 更新 tests，测试开发阶段不保留长期 alias |

## 成功标准

- Quick Start 只展示一级命令。
- 默认 help 只以一级命令作为用户入口。
- 文档中不再把 `wiki` / `governance` 写成普通用户 namespace。
- 文档中不把 `advanced` 写成命令 namespace，只作为 help 分组。
- `spec-wiki init` 是唯一初始化入口。
- `status / update / query` 默认融合 Wiki runtime 与 governance runtime 状态。
- Agent / Skill 不再要求用户选择 `wiki` 或 `governance` 前缀。

## 结论

SpecWiki CLI 应收敛为“一级命令表达用户意图，内部 runtime 承担领域分发”。

最终用户心智应是：

```text
init -> status -> query / update -> validate -> archive
```

而不是：

```text
init + wiki namespace + governance namespace
```
