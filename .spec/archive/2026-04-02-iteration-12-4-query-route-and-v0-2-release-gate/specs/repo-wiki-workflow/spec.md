## REMOVED Requirements

### Requirement: `v0.1.0` 的公开 workflow surface 必须收敛为 `init`、`status`、`update`、`query`
**Reason**: `v0.2.0` 继续保留同一组公开命令，但其发布语义已经从 `v0.1.0` 的 index-only 过渡态升级为 knowledge runtime first-class release。
**Migration**: 改为遵循新增的 `v0.2.0` 公开 workflow surface requirement；宿主与文档仍只暴露 `init / status / update / query`，但不再以 `v0.1.0` 口径描述。

### Requirement: `init` 与 `update` 的正式承诺必须收敛到 index-only runtime
**Reason**: `v0.2.0` 的正式成功语义不再接受“只有 facts/index 可查”作为 workflow 完成标准。
**Migration**: 改为遵循新增的 knowledge runtime requirement；`init / update` 必须以正式 `.knowledge + pages + metadata + cache` snapshot 作为发布口径。

### Requirement: `status` 必须表达 index-first readiness，而不是伪装成完整页面 runtime ready
**Reason**: `status` 在 `v0.2.0` 下必须回答 knowledge runtime 是否成立，以及恢复态/待更新态的推荐动作，而不只是 index-first readiness。
**Migration**: 改为遵循新增的 knowledge runtime readiness requirement；宿主仍可薄消费，但要按 `ready / stale / needs_update / blocker` 与恢复态语义消费。

### Requirement: `query` 必须以 index-first 结果为正式稳定合同
**Reason**: `v0.2.0` 的 query 稳定合同仍保持外部 `term-only`，但内部正式 route 已升级为 `index -> knowledge -> page fallback`。
**Migration**: 改为遵循新增的 query route requirement；调用方继续消费 `query_mode / query_trust / recommended_action / provenance_summary`，但不得再把 index-only 视为唯一正式结果层。

## ADDED Requirements

### Requirement: `v0.2.0` 的公开 workflow surface 必须继续收敛为 `init`、`status`、`update`、`query`
系统 MUST 把 `spec-wiki v0.2.0` 的公开 CLI workflow 继续限定为 `init`、`status`、`update`、`query` 四个入口。`sync` 与 `rebuild` MAY 在内部实现、验证脚本或后续迭代中存在，但 MUST NOT 被描述为 `v0.2.0` 的新增公开命令。

#### Scenario: 用户查看 `v0.2.0` 的公开 CLI 入口
- **WHEN** 用户阅读 README、release 说明、宿主技能或主包帮助文本
- **THEN** 公开 workflow 入口 MUST 仍只包含 `init`、`status`、`update`、`query`
- **THEN** 文档 MUST NOT 把 `sync` 或 `rebuild` 描述成 `v0.2.0` 的公开命令

#### Scenario: 宿主生成 `v0.2.0` 的显式技能入口
- **WHEN** 宿主 bootstrap 为 `v0.2.0` 生成显式 Wiki action skills
- **THEN** 公开暴露的显式入口 MUST 仍只覆盖 `wiki-init`、`wiki-status`、`wiki-update`、`wiki-query`
- **THEN** 宿主 MUST NOT 因 `v0.2.0` 就新增 `wiki-sync` 或 `wiki-rebuild` 之类的公开入口

### Requirement: `init` 与 `update` 的正式承诺必须收敛到 knowledge runtime
系统 MUST 把 `init` 与 `update` 的正式发布承诺收敛到 knowledge runtime，而不是继续停留在 index-only runtime。执行成功后，系统 MUST 至少保证 `.wiki/.knowledge/**`、`.wiki/pages/**`、`wiki.metadata.json` 与可重建 `.wiki/.cache/**` 的正式 snapshot 成立；仅有 facts/index snapshot 可查而缺失正式 knowledge/runtime 提交时，系统 MUST NOT 将该结果表述为 `v0.2.0` 的 workflow 成功。

#### Scenario: `init` 成功建立 knowledge runtime
- **WHEN** 用户在有效本地仓库上执行 `spec-wiki wiki init`
- **THEN** 系统 MUST 建立可供后续 `status`、`query` 与恢复链消费的正式 knowledge runtime snapshot
- **THEN** 系统 MUST NOT 仅凭 facts/index 可查就把缺失 `.knowledge / pages / metadata` 的结果表述为 `v0.2.0` init 成功

#### Scenario: `update` 以 knowledge-first refresh 提交成功
- **WHEN** 用户在已有 runtime 的仓库上执行 `spec-wiki wiki update`
- **THEN** 系统 MUST 以 knowledge-first refresh 更新正式 `.knowledge / pages / metadata / cache` snapshot
- **THEN** 系统 MUST NOT 继续把“只刷新 facts/index”表述为 `v0.2.0` update 的正式成功语义

### Requirement: `status` 必须表达 knowledge runtime readiness 与恢复态推荐动作
系统 MUST 把 `status` 作为 `v0.2.0` 的 knowledge runtime 检查入口，稳定表达当前仓库是否处于 `ready`、`stale`、`needs_update` 或 `blocker`，以及当前结果是否只是恢复态可消费 runtime。`status` MAY 暴露实现可见的详细诊断字段，但 MUST NOT 让调用方把“已恢复可查询”误解为“已达到 release-ready knowledge runtime”。

#### Scenario: 恢复态 runtime 可查询但尚未达到 release-ready
- **WHEN** 当前仓库已从正式产物恢复出本地 runtime，且 `query` 已可工作，但当前代码与正式快照不一致
- **THEN** `status` MUST 把该状态表达为 `stale` 或 `needs_update`
- **THEN** `status` MUST NOT 把这种状态表述成全新 `ready` 或 fresh release-ready runtime

#### Scenario: `status` 给出 knowledge runtime 推荐动作
- **WHEN** 用户执行 `spec-wiki wiki status`
- **THEN** 响应 MUST 能表达当前是否需要 `init`、`update` 或其它恢复动作
- **THEN** 宿主 MUST 能直接消费这些推荐动作，而不需要自行重建一套 `v0.2.0` 状态机

### Requirement: `query` 必须以 `index -> knowledge -> page fallback` 结果为正式稳定合同
系统 MUST 将 `query` 的正式稳定合同收敛到外部 `term-only`、内部 `index -> knowledge -> page fallback` 的结果路由。宿主与用户当前版本可稳定依赖的字段 MUST 继续以 `query_mode`、`query_trust`、`recommended_action`、`matched_pages` 和 `provenance_summary` 为主；其中 `provenance_summary` MUST 至少能稳定区分 `index_hit`、`knowledge_hit` 与 `page_fallback` 三类 route tags。其他实现可见字段 MAY 出现，但 MUST NOT 脱离该 route 语义漂移成新的公开 payload。

#### Scenario: `query` 优先返回 index 与 knowledge 命中
- **WHEN** 用户执行 `spec-wiki wiki query`，且当前 query 同时存在 facts/index 或 formal knowledge 命中
- **THEN** 响应 MUST 优先体现 `index` 与 `knowledge` 层结果
- **THEN** 调用方 MUST 能从 `provenance_summary` 看出当前命中属于 `index_hit` 或 `knowledge_hit`，而不是 page fallback 伪装

#### Scenario: 只有页面兜底时显式保留 fallback 语义
- **WHEN** 当前 query 没有足够的 index 或 knowledge 命中，只能依赖页面内容兜底
- **THEN** `query` MUST 继续返回可消费结果
- **THEN** 调用方 MUST 能稳定读取到 `page_fallback` provenance 与后续推荐动作
