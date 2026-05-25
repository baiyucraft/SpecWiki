# Repo Wiki Runtime Design

## 文档定位

本文档描述的是整个 wiki 系统如何运行，而不只是 `wiki-runtime` 这个 crate。

它回答五类问题：

- 四个 crate 如何协作
- `.wiki/` 内部到底放什么
- query 是怎么走的
- `init / update / sync / rebuild / status / query` 的状态如何流转
- A 用户提交后，B 用户如何基于上库产物恢复本地 runtime

宿主接入、bootstrap、全局 CLI 与多宿主扩展模型不在本文档展开，单独见 [DESIGN-AGENTS.md](./DESIGN-AGENTS.md)。

## 参考实现边界

本文档允许在实现细节上借鉴本地参考仓库，但这些参考不直接构成当前 runtime 的真相来源。

当前原则：

- runtime 设计以本仓库的 3.0 思想、场景文档和当前代码演化目标为准
- 参考仓库必须以实际源码实现为依据，而不是只参考文档说明
- 如果旧设计里某条“参考结论”没有被重新验证到真实源码，可以推翻
- 参考仓库只用于帮助判断具体实现策略，例如索引、研究、投影、查询和消费层做法
- 参考仓库不直接决定 `.wiki/` 结构、知识分层或包边界

## 源码核实后的实现参考矩阵

这个矩阵只回答“某一层实现手法优先看哪个参考仓库”，不反向定义当前系统的设计边界。

| 参考仓库 | 参考层 | 源码证据 | 可采纳结论 |
| --- | --- | --- | --- |
| `deepwiki-rs` | `wiki-knowledge` 的 `research / compose orchestration` | [.upstream/deepwiki-rs/src/generator/workflow.rs](E:/project/!byAI/spec-wiki/.upstream/deepwiki-rs/src/generator/workflow.rs) `76-107` 明确分成 `preprocess -> research -> compose` 三段；[.upstream/deepwiki-rs/src/generator/step_forward_agent.rs](E:/project/!byAI/spec-wiki/.upstream/deepwiki-rs/src/generator/step_forward_agent.rs) `34-77` 定义 `DataSource` 与 `AgentDataConfig`；[.upstream/deepwiki-rs/src/generator/compose/agents/overview_editor.rs](E:/project/!byAI/spec-wiki/.upstream/deepwiki-rs/src/generator/compose/agents/overview_editor.rs) `26-35` 直接声明 compose 依赖 research result | 可以参考“先 research，再由 compose 消费 research 产物”的编排方式，也可以参考按 agent 声明输入依赖的 contract 设计 |
| `CodeWiki` | `wiki-knowledge` 的 `leaf-first assembly` | [.upstream/codewiki/codewiki/src/be/documentation_generator.py](E:/project/!byAI/spec-wiki/.upstream/codewiki/codewiki/src/be/documentation_generator.py) `74-85` 先递归处理 children 再处理 parent；同文件 `99-123` 在父级 overview 生成前装入一层子文档；同文件 `201-223` 明确 parent doc 基于 children docs 生成 | 可以参考“叶子先产出，父级再消费子级结果”的装配顺序，适合知识单元投影和稳定 page 的逐层汇总 |
| `GitNexus` | `wiki-index` 的 `index / symbol / graph / resolution substrate` | [.upstream/GitNexus/gitnexus/src/core/ingestion/model/symbol-table.ts](E:/project/!byAI/spec-wiki/.upstream/GitNexus/gitnexus/src/core/ingestion/model/symbol-table.ts) `90-174` 有 `fileIndex / globalIndex / callableIndex / fieldByOwner`；[.upstream/GitNexus/gitnexus/src/core/ingestion/pipeline.ts](E:/project/!byAI/spec-wiki/.upstream/GitNexus/gitnexus/src/core/ingestion/pipeline.ts) `97-99` 定义 `20MB` chunk budget，`479-488` 明确 parse / import / call / heritage 分阶段 loop，`254-281` 有跨文件绑定传播，`672-678` 在 call resolution 前做 wildcard import synthesis 与 type seeding；[.upstream/GitNexus/ARCHITECTURE.md](E:/project/!byAI/spec-wiki/.upstream/GitNexus/ARCHITECTURE.md) `19` 明确 ingestion pipeline 是 phase DAG，`66-67` 给出 `scan -> structure -> parse -> crossFile -> mro -> communities -> processes`，`82-85` 说明关键阶段的依赖与职责 | 可以从三层参考它：一是“厚索引底座”，让 `provider / agent` 先命中 file-symbol-edge 图事实，再做更高层 query；二是 `query / impact / graph consumption` 侧的工具化组织，尤其适合方法定位、调用关系、跨文件解析和 impact analysis；三是 `wiki-index` 内部的 phase DAG 编排与阶段拆层，用于提升索引构建的可演进性与可诊断性，但这不构成 knowledge 主链模板 |
| `deepwiki-open` | `wiki-runtime` 或 query 面的 `session / RAG / consumption layer` | [.upstream/deepwiki-open/api/data_pipeline.py](E:/project/!byAI/spec-wiki/.upstream/deepwiki-open/api/data_pipeline.py) `153-180` 递归读文档，`382-446` 建 `splitter + embedder` pipeline，`831-912` 优先复用本地 DB；[.upstream/deepwiki-open/api/websocket_wiki.py](E:/project/!byAI/spec-wiki/.upstream/deepwiki-open/api/websocket_wiki.py) `88-115` 准备 retriever，`189-242` 以 query 拉取上下文，`268-313` 明确它是面向 query 的多轮研究流程 | 可以参考 query/session 侧如何组织检索上下文、如何复用本地检索库、如何围绕用户问题做消费层编排，但它不是 core 生成主链模板 |

