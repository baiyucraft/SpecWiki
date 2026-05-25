## Why

`12.1` 已经把正式 workflow 收口到 provider-backed research，并把 `runtime_incomplete / blocker / needs_update` 变成正式状态语义；但当前 knowledge runtime 仍然主要停留在 SQLite 和本地 `.cache` 中，`.wiki/.knowledge/**` 还没有形成可上库、可恢复、可审计的最小正式产物集。这会直接卡住 `v0.2.0` 的 C 段目标，也使得“B 用户拉代码后基于正式产物恢复本地 runtime”这一核心场景仍然不成立。

## What Changes

- 为 `v0.2.0` 的 knowledge runtime 定义最小 Git-tracked 正式产物集，并将其落盘到 `.wiki/.knowledge/**`
- 新增 `.wiki/.knowledge/**` 的正式对象 contract，最小覆盖 `knowledge_domains`、`knowledge_units`、`knowledge_tree`、parent / unit research 摘要、`page_digests` 与 runtime gates / readiness 摘要
- 收口 truth boundary：`.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json` 是正式可共享产物，SQLite 与 `.wiki/.cache/**` 退回 cache / working state
- 定义基于正式产物恢复本地 `.wiki/.cache/**` 的最小闭环，使 `status / query` 可以在 cold restore 后直接工作，而不是要求每次都重新全量 `init`
- 为 `storybook` 优先补专项验证，确认 `.knowledge` 落盘、cache rebuild、恢复后 `status / query` 与 blocker/readiness 语义成立；`dagger` 仅保留观察项
- 本轮明确不做 knowledge-first update、不收 query route 最终发布语义、不进入 declared knowledge 生命周期，也不做 `v0.2.0` 最终 release 验收

## Capabilities

### New Capabilities
- `knowledge-runtime-artifacts`: 定义 `.wiki/.knowledge/**` 的最小正式对象集、文件职责、恢复 contract 与 cache rebuild 边界

### Modified Capabilities
- `repo-wiki-runtime`: 调整 runtime 对 `.wiki/.knowledge/** + pages + metadata + cache` 分层职责、恢复入口与正式 truth boundary 的要求
- `research-driven-page-composition`: 要求 research / compose 的正式结果沉淀为可落盘、可恢复的 knowledge 摘要，而不再只停留在运行期缓存
- `sqlite-cache-storage`: 明确 SQLite 中 knowledge 相关表退回本地 working cache / rebuild target，而不是唯一正式 knowledge truth
- `workflow-verification`: 增加 `.knowledge` 落盘、cold restore / cache rebuild，以及恢复后 `status / query` 可用性的专项验证要求

## Impact

- 重点影响 Rust runtime 与存储层：`crates/wiki-runtime/src/storage/**`、`crates/wiki-runtime/src/workflows/{init,status,query,rebuild}.rs`、`crates/wiki-runtime/src/domain/**`
- 重点影响 knowledge 输出与摘要契约：`crates/wiki-knowledge/src/{planning,research,compose}.rs` 与相关 store trait
- 重点影响 `.wiki/` 正式目录职责、协作恢复路径和验证脚本：`scripts/run-test-projects.mjs`、`scripts/test-wiki-lifecycle.mjs`
- 重点影响 UniSpec 契约：新增 `knowledge-runtime-artifacts`，并修改 `repo-wiki-runtime`、`research-driven-page-composition`、`workflow-verification`
