# refactor-specwiki-around-contract-closure-projection-writeback-boundaries 系统测试用例

## 用例总览

本系统测试用例覆盖 projection/writeback 边界闭合后的可观察验收：自动生成 section 必须具备稳定 ownership 与 binding；declared authoring 必须走 runtime extract -> knowledge validate -> runtime commit；derived/static/manual drift 必须按状态分类处理；metadata、projection digest、正式页面树和 snapshot manifest 必须保持闭环；旧 `.wiki/pages/**` 与 legacy heading fallback 不再作为成功路径。

系统测试以 Rust runtime 集成测试为主，重点落在 `crates/wiki-runtime/tests/runtime/editable_runtime.rs`、`crates/wiki-runtime/tests/runtime/knowledge_artifacts_roundtrip.rs`、`crates/wiki-runtime/tests/runtime/state_kernel.rs`、`crates/wiki-runtime/tests/runtime/metadata_roundtrip.rs` 和 `crates/wiki-runtime/tests/runtime/managed_section_kernel.rs`。本 change 不新增 query、graph、CLI、governance 或 archive 验收。

## 系统测试用例

### ST-001 Generated Section Binding Roundtrip

- 关联成功标准: 自动生成的 managed section 均有稳定 section id、ownership、knowledge/source/projection binding 和 content hash；`wiki.metadata.json` 能反查 page -> section -> knowledge refs。
- 覆盖设计点: `SectionOwnership`、`SectionBinding`、`ProjectionDigest`、metadata section binding、runtime page protocol。
- 前置条件: 临时仓库包含最小源码文件，执行 `run_init`。
- 操作 / 触发: 初始化后读取正式 `.wiki/**/*.md` 页面、`.wiki/wiki.metadata.json`、`.wiki/.knowledge/runtime/**` projection digest 与 snapshot manifest。
- 期望结果: 每个自动生成 section 都有 `owner_kind`、`section_id`、`knowledge_refs`、`source_refs`、`input_hash`、`content_hash`、`projection_digest_ref`；metadata 正向与反向引用均能解析到存在的 page、section 和 knowledge artifact；无 `managed: bool` 作为正式断言字段。
- 验证方式: Rust 集成测试，扩展 `metadata_roundtrip.rs` / `state_kernel.rs` / `knowledge_artifacts_roundtrip.rs`，命令 `cargo test -p wiki-runtime --test runtime generated_section_binding_roundtrip` 或等价聚焦命令。

### ST-002 Declared Writeback Contract

- 关联成功标准: 合法 declared authoring 修改后，`sync` 产生 `declared_writeback`，更新 `.wiki/.knowledge/declared/**`，并标记 impacted unit/projection stale。
- 覆盖设计点: declared writeback 三段式、`DeclaredAuthoringCandidate`、`DeclaredWritebackDecision`、runtime persist/metadata/stale commit。
- 前置条件: `run_init` 生成正式页面树，测试在一个 `declared_managed` section 中写入合法 declared authoring block。
- 操作 / 触发: 执行 `run_sync`。
- 期望结果: sync page outcome 为 `declared_writeback`；declared artifact 被写入 `.wiki/.knowledge/declared/**`；metadata section binding 指向 declared record；对应 unit/projection 进入 stale 或等价待更新状态；snapshot manifest 能追踪本次 binding。
- 验证方式: Rust 集成测试，调整 `editable_runtime.rs` 中现有 declared writeback 场景，命令 `cargo test -p wiki-runtime --test runtime sync_classifies_valid_declared_block_as_declared_writeback` 或等价聚焦命令。

### ST-003 Non-Declared Drift Does Not Pollute Knowledge

