# 9.2 注释检查记录

检查日期：2026-03-11

本轮按 [COMMENTING.md](/E:/project/!byAI/spec-wiki/COMMENTING.md) 重新核对了 dossier、session、budget、usage 相关新增代码的注释边界，重点确认两件事：

- 注释是否解释“为什么有这层对象/流程”，而不是逐字段复述代码。
- 新增 helper 是否只在复杂控制流、预算裁剪、session/tool 协议处保留说明，避免到处堆解释性噪音。

本轮已重点检查：

- [llm/mod.rs](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/llm/mod.rs)
- [context.rs](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/domain/context.rs)
- [generation/context.rs](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/context.rs)
- [page_render.rs](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/page_render.rs)
- [steering.rs](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/domain/steering.rs)
- [progress.rs](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/progress.rs)
- [helpers.mjs](/E:/project/!byAI/spec-wiki/scripts/testing/helpers.mjs)
- [run-test-projects.mjs](/E:/project/!byAI/spec-wiki/scripts/run-test-projects.mjs)
- [collect-reference-project-reports.mjs](/E:/project/!byAI/spec-wiki/scripts/collect-reference-project-reports.mjs)

结论：

- `dossier / PageResearchResult / session state` 的结构注释已经够用，能说明它们是 deterministic render 前的中间研究对象。
- `provider tool-calling / emulated tools / response_format retry` 附近的注释仍保持在协议边界和回退原因，没有退化成逐行注解。
- `budget trim / usage reporter / cache mode` 新增逻辑的注释基本聚焦在“为什么要这样裁剪/保留/上报”，符合当前规范。
- 脚本层新增的 progress/usage 辅助函数注释已补到函数级，没有在简单字符串拼接处重复写低价值说明。

后续仍可继续收紧的点：

- 若 9.2 后续继续扩 `research tool`，工具结果 DTO 最好补一层更明确的字段级语义注释。
- 如果 lifecycle 脚本也升级到 provider/session 指标，相关 helper 需要同步做一次同口径复查。
