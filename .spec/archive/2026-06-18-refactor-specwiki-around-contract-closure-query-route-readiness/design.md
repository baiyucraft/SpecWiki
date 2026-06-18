# refactor-specwiki-around-contract-closure-query-route-readiness 设计方案

## 方案概述

本 change 负责把 SpecWiki 的 query 公共合同收口成稳定、可验证、可被人解释的统一查询协议。它不再把 query 结果当作一个粗粒度字符串摘要，而是把来源层、可信度、推荐动作、引用对象和降级原因都显式化。

设计方向是：`wiki-runtime` 继续负责 query 装配与路由，`wiki-model` 承担稳定公开 DTO，`packages/spec-wiki` 只做结构校验与薄解析，`wiki-index` 只输出 facts-owned 查询结果，不知道公开 route tag。

### 需求背景引用

- 业务现状：query 侧已有 `query_mode`、`query_trust`、`readiness` 和 `provenance_summary`，但公开语义仍停留在 `index_hit / knowledge_hit / page_fallback`。
- 驱动因素：后续 `code-graph-index`、`cli-product-surface`、`governance-isolation` 都会消费 query 输出，必须先钉死主合同。
- 痛点 / 机会：如果不先统一 route tag、DTO、readiness 与 trust，后续 child 会各自发明私货字段，导致协议漂移。

### 方案目标

- 统一 query route tag、result DTO、readiness、trust、recommended action 和解释性输出。
- 让 query response 能区分 index、knowledge、projection、governance 和 fallback 的来源。
- 让 TS/Agent/Skill 只消费稳定合同，不参与业务推理。
- 让后续 `code-graph-index`、`governance-isolation`、`cli-product-surface` 只接边界，不重写合同。

### 方案范围

- 覆盖 query 主合同、DTO 所有权、route/ready/fallback 行为矩阵、transport 边界和测试方向。
- 边界说明：不做 graph schema、不做 governance scanner、不改 CLI 产品面、不回头改 projection writeback。
- 设计边界：本 change 只定义 query 消费协议，不进入 code graph substrate 或治理状态机。

### 核心设计思路

把 query 拆成三层：

1. `wiki-model` 提供公开对象语言。
2. `wiki-runtime` 负责把 index / knowledge / page fallback / governance placeholder 装配成最终响应。
3. `packages/spec-wiki` 只做协议校验和结果展示，不推导业务语义。

## 架构分析

### 现有架构概述

| 架构层级 | 组件 / 模块 | 说明 |
| --- | --- | --- |
| 模型层 | `wiki-model` | 当前承载稳定状态 DTO，但尚无 query 专属公开合同 |
| 索引层 | `wiki-index` | 只输出 facts-owned 查询结果和 MatchBasis |
| 运行时层 | `wiki-runtime` | 已有 query workflow、readiness、rich report 和 slim transport |
| 接入层 | `packages/spec-wiki` | 只做薄解析、转发和 Agent 文案消费 |

### 方案与现有架构的关系

| 维度 | 说明 |
| --- | --- |
| 复用模块 | `RuntimeReadiness`、`QueryTrust`、`QueryMode`、`AnswerEnvelope`、现有 query workflow、现有 slim transport |
| 新增组件 | `wiki-model::domain::query` 公开 query DTO 与 route tag |
| 改造模块 | `wiki-runtime` query 装配、transport 映射、TS parser 的结构校验 |
| 技术栈 | 沿用现有 Rust + TypeScript + CLI forwarding 方案 |

### 依赖关系

| 依赖项 | 类型 | 用途 | 来源 / 文档 | 备注 |
| --- | --- | --- | --- | --- |
| `wiki-model::domain::query` | 模块 | 稳定 query DTO 与枚举归属 | 本 change 设计 | 新增 |
| `wiki-runtime::workflows::query` | workflow | rich query assembly | 现有代码 | 改造 |
| `wiki-runtime::transport::query_payload` | transport | slim payload 映射 | 现有代码 | 改造 |
| `packages/spec-wiki/src/runtime/parseResult.ts` | parser | TS 结构校验 | 现有代码 | 改造 |
| `.docs/design/specwiki-contract-closure.md` | 设计草案 | query route/readiness 上位约束 | 设计文档 | 直接依据 |

