## Why

迭代 6 已经把 runtime 从散落 JSON 推进到单库 SQLite，但当前实现仍然把 `WikiState`、`ScanReport` 和 `ModuleTree` 作为 JSON 大对象写入 `kv_store`，`query` 也仍停留在内存结构匹配，无法为后续符号表、关系图和全文检索提供稳定存储基础。现在推进迭代 6.5，是为了在进入 tree-sitter 符号解析前，先把存储 schema、文件角色分类和页面检索面补齐到 `.wiki/06-设计文档/00-总体设计.md` 定义的边界。

## What Changes

- 把 `.wiki/.cache/wiki-cache.db` 从“`kv_store` + per-page cache”升级为可扩展的关系型 schema，正式落地 `wiki_pages`、`source_states`、`modules` 及其映射表，并预建 `symbols`、`edges`、`communities`、`processes`、`llm_cache` 等后续迭代表结构。
- 保留扫描缓存与生成缓存，但把 runtime 完整性检查、状态读写和增量缺失检测从“查 JSON key”升级为“查表 / 查行 / 查映射”。
- 将扫描阶段的 `ScannedFile.kind` 粗粒度字符串升级为 `FilePurpose` 文件角色分类，先落 deterministic 路径/文件名规则，并让 hierarchy / planner / context / query 使用新角色信号。
- 在 SQLite 中建立 `wiki_pages_fts` 和 `symbols_fts` FTS5 虚拟表；本迭代先完成页面标题与路径的 BM25 查询接入，并为迭代 7 的符号搜索预留 schema。
- 升级 `WikiPageState` 的 section 级状态，补入页面级 `section_anchors` 聚合字段，让 `sync`、`change_set` 与 section-level 脏检测共享同一组稳定锚点。
- 升级 steering 扫描配置结构，使其对齐 `.wiki/06-设计文档/00-总体设计.md` 的 `scan.ignore` / `scan.include` 形态，并明确 include 对 ignore 的覆盖关系。

## Capabilities

### New Capabilities
- `scanner-file-purpose`: 为扫描层引入稳定的 `FilePurpose` 分类，并让后续模块拆分、关键源码选择和页面规划消费该角色信号。
- `wiki-bm25-query`: 为 Wiki 页面建立 FTS5/BM25 搜索能力，并把页面标题/路径检索接入现有 `query` workflow。

### Modified Capabilities
- `sqlite-cache-storage`: 把 SQLite runtime 从 `kv_store` 主导升级为关系型 schema + FTS5 索引，并重写完整性检查与持久化要求。
- `wiki-state-kernel`: 调整 WikiState 的持久化边界和页面状态字段，补入 `section_anchors` 并明确关系型读写模型。
- `repo-wiki-workflow`: 更新 `init / update / sync / rebuild` 对关系型 state、FTS 索引和页面锚点的写入与刷新要求。
- `wiki-change-set-kernel`: 更新 change-set 对 SQLite runtime 完整性、页面锚点和 section-level 失效边界的判断方式。
- `wiki-steering-config`: 调整扫描相关 steering schema，从旧的 ignore 结构升级到 `scan.ignore` / `scan.include`。

## Impact

- 主要影响 `crates/wiki-core/src/storage/*`、`crates/wiki-core/src/repo/scanner.rs`、`crates/wiki-core/src/generation/*`、`crates/wiki-core/src/workflows/{init,query,sync,update}.rs`、`crates/wiki-core/src/domain/{state,change_set,steering}.rs`。
- 需要新增 SQLite schema 初始化 / 不完整库回退逻辑、FTS5 索引维护逻辑，以及围绕 `FilePurpose` 的测试夹具与工作区级验证。
- 不引入外部搜索服务；继续使用 `rusqlite bundled`，并保持 `.wiki/*.md`、`wiki.metadata.json`、`.wiki/.cache/**` 的三层 runtime 边界不混用。
