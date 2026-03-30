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

### Requirement: runtime 必须持久化 dossier、child rollup 与 session 摘要缓存
系统 MUST 在现有 runtime/state/cache 主链内持久化 dossier、child rollup、`section_plan` 和显式 research session state，而不是新增 `.wiki/` 之外的 sidecar 层。相关 identity/hash MUST 可被 `update`、`rebuild` 和 cache 命中逻辑复用。显式 session state MUST 至少包含 `session_id`、`session_summary`、`recent_turns` 和 `tool_artifact_refs`。

#### Scenario: dossier、child rollup 与 section_plan 进入现有 cache/state
- **WHEN** workflow 完成某个页面的 dossier 组装、child rollup 计算或 section 计划生成
- **THEN** 系统 MUST 把对应 identity、input hash 和必要摘要写入现有 runtime/cache 主链
- **THEN** 后续 `update` MUST 能基于这些缓存判断是否需要重建父页

#### Scenario: 显式 session state 复用现有 runtime contract
- **WHEN** research session 生成 `session_id`、`session_summary`、`recent_turns` 或 `tool_artifact_refs`
- **THEN** 系统 MUST 在现有 cache/state contract 内持久化这些可复用状态
- **THEN** 系统不得为此新增独立正式 runtime 目录

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

### Requirement: runtime 必须持久化 family 页面与 parent-child compose 结果
系统 MUST 在现有 runtime/state/cache 主链内持久化 family index、family child、child page digest 和 parent compose 输入。family 页 MUST 与其他正式页面共用同一套 `page_id`、managed section、cache 和增量更新 contract，而不是引入新的 sidecar 目录。

#### Scenario: family 页面进入正式 runtime
- **WHEN** planner 生成 family index 或 family child 页面
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
系统 MUST 在 `wiki-runtime` 中把 SQLite 具体实现拆成 `index_store`、`knowledge_store` 和 `runtime_store` 三段。`wiki-index` 与 `wiki-knowledge` 必须各自定义自己需要的 store trait，`wiki-runtime` 只实现这些 trait。系统 MUST 明确每张表的 schema contract owner、读写 API owner 与 truth kind，避免跨层直接读库。

#### Scenario: index facts 通过 index_store 持久化
- **WHEN** 系统持久化或读取 `modules`、`symbols`、`edges` 或 graph analysis 相关数据
- **THEN** 这些表 MUST 归 `wiki-index` 合同所有
- **THEN** 读写 API MUST 由 `wiki-index` 定义 trait、由 `wiki-runtime::storage::sqlite::index_store` 实现
- **THEN** 这些数据 MUST 被视为 facts/index formal snapshot

#### Scenario: knowledge cache 通过 knowledge_store 持久化
- **WHEN** 系统持久化或读取 `research_cache`、`page_digests` 或 `page_drafts`
- **THEN** 这些表 MUST 归 `wiki-knowledge` 合同所有
- **THEN** 读写 API MUST 由 `wiki-knowledge` 定义 trait、由 `wiki-runtime::storage::sqlite::knowledge_store` 实现
- **THEN** 这些数据 MUST 被视为 derived knowledge cache 或 compose artifact，而不是 runtime 真相

#### Scenario: runtime projection 与 lifecycle 通过 runtime_store 持久化
- **WHEN** 系统持久化或读取 `wiki_pages`、`wiki_page_sections`、`wiki_relations`、`runtime_meta`、`pipeline_checkpoint` 或 `unit_runtime_gates`
- **THEN** 这些表 MUST 归 `wiki-runtime` 合同所有
- **THEN** 系统 MUST 把 `wiki_pages` 视为 runtime projection truth，把 `pipeline_checkpoint` 与 `unit_runtime_gates` 视为 lifecycle truth
- **THEN** 系统 MUST NOT 让 `page_drafts` 或 `page_digests` 充当 runtime 正式状态

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

### Requirement: runtime 的 query route 必须以 index 投影为主、page fallback 为辅
系统 MUST 让 `run_query` 的正式结果优先来自 `wiki-index::query` 的投影，而不是继续由 runtime 自己重新实现 facts 查询语义。`matched_symbols`、`matched_sources`、`matched_modules`、`matched_symbol_edges` 等字段 MUST 以 index 投影为主，`matches` 中的页面结果只可作为 fallback 或补充 provenance。

#### Scenario: 存在 index 命中时优先返回 index 投影
- **WHEN** 某次 query 在 `wiki-index::query` 中命中了 symbol、source、module、entrypoint 或 graph 结果
- **THEN** runtime 返回中的 `matched_symbols`、`matched_sources`、`matched_modules`、`matched_symbol_edges` MUST 来自 index 投影
- **THEN** runtime MUST NOT 再通过 `WikiState` 自行重算一套等价 facts 结果

#### Scenario: page fallback 必须显式标注 provenance
- **WHEN** 某次 query 没有有效 index 命中，只能通过 page fallback 命中页面内容
- **THEN** runtime MUST 在结果中显式标注 page fallback provenance
- **THEN** 系统 MUST NOT 把页面命中伪装成 facts/index 命中

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
