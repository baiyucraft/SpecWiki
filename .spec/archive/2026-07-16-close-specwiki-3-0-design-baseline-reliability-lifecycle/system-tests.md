# close-specwiki-3-0-design-baseline-reliability-lifecycle 系统测试用例

## 用例总览

本文件以 A1/A6/A8/A9、KnowledgeUnit resume 和扩展场景分类为验收主线，覆盖 proposal 的全部成功标准。验证以离线 Rust integration/acceptance tests 和 workspace contract tests 为主，不依赖真实网络 provider、外部大仓或 wall-clock SLA。

## 系统测试用例

### ST-001 A1 freshness 生命周期跨 workflow 一致

- 关联成功标准: 唯一可靠性 authority；fresh -> stale/needs_update -> update/rebuild -> fresh；旧 route 不得 direct-trust。
- 覆盖设计点: `ReliabilityAssessment`、freshness/consumability 双轴、action 前置顺序、route-local trust。
- 前置条件: 动态临时仓库已完成 init，formal artifacts、index、projection 与 metadata 一致。
- 操作 / 触发: 分别切换 branch/head、修改 source、触发 legal declared drift、执行 status/query/update/rebuild。
- 期望结果: source drift 只把 index/derived/projection 标为 stale，declared 保持 current；sync 优先于 update；恢复后所有层重新 current，旧 cache/checkpoint 不提升 trust。
- 验证方式: `cargo test -p wiki-runtime --test runtime reliability_lifecycle` 的串联 fixture。

### ST-002 formal artifact 和 page binding drift fail closed

- 关联成功标准: branch/source/formal drift 自动化覆盖；旧 snapshot、stale route、非法 page drift 不得 trusted。
- 覆盖设计点: formal integrity priority、metadata/mirror authority、cache 非 authority。
- 前置条件: 可 Level 1 restore 的正式 snapshot。
- 操作 / 触发: 分别篡改 manifest、declared snapshot、projection digest、managed page hash、metadata binding，并删除 local cache。
- 期望结果: restore/status/query 对受影响 layer blocked，推荐 rebuild/review；不会从 page 或 cache 反推 formal truth，也不生成伪 index route。
- 验证方式: Rust restore/integrity integration tests。

### ST-003 declared replacement graph 与 authority 状态闭合

- 关联成功标准: active、deprecated、superseded、replaced、零 head、并行 head、关系错误和环具有唯一行为。
- 覆盖设计点: canonical `old -> new` edge、semantic/authoring/authority 三轴、replaced head 可 query。
- 前置条件: 含 repo/domain scope declared blocks 的临时仓库。
- 操作 / 触发: 创建 single active、A->B->C、all deprecated、parallel head、missing/cross-scope/self/cycle candidate 并 sync/query/status/restore。
- 期望结果: single/replacement chain 产生唯一 head；replaced head 可 query；all deprecated 为合法 none；非法 graph 原子拒绝；并行/无合法 head 为 `review_governance`。
- 验证方式: wiki-knowledge graph tests + wiki-runtime declared integration tests。

### ST-004 declared block 删除保留 truth 和治理历史

- 关联成功标准: 删除/失联语义唯一；非法 drift 不覆盖 formal truth；冲突与解决证据可追溯。
- 覆盖设计点: `bound/missing/detached`、open conflict view、append-only governance event。
- 前置条件: active/replaced 与 deprecated/superseded records 已成功提交。
- 操作 / 触发: 删除 authority block、恢复同 authoring_id、先 deprecated 再删除、解决并再次制造同 scope conflict。
- 期望结果: authority block 删除保留 record并产生 missing/conflict；恢复后 current conflict 关闭但历史保留；非 authority block 可 detach；no-op sync 不重复事件；不提供隐式 hard purge。
- 验证方式: Rust sync/update/query/restore integration tests。

### ST-005 same-facts declared 变化确定性传播

- 关联成功标准: declared 变化在无 source dirty set 时传播到 derived/projection/health；restore 使用同一 lifecycle identity。
- 覆盖设计点: committed snapshot identity 包含 declared/authority/projection/page hashes。
- 前置条件: source/facts 不变且已有 committed runtime。
- 操作 / 触发: 合法修改 declared status/relation并 sync/update，再在无 cache 环境 restore。
- 期望结果: snapshot id 改变，受影响 unit/projection/health 刷新；未受影响 source/index identity 保持；Level 1 restore 保真 authority state。
- 验证方式: Rust knowledge artifacts roundtrip 与 status/update fixture。

