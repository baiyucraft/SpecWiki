# repo-wiki-workflow Specification

## Purpose
定义 `spec-wiki v0.1.0` 当前真实公开支持的 workflow 合同。这个规范只描述打包版 CLI 与宿主可依赖的发布面，不把长期的 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble` 完整目标直接当作本版正式承诺。

## Requirements
### Requirement: `v0.1.0` 的公开 workflow surface 必须收敛为 `init`、`status`、`update`、`query`
系统 MUST 把 `spec-wiki v0.1.0` 的公开 CLI workflow 限定为 `init`、`status`、`update`、`query` 四个入口。`sync` 与 `rebuild` MAY 在内部实现、测试或后续迭代中存在，但 MUST NOT 被描述为当前版本的公开支持面。

#### Scenario: 用户查看公开 CLI 入口
- **WHEN** 用户阅读 README、release 说明、宿主技能或主包帮助文本
- **THEN** 公开 workflow 入口 MUST 只包含 `init`、`status`、`update`、`query`
- **THEN** 文档不得把 `sync` 或 `rebuild` 描述成 `v0.1.0` 的正式公开命令

#### Scenario: 宿主生成显式技能入口
- **WHEN** 宿主 bootstrap 为当前版本生成显式 Wiki action skills
- **THEN** 当前版本公开暴露的显式入口 MUST 只覆盖 `wiki-init`、`wiki-status`、`wiki-update`、`wiki-query`
- **THEN** 不得为 `sync` 或 `rebuild` 生成当前版本的公开显式入口

### Requirement: `init` 与 `update` 的正式承诺必须收敛到 index-only runtime
系统 MUST 把 `init` 与 `update` 的正式发布承诺收敛到 index-first、facts/index 可查询的 runtime，而不是完整 knowledge/page 终态。执行成功后，系统 MUST 至少保证 `.wiki/.cache/wiki-cache.db` 和等价 facts/index snapshot 可用；`wiki.metadata.json`、`.wiki/pages/**`、`.wiki/.knowledge/**` 等完整页面产物 MAY 缺失，且缺失本身 MUST NOT 使 `v0.1.0` 发布合同失真。

#### Scenario: `init` 成功建立 index-only runtime
- **WHEN** 用户在有效本地仓库上执行 `spec-wiki wiki init`
- **THEN** 系统 MUST 建立可供后续 `status` 与 `query` 使用的本地 index-first runtime
- **THEN** 系统 MUST NOT 把完整 knowledge/page 产物缺失视为 `v0.1.0` init 失败

#### Scenario: `update` 刷新 index-only runtime
- **WHEN** 用户在已有 runtime 的仓库上执行 `spec-wiki wiki update`
- **THEN** 系统 MUST 刷新 facts/index 层，使后续 `status` 与 `query` 看到最新源码状态
- **THEN** 系统 MUST NOT 因完整 knowledge/page runtime 尚未完成就否定本轮 `update` 的正式成功语义

### Requirement: `status` 必须表达 index-first readiness，而不是伪装成完整页面 runtime ready
系统 MUST 把 `status` 作为当前 runtime 检查入口，稳定表达 facts/index 是否可用、repo 是否 stale，以及推荐的下一步动作。`status` MAY 暴露实现可见的 runtime 诊断字段，但 MUST NOT 让调用方把当前结果误解为完整 knowledge/page runtime 已 ready。

#### Scenario: index-only runtime 可查询但页面层未完成
- **WHEN** 当前仓库已有 facts/index snapshot，且 `query` 已可工作，但页面层或 research/compose/assemble 尚未完成
- **THEN** `status` MUST 继续把当前状态表达为可查询的 index-first runtime
- **THEN** `status` MUST NOT 把这种状态表述成完整 knowledge/page runtime ready

#### Scenario: `status` 给出推荐动作
- **WHEN** 用户执行 `spec-wiki wiki status`
- **THEN** 响应 MUST 能表达当前是否需要 `init`、`update` 或无需动作
- **THEN** 宿主可以直接薄消费该结果，而不需要重建宿主侧 Wiki 状态机

### Requirement: `query` 必须以 index-first 结果为正式稳定合同
系统 MUST 将 `query` 的正式稳定合同收敛到 index-first 结果。宿主与用户当前版本可稳定依赖的字段 MUST 以 `query_mode`、`query_trust`、`recommended_action`、`matched_pages` 和 `provenance_summary` 为主；其他实现可见字段 MAY 出现，但 MUST NOT 被升级为本版正式公开合同。

#### Scenario: `query` 返回 index-first 稳定字段
- **WHEN** 用户执行 `spec-wiki wiki query`
- **THEN** 响应 MUST 以 index-first 结果为主
- **THEN** 调用方 MUST 能稳定读取 `query_mode`、`query_trust`、`recommended_action`、`matched_pages` 和 `provenance_summary`

#### Scenario: 页面层未完成时 `query` 仍可工作
- **WHEN** 当前 runtime 仍处于 index-only 或 downstream incomplete 状态
- **THEN** `query` MUST 继续基于已提交的 facts/index snapshot 返回结构化结果
- **THEN** 调用方 MUST NOT 被迫等待完整页面 runtime 才能执行当前版本的正式查询能力

### Requirement: `init` 与 `update` 的长流程协议必须保持可流式消费
系统 MUST 让 `init` 与 `update` 继续保留可流式消费的事件协议，以便 CLI passthrough 与宿主桥接在不重写业务语义的前提下消费 `progress / result / error`。`status` 与 `query` 继续作为短流程 JSON 调用即可。

#### Scenario: 宿主消费 `init` 或 `update` 的长流程事件
- **WHEN** CLI 或宿主调用 `init` 或 `update`
- **THEN** 系统 MUST 允许调用方消费 `progress / result / error` 事件流
- **THEN** 调用方不得为了当前版本另建一套 workflow 事件语义