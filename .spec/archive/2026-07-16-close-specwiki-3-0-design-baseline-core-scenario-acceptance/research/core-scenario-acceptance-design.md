# 核心场景验收技术设计调研

## 调研目的

- 阶段：design
- 关联 change：close-specwiki-3-0-design-baseline-core-scenario-acceptance
- 服务边界：design
- 要回答的问题：9 场景矩阵、质量 gate 聚合、退出码和 A/B restore fixture 应落在哪些现有模块，如何避免扩大 Runtime 与其他 child 边界。
- 停止条件：能够确定模块职责、数据合同、调用流程、异常语义、验证方向和主要风险对策。

## 结论摘要

- `scripts/testing/quality-gates.mjs` 已是 lifecycle、项目集和 reference reporting 的唯一共享 gate 层，应直接升级而不是新建第二套聚合内核。
- 9 场景 acceptance matrix 应作为版本化、机器可读的独立代码资产；Wiki 只投影稳定结论，Rust/TypeScript 测试提供行为证据。
- gate-level decision 与 overall decision 需要分层；一个 `failure_id` 只能有一个 formal/primary owner，baseline 和 diagnostic 只引用该 failure。
- lifecycle 当前用总失败数同时污染四个 formal gates，reference blocker 不改变退出码，diagnostic/skip 可能被报告为 pass；三类脚本都必须改为消费共享聚合结果和退出策略。
- A/B restore 必须使用两个动态临时目录，只复制完整正式 `.wiki/.knowledge/**`、metadata 声明的页面和相同源码，不复制 cache；恢复只能声明实际 restored level 和 recommended action。

## 已读取资料

| 路径 / 来源 | 目的 | 关键发现 |
| --- | --- | --- |
| `scripts/testing/quality-gates.mjs` | 核对共享 gate 内核 | 已定义四个 formal gates、summary 和 capability matrix，但没有 failure owner、required coverage 聚合和统一退出码。 |
| `scripts/run-test-projects.mjs` | 核对 baseline guard | diagnostic run 会包装为 `ok: true`，CLI 只在 `ok: false` 时非零退出。 |
| `scripts/test-wiki-lifecycle.mjs` | 核对 lifecycle gate | `failedAssertions > 0` 会让四个 formal gates同时 blocker；全 skipped/diagnostic 仍可能 pass。 |
| `scripts/collect-reference-project-reports.mjs` | 核对 primary gate | primary 条件硬编码样本与阈值，blocker summary 不保证非零退出，formal companion coverage 不完整。 |
| `scripts/testing/reference-fidelity.mjs` | 确认 fidelity 职责 | 只负责页面匹配与质量指标，可继续作为 primary input，不应拥有 formal gate authority。 |
| `scripts/tests/quality-gates-contract.test.ts` | 核对合同测试 | 保护当前 summary 形状，但缺 failure ownership、not-covered/incomplete、退出码和最终聚合负例。 |
| `crates/wiki-runtime/tests/acceptance/**` | 核对系统级 fixture | 已有 init、canonical transport 和 baseline fixture，可扩展独立 core scenarios 模块。 |
| `crates/wiki-runtime/tests/runtime/**` | 核对行为证据 | 已覆盖 declared writeback、governance conflict、query routes、update scope 和同目录 cache restore。 |
| `packages/spec-wiki/src/runtime/exitPolicy.ts` | 核对 TS 退出策略 | 已使用 0/1/2 表达成功、失败和 partial，可与脚本 gate 退出策略保持语义一致。 |

## 子代理调研

| 子代理 / 角色 | 调研主题 | 结论 | 主会话复核 |
| --- | --- | --- | --- |
| `gate_callgraph_design` | gate/report 调用链、共享接口和退出码 | 扩展 `quality-gates.mjs`；新增 acceptance plan/matrix；三个脚本统一聚合；修复 lifecycle 重复归因和 reference exit。 | accepted；gate 内核与场景矩阵保持解耦。 |
| `scenario_fixture_design` | 9 场景 fixture、Rust 证据和 A/B restore | 1/5/9 supported，其余 degraded；新增 Rust core scenarios 模块；A/B 只复制正式产物并断言 level1/rebuild。 | accepted；不新增 authoring/query 产品接口。 |

