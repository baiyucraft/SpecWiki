## Why

迭代 9.1 已经让专题页、evidence layer 和 facts-driven 图进入正式主链，但对四个参考项目的真实代码对照仍然说明三个根因没有解决：

- 当前 `wiki-core` 在页面生成阶段主要把 `facts / summary_inputs / evidence_groups / diagram_inputs` 这类二手聚合材料送给 LLM，缺少稳定的研究中间层；
- 父页仍然偏向重复消费底层 facts，而不是消费已经沉淀出来的 child rollup；
- LLM 路径的上下文预算、并行度、缓存模式和 token 成本在正常模式下仍然不够可控、不够可观测。

继续主要调 prompt，收益已经明显下降。下一步需要把 `TopicDossier / ModuleDossier`、有边界的 research session、以及 LLM budget/usage 控制一起补齐。

## What Changes

- 新增 `page-research-dossier`：在 `RepoContext / ModuleContext` 与 `PlannedPage / RenderedPage` 之间引入稳定 dossier 层，沉淀 `key_sources`、`key_symbols`、`source_snippets`、`cross_module_edges`、`process_candidates`、`evidence_rollup`、`child_page_rollup` 和 `diagram_rollup`。
- 新增 `agent-session-bridge`：先把当前单轮 LLM 增强升级为有边界的 research session contract，但 9.2 只要求 core/provider 直连路径先跑通、先验证；CodeBuddy Agent bridge 的实现与测试后置到后续迭代。
- 新增 `llm-budget-observability`：支持按阶段配置上下文上限、并行度、cache mode 和显式 cold-start，并在非 debug 模式下实时输出 token usage 汇总。
- 调整 workflow：`init / update / rebuild` 先生成 dossier，再按需要进入 bounded research session，并让父页显式消费 child rollup。
- 调整验证与脚本：测试和项目报告必须区分 cold / warm run，覆盖 dossier/rollup、agent session、phase budget、实时 token 统计，以及同类型 uncertainty gate 批量化。

## Capabilities

### New Capabilities

- `page-research-dossier`
- `agent-session-bridge`
- `llm-budget-observability`

### Modified Capabilities

- `repo-wiki-workflow`
- `repo-wiki-runtime`
- `wiki-llm-enhancement`
- `workflow-progress-streaming`
- `wiki-steering-config`
- `workflow-verification`

## Impact

- 受影响代码集中在 `crates/wiki-core/src/generation/{context,planner,sections}.rs`、`crates/wiki-core/src/llm/mod.rs`、`crates/wiki-core/src/transport/json_rpc.rs`、`crates/wiki-core/src/workflows/{init,update,rebuild,page_render}.rs` 和 `crates/wiki-core/src/storage/**`。
- `wiki.dev.yaml` / steering schema 会新增 phase budget、parallelism、cache mode、session 上下文等配置项。
- progress/event stream 会新增 research session 和实时 usage 相关事件或字段，但 9.2 只要求 provider 直连路径先具备这些可观测性；最终仍只收口为一个 `result` 或 `error`。
- 9.2 的实现边界只覆盖 `module` / `topic` 页的研究会话，不把 `overview / architecture / init` 整条链直接改成自由对话 agent。
