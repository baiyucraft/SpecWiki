## MODIFIED Requirements

### Requirement: `init` 与 `update` 的长流程协议必须保持可流式消费
系统 MUST 让 `init`、`update` 与 `rebuild` 保留可流式消费的事件协议，以便 CLI passthrough 与宿主桥接在不重写业务语义的前提下消费 `progress / result / error`。`status`、`query` 与 `sync` 继续作为短流程 JSON 调用即可。

#### Scenario: 宿主消费 `init` 或 `update` 的长流程事件
- **WHEN** CLI 或宿主调用 `init`、`update` 或 `rebuild`
- **THEN** 系统 MUST 允许调用方消费 `progress / result / error` 事件流
- **THEN** 调用方不得为了当前版本另建一套 workflow 事件语义

### Requirement: `v0.2.0` 的公开 workflow surface 必须继续收敛为 `init`、`status`、`update`、`query`
系统 MUST 把当前版本的公开 CLI workflow 收敛为 `init`、`status`、`update`、`query`、`sync`、`rebuild` 六个入口。README、release 说明、宿主技能、主包帮助文本与公开 action skills MUST 对这六个入口保持一致，不得继续把 `sync` 或 `rebuild` 描述成 internal-only 能力。

#### Scenario: 用户查看 `v0.2.0` 的公开 CLI 入口
- **WHEN** 用户阅读 README、release 说明、宿主技能或主包帮助文本
- **THEN** 公开 workflow 入口 MUST 包含 `sync` 与 `rebuild`
- **THEN** 文档 MUST NOT 再把两者描述成“未来迭代再开放”或“仅内部使用”

#### Scenario: 宿主生成 `v0.2.0` 的显式技能入口
- **WHEN** 宿主 bootstrap 为当前版本生成显式 Wiki action skills
- **THEN** 公开暴露的显式入口 MUST 覆盖 `wiki-init`、`wiki-status`、`wiki-update`、`wiki-query`、`wiki-sync`、`wiki-rebuild`
- **THEN** 宿主不得继续省略 `wiki-sync` 或 `wiki-rebuild`

### Requirement: `status` 必须表达 knowledge runtime readiness 与恢复态推荐动作
系统 MUST 把 `status` 作为 knowledge runtime 检查入口，稳定表达当前仓库是否处于 `ready`、`stale`、`needs_update` 或 `blocker`，并返回调用方可直接执行的单值 `recommended_action`。当前版本正式支持的推荐动作 MUST 至少包括 `none`、`init`、`update`、`sync`、`rebuild`。

#### Scenario: `status` 给出 knowledge runtime 推荐动作
- **WHEN** 用户执行 `spec-wiki wiki status`，且当前 runtime 诊断需要页面回写同步或显式全量重建
- **THEN** 响应 MUST 返回 `recommended_action = sync` 或 `recommended_action = rebuild`
- **THEN** 宿主 MUST 能直接消费这些推荐动作，而不需要自行重建一套 host-side 状态机

## ADDED Requirements

### Requirement: `sync` 必须作为正式公开 workflow 同步页面回写
系统 MUST 将 `sync` 作为正式公开 workflow，用于把 `.wiki` 受管页面的人工编辑、managed drift 与 section 结构变化同步回 runtime state、metadata 与本地 cache。`sync` 只处理页面层 contract，不得被表述成源码扫描、knowledge refresh 或普通增量更新的替代物。

#### Scenario: 用户显式执行 `spec-wiki wiki sync`
- **WHEN** 用户修改 `.wiki` 页面后执行 `spec-wiki wiki sync`
- **THEN** 系统 MUST 回写相关页面状态、metadata 与本地 cache
- **THEN** 系统 MUST NOT 因该动作重新进入 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble`

### Requirement: `rebuild` 必须作为正式公开 workflow 执行显式全量重建
系统 MUST 将 `rebuild` 作为正式公开 workflow，用于调用方显式要求强制全量重建 runtime。`rebuild` MUST 保持独立于普通 `update` 的语义边界，并继续作为 long-running workflow 对外暴露。

#### Scenario: 用户显式执行 `spec-wiki wiki rebuild`
- **WHEN** 用户或宿主明确调用 `spec-wiki wiki rebuild`
- **THEN** 系统 MUST 执行正式 runtime 的全量重建
- **THEN** 系统 MUST NOT 把该动作表述成普通 `update` 的别名或隐式 fallback 文案
