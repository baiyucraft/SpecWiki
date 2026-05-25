## Why

迭代 3 已经把 `ChangePlan`、page-level cache 和 section 粒度渲染落下来了，但当前 `.wiki` 仍然是“一次性生成物”而不是“可持续维护的文档层”：所有 section 默认都由 runtime 托管，`renderer` 只输出普通 Markdown 标题，`sync` 只刷新整页 `content_hash`，不会解析页面结构、不会更新 summary，也不会把用户新增内容纳入状态层。

最新归档下的 [`test-project-analysis.md`](E:/project/!byAI/spec-wiki/.spec/archive/2026-03-08-iteration-3-incremental-runtime/test-project-analysis.md) 已经把迭代 3 的结论收敛为“runtime 主链稳定，差异仍集中在 planner / composition 质量”。这说明现在推进 Editable Wiki Runtime 的时机是合适的：运行时边界已经足够稳定，不需要再等下一轮状态内核重构。一个最小实验仓库也验证了当前缺口：插入 `## 手工笔记` 后，`sync` 能把 runtime 恢复为 `fresh`，但 `WikiState.sections` 与 metadata summary 仍保持旧值，随后 `update` 会整页覆盖掉这段手工内容。

## What Changes

- 引入 `wiki-managed-section-kernel`，定义 managed section marker、页面解析、用户区段锚点和 merge 语义，让“生成内容”和“人工内容”有清晰边界。
- 扩展 Repo Wiki Runtime 页面格式和 `WikiState` section 模型，使 runtime 能区分 generated managed sections、synced user sections 和 managed drift，而不是只记录整页 hash。
- 改造 `sync`，把它从“整页 hash 回写”升级为“页面结构解析 + page/section 状态回写 + summary/metadata 同步”。
- 改造 `init / update / rebuild` 的页面装配路径：页面 identity 稳定时只替换 managed sections，保留已同步的 user sections；legacy 无 marker 页面提供 best-effort 迁移路径。
- 补齐 editable runtime 的验证面：手工插入 section、手工修改 managed block、legacy 页面迁移、source change 后保留 user content，以及测试项目集分析与注释规范检查。

## Capabilities

### New Capabilities
- `wiki-managed-section-kernel`: 定义 managed section marker、页面解析、user section 锚点和 merge 内核。

### Modified Capabilities
- `repo-wiki-runtime`: 正式 Wiki 页面从“纯生成 Markdown”升级为“managed section + user section 共存”的运行时格式。
- `repo-wiki-workflow`: `init / update / sync / rebuild` 的 requirement 需要升级为 editable runtime 语义。
- `wiki-state-kernel`: WikiState 需要承载 managed/user section 状态、generated/observed hash 和用户区段锚点。
- `workflow-verification`: 验证 requirement 需要覆盖手工编辑、sync 回写、legacy 页面迁移和 user content 保留。

## Impact

- `crates/wiki-core/src/generation/`：需要新增或重构页面 marker 写入、页面解析和 merge 组装逻辑。
- `crates/wiki-core/src/domain/` 与 `src/storage/`：需要扩展 `WikiSectionState`、state/cache 持久化和 legacy runtime 识别。
- `crates/wiki-core/src/workflows/`：`init.rs`、`update.rs`、`sync.rs`、`rebuild.rs` 需要改为围绕 managed section kernel 工作；`query.rs` 与 metadata 导出需要读取同步后的 summary 和 page hash。
- `crates/wiki-core/tests/` 与 `scripts/tests/`：需要新增 editable runtime 的单测、集成测试和 e2e 验证，并按 AGENTS 要求补做测试项目集分析。
- Agent 层 JSON IPC 不预期发生 breaking change；如果需要暴露 sync warning，只允许新增字段，不改变现有动作或主状态语义。
