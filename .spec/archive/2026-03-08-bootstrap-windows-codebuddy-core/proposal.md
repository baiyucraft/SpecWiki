## Why

当前仓库已经有 `wiki-core` 与 `agents/codebuddy` 的最小代码骨架，但 `init/status/update/query/sync/rebuild` 的行为、`.wiki/` 运行产物、`wiki.metadata.json` 字段约束以及 Windows 下的 CodeBuddy Agent 接入边界还没有被明确约束。继续推进后续迭代前，需要先把“Windows + CodeBuddy 跑通”的第一阶段闭环定成可验证的规范，避免 core、runtime 和 Agent 各自演化。

## What Changes

- 明确 Windows 平台下 Repo Wiki Runtime 的目录结构、提交约束与 `wiki.metadata.json` 的最小字段语义。
- 明确 `wiki-core` 在第一阶段的六个工作流：`init`、`status`、`update`、`query`、`sync`、`rebuild` 的输入输出和最小行为。
- 明确 CodeBuddy Agent 在当前阶段的接入方式：仅支持 Windows，Agent 通过本地 binary 调用 core，不在 Agent 层承载 Wiki 业务逻辑。
- 约束第一阶段采用 deterministic baseline：先稳定扫描、页面生成、metadata 与状态流转，不要求引入 LLM、图生成或跨平台能力。

## Capabilities

### New Capabilities
- `repo-wiki-runtime`: 定义 `.wiki/`、`wiki.metadata.json` 与 `.wiki/.cache/` 的第一阶段运行时结构和约束。
- `repo-wiki-workflow`: 定义 `wiki-core` 在第一阶段的 `init/status/update/query/sync/rebuild` 行为与状态语义。
- `codebuddy-agent-integration`: 定义 Windows 下 CodeBuddy Agent 对 `wiki-core` 的调用边界、binary 解析与结果传递方式。

### Modified Capabilities

- None.

## Impact

- 影响 Rust core：`crates/wiki-core/src/workflows/*`、`crates/wiki-core/src/domain/*`、`crates/wiki-core/src/storage/*`、`crates/wiki-core/src/repo/*`
- 影响 CodeBuddy Agent：`agents/codebuddy/src/*`
- 影响运行时产物：目标仓库 `.wiki/`、`wiki.metadata.json`、`.wiki/.cache/`
- 为后续迭代 2 到迭代 5 提供稳定的 runtime、workflow 与 Agent 边界
