---
implementation-ready: true
---

# refactor-specwiki-around-contract-closure-projection-writeback-boundaries 任务计划

## 任务总览

任务按 TDD 组织，先让 projection/writeback 合同的关键断言失败，再做最小实现，最后重构收口。大 task 对齐设计中的能力块：共享 projection 模型、runtime page protocol、drift 分类、declared writeback、metadata/snapshot binding、projection digest 归属和 legacy fallback 删除。

## 实现模式

tdd

先写失败单元测试并确认失败，再写最小实现，通过后重构。每个 Red task 必须先提交或记录当前实现的精确失败原因；不得先移动 DTO、拆函数或删除 fallback。

## 1. 共享 Projection 模型合同

- [x] 1.1 Red: UT-001 编写 `SectionOwnership / SectionBinding / ProjectionDigest / SyncResultKind` 的失败单元测试，并确认缺模型或缺 validate 的失败原因符合预期。
- [x] 1.2 Green: UT-001 在 `wiki-model/src/domain/projection.rs` 实现最小共享模型、serde 和 validate，使测试通过。
- [x] 1.3 Refactor: UT-001 清理模型导出、命名和注释，确保 `ProjectionDigest` 不进入 `knowledge_artifact.rs`，并补齐公开字段注释。

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（建议命令：`cargo test -p wiki-model section_binding_requires_owner_refs_and_hashes`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 2. Runtime Marker Protocol 与 Section Binding Roundtrip

- [x] 2.1 Red: UT-002 编写 marker 缺 `owner/id/version`、start/end mismatch、owner 与 metadata mismatch 的 typed diagnostic 失败测试。
- [x] 2.2 Green: UT-002 扩展 `ManagedSectionBlock`、`ParsedWikiPage` 和 parser diagnostic，确保非法 marker 不被 best-effort 接受。
- [x] 2.3 Refactor: UT-002 清理 parser 内部结构，确保 diagnostic 使用 enum，不依赖 warning string。
- [x] 2.4 Red: UT-003 编写 render/parse roundtrip 保留 `owner_kind / hashes / refs / projection_digest_ref` 的失败测试。
- [x] 2.5 Green: UT-003 扩展 marker render 与 parse schema，使 roundtrip 保留 section binding。
- [x] 2.6 Refactor: UT-003 收敛 renderer 与 managed section kernel 的重复构造逻辑，避免通过标题或顺序猜测 ownership。

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（建议命令：`cargo test -p wiki-runtime --test runtime managed_section_kernel`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 3. Drift 分类与 Page-Level Atomic 规则

- [x] 3.1 Red: UT-004 编写 `classify_section_drift` 五类结果失败测试，覆盖 declared、manual、derived/static、owner mismatch、stale hash。
- [x] 3.2 Green: UT-004 实现最小 drift classifier，并让 `sync` 复用分类结果而不是手写字符串状态。
- [x] 3.3 Refactor: UT-004 清理 `sync` 中 drift 相关重复逻辑，确保结果分类不压平成 success/warning。
- [x] 3.4 Red: ST-003 / ST-004 编写或调整 runtime 集成测试，确认 derived/static drift 不写 derived knowledge，同页 illegal drift 阻止 declared partial writeback。
- [x] 3.5 Green: ST-003 / ST-004 接入 page-level atomic writeback，使失败页不提交 declared truth，无关页可继续提交。
- [x] 3.6 Refactor: ST-003 / ST-004 清理 sync outcome 与 health/conflict 信号生成边界。

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（建议命令：`cargo test -p wiki-runtime --test runtime classify_section_drift_returns_all_contract_kinds` 和相关 `editable_runtime` 聚焦测试）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 4. Declared Writeback 语义迁入 Knowledge

- [x] 4.1 Red: UT-005 编写 `validate_declared_writeback(candidate, snapshot)` 失败测试，要求合法 candidate 输出 patch，scope/lifecycle/relation 冲突输出 rejected/conflict。
- [x] 4.2 Green: UT-005 在 `wiki-knowledge` 实现最小 declared writeback validation API，并复用 `wiki-model` declared record 校验。
- [x] 4.3 Refactor: UT-005 确认 knowledge API 不接受 Markdown、marker string、文件路径写入句柄或 metadata patch。
- [x] 4.4 Red: ST-002 编写或调整合法 declared authoring 的系统测试，断言 runtime extract -> knowledge validate -> runtime persist/metadata/stale commit。
- [x] 4.5 Green: ST-002 改造 `sync` 只提取 candidate 并调用 knowledge validation，再提交 declared patch。
- [x] 4.6 Refactor: ST-002 清理 `sync.rs` 里直接 materialize declared record 的旧逻辑，保留 runtime parse/extract/commit 职责。

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（建议命令：`cargo test -p wiki-knowledge validate_declared_candidate_normalizes_scope_and_lifecycle` 和 `cargo test -p wiki-runtime --test runtime sync_classifies_valid_declared_block_as_declared_writeback`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 5. Metadata Section Binding 与 Snapshot Restore 闭环

