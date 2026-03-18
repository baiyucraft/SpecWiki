## 1. 共享 Fidelity 分析基础设施

- [x] 1.1 在 `scripts/testing/` 下抽出共享的 runtime 体检模块，统一检查 `.wiki/*.md`、`wiki.metadata.json`、`.cache/wiki-cache.db` 与关键 SQLite 表计数，并输出 `ready / runtime_incomplete / missing` 状态
- [x] 1.2 在 `scripts/testing/` 下抽出共享的 reference fidelity 分析模块，统一保留 `reference -> generated` 映射对，并计算 `reuse_count(page)`、`reuse_pages`、`severe_reuse_pages`、`reuse_overage`、`skeleton_score`、`key source coverage`
- [x] 1.3 为 heading normalize、ignore list、`file://` citation 提取、文件提及 fallback、many-to-one reuse 分类与 `runtime_incomplete` gate 补脚本级测试

## 2. 升级 Reference 专项报告

- [x] 2.1 重构 `scripts/collect-reference-project-reports.mjs`，接入共享 runtime gate 与 fidelity 分析层，修复“`overall=100%` 但 reuse 严重 / `collapsed=0`”的失真
- [x] 2.2 让专项报告同时输出结构化快照与 Markdown 报告，明确区分 `run_metrics` 与 `runtime_metrics`，并保证 `_summary.md`、项目报告和 gap ledger 来自同一批 `results[]`
- [x] 2.3 在专项报告中新增 `top reuse offenders`、`skeleton lowest pages`、`key-source lowest pages` 与 `symptom -> metric -> offending pages -> contract hypothesis` gap ledger
- [x] 2.4 为 `storybook + dagger` 增加 warm report 重跑与稳定性判定，记录 `reuse_overage`、`median skeleton fidelity` 与 `median key source coverage` 的波动

## 3. 升级项目分析脚本到 2.0 数据面

- [x] 3.1 重构 `scripts/collect-test-project-analysis.mjs`，改为读取 `knowledge_units`、`knowledge_domains`、`research_cache`、`wiki_pages`、`page_digests` 等 2.0 runtime 数据
- [x] 3.2 删除脚本内对旧 `page_context_cache.context.research_result`、`topic_dossier` 和旧 `page_type/topic` 语义的依赖，统一到 KnowledgeUnit / decomposition 口径
- [x] 3.3 让项目分析脚本与 reference 报告脚本共享同一套 runtime gate、页面类型/分解信号与 Markdown fidelity 规则，避免两个脚本继续给出互相矛盾的结论
- [x] 3.4 为项目分析脚本补回归测试，覆盖 `storybook` 完整 runtime、`dagger` runtime incomplete 与 2.0 research 统计不再误报为 0 的场景

## 4. 专项验证、注释与收尾

- [x] 4.1 运行 `storybook + dagger` 的专项报告生成流程，产出新的 `_snapshot.json`、`_summary.md`、项目报告与 gap ledger，确认 9.6 门禁能稳定回答页数、reuse、骨架与关键文件四个问题
- [x] 4.2 运行 warm report 重跑验证，确认 `reuse_overage / median skeleton fidelity / median key source coverage` 波动可控，能作为 9.7-9.9 的验收基线
- [x] 4.3 按 [COMMENTING.md](E:/project/!byAI/spec-wiki/COMMENTING.md) 检查本轮新增或重构脚本中的注释，确保中文摘要、参数和关键流程注释符合仓库规范
- [x] 4.4 汇总最终 9.6 基线结论，明确记录哪些样本 `ready`、哪些 `runtime_incomplete`、哪些 reuse/skeleton/key-source 指标仍是后续 9.7-9.9 的主要收敛入口


