# close-specwiki-3-0-design-baseline-core-scenario-acceptance 系统测试用例

## 用例总览

本文件以 15 条系统用例覆盖 proposal 的 10 条成功标准和 design 的 6 个能力块：9 场景 authority、Acceptance Plan、Gate Kernel v2、三类脚本迁移、Runtime 场景行为、A/B restore 以及长期 Wiki/capability authority。验证以 Rust integration、workspace Vitest、离线子进程和固定路径合同为主，不涉及 UI 或浏览器交互。

## 系统测试用例

### ST-001 9/9 canonical matrix 合同闭集

- 关联成功标准: 9/9 场景都有 canonical matrix 记录并具备完整字段。
- 覆盖设计点: `CoreScenarioRecord`、稳定场景 ID、matrix validator。
- 前置条件: 导入版本化 matrix module。
- 操作 / 触发: 校验 `CS-01` 至 `CS-09`、闭集、必填字段、唯一性和 evidence paths；注入重复/未知 ID、未知 enum 和缺失 evidence。
- 期望结果: 正常矩阵 9/9 通过并稳定排序；非法输入 fail closed。
- 验证方式: workspace Vitest contract test。

### ST-002 支持等级与 deferred/query 边界

- 关联成功标准: 每场景明确 supported/degraded/deferred，场景 3/4 不扩大 richer query。
- 覆盖设计点: 9 场景支持矩阵、Runtime Query 非目标。
- 前置条件: canonical matrix 可读取。
- 操作 / 触发: 检查 CS-01/05/09 为 supported，其余为 degraded；检查 deferred capabilities 和 public entrypoints。
- 期望结果: intent/owner/entrypoint/callers/callees/impact 不会被写成公开支持；旧 query 字段不出现。
- 验证方式: workspace Vitest contract test。

### ST-003 Gate registry 与 Acceptance Plan 显式合同

- 关联成功标准: gate contract 字段完整；primary fixtures/guards/companions 来自显式 plan。
- 覆盖设计点: `GateDefinition`、`createAcceptancePlan`、闭集 validator。
- 前置条件: Gate Kernel v2 和 matrix module 可导入。
- 操作 / 触发: 创建自定义 acceptance plan，并注入未知 gate/decision、空 primary fixture 和冲突 level。
- 期望结果: 合法 plan 原样归一化并冻结；不自动注入历史样本；非法 plan fail closed。
- 验证方式: workspace Vitest contract test。

### ST-004 单 failure 唯一 owning blocker

- 关联成功标准: 同一 failure identity 只产生一个 formal/primary blocker。
- 覆盖设计点: `FailureRecord`、唯一 owner、failure refs 去重。
- 前置条件: plan 声明 formal/primary/baseline gates。
- 操作 / 触发: 同一 failure 被 formal owner、baseline 和 diagnostic 引用；再注入无 owner、双 owner和非法 owner。
- 期望结果: blocker 只计入唯一 owner；无关 gates 不污染；非法 ownership 产生 gate-contract blocker。
- 验证方式: workspace Vitest 纯函数负例。

### ST-005 Required coverage 与 diagnostic/skip 不得伪 pass

- 关联成功标准: 未覆盖、diagnostic 或短路状态可观察且不可伪装为 pass。
- 覆盖设计点: gate-level `not_covered`、overall `incomplete/diagnostic` 优先级。
- 前置条件: acceptance plan 定义 required gates/primary。
- 操作 / 触发: required gate 缺失、primary 未执行、diagnostic 短路、全 skipped/零 assertion。
- 期望结果: required 未覆盖一律 incomplete；仅 diagnostic-only plan 才可 diagnostic；均不得 pass。
- 验证方式: workspace Vitest 表驱动聚合测试。

### ST-006 统一 decision/exit 与 report-only

- 关联成功标准: decision 与 CLI/process 退出语义一致。
- 覆盖设计点: `exitCodeForAcceptance`、默认 0/1/2、显式 report-only。
- 前置条件: 离线 acceptance fixture plan。
- 操作 / 触发: 子进程分别生成 pass、blocker、incomplete、diagnostic；重复非 pass 用 report-only。
- 期望结果: 默认 exit 为 0/1/2/2；report-only exit 0 但 JSON decision 不变。
- 验证方式: Vitest `spawnSync` 子进程测试，无网络。