- [x] 5.1 Red: UT-006 编写 metadata mapper section reverse refs 失败测试，覆盖 page -> section -> knowledge/source/projection_digest 与反向索引。
- [x] 5.2 Green: UT-006 扩展 `WikiMetadata` 和 `metadata_mapper`，写入 section-level binding 和 reverse refs。
- [x] 5.3 Refactor: UT-006 清理 metadata validation，拒绝或标记悬挂 page/section/artifact refs。
- [x] 5.4 Red: ST-001 / ST-005 / ST-006 编写或调整 artifact roundtrip 测试，确认 init/update/sync/restore 后 metadata、page、knowledge、manifest binding 一致，破坏任一 binding 时 restore 失败。
- [x] 5.5 Green: ST-001 / ST-005 / ST-006 接入现有 snapshot manifest 校验，不重做完整 commit protocol。
- [x] 5.6 Refactor: ST-001 / ST-005 / ST-006 收窄 `knowledge_artifacts` storage adapter，避免在 adapter 内派生 declared lifecycle、conflict 或 projection health。

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（建议命令：`cargo test -p wiki-runtime --test runtime metadata_mapper_builds_section_reverse_refs` 和相关 `knowledge_artifacts_roundtrip` 聚焦测试）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 6. ProjectionDigest 归属与重复模型清理

- [x] 6.1 Red: UT-007 编写 persisted artifacts 使用 `ProjectionDigest` 而非 `PageDraft/PageDigest` restore anchor 的失败测试。
- [x] 6.2 Green: UT-007 将 persisted projection digest 迁到 `wiki-model/src/domain/projection.rs`，runtime storage 消费 model DTO。
- [x] 6.3 Refactor: UT-007 拆分 compose 子页摘要与 projection recovery anchor，避免 `PageDraft` 写入 `.wiki/.knowledge/**`。
- [x] 6.4 Red: ST-001 / ST-005 增加系统级断言，确认对外 binding 不因双 page generation 通道分叉。
- [x] 6.5 Green: ST-001 / ST-005 删除或替换 runtime 侧重复 compose/research DTO，runtime 直接消费 model/knowledge DTO。
- [x] 6.6 Refactor: ST-001 / ST-005 清理命名，将 `PlannedPage / PlannedSection` 一次性替换为 `PagePlan / SectionPlan`，不保留兼容别名。

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（建议命令：`cargo test -p wiki-runtime --test runtime persisted_artifacts_use_projection_digest_not_page_draft`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 7. 删除 Legacy Heading Fallback 与旧 Page Surface

- [x] 7.1 Red: UT-008 编写无 marker 页面不自动升级 managed section、`.wiki/pages/**` 不进入 runtime surface 的失败测试。
- [x] 7.2 Green: UT-008 删除正式路径中的 legacy heading fallback 和 `.wiki/pages/**` 读取/迁移/诊断逻辑。
- [x] 7.3 Refactor: UT-008 删除或改写保护 legacy heading 行为的旧测试，确保新断言以 marker missing / illegal drift / conflict 为准。
- [x] 7.4 Red: ST-007 增加 runtime workflow guard，确认 `sync/update` 不把 `.wiki/pages/**` 作为 page truth。
- [x] 7.5 Green: ST-007 接入 official page path 过滤，排除 `.wiki/pages/**`。
- [x] 7.6 Refactor: ST-007 清理注释和文档化边界，避免出现兼容性承诺。

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（建议命令：`cargo test -p wiki-runtime --test runtime page_without_markers_is_not_legacy_managed`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 | 1. 共享 Projection 模型合同；2. Runtime Marker Protocol；5. Metadata Section Binding；6. ProjectionDigest 归属 | 1.1-1.3 / 2.4-2.6 / 5.4-5.6 / 6.4-6.6 / UT-001 / UT-003 / UT-006 / UT-007 |
| ST-002 | 4. Declared Writeback 语义迁入 Knowledge | 4.1-4.6 / UT-005 |
| ST-003 | 3. Drift 分类与 Page-Level Atomic 规则 | 3.1-3.6 / UT-004 |
| ST-004 | 3. Drift 分类与 Page-Level Atomic 规则；4. Declared Writeback 语义迁入 Knowledge | 3.4-3.6 / 4.4-4.6 / UT-004 / UT-005 |
| ST-005 | 5. Metadata Section Binding；6. ProjectionDigest 归属 | 5.1-5.6 / 6.1-6.6 / UT-006 / UT-007 |
| ST-006 | 5. Metadata Section Binding | 5.1-5.6 / UT-006 |
| ST-007 | 7. 删除 Legacy Heading Fallback 与旧 Page Surface | 7.1-7.6 / UT-008 |

## 执行顺序

- 先执行 task 1，建立共享模型，否则后续 parser、metadata、sync 无法引用统一 DTO。
- 再执行 task 2 和 task 3，固定 runtime page protocol 与 drift 分类。
- task 4 在 runtime candidate 提取边界稳定后执行，把 declared semantic validation 收口到 `wiki-knowledge`。
- task 5 和 task 6 处理 metadata / artifact / restore binding 与 digest 归属，避免 storage 继续承载语义。
- 最后执行 task 7，删除 legacy fallback 和旧 page surface，清理旧测试。
- 每个大 task 完成后运行对应聚焦测试；最终进入 review 前再运行 `cargo test`，必要时补 `pnpm run test` / `pnpm run lint` 以覆盖 TS wrapper 影响。

## 暂缓事项

- 不实现 query route/readiness、code graph/index schema、CLI product surface、governance/archive transaction。
- 不重做完整 snapshot commit protocol，只验证 projection/writeback binding 接入现有 manifest。
- 不保留 `PlannedPage / PlannedSection` 或 `managed: bool` 的历史兼容别名。
