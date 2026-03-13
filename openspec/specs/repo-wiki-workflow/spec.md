# repo-wiki-workflow Specification

## Purpose
定义 Repo Wiki 核心 workflow 在 `init`、`update` 与 `rebuild` 场景下的执行边界、可观测性与状态收口要求。
## Requirements
### Requirement: `init` 必须为有效本地代码目录建立第一阶段 Repo Wiki
系统 MUST 在有效本地代码目录上执行完整初始化，生成第一阶段所需的 Wiki 页面、metadata、关系型状态表、symbol snapshot、symbol graph 和增量缓存，并在页面规划阶段基于递归模块树生成层级化页面。Git 信息在存在时可作为元数据补充，但不得成为初始化前提。初始化主链 MUST 按 `scan -> parse_symbols -> resolve_symbol_graph -> analyze_symbol_graph -> module_tree -> page_planner -> render` 的顺序执行，而不是绕过新增的关系解析和图分析阶段。init 完成后 MUST 把 definitions 写入 `symbols`，把 `IMPORTS / CALLS / EXTENDS / IMPLEMENTS` 写入 `edges`，把 community/process 结果写入对应图分析表，再装配 `WikiState` 并导出 `wiki.metadata.json`。为了支撑 editable runtime 和后续检索，init 还 MUST 初始化 page context cache、page generation cache、section 状态、managed section marker、页面级 `section_anchors` 和 `wiki_pages_fts`，而不是只落盘 plain Markdown。

#### Scenario: 初始化成功
- **WHEN** 用户在有效本地代码目录上执行 `init`
- **THEN** 系统 MUST 生成 `.wiki/` 运行产物
- **THEN** 系统 MUST 依次执行 `parse_symbols`、`resolve_symbol_graph` 和 `analyze_symbol_graph`
- **THEN** 系统 MUST 把 definitions 写入 `.wiki/.cache/wiki-cache.db` 的 `symbols` 与 `symbols_fts`
- **THEN** 系统 MUST 把解析后的符号关系写入 `edges`，并把 community/process 结果写入对应图分析表
- **THEN** 系统 MUST 装配 `WikiState` 并写入关系型状态表，然后导出 `wiki.metadata.json`

#### Scenario: 初始化时遇到单文件符号或关系解析失败
- **WHEN** `init` 的 `parse_symbols` 或 `resolve_symbol_graph` 阶段遇到某个源码文件无法成功解析
- **THEN** 系统 MUST 继续完成图分析、模块树、页面渲染、状态写盘和 metadata 导出
- **THEN** 该文件不得在最终 `symbols` 或 `edges` 表中保留陈旧 rows

#### Scenario: 初始化时读取 steering 配置
- **WHEN** 用户在有效本地代码目录上执行 `init`，且 `.wiki/wiki.steering.yaml` 存在
- **THEN** 系统 MUST 读取 steering 配置并传递给 scanner、module tree builder 与 planner
- **THEN** steering 配置中的扫描忽略/包含规则 MUST 在扫描阶段生效
- **THEN** steering 配置中的模块提升/降级与页面阈值配置 MUST 在页面规划阶段生效

#### Scenario: 在非 Git 目录中初始化
- **WHEN** 用户在不包含 `.git` 的本地代码目录上执行 `init`
- **THEN** 系统 MUST 仍然完成页面、symbol snapshot、symbol graph、SQLite 状态表、metadata 和缓存的生成
- **THEN** 系统不得因为缺失 Git 元信息而拒绝初始化

### Requirement: `update` 必须将过期 Runtime 刷新到 fresh
系统 MUST 在发现 Runtime 为 `stale` 时基于 `ChangeSet` 和 `AffectedSet` 执行增量刷新，而不是无条件重跑 `init`。局部可修复时，`update` MUST 只重建受影响页面及其 page context / generation cache、页面状态行、section 状态行和 FTS 索引，并保持未受影响页面不重写。对源码变化，`update` 还 MUST 只重解析新增、修改和删除的受影响源码文件对应的 symbols 与 edges；当 graph 发生变化时，`update` MUST 重新生成 communities、processes 和 cycle/topology 派生结果。对于仍然存在的同一 `page_id` 页面，`update` MUST 只替换 managed sections，并保留已同步的 user sections 与稳定锚点。Runtime 为 `missing` 时，`update` MUST 以等价于 `init` 的方式恢复运行时；Runtime 为 `needs_rebuild` 时，`update` MUST 走 full rebuild 路径。