## 运行时总架构

```text
                +------------------+
                |   Scan Code      |
                +------------------+
                         |
                         v
                +------------------+
                |   Build Index    |
                | file symbol edge |
                +------------------+
                         |
                         v
                +------------------+
                |  Plan Knowledge  |
                | unit / domain    |
                +------------------+
                         |
                         v
                +------------------+
                | Research / Merge |
                +------------------+
                         |
                         v
                +------------------+
                | Knowledge Store  |
                +------------------+
                    |          |
           +--------+          +--------+
           |                            |
           v                            v
   +------------------+       +------------------+
   | Query for AI     |       | Page Projection  |
   | fast find path   |       | only if needed   |
   +------------------+       +------------------+
```

对应到四包：

```text
wiki-model
  -> 提供稳定共享对象

wiki-index
  -> 产出 files / symbols / edges / module tree / graph facts

wiki-knowledge
  -> 产出 domains / units / research / records / projections

wiki-runtime
  -> 编排 workflow / query / storage / transport / lifecycle
```

## 双主链

本系统同时存在两条正式主链：

生成链：

```text
Facts -> Knowledge Planning -> Research -> Compose -> Assemble
```

查询链：

```text
symbol -> graph -> declared knowledge -> derived knowledge -> page
```

含义：

- 生成链负责把代码事实组织成知识并落成正式产物
- 查询链负责让人和 Agent 先命中最有效的答案层，而不是默认先翻 page

当前 `v0.2.0` 对这两条主链的公开承诺，已经收稳为 `minimal formal knowledge runtime`：

- 已正式承诺最小 `KnowledgeUnit / declared record / research summary / projection digest / health signal` 合同
- 已正式承诺 `status / query / sync / update / rebuild` 的最小运行语义
- 未承诺完整 knowledge system
- 未承诺 provider-backed 大仓库样本已稳定完成 full compose

## 包边界与依赖合同

### 依赖方向

```text
wiki-model      <- 所有包依赖它
wiki-index      <- 依赖 wiki-model
wiki-knowledge  <- 依赖 wiki-model + wiki-index
wiki-runtime    <- 依赖 wiki-model + wiki-index + wiki-knowledge
```

### 包边界

- `wiki-model`
  - 负责稳定共享模型、公共枚举、query DTO 和正式 state/metadata 对象
  - 不负责 workflow、索引构建、knowledge 生成或运行时落盘
- `wiki-index`
  - 负责 scan、symbols、graph facts、module tree 与 index-first query 底座
  - 不负责 declared knowledge、page projection 或 `.wiki` 生命周期
- `wiki-knowledge`
  - 负责 knowledge planning、research、compose、declared/derived knowledge 与 projection decision
  - 不负责 transport、CLI/IPC、`.cache` 恢复或宿主集成
- `wiki-runtime`
  - 负责 workflow orchestration、query route、storage、transport、`.wiki` 生命周期与恢复
  - 不重新拥有 facts/index 和 knowledge 的主实现，只负责编排它们

### 宿主边界

- `Agents` 只负责宿主接入、参数收集、binary 调用与结果消费
- `Agents` 的公共 bootstrap 内核、资产模型与宿主扩展方式见 [DESIGN-AGENTS.md](./DESIGN-AGENTS.md)
- `Agents` 不承载 Wiki 业务规则，不重写知识模型与 query 语义

