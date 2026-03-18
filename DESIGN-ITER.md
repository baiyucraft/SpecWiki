# 实施迭代

## 重排历史

迭代 0-5 期间经历过五次重排（详见 git 历史）。迭代 5 完成后，DESIGN-CORE.md 基于四个上游项目（GitNexus、codewiki、deepwiki-rs、deepwiki-open）源码实际阅读进行了完全重写，核心架构从文件级扫描升级为三层解析（文件扫描 → 符号解析 → 图分析），存储从 JSON 文件迁移到 SQLite，搜索从结构化查询升级为 BM25 + 图查询 + 混合搜索。迭代 9.4 完成后，基于 storybook/dagger 两个验收样本的 reference 深度对比和四个上游项目源码的再次深入阅读，核心设计升级为 DESIGN-CORE2.0.md，pipeline 从三段式（Facts → Planner → Renderer）重构为四层式（Facts → Knowledge Planning → Research → Compose），引入知识单元抽象、research-first composition 和 leaf-first/parent-consume-child 模式。

迭代 6 完成了基础 SQLite 迁移（kv_store + page cache）、Steering 配置、页面拓扑稳定和 section 模板丰富。随后插入的迭代 6.5 已完成并归档，补齐了完整关系型 schema、FilePurpose 分类和 FTS5 搜索；迭代 7 现已完成并归档；迭代 8、8.5、9、9.1、9.2 也均已完成并归档。当前 9.3-9.5 已把主链推进到 `Facts -> Knowledge Planning -> Research -> Compose`、provider-first research、final markdown evidence/citation/diagram contract 和 reference 专项收敛；后续新增迭代从 9.6 开始，优先解决 `overall=100%` 但 `low-fidelity=全量` 的验收与内容保真问题。

## 依赖关系

```text
迭代 6: SQLite 基础 + 页面拓扑 + Steering（已完成）
    ↓
迭代 6.5: SQLite 关系型 schema 升级 + FilePurpose + FTS5（已完成）
    ↓
迭代 7: 符号解析（tree-sitter，已完成）
    ↓
迭代 8: 符号关系解析 + 图分析 + 搜索升级
    ↓
迭代 8.5: 进度流 + 解析热路径优化
    ↓
迭代 9: LLM 增强（已完成）
    ↓
迭代 9.1: Topic Planner + Evidence Layer（已完成）
    ↓
迭代 9.2: Dossier + Research Session + Budget/Usage（已完成）
    ↓
迭代 9.3-9.5: 2.0 主链重构 + provider-first/evidence/decomposition 专项收口（进行中）
    ↓
迭代 9.6: Fidelity Gates & Reference Report Hardening
    ↓
迭代 9.7: Deterministic Ownership & KnowledgeUnit Refinement
    ↓
迭代 9.8: PageResearchPacket & Child Digest Utilization
    ↓
迭代 9.9: Compose Contract Tightening & Fidelity Convergence
    ↓
迭代 10: Agent 消费层（TOON + RAG）
    ↓
迭代 11: CodeBuddy Agent 可用化
    ↓
迭代 12: 分发与平台扩展
```

## 已完成迭代

- [x] 迭代 0：Bootstrap & Host Shell
  - Windows + CodeBuddy 薄接入、JSON IPC、基础 runtime 落盘。
- [x] 迭代 1：Deterministic Structural Baseline
  - deterministic 主链打通。递归层级、结构优先 query、metadata 导出、baseline 验收闭环。
- [x] 迭代 2：State Kernel & Query Contract
  - WikiState 成为事实主模型，MetadataMapper 单向导出，query 新增 context_pack 和 provenance_summary。
- [x] 迭代 3：Incremental Runtime
  - source → module → page 映射、DirtyState、section-level 重生成接口。
- [x] 迭代 4：Scanner & Hierarchy Quality
  - 噪声过滤、单文件模块抑制、module kind 多维分类、关键源码选择信号修复。
- [x] 迭代 5：Editable Wiki Runtime
  - managed section kernel、parser-first sync、user section 保留、legacy 页面迁移。
- [x] 迭代 6：Page Topology, Steering & SQLite Storage
  - SQLite 基础迁移（kv_store + page_context_cache + page_generation_cache 三张表）。
  - Steering 配置（`.wiki/wiki.steering.yaml`）：忽略路径、模块提升/降级、合并阈值。
  - page_id 锚定到模块 root_paths[0]，page_path 按模块树层级映射。
  - planner 合并策略（页面权重评分）和父子关系按模块树层级分配。
  - 页面类型扩展为 overview / architecture / module / workflow。
  - section 模板差异化丰富（overview 5 sections、architecture 4 sections、module 5 sections、workflow 4 sections）。
