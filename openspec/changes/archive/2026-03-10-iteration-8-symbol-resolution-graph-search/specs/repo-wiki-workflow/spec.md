## MODIFIED Requirements

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

