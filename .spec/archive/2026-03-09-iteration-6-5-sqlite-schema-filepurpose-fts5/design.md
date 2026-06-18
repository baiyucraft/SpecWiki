## Context

当前 `wiki-core` 已经把 runtime 落到了 `.wiki/.cache/wiki-cache.db`，但真实实现仍停留在“SQLite 只是 JSON 容器”的阶段：

- `crates/wiki-core/src/storage/sqlite_store.rs` 只初始化 `kv_store`、`page_context_cache`、`page_generation_cache` 三张表。
- `crates/wiki-core/src/storage/state_store.rs` 仍把整个 `WikiState` 当作 `wiki-state` JSON 写进 `kv_store`。
- `crates/wiki-core/src/storage/cache_store.rs` 仍把 `repo-scan`、`module-tree` 当作 `kv_store` 的 key 读写。
- `crates/wiki-core/src/repo/scanner.rs` 的 `ScannedFile.kind` 仍是 `source/config/docs/...` 这类粗粒度字符串。
- `crates/wiki-core/src/workflows/query.rs` 仍主要基于内存结构匹配，尚未消费 SQLite 全文索引。

为避免 proposal 只复述文档，本次设计直接参考了上游源码中的具体实现，而不是只看 README：

- `deepwiki-rs/src/generator/preprocess/agents/code_purpose_analyze.rs`
  - 先走 `CodePurposeMapper::map_by_path_and_name()`，命不中再交给 AI 增强；这证明“规则优先、模糊情况再升级”是可落地的。
- `deepwiki-rs/src/types/code.rs`
  - `CodePurpose` 已有较细文件角色枚举，且路径/文件名双通道分类规则相当直接，适合作为 `FilePurpose` 设计参考。
- `deepwiki-rs/src/generator/workflow.rs`、`context.rs`、`compose/mod.rs`、`outlet/mod.rs`
  - workflow/context/compose/outlet 是显式阶段边界，不把缓存、内存、输出混在一起；这与本仓库“扫描 -> 模块树 -> 页面规划 -> Wiki Assembler”的主路径一致。
- `codewiki/codewiki/src/be/dependency_analyzer/ast_parser.py`、`topo_sort.py`、`documentation_generator.py`
  - 依赖图、Tarjan 环处理和“叶子先处理、父模块后处理”的顺序被固化在源码里，而不是停留在概念层；这提醒我们 6.5 要先把后续符号/关系迭代所需的 schema 预留好。
- `deepwiki-open/api/api.py`
  - wiki cache 仍是 JSON 文件；这适合消费层，但不适合作为 core 事实层模板，因此本次只借鉴其“不要把消费缓存反向污染 core”的边界，不借其存储形态。

## Goals / Non-Goals

**Goals:**

- 把 runtime 持久化从 `kv_store` JSON 主导升级为“关系型状态表 + 专用缓存表 + FTS5 虚拟表”。
- 保持 `WikiState` 作为内存事实主模型，但其读写来源改为关系型表组装，而不是单条 JSON blob。
- 引入 `FilePurpose` 24 类稳定文件角色，并让 scanner / hierarchy / planner / context 共享该角色信号。
- 让 `query` 具备页面标题/路径的 BM25 搜索能力，同时为迭代 7 的 `symbols` / `symbols_fts` 预建 schema。
- 给页面状态补入可复用的 `section_anchors` 聚合信息，降低 `sync`、`change_set`、section-level 脏检测的重复推导成本。
- 让 steering 的扫描配置对齐 `scan.ignore` / `scan.include`，并保留一轮兼容读取空间。

**Non-Goals:**

- 本迭代不实现 tree-sitter 符号抽取，不向 `symbols`、`edges`、`communities`、`processes` 写真实数据。
- 本迭代不引入向量检索、RRF、sqlite-vec 或 LLM 参与 `FilePurpose` 分类。
- 本迭代不重写 Agent 接入层；`agents/*` 继续只做参数收集和 binary 调用。
- 本迭代不改变 `.wiki/*.md` 与 `wiki.metadata.json` 的产物边界，也不把消费层缓存逻辑搬进 core。

## Decisions

### 1. 用“新 schema 初始化 + 不完整库回退”升级 SQLite runtime，而不是维护旧 DB 自动迁移入口

**Decision**

