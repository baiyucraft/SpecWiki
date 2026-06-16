---
review-result: fail
scope: full
---

# formalize-declared-authoring-contract 审查报告

## 审查结论

- review-result: fail
- scope: full
- 结论摘要：declared authoring 的主体实现已经存在，但 system-tests、wiki baseline 与 runtime relation 证据仍有阻塞缺口，不能进入 verification 或归档。

## 审查概览

| 项目 | 内容 |
| --- | --- |
| change-id | formalize-declared-authoring-contract |
| 审查类型 | full |
| 审查对象 | proposal / design / system-tests / tasks / meta、change specs、当前 HEAD 的 Rust 实现与测试、`.wiki` baseline |
| 问题总数 | 8 |

## 审查范围

- 整体 change：DeclaredRecord schema、typed scope、relations/status、sync 分类优先级、declared artifact restore/roundtrip、status/update propagation、禁止从页面正文反推 declared truth。
- 当前工作区没有未提交实现 diff；本次按当前 HEAD 与 change artifacts / wiki baseline 的一致性审查。

## 采用的审查规范

| 规范 | 选择依据 | 覆盖范围 |
| --- | --- | --- |
| `.agents/skills/unispec-review/references/review-standard.md` | always | 整体 change、artifacts、代码、测试、wiki baseline |

## 部分范围

无。

## Artifact 一致性

- proposal.md：基本符合。当前实现可见 `DeclaredKnowledgeRecord`、typed `DeclaredKnowledgeScope`、relations/status、`.wiki/.knowledge/declared/records.jsonl`、`sync/status/update` 消费链。
- design.md：部分不符合。实现大体沿设计推进，但删除语义、relations/status 的 runtime roundtrip 证据和 baseline 同步仍未闭环。
- system-tests.md：不符合。仍是治理迁移占位，只覆盖 required artifact 结构和任务可追踪性，未覆盖本 change 的核心验收。
- tasks.md：不符合。所有任务均已勾选，但 4.4 的样本验证证据无有效 pass，5.1 的 wiki baseline 同步存在冲突，relations/status runtime 路径缺少对应测试证据。

## 问题统计

| 类型 | 数量 | 说明 |
| --- | --- | --- |
| 阻塞问题 | 4 | 系统测试 artifact、wiki baseline、runtime relation test、测试执行证据 |
| 非阻塞问题 | 1 | 删除语义需要在 contract 中收敛 |
| Artifact 同步问题 | 2 | system-tests 占位；tasks 勾选与证据不匹配 |
| Wiki 同步问题 | 2 | capability baseline 存在旧 contract 与新 contract 冲突 |
| 证据缺口 | 3 | 缺 relation sync/restore/status 测试；缺有效样本验证；workspace test 失败 |

## 阻塞问题

- [B1] `system-tests.md` 仍是迁移占位，不能验证本 change 的成功标准。它只包含 ST-001/ST-002 的 artifact 结构与追踪检查，没有覆盖 declared schema、typed scope、relations/status、`illegal_drift > declared_writeback > metadata_only`、restore/roundtrip、status/update propagation、禁止从页面正文反推 declared truth。建议回到 plan 阶段补齐 change-specific system tests。
- [B2] `.wiki` capability baseline 与本 change 的 declared lifecycle contract 冲突。`knowledge-runtime-artifacts/spec.md` 同时保留旧“declared lifecycle 不进入正式 contract”和新“declared artifact 成为 snapshot 组成部分”的要求；`declared-knowledge-lifecycle/spec.md` 仍未完整表达 typed `scope` + `relations` 的正式 schema。建议回到 apply/docs 同步 baseline 后再 review。
- [B3] relations/status 的 runtime authoring path 缺少直接测试证据。模型层 validator 有 lifecycle tests，runtime parser 也支持 `deprecated / replaced_by / supersedes`，但缺少 managed declared block 写入 artifact、restore 后保持、并影响 status/query 的闭环测试。
- [B4] tasks 已勾选“执行相关层级测试”和样本验证，但本轮 `node scripts/run-test-projects.mjs` 只得到 0 passed / 0 failed / 0 total，未形成有效样本项目 pass 证据；`pnpm test` 仍有一个 workspace 单测失败。

## 非阻塞问题

- [P1] 删除语义与 design wording 需要收敛。当前实现通过合法 managed baseline + previous/current snapshot diff 将删除分类为 `declared_writeback`，随后从 `records.jsonl` 中 prune。这个行为可能可视为等价删除语义，但 contract 需要明确它是否满足 tombstone/deprecated 的审计要求。

## Artifact 同步问题

- `system-tests.md` 与 proposal/design 不同步：它仍是迁移补齐产物，不是本 change 的可验收测试设计。
- `tasks.md` 与证据不同步：4.4/5.1 已勾选，但样本验证、wiki baseline 同步和 runtime relation 测试仍有缺口。

## Wiki 同步问题

- `.wiki/05-规格基线/capabilities/knowledge-runtime-artifacts/spec.md` 同时保留旧 declared 占位 contract 与新 declared snapshot contract。
- `.wiki/05-规格基线/capabilities/declared-knowledge-lifecycle/spec.md` 尚未升级到 typed `scope` + `relations` 的正式 schema；`.wiki/06-设计文档/01-Runtime设计.md` 已表达新 contract，但 capability baseline 未完全同步。

## 证据缺口

- 缺少 runtime integration test 覆盖 managed declared block 中 `deprecated / replaced_by / supersedes` 的 parse -> artifact -> restore/status/query roundtrip。
- 缺少 cache-less restore 场景中“页面正文包含不同 declared block 但 artifact 为准”的负向证据。
- 缺少有效样本项目验证证据；当前样本验证命令没有实际覆盖项目。

## 剩余风险

- `sync` 当前是 page-level atomic：同页一处 illegal drift 会阻断该页全部 declared writeback。若这是有意取舍，需要同步到 design/spec，否则后续会被误判为局部恢复缺失。
- `status` 对非 active declared lifecycle 的 health signal 当前更偏诊断而非 blocker。如果产品期望 deprecated/replaced 也驱动 update/review，需要另行定义。

## 下一步

- 回到 plan/apply 补齐 `system-tests.md` 的核心 ST 覆盖。
- 同步 `.wiki/05-规格基线/**`，删除旧 declared 占位 contract，补 typed scope / relations / restore truth source baseline。
- 补 runtime tests：`deprecated`、`replaced_by`、`supersedes` 从受管 block 写回 artifact、restore roundtrip、status/query 可观察。
- 修复或隔离 `pnpm test` 失败用例，并补有效样本验证证据后重新进入 `unispec-review`。