### 当前阶段明确不拆

- `wiki-page`
- `wiki-storage`
- `wiki-llm`
- `wiki-query`

原因：

- 当前对象模型和知识生命周期仍在演化
- 过早细拆只会放大 DTO、trait 边界和循环依赖成本
- 当前收益最大的边界不是更碎，而是先把四层收稳

## 核心对象分层

### Layer A: Code Facts / Index

回答：

- 代码里有什么
- 方法在哪
- 符号之间如何调用
- 哪些入口和模块最相关

主要对象：

- file
- symbol
- edge
- process
- community
- module tree

特点：

- 可重建
- 确定性优先
- 是 `query` 和 `impact analysis` 的第一入口

### Layer B: Derived Knowledge

回答：

- 系统从 facts 中整理出了什么知识
- 哪些知识域和知识单元成立
- 哪些内容值得做稳定摘要或 page 投影

主要对象：

- `KnowledgeDomain`
- `KnowledgeUnit`
- research packet
- digest / summary
- projection state

特点：

- 来源于 facts，但不是 facts 本身
- 允许持续刷新
- 可能变化快于 declared knowledge

### Layer C: Declared Knowledge

回答：

- 团队明确声明了什么规范、约定、决策、避坑
- 这些知识适用于什么范围
- 当前是生效、冲突、过时还是被替代

主要对象：

- convention
- policy
- guideline
- pitfall
- decision
- workflow rule

特点：

- 需要可审计
- 需要作用范围
- 需要状态与来源
- 既服务人读，也服务 Agent 约束

## `.wiki/` 运行时结构

```text
.wiki/
├─ .knowledge/
│  ├─ declared/
│  ├─ derived/
│  └─ runtime/
├─ pages/
│  ├─ 项目概览.md
│  ├─ 系统架构.md
│  └─ ...
├─ wiki.metadata.json
└─ .cache/
```

### `.wiki/.knowledge/`

这是 Git-tracked 的知识层。

职责：

- 保存正式知识记录
- 保存知识摘要与投影所需的稳定对象
- 为 query、update、rebuild 和协作恢复提供上库基础

建议子层：

- `declared/`
  - 人明确声明的规范、约定、避坑、政策、决策
- `derived/`
  - 系统从 facts 中提炼出的知识摘要、模式和单元级结果
- `runtime/`
  - 需要上库的轻量运行时状态，例如投影绑定、知识版本、恢复锚点

### declared artifact contract

当前 `declared/**` 不再只是最小 writeback 占位，而是正式 truth layer。

最小约束：

- declared record 必须具备稳定 `record_id` 与 `authoring_id`
- scope 必须是 typed scope object，而不是松散字符串
- lifecycle 关系最小只承诺 `deprecated / replaced_by / supersedes`
- page 只是 authoring surface；正式 declared truth 以 `.wiki/.knowledge/declared/**` 为准

恢复前提也要一起成立：

- `wiki.metadata.json` hash 一致
- `.wiki/.knowledge/declared/**` snapshot 一致
- `.wiki/pages/**` 当前内容仍与 metadata 记录的 content hash 一致

```mermaid
flowchart LR
    A[page declared block] -->|sync validate| B[declared artifact]
    B --> C[update/status/query]
    B --> D[recovery manifest]
    D --> E[restore cache/runtime]
    A -.不是 truth source.-> E
```

### `.wiki/pages/`

这是正式 page 投影层。

职责：

- 提供面向人阅读的稳定入口
- 为 Agent 提供补充上下文与稳定引用锚点

原则：

- page 是 projection，不是主本体
- 不允许为了“看起来很全”而盲目增殖 page
- 新知识默认先进入记录或摘要层，足够稳定再升级为 page

### `wiki.metadata.json`

这是 Git-tracked 的正式索引层。

职责：

- 保存 page、knowledge、scope、状态、版本和恢复所需的稳定索引
- 为 query route、sync、cache rebuild 提供正式入口

不负责：

- 不承载全部知识正文
- 不替代 `.knowledge`
- 不替代 `.cache`

### `.wiki/.cache/`

这是本地运行时缓存层，不上库。

职责：

- 加速 query、update、rebuild
- 保存可重建的派生运行时结构

原则：

- 不是正式真相源
- 应可由 `.knowledge + pages + wiki.metadata.json` 重建
- 如果本地代码与上库状态不一致，恢复后应显式标记为 `stale` 或 `needs_update`

