# Change Proposal: iteration-12-5-sync-rebuild-public-surface

## Why
`sync` 与 `rebuild` 已经在 runtime 内部存在真实实现，`status/query` 也已经返回 `recommended_action = sync | rebuild`，但当前公开合同仍把它们排除在 CLI、帮助文本、宿主 action skills 和正式 spec 之外。结果是实现、诊断与发布面彼此撕裂：runtime 已经知道什么时候该 `sync` 或 `rebuild`，外部调用方却不能把这两个动作当成稳定入口来依赖。

这轮 change 只解决公开合同撕裂问题，把已存在的 `sync/rebuild` 升级成正式 public surface，不重写 runtime 主链，也不改现有 `status/query` 的推荐判定逻辑。

## What Changes
- 将公开 workflow surface 从 `init / status / update / query` 扩展为 `init / status / update / query / sync / rebuild`
- 明确 `sync` 的公开定位是 `.wiki` 页面回写同步，而不是源码刷新或 `update` 别名
- 明确 `rebuild` 的公开定位是显式强制全量重建，并继续沿用现有 long-running transport
- 对齐 CLI/help、宿主 bootstrap 资产、CodeBuddy action skills 与 public-surface 验证
- 将现有 `recommended_action = sync | rebuild` 视为公开入口对齐依据，但不新增字段、不修改推荐条件

## Non-Goals
- 不重写 lifecycle 状态机
- 不引入新的 query payload 或 query transport
- 不修改 `status/query` 的决策逻辑或字段语义
- 不重定义 knowledge lifecycle、page semantics 或新的宿主状态机
- 不把本轮扩大成 19 项目全量回归或完整 lifecycle 2.0 重做

## Impacted Capabilities
### Modified
- `repo-wiki-workflow`
- `workflow-verification`
- `codebuddy-agent-integration`

### Added
- None
