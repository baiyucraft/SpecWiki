## 1. Change Planning Kernel

- [x] 1.1 扩展 `crates/wiki-core/src/domain/change_set.rs`，定义 `ChangeSet / AffectedSet / ChangePlan`，覆盖 added/modified/removed/structural sources 与 affected modules/pages/sections
- [x] 1.2 为 scan cache 与当前仓库快照实现统一比对逻辑，识别局部可修复变化、结构变化和 `needs_rebuild` 边界
- [x] 1.3 抽出共享的 runtime bundle / change planning 入口，供 `status`、`update`、`rebuild` 复用同一套变化规划

## 2. State And Cache Model

- [x] 2.1 扩展 `crates/wiki-core/src/domain/state.rs`，为 `WikiPageState` 引入 `input_hash`、section 状态和稳定的 source->module->page->section 映射
- [x] 2.2 扩展 `crates/wiki-core/src/storage/`，新增 `.wiki/.cache/page-contexts/` 与 `.wiki/.cache/page-generation/` 的读写、删除和按页面失效能力
- [x] 2.3 调整 `init` / `rebuild` 的初始化路径，确保首次生成就写出完整的增量 runtime cache 布局，而不是只有整仓级 cache

## 3. Section-Level Generation

- [x] 3.1 重构 `crates/wiki-core/src/generation/sections.rs`，把页面章节输出改为稳定 section draft（含 section id、title、managed、source/relation 映射）
- [x] 3.2 重构 `crates/wiki-core/src/generation/renderer.rs` 与相关组装逻辑，支持按 section 渲染并统一装配整页 Markdown
- [x] 3.3 为页面上下文和 section 渲染结果建立按 page id + input hash 复用的缓存入口

## 4. Incremental Workflows

- [x] 4.1 重写 `crates/wiki-core/src/workflows/status.rs`，基于 `ChangePlan` 返回准确的 `fresh / stale / missing / needs_rebuild` 和受影响页面集合
- [x] 4.2 重写 `crates/wiki-core/src/workflows/update.rs`，在局部可修复时只重建受影响页面，并处理页面新增、删除和祖先链变化
- [x] 4.3 校准 `crates/wiki-core/src/workflows/rebuild.rs`、`sync.rs` 和 `query.rs`，确保它们与扩展后的 state/cache/section 布局一致

## 5. Automated Verification

- [x] 5.1 新增 change-set 与状态层测试，覆盖单文件修改、新增/删除源码、结构变化、section hash 稳定性和 `needs_rebuild` 升级边界
- [x] 5.2 新增 workflow / integration 测试，验证单文件更新只触达相关页面、结构变化后页面计划正确更新、未受影响页面不被重写
- [x] 5.3 新增 cache 缺失测试，覆盖 `wiki-state.json`、page context cache、generation cache 缺失后的 `status` / `update` / `rebuild` 行为
- [x] 5.4 跑通仓库自动化测试入口（至少 `cargo test` 与根级 `test`），修复增量 runtime 引入的回归

## 6. Test Project Set Analysis

- [x] 6.1 对 `E:\\project\\aLocal` 执行 `init`，把结果与 reference 的页面结构、内容信号和 `wiki.metadata.json` 字段做对照分析
- [x] 6.2 对当前仓库 `E:\\project\\!byAI\\spec-wiki` 执行 `init`，重点分析 fixture 误提升、单文件模块、关键源码淹没、kind 分类和增量 runtime 边界
- [x] 6.3 按 `DESIGN.md` 的测试项目集要求，对其余目标仓库执行 `init` 并记录差异，必要时回调本 change 的 spec / design / tasks 或实现

## 7. Commenting Compliance

- [x] 7.1 按 `COMMENTING.md` 检查本轮新增或修改文件的注释，覆盖公共接口、核心类型、关键流程和测试场景注释，并修正不符合规范的注释