## 关键发现

### gate-level 与 overall decision 必须分离

- 证据：`buildAcceptanceHarnessSummary` 当前用单一 `decision` 同时表达 gate 和整体结果。
- 说明：单 gate 的 `not_covered` 与整体的 `incomplete` 不是同一层状态；diagnostic 也不是 formal decision。
- 影响：design 定义 `GateDecision`、`AcceptanceDecision` 和独立 `DiagnosticObservation`。

### failure identity 是唯一归因的必要条件

- 证据：lifecycle 只记录总失败数，并把所有 formal gates 同时设为 blocker。
- 说明：没有稳定 failure identity 和 owner gate，就无法 dedupe 或证明无关 gate 未被污染。
- 影响：每个 assertion failure 先登记 `failure_id`、`owner_gate_id` 和 source/evidence，再聚合 gate。

### acceptance matrix 不应进入 Runtime DTO

- 证据：Runtime Query 合同已收口，场景矩阵属于验证与长期设计 authority。
- 说明：把矩阵塞进 query/status transport 会扩大产品公开接口并耦合验证脚本。
- 影响：矩阵位于 `scripts/testing`，由 contract test、orchestrator 和 Wiki 投影消费。

### A/B restore 的 trust anchor 是正式 artifact identity

- 证据：restore helper 需要完整 knowledge artifact 集合和 metadata snapshot pointer；manifest 还包含 A 的绝对 `repo_root`。
- 说明：绝对路径不应成为跨工作副本恢复的身份或信任锚点，fixture 也不能修改 manifest 伪造 B 路径。
- 影响：测试原样复制 A 的正式产物，验证 B 能恢复；hash/snapshot/page drift 仍按现有校验拒绝。

## 方案与取舍

| 选项 | 优点 | 缺点 | 结论 |
| --- | --- | --- | --- |
| 场景矩阵和 gate 聚合放在一个大模块 | 入口集中 | 产品场景、脚本流程和通用 gate 强耦合 | 不采用 |
| 在每个脚本内分别修复 decision | 局部改动小 | 冲突会再次出现，无法做单 failure owner | 不采用 |
| 独立场景矩阵 + 共享 gate 内核 + 薄脚本 adapter | authority 清楚，可单测，可逐步接入 | 需要迁移三个脚本的 summary/exit | 采用 |
| 提交静态 A/B `.wiki` fixture | 直观看到产物 | 体积大、易漂移、可能提交 cache | 不采用；运行时动态生成。 |

## 风险与未知项

| 项目 | 影响 | 处理方式 |
| --- | --- | --- |
| 旧 summary 消费者依赖 v1 字段 | 直接替换可能暴露测试失败 | 当前测试阶段允许破坏性替换；统一更新当前源码和测试，不保留 fallback。 |
| diagnostic 是否总应非零退出 | baseline 观察脚本可能需要报告后继续 | 默认执行返回 2；只有显式 `report-only` 可返回 0，summary 仍保留 diagnostic。 |
| 场景 9 的 B 源码漂移结果 | 可能因 restore 顺序得到 needs_update 或 needs_rebuild | system tests 先锁定现有 authority；若与 proposal 冲突，先修实现，不降低断言。 |
| workflow-verification 旧 MUST 数量多 | 只追加新 requirement 不会消除冲突 | 当前 authority 直接重写为统一 gate 规范；历史证据只留 archive。 |

## 对当前 artifact 的影响

- 应写入：`design.md`。
- 影响内容：模块架构、场景矩阵数据结构、gate/failure/summary 合同、聚合流程、退出策略、A/B restore fixture、文档迁移和验证方向。
- 后续阶段处理：具体系统测试编号、TDD 单元测试和实现任务 deferred-to-plan。

## 未采纳内容

- 不新增产品 CLI、HTTP API、数据库表或 Runtime transport 字段。
- 不保留旧 quality gate v1 fallback，不修改历史 archive。
- 不使用 upstream；本设计由仓库现有模块和合同推导。
