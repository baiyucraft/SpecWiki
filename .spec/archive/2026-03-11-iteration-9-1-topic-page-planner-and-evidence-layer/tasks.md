## 1. 专题发现与页面规划

- [x] 1.1 在 `crates/wiki-core/src/generation/planner.rs` 中引入稳定的专题候选与 `topic` 页面类型，并为专题页补齐稳定 `page_id / relative_path / parent_id / scope` 规则。
- [x] 1.2 扩展 `crates/wiki-core/src/repo/hierarchy.rs` 与相关聚合输入，输出根级高信号文件簇、模块能力簇和流程主题线索，但不得把这些专题线索直接写成 synthetic `ModuleNode`。
- [x] 1.3 在 planner 中实现专题页与模块页的去重/抑制规则，避免生成内容高度重叠的正式页面。

## 2. Evidence Layer 与图表达

- [x] 2.1 扩展 `crates/wiki-core/src/domain/context.rs` 与 `crates/wiki-core/src/generation/context.rs`，为 `PageContext` 增加结构化 evidence layer 与 diagram inputs，而不是继续只传 `facts / summary_inputs` 字符串。
- [x] 2.2 升级 `crates/wiki-core/src/generation/{sections,renderer}.rs`，为专题页、模块页和 workflow 页渲染稳定 evidence block，并保持 managed section 身份稳定。
- [x] 2.3 在渲染链中实现 facts-driven Mermaid：至少支持模块依赖图、父子结构图和流程图三类 deterministic-first 图输入，并在输入不足时回退到纯文本。

## 3. Runtime 与增量收口

- [x] 3.1 调整 `crates/wiki-core/src/workflows/{init,update,rebuild,page_render}.rs` 与相关状态装配逻辑，让专题页和 evidence block 进入现有 runtime/state/cache 主链，而不是新增 sidecar 层。
- [x] 3.2 调整页面 input hash、affected set 和增量 update 收口逻辑，确保专题页、evidence 输入和 diagram input 变化时只重建受影响页面及其父页。
- [x] 3.3 为 `wiki-core` 增加可配置 debug trace 模式，允许通过 steering 或启动参数开启，并确保 stdout 正式 JSON IPC 不被调试输出污染。

## 4. LLM 增强对齐

- [x] 4.1 扩展 `crates/wiki-core/src/llm/mod.rs` 中的页面增强输入，让 LLM 消费专题页摘要、evidence groups 和 deterministic diagram inputs，而不是只消费扁平 facts。
- [x] 4.2 收紧 LLM 图增强 contract：图结构必须来自 deterministic inputs，LLM 只能补充解释和轻量说明，不得凭空发明结构图。
- [x] 4.3 在 debug trace 模式下记录 provider 直连和 Agent bridge 两条路径的完整 LLM 请求/响应 JSON。

## 5. 验证与项目集

- [x] 5.1 为 Rust core 增加自动化测试，覆盖专题页发现、专题页 identity 稳定、evidence block 落页、facts-driven 图回退与增量 update 受影响集合。
- [x] 5.2 升级 `scripts/collect-reference-project-reports.mjs` 与相关分析输出，新增专题页覆盖率、evidence 落页和图表达覆盖统计。
- [x] 5.3 运行 `node scripts/run-test-projects.mjs` 对 `.wiki/06-设计文档/00-总体设计.md` 定义的 19 个测试项目全量执行 `init`，并更新 `test-project-analysis.md`。
- [x] 5.4 运行 `node scripts/test-wiki-lifecycle.mjs` 验证 `init -> status -> sync -> query -> update -> rebuild` 全链路在专题页与 evidence layer 引入后仍保持正确状态流转。
- [x] 5.5 运行 `node scripts/collect-reference-project-reports.mjs --jobs <N>` 对全部有 reference 的项目生成逐项目报告，并在当前 change 目录下输出新的差距分析。
- [x] 5.6 新增一个针对指定测试项目的 debug init 脚本，要求输出进度、完整 LLM JSON trace，并把日志目录固定落到当前 change 下。

## 6. 注释与收尾

- [x] 6.1 按 `.wiki/02-开发指南/00-代码注释规范.md` 对本轮新增或修改的 `crates/wiki-core/**`、`scripts/**` 注释做一次单独检查，补齐专题发现、evidence layer 和图输入相关的关键注释。
- [x] 6.2 将四个参考项目得出的“下一步优化建议”沉淀到当前 `9.1` change 目录，避免只存在对话上下文里。