#### Scenario: 过期后局部更新
- **WHEN** `status` 为 `stale` 且变化只影响已有页面集合
- **THEN** 系统 MUST 只重建受影响页面
- **THEN** 系统 MUST 只刷新这些页面对应的状态行、section 行、页面缓存和 FTS 记录
- **THEN** 系统 MUST 只重解析受影响源码文件及其一跳 graph dependents 的 symbol/edge rows
- **THEN** 如果 graph 结果发生变化，系统 MUST 重算 communities、processes 和 cycle/topology 派生结果
- **THEN** 系统 MUST 保持未受影响页面不重写并保留同页 user sections

#### Scenario: update 删除源码后清理符号与关系
- **WHEN** 某个已存在源码文件在仓库中被删除并触发 `update`
- **THEN** 系统 MUST 删除该文件对应的 symbol rows、edge rows 和 `symbols_fts` 记录
- **THEN** 后续 query 不得继续命中这些已删除 definitions 或关系

#### Scenario: update 消费 steering 配置
- **WHEN** `update` 在增量路径中执行扫描、图解析与页面规划
- **THEN** 系统 MUST 读取 steering 配置并传递给 scanner、module tree builder 与 planner
- **THEN** steering 配置中的扫描忽略/包含、合并阈值和模块提升/降级 MUST 影响页面规划结果

### Requirement: `rebuild` 必须强制全量重建并保留同页 user sections
系统 MUST 在 `rebuild` 时忽略旧 generation cache 和旧 dirty state，但对仍然存在的同一 `page_id` 页面继续复用已同步的 user sections。`rebuild` MUST 删除旧数据库后重新创建全量状态表、symbol 表、graph 表、缓存表和 FTS 索引，并读取 steering 配置参与新一轮扫描、符号解析、符号关系解析、图分析与页面规划。

#### Scenario: rebuild 消费 steering 配置
- **WHEN** 用户执行 `rebuild`
- **THEN** 系统 MUST 读取 steering 配置并传递给 scanner、module tree builder 与 planner
- **THEN** rebuild 后的页面拓扑、扫描结果和 workflow 页面 MUST 反映 steering 配置的影响
- **THEN** 新数据库 MUST 包含完整的状态表、symbol 表、graph 表、缓存表和 FTS 索引

#### Scenario: rebuild 全量重建 symbol graph
- **WHEN** 用户执行 `rebuild`
- **THEN** 系统 MUST 对全部受支持源码重新执行 `parse_symbols`、`resolve_symbol_graph` 和 `analyze_symbol_graph`
- **THEN** 系统 MUST 重建 `symbols`、`edges`、`communities`、`processes` 及其派生表，而不是复用旧 graph rows

### Requirement: `init`、`update` 与 `rebuild` 必须输出稳定的阶段进度
系统 MUST 让 `init`、`update` 和 `rebuild` 在真实 workflow 阶段边界上输出稳定 progress 事件，而不是只输出最终结果。阶段划分 MUST 基于当前主链中的实际步骤，例如扫描、symbol parsing、graph resolution、graph analysis、module tree、context、page planning、render、state write 与 metadata write。

#### Scenario: init 输出主链阶段进度
- **WHEN** 调用方执行 `init`
- **THEN** 系统 MUST 至少为扫描、符号解析、页面渲染和状态写盘这些阶段输出 progress 事件
- **THEN** 这些事件的阶段顺序 MUST 与实际 `init` 主链执行顺序一致

#### Scenario: update 回退时仍输出当前实际路径的阶段进度
- **WHEN** `update` 因 `missing` 或 `needs_rebuild` 回退到 `init` 或 `rebuild`
- **THEN** 系统 MUST 继续输出回退后实际执行路径对应的 progress 事件
- **THEN** 调用方 MUST 能从事件流中区分“增量 update”与“回退到 init/rebuild”的真实执行情况

### Requirement: `update` 必须优先使用局部 symbol/edge 工作集刷新
当 `update` 只涉及有限数量的 `graph_refresh_sources` 时，系统 MUST 优先按受影响文件及其必要 graph frontier 读取 symbol/edge 工作集，并与本轮 changed snapshot 合并，而不是对任意小变更都无条件回读全量 `symbols / edges`。当受影响范围、frontier 膨胀或状态缺失超出局部刷新可控范围时，系统 MUST 显式回退到全量读取或更高等级的 fallback 路径，而不是静默退化为固定的全量路径。

#### Scenario: 小范围图变化优先走局部工作集
- **WHEN** `update` 只涉及少量受影响源码文件，且 graph frontier 仍处于可控范围
- **THEN** 系统 MUST 优先读取这些文件及必要 dependents 对应的 symbol/edge rows
- **THEN** 系统 MUST 不得对这类小变更固定执行全量 `symbols / edges` 回读