## 什么上库、什么不上库

上库：

- `.wiki/.knowledge/**`
- `.wiki/pages/**`
- `.wiki/wiki.metadata.json`

不上库：

- `.wiki/.cache/**`

理由：

- `.knowledge / pages / meta` 共同组成可共享、可审计、可恢复的正式产物
- `.cache` 只是本地派生层，应该始终可丢弃、可恢复

## Page Projection 策略

### 默认策略

新知识产生时，默认先更新：

- 知识记录
- 知识摘要
- 投影状态

不是默认直接长 page。

### 何时升级为 page

满足下列条件时可升级：

- 主题足够稳定
- 适用范围明确
- 被重复查询或重复引用
- 具备长期阅读价值

典型保留 page：

- 项目概览
- 系统架构
- 规范索引
- 故障排查索引
- 少量稳定主题页

## Query Route

目标查询顺序：

```text
symbol first
-> graph next
-> declared knowledge next
-> derived knowledge next
-> page last
```

展开后：

```text
[Agent Ask]
      |
      v
[Query Route]
      |
      +--> [Symbol Match]
      |
      +--> [Graph Match]
      |
      +--> [Declared Knowledge Match]
      |
      +--> [Derived Knowledge Match]
      |
      +--> [Page Match]
      |
      v
[Return Best Answer]
```

意图：

- 查方法、查入口、查调用链时优先命中事实层
- 查规范、查约定、查避坑时优先命中 declared knowledge
- 查项目理解和稳定阅读入口时再回到 page

## 当前 query 边界与演进方向

### `v0.1.0` 当前 query 边界

当前收敛版发布里，外部 query 合同仍保持：

```text
term only
-> runtime route
-> index-first result
-> page fallback only if needed
```

这意味着：

- 当前外部正式输入仍是 `term`
- runtime 当前优先把 query 路由到 facts/index substrate
- page 仍然只是 fallback 或补充投影，不是 query 主体
- process / community 当前虽然属于 Layer A facts，可作为后续 query 扩展依据，但不应被误写成 `v0.1.0` 已正式承诺的稳定命中层

### GitNexus 对 query 的参考边界

GitNexus 对当前系统的 query 演进有明确参考价值，但参考面主要集中在：

- `wiki-index` 的厚索引底座
- symbol / graph / process / community 的 facts-owned query substrate
- impact analysis、context drilling 和 process trace 的查询消费组织
- `wiki-index` 内部 ingestion pipeline 的 phase DAG 编排
- hybrid ranking、provenance 和结果分组策略

不应把它直接当成：

- `wiki-knowledge` 的知识组织模板
- `.wiki/pages/**` 的页面语义模板
- 当前 runtime query contract 的直接真相来源

换句话说：

- GitNexus 更适合作为 `query / impact / graph consumption` 侧参考
- 它也可作为 `wiki-index` 内部 phase DAG 编排的工程参考
- 它不直接定义当前系统的 `KnowledgeUnit` 主线
- 它也不直接定义当前系统的 knowledge/page projection 结构

补充说明：

- “厚索引底座”与 `query / impact / graph consumption` 参考不是新增判断，只是对既有表述的进一步收紧
- 这次新增强调的参考点，是 GitNexus 把 `scan -> structure -> parse -> crossFile -> mro -> communities -> processes` 拆成显式 phase DAG，并用清晰的阶段输入输出约束提升索引构建的可演进性与可诊断性
- 这条参考只适用于 `wiki-index` 内部的构建编排与阶段拆层，不构成 `spec-wiki` 的知识主链模板

### query 演进原则

后续 query 若继续增强，优先级应是：

1. 先补清晰的 query routing / result shaping 规则
2. 再定义 process / community 的命中语义、排序规则、截断策略和 provenance
3. 再把 richer graph projection 正式提升到 transport / DTO 合同
4. 最后再考虑 semantic search 作为可选增强层，而不是当前主链前提

具体约束：

- 短期内外部可继续保持 `term-only` 合同稳定
- 内部可以继续演进 intent routing、graph projection 和 result shaping
- 当前内部已经存在 `callers / callees / impact slice` 一类 graph substrate；后续应明确哪些升级为正式输出，哪些仍保持内部能力
- process grouping 不应压过 `symbol -> graph -> declared knowledge -> derived knowledge -> page` 这条查询主线
- 一旦 query 同时返回 facts、knowledge、process 和 page，多层结果必须有统一 ranking 与 provenance，避免宿主消费失真

