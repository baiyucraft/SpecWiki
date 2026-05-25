## MODIFIED Requirements

### Requirement: `init` 与 `update` 的正式承诺必须收敛到 knowledge runtime
系统 MUST 把 `init` 与 `update` 的正式发布承诺收敛到 knowledge runtime，而不是继续停留在 facts-only runtime。执行成功后，系统 MUST 至少保证 `.wiki/.knowledge/**`、`.wiki/pages/**`、`wiki.metadata.json` 与可重建 `.wiki/.cache/**` 的正式 snapshot 成立；仅有 facts/index snapshot 可查而缺失正式 knowledge/runtime 提交时，系统 MUST NOT 将该结果表述为 `v0.2.0` 的 workflow 成功。对于 `update`，系统 MUST 同时消费源码 dirty set 与 declared lifecycle 产生的 stale scope；即使源码层没有新增 dirty files，只要 declared scope、status 或 relation 变化导致正式 stale scope 成立，系统也 MUST 刷新受影响的 derived / projection snapshot。

#### Scenario: `init` 成功建立 knowledge runtime
- **WHEN** 用户在有效本地仓库上执行 `spec-wiki wiki init`
- **THEN** 系统 MUST 建立可供后续 `status`、`query` 与恢复链消费的正式 knowledge runtime snapshot
- **THEN** 系统 MUST NOT 仅凭 facts/index 可查就把缺失 `.knowledge / pages / metadata` 的结果表述为 `v0.2.0` init 成功

#### Scenario: `update` 以 declared-aware refresh 提交成功
- **WHEN** 用户在已有 runtime 的仓库上执行 `spec-wiki wiki update`，且当前 declared lifecycle 标记了 stale scope
- **THEN** 系统 MUST 刷新受影响的 derived / projection / metadata snapshot
- **THEN** 系统 MUST NOT 因源码 dirty set 为空就跳过这次 update

### Requirement: `status` 必须表达 knowledge runtime readiness 与恢复态推荐动作
系统 MUST 把 `status` 作为 knowledge runtime 检查入口，稳定表达当前仓库是否处于 `ready`、`stale`、`needs_update` 或 `blocker`，并返回调用方可直接执行的单值 `recommended_action`。当前版本正式支持的推荐动作 MUST 至少包括 `none`、`init`、`update`、`sync`、`rebuild`。当 declared lifecycle 产生 `deprecated`、`replaced_by`、illegal drift 或下游 stale 时，`status` MUST 能将这些诊断折叠进稳定 readiness / recommended_action 输出，而不是把它们压平成普通存在性检查。

#### Scenario: `status` 给出 declared-aware 推荐动作
- **WHEN** 用户执行 `spec-wiki wiki status`，且当前 runtime 诊断发现 declared lifecycle 变更已使下游 projection 过时
- **THEN** 响应 MUST 返回与 declared 诊断一致的 `recommended_action`
- **THEN** 宿主 MUST 不需要自行重建一套 declared lifecycle 状态机

### Requirement: `sync` 必须返回稳定的知识回写结果分类
系统 MUST 让 `sync` 返回稳定的知识回写结果分类，至少区分 `declared_writeback`、`metadata_only` 与 `illegal_drift`。`sync` 结果 MUST 同时返回受影响对象引用与推荐动作，使调用方能区分“已写回正式知识”“只更新 runtime 状态”“必须人工处理或 rebuild”。当 `declared_writeback` 成立时，结果 MUST 同时暴露受影响 `declared_record_ids`、declared lifecycle 导致的 stale unit / projection scope，或等价稳定引用。

#### Scenario: sync 只命中 metadata-only 变更
- **WHEN** 用户编辑的内容只影响标题、排序、section hash 或其它不改变知识真相的 projection 元数据
- **THEN** `sync` MUST 将其标记为 `metadata_only`
- **THEN** 系统 MUST NOT 伪装成 declared knowledge 已被写回

#### Scenario: sync 成功写回 declared lifecycle 变更
- **WHEN** 用户通过受管编辑面更新了 declared 的 scope、状态或替代关系
- **THEN** `sync` MUST 返回 `declared_writeback`
- **THEN** 结果 MUST 同时返回受影响 declared records 与下游 stale scope

#### Scenario: sync 发现非法 drift
- **WHEN** 用户进行了无法映射为受管回写的页面编辑
- **THEN** `sync` MUST 返回 `illegal_drift`
- **THEN** 结果 MUST 同时提供稳定 `recommended_action`，例如 `rebuild` 或人工修正

### Requirement: `status` 必须同时表达 readiness 与 knowledge health
系统 MUST 让 `status` 同时表达 knowledge runtime readiness 与 knowledge health 摘要，而不是只输出单层存在性状态。调用方 MUST 能稳定区分“ready but degraded”“needs_update because derived stale”“blocker because illegal drift or unrecoverable mismatch”等情况；当 declared lifecycle 导致 superseded rule、deprecated record 或 declared/derived mismatch 时，`status` MUST 同时暴露对应 health 摘要与推荐动作。

#### Scenario: ready 但存在 declared lifecycle degradation
- **WHEN** 当前 runtime 可被 `query` 消费，但存在 deprecated declared、projection stale 或 missing provenance
- **THEN** `status` MUST 继续返回 readiness
- **THEN** `status` MUST 同时返回 health 摘要与可执行推荐动作

#### Scenario: illegal drift 进入 workflow 级诊断
- **WHEN** 最近一次 `sync` 产生了 `illegal_drift`
- **THEN** 后续 `status` MUST 能反映该知识层诊断
- **THEN** 系统 MUST 不得把该状态压平为普通 `needs_update`
