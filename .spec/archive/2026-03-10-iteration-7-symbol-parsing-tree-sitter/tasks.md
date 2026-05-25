## 1. Symbol Parser 基座

- [x] 1.1 先升级 `crates/wiki-core/Cargo.toml` 中现有的 tree-sitter 栈到统一版本族，再补齐迭代 7 需要的 grammar 依赖，并把现有 JavaScript / TypeScript / Python parser 调用迁移到新 API
- [x] 1.2 在 `crates/wiki-core/src/repo/` 下新增独立的 `symbols/` 子模块，定义 `SymbolNode`、原始 capture 模型、`ParsedFileSymbols`、`SymbolTable` 和 parser registry
- [x] 1.3 为 symbol parsing 设计稳定的 symbol seed / `stable_id("symbol", ...)` 规则，并补充基础单元测试，覆盖同文件重名、跨文件同名和导出可见性字段

## 2. 多语言解析与批处理

- [x] 2.1 按 `DESIGN-ITER.md` 的 12 种核心语言实现 tree-sitter parser / query 定义，至少让 definition capture 可稳定产出符号节点，并补 query registry coverage 单测，确保每种 supported language 都有对应 query
- [x] 2.2 先收口到新版 grammar 直接覆盖的语言与 React(JSX/TSX) 委托解析；Vue/Svelte symbol 包装层延期到迭代 8，并对当前未覆盖场景保持 fail-soft
- [x] 2.3 实现 symbol parsing 的字节预算批处理、稳定排序和单文件失败隔离策略，并补解析失败与超预算场景测试

## 3. SQLite 与 Workflow 集成

- [x] 3.1 扩展 `crates/wiki-core/src/storage/sqlite_store.rs`，实现 `symbols` / `symbols_fts` 的写入、按文件替换、删除和 BM25 查询接口
- [x] 3.2 修改 `crates/wiki-core/src/workflows/init.rs` 与 `crates/wiki-core/src/workflows/rebuild.rs`，把主链升级为 `scan -> parse_symbols -> module_tree -> planner -> render`，并在同轮 workflow 中写入 symbol snapshot
- [x] 3.3 修改 `crates/wiki-core/src/workflows/update.rs`，基于受影响源码文件增量重解析符号，处理新增、修改、删除和解析失败四类刷新路径

## 4. Query Contract 升级

- [x] 4.1 扩展 `crates/wiki-core/src/workflows/query.rs` 的结构化返回，新增 `matched_symbols` 与 symbol 相关 `context_pack` 字段，同时保持现有页面/模块/源码/关系结果兼容
- [x] 4.2 在 `query` workflow 中同时接入 `wiki_pages_fts` 与 `symbols_fts`，实现 symbol 命中到 source/page/module 的上下文回填和 provenance 合并
- [x] 4.3 为页面 BM25、symbol BM25、结构化命中合并、FTS 空索引回退和删除源码后的 symbol 清理补充 query 级测试

## 5. 端到端验证与项目集分析

- [x] 5.1 在 `crates/wiki-core/tests/` 中补齐 symbol parsing、导出判断、增量重解析、解析失败隔离和 lifecycle 集成测试
- [x] 5.2 运行 `cargo test -p wiki-core` 与根级 `pnpm test`，修复本迭代引入的回归
- [x] 5.3 运行 `node scripts/run-test-projects.mjs` 对 `DESIGN.md § 测试项目集` 的完整项目集执行 `init`，重点检查多语言 symbol 提取、symbol query 命中和既有页面拓扑不变量，并输出本 change 的 `test-project-analysis.md`；报告格式参考 [archive/2026-03-09-iteration-5-editable-wiki-runtime/test-project-analysis.md](E:/project/!byAI/spec-wiki/.spec/archive/2026-03-09-iteration-5-editable-wiki-runtime/test-project-analysis.md) 与 [archive/2026-03-09-iteration-6-page-topology-and-steering/test-project-analysis.md](E:/project/!byAI/spec-wiki/.spec/archive/2026-03-09-iteration-6-page-topology-and-steering/test-project-analysis.md)，对每个项目逐个分析、逐个输出，不得只给总表
- [x] 5.4 使用拆分后的生命周期脚本（例如 `node scripts/test-wiki-lifecycle.mjs --phase bootstrap|steady|mutation|rebuild` 或对应薄包装脚本）分阶段验证 `init -> status -> sync -> query -> update -> rebuild` 全链路，确认 symbol snapshot 与 query contract 在生命周期内保持一致（已执行全量 `bootstrap=147/147`、`steady=249/249`、`mutation=228/228`、`rebuild=257/257`）

## 6. 注释与收口

- [x] 6.1 按 `COMMENTING.md` 复核本轮新增/修改的 Rust 与 TypeScript 注释，重点覆盖 parser registry、SQLite symbol 存储、workflow 接入和 query 返回结构
- [x] 6.2 根据测试项目集与生命周期验证结果回调本 change 的 design / specs / tasks，确保最终提案与真实实现边界一致