## 功能设计

### 功能模块划分

| 模块名称 | 功能描述 | 优先级 | 依赖模块 | 对应 proposal 内容 |
| --- | --- | --- | --- | --- |
| Query Public Contract | 定义 route tag、DTO、ref kind、readiness、trust | P0 | `wiki-model` | 主合同收口 |
| Query Fusion Adapter | 把 index / knowledge / fallback / governance placeholder 映射成公开响应 | P0 | `wiki-runtime` | 来源分层与降级 |
| Runtime Transport Slimmer | 输出可消费但更小的 transport payload | P1 | `wiki-runtime` | 公开 payload 边界 |
| TS Parser Guardrail | 结构校验 + 原样保留，不推导业务语义 | P1 | `packages/spec-wiki` | 薄消费边界 |
| Human Explanation Contract | 统一 explanation 文案与 route 来源说明 | P1 | `wiki-runtime` / `packages/spec-wiki` | 可解释输出 |

### 功能详细设计

#### Query Public Contract

- 功能说明：在 `wiki-model::domain::query` 中定义公开 query DTO。
- 前置条件处理：DTO 必须独立于 runtime helper，且可被 status/query/Agent 共同消费。
- 业务逻辑实现：将 `QueryRouteTag`、`QueryResultDto`、`QueryRefKind`、`QueryRouteGroup`、`QuerySourceRef`、`QueryTrust`、`RecommendedAction`、`RuntimeReadiness` 作为稳定对象语言。
- 输出结果生成：Rust runtime、TS parser 和 Agent 资产都以此为基准。
- 异常场景处理：若新字段只存在于 runtime 私有层，不能进入主合同。

#### Query Fusion Adapter

- 功能说明：在 `wiki-runtime` 中把 index、knowledge、fallback、governance placeholder 装配成公开 query response。
- 前置条件处理：先完成 runtime readiness 判定，再决定是否允许 index/knowledge/fallback 命中。
- 业务逻辑实现：  
  - index ready 时，允许输出 `index_symbol_hit / index_path_hit / index_graph_hit`。  
  - knowledge ready 时，允许输出 `knowledge_declared_hit / knowledge_derived_hit`。  
  - page fallback 只能输出 `rendered_page_debug_fallback`。  
  - governance 未启用时，只能输出 `governance_readiness: not_enabled` 或空组。
- 输出结果生成：生成 `route_groups` 与 `results`，并汇总 `query_trust` 与 `recommended_action`。
- 异常场景处理：index missing/stale/blocked 时禁止伪造 index route；只能降级到 knowledge / fallback。

#### Runtime Transport Slimmer

- 功能说明：保留现有 slim payload 的思想，但用新公开 DTO 作为上游。
- 前置条件处理：rich runtime report 先完成装配，再做 transport slimming。
- 业务逻辑实现：`ExternalQueryReport` 继续负责 summary、hits 和 human readable payload，但它不再定义主合同。
- 输出结果生成：保留兼容摘要字段，但主合同由 route groups / results / readiness 承担。
- 异常场景处理：传输层不得重建 query 业务语义。

#### TS Parser Guardrail

- 功能说明：TS 侧做结构校验和原样保留，不推导 query 语义。
- 前置条件处理：parser 接受 Rust 返回的稳定主合同。
- 业务逻辑实现：校验枚举值、数组结构和必填字段；未知字段原样保留。
- 输出结果生成：Agent / CLI 继续读 runtime 输出，但不在 TS 中重新算 trust 或 route。
- 异常场景处理：遇到未知主 route tag 时直接报协议错误，而不是悄悄吞掉。

### 处理流程

```text
runtime readiness
  -> query fusion
  -> route grouping
  -> result dto assembly
  -> answer / explanation
  -> slim transport payload
  -> TS parse guardrail
  -> Host / Agent consumption
```

### 业务规则实现

