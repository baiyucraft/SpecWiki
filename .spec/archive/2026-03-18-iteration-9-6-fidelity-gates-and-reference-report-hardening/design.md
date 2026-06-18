## Context

`9.6` 的目标不是继续修改 `wiki-core` 主链，而是先把 `storybook + dagger` 专项验收口径收紧到可信状态。当前仓库已经出现几类足以污染后续迭代判断的失真：

- `scripts/collect-reference-project-reports.mjs` 当前的 `overallMatchRate` 只是“每个 reference 页都能找到一个分数 >= 60 的生成页”，允许大量 `many-to-one` 映射仍然显示 `overall = 100%`。
- 同一脚本已经能计算 `reusedGeneratedPages`，但 `collapsedPages` 只统计 `reference 专题被折叠进非专题页` 这一种窄化 note，导致报告会同时出现“`collapsed = 0`”和“同一生成页被多个 reference 页共享映射”的自相矛盾结果。
- `scripts/collect-test-project-analysis.mjs` 仍然读取旧的 `page_context_cache.context.research_result.section_plan`、`page_type=topic/overview/architecture`、`topic_dossier.topic_kind` 口径；而当前 2.0 runtime 写入的是 `knowledge_units / research_cache / wiki_pages` 主线，`page_context_cache` 里只剩最小 `source_ids`。这会稳定产出假负例。
- 当前 `tmp/test/dagger/.wiki` 只有 `.cache/wiki-cache.db`，没有 Markdown 页面和 `wiki.metadata.json`；但 DB 内已经有 `knowledge_units` 与 `research_cache` 数据。这说明报告脚本若不先做 runtime 完整性 gate，就会把“未装配完成的 runtime”误当成“低质量页面”去分析。
- 归档报告还混用了“本次 collect 的 run usage”和“runtime 内历史 cache 统计”，导致 `LLM usage=0` 与 `completed(205)` 这类跨时态数字同时出现，复现性和可追溯性都不够。

这些问题与上游源码借鉴点是吻合的：

