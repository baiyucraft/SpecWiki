## Context

当前仓库已经完成到迭代 4 的运行时主链：

- `Repository Scan -> ModuleTree -> Page Planner -> Wiki Assembler` 主路径已经稳定。
- `WikiState`、`ChangePlan`、page context / generation cache、section 粒度渲染都已落地。
- 迭代 4 已修复 scanner 噪声过滤（fixture 排除、嵌套仓库排除、非代码产物目录排除）、单文件模块抑制、module kind 多维分类和关键源码选择信号。
- 最新归档 [`test-project-analysis.md`](E:/project/!byAI/spec-wiki/.spec/archive/2026-03-09-iteration-4-scanner-hierarchy-quality/test-project-analysis.md) 的结论：页面结构显著收敛（spec-wiki 从 36 页降至 7 页），噪声问题基本消除，module kind 分类准确。

但当前实现距离”Editable Wiki Runtime”还有一个关键断层：

- `crates/wiki-core/src/generation/sections.rs` 虽然已经输出 section draft，但所有 section 都是 `managed = true`。
- `crates/wiki-core/src/generation/renderer.rs` 仍然只输出普通 Markdown 标题，没有任何 managed marker 或页面结构协议。
- `crates/wiki-core/src/workflows/sync.rs` 只重新计算整页 `content_hash`；它不会解析 Markdown，不会更新 section 状态，不会更新 summary，也不会把人工新增区段纳入状态层。
- 因此 `update` 与 `rebuild` 仍只能按”整页生成结果”覆盖页面。用户一旦直接改 `.wiki/*.md`，下一次源码变化就会把人工内容冲掉。

这个缺口已经可以通过一个最小实验仓库复现：

- 初始化后，在 `项目概述.md` 中插入 `## 手工笔记`，并改写已有简介。
- 执行 `sync` 后，`WikiState.pages[*].content_hash` 会更新，但 `summary` 仍是旧值，`sections[*].content_hash` 仍是生成态旧值。
- 随后修改源码并执行 `update`，runtime 会把 `项目概述.md` 整页重写，手工 section 被直接覆盖。

设计约束来自四个方向：

- 项目边界：继续遵守 `Repository Scan -> ModuleTree -> Page Planner -> Wiki Assembler` 主路径，不能绕过 planner 或直接让 `sync` 手工改 metadata。
- 参考实现：
  - `deepwiki-rs` 的 `workflow / context / memory / outlet` 分层说明 parse/merge 应该是运行时与输出层之间的明确子层，而不是零散塞进 workflow。
  - `CodeWiki` 的递归模块生成方式说明”稳定树结构 + 自底向上组装”仍应保持，不要因为 editable runtime 回退到整页字符串拼接。
  - `deepwiki-open` 只提供消费层 wiki 结构与 cache 边界参考，不直接作为 core merge runtime 模板。
- `.wiki/06-设计文档/00-总体设计.md` 边界：本迭代要解决 managed section、用户内容保留和 `sync` 回写，不进入 Explanation Layer，也不趁机把 planner 质量 backlog 全部并进来。
- 仓库现状：迭代 4 已修复 fixture 误提升、单文件模块、.spec 被提升为模块、module kind 误分类等问题。当前基线干净，可以专注于 editable runtime。

## Goals / Non-Goals

**Goals:**

- 定义稳定的 managed section Markdown 协议，让 runtime 能显式识别哪些区段属于生成层、哪些区段属于用户内容。
- 引入页面解析与 merge 内核，支持从当前 `.wiki/*.md` 恢复 managed/user 区段顺序，而不是只拿整页字符串做 hash。
- 扩展 `WikiState`，让它能同时表达 generated managed state、当前磁盘内容状态和 user section 锚点。
- 把 `sync` 升级为 parser-first workflow：回写 page/section 状态、summary、metadata，而不是只改 page hash。
- 让 `update` 和 `rebuild` 在页面 identity 稳定时只替换 managed sections，保留已同步的 user sections。
- 为现有迭代 3 页面提供 best-effort legacy migration 路径，避免第一次进入迭代 5 就强制丢弃旧页面里的人工内容。
- 补齐 editable runtime 测试、测试项目集分析和 `.wiki/02-开发指南/00-代码注释规范.md` 合规检查。
- 在 tasks 设计和测试阶段都对 `.wiki/06-设计文档/00-总体设计.md § 测试项目集` 全量执行 `init` 分析，并在存在 reference 时做结构与 metadata 对照。

