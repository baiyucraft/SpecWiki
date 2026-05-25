## 1. SQLite Schema 基座

- [x] 1.1 扩展 `crates/wiki-core/src/storage/sqlite_store.rs`，为关系型状态表、映射表、预建符号/流程表、`llm_cache` 与 FTS5 表建立统一 schema 初始化逻辑
- [x] 1.2 固定 `wiki-cache.db` 的当前 schema 初始化与不完整库回退路径，不要求旧版 DB 自动迁移
- [x] 1.3 为新的 schema 初始化与不完整库回退路径补充 `crates/wiki-core/tests/sqlite_storage.rs` / `crates/wiki-core/tests/sqlite_lifecycle.rs` 级别测试

## 2. WikiState 行式持久化

- [x] 2.1 重构 `crates/wiki-core/src/storage/state_store.rs`，将 `WikiState` 的读写从单条 JSON 改为页面、section、源码、模块、关系与映射表的组装/拆分
- [x] 2.2 扩展 `crates/wiki-core/src/domain/state.rs`，为页面状态补入 `section_anchors` 并保持 section 级状态可由数据库重建
- [x] 2.3 调整 `crates/wiki-core/src/storage/cache_store.rs` 与 runtime 完整性检查逻辑，使其基于表/行存在性而不是 `kv_store` key
- [x] 2.4 为行式 `WikiState` 装配、缺失回退与 section anchor 持久化补充单元/集成测试

## 3. Scanner FilePurpose 与 Steering 扫描边界

- [x] 3.1 在 `crates/wiki-core/src/repo/scanner.rs` 引入 `FilePurpose` 分类及 deterministic 路径/文件名规则，替换当前粗粒度 `kind`
- [x] 3.2 更新 hierarchy、planner、context 等下游模块对文件角色的消费方式，确保高信号/低信号文件的权重与摘要行为符合新分类
- [x] 3.3 重构 `crates/wiki-core/src/domain/steering.rs` 与扫描入口，支持 `scan.ignore` / `scan.include` 新结构，并兼容旧配置的读取迁移
- [x] 3.4 为 `FilePurpose` 分类、扫描 include/ignore 覆盖行为和旧 steering 配置兼容路径补充测试

## 4. Query 与 FTS5 检索

- [x] 4.1 建立 `wiki_pages_fts` 与 `symbols_fts` 的初始化与更新逻辑，确保页面写入时同步刷新 FTS 索引
- [x] 4.2 重构 `crates/wiki-core/src/workflows/query.rs`，接入基于 SQLite FTS5 的 BM25 页面检索，并与现有结构化结果合并
- [x] 4.3 为页面标题/路径命中、BM25 与结构化命中合并、FTS 空索引回退补充 query 测试

## 5. Workflow 集成与增量一致性

- [x] 5.1 更新 `init`、`update`、`sync`、`rebuild` 工作流，使其统一写入关系型状态表、section anchors、扫描缓存和 FTS 索引
- [x] 5.2 更新 `change_set` / `status` 的 runtime 缺失检查与受影响页面推导逻辑，消费新的状态表、section 行和页面锚点
- [x] 5.3 校准旧数据库损坏与不完整 schema 的 `needs_rebuild` 回退路径，避免出现半成品 runtime

## 6. 注释与验证

- [x] 6.1 按 `COMMENTING.md` 检查本轮修改涉及的 Rust/TypeScript 注释，补齐或收敛不符合规范的注释
- [x] 6.2 运行 `cargo test -p wiki-core`（或等效工作区测试命令），确认 SQLite、scanner、query、workflow 相关测试全部通过
- [x] 6.3 运行 `node scripts/run-test-projects.mjs` 对 19 个测试项目批量执行 `init`，对照 `tmp/reference/*` 检查 `.wiki/*.md` 与 `wiki.metadata.json`，并产出/更新本 change 的 `test-project-analysis.md`
- [x] 6.4 运行 `node scripts/test-wiki-lifecycle.mjs` 验证 `init → status → sync → query → update → rebuild` 全链路 JSON 响应、marker 覆盖率与状态流转