### 已明确延期到后续设计的 query 能力

这轮 `v0.1.0` 收口后，以下能力明确保留为后续设计，不作为当前 runtime 对外合同：

- `intent-aware query` 外部输入
  - 当前正式输入仍然只保留 `term`
  - 后续若要参考 GitNexus 的 query substrate，把 intent 显式升级为外部 payload，必须先定义稳定的 `intent / focus / scope / traversal` 合同，而不是让宿主靠 description 猜
- 稳定的 `owner / entrypoint / impact` 输出 schema
  - 当前 index/graph 内部已经有一部分 substrate，但还没有形成稳定 transport 字段、排序规则与置信度解释
  - 后续只有在字段定义、ranking、provenance 和截断策略稳定后，才适合提升为正式 query contract
- 更完整的 query 质量信号模型
  - 当前外部只正式暴露 `runtime_state / query_mode / query_trust / recommended_action`
  - 后续若要补 richer quality signal，应统一回答“结果是否完整、是否来自 fallback、是否需要 rebuild、覆盖面有多大”，避免宿主继续自己拼状态机

## Workflow 生命周期

### `init`

目标：

- 首次建立 index、knowledge、page projection、metadata 和本地 cache

主路径：

```text
scan
-> build index
-> plan knowledge
-> research
-> compose
-> assemble
-> write .knowledge / pages / meta / .cache
```

### `update`

目标：

- 在代码变更后更新相关知识，而不是盲目重写全部 page

主路径：

```text
detect affected facts
-> locate affected knowledge scope
-> refresh derived knowledge
-> merge or append declared/linked records when needed
-> refresh impacted projections
-> update metadata and cache
```

### `sync`

目标：

- 处理人改 page、人声明规则、知识记录变化与 runtime 之间的同步

关注点：

- 记录变化是否需要更新 page
- page 是否与知识记录脱节
- 当前状态是 `ready`、`stale`、`needs_update` 还是 `conflict`

### `rebuild`

目标：

- 丢弃本地 cache 或派生层后，从正式产物和当前代码重新构建运行时

### `status`

目标：

- 告知当前 runtime 是否可用、是否过时、是否存在冲突或待同步事项

### `query`

目标：

- 在 facts、knowledge 和 page projection 之间做正式路由，而不是继续 page-first

## 协作恢复场景

对应 [SCENE-1.md](./SCENE-1.md) 的场景 9。

场景：

- A 用户执行 `init` 或 `update`
- A 用户提交 `.wiki/.knowledge/`、`pages/`、`wiki.metadata.json`
- B 用户拉取代码，但本地没有 `.cache`

期望流程：

```text
read .knowledge + pages + meta
-> rebuild local .cache
-> verify current repo state
-> mark ready / stale / needs_update
```

约束：

- B 不应被强制重新全量 `init`
- `.cache` 恢复成功后应可直接 `query`
- 如果当前代码比上库 wiki 更新，应允许恢复后进入增量校正

## 主动声明规范与知识生命周期

对应 [SCENE-1.md](./SCENE-1.md) 的场景 6、7、8。

### 主动新增规范

例如用户主动增加“注释规范”：

```text
declare convention/policy
-> bind scope
-> write declared record
-> refresh summary/projection
-> make query and agent prompt consumable
```

### 规范废弃或替代

不是简单从系统里“物理删除”。

更合理的是：

- 修改记录状态
- 标记为 deprecated / replaced
- 记录替代关系
- 刷新投影与查询结果

当前 authoring 约束也要写死：

- 若同一 section 内需要并存多条同 `kind + scope` 的 declared record，必须显式提供不同 `id`
- `deprecated`、`replaced_by`、`supersedes` 不能随意混用，必须能唯一推导 lifecycle status
- relation target 必须存在于 merged declared snapshot，且保持同 kind、同 canonical scope、无环

### bug 经验升级

bug 修复后默认先沉淀为 pitfall record。

只有当满足下列条件时再升级为 convention 或 policy：

- 被重复验证
- 适用范围清晰
- 不再只是一次性事故

## 冲突治理

当声明型知识与现实代码不一致时，系统不能静默选边。

必须显式暴露：

- 冲突对象
- 事实侧证据
- 规则侧证据
- 当前暂定状态
- 下一步治理建议

## 一句话总结

```text
3.0 的运行时不是“把 Markdown 写进 .wiki”这么简单，
而是让 facts、derived knowledge、declared knowledge、page projection 和 local cache 各归其位，并能被人和 Agent 共同消费。
```



