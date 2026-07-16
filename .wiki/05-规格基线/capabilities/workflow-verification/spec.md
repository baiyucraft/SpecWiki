# workflow-verification Specification

## Purpose

定义 Repo Wiki workflow 的当前验证 authority：9 个核心场景、显式 Acceptance Plan、`knowledge-quality-gates.v2`、公开 transport evidence 和可重建 runtime。

## Requirements

### Requirement: 核心场景验证必须覆盖固定 9/9 matrix

系统 MUST 以 `CS-01..CS-09` 的机器可读 matrix 作为场景身份 authority，并为每条记录声明支持等级、公开入口、正式产物、readiness、失败/降级、恢复、evidence 与延期能力。

#### Scenario: 场景汇总身份完整

- **WHEN** orchestrator 消费一次 acceptance fixture
- **THEN** summary MUST 恰好包含 `CS-01..CS-09`，不得缺失、重复或增加未知 ID
- **THEN** 每条结果 MUST 携带本次执行产生的非空 evidence refs

### Requirement: Gate v2 必须 fail closed

系统 MUST 使用 `knowledge-quality-gates.v2`。Acceptance Plan MUST 显式声明 required gates、required primary gates、required guards、primary fixtures 和 report-only；任何 required coverage 缺失 MUST 产生 `not_covered` 和 overall `incomplete`。

#### Scenario: Failure 保持 single owner

- **WHEN** 一个 assertion 失败
- **THEN** 该 failure MUST 只有一个 formal 或 primary single owner
- **THEN** baseline guard MAY 引用相同 failure，但 MUST NOT 再增加 blocker 数

#### Scenario: Diagnostic 不伪装成通过

- **WHEN** primary flow 被 diagnostic、全跳过或零样本短路
- **THEN** 未执行的 required gate MUST 保持 `not_covered`
- **THEN** overall MUST NOT 为 pass

#### Scenario: Decision 与退出码一致

- **WHEN** overall 分别为 pass、blocker、incomplete 或 diagnostic
- **THEN** 默认进程状态 MUST 分别为 0、1、2、2
- **THEN** 显式 report-only MAY 以 0 退出，但 MUST 保留原 decision

### Requirement: 三类 adapter 必须共享聚合语义

reference fidelity 只提供 primary input，project-set 与 lifecycle 只提供 baseline guard 和 formal evidence。三者 MUST 输出共享 gate/failure/diagnostic 结构，不得自行计算另一套 overall decision，也不得固定 primary fixture 组合。

#### Scenario: Lifecycle failure 不污染无关 gate

- **WHEN** 单个 restore assertion 失败
- **THEN** 只有 owning formal gate 可成为 blocker
- **THEN** 未执行的 query/status gate 保持 `not_covered`，已执行的无关 gate保持其真实 decision

### Requirement: 公开 query 验证必须只消费 canonical transport

核心场景 query 验证 MUST 只消费 `route_groups`、`answer`、readiness、query trust、recommended action 和结果内 provenance/source refs。内部丰富对象不得作为公开合同证据。

#### Scenario: Symbol、path、module 与 graph route 可追溯

- **WHEN** 动态仓库完成 init 并查询已知 term
- **THEN** canonical route results MUST 提供相应 route tag 与 source evidence
- **THEN** graph 未 ready 时 MUST 给出降级与恢复动作，不得伪造完整影响结论

### Requirement: Declared knowledge 与结构化 conflict 必须可验证

系统 MUST 覆盖 policy、convention、pitfall 的 managed edit -> sync -> update -> query；非法/重复 marker MUST 原子拒绝。结构化 parallel-active conflict MUST 提供双方 refs、去重、clear 和 `review_governance`。

#### Scenario: Conflict 创建与清理

- **WHEN** 同 kind + canonical scope 存在多条 active declared records
- **THEN** formal artifacts MUST 只生成一条稳定 conflict record
- **THEN** query MUST 返回 constrained answer 与治理 review 动作
- **WHEN** 冲突记录被解决并再次 sync
- **THEN** conflict artifact MUST 被清理

### Requirement: 正式产物必须支持真实 A/B level1 restore

系统 MUST 在两个独立工作副本中验证恢复。B 只复制相同源码、`.wiki/.knowledge/**`、`wiki.metadata.json` 和 metadata 声明页；不得复制 `.wiki/.cache/**`、SQLite、trace 或临时报告。

#### Scenario: 相同源码恢复 level1

- **WHEN** B 缓存为空且正式产物与源码一致
- **THEN** knowledge/projection MUST ready，index MUST missing，fusion MUST degraded，restored level MUST 为 level1
- **THEN** recommended action MUST 为 rebuild，query MUST NOT 产生 index route

#### Scenario: 页面和源码漂移分类

- **WHEN** B 的正式页面 hash 漂移
- **THEN** restore MUST fail closed，并给出 rebuild
- **WHEN** B 的源码相对正式 fingerprint 漂移
- **THEN** status MUST 为 needs_update，recommended action MUST 为 update

### Requirement: Wiki authority 必须与源码合同一致

核心场景、测试指南、脚本指南与本 capability MUST 投影同一 matrix/gate identity。当前文档不得重新引入旧 query 字段、固定 primary fixture 或与 required coverage 冲突的门禁要求；历史 archive 保持只读。