- `CodeWiki` 的 [`documentation_generator.py`](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/documentation_generator.py) 把 `get_processing_order()` 和 `build_overview_structure()` 做成显式中间产物，说明“父页消费子页结果”必须可检查，而不是隐含在 prompt 里。
- `deepwiki-rs` 的 [`research/orchestrator.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/research/orchestrator.rs) 和 [`compose/agents/overview_editor.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/compose/agents/overview_editor.rs) 明确区分 research / compose 数据边界；[`outlet/summary_generator.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/outlet/summary_generator.rs) 说明报告应该是正式 outlet，而不是一次性字符串拼接。
- `GitNexus` 的 [`pipeline.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/pipeline.ts) 与 [`call-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/call-processor.ts) 体现的是“厚 facts + 可解释诊断”；其 formatter/test 组织方式说明 9.6 应该把报告指标做成稳定结构与可测试输出。

因此，`9.6` 的最佳落点不是新增一份大脚本，而是在现有专项报告和项目分析脚本之上抽出共享的 runtime 检查、fidelity 分析和快照输出层，把验收输入、指标、诊断和快照原子性统一起来。

## Goals / Non-Goals

**Goals:**

- 为 `storybook + dagger` 专项报告建立正式的 runtime 完整性 gate，禁止对未装配完成的 `.wiki` 做 fidelity 汇总。
- 将 `storybook + dagger` 的正式验收基线固定为“fresh run 生成的原子快照 + warm rerun 稳定性对照”；当前 `tmp/test/*` 现状只用于诊断，不直接作为通过基线。
- 把 `many-to-one reuse`、`skeleton fidelity`、`key source coverage` 升级为正式一级指标，并保留 `reference -> generated` 映射对作为可追溯输入。
- 将 `collect-test-project-analysis` 从旧 `PageContext/topic` 口径迁移到 2.0 的 `knowledge_units / research_cache / wiki_pages` 口径。
- 明确区分“本次运行指标”和“当前 runtime/cache 状态指标”，消除跨时态混算。
- 把报告输出做成原子快照，同时产出 Markdown 摘要和结构化 JSON，供后续 `9.7-9.9` 消费。
- 为 heading normalize、reuse 分类、关键文件提取、runtime gate 和 warm report 稳定性补齐脚本级测试。

**Non-Goals:**

- 不修改 `crates/wiki-core/**` 的 planner / research / compose / assemble 主链。
- 不新增 KnowledgeUnit、evidence、citation、diagram 的 core 数据模型。
- 不调整 provider 行为、budget、checkpoint 语义或页面生成模板。
- 不把 9.6 扩展回 19 项目全量回归的实现门槛；本轮正式验收仍固定为 `storybook + dagger`。

## Decisions

### 决策 1：新增共享的 runtime 体检层，先判定 `.wiki` 是否“可被验收”

当前 `collect-reference-project-reports` 已经有 `assertGeneratedWikiReady()`，但只在自己内部使用；`collect-test-project-analysis` 则仍直接假定 `wiki.metadata.json` 必然存在。9.6 将把 runtime 完整性检查抽成共享模块，建议落在：

- `scripts/testing/wiki-runtime-inspection.mjs`

该模块统一返回 `RuntimeSnapshot`，至少包含：

- `wikiDirExists`
- `metadataExists`
- `markdownPageCount`
- `cacheDbExists`
- `dbCounts`：`knowledge_units / research_cache / page_digests / wiki_pages / pipeline_checkpoint`
- `runtimeState`：`ready | runtime_incomplete | missing`
- `incompleteReason`
- `baselineClass`：`diagnostic_only | acceptance_candidate`

两个消费方式：

- `collect-reference-project-reports`：只有 `runtimeState = ready` 才允许进入 fidelity 对比；否则在项目报告中直接标记 `runtime_incomplete`，并附带 DB 状态摘要。
- `collect-test-project-analysis`：同样先做 gate，避免再因为缺 metadata 或缺 Markdown 直接崩溃。

正式基线规则也在这一层统一：

- 9.6 的验收快照必须来自一次 fresh run（不允许 `--skip-init` 直接复用旧 `tmp/test/*`）；
- warm 稳定性对照必须建立在 fresh run 已完成、且同一 runtime 再次 collect 的前提下；
- 现有 `tmp/test/storybook`、`tmp/test/dagger` 只作为当前问题诊断输入，不直接作为通过基线。

选择这个方案，是因为当前最危险的误差源不是“指标算错 5%”，而是“拿未装配完成的 runtime 去做页面质量结论”。这一层必须先统一。

备选方案：

- 仅在两个脚本里各自补 `existsSync(wiki.metadata.json)`。
  - 否决原因：这会继续复制逻辑，而且无法暴露“research 已完成但 assemble 未完成”的中间状态。

### 决策 2：保留显式 `reference -> generated` 映射对，并以此正式化 reuse/collapse 指标

当前脚本在 `collectProject()` 里已经形成了 `comparisons[]`，但后续只把它用于 `matched/missing` 和低保真 note 统计。9.6 将把 `comparisons[]` 视为正式分析输入，并新增共享分析模块，建议落在：

- `scripts/testing/reference-fidelity.mjs`

该层输出：

- `reuse_count_by_generated_page`
- `reuse_pages`
- `severe_reuse_pages`
- `reuse_overage`
- `collapsed_pairs`
- `top_reuse_offenders`

口径上做三件事：

1. `reuse_*` 作为正式一级指标单独保留，不再只是附注。
2. `collapsed_pages` 改为派生指标：统计所有参与 `many-to-one reuse` 或 `topical demote` 的 reference 页面数。
3. 项目 summary 与 workflow gate 同时展示 `reuse_*` 与 `collapsed_pages`，其中：
   - `reuse_*` 用于诊断“多少生成页承担了过多 reference 主题”
   - `collapsed_pages` 用于维持单页层面的专项门槛与历史可读性

这样做的原因，是当前脚本已经能证明复用现象存在，却没有把它纳入正式门禁。9.6 必须把“报告承认的问题”升级成“报告阻止假通过的条件”。

备选方案：

- 保留 `collapsedPages` 现状，只在 Markdown 报告里多写几行 reuse 提示。
  - 否决原因：这无法修复 `overall=100% + collapsed=0` 的假通过。
- 彻底删除 `collapsed_pages`，只保留 `reuse_*`。
  - 否决原因：`workflow-verification` 当前已经以 `collapsed pages` 作为专项术语，完全删除会让 9.6 变更面不必要地扩大。

### 决策 3：把 skeleton fidelity 与 key source coverage 做成共享的 Markdown 产物分析器

heading normalize、ignore list、文件提及提取、`file://` citation 提取，当前都散落在 `collect-reference-project-reports.mjs`。9.6 将把这些规则收编为正式分析器，仍建议放在：

- `scripts/testing/reference-fidelity.mjs`

分析器职责：

- 从最终 `.wiki/*.md` 和 reference markdown 读取：
  - H2/H3 heading
  - `file://` citation
  - 文件名 mention fallback
  - evidence/citation/mermaid 计数
- 统一做 normalize / ignore
- 输出：
  - `skeleton_score`
  - `skeleton_shortfall_reason`
  - `key_source_coverage`
  - `missing_key_sources`
  - `template_duplication_fingerprint`

其中 `template_duplication_fingerprint` 先作为观察指标，不做本轮 hard gate；它用于定位 storybook 当前“大批同壳页面”的现象。

这样设计，是因为 9.6 的目标不是修改页面，而是把“像不像 reference”变成稳定可重算的 Markdown 分析结果。规则必须集中、共享、可测试。

备选方案：

- 各脚本继续各自用一套 regex。
  - 否决原因：`reference-report` 与 `test-project-analysis` 会继续给出不同结论。

### 决策 4：`collect-test-project-analysis` 改读 2.0 数据面，不再依赖旧 `PageContext/topic` 残余字段

当前项目分析脚本的错，不是“算得不准”，而是“在读已经退场的数据层”。9.6 会把它的研究/页面统计迁到：

- `knowledge_units`
- `knowledge_domains`
- `research_cache`
- `wiki_pages`
- `page_digests`
- `page_drafts`

替代当前的：

- `page_context_cache.context.research_result`
- `page_type`
- `topic_dossier.topic_kind`

映射契约在设计阶段直接钉死，不留到实现时再猜：

- `knowledge_units.id = research_cache.target_id`（`research_type = 'unit'`）
- `knowledge_units.id = page_digests.unit_id`
- `knowledge_units.id = page_drafts.unit_id`
- `knowledge_units.relative_path` 与 `wiki_pages.path` 通过 `.wiki/<relative_path>` 对齐
- `page_drafts.draft.page_id` 与 `page_digests.digest.page_id` 可作为 page 级稳定键

也就是说，项目分析脚本不需要回头依赖 `page_context_cache`，更不需要为了补分析去修改 core 写盘结构。

对应调整：

- `sectionPlanPages` 改为从 `research_cache.result.section_plan` 统计。
- `overview/architecture` 命中改为按 `knowledge_units.unit_type` 或最终页面路径/标题判断。
- topic/decomposition 统计改为按 `knowledge_units` 和共享 `inferDecompositionSignals()`，而不是旧 `TOPIC_PAGE_PATTERN`。

原因很直接：当前 storybook 数据库里已经有 `research_cache=220`、`page_digests=220`、`page_drafts=220`，但脚本还能统计出 `sectionPlanPages=0`，这说明脚本口径已经和 runtime 实现脱节，必须切换数据面。

备选方案：

- 在 `wiki-core` 里补写旧 `page_context_cache` 字段，兼容脚本。
  - 否决原因：这违反 9.6 “不改 core 主链”的边界，也会把旧语义重新养大。

### 决策 5：报告输出改为原子快照，显式区分本次 run 指标与 runtime cache 指标

当前归档 summary 可能被 partial rerun 覆盖，而且 `usage=0` 与 cache stop reasons 混在一起。9.6 将统一输出一个快照对象，建议结构为：

- `reference-project-reports/_snapshot.json`
- `reference-project-reports/_summary.md`
- `reference-project-reports/<project>.md`
- `reference-project-reports/<project>-gap-ledger.md`

`_snapshot.json` 至少包含：

- `generated_at`
- `projects`
- `run_mode`
- `cache_mode`
- `skip_init`
- `source_roots`
- `results[]`

并把指标分成两栏：

- `run_metrics`
  - 本次 collect 的 usage / runMode / elapsed / init result
- `runtime_metrics`
  - 来自 `.wiki/.cache/wiki-cache.db` 的 stop reasons / cache stats / runtime completeness

专项 gate 语义也在这里定死：

- 若 `runtimeState != ready`，项目结果状态直接为 `runtime_incomplete`；
- `runtime_incomplete` 项目不计算 `overall_match_rate`、`reuse_overage`、`median_skeleton`、`median_key_source`；
- summary 中该项目显示为 `N/A + fail-hard`，不参与任何“达到 95%”的通过判断；
- 只有 `runtimeState = ready` 的项目才进入 fidelity denominator。

`_summary.md` 与项目报告只从同一份 `results[]` 渲染，不允许先写部分项目、再就地复写 summary。

之所以这么做，是因为 9.6 的报告要成为后续迭代基线；如果连“这一行数字属于哪次 run”都说不清，基线没有价值。

备选方案：

- 继续保留当前输出，只在文案里补充“本次可能是 warm/reuse”。
  - 否决原因：无法解决 partial rerun 覆写和跨时态混算。

### 决策 6：测试以“指标规则 + 快照一致性 + warm 稳定性”为主，不再只做存在性检查

当前 `scripts/tests` 主要覆盖分发和 e2e，几乎没有脚本级验收逻辑。9.6 将新增脚本测试，建议按三层组织：

- `scripts/tests/reference-fidelity-reporting.test.ts`
  - `reuse/collapse` 分类
  - heading normalize / ignore list
  - key source extraction 优先级
  - `runtime_incomplete` gate
- `scripts/tests/reference-report-snapshot.test.ts`
  - summary 与 per-project report 来自同一批 `results[]`
  - `run_metrics` / `runtime_metrics` 不混栏
- `scripts/tests/reference-report-stability.test.ts`
  - 同一 fixture 重跑两次，`reuse_overage / median_skeleton / median_key_source` 波动在阈值内

原因：

- `GitNexus` 的 formatter 之所以可信，是因为输出格式和解释逻辑都有单测。
- 9.6 的核心交付物不是页面，而是“可信验收器”；测试必须直接覆盖验收器本身。

备选方案：

- 只在 UniSpec tasks 里要求手工 warm run 两次。
  - 否决原因：没有脚本级断言，回归很容易静默退化。

## Risks / Trade-offs

- [风险：`runtime_incomplete` gate 会让当前 dagger 等样本直接从“低分”变成“不可评估”] → 这是有意为之；报告必须先区分“未生成完成”和“生成质量差”，并在 summary 中单独列出 incomplete 状态。
- [风险：reuse/collapse 口径收紧后，短期内 summary 会比 9.5 更差看] → 接受这一短期回撤；9.6 的目标就是把假通过打掉，而不是制造更好看的数字。
- [风险：将 `collect-test-project-analysis` 迁到 2.0 数据面后，旧报表字段会变化] → 在脚本输出中明确标注字段语义升级，并保留必要的迁移注释，不保留旧统计逻辑本身。
- [风险：模板重复页诊断可能误伤真正结构相似的 docs-backed 页面] → 本轮先作为观察指标进入报告 top offenders，不纳入 hard gate。
- [风险：新增共享模块后，短期内脚本改动面较大] → 通过“先抽共享分析器、再替换两个脚本消费”的顺序推进，并补充脚本级测试锁边界。

## Migration Plan

1. 新增 `scripts/testing/wiki-runtime-inspection.mjs`，统一 runtime 快照与完整性判定。
2. 新增或抽取 `scripts/testing/reference-fidelity.mjs`，集中 heading、citation、key-source、reuse/collapse 分析逻辑。
3. 先改 `collect-reference-project-reports.mjs`：接入 runtime gate、显式映射对、reuse/skeleton/key-source、新快照输出。
4. 再改 `collect-test-project-analysis.mjs`：切到 2.0 数据面，并复用相同的 runtime gate 与 Markdown 分析逻辑。
5. 补脚本级测试与 warm 稳定性测试。
6. 用 `storybook + dagger` 生成新的专项快照，作为 `9.7-9.9` 的验收基线。

## Open Questions

- `template_duplication_fingerprint` 本轮是否仅做观察项，还是需要为 storybook/dagger 增加一个软阈值。
- `collect-test-project-analysis` 最终是否继续保留部分旧字段名称，还是直接切成新的 2.0 命名，减少误导。
