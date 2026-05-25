## ADDED Requirements

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
