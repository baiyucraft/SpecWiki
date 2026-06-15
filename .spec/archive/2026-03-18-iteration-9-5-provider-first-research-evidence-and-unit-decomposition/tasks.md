## 1. 验收基线与 95% 目标锁定

- [x] 1.1 以当前 `storybook + dagger` 的 reference 报告为基线，明确本轮统一评分口径：`overall_match_rate = matched / reference`，并同步输出 `generated/reference/matched/missing/extra`、page collapse、citation、diagram、decomposition 命中
- [x] 1.2 为 `storybook + dagger` 各自建立“95% 差距台账”，把未达标项固定拆成 `missing pages`、`collapsed pages`、`low-fidelity matched pages` 三类，并把每类差距映射回对应的 KnowledgeUnit、ResearchProfile、renderer contract 或报告口径
- [x] 1.3 更新 `scripts/collect-reference-project-reports.mjs` 与相关分析脚本，使最终报告能直接给出 `storybook` 与 `dagger` 对 reference 的总匹配率、三类差距明细和 `extra generated pages`，并把 `>=95%` 作为本轮通过条件
- [x] 1.4 固定本轮测试范围为 `storybook + dagger`，所有任务完成判定、专项报告和变更说明都只以这两个样本为准

## 2. Storybook 页树收敛到 95%

- [x] 2.1 先用 `storybook` 的 reference 差距台账锁定高频缺口页族，优先把当前被压到 `项目概述`、`ConceptGuide`、大模块页中的 API / config / addon / framework / troubleshooting / advanced feature 主题映射到独立 KnowledgeUnit 与 `DecompositionProfile`
- [x] 2.2 按“先补 missing、再拆 collapse、最后补 low-fidelity”顺序修 `storybook`：缺页优先改 planner / decomposition，折叠优先改 leaf-first parent-consume-child，命中但内容弱优先改 research / citation / diagram / renderer
- [x] 2.3 让 `storybook` 的 leaf-first research / compose 真正消费 docs anchors、public API surface、config surface 和 child digest，确保新增页面不是空壳页，而是能承接真实证据和 child summary
- [x] 2.4 反复跑 `storybook` 专项 init / lifecycle / reference 报告；若总体分数上升但 `extra generated pages` 明显增加，则回收拆分阈值；若仓库同时存在原始英文 docs 语料与本地化/派生 docs 语料，则 planner 必须优先后者，避免最终 `.wiki/` 中继续大批保留英文 raw docs 文件名，直到 `storybook` 在不靠增页堆分的前提下稳定保持 `>=95%`

## 3. Dagger 页树收敛到 95%

- [x] 3.1 先用 `dagger` 的 reference 差距台账锁定高频缺口页族，把 runtime / compiler / API / testing / example / tutorial / Android / Hilt 主题从大模块页中解耦为独立 KnowledgeUnit 与 `DecompositionProfile`
- [x] 3.2 按“先补 missing、再拆 collapse、最后补 low-fidelity”顺序修 `dagger`：缺页优先改 planner / decomposition，折叠优先改 runtime graph + child digest 消费，命中但内容弱优先改 research / citation / diagram / renderer
- [x] 3.3 让 `dagger` 的 leaf-first research / compose 真正消费 runtime graph、compiler pipeline、public API surface、tests/examples 与 child digest，消除 reference 中被折叠进 `dagger-runtime`、`dagger.md`、`javatests.md` 的主题
- [x] 3.4 反复跑 `dagger` 专项 init / lifecycle / reference 报告；若总体分数上升但 `extra generated pages` 明显增加，则回收拆分阈值；若仓库同时存在原始英文 docs 语料与本地化/派生 docs 语料，则 planner 必须优先后者，避免最终 `.wiki/` 中继续大批保留英文 raw docs 文件名，直到 `dagger` 在不靠增页堆分的前提下稳定保持 `>=95%`

## 4. Provider-First Research 与成页质量补齐

