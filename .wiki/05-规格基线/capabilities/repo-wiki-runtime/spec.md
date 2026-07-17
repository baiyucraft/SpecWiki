# repo-wiki-runtime Specification

## Purpose
定义 Repo Wiki 页面运行时、页面状态、managed section 与 steering 配置消费的持久化约束。
## Requirements
### Requirement: 正式 Wiki 页面必须稳定映射到模块结构
系统 MUST 让正式 Wiki 页面与模块结构保持稳定映射，以便后续 `query`、`sync`、`update` 和 `rebuild` 可以围绕层级化页面工作。模块页的关键源码表达 MUST 优先选择入口、依赖证据和核心实现文件，并抑制日志、锁文件、纯文档和低价值配置噪声。模块页的页面标识或路径必须由模块 root_path 稳定导出，并记录基础 provenance。页面状态还 MUST 记录稳定 section 标识、section 级 generated/observed hash，以及 user section 的锚点信息，使 runtime 能在保留人工内容的同时局部重组页面。页面的父子关系 MUST 按模块树层级分配。被合并的小模块 MUST 作为父模块页面的子模块概述 section 出现，而不是生成独立页面。steering 配置 MUST 作为 runtime 的一部分被读取和消费。

#### Scenario: 写入模块页
- **WHEN** 系统为模块生成正式 Wiki 页面
- **THEN** 页面状态中必须记录对应模块标识
- **THEN** 页面来源中必须记录该页面依赖的源码集合
- **THEN** 同一模块在重复生成时必须使用稳定页面标识或路径
- **THEN** 页面标识必须锚定到模块 root_path，而不是模块名或发现顺序

#### Scenario: 写入递归模块页
- **WHEN** 模块页对应多级嵌套模块
- **THEN** 系统必须为该页面写入稳定的父页面关系，父页面 MUST 是模块树中父模块对应的页面
- **THEN** 系统必须避免因 Windows 非法路径字符或同名模块导致落盘冲突

#### Scenario: 导出高信号关键源码
- **WHEN** 系统生成模块页摘要和 metadata 页面来源
- **THEN** 关键源码列表必须优先包含入口、依赖证据或核心实现文件
- **THEN** 关键源码列表不得被日志、锁文件、纯文档或低价值配置主导

#### Scenario: 重复生成页面时 section 身份稳定
- **WHEN** 同一页面在未发生对应 section 输入变化的情况下被重复生成
- **THEN** 页面中的 section 标识必须保持稳定
- **THEN** 未变化 section 的 generated hash 和 provenance 映射必须保持可复用

#### Scenario: 页面写盘时包含 managed section marker
- **WHEN** 系统在 `init`、`update` 或 `rebuild` 中写出正式 Wiki 页面
- **THEN** 所有 runtime 托管 section 必须以稳定 marker 包裹
- **THEN** 页面文件仍必须保持为正常 Markdown 文档

#### Scenario: 被合并模块作为父模块页面的子模块概述
- **WHEN** 某模块因合并策略不生成独立页面
- **THEN** 该模块的内容 MUST 作为父模块页面的"子模块概述" managed section 出现
- **THEN** 该 section MUST 包含被合并模块的名称、角色和关键源码信息

#### Scenario: steering 配置作为 runtime 的一部分
- **WHEN** `.wiki/config.yaml` 存在
- **THEN** 系统 MUST 在 `init`、`update`、`rebuild` 时读取该配置
- **THEN** 配置 MUST 影响 planner 的页面规划决策

#### Scenario: 扩展的 section 模板提升内容密度
- **WHEN** 系统生成 overview 或 architecture 页面
- **THEN** overview 页面 MUST 包含技术栈和入口与构建 section
- **THEN** architecture 页面 MUST 包含模块树的文本化层级表达
- **THEN** module 页面 MUST 包含关键源码列表和依赖关系 section

### Requirement: 正式 Wiki 页面必须支持可回退的 LLM 增强 section 与图内容
系统 MUST 允许 `overview`、`architecture`、`module` 和 `topic` 页面在现有 managed sections 中承载 research-driven section 内容和 Mermaid 图内容。增强内容 MUST 复用现有 `page_id`、`section_id` 和 managed marker，而不是创建新的运行时层或脱离页面状态表的 sidecar 文件。增强内容不可用时，系统 MUST 回退到 deterministic section body，并继续写出合法 Markdown 页面。