### ST-006 projection policy 默认有界且可重放

- 关联成功标准: 每个正式 page 可追溯 decision；默认策略不随 KnowledgeUnit 无条件一对一增殖。
- 覆盖设计点: required/selected/knowledge_only、domain leaf budget、include/exclude/priority/hints 边界。
- 前置条件: 动态生成多个 domain、每个 domain 多于预算的 leaf units。
- 操作 / 触发: 用默认 policy 和显式 include/exclude/priority 重复规划并 init/rebuild。
- 期望结果: structural pages 必保留；leaf 数满足预算加显式 include 上界；排序稳定；hints 不改变 eligibility；每 unit 都有 decision但只有 eligible unit 有 page。
- 验证方式: wiki-knowledge policy tests + runtime init/rebuild integration tests。

### ST-007 projection demotion 安全回收与保护

- 关联成功标准: promotion/retain/refresh/demotion/removal 与 readiness 正交；安全回收 page/binding/digest/metadata/cache，不丢 protected content。
- 覆盖设计点: retiring preflight、manual/declared/manual-link protection、managed link rewrite、commit plan。
- 前置条件: projected leaf pages 含 clean、manual section、authority block、manual inbound link 四种页面。
- 操作 / 触发: policy 使页面不再 eligible，分别执行 update/rebuild，并在 commit 各阶段注入中断。
- 期望结果: clean page 同 commit 完整回收并保留 retired decision；protected page 停在 retiring且文件不删；managed link 无悬空；中断可 rollback/roll-forward，status 不 direct-trust incomplete commit。
- 验证方式: Rust projection governance/commit recovery integration tests。

### ST-008 production provider failure matrix 单值一致

- 关联成功标准: A9 覆盖 provider unavailable/transport/timeout/turn/call budget/invalid/no delta；无有效 output 不得正式成功。
- 覆盖设计点: `ResearchOutcomeDecision`、`ProviderFailureKind`、production/development policy。
- 前置条件: 可注入 stop reason/failure 的离线 fake provider。
- 操作 / 触发: 枚举 completed/no-tools valid output 与所有无 output/failure 条件，分别运行 production 和 development fixture。
- 期望结果: production 仅 valid output accepted；其它均 blocked、写 gate/checkpoint/summary、返回 failure/nonzero；development structural fallback 仅 diagnostic且不写 production-ready artifact。
- 验证方式: reducer table unit tests + init/update/rebuild provider integration tests + transport tests。

### ST-009 parser/index/compose/assemble failure remaining capability 一致

- 关联成功标准: failure matrix 覆盖 index unavailable、runtime incomplete、compose/assemble interruption、illegal page drift；status/query/process 一致。
- 覆盖设计点: layer assessment、原 workflow action retry、Level 1 degraded route。
- 前置条件: 有 committed snapshot 和可注入 pipeline failure 的临时仓库。
- 操作 / 触发: 在 parser/index/graph/research/compose/assemble 各阶段失败。
- 期望结果: 对应 layer blocked/degraded；旧 formal snapshot仅按 evidence受限查询；无伪 route；workflow terminal、exit、status state/action和 checkpoint一致。
- 验证方式: Rust workflow integration + NDJSON/CLI workspace contract tests。

### ST-010 KnowledgeUnit resume 复用完整提交点并拒绝变更 identity

- 关联成功标准: 相同 action/facts复用完成 unit，计数不重复；facts/action变化拒绝旧 checkpoint/cache；完成后无伪 interrupted。
- 覆盖设计点: `PipelineResumeIdentity`、research commit、draft+digest pair、checkpoint cleanup。
- 前置条件: 动态 32-64 unit workspace和计数 fake provider。
- 操作 / 触发: 第 K unit 中断后以相同 identity恢复；再分别改变 action、facts、contract version；构造单边/错配 compose pair。
- 期望结果: 已完成 unit不再调用provider/compose；当前unit重跑；变化identity全部拒绝；半写pair整unit重算；公开workflow完成后checkpoint清空且summary/gates completed。
- 验证方式: `cargo test -p wiki-runtime --test runtime compose_resume_large_fixture`。

### ST-011 provider session 保持 request-local

