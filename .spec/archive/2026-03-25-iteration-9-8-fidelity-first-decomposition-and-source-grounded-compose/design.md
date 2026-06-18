## Context

当前 `wiki-core` 的正式生成主链已经切到 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble`，真实入口位于 `crates/wiki-core/src/workflows/init.rs` 与 `crates/wiki-core/src/workflows/page_render.rs`。但这条主链仍处于“2.0 数据面已接上、1.x/9.x 旧语义仍在托底”的混合态：

- `knowledge_planner.rs` 已经输出 `KnowledgeDomain / KnowledgeUnit / KnowledgeTree`，但仍残留大量样本化关键词和目录信号，`storybook`、`dagger` 相关经验尚未完全收回通用抽象。
- `research_provider.rs` 目前只有 `unit research` 在 provider 直连时走 provider-backed merge，`system/domain research` 仍是 structural-only；一旦不是 provider 直连路径，正式 workflow 仍可能整体退回 `StructuralResearchProvider`。
- `compose_engine.rs` 对 `Overview / Architecture / DomainIndex / config_surface parent unit` 仍保留固定骨架或轻量摘要分支，且父页与部分 docs-backed 页面仍偏向“摘要拼装 + 通用段落”而非真正的 source-grounded section plan。
- `init.rs` / `rebuild.rs` 在拿到 `PageDraft` 后，仍通过旧的 `PlannedPage / PageContext / assemble_state` 桥接回写 runtime，导致 2.0 页面 contract 与旧 runtime/cache 语义并存。

`9.7` 已经把 runtime gate 和 parent contract 做到可诊断，但 `storybook + dagger` 的最终产物仍未通过验收：

- `storybook`: `overall=92.05% / reuse_overage=123 / median_skeleton=0.13 / median_key_source=0.03`
- `dagger`: `overall=95.38% / reuse_overage=25 / median_skeleton=0.08 / median_key_source=0.07`

这些差距与当前源码结构逐项对应：

- `knowledge_planner.rs` 的拆分规则还不够通用，导致 missing page 与 many-to-one reuse。
- `research_engine.rs` / `research_provider.rs` 的研究结果还没有稳定产出“reference 风格章节骨架 + 关键源码簇”。
- `compose_engine.rs` 还没有把 `section_plan / key sources / citation clusters / child rollup` 压成统一的最终页面 contract。

上游参考实现提供了明确的借鉴边界：

- `deepwiki-rs` 值得借鉴的是 `StepForwardAgent` 式声明式输入边界，以及 compose 只消费 research 结果，而不是它固定的 C4 页面集合。
- `CodeWiki` 值得借鉴的是后序处理与 parent-consume-child 的执行纪律，而不是它的 `module tree -> 文档树` 抽象，更不能照搬“父页读子页全文 Markdown”。
- `GitNexus` 值得借鉴的是厚 facts 层的分块、索引、置信度与图分析工程实现，而不是它的 graph/query 产品形态。

## Goals / Non-Goals

**Goals:**

- 让 `KnowledgeUnit` 拆分真正建立在通用 `repo_archetype_signals`、surface clusters 和 collapse guard 上，消除样本化关键词表对页面集合的主导作用。
- 在当前 runtime 已选择 provider-backed research 路径时，让 `system / domain / unit` 都进入同一条 source-grounded 主链，稳定产出 `section_plan`、`key source clusters`、`evidence clusters` 与 `skeleton profile`。
- 让所有正式页面都由统一的 compose contract 生成，取消 `Overview / Architecture / DomainIndex / config_surface parent unit` 的固定骨架主导路径。
- 让 reference/rdb 分析可以把 `missing / reuse / skeleton / key-source` 四类缺口直接映射回 planner、research、compose 三段断点。
- 保持当前 2.0 边界：`Facts -> Knowledge Planning -> Research -> Compose -> Assemble`，不重新长出旁路生成链。

**Non-Goals:**

- 不回退到 `module / topic / family` 旧页面语义重新主导规划。
- 不为了 `storybook` 或 `dagger` 新增仓库名、reference 标题或固定目录结构的专有 planner / renderer 分支。
- 不把 `CodeWiki` 的 child full-doc 注入或 `deepwiki-rs` 的固定页面族直接搬进 core。
- 不在 `9.8` 中优先解决 cold-start research 吞吐和 provider 并发调度问题；这些属于后续 `9.9` 的性能主线。
- 不在 `9.8` 中单独重写现有的 provider availability、Agent bridge 与 deterministic fallback 总策略；本轮只收紧会进入最终页面的 fidelity contract。
- 不改变 `.wiki/*.md + wiki.metadata.json + .cache/` 的 runtime 三层分工。

## Decisions

### 决策 1：把 planner 从“关键词散点规则”收敛为“typed surface clusters + decomposition policy”

`knowledge_planner.rs` 仍然在大量路径、文件名、关键词表上直接下结论，这会导致两类问题：

- 结论很难证明是通用规则，容易继续向 `storybook / dagger` 偏斜。
- planner 很难回答“为什么这个页该独立、为什么那个页该合并”，因此也无法稳定驱动 gap ledger 回写。

`9.8` 将把 planner 的输入收敛为一组稳定的 typed surfaces：

- `docs anchors`
- `public api surfaces`
- `config surfaces`
- `runtime/compiler/testing/example/troubleshooting/integration signals`
- `graph/process/community facts`
- `repo archetype signals`

在这些 surfaces 之上，引入显式的 `DecompositionSignalBundle` 和 `CollapseGuard` 概念：

- `DecompositionSignalBundle`
  - 负责把单个候选单元的证据来源、主题边界、关键文件簇和父子关系输入收敛为稳定对象。
- `CollapseGuard`
  - 负责判断某类候选是否必须独立成 unit、是否允许回收进父节点、以及何时判定为 severe reuse 风险。

这样做的结果是：

- storybook 的 `API / config / framework / addon / troubleshooting / example` 不再共享同一类泛化 docs 单元。
- dagger 的 `runtime / compiler / android / hilt / testing / example / troubleshooting` 不再继续默认折叠回大模块页。

备选方案：

- 继续在现有 planner 上追加更多关键词。
  - 否决原因：这会继续扩大样本耦合，无法满足 `KnowledgeUnit` 通用性约束。
- 完全交给 LLM 重新规划页面集合。
  - 否决原因：这会破坏 deterministic identity、稳定路径和 runtime diff 能力。

### 决策 2：在 provider-backed 路径已被 runtime 选中时，把 `system / domain / unit` research 收敛为同一条 source-grounded 主链

当前实现只让 `unit research` 在 provider 直连时增强，`system/domain` 仍 structural-only。这会造成两个直接后果：

- 高层页的章节骨架和关键源码覆盖天然偏弱。
- `section_plan` 仍更像 leaf/unit 层的“补丁增强”，而不是三级 research 共同驱动的正式输入。

`9.8` 将把 research contract 统一为：

- `SystemResearch`
  - 必须产出项目级 `skeleton profile`、高层关键源码簇、跨 domain 关系和顶层 evidence clusters。
- `DomainResearch`
  - 必须产出 domain 级 `section_plan seed`、domain key sources、domain-specific evidence clusters 与 domain-level child rollup policy。
- `UnitResearch`
  - 必须产出最终 compose 可直接消费的 `section_plan`、`key source clusters`、`evidence_clusters`、`diagram_suggestions` 和 `child slots`。

这里收紧的是“已进入 provider-backed 主链时，system/domain 不能再长期 structural-only”，不是在 `9.8` 里重新定义现有 runtime 的 provider availability、Agent bridge 或 deterministic fallback 全局策略。换句话说：

- 如果 runtime 已选择 provider-backed research 路径，`system / domain / unit` 都必须进入同一条 source-grounded contract。
- 如果 runtime 仍走当前保留的 bridge / fallback 路径，本轮不单独改写其可用性政策；这部分继续由现有 runtime/verification 能力约束管理。

备选方案：

- 保持 `system/domain` structural-only，只增强 `unit research`。
  - 否决原因：这会让 overview/domain index 继续天然弱于 leaf 页，无法解决高层骨架和 source grounding 问题。
- 直接在 `9.8` 一次性改写全部 provider/fallback 政策。
  - 否决原因：这会把 fidelity 收敛和 runtime availability 政策混成一轮，超出本变更边界。
- 只在 compose 阶段补强高层页 prompt。
  - 否决原因：compose 不应承担 research 缺失后的补洞职责。

### 决策 3：统一 `ComposePageContract`，移除高层页固定骨架主导路径

`compose_engine.rs` 当前对 `Overview / Architecture / DomainIndex` 仍有明显固定骨架分支，`config_surface` parent unit 也还没有进入同一套父页 contract，这与 2.0 的 “compose 只消费 research + child rollup” 相冲突。`9.8` 将统一正式页面输入为 `ComposePageContract`：

- `section_plan`
- `section_grounding_refs`
- `key_source_clusters`
- `evidence_clusters`
- `diagram_suggestions`
- `child_digest_rollup`
- `child_section_citation_digest`
- `child_diagram_digest`
- `child_key_sources`
- `child_readiness`
- `skeleton_profile`

其中：

- leaf 页只使用自己的 `section_plan + key_source_clusters + evidence_clusters`
- parent 页在同一 contract 中附加 `child_digest_rollup + child_section_citation_digest + child_diagram_digest + child_key_sources + child_readiness`
- `Overview / Architecture / DomainIndex / config_surface parent unit` 不再走独立的固定 section 模板分支，而是复用同一 contract

为了避免页面风格发散，`skeleton_profile` 由 `SystemResearch / DomainResearch` 提供 seed，但最终以 `UnitResearch` 中的 unit-level `skeleton_profile` 作为 compose 的唯一正式输入；renderer 不再自行补骨架。renderer 只负责：

- 稳定 section identity
- citation / evidence / mermaid 落盘
- Markdown 结构装配

备选方案：

- 保留 `compose_system_page()` / `compose_index_page()`，只增加更多 child digest。
  - 否决原因：问题不只是 child digest 不够，而是高层页仍由固定骨架主导。
- 让 renderer 根据 page type 自动补骨架。
  - 否决原因：这会把页面质量问题重新隐藏到 renderer fallback 中。

### 决策 4：把 key-source coverage 前移为 research/compose 的正式 contract，并显式回连到 section 级 evidence identity

当前 `reference-fidelity-reporting` 能测出 key-source coverage 很低，但 core 内部没有正式对象表达“这一页必须围绕哪些关键文件来写”。`9.8` 将把它前移到正式数据面：

- planner 负责为每个 unit 选出候选 `source clusters`
- research 负责把候选 clusters 收敛为 `key_source_clusters`
- compose 负责把这些 clusters 通过 `section_grounding_refs` 显式分配到 section
- runtime/cache 至少保留每页的 `planned_key_sources`、`grounded_key_sources` 和 `section_grounding_refs`

这样报告脚本不再只是被动发现低 coverage，而是可以直接回答：

- planner 是否没拆出对应 unit
- research 是否没选中正确 key sources
- compose 是否没把 key sources 通过 section-grounded evidence 落到正文 section

备选方案：

- 继续只在 reference 报告里计算 key-source coverage。
  - 否决原因：只能看到结果，无法把问题映射回主链阶段。

### 决策 5：`storybook + dagger` 继续作为验收样本，但结论必须沉淀为通用 fidelity policy

`9.8` 的验收仍以 `storybook + dagger` 为专项样本，因为当前 gap ledger 已经精确暴露问题链；但样本只能用于抽象规则，不能成为 core 分支条件。具体验收将围绕四类目标：

- `missing pages`
- `reuse overage`
- `skeleton fidelity`
- `key source coverage`

这四类目标会分别映射回：

- planner
- planner + compose
- research + compose
- research + compose + evidence layer

备选方案：

- 直接把 `storybook + dagger` 的高频标题抄成规则表。
  - 否决原因：违反 2.0 的通用抽象边界。
- 在 9.8 本轮就回到 19 项目全量做同权验收。
  - 否决原因：这会稀释当前 `storybook + dagger` 的主矛盾，不利于本轮围绕 fidelity 断点收敛。

## Risks / Trade-offs

- [风险：planner 抽象升级后短期可能引发 unit 路径波动] → 用稳定 signal bundle 和 deterministic collapse guard 驱动 identity，先锁 `unit id / relative_path / parent_unit_id` 的导出规则。
- [风险：provider-backed 路径下的全层 research 会增加调用成本] → 通过 surface cluster 压缩、child rollup 复用和 research cache versioning 控制调用规模。
- [风险：移除高层固定骨架后，页面章节可能短期抖动] → 用 `skeleton_profile + stable section_key` 约束章节身份，避免自由生成章节集合。
- [风险：source grounding 前移后，compose 可能暴露更多“无足够关键源码”的失败] → 这是有意暴露；对正式 runtime 来说，先可诊断、后再谈兜底。
- [风险：旧 `PageContext / PlannedPage / assemble_state` bridge 仍在，会拖慢 2.0 收敛] → 9.8 不直接删除整套 bridge，但要求它只做 runtime 兼容托底，不能再决定正式页面语义。

## Migration Plan

1. 先把 `knowledge_planner` 中的样本化关键词与路径特判收回到 typed signal / collapse guard 抽象，补对应 Rust 测试。
2. 再升级 `research_provider` 与 `research_engine`，让 `system / domain / unit` 都产出可供 compose 直接消费的 source-grounded research contract。
3. 重写 `compose_engine` 的高层页路径，统一到 `ComposePageContract`，收缩固定骨架分支。
4. 扩展 runtime/cache 和报告脚本，让 `planned_key_sources / grounded_key_sources / section_grounding_refs / skeleton_profile` 可被读取和诊断。
5. 先对 `storybook + dagger` 跑专项 init / report / lifecycle，对 gap ledger 做前后对比。
6. 专项收敛后，再决定是否进入 `9.9` 的 provider 吞吐与 cold-start 性能优化。

## Open Questions

- `planned_key_sources / grounded_key_sources / section_grounding_refs` 应落在 `page_context_cache`、`page_digests` 还是新的 runtime 表；当前倾向优先复用已有 runtime 面，避免新增过多平行表。
