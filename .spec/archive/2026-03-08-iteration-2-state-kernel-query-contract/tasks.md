## 1. WikiState 装配与持久化

- [x] 1.1 补齐 `domain/state.rs` 中 `WikiState` 的装配函数：从 init pipeline 的构建结果（PlannedPage、PageContext、ScanReport、ModuleTree）装配完整 WikiState
- [x] 1.2 新增 `storage/state_store.rs`：实现 `write_state(repo_root, &WikiState)` 和 `read_state(repo_root) -> io::Result<WikiState>`，持久化到 `.wiki/.cache/wiki-state.json`
- [x] 1.3 新增 `load_or_rebuild_state(repo_root) -> io::Result<WikiState>`：优先读 WikiState，不存在时从 WikiMetadata 重建，都不存在时返回错误
- [x] 1.4 实现从 WikiMetadata 重建 WikiState 的回退路径：`fn rebuild_state_from_metadata(metadata: &WikiMetadata) -> WikiState`
- [x] 1.5 在 `storage/mod.rs` 中导出 state_store，在 `domain/mod.rs` 中确认 state 模块已导出

## 2. MetadataMapper 导出层

- [x] 2.1 新增 `domain/metadata_mapper.rs`：实现 `fn export_metadata(state: &WikiState, context: &ExportContext) -> WikiMetadata`
- [x] 2.2 定义 `ExportContext` 结构体：承载 repo_root、branch、last_indexed_commit、language、schema_version 等外部展示字段
- [x] 2.3 实现 WikiPageState → WikiItem 的映射逻辑：从 WikiState 提取核心字段，从构建上下文补齐 title、item_type、parent_id、ancestor_ids、summary、provenance
- [x] 2.4 实现 SourceState → SourceFileRecord 的映射逻辑
- [x] 2.5 验证导出的 WikiMetadata 与迭代 1 的字段结构完全兼容（所有已有字段不缺失、不重命名）

## 3. init Workflow 迁移

- [x] 3.1 重构 `init.rs`：在页面渲染完成后调用 WikiState 装配函数，替代当前 120-142 行的手工 WikiMetadata 拼装
- [x] 3.2 init 完成后先写 WikiState（state_store），再通过 MetadataMapper 导出 WikiMetadata 写盘
- [x] 3.3 确保 init 的外部响应结构（InitReport）不变
- [x] 3.4 运行现有 init 相关测试，确认全部通过

## 4. status Workflow 迁移

- [x] 4.1 重构 `status.rs`：使用 `load_or_rebuild_state()` 替代 `read_metadata()`
- [x] 4.2 基于 WikiState 中的 SourceState.fingerprint 与当前扫描结果比对判定脏状态
- [x] 4.3 处理 WikiState 丢失但 metadata 存在的回退场景
- [x] 4.4 确保 status 的外部响应结构（StatusReport）不变
- [x] 4.5 运行现有 status 相关测试，确认全部通过

## 5. query Workflow 迁移与输出统一

- [x] 5.1 重构 `query.rs`：将 `run_query` 的数据来源从 `read_metadata()` 切换到 `load_or_rebuild_state()`
- [x] 5.2 调整内部索引构建逻辑，从 WikiState 的 pages/sources/modules/relations 构建查询索引
- [x] 5.3 为 `QueryMatch` 新增 `context_pack` 字段：包含关联模块摘要、关键源码路径和关系证据
- [x] 5.4 为 `QueryReport` 新增 `provenance_summary` 字段：整体命中来源的结构化摘要
- [x] 5.5 确保新增字段为向后兼容（新增不删除），现有 query 测试全部通过

## 6. sync / update / rebuild Workflow 迁移

- [x] 6.1 重构 `sync.rs`：读取 WikiState → 更新 content_hash → 持久化 WikiState → 通过 MetadataMapper 导出 metadata
- [x] 6.2 确认 `update.rs` 和 `rebuild.rs` 当前调用 init，自动跟随迁移，无需额外修改
- [x] 6.3 运行现有 sync/rebuild 相关测试，确认全部通过

## 7. 状态与 cache 分层验证

- [x] 7.1 更新 `cache_store.rs`：新增 `write_state_cache` / `read_state_cache` 或确认 state_store 已覆盖
- [x] 7.2 更新 `has_cache_layout()` 检查，将 `wiki-state.json` 纳入缓存完整性判断
- [x] 7.3 验证 cache 全部删除后，status 仍能从 metadata 回退工作

## 8. 测试与验收

- [x] 8.1 新增状态不变量测试：验证 `WikiState -> MetadataMapper -> WikiMetadata` 的 roundtrip 一致性
- [x] 8.2 新增 query 输出结构测试：验证 context_pack 和 provenance_summary 字段存在且内容合理
- [x] 8.3 新增 WikiState 回退测试：删除 wiki-state.json 后 status/query 仍能从 metadata 重建并正常工作
- [x] 8.4 对 `E:\project\aLocal` 执行 init，将产物快照放到 `tmp/test-aLocal`，与 `tmp/reference-zh` 对照，确认页面结构和 metadata 字段无退化
- [x] 8.5 运行全量测试（`pnpm run test`），确认无回归
