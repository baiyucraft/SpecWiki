## Why

`v0.2.0` 已经把 `spec-wiki` 从 `facts-only` 过渡口径收敛到 `minimal formal knowledge runtime`，但当前 knowledge contract 仍明显偏薄：`declared knowledge` 还不是正式可操作对象，`KnowledgeUnit` 的最小 authoring contract 不够硬，`research / compose` 仍缺少最小正式边界，`sync` 与 `status` 也还无法稳定表达 knowledge 回写与健康状态。现在如果继续把系统表述成“knowledge runtime 已完成”，会掩盖正式 contract 仍不够可依赖、可审计、可测试的事实。

因此，下一轮不应扩张为“补完 knowledge system”，而应把 `v0.2.0` 已经存在但仍偏薄的 knowledge runtime contract 收敛成更稳定的正式边界，使 `declared / derived / projection / cache` 四层职责、流转和诊断都更清晰。

## What Changes

- 收敛 `declared knowledge` 的最小正式 authoring / lifecycle contract，明确哪些对象可以作为可审计长期知识进入正式层，哪些仍留在 working cache 或页面工作态。
- 明确 `KnowledgeUnit` 的最小 contract，补齐 identity、status、citation/source、versioning 与被引用/失效/刷新语义，避免继续把它停留在“有对象名词、无硬合同”的状态。
- 把 `research / compose` 收敛为最小正式 contract，只定义当前 runtime 必须稳定依赖的 research summary、compose 输入与 answer assembly / page projection 边界，不展开为 0.3.0 级重构。
- 收敛 `declared -> derived -> page projection -> cache` 的状态流转规则，明确 `update / sync / rebuild / status / query` 分别作用于哪一层、触发哪些失效与重算。
- 把 `sync` 从“页面文件同步动作”收敛为正式 knowledge writeback contract，明确哪些页面编辑可回写 declared、哪些只更新 metadata/runtime、哪些必须报 drift 或建议 rebuild。
- 引入最小 knowledge health signals，让 `status` 与相关诊断不只回答 runtime 是否存在，还能回答 knowledge 是否稳定、是否存在孤儿单元、缺失来源、派生失效或 projection 过期。
- **BREAKING**：若现有实现或文档仍把 `sync` 视为纯页面层动作、把 `declared/**` 视为纯占位、或把 `research / compose` 视为不需要正式合同的内部细节，本轮将收紧这些表述与对应 contract。

## Capabilities

### New Capabilities
- `declared-knowledge-lifecycle`: 定义 `declared knowledge` 的最小 authoring、writeback、状态与审计 contract。
- `knowledge-runtime-health-signals`: 定义 knowledge runtime 的最小健康信号、诊断对象与推荐动作边界。

### Modified Capabilities
- `knowledge-runtime-artifacts`: 扩展最小正式知识产物 contract，使 `KnowledgeUnit` 与 `declared` 相关正式对象不再停留在弱占位状态。
- `knowledge-first-update`: 把 update 的作用范围从“刷新 derived/projection”收紧为显式覆盖 `declared -> derived -> projection` 的失效与刷新关系。
- `repo-wiki-workflow`: 收紧 `status / sync / rebuild` 的知识语义，明确 workflow 对 knowledge runtime readiness、writeback 与 health diagnostics 的正式承诺。
- `research-driven-page-composition`: 把当前 runtime 必须依赖的最小 `research / compose / answer assembly` contract 写实，但不承诺完整的 knowledge pipeline 终态。

## Impact

- 影响 `wiki-knowledge` 的对象模型、artifact schema、research/compose 输入输出与 writeback 语义。
- 影响 `wiki-runtime` 的 `status / update / sync / rebuild / query` 生命周期与诊断边界。
- 影响 `.wiki/.knowledge/**`、`.wiki/pages/**`、`wiki.metadata.json` 与 `.wiki/.cache/**` 的四层职责说明与一致性要求。
- 影响当前 README / 设计文档 / UniSpec 对 `v0.2.0 knowledge runtime` 的表述精度，但不会把本轮扩张成完整 knowledge system 或 page-first 重构。
