## ADDED Requirements

### Requirement: evidence layer 必须提供 key source cluster 与 section grounding 引用
系统 MUST 让 evidence layer 在正式页面、runtime cache 和报告脚本之间共享稳定的 `key_source_cluster` 与 `section_grounding_ref` 身份。每个 `section_grounding_ref` MUST 能回连到对应 section 的 evidence identity、key source cluster 和最终落页的 citation / evidence block，而不是只保留“本页有哪些关键文件”的扁平列表。

#### Scenario: section grounding ref 可回连到最终落页 evidence
- **WHEN** 某个 section 使用了明确的 `section_grounding_refs`
- **THEN** 每个 grounding ref MUST 能回连到对应的 evidence identity、key source cluster 和 section identity
- **THEN** renderer、runtime 检查和 reference 报告 MUST 能据此判断关键文件是否真正落在该 section 中

#### Scenario: key source cluster 不得退化为整页文件清单
- **WHEN** 某个页面需要围绕多个关键源码簇解释主题
- **THEN** evidence layer MUST 允许把这些来源组织成稳定的 `key_source_clusters`
- **THEN** 系统 MUST NOT 仅以“整页关键文件列表”的方式替代 section-grounded 证据表达

### Requirement: runtime 必须能诊断 planned 与 grounded key sources 的偏差
系统 MUST 在正式 runtime 中保留可诊断的 `planned_key_sources`、`grounded_key_sources` 和 `section_grounding_refs`。报告脚本 MUST 能据此区分“planner / research 选错关键文件”和“compose 没把关键文件真正落到正文 section”。

#### Scenario: runtime 能区分 planned 与 grounded 偏差
- **WHEN** 某个页面的 `planned_key_sources` 与 `grounded_key_sources` 不一致
- **THEN** runtime 和报告脚本 MUST 能指出偏差发生在 planner/research 选择阶段还是 compose 落页阶段
- **THEN** 系统 MUST NOT 只输出一个整页 coverage 分数而丢失链路诊断