| 规则 | 实现方式 | 对应 proposal 内容 |
| --- | --- | --- |
| `index_*` 只在 index ready 时返回 | runtime route gate | route tag 收口 |
| fallback 必须标记 `rendered_page_debug_fallback` | runtime result mapping | 降级语义 |
| `governance_*` 本 child 不做 scanner | 空组 / not_enabled 占位 | governance placeholder |
| score 不跨 route 乱比 | route group 输出 | DTO 收口 |
| TS 不推导 trust/action | parser guardrail | 薄消费边界 |

### 异常处理设计

| 异常场景 | 异常类型 | 处理策略 | 用户提示 / 系统行为 |
| --- | --- | --- | --- |
| index missing/stale/blocked 仍命中 index route | 协议错误 | runtime 不输出该 route | 降 trust 或拒绝响应 |
| fallback 混成正向命中 | 协议错误 | 标记为 `rendered_page_debug_fallback` | 明确降级 |
| governance 未启用 | 正常降级 | 返回 `not_enabled` 或空组 | 不阻断普通 query |
| TS 侧未知主枚举 | 解析错误 | 拒绝 payload | 早失败 |

## 数据设计

本 change 会新增稳定 query 对象语言，但不会引入新存储介质。

### 数据模型设计

| 字段 / 实体 | 类型 | 是否必填 | 业务含义 | 数据约束 | 对应 proposal 内容 |
| --- | --- | --- | --- | --- | --- |
| `QueryRouteTag` | enum | 是 | 公开 route 分类 | 枚举闭集 | route tag 收口 |
| `QueryResultDto` | struct | 是 | 单条 query 命中结果 | 必填字段闭集 | result DTO 收口 |
| `QueryRouteGroup` | struct | 是 | 按来源分组的结果集合 | group 内 score 可比较，组间不强比 | grouping 规则 |
| `QueryRefKind` | enum | 是 | 引用对象类型 | 必须区分 source/knowledge/projection/governance | source refs |
| `QuerySourceRef` | struct | 是 | 结果引用来源 | 必须携带 ref kind | provenance/source refs |
| `RuntimeReadiness` | struct | 是 | index/knowledge/projection/governance/fusion 状态 | 复用现有状态语义 | readiness 主合同 |

### 数据流向设计

```text
wiki-index facts
  -> wiki-runtime query fusion
  -> query route groups / results
  -> runtime summary / answer
  -> transport slim payload
  -> TS parse / display
```

### 存储方案

| 数据类型 | 存储介质 | 存储位置 | 索引 / 查询策略 | 保留策略 |
| --- | --- | --- | --- | --- |
| query DTO | 无新增存储 | 仅内存与 transport | 无 | 不落盘为 truth |
| transport payload | JSON | stdout / IPC | 由 CLI 消费 | 临时输出 |
| legacy summary | 字符串派生 | transport | 向后兼容阅读 | 可逐步弱化 |

## 接口设计

本 change 调整 query 公开契约，但不新增对外命令。

### 接口概览

| 接口名称 | 接口类型 | 方向 / 方法 | 所属模块 | 对应功能 / 集成需求 |
| --- | --- | --- | --- | --- |
| Query Response DTO | RPC / JSON | Rust -> TS | `wiki-model` / `wiki-runtime` | query 主合同 |
| Runtime Query Payload | CLI / JSON | Rust -> CLI | `wiki-runtime` | slim transport |
| parseResult | TS parser | CLI incoming | `packages/spec-wiki` | 结构校验 |

### 接口详细定义

#### Query Response DTO

- 接口路径 / 命令 / 事件名：`query`
- 接口描述：公开 query 的稳定返回对象语言。
- 所属模块：`wiki-model`
- 请求 / 输入参数：沿用当前 `term` 输入；本 change 不改 query 入参。
- 响应 / 输出结构：`readiness`、`query_trust`、`recommended_action`、`route_groups`、`results`、`answer`、`summary`、`hits`。
- 错误码 / 异常定义：未知主 route tag、非法 ref kind、fallback 未标记、index 伪命中都应视为协议错误或降级错误。

#### Runtime Query Payload

- 接口路径 / 命令 / 事件名：`spec-wiki query`
- 接口描述：runtime 对 CLI 的 JSON 输出。
- 所属模块：`wiki-runtime`
- 请求 / 输入参数：与现有 CLI 保持一致。
- 响应 / 输出结构：保留 transport-only slim 字段，主合同通过公开 DTO 承载。
- 错误码 / 异常定义：如果 TS 解析失败，应早失败而不是猜测修复。

