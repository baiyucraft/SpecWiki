# repo-wiki-workflow Specification

## Purpose
定义 `spec-wiki v0.2.0` 当前真实公开支持的 workflow 合同。这个规范只描述打包版 CLI 与宿主可依赖的发布面，不把长期的 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble` 完整目标直接当作本版正式承诺。
## Requirements
### Requirement: `init` 与 `update` 的长流程协议必须保持可流式消费
系统 MUST 让 `init`、`update` 与 `rebuild` 保留可流式消费的事件协议，以便 CLI passthrough 与宿主桥接在不重写业务语义的前提下消费 `progress / result / error`。`status`、`query` 与 `sync` 继续作为短流程 JSON 调用即可。

#### Scenario: 宿主消费 `init` 或 `update` 的长流程事件
- **WHEN** CLI 或宿主调用 `init`、`update` 或 `rebuild`
- **THEN** 系统 MUST 允许调用方消费 `progress / result / error` 事件流
- **THEN** 调用方不得为了当前版本另建一套 workflow 事件语义

### Requirement: 当前正式公开 workflow surface 必须收敛为 `init`、`status`、`update`、`query`、`sync`、`rebuild`
系统 MUST 把当前版本的公开 CLI workflow 收敛为 `init`、`status`、`update`、`query`、`sync`、`rebuild` 六个入口。README、release 说明、宿主技能、主包帮助文本与公开 action skills MUST 对这六个入口保持一致，不得继续把 `sync` 或 `rebuild` 描述成 internal-only 能力。

#### Scenario: 用户查看当前正式公开 CLI 入口
- **WHEN** 用户阅读 README、release 说明、宿主技能或主包帮助文本
- **THEN** 公开 workflow 入口 MUST 包含 `sync` 与 `rebuild`
- **THEN** 文档 MUST NOT 再把两者描述成“未来迭代再开放”或“仅内部使用”

#### Scenario: 宿主生成当前版本的显式技能入口
- **WHEN** 宿主 bootstrap 为当前版本生成显式 Wiki action skills
- **THEN** 公开暴露的显式入口 MUST 覆盖 `wiki-init`、`wiki-status`、`wiki-update`、`wiki-query`、`wiki-sync`、`wiki-rebuild`
- **THEN** 宿主不得继续省略 `wiki-sync` 或 `wiki-rebuild`

### Requirement: 当前正式 release truth sources 必须对齐到同一 workflow 合同
系统 MUST 让 `README.md`、`README-CN.md`、release note、主包帮助文本、staged README 与主包版本号共同对齐到同一套正式 workflow 合同。当前版本一旦正式采用 `v0.2.0 minimal formal knowledge runtime` 与 6 个公开 workflow，系统 MUST NOT 再允许其中任一 truth source 继续描述旧版 workflow 入口或旧版 runtime contract。

#### Scenario: 用户读取任一正式 release 文档
- **WHEN** 用户查看 `README.md`、`README-CN.md`、release note 或 staged README
- **THEN** 文档 MUST 一致描述当前正式 runtime contract，而不是混用旧版 facts-only 叙事与 `v0.2.0` knowledge runtime
- **THEN** 文档 MUST 一致列出当前正式公开的 workflow 入口

#### Scenario: 主包版本号切到正式 release 口径
- **WHEN** 当前发布被命名为 `v0.2.0`
- **THEN** `packages/spec-wiki/package.json` 的版本号与 release 文档 MUST 使用同一版本口径
- **THEN** 系统 MUST NOT 继续保留与当前 release 叙事冲突的旧版本说明

### Requirement: `init` 与 `update` 的正式承诺必须收敛到 knowledge runtime
系统 MUST 把 `init` 与 `update` 的正式发布承诺收敛到 knowledge runtime，而不是继续停留在 facts-only runtime。执行成功后，系统 MUST 至少保证 `.wiki/.knowledge/**`、official page tree、`wiki.metadata.json` 与可重建 `.wiki/.cache/**` 的正式 snapshot 成立；official page tree 只包括 `.wiki/INDEX.md`、`.wiki/<栏目路径>/INDEX.md` 和 `.wiki/<栏目路径>/NN-主题.md`。仅有 facts/index snapshot 可查而缺失正式 knowledge/runtime 提交时，系统 MUST NOT 将该结果表述为 `v0.2.0` 的 workflow 成功。

#### Scenario: `init` 成功建立 knowledge runtime
- **WHEN** 用户在有效本地仓库上执行 `spec-wiki init`
- **THEN** 系统 MUST 建立可供后续 `status`、`query` 与恢复链消费的正式 knowledge runtime snapshot
- **THEN** 系统 MUST NOT 仅凭 facts/index 可查就把缺失 `.knowledge / official page tree / metadata` 的结果表述为 `v0.2.0` init 成功

#### Scenario: `update` 以 knowledge-first refresh 提交成功
- **WHEN** 用户在已有 runtime 的仓库上执行 `spec-wiki update`
- **THEN** 系统 MUST 以 knowledge-first refresh 更新正式 `.knowledge / official page tree / metadata / cache` snapshot
- **THEN** 系统 MUST NOT 继续把“只刷新 facts/index”表述为 `v0.2.0` update 的正式成功语义

### Requirement: `status` 必须表达 knowledge runtime readiness 与恢复态推荐动作
系统 MUST 把 `status` 作为 knowledge runtime 检查入口，稳定表达当前仓库是否处于 `ready`、`stale`、`needs_update` 或 `blocker`，并返回调用方可直接执行的单值 `recommended_action`。当前版本正式支持的推荐动作 MUST 至少包括 `none`、`init`、`update`、`sync`、`rebuild`。

#### Scenario: `status` 给出 knowledge runtime 推荐动作
- **WHEN** 用户执行 `spec-wiki status`，且当前 runtime 诊断需要页面回写同步或显式全量重建
- **THEN** 响应 MUST 返回 `recommended_action = sync` 或 `recommended_action = rebuild`
- **THEN** 宿主 MUST 能直接消费这些推荐动作，而不需要自行重建一套 host-side 状态机

### Requirement: `query` 必须以 canonical route groups 为正式稳定合同
系统 MUST 将 `query` 的公开输入保持为非空 `term-only`，并以 `readiness`、`query_mode`、`query_trust`、`recommended_action`、`governance`、`route_groups` 和 `answer` 作为 canonical response。`route_groups` MUST 是唯一结果 authority；已删除的顶层 `matched_pages`、`provenance_summary`、`summary`、`hits` 或平铺 `results` MUST NOT 作为兼容字段继续暴露。

#### Scenario: `query` 返回分组结果与 Runtime answer
- **WHEN** 用户执行 `spec-wiki query`，且当前 query 存在一个或多个 route 命中
- **THEN** 响应 MUST 在 `route_groups` 中按 Runtime 定义的组内 rank 提供结果与 supporting refs
- **THEN** 调用方 MUST 使用 Runtime `answer`，不得在宿主层重建 route、ranking 或结论

#### Scenario: 合法 term 无命中
- **WHEN** query 输入合法但没有任何 route 命中
- **THEN** `route_groups` MUST 序列化为空数组并返回 empty `answer`
- **THEN** 系统不得伪装 typed failure，也不得恢复已删除旧字段

### Requirement: `sync` 必须作为正式公开 workflow 同步页面回写
系统 MUST 将 `sync` 作为正式公开 workflow，用于把 `.wiki` 受管页面的人工编辑、managed drift 与 section 结构变化同步回 runtime state、metadata 与本地 cache。`sync` 只处理页面层 contract，不得被表述成源码扫描、knowledge refresh 或普通增量更新的替代物。

#### Scenario: 用户显式执行 `spec-wiki sync`
- **WHEN** 用户修改 `.wiki` 页面后执行 `spec-wiki sync`
- **THEN** 系统 MUST 回写相关页面状态、metadata 与本地 cache
- **THEN** 系统 MUST NOT 因该动作重新进入 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble`

### Requirement: `rebuild` 必须作为正式公开 workflow 执行显式全量重建
系统 MUST 将 `rebuild` 作为正式公开 workflow，用于调用方显式要求强制全量重建 runtime。`rebuild` MUST 保持独立于普通 `update` 的语义边界，并继续作为 long-running workflow 对外暴露。

#### Scenario: 用户显式执行 `spec-wiki rebuild`
- **WHEN** 用户或宿主明确调用 `spec-wiki rebuild`
- **THEN** 系统 MUST 执行正式 runtime 的全量重建
- **THEN** 系统 MUST NOT 把该动作表述成普通 `update` 的别名或隐式 fallback 文案

### Requirement: `sync` 必须返回稳定的知识回写结果分类
系统 MUST 让 `sync` 返回稳定的知识回写结果分类，至少区分 `declared_writeback`、`metadata_only` 与 `illegal_drift`。`sync` 结果 MUST 同时返回受影响对象引用与推荐动作，使调用方能区分“已写回正式知识”“只更新 runtime 状态”“必须人工处理或 rebuild”。

#### Scenario: sync 只命中 metadata-only 变更
- **WHEN** 用户编辑的内容只影响标题、排序、section hash 或其它不改变知识真相的 projection 元数据
- **THEN** `sync` MUST 将其标记为 `metadata_only`
- **THEN** 系统 MUST NOT 伪装成 declared knowledge 已被写回

#### Scenario: sync 发现非法 drift
- **WHEN** 用户进行了无法映射为受管回写的页面编辑
- **THEN** `sync` MUST 返回 `illegal_drift`
- **THEN** 结果 MUST 同时提供稳定 `recommended_action`，例如 `rebuild` 或人工修正

### Requirement: `status` 必须同时表达 readiness 与 knowledge health
系统 MUST 让 `status` 同时表达 knowledge runtime readiness 与 knowledge health 摘要，而不是只输出单层存在性状态。调用方 MUST 能稳定区分“ready but degraded”“needs_update because derived stale”“blocker because illegal drift or unrecoverable mismatch”等情况。

#### Scenario: ready 但存在 health degradation
- **WHEN** 当前 runtime 可被 `query` 消费，但存在 orphan unit、projection stale 或 missing provenance
- **THEN** `status` MUST 继续返回 readiness
- **THEN** `status` MUST 同时返回 health 摘要与可执行推荐动作

#### Scenario: illegal drift 进入 workflow 级诊断
- **WHEN** 最近一次 `sync` 产生了 `illegal_drift`
- **THEN** 后续 `status` MUST 能反映该知识层诊断
- **THEN** 系统 MUST 不得把该状态压平为普通 `needs_update`
