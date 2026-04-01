# knowledge-first-update Specification

## Purpose
定义 knowledge-first update 的正式 contract，确保 `update` 先定位 `AffectedKnowledgeScope`，再定向刷新 derived knowledge 与 projection。

## Requirements

### Requirement: `update` 必须先计算受影响知识范围
系统 MUST 在完成 facts/index refresh 之后、执行 research / compose 之前，把 `ChangeSet` 映射为独立的 `AffectedKnowledgeScope`。`AffectedKnowledgeScope` MUST 至少覆盖 `direct_unit_ids`、`propagated_parent_unit_ids`、`removed_unit_ids`、`affected_domain_ids`、`projection_target_page_ids` 与当前 escalation level / reason。scope planning MUST 以 `.wiki/.knowledge/derived/knowledge-units.jsonl`、`.wiki/.knowledge/derived/knowledge-tree.json` 或等价 formal identity objects 与当前 planning 结果为主输入；系统 MUST NOT 先比较 `planned_pages`、`page_digests` 或最终 Markdown，再反推 KnowledgeUnit 作用域。

#### Scenario: 局部源码变化先命中稳定 KnowledgeUnit
- **WHEN** 某个已登记源码文件发生变化，且当前规划仍能把它稳定映射到既有 `KnowledgeUnit`
- **THEN** 系统 MUST 先把该变化记录到对应的 `direct_unit_ids`
- **THEN** 系统 MUST 只在需要 parent propagation 或 projection refresh 时再扩展到祖先单元与页面目标

#### Scenario: projection 差异不得反向主导 knowledge scope
- **WHEN** 某次 update 观察到 `planned_pages`、`page_digests` 或最终 Markdown 存在差异
- **THEN** 系统 MUST NOT 仅凭这些 projection 差异反推或扩大 `AffectedKnowledgeScope`
- **THEN** 系统 MUST 回到 formal identity objects 与当前 planning 结果重新判定受影响单元

### Requirement: parent refresh 必须由 child contract changed 驱动
系统 MUST 将 parent `KnowledgeUnit` 的增量传播建立在 child contract 变化上，而不是“任一 child 源码 touched 就默认重刷祖先链”。对 parent unit 而言，child contract 至少包括 child membership、child digest 引用、parent aggregate input hash 与等价 compose 输入身份。系统 MUST 区分 `child source touched` 与 `child contract changed` 两类事件。

#### Scenario: child 源码 touched 但 child contract 未变化
- **WHEN** 某个 child unit 关联源码发生变化，但重新计算后 child digest、child membership 与 parent aggregate input hash 保持不变
- **THEN** 系统 MUST 允许该 child unit 局部 refresh
- **THEN** 系统 MUST NOT 仅因这次源码触达就把对应 parent unit 提升为受影响单元

#### Scenario: child contract 变化向 parent 传播
- **WHEN** 某个 child unit 的 digest、membership 或 parent aggregate input hash 发生变化
- **THEN** 系统 MUST 把依赖该 child contract 的 parent unit 加入 `propagated_parent_unit_ids`
- **THEN** parent refresh MUST 继续按祖先链逐层传播，直到 aggregate input 恢复稳定

### Requirement: projection refresh 必须由受影响知识范围派生
系统 MUST 让页面投影、projection anchors 与页面删除动作只由 `AffectedKnowledgeScope` 派生，而不是与其并列成为 update planner 的一级真相。`AffectedSet` MAY 保留为 projection 层结果，但 MUST 由 `AffectedKnowledgeScope` 派生生成。

#### Scenario: 局部知识范围只刷新局部投影
- **WHEN** `AffectedKnowledgeScope` 只覆盖局部 leaf unit 与有限 parent propagation
- **THEN** 系统 MUST 只刷新这些 unit 对应的 `projection_target_page_ids`
- **THEN** 系统 MUST NOT 因无关页面仍存在于同一仓库而默认重写整批页面

#### Scenario: 移除单元触发投影回收
- **WHEN** `AffectedKnowledgeScope.removed_unit_ids` 非空，且其中某些单元曾投影到正式页面
- **THEN** 系统 MUST 显式删除对应的 projection anchors 与页面投影，或把其从 surviving parent projection 中移除
- **THEN** 系统 MUST NOT 仅通过保留过期页面来伪装 update 成功

### Requirement: scope 无法局部定位时必须升级 escalation 并保留原因
系统 MUST 在 `AffectedKnowledgeScope` 中显式记录 escalation level 与 reason。escalation 至少 MUST 区分 `local_refresh`、`subtree_replan`、`repo_replan` 与 `rebuild_recommended`。整树 knowledge replan 仍属于 knowledge-first update 主线；只有 formal artifact、cache mirror、projection/runtime state 无法恢复一致性时，系统才 MAY 升级为 `rebuild_recommended`。

#### Scenario: 结构变化触发整树 knowledge replan 但不等于 rebuild
- **WHEN** workspace root、关键 config surface、入口集或 decomposition signal 变化导致 scope 无法局部限定
- **THEN** 系统 MUST 允许把 escalation 提升为 `repo_replan`
- **THEN** 系统 MUST 将其视为 knowledge-first update 的一部分，而不是直接视为 runtime rebuild

#### Scenario: 正式产物与 runtime 状态不一致时显式建议 rebuild
- **WHEN** formal knowledge artifacts、projection anchors 与 runtime/cache state 之间的一致性无法可信恢复
- **THEN** 系统 MUST 把 escalation 提升为 `rebuild_recommended`
- **THEN** 系统 MUST 同时保留导致升级的具体 reason，而不是把它伪装成普通局部刷新