**Non-Goals:**

- 不实现跨页面的人工内容自动迁移；页面 path / page id 发生变化时，用户内容不承诺自动搬运到新页面。
- 不引入 LLM、RAG、TOON、Diagram 或 explanation layer 能力。
- 不做富文本编辑器、前端可视化编辑或宿主 UI 协议设计。
- 不改变 Agent 层动作集合和主状态语义；如需补充 sync warning，只允许新增字段，不做 breaking change。
- 不在本迭代做 planner composition 质量优化（如页面合并策略、overview 页面内容丰富度等）；这些留给后续迭代。

## Decisions

### 决策 1：新增独立的 `wiki-managed-section-kernel`，而不是把 editable 逻辑散落到 `sync/update` 里

本迭代引入独立的页面 parse / merge 内核，例如：

- `ParsedWikiPage`
- `ManagedSectionBlock`
- `UserSectionBlock`
- `PageMergePlan`
- `PageParseMode`（`managed_markers / legacy_headings`）

它们的职责是：

- 从当前磁盘上的 Markdown 页面恢复“有序区段序列”
- 区分 runtime 托管区段和用户区段
- 为用户区段建立相对于 managed section 的锚点
- 把“新生成的 managed sections”与“已同步的 user sections”重新组装成最终页面

这样 `sync`、`update`、`rebuild` 共享同一套页面语义，而不是各自手写字符串规则。

备选方案：

- 方案 A：继续只在 workflow 里比较整页字符串
  - 否决原因：无法表达区段边界，也无法支持 managed-only 替换。
- 方案 B：直接做三方 Markdown diff/merge
  - 否决原因：当前页面结构还比较规则，先做显式 marker + block merge 更可控，范围也更适合迭代 5。

### 决策 2：managed section 使用 HTML comment marker 包裹，而不是依赖标题文本做隐式识别

页面文件中的 runtime 托管区段改成显式 marker 格式，例如：

```md
<!-- wiki:managed:start id=section-ba2b37bf1d81 title="简介" version=1 -->
## 简介

由 codebuddy-wiki 自动生成的仓库概览。
<!-- wiki:managed:end id=section-ba2b37bf1d81 -->
```

选择这类 marker 的原因：

- 对 Markdown 渲染基本透明，不会污染用户阅读体验。
- 解析简单，且不依赖用户保留原始标题文本。
- 能直接携带稳定 `section_id` 和格式版本，便于 state/cache/schema 演进。

边界定义：

- `# 页面标题` 仍视为 runtime 托管内容，不承诺保留用户对一级标题的修改。
- 用户区段不需要显式 marker；任何位于 managed markers 之间、且不属于 managed block 的内容，都视为 user section。

备选方案：

- 方案 A：完全靠 `## 标题` 名字匹配 managed section
  - 否决原因：用户改标题、重复标题或插入同名 section 时，边界会立刻失稳。
- 方案 B：使用 YAML frontmatter 统一存整页结构
  - 否决原因：会显著改变页面可读性，也会把页面从“普通 Markdown 文档”变成“协议文件”。

### 决策 3：`WikiSectionState` 同时记录 observed hash 与 generated hash，区分用户内容和 managed drift

现有 `WikiSectionState` 只有一个 `content_hash`，且默认代表“生成内容 hash”。迭代 5 之后需要区分三种情况：

- managed section 且磁盘内容未被用户改动
- managed section 仍是托管区段，但用户手工改了其中内容（managed drift）
- user section，本身不对应生成器输出

因此状态层扩展为：

- `managed`
- `content_hash`
  - 当前磁盘上这个区段的实际内容 hash
- `generated_content_hash: Option<String>`
  - 仅对 managed section 存在，表示最近一次生成器输出的内容 hash
- `anchor_before_section_id: Option<String>`
- `anchor_after_section_id: Option<String>`
  - 仅对 user section 有意义，用于 merge 时恢复插入位置

