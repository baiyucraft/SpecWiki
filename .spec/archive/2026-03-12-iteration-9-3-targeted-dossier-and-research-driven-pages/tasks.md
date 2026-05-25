## 1. Dossier 与出处数据结构

- [x] 1.1 在 `crates/wiki-core/src/generation/context.rs`、相关 domain/runtime 结构中引入 `TargetedSnippet`、section-scoped evidence provenance 和升级后的 dossier 字段
- [x] 1.2 用符号定义、call/process trace、entry point、child rollup 等真实代码线索替换当前“文件前 24 行”预览路径
- [x] 1.3 升级 evidence/source layer，持久化真实 `path/source_id/start_line/end_line/evidence_type/section_refs/note`，并为 coarse span 定义显式降级语义

## 2. Research Contract 与页面组装

- [x] 2.1 升级 `PageResearchResult` 为 `summary + section_plan + evidence_rollup + diagram_rollup + open_questions`，移除 9.2 的 summary-only 假设
- [x] 2.2 在 `crates/wiki-core/src/generation/sections.rs` 中实现 section-plan-driven 组装，让 renderer 消费 `section_key/title/summary/evidence_refs/diagram_refs/child_refs`
- [x] 2.3 保持 managed section、`section_id`、marker 和 deterministic fallback 稳定，确保 research 失败时仍能回退到合法 Markdown

## 3. Workflow、Session 与 LLM 输入

- [x] 3.1 在 `crates/wiki-core/src/workflows/page_render.rs` 与 `crates/wiki-core/src/llm/mod.rs` 中把 research 覆盖范围扩到 `overview/architecture/module/topic`
- [x] 3.2 把 `session_id/session_summary/recent_turns/tool_artifact_refs` 变成显式 session state，并在同页重复 research 时复用
- [x] 3.3 做实 research 工具与输入裁剪规则，优先让 `read_source_snippets`、symbol/process/page 工具返回可支撑 section-plan 的结构化材料

## 4. Planner 与 Runtime

- [x] 4.1 在 `crates/wiki-core/src/generation/planner.rs` 和相关 hierarchy 逻辑中按 repo archetype 扩展专题页族
- [x] 4.2 完善专题页去重、抑制和父子关系规则，避免 archetype 专题与模块页/兄弟专题页重复
- [x] 4.3 在 runtime/state/cache 主链内持久化 `section_plan`、精准 evidence provenance 和显式 session state，确保 `update/rebuild` 可增量复用

## 5. 自动化测试

- [x] 5.1 为 targeted snippets、line-span evidence、section plan、overview/architecture research 和 planner archetype 规则补齐 Rust 测试
- [x] 5.2 为 provider-first research session 补齐回归测试，覆盖结构化结果、显式 session state、budget 裁剪和 deterministic fallback
- [x] 5.3 检查并修正本轮新增/修改代码中的注释，使其符合 [COMMENTING.md](E:/project/!byAI/spec-wiki/COMMENTING.md)

## 6. 项目集与 Reference 验证

- [x] 6.1 运行 `node scripts/run-test-projects.mjs` 对 `DESIGN.md` 约定的 19 个测试项目执行 `init` 分析，并检查 `.wiki/` 产物
- [x] 6.2 运行 `node scripts/test-wiki-lifecycle.mjs` 覆盖 init → status → sync → update → rebuild 全链路
- [x] 6.3 更新 `test-project-analysis.md`，逐项目输出 section-plan 覆盖、overview/architecture research 命中、精准 evidence 引用密度和与 reference 的差异
- [x] 6.4 重新生成带 reference 项目的逐项目对比报告，输出 archetype 高频专题、citation 密度、图覆盖和缺失专题类别

## 7. 收尾

- [x] 7.1 运行 `.spec validate iteration-9-3-targeted-dossier-and-research-driven-pages`
- [x] 7.2 复核 proposal/design/tasks 与实现一致，补充必要的优化结论或后续建议
