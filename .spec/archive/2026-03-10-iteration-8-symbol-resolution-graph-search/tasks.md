## 1. Parser 与图事实基础

- [x] 1.1 扩展 `repo::symbols` 的 raw capture 模型，补齐 import/call/heritage 所需的行号、owner/source symbol 与文本元信息
- [x] 1.2 为 Vue / Svelte 单文件组件实现 `<script>` / `<script setup>` 抽取、底层 JS/TS 委托解析和原始行号映射
- [x] 1.3 新增独立的 `repo::symbol_graph` 子层与核心模型，区分 parsed symbols、resolved edges、communities、processes 和 cycle summary
- [x] 1.4 在 `storage/sqlite_store.rs` 中补齐 graph 表的真实读写接口，并保证 `symbols / edges / communities / processes` 能在同一事务中刷新

## 2. Symbol Resolution

- [x] 2.1 实现 `ImportResolutionContext`、`SuffixIndex`、resolve cache 以及 TS/Rust/Java/Kotlin/Go/PHP/Swift 的导入解析规则
- [x] 2.2 实现 `resolve_imports`，把 raw import captures 解析为稳定 `IMPORTS` edges，并补充 diagnostics / unresolved 处理
- [x] 2.3 实现带 built-in/noise 过滤与分层置信度的 `resolve_calls`，生成 `CALLS` edges
- [x] 2.4 实现 `resolve_heritage`，生成 `EXTENDS / IMPLEMENTS` edges
- [x] 2.5 把 `init / rebuild / update` 接到新的 symbol resolution 阶段，并支持按文件替换/删除陈旧 edge rows
- [x] 2.6 扩展 `ChangeSet` / `AffectedSet`，让 update 覆盖 dirty files 与一跳 graph dependents 的 edge refresh

## 3. Graph Analysis 与页面/查询集成

- [x] 3.1 实现基于 resolved symbol graph 的 community 检测，并写入 `communities / community_members`
- [x] 3.1a 将 community 检测从 fallback-only 连通分量升级为首选聚类算法 + deterministic fallback，并在大图场景启用降噪
- [x] 3.2 实现入口点评分、BFS trace、子集/端点去重的 process 检测，并写入 `processes / process_steps`
- [x] 3.3 实现 Tarjan SCC、断边/降级与拓扑提示生成，形成可供 planner/query 消费的 cycle summary
- [x] 3.4 定义并接入 `GraphSummary`，让 `build_module_tree`、`build_repo_context`、`build_module_contexts` 与 planner 消费 symbol graph 聚合信号
- [x] 3.5 升级 workflow 页面规划与渲染，使 detected processes 能驱动 workflow 页面内容，而不仅依赖 CI/CD 文件线索
- [x] 3.6 升级 `run_query()`，在保留页面 / symbol BM25 的基础上回填 graph relations、processes、communities 和 provenance
- [x] 3.6a 为 `run_query()` 补齐基于 `edges` 的多跳调用链 / 影响范围扩展，使用 `WITH RECURSIVE` CTE 或等价实现输出有限深度 graph query 结果

## 4. 测试、项目集与注释检查

- [x] 4.1 为 Vue/Svelte wrapper、import resolution、call/heritage resolution、SQLite graph persistence、cycle/process/community detection 补 Rust 单元/集成测试
- [x] 4.1a 为首选 community 检测与 deterministic fallback 补 Rust 测试，覆盖大图降噪和 fallback 口径
- [x] 4.2 为 `query`、`update`、`rebuild` 的 graph lifecycle 补 workflow 级 Rust 测试，覆盖 edge refresh、graph fallback 与 graph query 命中
- [x] 4.2a 为多跳 graph query 补 workflow 级 Rust 测试，覆盖调用链与影响范围扩展、graph provenance 和空图回退
- [x] 4.3 扩展 `scripts/test-wiki-lifecycle*.mjs` 与相关整体测试，验证 graph tables、graph query 和大型仓库超时策略
- [x] 4.4 运行 `node scripts/run-test-projects.mjs` 对完整 19 个测试项目执行 `init` 分析，并输出本轮 `test-project-analysis.md`
- [x] 4.5 运行 `node scripts/test-wiki-lifecycle.mjs` 完成 init → status → sync → query → update → rebuild 全链路验证
- [x] 4.6 按 `.wiki/02-开发指南/00-代码注释规范.md` 检查本轮新增或修改代码的注释粒度与格式，必要时补充或收敛注释