#### Scenario: 局部工作集超阈值时显式回退
- **WHEN** `update` 的受影响文件集合、graph frontier 或关键状态缺失超出局部刷新阈值
- **THEN** 系统 MUST 显式回退到全量读取或更高等级 fallback 路径
- **THEN** 该回退 MUST 能被 progress 事件、诊断或等价可观测方式识别

### Requirement: `init`、`update` 与 `rebuild` 必须支持可回退的 LLM 增强阶段
系统 MUST 在保持现有 deterministic facts 主链的前提下，为 `init`、`update` 和 `rebuild` 增加可选的 `llm_uncertainty_gate` 与 `llm_enrichment` 阶段。`llm_uncertainty_gate` MUST 发生在 scanner / hierarchy / 低置信度依赖语义判定期间；`llm_enrichment` MUST 发生在 page context 已稳定、正式写盘之前。无论任一阶段是否启用、命中缓存或回退，workflow 的最终写盘结果都 MUST 保持可追溯且可落回 deterministic 内容。

#### Scenario: init 在 facts 稳定后执行 research-driven 页面增强
- **WHEN** 用户执行 `init`，且当前运行环境已协商开启 LLM 增强
- **THEN** 系统 MUST 先完成 deterministic 的扫描、symbol graph、module tree、page planning 和 page context 构建
- **THEN** 系统 MUST 在正式渲染和写盘前对 `overview`、`architecture`、`module` 和 `topic` 页执行 research-driven `llm_enrichment`
- **THEN** 若增强阶段失败，系统 MUST 回退到 deterministic 页面内容继续完成 `init`

#### Scenario: rebuild 在 research 不可用时保持 deterministic
- **WHEN** 用户执行 `rebuild`，但当前 provider 不可用、预算关闭或 research contract 校验失败
- **THEN** 系统 MUST 跳过 research-driven 结果并走完整 deterministic 路径
- **THEN** rebuild 的最终状态与现有 deterministic 语义保持一致

### Requirement: workflow 主链必须在 page planner 中支持专题页
系统 MUST 在 `build_contexts -> plan_pages -> render_pages` 主链中支持专题页，而不是绕过现有 planner 直接拼装额外页面。专题页的父子关系、受影响集合和增量更新语义 MUST 与现有页面主链保持一致。

#### Scenario: init 在正式 planner 中生成专题页
- **WHEN** 用户执行 `init`，且当前仓库存在稳定专题候选
- **THEN** 系统 MUST 在 `plan_pages` 阶段生成专题页计划
- **THEN** 后续 `render_pages` MUST 把这些页面与其他正式页面一起渲染

#### Scenario: update 只重建受影响专题页
- **WHEN** 用户执行 `update`，且变化只影响部分专题候选或其 evidence 输入
- **THEN** 系统 MUST 只重建对应专题页及其受影响父页
- **THEN** 未受影响的专题页不得被无谓重写

### Requirement: 页面渲染必须支持 facts-driven 图表达
系统 MUST 在正式页面渲染阶段支持 facts-driven 图表达。图输入 MUST 由现有 `ModuleTree`、cross-module edges、父子层级和 detected processes 等稳定事实构造，而不是要求 LLM 自由生成结构。系统 MUST 至少支持模块依赖图、父子结构图和流程图三类受控图表达。

#### Scenario: 模块关系图来自稳定 cross-module edges
- **WHEN** 某个页面包含明确的跨模块依赖事实
- **THEN** renderer MUST 能基于这些事实构造受控 Mermaid 图
- **THEN** 图结构不得依赖 LLM 凭空补完

#### Scenario: 流程图来自 detected processes
- **WHEN** 某个专题页或 workflow 页消费稳定 detected process
- **THEN** renderer MUST 能基于该流程生成受控流程图
- **THEN** 当流程事实不足时，系统 MUST 回退到无图或纯文本说明

### Requirement: workflow 主链必须在正式渲染前组装 dossier 并按需执行 bounded research session
系统 MUST 在保持 deterministic 主链的前提下，在正式渲染前组装 dossier，并按页面类型决定是否执行 bounded research session。research session 的输入 MUST 来自 dossier、targeted snippets、child rollup 和显式 session state，而不是绕过主链重新扫描仓库。9.3 要求 `overview`、`architecture`、`module` 和 `topic` 页都可进入 bounded research session；`workflow` 页继续保持 deterministic + facts-driven diagram。