`WikiPageState` 保持：

- `content_hash` 仍表示整页实际磁盘内容 hash
- `summary` 改为“当前页面已同步内容”的摘要，而不是只保留生成时摘要

这样 `sync` 可以把当前磁盘事实写回状态层，同时不丢掉“这段内容原本是不是生成出来的”这一关键信号。

备选方案：

- 方案 A：对 managed section 继续只保存一个 hash
  - 否决原因：一旦用户手工改了托管区段，就无法区分“生成内容变了”还是“磁盘内容被改了”。
- 方案 B：把 user section 单独放到另一张表/另一份文件
  - 否决原因：merge 仍需要页面内顺序和邻接关系，拆开后实现反而更复杂。

### 决策 4：`sync` 改为 parser-first，并负责刷新 summary / metadata，而不是只刷新整页 hash

新的 `sync` 流程：

1. 读取 `WikiState`、页面文件和必要的 page generation cache
2. 解析页面为 managed/user blocks
3. 用解析结果回写 `WikiPageState.sections`
4. 重新计算整页 `content_hash`
5. 基于当前页面内容重新提取 `summary`
6. 通过 `MetadataMapper` 导出更新后的 metadata

关键语义：

- 用户新增的 section 或段落，必须变成 `managed = false` 的 user section 状态。
- 用户改动 managed block 内容时，该区段仍然必须保持 `managed = true`，但 `content_hash != generated_content_hash`。
- user section 的 `source_ids / relation_ids` 为空；query 仍然优先依赖结构化事实，不把用户区段错误当成源码 provenance。

选择 parser-first 的原因是，editable runtime 的事实来源不再只是“最新生成结果”，还包括“当前磁盘页长什么样”。只有 `sync` 显式接管这层事实，后续 workflow 才能一致。

备选方案：

- 方案 A：让 query 直接读取磁盘 Markdown，绕过 sync
  - 否决原因：会让 query、metadata 和 state 各自看到不同事实，破坏状态内核边界。
- 方案 B：`sync` 继续只刷 page hash，把 summary 留给下次 update
  - 否决原因：实验已经证明这会让 metadata/query 长时间停留在过期状态。

### 决策 5：`update` 与 `rebuild` 都改为“重生 managed section，复用已同步 user section”；保留范围仅限 page identity 稳定

新的 merge 语义：

- `init`
  - 只写 managed sections，不写 user sections
- `update`
  - 基于新的 `PageContext` / `SectionDraft` 生成新 managed sections
  - 从旧页面状态中取出 user sections
  - 按锚点把 user sections 插回新页面
  - 仅在页面 identity 稳定（同一 `page_id`）时保留 user content
- `rebuild`
  - 仍忽略旧 generation cache 和旧 dirty state
  - 但对仍然存在的同一 `page_id` 页面，继续复用已同步的 user sections

锚点插入规则：

- 如果 user section 同时存在 `before` / `after` 两侧锚点，优先插回原位置。
- 只剩一侧锚点时，挂到最近仍存在的 managed section 前/后。
- 锚点都不存在但页面仍存在时，退化为追加到页面末尾，并发出 warning。

明确不承诺的边界：

- 如果页面 path / page id 因结构变化而消失或替换为新页面，本迭代不自动把 user content 跨页面迁移。

备选方案：

- 方案 A：只有 `update` 保留 user section，`rebuild` 一律覆盖
  - 否决原因：会把 `rebuild` 变成高风险操作，和“可持续维护的文档层”目标相冲突。
- 方案 B：要求用户显式写 `<!-- wiki:user -->` marker 才能保留
  - 否决原因：协议负担太重，不适合作为第一版 editable runtime。

### 决策 6：对迭代 3 的 legacy 页面提供 best-effort heading migration，而不是要求用户先手动重建

当前所有已生成页面都没有 managed marker。为了避免升级到迭代 5 后直接丢掉既有手工内容，页面解析支持两种模式：

- `managed_markers`
  - 优先模式，marker 完整时直接解析
- `legacy_headings`
  - 当页面还没有 marker 时，按 `WikiState` 或 `page_type` 已知的稳定 section 标题集合做 best-effort 迁移