#### Scenario: research-driven section 不会改变 section 身份
- **WHEN** 同一页面在启用 research-driven section 组合的情况下被重复生成，且对应 section 的输入哈希未变化
- **THEN** 页面中的 `section_id` 和 managed marker MUST 保持稳定
- **THEN** 该 section 允许更新正文，但不得更换 section 身份

#### Scenario: 增强内容失效时页面仍写出合法 Markdown
- **WHEN** 某个页面的 research 结果、section 计划或 Mermaid 图未通过校验，或当前运行环境不提供增强结果
- **THEN** 系统 MUST 写出 deterministic 的 section 正文
- **THEN** 页面文件 MUST 仍是包含 managed marker 的正常 Markdown 文档

### Requirement: runtime 必须持久化专题页与 evidence block 的稳定身份
系统 MUST 在现有页面 runtime 中持久化专题页和 evidence block 的稳定身份。专题页 MUST 与现有 `overview / architecture / module / workflow` 页面共用同一套页面状态、managed section 和缓存 contract；evidence block MUST 复用现有 page/section runtime，而不是写入新的 sidecar 层。section-scoped evidence provenance MUST 同时持久化 `source_id`、`line span` 和 `section_refs`。

#### Scenario: 专题页进入正式 runtime
- **WHEN** planner 生成专题页
- **THEN** runtime MUST 为其写入正式页面状态、input hash 和 managed sections
- **THEN** 该页面 MUST 与其他正式页面一样进入 `.wiki/*.md` 和状态库

#### Scenario: evidence block 复用现有 runtime contract
- **WHEN** 页面 section 中存在 evidence block
- **THEN** 这些 block MUST 继续受现有 managed section contract 管理
- **THEN** 系统 MUST 为 evidence provenance 写入 `source_id`、`start_line`、`end_line` 和 `section_refs`
- **THEN** 系统不得为 evidence 单独引入新的正式 runtime 目录

### Requirement: debug trace 模式必须通过显式配置开启且不得污染正式协议
系统 MUST 提供可选的 debug trace 模式，并允许通过启动参数或 steering 配置显式开启。该模式写出的调试信息 MUST 与现有 stdout JSON IPC 正式协议隔离，不能改变 `result/error/progress` 的编码 contract。

#### Scenario: 启动参数开启 debug trace
- **WHEN** 宿主以启动参数显式传入 debug trace 目录或开启标记
- **THEN** 系统 MUST 在指定目录写出调试 trace
- **THEN** stdout 上的正式 JSON 协议 MUST 保持不变

#### Scenario: steering 开启 debug trace
- **WHEN** repo 根配置显式开启 debug trace
- **THEN** `init/update/rebuild` MUST 为本次 workflow 写出调试 trace
- **THEN** debug trace 关闭时系统不得额外写出调试产物

### Requirement: runtime 必须持久化可恢复材料并禁止持久化 provider session
系统 MUST 在现有 runtime/state/cache 主链内持久化 dossier、child rollup、`section_plan` 及其可恢复 identity/hash，而不是新增 `.wiki/` 之外的 sidecar 层。Provider session 只属于单次 `research_page` 调用；`session_id`、`session_summary`、`recent_turns`、`tool_artifact_refs` 和原始对话 MUST NOT 进入 durable state。

#### Scenario: dossier、child rollup 与 section_plan 进入现有 cache/state
- **WHEN** workflow 完成某个页面的 dossier 组装、child rollup 计算或 section 计划生成
- **THEN** 系统 MUST 把对应 identity、input hash 和必要摘要写入现有 runtime/cache 主链
- **THEN** 后续 `update` MUST 能基于这些缓存判断是否需要重建父页

#### Scenario: workflow 恢复后重新建立 provider request
- **WHEN** workflow 从 checkpoint 或 retry 恢复 research unit
- **THEN** provider request MUST 从 `session=None` 开始，并从 durable dossier/context 重建输入
- **THEN** checkpoint、cache、`.wiki/` 和 formal artifact MUST NOT 包含上一调用的 session 或 tool state

### Requirement: LLM cache 生命周期必须与 runtime 清理解耦
系统 MUST 让 LLM cache 生命周期独立于普通 runtime 清理。`init`、`rebuild` 默认不得隐式清空 LLM cache；当用户显式要求 cold-start 或 cache mode 为 `clear`/`refresh` 时，系统才 MAY 清空或失效对应缓存。

