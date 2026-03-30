## Why

当前 `wiki-knowledge` 已经具备 `KnowledgeDomain / KnowledgeUnit / KnowledgeTree -> Research -> Compose` 的主链骨架，但高层父页仍然没有真正进入这条主线。`Overview / Architecture / DomainIndex` 以及 `config-surface parent` 仍可能跳过自己的 `UnitResearch`，直接依赖 `SystemResearch / DomainResearch` seed 或固定骨架成页，这会持续放大 many-to-one reuse，也让父页无法围绕 child rollup 稳定组织正文。

现在需要先把高层 parent unit 扶正为一等研究对象。只有先解决 parent contract，后续的 provider policy、正式知识产物落盘和 knowledge-first update 才有稳定输入边界。

## What Changes

- 收口高层 parent unit 的研究合同，要求 `Overview / Architecture / DomainIndex / config-surface parent` 都拥有自己的 `UnitResearch`，不再跳过 unit-scoped research。
- 调整 parent compose 输入合同，要求父页按层级消费 `child digest / citation digest / diagram digest / key sources / readiness`，而不是越层直接抓 leaf 或只拼 child summary。
- 收紧 `SystemResearch / DomainResearch` 的职责，使其降为 seed / overlay，而不是 parent page 的直接替代品。
- 补齐 runtime 对 parent contract 的最小诊断能力，确保高层父页的 child-backed compose 输入与 readiness 可被 cache / state / verification 回溯。
- 更新专项验收口径，本轮先要求 `storybook` 能直接证明高层 parent contract 成立，而不是只比较最终页面数量或表面匹配率。

## Capabilities

### New Capabilities

- 无

### Modified Capabilities

- `knowledge-unit-decomposition`: 高层 parent `KnowledgeUnit` 的稳定身份、child 边界和 `config-surface parent` 聚合规则需要收紧。
- `research-driven-page-composition`: 父页必须拥有自己的 unit-scoped research contract，并显式消费 child rollup，而不是直接依赖 system/domain seed 成页。
- `repo-wiki-runtime`: runtime 必须能持久化 parent contract 的最小 child-backed 摘要与 readiness 线索，便于恢复和诊断。
- `workflow-verification`: 自动化验证需要把高层 parent contract 是否成立纳入正式验收，本轮先以 `storybook` 为专项门禁。

## Impact

- 影响 `crates/wiki-knowledge/src/planning.rs`、`crates/wiki-knowledge/src/compose.rs`、`crates/wiki-knowledge/src/research.rs` 与 `crates/wiki-runtime/src/workflows/page_render.rs` 的高层父页路径。
- 影响 runtime 的 parent compose 输入缓存、gate/readiness 诊断以及对应测试。
- 影响 `storybook` 专项验收口径，但不要求在本 change 中同时完成 production no-fallback、`.wiki/.knowledge/**` 最小产物落盘或 `v0.2.0` 的完整 release 收口。