- `open_db()` 在打开数据库后统一确保当前关系型 schema 和 FTS 表存在。
- 对于缺失关键状态表、关键映射表或关键记录的不完整库，不提供旧版 DB 自动回填入口；读取层直接回退到 metadata / rebuild 路径。
- 新的写路径只写关系型状态表、缓存表和 FTS 表；`kv_store` 不再作为 6.5 的事实源或升级入口。

**Rationale**

- 当前版本还未正式发布，旧版 DB 不构成需要维护的线上兼容面。
- 与其维护一条一次性 legacy 回填链，不如把行为收敛到“新库初始化 + 不完整库回退”，减少半升级 runtime 的复杂度。

**Alternatives considered**

- 强制检测到旧 schema 就报 `needs_rebuild`：实现最简单，也是当前阶段可接受的边界。
- 新建第二个 DB 文件：会制造双状态源，违背 `.wiki/.cache/wiki-cache.db` 的单库边界。

### 2. 保留 `WikiState` 聚合模型，但把持久化拆成关系型行

**Decision**

- 新增以下事实表：
  - `wiki_pages`
  - `wiki_page_sections`
  - `source_states`
  - `modules`
  - `module_source_map`
  - `page_source_map`
  - `page_module_map`
  - `wiki_relations`
- `read_state()` 改为从这些表装配 `WikiState`；`write_state()` 改为把 `WikiState` fan-out 到这些表。
- `WikiPageState.section_anchors` 作为页面级聚合字段持久化到 `wiki_pages`，其值来自 `wiki_page_sections` 中的 managed section ID 序列。

**Rationale**

- 现有 `query`、`sync`、`change_set` 都围绕 `WikiState` 工作，直接删除该聚合模型会让 6.5 变成架构重写。
- 关系型拆分可以让后续迭代直接把 `symbols`、`edges`、`processes` 接到同一 DB，而不需要再次拆 JSON blob。
- `wiki_page_sections` 让 section-level 脏检测、managed drift 和用户区段锚点不再依赖页面 JSON 反序列化。

**Alternatives considered**

- 继续把 `WikiState` 序列化成一条 JSON：无法支持 FTS、关系查询和后续 symbol/process 表接入。
- 只把页面行拆表，section 仍存 JSON：会让 `sync` 和 `change_set` 继续重复解析页面状态，收益不足。

### 3. 缓存表保持“专用表”策略，不在 6.5 引入新的泛化缓存抽象

**Decision**

- 保留现有 `page_context_cache` 与 `page_generation_cache` 两张每页缓存表，避免一次性重写增量 runtime。
- 新增 `scan_cache` 表承载扫描缓存。
- 新增 `llm_cache` 表，但 6.5 只建表不写业务数据。
- 预建 `symbols`、`edges`、`communities`、`community_members`、`processes`、`process_steps`，6.5 只保证 schema 可写、可查。

**Rationale**

- 当前 `update` / `sync` / `rebuild` 已经稳定依赖 page-scoped cache，直接切到泛化 `generation_cache` 会制造大量无关 churn。
- 6.5 的目标是把未来迭代的 schema 基座打好，不是今天就把所有 cache 抽象重做一遍。

**Alternatives considered**

- 现在就把 page cache 改成统一 `generation_cache`：更“漂亮”，但与当前代码路径不匹配，收益小于风险。

### 4. `FilePurpose` 采用“细角色 + 粗语义分组”双层消费模式

**Decision**

- 把 `ScannedFile.kind: String` 升级为 `ScannedFile.purpose: FilePurpose`。
- `FilePurpose` 使用 24 个稳定角色：`Entry`、`Router`、`Controller`、`Handler`、`Service`、`Model`、`Repository`、`Domain`、`Agent`、`Library`、`Middleware`、`Plugin`、`Utility`、`Helper`、`Constant`、`Type`、`Page`、`Component`、`Widget`、`Layout`、`Config`、`Migration`、`Test`、`Docs`。
- 为兼容当前 hierarchy / planner / context 的粗粒度判断，引入 `purpose_family()` / `is_code_like()` / `is_low_signal()` 这类辅助方法，而不是让所有旧逻辑立刻理解 24 类角色。
- 6.5 只实现 deterministic 的路径/文件名规则；未命中的代码文件回落到保守泛型角色（如 `Library`），不调用 LLM。

**Rationale**