### ST-007 三类脚本 adapter 使用同一聚合语义

- 关联成功标准: primary、baseline 和 companions 显式声明，三类脚本不再各自决策。
- 覆盖设计点: lifecycle/project-set/reference adapters、Gate Kernel v2。
- 前置条件: 相同 plan/evidence/failure fixture。
- 操作 / 触发: 通过三个 adapter 生成 summary；覆盖 lifecycle 单失败、diagnostic project 和 reference blocker。
- 期望结果: gate/overall/exit 一致；不批量污染 gates、不包装 diagnostic 为 pass、不硬编码 storybook+dagger。
- 验证方式: workspace Vitest adapter 和 snapshot tests。

### ST-008 CS-01/CS-05 init-status-update 闭环

- 关联成功标准: 现有公开入口具备可追溯场景验收证据。
- 覆盖设计点: supported CS-01/05、formal artifacts、scoped update。
- 前置条件: 动态多模块临时仓库。
- 操作 / 触发: init 后读取 formal knowledge/metadata/pages/status；修改单一源码并执行 status/update/status。
- 期望结果: init 产物与 readiness 可解释；变更先 needs_update，update 只刷新相关范围，最终 fresh；未影响 records/pages 稳定。
- 验证方式: Rust acceptance integration。

### ST-009 CS-02/03/04 canonical rule/query/graph

- 关联成功标准: 场景验收只消费 canonical query，degraded 边界显式。
- 覆盖设计点: declared route、symbol/path/module/graph routes、typed errors。
- 前置条件: 动态仓库已 init，包含 declared policy/convention 和 graph facts。
- 操作 / 触发: sync/update/query 多类 term；覆盖 graph ready/stale/missing、空命中和未初始化。
- 期望结果: 只断言 `route_groups/answer/readiness/query_trust/recommended_action` 和逐结果 refs；不出现旧字段或内部 rich DTO。
- 验证方式: Rust transport acceptance。

### ST-010 CS-06/CS-08 pitfall、policy、convention

- 关联成功标准: 场景 6/8 的结构化 declared evidence 可执行。
- 覆盖设计点: managed declared edit、sync/update/query、原子写回。
- 前置条件: 可编辑 declared-owned section。
- 操作 / 触发: 写入三类 marker，执行 sync/update/query；注入非法/重复 ID 和 derived-owned edit。
- 期望结果: 合法记录进入 formal artifacts 并可 query；非法路径 fail closed 且不污染旧 snapshot；authoring API 保持 deferred。
- 验证方式: Rust acceptance integration。

### ST-011 CS-07 结构化 governance conflict 生命周期

- 关联成功标准: 场景 7 只验收结构化 conflict 边界。
- 覆盖设计点: conflict create/status/query/dedup/clear、review action。
- 前置条件: 两条同 scope 的 parallel active declared records。
- 操作 / 触发: sync 产生冲突，重复 sync，查询 status/query，然后移除冲突并刷新。
- 期望结果: 双方 refs、conflict state、degraded answer/review action 可追溯；不重复；清理后消失；不承诺自然语言语义检测。
- 验证方式: Rust acceptance integration。

### ST-012 CS-09 A/B 正式产物 cold restore 成功

- 关联成功标准: B 只复制正式产物即可恢复本地 runtime。
- 覆盖设计点: 动态双目录 fixture、完整 formal set、level1 readiness。
- 前置条件: A/B 同源码；A 已 init；B 无 cache。
- 操作 / 触发: 只复制 `.wiki/.knowledge/**`、metadata 和声明页面到 B，保留 A manifest repo_root，执行 B status/query。
- 期望结果: B 重建 cache；restored level1，knowledge/projection ready、index missing、fusion degraded、action rebuild；query 不伪造 index route。
- 验证方式: Rust acceptance integration。

### ST-013 CS-09 页面与源码漂移 fail closed

- 关联成功标准: A/B restore 的漂移拒绝、状态和 action 可观察。
- 覆盖设计点: page/hash validation、source delta、恢复顺序。
- 前置条件: ST-012 的动态 fixture helper。
- 操作 / 触发: 分别修改 B 的正式页面和源码后执行 status/query。
- 期望结果: 页面漂移拒绝可信恢复并给 rebuild；源码漂移返回 needs_update/update；均不可误报 ready/pass。
- 验证方式: Rust acceptance 表驱动 integration。