#### Scenario: overview、architecture、module 或 topic 页在 render 前执行 research session
- **WHEN** 用户执行 `init`、`update` 或 `rebuild`，且当前页面为 `overview`、`architecture`、`module` 或 `topic`
- **THEN** 系统 MUST 先完成 dossier 组装
- **THEN** 若 research session 开启，系统 MUST 在 render 前执行该 session 并消费结构化结果

#### Scenario: update 只重建受影响 dossier、section plan 与父页 rollup
- **WHEN** 变化范围只影响部分 dossier、child rollup、section plan 或 session 结果
- **THEN** 系统 MUST 只重建这些页面及其受影响父页
- **THEN** 未受影响页面不得因为 dossier/session 引入而被无谓重写

### Requirement: uncertainty gate 必须按同类型批量和有限并行执行
系统 MUST 让 `uncertainty_gate` 优先按同类型候选进行批量判断，并允许在阶段内有限并行。不同阶段、不同 schema 的候选不得被揉成一个跨阶段 mega prompt。

#### Scenario: file_purpose 批量判断
- **WHEN** 同一阶段内存在多条 `file_purpose` 候选
- **THEN** 系统 MUST 优先按批次请求这些候选
- **THEN** 系统不得默认逐条串行请求每个文件角色判断

#### Scenario: 不同 schema 的 gate 保持分阶段
- **WHEN** workflow 同时存在 `file_purpose`、`top_level_promotion`、`dependency_edge` 等不同类型 gate
- **THEN** 系统 MUST 允许它们分别批量
- **THEN** 系统不得把不同 schema 混成一个总 prompt

### Requirement: workflow 主链必须在 render 前引入 family planning、leaf-first research 与 compose
系统 MUST 在 `build_contexts -> plan_pages -> render_pages` 主链中加入 family planning、leaf-first research 与 compose 阶段。正式顺序 MUST 至少体现：先规划 family/module/topic 页面集合，再为叶子页组装 dossier 并执行 research，随后让父页消费子页结果完成 compose，最后再由 deterministic renderer 落盘。

#### Scenario: 叶子页先研究、父页后组合
- **WHEN** workflow 处理同时包含 family、module 和 topic 的页面树
- **THEN** 叶子 family child、叶子模块页和高置信 topic 页 MUST 先完成 research
- **THEN** 父页 MUST 在子页结果可用后再执行 compose

#### Scenario: workflow 不得绕过 planner 直接拼 family 页面
- **WHEN** 系统为 docs-heavy 或 platform 仓库生成 family 页面
- **THEN** 这些页面 MUST 由正式 planner 规划出来
- **THEN** render 阶段不得绕过 planner 临时创建额外页面

### Requirement: workflow 必须在 LLM 失败时中断并保存检查点
系统 MUST 在 Research 或 Compose 阶段的 LLM 调用失败时立即中断 pipeline，保存已完成的中间结果和中断位置到 `pipeline_checkpoint` 表。系统 MUST NOT 静默跳过失败的 LLM 调用或退化到模板填充。

#### Scenario: LLM 调用失败触发 pipeline 中断
- **WHEN** `research_system` / `research_domain` / `research_unit` 或 compose 阶段的 LLM 调用失败
- **THEN** workflow MUST 立即中断，不继续处理后续知识单元
- **THEN** 已完成的 research / compose 结果 MUST 已被写入缓存
- **THEN** `pipeline_checkpoint` MUST 被写入 SQLite，包含 `facts_input_hash`、中断阶段和目标 ID

### Requirement: workflow 必须在入口处检查并恢复检查点
系统 MUST 在 init / rebuild / update 的入口处检查 `pipeline_checkpoint` 表。若检查点有效（`facts_input_hash` 匹配），MUST 从中断处恢复而非从头开始。Pipeline 正常完成后 MUST 清除检查点记录。

#### Scenario: 从中断处恢复 pipeline
- **WHEN** workflow 入口检测到有效的 `pipeline_checkpoint` 且 `facts_input_hash` 匹配当前 Facts
- **THEN** pipeline MUST 跳过已缓存的 research / compose 步骤
- **THEN** pipeline MUST 从 `interrupted_stage` + `interrupted_target_id` 指定的位置继续执行

#### Scenario: Facts 变化导致检查点失效
- **WHEN** workflow 入口检测到 `pipeline_checkpoint` 但 `facts_input_hash` 不匹配
- **THEN** 系统 MUST 丢弃检查点记录
- **THEN** pipeline MUST 从头开始完整执行

