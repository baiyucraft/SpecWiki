## 1. Affected Knowledge Scope Contract

- [x] 1.1 在 `wiki-model / wiki-knowledge / wiki-runtime` 中定义 `AffectedKnowledgeScope`、escalation level / reason 与 projection target contract，明确 `AffectedSet` 只能由 scope 派生
- [x] 1.2 调整 `wiki-change-set-kernel`，让 `ChangeSet` 以 formal identity objects（`knowledge_units / knowledge_tree` 或等价 snapshot）为主输入，先计算 scope，再派生模块 / 页面 / section 影响集
- [x] 1.3 收口 scope planning 边界，确保 `planned_pages`、`page_digests` 与最终 Markdown 只作为 projection 层派生输入，不能反向主导 KnowledgeUnit 作用域

## 2. Scoped Knowledge Refresh

- [x] 2.1 调整 `wiki-knowledge` 增量主线，只对 `AffectedKnowledgeScope` 覆盖的 unit / domain 执行 research / summary / digest refresh，而不是默认跑整棵 knowledge pipeline
- [x] 2.2 实现 parent propagation 的 `child contract changed` 判定，确保 child source touched 但 contract 未变化时不默认重刷祖先链
- [x] 2.3 实现 `local_refresh / subtree_replan / repo_replan / rebuild_recommended` 的升级条件与 reason 产出，确保整树 knowledge replan 仍属于 `update` 主线

## 3. Runtime Update Commit

- [x] 3.1 重构 `update` workflow，使 research / compose / assemble 的执行范围由 `AffectedKnowledgeScope` 决定，并让 projection refresh 只覆盖派生出的 target pages
- [x] 3.2 调整 `.wiki/.knowledge/**`、`.wiki/pages/**`、`wiki.metadata.json` 与 `.wiki/.cache/**` 的提交流程，保证未受影响 records 保持稳定、移除单元/页面被显式回收、snapshot 锚点同步更新
- [x] 3.3 调整 `status / update` 的诊断摘要与推荐动作表达，使其能说明本次 scope、escalation 与是否需要更大范围 refresh 或 rebuild

## 4. Tests And Verification

- [x] 4.1 为 `AffectedKnowledgeScope`、parent propagation、projection target 派生、formal record 稳定性与移除回收补 Rust 单元/集成测试
- [x] 4.2 为“page diff 不能反推 knowledge scope”“child source touched 不默认重刷 parent”“repo_replan 仍属于 update”补 workflow / lifecycle 级验证
- [x] 4.3 运行 `storybook + dagger` 的 update-focused 专项验证，输出 affected scope、parent propagation、projection refresh 与 escalation reason 报告到 `.spec/changes/iteration-12-3-knowledge-first-update-mainline/reference-project-reports/*.md`
- [x] 4.4 以 `storybook-only` 的等价 lifecycle 路径补证 `baseline fresh -> no-op update -> mutation update -> source revert -> rebuild -> final fresh`，确认 knowledge-first update 主线没有把 `storybook` 的 `update / rebuild` 生命周期分支打回 `blocker` 或 `runtime_incomplete`；本任务不要求覆盖 `dagger`，也不声称覆盖 UniSpec 语义下的 `cold restore / warm restore preflight / cache rebuild`
- [x] 4.5 运行 `node scripts/run-test-projects.mjs` 的完整项目集 `init` 分析，输出 `test-project-analysis.md`，并明确该结果是 baseline guard、不是本轮 primary gate

## 5. Commenting And Boundary Hygiene

- [x] 5.1 检查本 change 涉及代码、脚本与测试的注释是否符合 `COMMENTING.md`
- [x] 5.2 复核 proposal、design、specs 与 tasks 的边界一致性，确保没有把 query release、declared knowledge lifecycle 或 13/14 阶段语义混入本轮