- 关联成功标准: session 不被描述或实现为跨 workflow durable state。
- 覆盖设计点: `session=None` 重启当前 unit、session字段不落盘。
- 前置条件: 记录 provider request 和 storage writes 的 fake runtime。
- 操作 / 触发: 同一 page 连续调用、当前 unit 中断恢复、已完成 unit resume。
- 期望结果: 新 provider 调用首个 request 均无 previous session；已完成 unit不再调用；checkpoint/cache/formal artifacts不含 session summary/turn/tool refs。
- 验证方式: Rust LLM/provider tests + storage contract scan。

### ST-012 A1-A10 与术语 authority 一致

- 关联成功标准: A1-A10 唯一分类；A4引用 query authority；session/decomposition/大仓声明与事实一致。
- 覆盖设计点: scenario classification、machine-verifiable boundary record、直接相关 Wiki/capability 更新。
- 前置条件: 当前 change 已实现并准备 verification。
- 操作 / 触发: workspace contract test读取扩展场景、Runtime、capability 与实现中的闭集 identity。
- 期望结果: A1/A4/A6/A8/A9 baseline，A2/A3/A5/A10 next，A7 non-goal；不存在 durable session、generic typed surface complete或任意大仓完成的当前承诺；未复制 query DTO。
- 验证方式: Vitest fixed-path contract test + `rg` 非历史 authority 扫描。

### ST-013 完整公开 workflow 与 committed snapshot 原子终态

- 关联成功标准: proposal/design/system tests/实现/review/verification 使用同一 identity；完整 workflow 不残留伪 checkpoint。
- 覆盖设计点: `RuntimeCommitPlan`、composite snapshot identity、init/update/rebuild/sync/restore parity。
- 前置条件: 小型离线 fixture，允许每个 commit phase 定点失败。
- 操作 / 触发: 分别完成 init、sync、update、rebuild、Level 1 restore，并重复恢复 incomplete commit。
- 期望结果: 成功终态 page/artifact/metadata/pointer/cache一致；失败终态可确定 rollback/roll-forward；同一 operation幂等；checkpoint只在完整提交后清除。
- 验证方式: Rust runtime commit/roundtrip integration tests和 UniSpec full review。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| 唯一跨层 reliability authority | ST-001、ST-002、ST-009 | Rust reducer与workflow integration |
| A1 stale/restore闭环 | ST-001、ST-002 | status/query/update/rebuild fixture |
| A6 states/head/delete/conflict | ST-003、ST-004、ST-005 | knowledge + runtime tests |
| declared无source dirty传播 | ST-005 | artifact roundtrip/update tests |
| A8每页decision与默认有界 | ST-006 | planner + init tests |
| projection安全demotion/removal | ST-007、ST-013 | protection/commit recovery tests |
| A9完整failure matrix | ST-008、ST-009 | exhaustive reducer + workflow/transport |
| production无output不得成功 | ST-008 | provider integration/exit assertions |
| KnowledgeUnit resume | ST-010 | 32-64 unit deterministic fixture |
| provider session/decomposition真实边界 | ST-011、ST-012 | Rust storage + docs contract |
| A1-A10分类与A4 authority | ST-012 | workspace contract test |
| lifecycle identity贯穿artifact/review | ST-013 | full verification + UniSpec validate |

## 边界与异常

- 真实网络/provider/外部大仓不进入 required gate；只允许显式 diagnostic evidence。
- hard purge、redirect、在线 usage ranking、turn-level checkpoint、跨机器 cache/session 不在本 change。
- current query payload不得新增 richer query 字段；governance blocker不得降低无关 core trust。
- manual/declared protection无法自动解决时允许 `retiring + review_governance`，但不允许删除或伪 ready。

## 验证数据与环境

- Rust `tempfile` 动态仓库与 workspace fixtures。
- 可计数/可注入 stop reason 的 fake provider，不访问网络。
- 32-64 unit deterministic workspace，规模按实际 planned unit 数断言。
- Node.js/pnpm、Rust stable、当前 workspace SQLite bundled 依赖。

## 未覆盖项

- 无 proposal 成功标准未覆盖。真实 Storybook/Dagger wall-clock 和网络 provider 质量只作为非 required diagnostic，不影响本 change 自动化 gate。

## 参考资料

- [proposal](./proposal.md)
- [design](./design.md)
- [可靠性与生命周期技术设计调研](./research/reliability-lifecycle-design.md)
- `.wiki/02-开发指南/01-测试与验收.md`
