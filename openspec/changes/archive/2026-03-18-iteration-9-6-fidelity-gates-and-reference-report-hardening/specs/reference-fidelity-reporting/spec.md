## ADDED Requirements

### Requirement: reference fidelity 报告必须先验证 runtime 完整性
系统 MUST 在读取最终 `.wiki/*.md`、`wiki.metadata.json` 与 runtime SQLite 状态后，先判定目标样本是否处于可验收的完整 runtime 状态。若 runtime 仅生成了 `.cache/wiki-cache.db` 或仅停留在 knowledge/research 阶段，报告 MUST 将该样本标记为 `runtime_incomplete`，并禁止把它纳入页面 fidelity 汇总结论。

#### Scenario: 样本仓库只生成了 cache 而没有最终 Markdown
- **WHEN** 报告脚本发现 `.wiki/.cache/wiki-cache.db` 存在，但 `.wiki/*.md` 或 `wiki.metadata.json` 缺失
- **THEN** 报告 MUST 输出 `runtime_incomplete`
- **THEN** 报告 MUST 同时给出 `knowledge_units / research_cache / wiki_pages / pipeline_checkpoint` 等 runtime 摘要
- **THEN** 报告 MUST 不得继续输出该样本的 `overall_match_rate`、`reuse`、`skeleton fidelity` 或 `key source coverage` 汇总值

### Requirement: reference fidelity 报告必须保留显式映射与 many-to-one reuse 指标
系统 MUST 将最终 `reference -> generated` 匹配对保留为正式分析输入，并基于这些映射输出 `reuse_count(page)`、`reuse_pages`、`severe_reuse_pages`、`reuse_overage` 与 `top reuse offenders`。报告 MUST 不得再把 many-to-one reuse 仅作为附加说明，而必须将其作为正式诊断指标。

#### Scenario: 多个 reference 页面映射到同一生成页
- **WHEN** 报告脚本为 `storybook` 或 `dagger` 生成专项报告，且多个 reference 页面命中同一生成页
- **THEN** 报告 MUST 显式输出该生成页的 `reuse_count`
- **THEN** 报告 MUST 将高复用页面列入 `top reuse offenders`
- **THEN** 报告 MUST 计算项目级 `reuse_pages`、`severe_reuse_pages` 与 `reuse_overage`

### Requirement: reference fidelity 报告必须从最终 Markdown 计算 skeleton fidelity 与 key source coverage
系统 MUST 直接从最终 `.wiki/*.md` 与 reference Markdown 提取 heading skeleton、`file://` citation 和文件提及，计算 `skeleton fidelity` 与 `key source coverage`。heading normalize 过程 MUST 忽略 `目录`、`附录`、`章节结构图` 与空标题 cite preamble，保证指标不会被 renderer 自动块误伤。

#### Scenario: docs-backed 页面存在稳定章节骨架与关键文件
- **WHEN** 报告脚本对 docs-backed 页面执行最终 Markdown 对比
- **THEN** 报告 MUST 计算该页的 `skeleton_score`
- **THEN** 报告 MUST 输出该页缺失的关键文件集合或 coverage 比例
- **THEN** 报告 MUST 能列出 `skeleton lowest pages` 与 `key-source lowest pages`

### Requirement: reference fidelity 报告快照必须区分本次运行指标与 runtime 指标
系统 MUST 将本次 collect 的 run 信息与 runtime SQLite 中读取到的缓存/停止原因信息分开记录。`run_metrics` 只能描述本次执行的 `run_mode / cache_mode / usage / elapsed`，`runtime_metrics` 只能描述当前 `.wiki` runtime 的 `stop reasons / cache counts / runtime completeness`，二者 MUST NOT 混算。

#### Scenario: 复用 warm runtime 生成专项报告
- **WHEN** 报告脚本以 `--skip-init`、`warm` 或 `reuse` 模式生成报告
- **THEN** 报告 MUST 将本次 collect 的 usage 统计单独放入 `run_metrics`
- **THEN** 报告 MUST 将 `research_cache` 中的 stop reason 和 delta 统计单独放入 `runtime_metrics`
- **THEN** 报告 MUST 不得将 `run_metrics` 的 `usage=0` 与 `runtime_metrics` 的历史 stop reason 拼成同一口径结论

### Requirement: reference fidelity 报告必须以原子快照写出
系统 MUST 先构造单次 collect 的完整 `results[]` 快照，再从同一批结果渲染 `_summary.md`、项目报告、gap ledger 和结构化快照文件。报告 MUST 记录 `generated_at`、`projects`、`run_mode`、`cache_mode` 与输入路径，避免 partial rerun 覆写 summary 后与单项目报告脱节。

#### Scenario: 子集 rerun 不得污染 summary 与单项目报告的一致性
- **WHEN** 报告脚本只对 `storybook` 或 `dagger` 子集重跑
- **THEN** 新写出的 summary MUST 明确记录当前快照只包含哪些项目
- **THEN** summary 与单项目报告 MUST 来自同一份 `results[]` 快照
- **THEN** 报告 MUST 不得在未更新对应项目报告的前提下单独覆写 summary
