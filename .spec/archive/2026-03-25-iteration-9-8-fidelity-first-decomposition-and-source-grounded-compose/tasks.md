## 1. Planner Signal 收敛

- [x] 1.1 重构 `crates/wiki-core/src/generation/knowledge_planner.rs`，把样本化关键词/路径特判收回到通用 `repo_archetype_signals`、surface clusters、`leaf decomposition policy` 与 `collapse guard` 抽象。
- [x] 1.2 为 `KnowledgeUnit` 拆分补充可诊断的 signal bundle / collapse guard 输出，确保 missing/collapsed page 能回溯到 planner 决策。
- [x] 1.3 为 docs-heavy、runtime-heavy、compiler-heavy 场景补 Rust 测试，覆盖 missing page、粗粒度折叠与 identity 稳定性。

## 2. Research 与 Compose Contract

- [x] 2.1 升级 `crates/wiki-core/src/generation/research_engine.rs` 与 `crates/wiki-core/src/workflows/research_provider.rs`，在 runtime 已选择 provider-backed 路径时，让 `system / domain / unit` research 一致产出 `skeleton_profile`、`key_source_clusters` 与可供 compose 直接消费的高层/低层 contract。
- [x] 2.2 重构 `crates/wiki-core/src/generation/compose_engine.rs`，引入统一 `ComposePageContract`，显式覆盖 `child digest / citation digest / diagram digest / readiness`，去掉高层页固定骨架主导路径，确保 `Overview / Architecture / DomainIndex / config_surface parent unit` 与普通 unit 走同一 contract 家族。
- [x] 2.3 为 `section_grounding_refs`、key-source grounding、section-level 落页、parent/child rollup 与高层页 contract 补 Rust 测试，验证最终 Markdown 不再主要依赖固定模板或轻量摘要。

## 3. Runtime 与报告映射

- [x] 3.1 扩展 runtime/cache 持久化面，记录 `planned_key_sources`、`grounded_key_sources`、`section_grounding_refs`、`skeleton_profile` 或等价诊断字段，且不混用 `.wiki/*.md`、`wiki.metadata.json` 与 `.cache/**` 三层职责。
- [x] 3.2 更新 `scripts/collect-reference-project-reports.mjs`、`scripts/testing/wiki-runtime-inspection.mjs` 与相关脚本测试，把 gap ledger 映射到 `planner / research / compose` 主链断点，并明确区分 fresh init 基线与 warm/skip-init 诊断口径。
- [x] 3.3 为 reference 报告与 workflow 验证补工作区级测试，覆盖 storybook/dagger 的 missing/reuse/skeleton/key-source 指标映射与 fresh/warm 口径分离。

## 4. 验证与注释检查

- [x] 4.1 运行 `cargo test -p wiki-core`、相关 `scripts/tests/*.test.ts`，确认 planner、research、compose、reporting 的行为变化都有对应测试。
- [x] 4.2 运行 `node scripts/run-test-projects.mjs storybook dagger`，检查 `tmp/test/storybook/.wiki/` 与 `tmp/test/dagger/.wiki/` 的 runtime 和页面产物，并输出本 change 的专项分析报告。
- [ ] 4.3 运行 `node scripts/test-wiki-lifecycle.mjs storybook dagger`，验证 init → status → sync → query → update → rebuild 全链路。（按当前用户范围，本轮 defer，不作为 init 专项验收前置）
- [x] 4.4 单独执行一轮注释规范检查，确认本轮涉及的 Rust/TS 代码与测试注释符合 [.wiki/02-开发指南/00-代码注释规范.md](E:/project/!byAI/spec-wiki/.wiki/02-开发指南/00-代码注释规范.md)。

## 5. Storybook-first 后续收敛计划

> `storybook` 仅作为 phase-1 主矛盾定位样本，用于压测 docs-heavy / docs-anchor-rich 场景下的通用 `decomposition`、`topic grounding` 与 `source-grounded compose` contract；任何改动都必须能用抽象信号、typed surfaces 或 contract 解释，不能以仓库名、reference 标题、固定目录模式或高频文件名分支成立。`dagger` 不再承担并行主收敛任务，但从 phase-1 起持续承担 anti-overfit 抽查职责。

### 5.1 Phase-1：Planner 优先，先打 `reuse_overage` / `missing-collapsed`