#### Scenario: runtime 清理不隐式删除 LLM cache
- **WHEN** 系统为 `init` 或 `rebuild` 清理旧 runtime 产物
- **THEN** 普通模式下 LLM cache MUST 保持可复用
- **THEN** 同仓库重复运行不得因为 runtime 清理而总是冷启动

#### Scenario: cache mode 控制 cache 失效方式
- **WHEN** 当前 workflow 显式设置 `cache_mode = clear` 或 `refresh`
- **THEN** 系统 MUST 按配置清空或强制刷新相应缓存
- **THEN** 失效行为 MUST 能被 trace、progress 或 summary 识别

### Requirement: runtime 必须持久化 unit 页面投影与 parent-child compose 结果
系统 MUST 在现有 runtime/state/cache 主链内持久化 domain/unit 页面投影、child page digest 和 parent compose 输入。这些页面投影 MUST 与其他正式页面共用同一套 `page_id`、managed section、cache 和增量更新 contract，而不是引入新的 sidecar 目录或平行 family identity。

#### Scenario: unit 页面投影进入正式 runtime
- **WHEN** planner 为 domain index 或 child KnowledgeUnit 生成页面投影
- **THEN** runtime MUST 为其写入正式页面状态、缓存和 managed sections
- **THEN** 这些页面 MUST 与 overview、module、topic 一样进入 `.wiki/*.md`

#### Scenario: parent-child compose 结果进入现有 cache/state
- **WHEN** 系统生成 child page digest 或 parent compose 输入
- **THEN** 系统 MUST 将这些对象写入现有 runtime/cache 主链
- **THEN** `update` 与 `rebuild` MUST 能基于这些结果进行增量复用

### Requirement: runtime 必须持久化 pipeline 中断检查点
系统 MUST 在 SQLite state 中维护 `pipeline_checkpoint` 表，用于存储 pipeline 中断时的进度信息。检查点 MUST 包含 `checkpoint_id`、`facts_input_hash`、`interrupted_stage`、`interrupted_target_id`、`error_message` 和 `created_at`。Pipeline 正常完成后 MUST 清除检查点。

#### Scenario: pipeline 中断时写入检查点
- **WHEN** Research 或 Compose 阶段因 LLM 调用失败而中断
- **THEN** 系统 MUST 写入一条 `pipeline_checkpoint` 记录
- **THEN** 记录 MUST 包含当前 Facts 输入的哈希、中断阶段和目标 ID

#### Scenario: pipeline 完成时清除检查点
- **WHEN** pipeline 所有阶段正常完成
- **THEN** 系统 MUST 删除 `pipeline_checkpoint` 表中的所有记录

### Requirement: runtime 必须持久化 parent compose readiness 与 gate 状态
系统 MUST 在正式 runtime/state/cache 主链内持久化 parent compose readiness 与 gate 状态，用于表达某个 unit 当前处于 `research-ready`、`compose-ready`、`assemble-ready` 还是被 soft blocker 卡住。该持久化 contract MUST 至少覆盖 `unit_id`、`unit_type`、当前阶段、缺失依赖、阻塞原因、更新时间和 workflow 级摘要。系统 MUST NOT 再把“research 已存在但 compose/assemble 未完成”的状态仅留给临时内存或通过 cache 缺失间接猜测。

#### Scenario: research 完成但 compose 不满足时仍有正式 gate 状态
- **WHEN** 某个 parent unit 已完成 research，但 child rollup、provider 输出或 compose 输入尚未满足
- **THEN** runtime MUST 持久化该 unit 当前不是 `compose-ready`
- **THEN** runtime MUST 记录该 unit 的阻塞原因与缺失依赖
- **THEN** 系统 MUST NOT 只留下 `research_cache` 而没有任何 gate/readiness 说明

#### Scenario: hard interruption 与 readiness 状态分离持久化
- **WHEN** workflow 因 provider 调用失败、compose 失败或写盘失败而中断
- **THEN** 系统 MUST 继续持久化 hard interruption checkpoint
- **THEN** 系统 MUST 同时保留当前 workflow 或 unit 的 readiness/gate 摘要
- **THEN** 系统 MUST 能区分“发生错误中断”和“尚未达到 compose-ready”这两类状态