- [x] 迭代 6.5：SQLite Schema 升级, FilePurpose & FTS5
  - SQLite runtime 从 `kv_store` 主导升级为关系型状态表、映射表、`scan_cache` 和 FTS5 索引。
  - `WikiState` 改为按页面、section、源码、模块、关系行式持久化，补入 `section_anchors`。
  - scanner 引入 `FilePurpose` 分类，并升级 steering 扫描边界为 `scan.ignore` / `scan.include`。
  - `query` 接入 `wiki_pages_fts` BM25 页面检索，并与结构化结果合并。
  - 对 19 个测试项目完成 `init` 与生命周期验证，change 已归档到 `openspec/changes/archive/2026-03-09-iteration-6-5-sqlite-schema-filepurpose-fts5/`。
- [x] 迭代 7：Symbol Parsing（tree-sitter）
  - 新增独立 `parse_symbols` 阶段，主链升级为 `scan -> parse_symbols -> module_tree -> planner -> render`。
  - 升级整套 tree-sitter 栈并接入 12 种核心语言；React(JSX/TSX) 委托到底层 JS/TS grammar，Vue/Svelte 包装层延期到迭代 8。
  - `symbols` / `symbols_fts` 从空 schema 升级为真实定义类符号存储，`query` 新增 `matched_symbols` 与 symbol BM25 命中回填。
  - 对 19 个测试项目完成 `init` 项目集分析，并完成 `bootstrap / steady / mutation / rebuild` 分阶段生命周期验证；change 归档到 `openspec/changes/archive/2026-03-10-iteration-7-symbol-parsing-tree-sitter/`。

## 迭代 6.5：SQLite Schema 升级, FilePurpose & FTS5

状态：已完成并归档

归档：`openspec/changes/archive/2026-03-09-iteration-6-5-sqlite-schema-filepurpose-fts5/`

目标：把迭代 6 的基础 SQLite 存储（kv_store + page cache）升级为 DESIGN-CORE 定义的完整关系型 schema，引入 FilePurpose 24 种文件角色分类和 BM25 全文搜索。

背景：迭代 6 的 SQLite 迁移采用了 kv_store 模式（全局数据存为 JSON 字符串），满足了从 JSON 文件到 SQLite 的基本迁移需求。但新 CORE 要求符号、关系、社区、执行流等数据以关系型表存储，且需要 FTS5 全文搜索索引。本迭代把存储层从 kv_store 升级为完整关系型 schema，为后续符号解析和图分析提供存储基础。

范围：

SQLite schema 升级：

- 新建关系型表替代 kv_store 中的 JSON 存储：
  - `wiki_pages`（替代 kv_store 中的 WikiState.pages JSON）
  - `source_states`（替代 kv_store 中的 WikiState.sources JSON）
  - `modules`（替代 kv_store 中的 ModuleTree JSON）
  - `module_source_map`
  - `page_source_map`
  - `page_module_map`
- 预建符号层和图分析层的空表（迭代 7/8 填充数据）：
  - `symbols`、`edges`
  - `communities`、`community_members`
  - `processes`、`process_steps`
- 预建 LLM 缓存表（迭代 9 使用）：
  - `llm_cache`
- 保留 `scan_cache` 和 `generation_cache` 表（从 kv_store 迁移为独立表，或保留现有 page cache 表）。
- 建立索引：`idx_symbols_file`、`idx_symbols_name`、`idx_symbols_label`、`idx_edges_source`、`idx_edges_target`、`idx_edges_type`、`idx_modules_parent`。
- 迁移 state_store / cache_store 的读写路径：从 kv_store JSON 读写改为关系型表读写。

FTS5 全文搜索：

- 建立 `wiki_pages_fts` FTS5 虚拟表（title, path）。
- 建立 `symbols_fts` FTS5 虚拟表（name, file_path, content）——表结构先建，数据在迭代 7 填充。
- 升级 `query` 工作流：引入 BM25 页面搜索（搜索 wiki_pages_fts），与现有结构化查询合并。

FilePurpose 分类：

- 升级 `FileRecord.kind` 为 `FilePurpose` 24 种文件角色分类。
- 实现 deterministic 文件名/路径模式匹配规则。
- 升级 Steering 配置：`scan.ignore` / `scan.include` 结构对齐新 CORE 定义。

WikiPageState 升级：

- 补入 `section_anchors` 字段（managed section 的 anchor ID 列表）。
- 升级 section-level 脏检测逻辑消费 section_anchors。

完成标准：

- WikiState 的 pages / sources / modules 通过关系型表读写，不再依赖 kv_store JSON。
- 符号层和图分析层的表结构就绪（空表），迭代 7/8 可直接写入数据。
- `query` 能通过 BM25 搜索页面标题和路径。
- FilePurpose 分类在测试项目集中覆盖主要文件角色。
- section_anchors 参与 section-level 脏检测。