- [x] 5.1.1 继续收紧 `crates/wiki-core/src/generation/knowledge_planner.rs` 的 `unit topic grounding`、`typed surface cluster` 与 `collapse guard`，重点解决 storybook 中 unit scope 过泛导致的 many-to-one reuse，不接受新增基于仓库名、reference 标题、目录白名单或高频文件名的显式/隐式特判。
- [x] 5.1.2 为 planner 增补 Rust 测试，覆盖 storybook 当前高频 collapse/reuse 症状对应的通用抽象：`docs-backed api/config/example/troubleshooting` 的独立 unit 保留、`theme/theming/a11y` 邻近主题的 unit topic 边界、`public types` 泛合同页不过度吞并真实主题主干。
- [ ] 5.1.3 只运行 `node scripts/run-test-projects.mjs storybook` 做 phase-1 主验证，确认 `reuse_overage`、`missing`、`collapsed` 至少出现一轮可解释下降；同一轮必须对 `dagger` 做最小 anti-overfit 抽查（可只跑 report/runtime 对照或定向页面检查），确认没有因为 storybook 收敛而扩大 runtime/compiler/testing/hilt 等非 docs-heavy 主题的 collapse/reuse。

### 5.2 Phase-2：Research 优先，先打 `key_source_coverage` / `planned->grounded drift`

- [ ] 5.2.1 继续收紧 `crates/wiki-core/src/generation/research_engine.rs` 与 `crates/wiki-core/src/workflows/research_provider.rs` 的 `topic spine selection`，让 unit-level source selection 更强消费 planner grounding，压制 `types.ts/public-types.ts/typings.d.ts/preset.ts/preview.tsx` 一类泛合同文件主导正文，但必须以通用 `generic contract saturation penalty` 或等价 typed policy 表达，不接受 storybook 专有文件族降权分支。
- [ ] 5.2.2 为 research 增补 Rust 测试，覆盖 `planned_key_sources -> grounded_key_sources` 的通用约束，验证 key source 选择能稳定命中真实主题主干，而不是只在候选池里做弱 rerank。
- [ ] 5.2.3 再次只跑 `storybook` 主验证，重点检查 `median_key_source_coverage`、`pagesWithGroundingGap` 与代表性页面（如 `Types API`、颜色/字体相关主题）的 planned/grounded key sources 是否同步改善；同一轮继续对 `dagger` 做 anti-overfit 抽查，确认 API/runtime/compiler/testing 主题没有因 docs-heavy 优化而退化。

### 5.3 Phase-3：Compose 保真，先打 `skeleton_fidelity` / `grounding_gap_pages`

- [ ] 5.3.1 检查并收紧 `crates/wiki-core/src/generation/compose_engine.rs` 中 `section_grounding_refs -> final markdown` 的保真链，确认 research 已选中的 key sources、section grounding 与 child digest 真正进入 section-level explanation，而不是停留在 evidence block、统一模板段落或 child rollup 壳层。
- [ ] 5.3.2 为 compose 增补 Rust 测试，覆盖 `section identity`、`section grounding refs`、`child digest rollup` 与最终 Markdown 的对应关系，避免“有 contract 但 renderer 没保住骨架/正文”的假阳性。
- [ ] 5.3.3 继续只跑 `storybook` 主验证，确认 `median_skeleton_fidelity`、`pagesWithGroundingGap` 与解释层正文密度出现可解释改善；同一轮继续对 `dagger` 做 anti-overfit 抽查，确认 parent-consume-child 与高层页 contract 没有被 docs-backed 页面优化破坏。

### 5.4 Phase-4：诊断面补强，但不优先于主链收敛

- [ ] 5.4.1 在 `scripts/testing/wiki-runtime-inspection.mjs`、`scripts/collect-reference-project-reports.mjs` 与等价 runtime 面中补最小必要的 planner diagnostics 暴露（如 `decomposition_profile`、`planner_signal_bundles`、`collapse_guard.reason`），让 `reuse/missing/collapsed` 可以更直接回映射到 planner contract，而不是只靠报告猜测。
- [ ] 5.4.2 补 `provider_stop_reason/session_stats -> generated page/unit` 的 join 诊断，避免后续把 provider 可用性问题误判成 planner/research/compose 主链问题。
- [ ] 5.4.3 保持当前专项阶段不新增更多 fidelity 指标；任何诊断增强都必须直接服务于已有 `reuse_overage / skeleton_fidelity / key_source_coverage / grounding_gap_pages` 四类主矛盾定位。

### 5.5 Phase-5：第二阶段交叉验证

- [ ] 5.5.1 仅当 storybook 在上述四类主指标上出现连续、可解释改善后，再恢复 `node scripts/run-test-projects.mjs storybook dagger` 的双样本专项验证，把 `dagger` 从 anti-overfit 抽查提升为正式 cross-check 样本。
- [ ] 5.5.2 恢复双样本专项验证时，要求所有收益都能映射回 `unit decomposition / source selection / section grounding` 三段 contract 的可解释变化；如果某项收益只能体现在 storybook、却无法通过 dagger 抽查或 contract 解释，则不计为有效收敛。
- [ ] 5.5.3 `node scripts/test-wiki-lifecycle.mjs` 仍放在上述专项收敛之后，再决定是否先跑 `storybook` 单样本生命周期验证，随后再补 `dagger`。
