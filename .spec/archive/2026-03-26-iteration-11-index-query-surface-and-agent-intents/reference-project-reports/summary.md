# iteration-11 汇总报告

## 已完成结论
- `wiki-index` 已形成正式 query surface：`auto / symbol_lookup / source_lookup / module_lookup / entrypoint_lookup / callers / callees / impact_slice`。
- `wiki-runtime` 的 `run_query(repo_root, term)` 已切到 `wiki-index::query(auto)`，`matched_symbols / matched_sources / matched_modules / matched_symbol_edges` 由 index 投影生成。
- facts snapshot 已从 `write_state_with_symbol_graph*` 中拆出，并前移到 `init / rebuild / update` 的 `knowledge_planning` 之前。
- readiness 已与 downstream 分离：在 facts snapshot 仍在但 metadata/state 缺失时，`status.facts_ready` 与 index-first query 仍可工作；在 DB/事实快照整体缺失时，query 会显式返回 `index not ready`。
- scoped update 现在只局部 parse/resolve dirty workset，但 graph-derived analysis 会基于全量 persisted graph 重算，因此 `communities / processes` 不再在局部更新后掉到 `0`。

## 与 iteration-11.5 的复核结论
- 本轮实施没有并入 `agents/codebuddy`、transport/JSON IPC、runtime consumption profile 等 11.5 范围改动。
- `iteration-11.5` 继续只承担 Provider/CodeBuddy 消费 `wiki-runtime` substrate 的能力设计，不并入本 change。
- 复核结果支持当前分界：11 负责 substrate/readiness/query surface，11.5 再负责宿主消费与场景化入口。

## 自动化验证
- 通过：`cargo test -p wiki-index -- --nocapture`
- 通过：`cargo test -p wiki-runtime --test runtime -- --nocapture`
- 通过：`cargo test -p wiki-runtime update_marks_storybook_family_parent_pages_dirty_when_family_child_sources_change -- --nocapture`
- 通过：`cargo test -p wiki-runtime update_rebuilds_topic_page_and_parent_pages_when_topic_sources_change -- --nocapture`
- 通过：`pnpm vitest run scripts/tests/run-test-projects-resume-policy.test.ts scripts/tests/streaming-protocol.test.ts`
- 通过：`node scripts/run-test-projects.mjs storybook dagger`
- 通过：`node scripts/test-wiki-lifecycle.mjs storybook --phase full --run-mode warm --jobs 1 --no-build --timeout-minutes 180`
- 通过：`node scripts/test-wiki-lifecycle.mjs dagger --phase full --run-mode warm --jobs 1 --no-build --timeout-minutes 180`

## 验收结论
- `storybook` 与 `dagger` 都已完成 lifecycle 样本验证；`init -> status -> sync -> query -> update -> rebuild` 下的 index readiness 与 query 没有回退。
- iteration-11 的实现、样本验证与注释检查现已闭环，可以进入归档准备。

## 注释检查
- 本轮新增的 `scripts/testing/init-resume.mjs`、`wiki-runtime/update.rs` 相关调整以及测试补充都保持了当前仓库约定的注释粒度，没有引入新的注释风格偏差。