## 迭代 7：Symbol Parsing（tree-sitter）

状态：已完成并归档

归档：`openspec/changes/archive/2026-03-10-iteration-7-symbol-parsing-tree-sitter/`

目标：引入 tree-sitter 符号级 AST 解析，建立 SymbolNode / SymbolEdge / SymbolTable 模型，完成 `parse_symbols` 阶段。

范围：

- 引入 `tree-sitter` crate 及 12 种语言的 grammar crate（TypeScript, JavaScript, Python, Java, C, Go, C++, C#, Rust, PHP, Kotlin, Swift）。
- 为每种语言编写 S-expression query，统一提取 `@definition`、`@import`、`@call`、`@heritage` 四类 capture。
- 实现 `SymbolNode` 模型（id, name, label, file_path, start_line, end_line, is_exported, language）。
- 实现 `SymbolEdge` 模型（id, source_id, target_id, edge_type, confidence, reason）。
- 实现 `SymbolTable` 双索引（file_index + global_index）。
- 实现每种语言的导出检测（isNodeExported）。
- 符号数据写入 `symbols` 表，同步更新 `symbols_fts` FTS5 索引。
- 实现文件内容按字节预算分块读取（CHUNK_BYTE_BUDGET，20MB/chunk）。
- `parse_symbols` 接入 init pipeline。

完成标准：

- 对测试项目集执行 `init` 后，SQLite 中 symbols 表包含函数/类/方法/结构体等符号节点。
- 12 种语言的 S-expression query 能正确提取定义和导入。
- 符号表双索引可用于后续的导入/调用解析。
- `query` 能通过 BM25 搜索符号名和文件路径。

## 迭代 8：Symbol Resolution, Graph Analysis & Search

状态：未开始

目标：完成符号关系解析（导入/调用/继承），引入图分析（社区检测/执行流/环检测），升级搜索为 BM25 + 结构化图查询。

范围：

符号关系解析：

- 实现 `SuffixIndex`（文件路径后缀索引，O(1) 导入路径匹配）。
- 实现 `resolve_imports`：语言专用导入解析器（TS/JS path alias、Rust crate::/super::、Go module path、Java/Kotlin wildcard、PHP PSR-4、Swift SPM），解析缓存。
- 实现 `resolve_calls`：三级置信度（import-resolved 0.9、same-file 0.85、fuzzy-global 0.3-0.5），内置函数过滤（BUILT_IN_NAMES）。
- 实现 `resolve_heritage`：继承/实现关系解析。
- 关系数据写入 `edges` 表。
- 补齐 Vue / Svelte 单文件组件的 symbol 包装层：抽取 `<script>` / `<script setup>`，识别 `lang="ts"`，委托到底层 JS / TS parser，并把符号行号映射回原组件文件。

图分析：

- 实现 `detect_communities`：Leiden 算法社区检测，大图优化（>10K 符号时过滤低置信度边），社区标签启发式。数据写入 `communities` + `community_members` 表。
- 实现 `detect_processes`：入口点评分（callRatio × exportMultiplier × nameMultiplier × frameworkMultiplier），BFS 正向追踪（maxDepth=10, maxBranching=4），子集去重 + 端点去重。数据写入 `processes` + `process_steps` 表。
- 实现 `detect_cycles`：Tarjan SCC 环检测 + 断边策略 + 拓扑排序。

搜索升级：

- 实现 BM25 全文搜索：同时搜索 `symbols_fts` 和 `wiki_pages_fts`，按 score 合并排序。
- 实现结构化图查询：基于 edges 表的 `WITH RECURSIVE` CTE 多跳遍历，支持调用链追踪和影响分析。
- 升级 `query` 工作流：BM25 搜索 + 图查询 → merge & rank → 结构化输出。

Pipeline 升级：

- 升级 `build_module_tree`：消费 CALLS/IMPORTS 边增强模块间关系推断。
- 升级 `build_contexts`：消费社区、执行流，新增 `symbol_stats` 和 `call_hotspots`。
- 升级 `plan_pages`：消费社区聚类，workflow 页面类型消费执行流。
- 升级 `update` 工作流：增量重解析受影响文件的符号。

完成标准：

- 对测试项目集执行 `init` 后，edges 表包含 IMPORTS/CALLS/EXTENDS/IMPLEMENTS 关系，置信度分布合理。
- 社区检测能按代码协作关系聚类，而非仅按目录。
- 执行流检测能从入口点追踪到完整调用链。
- `query` 能通过 BM25 搜索符号和页面，能通过图查询追踪调用链。
- 符号图结果能够稳定供后续 planner、query 和 LLM 增强阶段消费。

## 迭代 8.5：Progress Streaming & Parser Hot Path

状态：规划中