### 外部系统集成接口

| 对接系统 | 接口方向 | 接口列表 | 数据格式 | 对接方式 |
| --- | --- | --- | --- | --- |
| `packages/spec-wiki` | 调用 | runtime query payload | JSON | 同步 |
| Agent assets | 被调用 | query result explanation | 文本 + JSON | 同步 |

## 非功能性设计

### 性能设计

| 性能指标 | 需求期望值 | 设计目标值 | 实现策略 |
| --- | --- | --- | --- |
| Query 延迟 | 现有水平 | 不显著劣化 | 只做协议收口，不增加新的扫描链 |
| Payload 体积 | 现有水平 | 稍增但可控 | 通过 slim transport 保持外部输出小体积 |

### 可维护性设计

| 设计维度 | 实现方案 |
| --- | --- |
| 日志规范 | query result 记录 route tag、readiness 和 recommended action |
| 配置管理 | 不新增 query 配置开关 |
| 版本兼容 | 当前阶段不要求历史兼容；旧字段仅可作为派生摘要 |

### 兼容性设计

| 兼容性要求 | 支持范围 | 实现方案 |
| --- | --- | --- |
| 旧 summary 字段 | 兼容阅读，不作为主合同 | 保留 `provenance_summary` 派生字段 |
| 旧 route 字符串 | 不作为主合同 | 明确降级，不再让其决定语义 |

## 资源评估

无新增资源要求。

## 风险与对策

### 技术风险

| 风险描述 | 影响程度 | 发生概率 | 应对策略 | 预留方案 |
| --- | --- | --- | --- | --- |
| 旧 `provenance_summary` 与新 route groups 双轨漂移 | 高 | 中 | 明确派生化，不让其当主合同 | 后续逐步弱化 |
| TS 重新推理业务语义 | 高 | 中 | 只做结构校验和原样保留 | 收紧 parser 校验 |
| `index_*` 在非 ready 状态被伪造 | 高 | 低 | runtime route gate | 直接失败或降级 |

### 依赖风险

| 依赖项 | 风险描述 | 应对策略 |
| --- | --- | --- |
| `code-graph-index` | 后续 graph 结果未成熟 | 本 change 只定义公开 route tag，不实现 graph schema |
| `governance-isolation` | governance 扫描范围膨胀 | 本 change 只做 `not_enabled` / 空组占位 |

## 设计决策

- query 公开 DTO 归属 `wiki-model`，不是 `wiki-runtime` 私有 helper。
- `wiki-runtime` 负责 query fusion 与 transport，`wiki-model` 负责公开对象语言。
- `packages/spec-wiki` 只做结构校验，不推导业务语义。
- `governance` 在本 child 中只做占位消费合同，不做 scanner。
- `provenance_summary` 保留为只读派生摘要，不作为主合同。
- `QueryResultDto` 的字段命名在本 child 中固定；后续 `code-graph-index` 只能适配这个公开合同，不能反向重命名主合同。

## 待确认问题

- 无。

## 参考资料

- `.spec/changes/refactor-specwiki-around-contract-closure-query-route-readiness/proposal.md`
- `.docs/design/specwiki-contract-closure.md`
- `.docs/design/knowledge-to-wiki-projection-contract.md`
- `.docs/design/governance-runtime-integration.md`
- `.docs/design/specwiki-cli-unification.md`
- `.wiki/03-模块指南/01-wiki-model.md`
- `.wiki/03-模块指南/04-wiki-runtime.md`
- `.wiki/03-模块指南/05-spec-wiki-cli.md`
- `.upstream/codegraph/src/context/index.ts`：仅借鉴 query result shaping，不直接迁移。
- `.upstream/codegraph/src/db/schema.sql`、`.upstream/codegraph/src/resolution/index.ts`：仅为后续 code-graph-index 借鉴，不在本 change 落地。
- `.upstream/GitNexus/ARCHITECTURE.md`：仅为 phase DAG 和 query consumption 借鉴，不在本 change 落地。