### Requirement: parent contract 摘要必须进入可复用 runtime/cache 主链
系统 MUST 将 parent compose 需要的最小 contract 摘要写入现有 runtime/cache 主链，以支持 `update`、`rebuild`、reference 报告与 runtime 诊断复用。对 parent unit 而言，摘要 MUST 至少覆盖 `child_unit_ids`、`child_page_ids`、child digest 引用、citation/diagram 摘要引用、readiness 状态以及当前 parent `UnitResearch` 对应的最小身份线索。系统 MUST NOT 继续只为这类页面写入最小 `source_ids`。

#### Scenario: parent page context 能回溯 child contract 输入
- **WHEN** 系统为某个 parent unit 写入正式 runtime cache
- **THEN** 后续读取方 MUST 能从 cache/state 中回溯该页面对应的 child unit 集合与最小 compose contract 摘要
- **THEN** 报告、诊断和增量更新 MUST 不需要依赖最终 Markdown 反推这些输入

#### Scenario: 高层 parent unit cache 能回溯自身 research 身份
- **WHEN** 系统为 `Overview`、`Architecture`、`DomainIndex` 或 `config_surface` parent unit 写入 runtime/cache
- **THEN** cache/state MUST 能区分“该页面已有自己的 parent `UnitResearch`”与“仅有 system/domain seed”
- **THEN** 系统 MUST NOT 把高层父页继续记成只消费 seed 的页面

### Requirement: runtime 必须只拥有 projection、lifecycle、transport 与 storage adapter
系统 MUST 让 `wiki-runtime` 的正式职责收敛为 workflow orchestration、projection/render、managed section merge、storage adapter、transport、query route 骨架与 lifecycle。`wiki-runtime` MUST NOT 再拥有 scanner、symbol graph、knowledge planning、research engine 或 compose engine 的主实现。

#### Scenario: runtime 只保留 projection 与 lifecycle 主实现
- **WHEN** 开发者检查 `wiki-runtime` crate 的模块边界
- **THEN** runtime MUST 包含 renderer、managed section、page merge、storage adapter、transport、workflow 与 lifecycle 逻辑
- **THEN** runtime MUST NOT 直接承载 facts/index 或 knowledge planning/research/compose 的主实现

### Requirement: runtime 的 SQLite 存储必须按 index、knowledge、runtime 三段分治
系统 MUST 在 `wiki-runtime` 中把 SQLite 具体实现拆成 `index_store`、`knowledge_store` 和 `runtime_store` 三段。`wiki-index` 与 `wiki-knowledge` 必须各自定义自己需要的 store trait，`wiki-runtime` 只实现这些 trait。系统 MUST 明确每张表的 schema contract owner、读写 API owner 与 truth kind，避免跨层直接读库。与此前不同的是，`knowledge_store` 中的 `knowledge_domains`、`knowledge_units`、`research_cache`、`page_digests` 等对象在本轮后 MUST 被视为本地 working cache、加速索引或 `.wiki/.knowledge/**` 的 rebuild target，而不是唯一正式 knowledge truth。正式可共享 truth MUST 由 `.wiki/.knowledge/** + official page tree + wiki.metadata.json` 承载。

#### Scenario: index facts 通过 index_store 持久化
- **WHEN** 系统持久化或读取 `modules`、`symbols`、`edges` 或 graph analysis 相关数据
- **THEN** 这些表 MUST 归 `wiki-index` 合同所有
- **THEN** 读写 API MUST 由 `wiki-index` 定义 trait、由 `wiki-runtime::storage::sqlite::index_store` 实现
- **THEN** 这些数据 MUST 被视为 facts/index formal snapshot

#### Scenario: knowledge cache 通过 knowledge_store 持久化
- **WHEN** 系统持久化或读取 `knowledge_domains`、`knowledge_units`、`research_cache`、`page_digests` 或 `page_drafts`
- **THEN** 这些表 MUST 归 `wiki-knowledge` 合同所有
- **THEN** 读写 API MUST 由 `wiki-knowledge` 定义 trait、由 `wiki-runtime::storage::sqlite::knowledge_store` 实现
- **THEN** 这些数据 MUST 被视为本地 working cache、derived knowledge cache 或 compose artifact，而不是唯一正式 knowledge truth

#### Scenario: formal knowledge artifact 可重建 knowledge_store
- **WHEN** 本地 `knowledge_store` 缺失或被清理，但 `.wiki/.knowledge/**` 仍然可读
- **THEN** 系统 MUST 能基于正式 knowledge artifact 重建对应的本地 `knowledge_store`
- **THEN** 系统 MUST NOT 依赖重新执行 planning、research 或 compose 才能恢复这些本地表

