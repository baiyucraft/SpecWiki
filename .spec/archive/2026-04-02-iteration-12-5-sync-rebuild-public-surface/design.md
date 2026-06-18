# Design: iteration-12-5-sync-rebuild-public-surface

## Context
当前仓库已经具备内部 `sync` 与 `rebuild` workflow、相应 runtime transport，以及 `RecommendedAction::Sync / Rebuild`。问题不在于缺少底层实现，而在于公开合同仍停留在 `init / status / update / query` 四入口，导致：

- CLI/help 明确拒绝 `sync` 与 `rebuild`
- 宿主 action skills 仍禁止公开 `wiki-sync` 与 `wiki-rebuild`
- `status/query` 已能建议 `sync/rebuild`，但调用方没有正式入口可执行

因此这轮应做 public surface uplift，而不是 runtime 重构。

## Goals
- 让 `sync` 与 `rebuild` 成为正式公开 workflow
- 让 CLI、宿主、验证与 spec 对同一套 workflow surface 达成一致
- 让调用方可以把已有 `recommended_action = sync | rebuild` 当成可执行入口，而不是死链接

## Non-Goals
- 不改变 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble` 主链
- 不扩展 query 外部输入或结果结构
- 不新增新的 runtime 状态枚举或宿主侧状态机
- 不修改 `status/query` 的推荐判定逻辑

## Decisions
### 1. 这轮是 public surface release，不是 runtime semantics change
本轮只收口合同与入口，不改变 `sync/update/rebuild` 现有实现语义，不重写 lifecycle。

### 2. `sync` 被钉死为页面回写入口
`sync` 只公开为 `.wiki` 受管页面、managed section 与相关 runtime/metadata 回写同步能力。它不是源码变更刷新，不是 `update` 的别名，也不是轻量 `rebuild`。

### 3. `rebuild` 被钉死为显式 full rebuild 入口
`rebuild` 继续表示调用方显式要求的强制全量重建，并沿用现有 long-running `progress / result / error` 协议。它不是 `repo_replan` 的公开名字，也不是普通 `update` 的更强模式。

### 4. `recommended_action` 只做公开入口对齐，不做语义扩写
`status/query` 中已有的 `recommended_action = sync | rebuild` 在本轮只用于说明“公开入口现已存在”。本轮不新增字段，不修改推荐条件，不引入 host-side 状态机。

### 5. transport 维持现有长短流程分工
- `sync` 保持短流程 JSON action
- `rebuild` 保持 long-running action，并继续走现有 `progress / result / error` 事件流
- `status` 与 `query` 继续保持短流程 JSON

## Release Surface
本轮必须同时收口以下公开面：
- UniSpec workflow、verification、CodeBuddy specs
- 主包 CLI help 与命令合同
- 宿主 bootstrap 输出的 action skills / prompts / command assets
- public-surface 自动化验证

本轮明确不改：
- runtime 推荐动作判定逻辑
- query payload
- 新的 lifecycle 状态枚举
- host-side wiki state machine
- knowledge/page semantic 层重构

## Acceptance Gate
- UniSpec change 明确公开 `sync/rebuild` 合同与非目标范围
- CLI/help/宿主 action skills 不再把 `sync/rebuild` 标记为非公开
- public-surface 验证证明 `sync` 与 `rebuild` 已在 CLI 和宿主两侧同步开放
- 验证至少覆盖 CLI/public surface、CodeBuddy action skills、`storybook` 的代表性闭环，以及 `.wiki/02-开发指南/00-代码注释规范.md` 检查任务