目标：在不扩张 graph 呈现范围的前提下，补齐长流程可感知进度，并把最影响体感的 parser / update 热路径先做快，让迭代 8 已有能力真正被用户感知。

范围：

运行时感知：

- 扩展 `wiki-core --json` 协议：`init / update / rebuild` 统一按 NDJSON 长流程事件流输出。
- 输出 NDJSON `progress / result / error` 事件流；`status / query / sync` 继续保持单个最终响应。
- 为长流程定义稳定阶段：`scan`、`parse_symbols`、`resolve_symbol_graph`、`analyze_symbol_graph`、`build_module_tree`、`build_contexts`、`plan_pages`、`render_pages`、`write_state`、`write_metadata`。
- CodeBuddy Agent 改为逐行消费 progress 事件并透传，但不改变六个工具的业务接口。

热路径优化：

- `parse_symbols` 改为同一 `ParseUnit` 只做一次 `parse_tree` 和一次 query 构建，definitions 与 raw captures 共用解析工件。
- parser worker 按语言复用 `Parser / Query`，避免大仓库在每个文件上重复编译同一套 tree-sitter 工件。
- 保留 `CHUNK_BYTE_BUDGET`，但在 chunk 内引入有限的文件级并行解析，最终结果保持 deterministic。
- `render_pages` 改为并行预渲染 page context / bundle、串行写盘与 cache 收口，优先压缩大仓库页面生成阶段的串行耗时。
- `update` 新增按文件路径读取 `symbols / edges` 的局部工作集路径；小范围 graph 变化优先走局部读取，超阈值时显式回退到全量路径。

延期到后续迭代：

- scanner 的目录遍历/指纹并发，明确归到 `迭代 9` 的吞吐补强。
- `community / process / cycle` 的子阶段并行化，明确归到 `迭代 9` 的 graph 吞吐补强。
- “更强 graph 呈现”“页面信息密度提升”明确归到 `迭代 9`。
- TOON / RAG / 混合搜索继续归 `迭代 10`。
- 宿主侧正式 progress UI / 消费面抛光明确归到 `迭代 10`。

完成标准：

- `init / update / rebuild` 能持续输出稳定 phase 进度，而不是只有最终结果。
- Agent 能消费 progress 事件并继续返回最终结果。
- symbol parsing 不再为同一 parse unit 重复 parse/query，且并行度变化不影响 symbol ID 和结果顺序。
- 小范围 `update` 不再固定回读全量 `symbols / edges`。

## 迭代 9：LLM-Enhanced Content Generation

状态：已完成并归档

归档：`openspec/changes/archive/2026-03-11-iteration-9-llm-enhanced-content-generation/`

里程碑：第一个可用版本。

目标：引入 LLM 辅助判断（Uncertainty Gate）和 LLM 内容增强，让 Wiki 从骨架模板升级为有实质信息密度的项目文档。

范围：

Uncertainty Gate：

- 定义 `LlmAssist` trait（identify_repo_type / classify_file_roles / judge_module_boundary / classify_module_kind / resolve_dependency_semantics）。
- 实现 `llm_cache` 表的读写（input_hash, prompt_type, response, model, created_at, ttl_seconds）。
- 实现 5 个触发场景：仓库类型识别、文件角色判断、模块边界判断、模块 kind 分类、跨模块关系语义。
- 实现 Steering 中 `llm.uncertainty_gate` 和 `llm.max_calls` 控制。
- 每个触发点实现 deterministic fallback。
- FilePurpose 分类中 deterministic 匹配不到的文件走 LLM 辅助判断。

LLM 内容增强：

- 实现叶子优先文档生成顺序（拓扑排序 → 叶子模块先生成 → 父模块基于子模块文档摘要生成）。
- 升级 Context Builder：用 LLM 生成模块职责说明、架构解释。
- 升级 Document Generator：用 LLM 写模块文档（输入：符号图 + 源码 + 子模块文档摘要）。
- 实现基础 Diagram Generator：LLM 生成 Mermaid 图 + MermaidFixer 自动修复。

吞吐与 graph 呈现补强：

- 实现 scanner 的目录遍历/指纹并发，避免进入 LLM 增强阶段后扫描成为新的长尾瓶颈。
- 实现页面 `context / render` 预计算并行，让内容增强和页面组装不会把总初始化时间进一步拉长。
- 实现 `community / process / cycle` 子阶段并行，降低 graph analysis 在大仓库上的额外耗时。
- 收口“更强 graph 呈现”和页面信息密度提升：让迭代 8 的 graph facts 在页面正文、架构解释和 workflow 描述中真正变成高信息密度内容，而不是只停留在底层事实层。

完成标准：