#### Scenario: runtime projection 与 lifecycle 通过 runtime_store 持久化
- **WHEN** 系统持久化或读取 `wiki_pages`、`wiki_page_sections`、`wiki_relations`、`runtime_meta`、`pipeline_checkpoint` 或 `unit_runtime_gates`
- **THEN** 这些表 MUST 归 `wiki-runtime` 合同所有
- **THEN** 系统 MUST 把 `wiki_pages`、`wiki_page_sections` 与关系表视为由 formal artifacts、页面树和 metadata 重建出的本地 projection mirror，不得把 SQLite 当成正式 truth
- **THEN** `pipeline_checkpoint` 与 `unit_runtime_gates` 只表达本地 workflow/lifecycle 状态；持久化 projection recovery anchor 以 formal `ProjectionDigest` 和 committed snapshot manifest 为准
- **THEN** `page_drafts` 只能是 transient compose artifact，`page_digests` 必须作为 formal projection anchor 而不是 SQLite 独占状态

### Requirement: runtime 的外部 query 入口本轮必须保持 `term` 合同稳定
系统 MUST 在本轮继续保留外部 `CoreCommand.term` 与 `run_query(repo_root, term)` 入口，不得提前引入新的外部结构化 query payload。runtime 内部 MAY 构造结构化请求调用 `wiki-index::query`，但该结构只属于 crate 内部边界，不属于本轮正式 transport 合同。

#### Scenario: 非空 `term` 映射为内部 `auto` 请求
- **WHEN** 调用方通过现有 query 入口传入非空 `term`
- **THEN** runtime MUST 将该输入映射为内部 `auto` 查询请求
- **THEN** runtime MUST 优先把该请求路由到 `wiki-index::query`

#### Scenario: 空 `term` 继续返回空结果
- **WHEN** 调用方通过现有 query 入口传入空 `term`
- **THEN** runtime MUST 继续返回空查询结果
- **THEN** 系统 MUST NOT 因此引入新的外部参数或 breaking contract

### Requirement: runtime 必须区分 `index not ready` 与“空命中”
系统 MUST 对外区分“facts snapshot 尚未提交”和“查询执行成功但没有命中”这两类状态。当前者发生时，runtime MUST 返回显式 `index not ready` 错误，而不能返回空命中成功。

#### Scenario: snapshot 未就绪返回显式错误
- **WHEN** 调用方执行 query，但当前仓库尚未完成首次 facts snapshot 提交
- **THEN** runtime MUST 返回显式 `index not ready` 错误
- **THEN** 返回结果 MUST NOT 伪装成正常空命中

#### Scenario: 查询成功但无命中返回空结果
- **WHEN** 调用方执行 query，且 facts snapshot 已就绪，但当前 term 或内部请求没有命中任何对象
- **THEN** runtime MUST 返回成功的空结果
- **THEN** 系统 MUST 不得把这种情况提升为 `index not ready`

### Requirement: runtime 必须把 `.knowledge / official page tree / metadata / cache` 作为正式分层
系统 MUST 将 `.wiki/.knowledge/**`、official page tree、`wiki.metadata.json` 与 `.wiki/.cache/**` 视为不同 truth kind 的正式分层，而不是继续让 SQLite 或 `.cache` 充当隐性主真相。`.wiki/.knowledge/**` MUST 承载可上库的 formal knowledge artifacts，official page tree MUST 承载 page projection 和 authoring surface，`wiki.metadata.json` MUST 承载正式索引与恢复入口，`.wiki/.cache/**` MUST 只承载本地 working state 与可重建缓存。official page tree 只包括 `.wiki/INDEX.md`、`.wiki/<栏目路径>/INDEX.md` 和 `.wiki/<栏目路径>/NN-主题.md`；`.wiki/pages/**` 位于 runtime surface 外。

#### Scenario: runtime 写盘时保持四层职责分离
- **WHEN** 系统执行正式 `init`、`update` 或 `rebuild`
- **THEN** `.wiki/.knowledge/**` MUST 只写入 formal knowledge artifacts 与 recovery anchors
- **THEN** official page tree MUST 只写入 page projection 和合法 authoring surface
- **THEN** `.wiki/pages/**` MUST NOT 作为写入、恢复、query 或 status 目标
- **THEN** `.wiki/.cache/**` MUST 只写入本地 working state
- **THEN** 系统 MUST NOT 把 `.cache` 或 SQLite 继续当成唯一正式 knowledge 真相

