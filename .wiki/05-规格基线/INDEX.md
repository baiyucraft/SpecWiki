---
title: 05-规格基线
description: spec-wiki 长期 capability 规格基线入口
updated: 2026-07-17
owner: docs
---

# 05-规格基线

本栏目保存从治理迁移沉淀来的稳定 capability 规格。它不是 active change 目录，也不是 runtime `.wiki/pages/**` 页面投影。

## 命名例外

普通 Wiki 页面默认使用 `NN-主题.md`；本栏目下的 capability 规格保留：

```text
capabilities/<capability>/spec.md
```

这是结构性例外，用于保持 capability 名称与规格正文的稳定映射。不要为了满足普通页面命名规则把这些文件改成 `NN-*.md`。

## Capability 索引

| Capability | 入口 |
| --- | --- |
| adapter-distribution | [spec](./capabilities/adapter-distribution/spec.md) |
| agent-session-bridge | [spec](./capabilities/agent-session-bridge/spec.md) |
| codebuddy-agent-integration | [spec](./capabilities/codebuddy-agent-integration/spec.md) |
| host-trigger-contract | [spec](./capabilities/host-trigger-contract/spec.md) |
| declared-knowledge-lifecycle | [spec](./capabilities/declared-knowledge-lifecycle/spec.md) |
| knowledge-first-update | [spec](./capabilities/knowledge-first-update/spec.md) |
| knowledge-runtime-artifacts | [spec](./capabilities/knowledge-runtime-artifacts/spec.md) |
| knowledge-runtime-health-signals | [spec](./capabilities/knowledge-runtime-health-signals/spec.md) |
| knowledge-unit-decomposition | [spec](./capabilities/knowledge-unit-decomposition/spec.md) |
| llm-budget-observability | [spec](./capabilities/llm-budget-observability/spec.md) |
| metadata-mapper | [spec](./capabilities/metadata-mapper/spec.md) |
| page-evidence-layer | [spec](./capabilities/page-evidence-layer/spec.md) |
| page-research-dossier | [spec](./capabilities/page-research-dossier/spec.md) |
| page-topology-stability | [spec](./capabilities/page-topology-stability/spec.md) |
| reference-fidelity-reporting | [spec](./capabilities/reference-fidelity-reporting/spec.md) |
| repo-hierarchy-model | [spec](./capabilities/repo-hierarchy-model/spec.md) |
| repo-wiki-runtime | [spec](./capabilities/repo-wiki-runtime/spec.md) |
| repo-wiki-workflow | [spec](./capabilities/repo-wiki-workflow/spec.md) |
| research-driven-page-composition | [spec](./capabilities/research-driven-page-composition/spec.md) |
| scanner-file-purpose | [spec](./capabilities/scanner-file-purpose/spec.md) |
| scanner-noise-filter | [spec](./capabilities/scanner-noise-filter/spec.md) |
| sqlite-cache-storage | [spec](./capabilities/sqlite-cache-storage/spec.md) |
| symbol-graph-analysis | [spec](./capabilities/symbol-graph-analysis/spec.md) |
| symbol-parsing | [spec](./capabilities/symbol-parsing/spec.md) |
| symbol-resolution | [spec](./capabilities/symbol-resolution/spec.md) |
| topic-page-planner | [spec](./capabilities/topic-page-planner/spec.md) |
| wiki-bm25-query | [spec](./capabilities/wiki-bm25-query/spec.md) |
| wiki-change-set-kernel | [spec](./capabilities/wiki-change-set-kernel/spec.md) |
| wiki-index-query-surface | [spec](./capabilities/wiki-index-query-surface/spec.md) |
| wiki-llm-enhancement | [spec](./capabilities/wiki-llm-enhancement/spec.md) |
| wiki-managed-section-kernel | [spec](./capabilities/wiki-managed-section-kernel/spec.md) |
| wiki-state-kernel | [spec](./capabilities/wiki-state-kernel/spec.md) |
| wiki-steering-config | [spec](./capabilities/wiki-steering-config/spec.md) |
| workflow-progress-streaming | [spec](./capabilities/workflow-progress-streaming/spec.md) |
| workflow-verification | [spec](./capabilities/workflow-verification/spec.md) |
| workspace-crate-boundaries | [spec](./capabilities/workspace-crate-boundaries/spec.md) |

## 维护要求

- 新增 capability 时，同步更新本索引。
- active change 的 delta 仍写入 `.spec/changes/**`，不要直接改基线替代 change artifact。
- 归档后若形成长期稳定结论，再同步回本栏目。