- 没有 LLM 时，全链路仍可完整运行 deterministic 主链。
- 有 LLM 时，Wiki 页面内容质量明显高于 deterministic baseline。
- Uncertainty Gate 的 LLM 调用结果被缓存，相同输入不重复调用。
- 页面能输出基础 Mermaid diagram。
- 页面内容信息密度和结构解释能力，相比迭代 8/8.5 的 deterministic graph baseline 有明显提升。

结果补充：

- 建立了 provider 优先、agent bridge 回退的 LLM 接入路径。
- 引入 `uncertainty_gate` 与 `content_enrichment` 两层能力，并补齐 provider cache / fallback / retry / debug trace 基线。
- 首轮 reference 对比表明剩余差距主要不在 prompt，而在 planner 粒度、evidence 密度和页面结构。

## 迭代 9.1：Topic Page Planner & Evidence Layer

状态：已完成并归档

归档：`openspec/changes/archive/2026-03-11-iteration-9-1-topic-page-planner-and-evidence-layer/`

目标：把 9 的 LLM 能力真正接到页面结构层，引入专题页 planner、evidence/source layer 和 deterministic Mermaid，让页面从“少量增强页”升级为“带专题拆分与出处层”的结构化 Wiki。

范围：

- 引入 `topic` 页面类型，稳定 `topic_kind/topic_key` 身份，并补齐根级主题、模块能力主题和流程主题规划。
- 引入 `ChildPageRollup`、`evidence_groups`、`diagram_inputs`，让父页消费结构化子页结果，而不是只吃扁平 `facts`。
- 为页面 renderer 增加 evidence block 和 deterministic Mermaid 落盘路径。
- 收口 `status/update` 对 LLM 扫描结果的使用边界，避免 runtime 状态判断被 LLM purpose 污染。
- 输出 reference 项目逐项目报告和下一步优化建议。

完成标准：

- 带 reference 的项目能稳定生成专题页、evidence block 和基础结构图。
- `status / update / rebuild` 在启用 LLM 时仍保持状态判断稳定。
- reference 对比报告能明确收敛剩余差距，而不是只停留在 prompt 层讨论。

## 迭代 9.2：Dossier, Research Session & LLM Budget Controls

状态：已完成并归档

归档：`openspec/changes/archive/2026-03-12-iteration-9-2-dossier-session-and-llm-budget-controls/`

目标：在 planner 与 render 之间引入 dossier 层和 bounded research session，补齐 provider tools、budget/usage、cache mode 和 learned capability，让 LLM 路径从“单轮增强”升级为“可观测、可控、可回退的 research 闭环”。

范围：

- 引入 `ModuleDossier`、`TopicDossier`、`ChildPageRollup` 和稳定 `PageResearchResult`。
- 为 `module/topic` 页接入 bounded provider research session 和只读 tool schema。
- 实现 `native_tools -> emulated_tools -> no_tools -> deterministic` 降级链，以及默认发送 `response_format` 的双保险策略。
- 增加 phase-specific budget、cold/warm cache mode、`~/.spec-wiki/config.yaml` / `state.yaml`、realtime token usage 和 provider learned capability。
- 补齐 provider 兼容、网络重试、warm cache、生效日志和 reference 项目报告。

完成标准：

- provider-first 路径下的 dossier / research session / tool loop / budget / usage 能稳定运行。
- cold run 与 warm run 能明确区分，并且 warm cache 实际减少请求与 token。
- reference 对比和 trace 复盘能说明剩余差距主要在 dossier 精度、session 深度和 renderer 消费方式，而不是“是否已接上大模型”。

## 迭代 9.6：Fidelity Gates & Reference Report Hardening

状态：未开始

目标：先把“什么叫像 reference 的可读页面”定义清楚。`9.6-9.9` 的总体目标不是继续堆指标或补索引能力，而是把 `.wiki/` 收敛成真正接近 reference 的可读文档：页数基本一致、主题页为主、docs-backed 页面稳定输出 `cite -> 目录 -> 简介 -> 项目结构 -> 核心组件 -> 架构总览 -> 详细组件分析 -> 依赖关系分析 -> 性能考量 -> 故障排查指南 -> 结论 -> 附录` 这样的主章节骨架，让人打开就能看懂，而不是只看到一堆索引和聚合页。本迭代先把 `storybook + dagger` 专项验收口径变成稳定、可复现、可定位的工程指标，优先修复“`overall_match_rate = 100%` 但 `low-fidelity = 全量`”与“reuse 严重但 `collapsed = 0`”这类报告口径失真问题；本迭代只升级报告与验证，不改 core 生成主链。

范围：

报告口径硬化：

- 升级 `scripts/collect-reference-project-reports.mjs`，把最终对比表中的 `reference -> generated` 匹配对显式保留为正式分析输入，而不是只在中间选择器阶段短暂存在。
- 新增 `many_to_one reuse` 一级指标：
  - `reuse_count(page)`
  - `reuse_pages`
  - `severe_reuse_pages`
  - `reuse_overage`