### Requirement: runtime 必须支持从正式产物恢复本地 cache 与可消费状态
系统 MUST 支持在 `.wiki/.cache/**` 缺失或需要重建时，从 `.wiki/.knowledge/** + official page tree + wiki.metadata.json` 恢复本地 cache 与 runtime 可消费状态。恢复完成后，`status` MUST 能表达当前仓库是 `ready`、`stale`、`needs_update` 还是 `blocker`；当前 `query` 入口 MUST 能消费恢复后的 runtime，而不是强制要求先执行 full `init`。该恢复成功语义 MUST 只表示“本地 cache 已恢复且 runtime 可被消费与诊断”，MUST NOT 被表述成“完整 wiki runtime 已 ready”。

#### Scenario: cold restore 后 `status` 可直接消费恢复态 runtime
- **WHEN** 系统基于正式产物完成本地 `.wiki/.cache/**` 重建
- **THEN** `status` MUST 能直接读取恢复出的 runtime 状态
- **THEN** 若当前代码与正式产物不一致，`status` MUST 返回 `stale`、`needs_update` 或 blocker，而不是伪装成全新 fresh runtime

#### Scenario: cold restore 后当前 `query` 入口无需 full init
- **WHEN** 系统已基于正式产物恢复出本地 runtime
- **THEN** 当前 `query` 入口 MUST 能消费该恢复态 runtime
- **THEN** 系统 MUST NOT 把“缺失 `.cache`”本身当成必须 full `init` 的理由

### Requirement: runtime 的 `update` 必须以 knowledge-first refresh 为正式主线
系统 MUST 让正式 `update` 遵循 `ChangeSet -> affected knowledge scope -> refresh derived knowledge -> refresh impacted projections` 主线，而不是继续把 page-first dirty rewrite 作为一级语义。facts/index refresh MAY 继续先行，但 research / compose / assemble 的刷新范围 MUST 由 `AffectedKnowledgeScope` 决定。整树 knowledge replan 仍属于 `update` 的一种合法执行路径，不得自动等同于 lifecycle rebuild。

#### Scenario: 局部变更先刷新局部 knowledge scope
- **WHEN** 当前变更可以稳定映射到局部 `AffectedKnowledgeScope`
- **THEN** runtime MUST 只对该 scope 覆盖的 derived knowledge 执行 refresh
- **THEN** runtime MUST 只对由该 scope 派生出的页面投影执行 compose / assemble

#### Scenario: 整树 knowledge replan 仍属于 update
- **WHEN** 当前变更需要执行 `repo_replan` 级别的 knowledge tree 刷新，但 formal artifacts 与 runtime 状态仍保持一致可用
- **THEN** runtime MUST 将这次执行继续视为 `update`
- **THEN** 系统 MUST NOT 仅因整树 replan 就自动切换到 rebuild workflow

### Requirement: `update` 提交必须按 knowledge scope 定向刷新正式产物与 projection anchors
系统 MUST 让 `update` 在提交阶段按 `AffectedKnowledgeScope` 定向刷新 `.wiki/.knowledge/**`、official page tree、`wiki.metadata.json` 与 `.wiki/.cache/**`。未受影响的 formal records、projection anchors 与页面投影 MUST 保持稳定；受影响或已移除的 unit/page 对应记录 MUST 被显式更新或回收。系统 MUST 以统一 snapshot 身份完成本次提交，而不是让 `.knowledge`、official page tree、metadata 与 cache 各自漂移。

#### Scenario: 未受影响 formal records 保持稳定
- **WHEN** 某次 `update` 只命中局部 `AffectedKnowledgeScope`
- **THEN** 不在该 scope 内的 formal knowledge records、projection anchors 与页面投影 MUST 保持稳定身份
- **THEN** 系统 MUST NOT 因为本次局部 refresh 而把未受影响记录一并标记为 touched

#### Scenario: 移除单元时显式回收正式产物
- **WHEN** 某次 `update` 生成了 `removed_unit_ids` 或 `removed_page_ids`
- **THEN** runtime MUST 显式回收对应的 formal records、projection anchors 与页面投影
- **THEN** runtime MUST 同步更新 metadata / recovery 锚点，使最终 snapshot 不再引用已移除对象

