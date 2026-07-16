# 核心场景验收与质量门禁调研

## 调研目的

- 阶段：propose
- 关联 change：close-specwiki-3-0-design-baseline-core-scenario-acceptance
- 服务边界：proposal
- 要回答的问题：9 个核心场景当前缺少哪些可验收合同，质量门禁冲突应如何纳入本 change 的边界。
- 停止条件：能够支撑 proposal 的 problem、goals、non-goals、success criteria、impact scope、risks 和 deferred-to-design 判断。

## 结论摘要

- 9 个核心场景仍是自然语言草案，尚未统一绑定 actor、trigger、公开入口、formal artifacts、runtime state/readiness、失败或降级、恢复和 verification fixture。
- 场景 3、4 必须受已归档 Runtime Query 合同约束；entrypoint、callers/callees/impact 等 richer query 能力不能被本 change 重新承诺。
- `workflow-verification`、reference report、lifecycle 和项目集脚本对 formal gate、primary gate、baseline guard、diagnostic 的职责和退出语义存在历史冲突，当前没有唯一的最终聚合器。
- 场景 6、7、8 的结构化 declared knowledge / governance 能力有部分实现证据，但自然语言语义冲突检测、专用 authoring API 和完整生命周期仍不是本 change 的已知完成能力。
- 场景 9 已覆盖本地 cache 删除后的 restore，但缺少 A 用户提交正式产物、B 用户只复制正式产物后恢复本地 runtime 的双工作副本 fixture。

## 已读取资料

| 路径 / 来源 | 目的 | 关键发现 |
| --- | --- | --- |
| `.wiki/06-设计文档/03-核心场景.md` | 核对 9 个故事和现有成功标准 | 每个场景有用户故事和自然语言成功标准，但没有统一的可执行验收映射。 |
| `.wiki/06-设计文档/05-产品基线与设计治理.md` | 确认单域完成和 evidence 规则 | 单域完成要求可验证合同及 full review/verification；实现、验证和发布证据正交。 |
| `.wiki/06-设计文档/06-Runtime查询合同.md` | 限定 query 公开边界 | 当前公开合同是 term-only、canonical `route_groups`/`answer` 和 typed errors；entrypoint、impact 等 richer query 延期。 |
| `.wiki/02-开发指南/01-测试与验收.md` | 核对测试层级和门禁顺序 | Rust、Agent、workspace、脚本四层测试均应按行为变化补证据。 |
| `.wiki/02-开发指南/02-脚本与工作流.md` | 核对脚本 gate 角色 | reference fidelity 是 primary 输入，项目集和 lifecycle 是 baseline guard，但最终组合规则未统一。 |
| `.wiki/05-规格基线/capabilities/workflow-verification/spec.md` | 核对历史正式要求 | 多轮 requirements 同时要求全项目、storybook/dagger、chi/zustand 等互相冲突的 gate 口径，并仍引用旧 query 字段。 |
| `.spec/changes/close-specwiki-3-0-design-baseline/research/design-debt-and-delivery-boundary.md` | 复用 parent exploration 证据 | 核心场景和 quality gate 冲突已被识别为独立 child 边界。 |
| `.spec/archive/2026-07-15-close-specwiki-3-0-design-baseline-product-contract/design.md` | 确认上游产品基线 | 核心场景 child 负责单域验收，不承担全库文档迁移或其他 child 的合同。 |
| `.spec/archive/2026-07-15-close-specwiki-3-0-design-baseline-runtime-query-contract/design.md` | 确认 query 依赖已稳定 | 旧 `provenance_summary` 等公开字段已被移除，场景验收只能消费 canonical DTO。 |

## 子代理调研

| 子代理 / 角色 | 调研主题 | 结论 | 主会话复核 |
| --- | --- | --- | --- |
| `core_scenario_scope` | 9 个场景缺口、边界和 fixture | 适合作为单一 acceptance-contract child；场景 3/4 限定现有 query；场景 9 需要 A/B restore fixture。 | accepted；未发现需要重新拆分的证据。 |
| `quality_gate_conflicts` | formal/primary/baseline/diagnostic 冲突 | 需要唯一 gate ownership、最终聚合、`not_covered`/`incomplete` 语义和退出码测试；旧 query 字段必须以 canonical 合同替换。 | accepted；实现细节 deferred-to-design/plan。 |

## 关键发现

### 场景叙述无法证明设计完成