- 新增 `skeleton fidelity` 一级指标：
  - 从最终 `.wiki/*.md` 与 reference markdown 提取 H2/H3 heading
  - 忽略 `目录`、`附录`、`章节结构图` 与空标题 cite preamble
  - 做 normalize 后基于 LCS 计算 `skeleton_score`
- 新增 `key source coverage` 一级指标：
  - 优先提取 `file://` citation 链接
  - 再 fallback 到 file mention regex
  - 计算 generated/reference 的关键文件集合重合率
- 保留 `overall_match_rate`，但降级为一级门槛，不再单独作为完成判据。

报告定位能力：

- 在项目报告中新增：
  - `reuse reverse index top offenders`
  - `skeleton lowest pages`
  - `key-source lowest pages`
- 把 `symptom -> metric -> offending pages -> contract hypothesis` 串成正式 gap ledger，而不是只给总分。

稳定性验证：

- 对同一项目执行 warm report 至少两次，验证 `reuse_overage / median skeleton / median key-source` 的波动在可接受范围内。
- 为 heading normalize / ignore list / key-source extraction 增加脚本级测试，避免 renderer 自动块导致指标抖动。

不在本迭代处理：

- 不修改 `wiki-core` 的 planner / research / compose 主链。
- 不引入新 evidence 数据模型。
- 不调整 citation 下限、diagram fallback 或 provider 行为。

完成标准：

- `storybook + dagger` 的专项报告不再只给 `overall_match_rate`，而会明确回答：
  - 生成页数是否接近 reference
  - 是否存在“一个生成页对应多个 reference 页”的粗页/复用问题
  - docs-backed 页面是否具备 reference 式章节骨架
  - 页面正文是否真正提到了 reference 中的关键文件
- `reuse` 成为与 `missing / collapsed / low-fidelity / extra generated` 并列的正式专项指标。
- `skeleton fidelity` 与 `key source coverage` 能直接从最终 Markdown 计算，并稳定进入项目报告。
- 报告能够定位最严重的 reuse page、骨架偏差页和关键文件缺失页，而不是只输出汇总数字。
- 对 `storybook` 与 `dagger` 的 warm report 重跑后，新指标波动可控，可作为后续 9.7-9.9 的验收基线。

## 迭代 9.7：Deterministic Ownership & KnowledgeUnit Refinement

状态：未开始

目标：把“少量粗页”和“过多噪声页”一起压下去，让页面数量和 page tree 先长对。在不引入 LLM 细分的前提下，把 coarse-page / reuse-page 的主要根因收回到 deterministic planner，建立 KnowledgeUnit 的 owner contract、路径归属上下文与有边界的局部 refinement。最终要让 `.wiki/` 更接近 reference 的页数和页树形态，并减少“看起来像目录，不像文档”的页面。

范围：

Facts/Planning 基础设施：

- 引入 `PathOwnershipContext` / `ImportResolutionContext` 一类的 deterministic 归属上下文，供 planner / research / query 共享只读结果。
- 归属上下文优先覆盖：
  - docs corpus ownership
  - module path ownership
  - config surface ownership
- 不在本迭代把 `calls / process / community` 全面 confidence 化；仅允许在 ownership 判断上增加 deterministic 信心分级。

KnowledgeUnit owner contract：

- 为 `KnowledgeUnit` 引入可检查的 owner/intention 结构，而不是单句描述，至少回答：
  - 它承载的知识意图是什么
  - 为什么由它而不是兄弟页承载
  - 它明确排除什么内容
- owner contract 要能被 planner、report 和后续 research packet 一致消费。

Deterministic refinement：

- refinement 不允许 LLM 参与，只允许根据 repo 自身 facts/signals 触发。
- 首批允许的触发条件：
  - 单个 unit 同时命中多个互斥 decomposition profile
  - 同一 unit 内 docs anchors 形成多个顶层 skeleton cluster
  - key source files 在 path ownership 上形成多个低交集簇
- refinement 输出必须重新落回正式 `KnowledgeUnit[]`，重新 build `KnowledgeTree.processing_order`，正常写库与缓存。
- 限制 refinement 只做一层局部细分，并引入页面预算守卫，避免 extra generated pages 反向失控。

Pruning/Ownership 收口：

- 延续并统一当前已有的 docs corpus 选择、meta docs 过滤、root README 回收、thin domain synthetic index 回收逻辑。
- 把这类规则正式归类为 ownership/pruning contract，而不是继续散落成样本收口经验。

完成标准：

