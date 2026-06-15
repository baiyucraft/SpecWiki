## Why

当前仓库已经具备 Repo Wiki 的 baseline 主链，但“迭代 1”仍停留在基础版完成状态：模块树还是偏一层结构，页面层级和模块树还没有完全收口，`query` 也仍保留较重的 Markdown 混合检索路径。若在这些缺口未收口前继续推进状态内核和增量运行时，后续迭代会建立在不稳定的结构事实上。

现在发起这一 change，是为了把 `.wiki/06-设计文档/00-总体设计.md` 中“Deterministic Structural Baseline”真正收尾到可交付状态：先把递归模块树、层级化页面规划、结构优先查询和 baseline 验收夹具做实，再进入后续运行时迭代。

## What Changes

- 收口递归 `ModuleTree`，让模块发现、父子层级、稳定 ID、`child_ids` 和根模块兜底策略形成完整 deterministic 规则。
- 补强会直接影响建树质量的结构化解析，包括 workspace、manifest、入口和依赖线索的归一化。
- 让 `Page Planner`、页面路径和 metadata 导出与递归模块树对齐，稳定表达页面层级、模块层级和页面来源。
- 将 `query` 收口为结构优先的 deterministic 检索，Markdown 文本匹配只作为回退路径。
- 建立面向中型仓库的 baseline 验收夹具，覆盖 `init -> metadata/cache -> query` 的整体输出质量。

## Capabilities

### New Capabilities
- 无

### Modified Capabilities
- `repo-hierarchy-model`: 收紧模块树、跨模块关系和页面规划的要求，使其必须支持递归层级与稳定排序。
- `repo-wiki-runtime`: 扩展 metadata 和页面映射要求，使其必须稳定表达递归模块页、页面父子关系和页面 provenance。
- `repo-wiki-workflow`: 收紧 `init` 与 `query` 的 baseline 行为，使其必须围绕递归模块树生成页面并执行结构优先检索。
- `workflow-verification`: 扩展验证要求，使其必须覆盖面向中型仓库的 baseline 验收夹具，而不是只验证最小可运行链路。

## Impact

- 受影响代码集中在 [crates/wiki-core](E:/project/!byAI/spec-wiki/crates/wiki-core) 的 `repo/`、`generation/`、`storage/`、`workflows/` 和测试目录。
- `.wiki/*.md` 与 `wiki.metadata.json` 的结构信息会继续扩展，但仍保持 deterministic facts 优先，不引入 LLM 依赖。
- [agents/codebuddy](E:/project/!byAI/spec-wiki/agents/codebuddy) 不引入新的业务规则，只会受益于更稳定的页面与查询输出。
- 参考实现将继续以 [tmp/codewiki-upstream/codewiki](E:/project/!byAI/spec-wiki/tmp/codewiki-upstream/codewiki)、[tmp/deepwiki-rs-upstream](E:/project/!byAI/spec-wiki/tmp/deepwiki-rs-upstream) 和 [tmp/reference-zh](E:/project/!byAI/spec-wiki/tmp/reference-zh) 为主，但边界仍以 [.wiki/06-设计文档/00-总体设计.md](E:/project/!byAI/spec-wiki/.wiki/06-设计文档/00-总体设计.md) 与 [.wiki/06-设计文档/00-总体设计.md](E:/project/!byAI/spec-wiki/.wiki/06-设计文档/00-总体设计.md) 为准。