legacy 迁移规则：

- 能唯一匹配到预期 managed 标题（例如 `简介 / 项目事实 / 关键信息`）时：
  - 对应区段转为 managed sections
  - 中间新增的区段或段落转为 user sections
- 无法唯一匹配时：
  - `sync` 必须给出 warning
  - 不把该页面误判为已完成 editable migration

这样可以把“当前 iteration 3 产物”平滑接到 iteration 5，而不是要求用户先删掉 `.wiki/` 再来。

备选方案：

- 方案 A：要求用户升级后立即 `rebuild`
  - 否决原因：已有手工编辑会被直接抹掉。
- 方案 B：只支持 marker 模式，不支持 legacy 页面
  - 否决原因：这等价于把迭代 5 限制成“只对新生成页面有效”。

### 决策 7：runtime 必须容忍未知页面和未知区段，不假设 planner 输出完美

迭代 4 已修复 scanner 噪声过滤、单文件模块抑制、module kind 分类等核心 planner 质量问题。但 planner composition 层面仍可能存在页面合并策略不够理想、overview 内容偏薄等情况，这些不在本迭代范围内。

因此本次 change 的处理原则是：

- `sync` / `update` / `rebuild` 必须能容忍：
  - 页面里存在未知 user section
  - `.wiki/` 中存在不在当前 `WikiState` 里的额外页面
  - 某些页面还处于 legacy 模式（无 managed marker）
- 不因为 planner 输出变化而让 editable runtime 崩溃或丢失用户内容

## Risks / Trade-offs

- [页面 path / page id 变化时无法自动迁移 user content] → 在 spec 和 task 中明确“same-page preservation only”，避免误承诺；后续如需要，再单开 orphan/rename 迁移 change。
- [legacy 页面被用户改坏标题后，best-effort migration 可能失败] → parser 必须显式给 warning，不得假装迁移成功；实现期优先保护“可识别的正常页面”。
- [managed marker 会让原始 Markdown 看起来稍微更技术化] → 使用 HTML comments，尽量保持渲染与阅读层无噪声。
- [sync 现在要解析用户手写 Markdown，复杂度明显上升] → 只支持非常小的语法面：managed markers、H2 标题和区段顺序，不引入通用 Markdown AST 大改。
- [planner composition 层面仍可能有页面合并策略不理想的情况] → 不在本 change 修复，但保证未知区段/额外页面不会让 merge runtime 崩溃。

## Migration Plan

1. 先引入 managed marker 与 parse / merge kernel，并扩展 `WikiSectionState` / cache schema。
2. 再改 `init` / `update` / `rebuild` 的页面写盘路径，确保新生成页面都采用 `managed_sections_v1` 格式。
3. 然后改 `sync`，支持 marker 模式和 legacy heading 模式，把 summary / metadata / section 状态一并回写。
4. 最后补齐测试与测试项目集分析，对 `.wiki/06-设计文档/00-总体设计.md § 测试项目集` 的完整项目集执行 `init`，并特别关注：
   - `spec-wiki` 自身的页面结构与 editable runtime 兼容性
   - `aLocal` 的真实运行时页面结构
   - 所有存在 reference 的项目（aLocal、axum、bat、chi、cobra、dagger、pinia、restaurant-app、storybook、zustand），与参考产物的页面结构和 metadata 字段差异
   - legacy 页面迁移和 hand-written section 保留

回滚策略：

- 如果 parse / merge kernel 在实现期不稳定，可以保留迭代 3 的整页重写路径作为临时 fallback，但不能在“宣称 editable runtime 已完成”的前提下默认启用。
- 页面 marker 采用增量兼容策略：旧页面先走 legacy parse，新页面走 marker parse，因此回滚时不会强依赖数据库迁移或外部依赖。

## Open Questions

- 是否要把 `sync` 发现的 managed drift / migration warning 作为结构化 JSON 字段返回给 Agent，而不只是日志输出。
- 对“页面被删除或 page id 改变后遗留的 user content”是否需要在本迭代就提供 orphan 备份目录；当前设计先不纳入必做范围。