- 关联成功标准: derived/projection/static managed section 被人工修改时结果必须是 `illegal_drift` 或 conflict，不得改写 `.wiki/.knowledge/derived/**`；manual/unmanaged section 变化只能进入 metadata-only，不能自动生成 declared record。
- 覆盖设计点: `SectionOwnership` 行为矩阵、`SyncResultKind` 五类结果、derived/static/manual drift 分类。
- 前置条件: `run_init` 后准备三个页面编辑：修改 `derived_managed` section 正文、修改 `projection_static` section 正文、修改或新增 `manual_unmanaged` section。
- 操作 / 触发: 分别执行 `run_sync`。
- 期望结果: derived/static 编辑返回 `illegal_drift` 或 conflict，`.wiki/.knowledge/derived/**` 内容 hash 不变；manual 编辑返回 `metadata_only`，不新增 declared record；sync report 不把这些结果压平成 success/warning。
- 验证方式: Rust 集成测试，扩展 `editable_runtime.rs`，命令 `cargo test -p wiki-runtime --test runtime sync_prioritizes_illegal_drift_over_declared_writeback` 和 `cargo test -p wiki-runtime --test runtime sync_classifies_user_only_edit_as_metadata_only` 或等价聚焦命令。

### ST-004 Page-Level Atomic Writeback

- 关联成功标准: runtime 不从 Markdown 正文反推 derived knowledge；合法 declared writeback 与同页 illegal drift 冲突时，不得部分提交污染 truth。
- 覆盖设计点: page-level atomic commit、`illegal_drift` 优先级、runtime persist/metadata/stale commit 边界。
- 前置条件: 同一页面同时包含一个合法 declared edit 和一个 derived/static managed body drift；另一个无关页面包含合法 declared edit。
- 操作 / 触发: 执行 `run_sync`。
- 期望结果: 发生 drift 的页面整体不提交 declared truth；无关页面可继续提交；`.wiki/.knowledge/declared/**` 只包含无关页面的合法 record；health/conflict 信号能定位失败页；无 derived artifact 被 Markdown 正文改写。
- 验证方式: Rust 集成测试，扩展 `editable_runtime.rs`，命令 `cargo test -p wiki-runtime --test runtime page_level_atomic_writeback`。

### ST-005 Restore Binding Closure

- 关联成功标准: `init / update / sync / restore` 后 `.wiki/.knowledge/**`、正式页面树、`wiki.metadata.json`、snapshot manifest 的绑定保持一致。
- 覆盖设计点: `ProjectionDigest`、section binding、snapshot manifest 接入、restore binding 校验。
- 前置条件: `run_init`、合法 declared writeback、`run_update` 或 `run_sync` 后删除 `.wiki/.cache/**`。
- 操作 / 触发: 调用 `restore_runtime_cache_from_artifacts`。
- 期望结果: manifest -> metadata -> page hash -> section binding -> knowledge refs 全部一致时 restore 成功；恢复后 query 不作为本 ST 验收对象；metadata 不指向不存在 page/section/knowledge artifact。
- 验证方式: Rust 集成测试，扩展 `knowledge_artifacts_roundtrip.rs`，命令 `cargo test -p wiki-runtime --test runtime knowledge_artifacts_roundtrip_preserves_declared_and_health_records` 或等价聚焦命令。

### ST-006 Restore Rejects Broken Binding

- 关联成功标准: `wiki.metadata.json` 不会指向不存在的 page、section 或 knowledge artifact；restore 后绑定保持一致。
- 覆盖设计点: restore diagnostics、metadata binding mismatch、hash mismatch、projection digest mismatch。
- 前置条件: 已有 committed snapshot；人为破坏 page hash、section binding、projection digest 或 metadata reverse ref 中任意一项。
- 操作 / 触发: 删除 `.wiki/.cache/**` 后调用 `restore_runtime_cache_from_artifacts`。
- 期望结果: restore 拒绝恢复并进入 blocked/conflict 或等价失败状态；不得从 Markdown page 反推或修补 derived/declared truth；previous committed snapshot 不被污染。
- 验证方式: Rust 集成测试，扩展 `knowledge_artifacts_roundtrip.rs` 中现有 drift/mismatch 用例，命令 `cargo test -p wiki-runtime --test runtime restore_refuses_projection_digest_metadata_mismatch` 或等价聚焦命令。

