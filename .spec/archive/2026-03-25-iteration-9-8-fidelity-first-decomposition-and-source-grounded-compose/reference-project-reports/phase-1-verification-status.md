# 5.1 Phase-1 验证状态

## 2026-03-24 临时搁置结论

- 当前迭代先停在“`storybook phase-1 验证使能` 已补齐，但 `9.8 fidelity` 主收敛尚未完成”的状态。
- 已经确认，`storybook` 的主 blocker 不是 planner 本身，而是 `init` 中断后不能从 incomplete runtime 续跑；之前每次重试都会删掉 `.wiki/`，导致 research/checkpoint 无法接续。
- 本轮已经补齐通用 runtime/resume 语义，而不是样本特判：
  - `init + preserve/refresh` 在“无 metadata、无 markdown、但已有 research/gates 的 incomplete runtime”上不再无条件删除 `.wiki/`
  - resume 时不再无条件清空 `unit_runtime_gates`
  - `runtime_summary` 会根据现有 gates 重建，避免二次续跑时重复累加计数
  - `init` 与 `page_render` 都增加了 `workflow_action` 同动作校验，避免把旧的 `rebuild/update` runtime 误续到 `init`
- `run-test-projects.mjs` 也已补“失败后 preserve-resume”的窄策略：
  - 仅对 timeout 或 retryable provider error 放行
  - 且必须命中真实 `runtime_incomplete` 快照，并且 `runtime_summary.runtime_state` 属于 `researching / compose_pending / compose_complete / interrupted`
- 定向测试现状：
  - Rust 定向 resume/gate/summary 测试通过
  - TS 定向 orchestration/resume policy 测试通过
  - `cargo test -p wiki-core` 仍有 1 个与本轮 resume 修复不直接相关的独立失败：`generation::research_engine::tests::type_topic_focus_promotes_story_and_preview_families_over_generic_contracts`
- `wiki.dev.yaml` 当前 provider 现状：
  - 默认模型 `proxy/gpt-5.2-codex` 已能成功返回最小 completion
  - 同 provider 下的 `gpt-5.2` 仍不稳定，当前测试返回 `502`
- 因此，当前可以认定：
  - `init/resume` 这条验证使能链已经补上
  - 但 `5.1.3` 还不能算通过
  - `9.8` 真正需要收敛的主矛盾仍停在 `research key-source drift` 与 `compose section grounding fidelity`

## 已完成

- `5.1.1` 已实现：
  - `knowledge_planner.rs` 新增基于结构化 `path/docs topic evidence` 的 leaf scope refinement。
  - `Types API`、`主题系统概览`、`自定义主题开发`、`颜色和字体系统` 不再只靠原始 `keywords -> matching_signal()` 平铺命中。
  - 对启用 refinement 的 leaf，引入了条件性 `generic contract saturation penalty` 与 `TopicScopeRefinement` collapse guard。
  - `dedup_units_by_relative_path()` 改为在同路径冲突时优先保留非 docs-backed 的 signal/structural unit，避免 docs-backed 同名页抢占 signal-family 叶子页。
  - 对 docs-heavy 主题页，planner 会按 docs grounding 额外补充 report 级候选，而不再被 `ConfigReference` 域内默认 candidate pool 限死。
- `5.1.2` 已实现并通过：
  - `types_api_prefers_topic_spine_over_generic_public_contracts`
  - `theme_signal_units_keep_distinct_topic_spines`
  - `dagger_api_units_keep_topic_spines_under_scope_refinement`
  - `cargo test -p wiki-core` 全绿。

## Storybook fresh init 结果

- 本轮两次 fresh `storybook init` 都没有在脚本时限内完成：
  - 第一次：默认 `60m` 超时。
  - 第二次：`--timeout-minutes 120` 后，仍在 `researching` 阶段超时。
- 这次超时发生在 provider-backed research 长时间执行期间，不是：
  - Rust 编译错误
  - planner 测试失败
  - page render 崩溃
- fresh run 超时后，`tmp/test/storybook/.wiki/` 未形成可验收的完整 runtime，`collect-reference-project-reports --skip-init storybook` 返回：
  - `status=runtime_incomplete`
  - `pipelineRuntimeState=researching`
  - `generatedPageCount=0`
  - 四项 fidelity 指标不可用

## 当前对 storybook 的判断

- 代码层面：
  - planner Phase-1 的通用抽象已落地，且 synthetic regression 已覆盖 `public types` 与 `theme/theming/a11y` 边界。
- 验证层面：
  - 由于 fresh init 未完成，当前**不能**宣称 `5.1.3` 通过。
  - 这一轮还无法给出新的 `reuse_overage / missing / collapsed` 对比值。
- 当前 blocker 已经不是“planner 改不动”，而是：
  - `storybook` 的 provider-backed init 吞吐不足，fresh runtime 无法在验收窗口内完成。

## Dagger 最小 anti-overfit 抽查

- `node scripts/run-test-projects.mjs --timeout-minutes 60 dagger` 成功完成：
  - `83` 个页面
  - runtime `completed`
- 但最小实产物抽查显示，实际页面仍有需要继续跟踪的风险：
  - [运行时API](E:/project/!byAI/spec-wiki/tmp/test/dagger/.wiki/API-参考/运行时API.md)
  - [编译时API](E:/project/!byAI/spec-wiki/tmp/test/dagger/.wiki/API-参考/编译时API.md)
  - [Hilt-API](E:/project/!byAI/spec-wiki/tmp/test/dagger/.wiki/API-参考/Hilt-API.md)
- 这三页当前仍共享一组非常接近的泛化入口：
  - `dagger-runtime/main/java/dagger/Module.java`
  - `dagger-runtime/main/java/dagger/Component.java`
  - `dagger-android-processor/main/java/dagger/android/processor/AndroidProcessor.java`
- 说明：
  - synthetic anti-overfit 测试表明 planner 抽象本身没有直接伤到 dagger 的 leaf scope 规则。
  - 但真实 dagger 产物里，API 页面的最终 source grounding 仍没有稳定拉开 `runtime / compiler / hilt` 边界。
- 同时，`collect-reference-project-reports --skip-init dagger` 本轮返回了 `generatedPageCount=0`，与 `tmp/test/dagger/.wiki/` 里实际存在 `83` 个 Markdown 页面不一致，说明 report/runtime 映射也需要额外检查。

## 结论

- `5.1.1`、`5.1.2` 已完成。
- `5.1.3` 仍未完成，原因分两类：
  - `storybook`：fresh init 超时，无法产出有效 fidelity 指标。
  - `dagger`：最小实产物抽查提示真实 API topic boundary 仍未拉开，不能算已通过 anti-overfit。

## 下一步建议

1. 先处理 `storybook fresh init` 的吞吐问题，至少要让 Phase-1 后的 init 能完整落盘并生成 report。
2. 在 `dagger` 上补一轮真实 runtime/source grounding 检查，确认为什么 synthetic planner test 通过，但实际 `运行时API / 编译时API / Hilt API` 仍共享同一批泛入口。
3. 若 report 继续出现 `generatedPageCount=0` 但 `.wiki/*.md` 已存在，需要优先排查 reporting/runtime 映射链，而不是误判成 planner regression。
