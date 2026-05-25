## 1. `wiki-index` 查询合同与读取面

- [x] 1.1 在 `crates/wiki-index` 中补齐 facts-owned 查询合同，覆盖 `source_lookup / module_lookup / entrypoint_lookup` 所需的 `module_source_map`、source 视图与 entrypoint 读取接口。
- [x] 1.2 在 `crates/wiki-runtime/src/storage/sqlite/index_store.rs` 中实现新增的 index 读取接口，并明确 `modules / module_source_map / scan_cache / module-tree` 的 contract owner 归 `wiki-index`。
- [x] 1.3 在 `crates/wiki-index` 中新增内部 query service 与最小 request/result 类型，统一 `auto / symbol_lookup / source_lookup / module_lookup / entrypoint_lookup / callers / callees / impact_slice` taxonomy。
- [x] 1.4 收紧查询结果字段语义，只保留 `match_basis`、FTS `score` 与 graph-derived `confidence / reason`，删除或避免引入伪统一 `tier`。

## 2. facts snapshot 落盘与 workflow readiness

- [x] 2.1 将 facts snapshot 写入从 `write_state_with_symbol_graph*` 中拆出独立 helper，分别负责 `scan_cache`、`module_tree`、`modules / module_source_map` 与 `symbols / edges / communities / processes` 的落盘。
- [x] 2.2 调整 `init` 与 `rebuild` 主链，在进入 `knowledge_planning / research / compose` 前先提交完整 facts snapshot。
- [x] 2.3 调整 `update` 主链，在 scoped/full 两种刷新路径下都先刷新受影响 facts snapshot，再进入 downstream 阶段。
- [x] 2.4 明确并实现 readiness 语义：facts snapshot 已提交后，`researching / compose_pending` 等 downstream incomplete 状态不得清空或跳过 index snapshot。

## 3. runtime query adapter 与外部入口保持

- [x] 3.1 重构 `crates/wiki-runtime/src/workflows/query.rs`，保留 `run_query(repo_root, term)` 外部入口，但内部统一映射为 `wiki-index::query` 的 `auto` 请求。
- [x] 3.2 让 `QueryReport` 的 `matched_symbols / matched_sources / matched_modules / matched_symbol_edges` 由 index 结果投影生成，去掉 runtime 自己平行拼装 facts 查询语义的主路径。
- [x] 3.3 保留 page fallback 作为次级路径，并在结果中显式标注 provenance，避免页面命中伪装成 index 命中。
- [x] 3.4 为 `index not ready` 与“查询成功但无命中”补齐显式区分与对应错误/结果处理。

## 4. 测试、样本验证与注释检查

- [x] 4.1 为 `wiki-index` 与 `wiki-runtime` 增加 Rust 测试，覆盖 `symbol_lookup / source_lookup / module_lookup / entrypoint_lookup / callers / callees / impact_slice` 以及 `match_basis / score / confidence / reason` 语义。
- [x] 4.2 增加 workflow/readiness 测试，验证 facts snapshot 已提交后，即使 downstream 处于 `researching / compose_pending`，index-first 查询仍可工作。
- [x] 4.3 运行 `node scripts/run-test-projects.mjs storybook dagger`，检查 `modules / module_source_map / symbols / edges` 落盘和 query 表现，并输出本 change 的专项分析报告。
- [x] 4.4 运行 `node scripts/test-wiki-lifecycle.mjs storybook dagger`，验证 `init -> status -> sync -> update -> rebuild` 下的 index readiness 与 query 不回退。
- [x] 4.5 在 `.spec/changes/iteration-11-index-query-surface-and-agent-intents/reference-project-reports/` 中补齐 `storybook`、`dagger` 与总结合并报告，明确记录 snapshot readiness、query 命中与剩余断点。
- [x] 4.6 单独执行一轮 `COMMENTING.md` 合规检查，确认本轮新增或修改代码的注释范围、粒度与风格符合要求。


