## 1. Formal Artifact Contract

- [x] 1.1 在 `wiki-model / wiki-knowledge / wiki-runtime` 中定义 `.wiki/.knowledge/derived/**` 与 `.wiki/.knowledge/runtime/**` 的最小对象模型、序列化结构和 `recovery-manifest` contract
- [x] 1.2 调整正式 `init / update / rebuild` 写盘链路，把 `knowledge_domains`、`knowledge_units`、`knowledge_tree`、research summaries、`page_digests`、runtime gates/readiness 摘要落到 `.wiki/.knowledge/**`
- [x] 1.3 确保本轮 `.wiki/.knowledge/**` 只覆盖 `derived/** + runtime/**` 最小正式对象集，`declared/**` 仅保留占位，且 `page_drafts`、`llm_cache`、临时 session state 不泄漏进正式产物

## 2. SQLite Mirror And Restore

- [x] 2.1 调整 `sqlite-cache-storage` 与相关 store trait，明确 SQLite knowledge 表是本地 mirror / cache / rebuild target，而不是唯一正式 knowledge truth
- [x] 2.2 实现从 `.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json` 重建 `.wiki/.cache/**` 的 cold restore 路径
- [x] 2.3 确保 restore 只做 cache rebuild 和 readiness 复算，不隐式触发 planning / research / compose / assemble，也不把 restore 伪装成 full `init`

## 3. Restored Runtime Consumption

- [x] 3.1 调整 `status` 对恢复态 runtime 的消费，明确区分 `ready`、`stale`、`needs_update` 与 blocker
- [x] 3.2 调整当前 `query` 入口对恢复态 cache 的消费，保证缺失 `.cache` 时不必先 full `init`，同时不把“可查询”误表述成“完整 runtime ready”
- [x] 3.3 收口 restore 失败、snapshot 锚点不一致和正式产物缺失时的推荐动作与错误摘要

## 4. Tests And Verification

- [x] 4.1 为 `.knowledge` 最小正式产物、`recovery-manifest`、SQLite mirror rebuild 和 restore 失败路径补 Rust 测试
- [x] 4.2 为“restore 不得隐式 regenerate”“恢复后 `status/query` 可消费但不等于完整 runtime ready”补集成测试或生命周期测试
- [x] 4.3 运行 `node scripts/run-test-projects.mjs storybook`，验证 `.knowledge` 落盘、cache rebuild 与恢复态 blocker/readiness 语义
- [x] 4.4 运行 restore-focused lifecycle（`node scripts/test-wiki-lifecycle.mjs --phase bootstrap --run-mode warm storybook`），确认 cold restore 后生命周期脚本与既有 lifecycle/index-readiness 基线不回退；`phase=full` 长尾留作后续性能/预算问题
- [x] 4.5 输出本轮 `storybook` 专项报告到 `.spec/changes/iteration-12-2-knowledge-runtime-artifact-minimum-set/reference-project-reports/*.md`，并把 `dagger` 作为观察项记录而不是主门禁

## 5. Commenting And Boundary Hygiene

- [x] 5.1 检查本 change 涉及代码与脚本的注释是否符合 `COMMENTING.md`
- [x] 5.2 核对实现、design、specs 与 tasks 的边界一致，确保没有把 knowledge-first update、query route 最终发布语义、declared knowledge lifecycle 或 `v0.2.0` release 验收混入本轮