- 证据：`.wiki/06-设计文档/03-核心场景.md`。
- 说明：9 个场景有触发条件、输入、输出和自然语言成功标准，但没有固定的 actor、command/API、正式产物、状态、失败恢复和 fixture 证据。
- 影响：proposal 必须要求统一 acceptance matrix，design 再确定矩阵字段和 fixture 结构。

### richer query 是明确边界

- 证据：`.wiki/06-设计文档/06-Runtime查询合同.md`。
- 说明：公开 query 仍是 term-only；entrypoint、owner、callers/callees/impact、process/community 等能力延期。
- 影响：场景 3 只能验收 symbol/path/module/source refs 候选，场景 4 只能验收现有 graph route 的局部上下文和显式限制。

### 门禁存在重复归因

- 证据：`workflow-verification/spec.md`、`scripts/testing/quality-gates.mjs`、reference/lifecycle/project-set 脚本。
- 说明：不同脚本各自产出 summary；单个失败可能被批量映射为多个 formal blocker，diagnostic 也可能被当作 pass 或隐藏后续断言。
- 影响：本 change 的成功标准必须要求单失败单 owning gate、`not_covered`/`incomplete` 传播和退出码一致，但不在 proposal 阶段确定实现架构。

### 声明型知识与冲突语义必须保守

- 证据：现有 declared marker、governance conflict 和相关 runtime 测试。
- 说明：pitfall、policy、convention 等结构化记录有部分落点；任意自然语言“规范与代码”语义冲突检测以及专用 authoring API 尚无确定证据。
- 影响：proposal 只承诺结构化 declared/governance 边界，未证明的能力列为 deferred 或 unknown。

### 协作恢复需要双工作副本证据

- 证据：现有 status/update/restore 测试和场景 9 描述。
- 说明：现有测试主要删除同一仓库的 cache 后恢复；没有证明 A 提交正式 `.wiki` 产物后，B 只复制正式产物即可恢复本地派生层。
- 影响：A/B fixture 作为成功标准，最小正式产物集合和漂移拒绝规则 deferred-to-design。

## 方案与取舍

| 选项 | 优点 | 缺点 | 结论 |
| --- | --- | --- | --- |
| 只把 9 个场景改成文档矩阵 | 改动小 | 仍无法证明 gate、恢复和失败语义可执行 | 不采用 |
| 让本 change 补齐所有场景产品能力 | 表面覆盖完整 | 会吞并 richer query、reliability、host 和 authoring 范围 | 不采用 |
| 以 acceptance matrix + gate contract + 自动化 fixture 为单一 child | 可独立验收，能约束后续实现而不伪造能力 | 需要处理历史 gate 迁移和多个测试层级 | 采用 |

## 风险与未知项

| 项目 | 影响 | 处理方式 |
| --- | --- | --- |
| 场景 3/4 被误写成 richer query 已交付 | 重新扩大已冻结 Runtime 合同 | 写入 non-goal，design 只定义现有 route 的验收边界。 |
| 场景 7 的自然语言语义冲突没有确定性实现 | 验收可能伪造能力 | 仅承诺结构化冲突 evidence；其余列为 deferred/unknown。 |
| 场景 9 与 reliability child 有边界重叠 | 两个 child 重复定义 restore lifecycle | 本 child 只定义 acceptance mapping 和 fixture 证据，生命周期策略留给 reliability child。 |
| 历史 gate requirement 分散且互相冲突 | 仅修改最后一段无法消除旧 MUST | design/plan 阶段建立冲突清单和 supersede 规则，保留历史 archive 不改写。 |
| 新增专用 declared authoring command 的范围不明 | 扩大 CLI 合同 | proposal 不承诺新 command，留作 design unknown。 |

## 对当前 artifact 的影响

- 应写入：`proposal.md`。
- 影响内容：问题、9/9 acceptance matrix 目标、gate ownership 目标、canonical query 边界、A/B restore 证据、non-goals 和 risks/unknowns。
- 后续阶段处理：矩阵字段、gate result schema、聚合流程、fixture 布局和具体代码迁移 deferred-to-design/plan。

## 未采纳内容

- 不把 reference fidelity 百分比直接当作 9 个核心场景的唯一 primary gate。
- 不把全项目 baseline guard 自动提升为当前 child 的 blocker。
- 不新增 intent-aware query、独立 entrypoint/impact API、自然语言冲突检测或宿主 trigger parity。
- 不修改历史 archive，不采用 upstream 实现。
