---
implementation-ready: true
---

# refactor-specwiki-around-contract-closure-page-tree-contract 任务计划

## 任务总览

任务拆分依据来自 `design.md` 的能力块：页面路径规划、正式页面树 predicate、runtime 写入与恢复边界、query/update/rebuild 验证、文档与 fixture 口径同步。本计划只进入规划阶段，不写实现代码；用户确认后才允许把 `implementation-ready` 改为 `true` 并进入实现。

## 实现模式

normal

采用 normal 模式：按结构化实现清单推进，完成每个大 task 前补齐单元测试或替代局部验证；不要求先生成 `unit-tests.md`，也不在 plan 阶段写真实测试或实现代码。

## 1. 页面树路径规划与正式路径 predicate

- [x] 1.1 修改 `crates/wiki-knowledge/src/planning.rs` 的页面路径规划，使 `KnowledgeUnit.relative_path` 直接产出 `.wiki/INDEX.md`、`.wiki/<栏目路径>/INDEX.md` 或 `.wiki/<栏目路径>/NN-主题.md` 对应的相对路径语义。
- [x] 1.2 调整 `crates/wiki-knowledge/src/projection.rs` 中 `PlannedPage.relative_path` 的透传与校验，确保下游不再接收根级散页或 `.wiki/pages/**` 作为正式规划结果。
- [x] 1.3 在 `crates/wiki-runtime/src/storage/wiki_fs.rs` 或相邻 domain helper 中新增正式页面树 predicate / normalizer，集中判断 `.wiki/INDEX.md`、栏目 `INDEX.md`、`NN-主题.md` 与 runtime 隐藏目录排除规则。
- [x] 1.4 将 planner 输出与正式页面树 predicate 接起来：planner 产出非正式路径时阻断 workflow，而不是由 runtime 下游静默改名。
- [x] 1.5 添加或调整 planner / predicate 局部测试，覆盖多级栏目、根 `INDEX.md`、栏目 `INDEX.md`、`NN-主题.md`、`.wiki/pages/**`、`.wiki/.knowledge/**`、`.wiki/.cache/**` 和 metadata 文件排除。

### CheckList

- [x] 单元测试或替代局部验证已覆盖
- [x] 相关验证通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 2. runtime 写入、metadata 与 SQLite 只消费正式页面

- [ ] 2.1 修改 `crates/wiki-runtime/src/workflows/init.rs` 的页面写入和 generated pages 统计，只写入并返回正式页面树路径。
- [ ] 2.2 修改 `crates/wiki-runtime/src/workflows/update.rs` 的 planned page 写入、路径变更处理和 `AffectedProjectionScope`，确保 `.wiki/pages/**` 不被刷新或纳入 affected scope。
- [ ] 2.3 修改 `crates/wiki-runtime/src/workflows/rebuild.rs` 与 restore 相关路径，使重建过程只从正式页面树和正式 metadata 恢复 runtime state。
- [ ] 2.4 修改 `crates/wiki-runtime/src/domain/state.rs` 与 `crates/wiki-runtime/src/domain/metadata_mapper.rs`，确保 `WikiState.pages` 和 `.wiki/wiki.metadata.json` 只导出正式页面。
- [ ] 2.5 修改 `crates/wiki-runtime/src/storage/knowledge_artifacts.rs`、state store 和 SQLite 写入路径，确保 `wiki_pages / wiki_pages_fts` 只记录正式页面。
- [ ] 2.6 添加或调整 runtime 集成测试，覆盖 init / update / rebuild / restore 后 metadata、state、SQLite 路径全量满足正式页面树 predicate。

### CheckList

- [ ] 单元测试或替代局部验证已覆盖
- [ ] 相关验证通过
- [ ] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [ ] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 3. query、status 与旧测试迁移验证

- [ ] 3.1 修改 `crates/wiki-runtime/src/workflows/query.rs` 的 Markdown fallback 与 page refs 回填逻辑，使默认 query 只返回正式页面路径。
- [ ] 3.2 检查 status machine-readable 输出路径，移除 `.wiki/pages/**` 专用状态、诊断字段或清理建议。
- [ ] 3.3 更新 `crates/wiki-runtime/tests/acceptance/init_generates_wiki.rs`、`crates/wiki-runtime/tests/acceptance/baseline_acceptance.rs` 和相关 runtime tests，将断言从根级散页 / 旧目录改为正式页面树。
- [ ] 3.4 更新 `scripts/tests/e2e.test.ts` 与涉及 reference report / init debug trace 的脚本测试，使其断言 `.wiki/INDEX.md`、栏目 `INDEX.md` 和 `NN-主题.md`。
- [ ] 3.5 添加 query 干扰文件验证：存在 `.wiki/pages/ignored.md` 且包含唯一短语时，默认 query 不返回该文件作为正式页面结果。

### CheckList

- [ ] 单元测试或替代局部验证已覆盖
- [ ] 相关验证通过
- [ ] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [ ] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 4. 文档、规格与 fixture 口径同步

- [ ] 4.1 更新 `.wiki/00-文档约定/00-边界与SSOT规则.md`、`.wiki/00-文档约定/01-页面模板.md` 和相关模块 / 对外方法页面，明确正式页面树规则。
- [ ] 4.2 更新 `.docs/design/knowledge-to-wiki-projection-contract.md`、`.docs/design/governance-runtime-integration.md`、`.docs/design/specwiki-contract-closure.md` 中仍把 `.wiki/pages/**` 写作 runtime page projection 的旧口径。
- [ ] 4.3 更新 capability specs、fixtures、测试快照和脚本文案，确保 `.wiki/pages/**` 只在“runtime surface 外目录”或历史上下文中出现。
- [ ] 4.4 执行文档搜索复核，确认没有把 `.wiki/pages/**` 描述为新版写入目标、query 来源、restore 来源或 status 分支。

### CheckList

- [ ] 单元测试或替代局部验证已覆盖
- [ ] 相关验证通过
- [ ] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [ ] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 | 1. 页面树路径规划与正式路径 predicate；2. runtime 写入、metadata 与 SQLite 只消费正式页面 | 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.6 |
| ST-002 | 2. runtime 写入、metadata 与 SQLite 只消费正式页面 | 2.3, 2.4, 2.5, 2.6 |
| ST-003 | 3. query、status 与旧测试迁移验证 | 3.1, 3.5 |
| ST-004 | 2. runtime 写入、metadata 与 SQLite 只消费正式页面；3. query、status 与旧测试迁移验证 | 2.2, 2.3, 2.5, 3.2 |
| ST-005 | 4. 文档、规格与 fixture 口径同步 | 4.1, 4.2, 4.3, 4.4 |

## 执行顺序

- 先执行 1，建立路径规划与 predicate 合同。
- 再执行 2，把写入、metadata、state、SQLite 与 restore 边界统一到正式页面树。
- 然后执行 3，收口 query、status 和现有测试断言。
- 最后执行 4，统一文档、规格、fixture 和脚本口径。
- 每个大 task 完成后先跑对应局部验证，再进入下一个大 task；最终再执行 `system-tests.md` 中的系统级验收。

## 暂缓事项

- 不迁移、不清理、不诊断 `.wiki/pages/**`。
- 不实现 truth kind、两级 restore、projection ownership、query route DTO、code graph index、governance isolation 或 archive dry-run manifest。
- 不保留旧 page id 兼容；路径变化按破坏性重建处理。