### Requirement: runtime 不得继续保留 `v0.1.0 index-only` 公开成功短路
系统 MUST 移除 `SPEC_WIKI_V0_1_INDEX_ONLY`、`index_only` 或等价“facts/index 可用即可成功”的公开短路语义。facts/index 仍可作为 query substrate、恢复基础或诊断依据存在，但 runtime MUST NOT 再把该层单独完成投影为 `init / update` 的正式成功态。

#### Scenario: `init` 与 `update` 不得在 facts snapshot 后提前成功返回
- **WHEN** runtime 已完成 facts/index snapshot，但 knowledge/runtime formal artifacts 尚未形成
- **THEN** `init` 或 `update` MUST 继续返回显式非完成诊断，或继续执行后续主链
- **THEN** 系统 MUST NOT 仅因 facts/index 已可查询就提前返回公开成功

#### Scenario: 对外状态投影不得再生成 `index_only`
- **WHEN** runtime 内部状态处于 `missing`、`runtime_incomplete` 或其它非完成态，且 facts/index 已可用
- **THEN** 对外状态投影 MUST 保持真实诊断态
- **THEN** 系统 MUST NOT 再把该状态改写为 `index_only`

### Requirement: runtime 必须按 formal layer fusion 组装 query routes

系统 MUST 从 facts/index、formal knowledge 和受控 page debug fallback 收集 route-local candidates，并由 Runtime 统一执行 formal layer fusion。各 route MUST 保留自己的 ranking basis、score direction、count、truncation 和 supporting refs；Runtime MUST NOT 把内部 hit DTO 平铺为顶层 transport，也不得把 page fallback 伪装成 facts 或 formal knowledge。

#### Scenario: facts 与 knowledge routes 同时命中

- **WHEN** 同一次 query 同时获得 index facts 和 formal knowledge candidates
- **THEN** Runtime MUST 分别组装对应 route groups 并保留 route-local rank
- **THEN** Runtime MUST NOT 跨 route 直接比较 score 或重建一套全局 matched fields

#### Scenario: page debug fallback 被显式隔离

- **WHEN** formal routes 无可用结果且 Runtime 允许页面调试兜底
- **THEN** 页面候选 MUST 进入 `rendered_page_debug_fallback` route group
- **THEN** readiness、query trust 与 recommended action MUST 继续表达真实 runtime 状态

### Requirement: runtime 的 query 结果必须通过 canonical fields 分离 readiness 与 route provenance
系统 MUST 让 `query` 结果中的 readiness 与 route provenance 分层表达。`readiness`、`query_trust` 与 `recommended_action` 负责回答“当前结果是否可直接消费、是否需要 update/rebuild”；`route_groups` 负责组织各 route 的结果与 supporting refs。`route_groups` MUST 是唯一结果 authority，系统 MUST NOT 恢复已删除的顶层 `provenance_summary` 或用单一字段混合状态与来源语义。

#### Scenario: 恢复态 runtime 可查询但不伪装成 ready
- **WHEN** 当前 runtime 是基于 `.wiki/.knowledge/** + official page tree + metadata` 恢复出的可查询状态，但当前代码与正式 snapshot 不一致
- **THEN** `query` MAY 返回可消费结果
- **THEN** `query_trust` 与 `recommended_action` MUST 提醒调用方该结果处于恢复态或待更新态
- **THEN** `route_groups` MUST 继续只组织实际命中与 refs，不得把恢复态编码为虚假 route

#### Scenario: knowledge 命中可通过 route group 观测
- **WHEN** 某次 query 主要依赖 formal knowledge artifacts 命中，而不是直接 index 命中或页面兜底
- **THEN** 对应结果 MUST 位于 Runtime 定义的 knowledge route group 并携带 supporting refs
- **THEN** 验证与宿主消费 MUST 使用 canonical group identity，不得复制 route enum 或恢复旧 provenance 字段

#### Scenario: page fallback 与 blocker 语义不混层
- **WHEN** 当前 query 命中了 page fallback，且 runtime 同时存在 `needs_update` 或 blocker 诊断
- **THEN** 结果 MUST 同时保留对应 route group 与 Runtime recommended action
- **THEN** 系统 MUST NOT 因为存在页面兜底就把 blocker/readiness 问题隐藏掉