- planner 能基于 deterministic 规则识别 coarse-page 候选并触发局部 refinement。
- `.wiki/` 中的低价值噪声页明显减少，不再靠索引页和派生页充数。
- `storybook + dagger` 上：
  - 生成页数更接近 reference
  - “一个页面复用多个 reference 页”的现象明显下降
  - 以可读主题页为主，不再由少量 overview/index 页承担大部分内容
- owner contract 与 refinement 输出都能稳定落库，进入后续 research/compose 输入。

## 迭代 9.8：PageResearchPacket & Child Digest Utilization

状态：未开始

目标：让 research 真正围绕“可读页面”产出内容，而不是只给摘要和索引。把 research contract 从“section plan + summary + evidence clusters”升级为更强的页面级研究包，并让 `parent-consume-child` 从架构描述变成可统计、可验收的正式 contract。重点是让 docs-backed 页面开始稳定长成 reference 式目录骨架，而不是继续生成“看起来像索引”的说明页。

范围：

Research packet 升级：

- 在现有 `UnitResearch` 基础上演化出 `PageResearchPacket`，至少包含：
  - `packet_version`
  - `unit_intent`
  - `topic_claims`
  - `section_blueprints`
  - `key_file_agenda`
  - `open_questions`
- packet schema 要进入 `research_cache` 的 input hash contract，并有长度上限与版本号。
- 不重做 evidence 底座；继续沿用现有 `SourceCitation`，必要时只做增量字段扩展。

Docs-backed 页面研究收口：

- 把 docs-backed 页面明确拆成两部分：
  - skeleton preservation
  - narrative enrichment
- 先固定主章节骨架，再为每章补 claims / citations / child inputs，避免 compose 重新泛化成通用模板。

Child digest utilization：

- 为 `PageDigest` 引入结构化 `claims`。
- `section_blueprints` 显式列出 `child_claim_refs`。
- renderer 将 child claim 的使用落成结构化标记，供 report 聚合，不允许仅靠 LLM 自述“我用了子页结果”。
- 新增 `child_digest_utilization` 指标，只对有 child units 的页面统计。

关键文件提及前移：

- 把“关键文件提及”从 report 事后诊断前移为 research packet 的正式 contract。
- `key_file_agenda` 必须能被 compose/renderer 落到最终页面，并被后续 report 验证。

完成标准：

- `PageResearchPacket` 能稳定缓存、聚合和恢复，不破坏现有 provider/session contract。
- docs-backed 页面在 research 阶段就能明确主章节骨架，而不是到 compose 时再临时组织。
- 页面会显式围绕关键文件、关键组件、关键关系展开，不再只停留在泛化摘要。
- 父页会真正吸收子页结论，页面之间开始形成清晰分工，减少“大页重新包一遍所有内容”。
- `parent-consume-child` 在最终页面中有可统计痕迹，而不是只存在于中间内存或 prompt。
- `child_digest_utilization` 和 `key source coverage` 能在 `storybook + dagger` 上形成正向相关提升。

## 迭代 9.9：Compose Contract Tightening & Fidelity Convergence

状态：未开始

目标：把最终 `.wiki/*.md` 收敛成真正可读、接近 reference 的页面。在 9.6-9.8 的新指标和新 contract 基础上，收紧 compose/assemble 的质量边界，把当前“靠 citation/diagram fallback 保底”的状态推进到“由 research packet 主导页面 fidelity”的状态，并完成 `storybook + dagger` 的专项收敛。最终要让用户打开页面时，看到的是接近 reference 的完整文档，而不是“有很多结构块，但读起来还是像系统产物”。

范围：

Compose contract 收紧：

- compose 主输入收敛为：
  - `PageResearchPacket`
  - `child digests / child claims`
  - `citation materialization`
- citation/diagram fallback 保留，但降级为兜底，不再承担主要质量来源。
- explanation density 不再用 citation 数近似，而改由：
  - section claims 是否被展开
  - key files 是否被解释
  - child digests 是否被实际吸收
 共同约束。

专项收敛：

- 以 `storybook + dagger` 为固定专项样本，围绕：
  - `reuse`
  - `skeleton fidelity`
  - `key source coverage`
  - `child digest utilization`
  逐轮收敛。
- `extra generated pages` 继续作为反向约束，防止靠盲目增页换 fidelity。
- provider stability 与 content fidelity 分离验收：
  - runtime/provider 问题单独记为稳定性 gate
  - 页面质量问题单独记为内容 gate

报告与 gate 收口：

- 把 9.6 引入的指标正式纳入专项完成条件，而不是只作为观察项。
- 报告输出统一按 `symptom -> layer -> contract -> fix path` 组织。

不在本迭代处理：

- 不把 query/runtime/session 混入 core fidelity 收敛。
- 不新增样本专有 planner / renderer 分支。

完成标准：