- `deepwiki-rs` 的 `CodePurposeMapper` 证明“路径 + 文件名 -> 角色”的 deterministic 分类足够实用。
- 直接把所有旧判断从 `source/config/docs` 一次改写成 24 类会把 6.5 变成行为重构；双层消费能先把数据模型升级，再渐进替换下游逻辑。

**Alternatives considered**

- 完全保留 `kind`，只额外挂 `purpose_hint`：会继续让主链路依赖旧字段，升级效果有限。
- 直接在 6.5 引入 LLM 分类：超出本迭代边界，也不符合 deterministic-first 原则。

### 5. 页面搜索先做 FTS5/BM25，查询结果继续回填到现有 QueryReport

**Decision**

- 在 `wiki_pages` 之上建立 `wiki_pages_fts`，索引字段至少包含 `title`、`path`，并允许追加 `summary` 作为检索文本。
- 预建 `symbols_fts`，但 6.5 不向其中写真实符号数据。
- `run_query()` 先执行 FTS5 页面搜索，拿到 `page_id` 后再从 `WikiState` / 关系型状态行 hydrate 页面、模块、源码、关系上下文；原有结构化匹配仍保留为补充和兜底。

**Rationale**

- 现有 `query` 已有稳定的返回结构，优先把召回源替换为 BM25，比一次性重写整个 query contract 更稳妥。
- `symbols_fts` 先建空表，可以避免迭代 7 再做一次 schema 升级。

**Alternatives considered**

- 继续扫描 Markdown 文件做文本匹配：实现简单，但无法复用 SQLite，性能和后续扩展性都差。
- 直接做页面 + 符号 + 图查询三路融合：这是迭代 8 的范围，当前数据基础还未齐。

### 6. Steering 扫描配置内部统一到 `scan.ignore/include`，读取层兼容旧 schema 一轮

**Decision**

- 新的内部模型使用：
  - `scan.ignore: string[]`
  - `scan.include: string[]`
- scanner 先应用内置忽略与 `scan.ignore`，再用 `scan.include` 做白名单恢复。
- 读取层允许把旧的 `ignore.global` / `ignore.<language>` 归一化到新模型，至少在 6.5 这一轮不直接打断现有测试与仓库配置。

**Rationale**

- `.wiki/06-设计文档/00-总体设计.md` 已经把 steering 扫描配置定义为 `scan.ignore/include`，6.5 应先把最基础的扫描入口对齐。
- 现有测试和仓库可能已经写了旧格式；完全硬切会让 proposal 落地成本不必要地增大。

**Alternatives considered**

- 彻底保留旧 schema：会继续扩大设计与实现的偏差。
- 只接受新 schema，不做兼容读取：会让现有 steering 文件一次性失效，切换成本不必要。

## Risks / Trade-offs

- `[旧库/半成品库处理不清]` → 统一把不完整 schema 视为 runtime 损坏，走 metadata/rebuild 回退，不在 6.5 维护自动升级链。
- `[状态表变多，初始化代码更长]` → 把 schema 初始化、状态装配拆到独立 storage 模块，避免 workflow 直接拼 SQL。
- `[FilePurpose 规则误分类]` → 在 `crates/wiki-core/tests/` 新增针对 fixture 和真实测试项目的分类断言，并优先让下游逻辑消费 family/helper 而不是硬编码枚举细节。
- `[FTS 索引与基础表不同步]` → 所有页面行写入和 FTS 更新放到同一事务中，`sync` / `update` / `rebuild` 共用统一写接口。
- `[旧 steering 配置长期拖延]` → 6.5 兼容读取，但输出 warning 并在文档中标记迁移方向，避免永久背双 schema。

## Runtime Plan

1. 打开 `.wiki/.cache/wiki-cache.db` 时执行 schema 检查和补表。
2. 所有 workflow 统一走新写接口：状态表、缓存表、FTS 表在单事务中写入。
3. 读取路径优先用关系型表装配 `WikiState`；当关键表或关键记录缺失时回退到 `wiki.metadata.json` / `rebuild`。
4. 对旧库、损坏库和半成品库，`status` 提升为 `needs_rebuild`，由 `rebuild` 生成干净的新库。

## Open Questions

- `kv_store` 是否在 6.5 之后完全移除，还是仅保留 page cache 以外的空壳表等待后续清理。
- `wiki_pages_fts` 是否在 6.5 就纳入 `summary` 字段；当前最小需求只需要标题与路径。
- 旧 steering schema 的兼容读取是否需要显式 warning，还是仅在文档中说明迁移方式。