- [x] 4.1 在 `crates/wiki-core/src/workflows/init.rs`、`rebuild.rs`、`update.rs` 引入统一的 runtime provider 选择逻辑，正式 workflow 不再默认绑定 `StructuralResearchProvider`
- [x] 4.2 扩展 `crates/wiki-core/src/generation/research_engine.rs`、`page_render.rs` 与相关 runtime contract，让 `system / domain / unit` research 真正成为 provider-first 主驱动，并由父页正式消费子页 digest
- [x] 4.3 围绕 `storybook + dagger` 的专项缺口补齐 `DecompositionProfile`、ResearchProfile、child digest、checkpoint / cache 相关测试，确保 research-first / leaf-first 主链稳定可复用
- [x] 4.4 把 LLM hard-stop 预算收敛成正式 steering contract：`max_calls`、page research turn budget 与相关硬限制必须可配置，且默认值不能再按小仓库调优，必须对 `storybook + dagger` 这类页数更高的仓库给出更高的默认预算；预算提升还必须真正体现在 page research 可分配配额上，而不是只改配置字段却仍被内部固定阈值卡死
- [x] 4.5 把停止条件正式收成三层 runtime contract：`workflow stop` 负责一次 `init / rebuild / update` 何时成功结束、何时因 checkpoint / fatal io / 全局硬限制提前返回；`unit research stop` 负责单个 `KnowledgeUnit` 在拿到最小完整 `PageResearchResult` 且连续两轮无有效增量时停止；`llm turn stop` 负责单轮 provider loop 在 `final / stalled / budget_exhausted / failed` 间明确分流，而不是继续只用 `Ok(None)` 表示所有非成功结束
- [x] 4.6 在 `crates/wiki-core/src/llm/mod.rs`、`crates/wiki-core/src/domain/research.rs`、`crates/wiki-core/src/workflows/research_provider.rs` 引入显式的 `ResearchStopReason` / `PageResearchSessionResult` 一类结构，至少区分 `Completed / NoFurtherToolCalls / NoMeaningfulDelta / TurnBudgetExhausted / CallBudgetRejected / ProviderError / InvalidOutput`，并让 `recent_turns / tool_calls / delta_evidence_count / delta_section_count / delta_diagram_count / child_digest_delta` 成为可记录的 session 指标
- [x] 4.7 在单页 provider research loop 中补齐“有效增量”判定与停止门槛：把 `新 section summary / 新 evidence cluster 或 citation path / 新 diagram suggestion / 新 child digest consumption` 统一收成 delta 规则；若连续两轮没有新增有效增量，则即使未打满 `max_turns` 也必须停止，并以显式 `stop_reason` 返回
- [x] 4.8 在 `page_render.rs`、checkpoint/cache 与 debug trace 中把 stop reason 向 workflow 层抬升，确保外层能区分“真实完成”“预算打断”“结果无效”“provider 失败”“卡住后主动停止”，并以此驱动 resume / report / 后续收敛，而不是把所有非成功分支都折叠成静默 fallback

## 5. Citation / Diagram / 最终 Markdown 对标

- [x] 5.1 扩展 `crates/wiki-core/src/domain/research.rs`、`compose_engine.rs`，让 section-scoped citation digest、evidence block 和 diagram draft 成为正式 compose 输出，而不是只停留在中间缓存
- [x] 5.2 调整 `crates/wiki-core/src/generation/renderer.rs`，把 citation / evidence / Mermaid 以最终 Markdown 可统计 contract 落盘，使 reference 报告从 `.wiki/*.md` 就能统计到真实 citation 与图表达
- [x] 5.3 以 `storybook + dagger` 为样本反复修正 citation / diagram 落页质量，直到最终报告中的 citation、diagram 与正文结构不再成为 95% 对标的主要短板
- [x] 5.4 以 docs-backed 页面为重点补齐最终 Markdown 章节骨架 contract：当 reference 页面表现为 `cite -> 目录 -> 简介 -> 项目结构 -> 核心组件 -> 架构总览 -> 详细组件分析 -> 依赖关系分析 -> 性能考量 -> 故障排查指南 -> 结论 -> 附录` 这类稳定章节序列时，生成页必须优先保留或收敛到同层级章节结构，不再退回英文原始 docs 标题或泛化模板章节
- [x] 5.5 在专项报告脚本与最终 `_optimization-notes.md` 中新增 stop reason 观测口径，至少输出 `budget_stopped_pages / stalled_pages / invalid_output_pages / provider_failed_pages`，确保后续判断页面质量时能区分“内容本身差”和“research 提前被截断”

## 6. 专项回归、注释与收尾

- [x] 6.1 为 `storybook + dagger` 的 planner、research、renderer、lifecycle 和 reference 对比补齐必要测试，确保每个行为变化都有对应断言，尤其覆盖“本地化 docs corpus 优先于英文 raw docs corpus”和“reference 主章节骨架稳定落页”这两类约束
- [x] 6.2 按 [.wiki/02-开发指南/00-代码注释规范.md](E:/project/!byAI/spec-wiki/.wiki/02-开发指南/00-代码注释规范.md) 审核本轮新增或重构代码注释，并修正不符合规范的注释
- [x] 6.3 输出最终 `storybook + dagger` 专项报告、`_optimization-notes.md` 和结论摘要，明确记录两者对 reference 的最终匹配率、三类差距收敛过程和剩余尾差
- [ ] 6.4 仅当 `storybook` 与 `dagger` 同时满足 `overall_match_rate >= 95%`、高频 collapse 不再是主问题、citation / diagram 不再是主短板、`extra generated pages` 未明显失控、英文 raw docs 文件名不再大批残留、docs-backed 页面主章节骨架不再明显偏离 reference 时，才将本轮标记为完成