- `storybook + dagger` 的最终 `.wiki/*.md` 同时满足：
  - 页数与 reference 基本接近
  - 页面主要由可读主题页构成，而不是索引页主导
  - docs-backed 页面稳定输出 reference 式主章节骨架
  - 页面正文能围绕关键组件、关键文件、依赖关系和排障内容展开
- `storybook + dagger` 不再出现“`overall=100%` 但 `low-fidelity=全量`”的假通过状态。
- compose 质量提升能够通过 `reuse/skeleton/key-source/child-digest` 四组指标共同反映，而不是靠单个统计数字。
- citation / diagram 继续稳定存在，但不再是页面唯一可见价值，也不再是报告中的主要短板。

## 迭代 10：Agent Query Surface & Consumption Layer

状态：未开始

目标：补齐面向 Agent 消费的高阶输出面，引入 TOON 格式和 RAG 向量检索。

范围：

TOON 格式：

- 实现 `QueryRequest.format` 支持 `json | toon`。
- 实现 TOON 序列化器（缩进 + 符号代替 JSON 括号引号，token 效率优化）。
- 实现 `ContextPack` 完整输出（pages, modules, sources, relations, symbols, processes, provenance_summary）。

RAG 向量检索：

- 引入 `sqlite-vec` 扩展。
- 建表 `embeddings`、`embeddings_vec` 虚拟表。
- 定义 `EmbeddingProvider` trait（embed_batch / dimensions / model_id）。
- 实现 Embedding Pipeline 四阶段（查询待 embedding 节点 → 文本生成 → 批量 embedding → 写入索引）。
- 实现按节点类型定制 embedding 文本（Function/Class/Method/Interface/Page）。
- 实现增量 embedding（只对新增或变更节点生成）。

混合搜索：

- 实现 RRF 融合排序（BM25 + 语义向量，k=60）。
- 升级 `query` 工作流：BM25 + 向量近邻 + 图查询 → RRF merge → 输出。

其他：

- query 引入 LLM 摘要增强。
- 补齐 Diagram Generator 高级形态。
- 在宿主能力允许时，把 8.5 的 progress 事件接到正式 Agent 消费/展示层，而不只停留在底层流式协议。
- 对 TOON / `ContextPack` / progress 展示做统一宿主消费面整理，让 Agent 真正把 8.5 和 9 的增强能力暴露出来。
- 为宿主消费面补齐 progress / 状态展示约定，但 CodeBuddy Agent 的正式 bridge 落地和可用性收口放到 `迭代 11`。

完成标准：

- Agent 可以通过 `format: "toon"` 获取 token 高效的结构化输出。
- Agent 可以通过语义搜索在 wiki 内容中检索相关信息。
- 混合搜索的结果质量优于纯 BM25。

## 迭代 11：CodeBuddy Agent Usability & Session Bridge

状态：未开始

目标：让 CodeBuddy Agent 对新一轮 LLM / research session 能力稳定可用，并补齐与 core 的正式宿主桥接。

范围：

- 校准 CodeBuddy Agent 的工具定义、参数约定、输出约定和错误呈现。
- 实现 CodeBuddy Agent 对 `agent_session_v1` 的 bridge，覆盖 session 事件、tool 调用、tool 结果和结构化 `PageResearchResult` 回填。
- 实现 Agent 侧只读 tool schema 执行与 usage 回传，不在宿主层重写 Wiki 业务语义。
- 补齐 CodeBuddy Agent 对 progress / realtime token usage / research session 状态的消费与展示约定。
- 补齐 CodeBuddy Agent 相关自动化测试与 e2e 验证，覆盖 provider session、回退路径和长流程稳定性。
- 按 CodeBuddy 官方接入方式收口安装、升级和可用性验证，确保这套宿主链路真正可用。

完成标准：

- 按 CodeBuddy 官方接入方式可以稳定安装和使用。
- CodeBuddy Agent 能稳定消费 `progress`、`usage` 和 `agent_session_v1` 事件，并继续返回最终 workflow 结果。
- CodeBuddy Agent 路径下的 provider session、tool 调用与回退行为有自动化验证覆盖。

## 迭代 12：Distribution, Hosts & Platform Expansion

状态：未开始

目标：完成分发、版本控制元信息抽象，以及跨平台和跨宿主扩展。

范围：

- 将 Git 元信息提升为统一版本控制元信息层。
- 从 CodeBuddy Agent 扩展到 CLI，再逐步考虑其他 IDE / Agent 宿主。
- 扩展 Linux / macOS 支持。
- 收口多宿主、多平台下的分发、安装和版本约束，不再让当前实现绑定单一平台或单一宿主。

完成标准：

- `wiki-core` 不再依赖单一平台或单一宿主。
- 至少一个非 CodeBuddy 宿主和一个非 Windows 平台具备可验证的正式接入路径。