### ST-014 核心场景 orchestrator 9/9 可追溯汇总

- 关联成功标准: 各阶段引用同一 matrix/gate evidence，不以文档存在替代行为。
- 覆盖设计点: orchestrator、稳定 summary、plan/failure/evidence identity。
- 前置条件: 9 条 scenario results 和 gate adapter fixture。
- 操作 / 触发: 执行 acceptance plan，重复运行同一输入；删除一个场景/evidence/required gate。
- 期望结果: summary 含 9/9 results、gates/failures/diagnostics/evidence、版本/plan/decision/exit 且稳定；缺失事实 fail closed。
- 验证方式: workspace Vitest + 离线子进程。

### ST-015 Wiki/capability authority 与跨阶段引用一致

- 关联成功标准: 长期文档和 change artifacts 使用同一场景/gate identity。
- 覆盖设计点: Wiki projection、workflow-verification v2、固定路径合同。
- 前置条件: 当前源码 matrix/gate registry 已稳定。
- 操作 / 触发: 解析核心场景、测试与脚本 Wiki、workflow capability 和 change artifacts。
- 期望结果: 9 场景等级一致；只保留 v2 gate authority；不含冲突 MUST、全局固定 primary 或旧公开 query 字段；archive 不被修改。
- 验证方式: workspace fixed-path contract test。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| 9/9 canonical matrix 完整 | ST-001、ST-014 | Matrix validator、orchestrator |
| supported/degraded/deferred 边界 | ST-002、ST-009、ST-010、ST-011 | Contract + Rust acceptance |
| gate 合同字段与四类职责清楚 | ST-003、ST-007 | Plan/gate contract tests |
| 单 failure 唯一 owner | ST-004 | 聚合负例 |
| diagnostic/skip/exit 不伪 pass | ST-005、ST-006、ST-007 | 聚合 + subprocess |
| primary/guard/companions 显式声明 | ST-003、ST-007、ST-014 | Plan + adapters + orchestrator |
| canonical query only | ST-009 | Rust transport acceptance |
| 场景 6/7/8 结构化证据 | ST-010、ST-011 | Rust acceptance |
| A/B restore 与漂移 | ST-012、ST-013 | Rust dual-worktree fixture |
| 全阶段同一 matrix/evidence | ST-014、ST-015 | Orchestrator + Wiki contract |

## 边界与异常

- CS-01 的“用户理解”是主观价值，本 change 以概览正式产物、模块/入口来源和 readiness 作为自动化替代证据，不声明用户研究结论。
- CS-02 只证明规则可被公开 query 命中，不覆盖宿主自动触发。
- CS-03/04 必须走公开 transport，不允许内部 `matched_*` 作为通过证据。
- 仅 diagnostic-only plan 可输出 diagnostic；任何 required gate 未覆盖时优先为 incomplete。
- evidence path 存在性不是行为通过证据；orchestrator 还必须绑定本次执行产生的 scenario result identity。
- CS-09 若现有源码漂移状态不符合 design，按 TDD 修主链，不降低断言。

## 验证数据与环境

- Node/pnpm/Vitest workspace，无网络。
- Rust stable、临时目录、真实 SQLite 和动态小仓库 fixture。
- A/B restore 使用两个独立 tempdir，测试结束清理，不提交 cache/SQLite/trace。
- 脚本 subprocess 使用离线 fixture plan，不运行真实 reference 项目。

## 未覆盖项

- 不覆盖 UI/browser；本 change 无浏览器交互，Playwright 不适用。
- 不覆盖宿主 trigger parity、专用 authoring UX、自然语言规则-vs-code 语义检测和 richer query；均为 proposal 明确非目标。
- 不把 reference fidelity 的真实外部样本运行作为单元门禁；最终 full verification 仍执行项目现有全量自动化。

## 参考资料

- [proposal](./proposal.md)
- [design](./design.md)
- [核心场景验收与质量门禁调研](./research/core-scenario-acceptance-audit.md)
- [核心场景验收技术设计调研](./research/core-scenario-acceptance-design.md)
