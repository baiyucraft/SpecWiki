## Why

`12.1 / 12.2 / 12.3` 已经把 provider-backed research、`.wiki/.knowledge/**` 正式产物和 knowledge-first update 主线收进来了，但仓库当前对外仍然停留在 `v0.1.0 index-only` 发布语义：`README`、`repo-wiki-workflow` 以及 `crates/wiki-runtime/src/workflows/release_scope.rs` 还在把“只有 facts/index 可查询”描述成正式成功路径。这样会让内部已经成立的 knowledge runtime 主线继续被旧 release 短路掩盖，`v0.2.0` 也就始终无法成为真实可承诺的发布面。

## What Changes

- 把公开 workflow 合同从 `v0.1.0 index-only` 升级到 `v0.2.0 knowledge runtime first-class release`：`init / update` 的正式成功语义不再是“只要 `.wiki/.cache/wiki-cache.db` 可用就算完成”，而是必须形成 `.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json + .wiki/.cache/**` 的最小正式知识运行时，或返回显式 blocker / 非完成诊断。
- 删除 `SPEC_WIKI_V0_1_INDEX_ONLY` 与 `index_only` 公开成功态在 runtime 主路径中的正式地位；facts/index 仍然可以作为 query substrate 与降级诊断存在，但不得再伪装成 `v0.2.0` 的 init/update 成功。
- 收紧 query route 的正式合同：外部仍保持 `term-only` 输入，但 runtime 内部必须正式遵循 `index -> knowledge -> page fallback`，并通过稳定的 `provenance_summary` route tags 显式区分 `index_hit / knowledge_hit / page_fallback`，禁止页面命中继续冒充 facts/index。
- 为 `v0.2.0` 定义统一 release gate：`storybook` 作为 primary gate，`chi + zustand` 作为固定 smoke gate，`dagger` 下沉为后续参考项，并把 `COMMENTING.md` 合规检查纳入本轮正式验收。完整 `19` 项目 `init` baseline guard 仅保留为后续可选参考，不再属于本轮强制收口。
- 同步收口对外文档、帮助文本和宿主消费预期，但这些文案改动只服务于已有 contract 的对齐，不额外扩张新的公开命令、payload 或 lifecycle surface。
- **BREAKING**：当前依赖“`init / update` 成功只表示 index-first runtime 可查询”的调用方和文档需要升级到 `v0.2.0` 语义；若 workflow 只停在 facts/index 或 `runtime_incomplete` 诊断态，系统不得再把它表述为正式成功初始化。

## Capabilities

### New Capabilities

- 无

### Modified Capabilities

- `repo-wiki-workflow`: 把公开 workflow/release 合同从 `v0.1.0 index-only` 升级为 `v0.2.0 knowledge runtime first-class release`，并收紧 `init / status / update / query` 的正式成功语义。
- `repo-wiki-runtime`: 删除 `index_only` 发布短路的正式地位，收紧 runtime readiness、`provenance_summary` route tags 与 `index -> knowledge -> page fallback` query route 的对外语义。
- `workflow-verification`: 把 `v0.2.0` 的 release gate、`storybook` primary gate、smoke 项目与注释合规检查提升为本轮正式验收要求，并把 `dagger` 与完整项目集 baseline guard 下沉为非阻塞参考项。

## Impact

- 重点影响 Rust runtime workflow 与公开状态面：`crates/wiki-runtime/src/workflows/{release_scope,init,update,status,query,rebuild}.rs`
- 重点影响 CLI/README/宿主可见合同：`README.md`、runtime command contract 测试、宿主 bootstrap/skill 文案
- 重点影响 release 验证与项目脚本：`scripts/test-wiki-lifecycle.mjs`、相关 `storybook` / smoke 报告，以及 `dagger` 与完整项目集 baseline guard 的范围说明
- 重点影响 UniSpec 契约：`.wiki/05-规格基线/capabilities/repo-wiki-workflow/spec.md`、`.wiki/05-规格基线/capabilities/repo-wiki-runtime/spec.md`、`.wiki/05-规格基线/capabilities/workflow-verification/spec.md`
