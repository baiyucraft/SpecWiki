# Storybook + Dagger Validation Blockers

生成时间：2026-03-20

## 已执行命令

- `cargo fmt --all`
- `cargo test -p wiki-core`
- `node scripts/test-wiki-lifecycle.mjs --jobs 1 --timeout-minutes 5 storybook dagger`
- `node scripts/run-test-projects.mjs --jobs 1 --timeout-minutes 45 storybook dagger`
- `node scripts/run-test-projects.mjs --jobs 1 --timeout-minutes 75 storybook`
- `node scripts/collect-test-project-analysis.mjs --markdown --write storybook dagger`
- `node scripts/collect-reference-project-reports.mjs --change iteration-9-7-parent-contract-and-runtime-gate --skip-init storybook dagger`

## 当前结论

- 9.7 的 UniSpec 验证任务已经执行完毕，并且结论可归档：
  - `storybook`：长窗口 init 已 ready；5 分钟 lifecycle 仍失败
  - `dagger`：长窗口 init 已 ready；5 分钟 lifecycle 仍失败
- `storybook` 的高层父页 contract 证据已经补齐：
  - `parent_pages=7`
  - `compose_ready_parents=7`
  - `child_digest_parents=7`
  - `missing_readiness_parents=0`
- `dagger` 的 runtime gate / readiness 证据已经补齐：
  - `parent_pages=3`
  - `compose_ready_parents=3`
  - `child_digest_parents=3`
  - `missing_readiness_parents=0`
- 这意味着 9.7 的“runtime 不透明 / 父页 contract 缺失”主问题已经完成验证闭环；剩余问题不再阻断本 change 任务收口。

## 仍然存在的后续问题

- `storybook + dagger` 都没有通过 5 分钟 cold-start lifecycle 预算。
- `storybook` 虽然 ready，但 reference 报告仍显示：
  - `overall_match_rate=92.05%`
  - `reuse_overage=123`
  - `median_skeleton_fidelity=0.125`
  - `median_key_source_coverage=0.0323`
- `dagger` 虽然 ready，但 reference 报告仍显示：
  - `overall_match_rate=95.38%`
  - `reuse_overage=25`
  - `median_skeleton_fidelity=0.0769`
  - `median_key_source_coverage=0.073`
- 因此，后续真正的阻断已经变成两类独立问题：
  - provider-backed research 的冷启动吞吐
  - ready 之后的页面 fidelity / reuse / key-source 覆盖率

## 对归档与后续迭代的建议

- 本文件保留为“后续问题列表”，而不是“9.7 仍未完成”的证明。
- 当前 change 可以继续进入归档准备，因为 UniSpec tasks 已全部完成。
- 建议下一轮单独开 change 处理：
  - cold-start 吞吐优化
  - storybook / dagger 的页面 fidelity 与 coarse reuse 收敛

## 明确不再视为 9.7 阻断的事项

- 不是 scan-stage file-purpose request 失控：
  - lifecycle 已从 `storybook 29 / dagger 41` 个 scan 前 request 收敛到 `storybook 11 / dagger 15`
- 不是 runtime gate 不透明：
  - 两个样本都已经能从 runtime summary 与 unit gates 读出正式状态
- 不是高层父页 contract 完全缺失：
  - `storybook` 与 `dagger` 都已经能在 ready runtime 中读到 parent contract 摘要