### ST-007 Legacy Paths And Heading Fallback Are Not Runtime Surface

- 关联成功标准: 当前阶段不保留 `.wiki/pages/**` 或 legacy compatibility 逻辑作为验收前提。
- 覆盖设计点: no `.wiki/pages/**` read/write/diagnose，删除 legacy heading parse fallback。
- 前置条件: 测试仓库存在 `.wiki/pages/old.md` 和一个无 marker 的 `##` heading 页面。
- 操作 / 触发: 执行 `run_sync` / `run_update` / managed section parse。
- 期望结果: runtime 不读取、不迁移、不诊断 `.wiki/pages/**`；无 marker 页面不会通过 heading best-effort 自动升级为 managed section；相关结果进入 marker missing/illegal drift/conflict 或等价拒绝状态。
- 验证方式: Rust 集成测试，改写 `managed_section_kernel.rs` 中 legacy heading 用例，并补 runtime workflow guard，命令 `cargo test -p wiki-runtime --test runtime managed_section_kernel` 或等价聚焦命令。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| 自动生成的 managed section 均有稳定 section id、ownership、knowledge/source/projection binding 和 content hash | ST-001 | Rust 集成测试 |
| `wiki.metadata.json` 能反查 page -> section -> knowledge refs，并且不会指向不存在 artifact | ST-001, ST-005, ST-006 | Rust 集成测试 |
| 合法 declared authoring 修改后产生 `declared_writeback` 并更新 declared artifacts / stale projection | ST-002, ST-004 | Rust 集成测试 |
| derived/projection/static drift 不得写回 `.wiki/.knowledge/derived/**` | ST-003, ST-004 | Rust 集成测试 |
| manual/unmanaged section 变化只能 metadata-only | ST-003 | Rust 集成测试 |
| `init / update / sync / restore` 后 knowledge、page、metadata、manifest 绑定一致 | ST-005, ST-006 | Rust 集成测试 |
| runtime 不从 Markdown 正文反推 derived knowledge；knowledge 不承担页面落盘、metadata、cache 或 transport | ST-003, ST-004, ST-006 | Rust 集成测试 + UT 互证 |
| 双 page generation 通道和重复模型不再造成对外行为分叉 | ST-001, ST-005 | Rust 集成测试 + UT 互证 |
| 不保留 `.wiki/pages/**` 或 legacy compatibility 逻辑 | ST-007 | Rust 集成测试 |

## 边界与异常

- marker 缺 `id`、缺 `owner`、版本非法、start/end id 不匹配、owner 与 metadata 不一致，必须是 typed diagnostic，不以 warning string 作为唯一状态。
- 同一页面内 `illegal_drift` 或 `conflict` 优先于 declared writeback，避免半提交污染 truth。
- `manual_unmanaged` 变化不能自动创建 declared record。
- restore 失败不能从 Markdown 反推 knowledge，也不能伪造 projection digest。
- `.wiki/pages/**`、query route、code graph/index、CLI product surface、governance/archive 均不纳入本 change 系统测试。

## 验证数据与环境

- Rust 临时仓库 fixture，使用 `tempfile::tempdir`。
- 最小源码文件如 `main.rs` / `src.ts`。
- 运行入口使用现有 `run_init`、`run_sync`、`run_update`、`restore_runtime_cache_from_artifacts`。
- 计划验证命令以 `cargo test -p wiki-runtime ...` 和 `cargo test -p wiki-model ...` 为主；最终 apply 阶段按实际 test target 调整聚焦命令。

## 未覆盖项

- Query route/readiness、code graph/index schema、CLI help/product surface、governance/archive transaction 均为 proposal 非目标，由后续 child change 覆盖。
- 完整 snapshot commit protocol 重构不是本 child 目标；本 child 只验证 projection/writeback binding 接入现有 manifest。

## 参考资料

- `proposal.md`
- `design.md`
- `.wiki/02-开发指南/01-测试与验收.md`
