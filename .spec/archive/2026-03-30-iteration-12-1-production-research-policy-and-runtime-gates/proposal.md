## Why

当前 3.0 主线已经完成四包拆分，并且刚刚收完高层 parent unit 的 research contract；但正式 runtime 仍停留在过渡态：`init / update / rebuild` 还能默认退回 `StructuralResearchProvider`，`status` 也还没有把 provider 缺失、runtime 未完成和 blocker 诊断收成正式语义。这会直接阻断 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble` 成为 `v0.2.0` 的真实发布主线，因此下一条 change 必须先把 Production Research Policy 收口。

## What Changes

- 收紧正式 workflow 的 provider policy：`init / update / rebuild` 默认必须走 provider-backed research，不再把 `StructuralResearchProvider` 当作正式成功路径。
- 将 `StructuralResearchProvider` 明确降级为测试、fixture 和显式开发模式专用实现；正式模式下无 provider、provider 不可用或 provider research 失败时，workflow 必须失败并保留 checkpoint / gate / blocker 诊断。
- 收口 runtime readiness 语义：`status`、runtime summary 与 gate/blocker 输出需要能区分 `runtime_incomplete`、`blocker`、`needs_update` 等正式 knowledge runtime 状态，而不是继续依赖 structural fallback 伪装成功。
- 补齐 `storybook` 的专项验收，重点验证 provider-backed research 执行路径、失败诊断和 runtime gate/readiness 是否成立，而不是只看最终页面数量或表面 query 可用；`dagger` 观察延后到后续 change。
- **BREAKING**：正式 runtime 不再接受“LLM 已启用但 provider 不可用时自动 structural fallback”的旧行为；这类情况将转为显式失败或显式开发模式。
- 本轮明确不做 `.wiki/.knowledge/**` 正式产物落盘、不做 knowledge-first update 改造、不做 query route / `v0.2.0` release 验收收口。

## Capabilities

### New Capabilities

- `production-research-policy`: 定义正式 workflow 的 provider-backed research 成功语义、fallback 禁令、开发模式例外与 runtime blocker 诊断边界。

### Modified Capabilities

- `research-driven-page-composition`: 正式 runtime 的 research/compose 链路改为 provider-backed 必选，移除默认 structural fallback 的正式成功语义。
- `wiki-llm-enhancement`: 统一 provider 选择、失败处理、checkpoint 与显式开发模式下的结构型 research 例外。
- `repo-wiki-runtime`: runtime 对外状态、gate/blocker 摘要和 `status` 诊断需要表达 `runtime_incomplete / blocker / needs_update`，而不是继续把 structural fallback 当成完成态。
- `workflow-verification`: 专项验证必须覆盖 provider-backed research 执行、失败诊断与 `storybook` 的 runtime readiness / blocker 报告。

## Impact

- 重点影响 Rust workflow 与状态面：`crates/wiki-runtime/src/workflows/research_provider.rs`、`crates/wiki-runtime/src/workflows/{init,update,rebuild,status}.rs`、`crates/wiki-runtime/src/domain/{checkpoint,runtime_profile}.rs`。
- 重点影响 research/compose 的正式契约和测试：`crates/wiki-knowledge/src/research.rs`、`crates/wiki-runtime/tests/{llm_runtime,runtime/**}`。
- 重点影响专项验证与报告脚本：`scripts/run-test-projects.mjs`、`scripts/test-wiki-lifecycle.mjs`、`changes/**/reference-project-reports/*.md`。
- 重点影响 UniSpec 契约：`.wiki/05-规格基线/capabilities/research-driven-page-composition/spec.md`、`.wiki/05-规格基线/capabilities/wiki-llm-enhancement/spec.md`、`.wiki/05-规格基线/capabilities/repo-wiki-runtime/spec.md`、`.wiki/05-规格基线/capabilities/workflow-verification/spec.md`，以及新增 `production-research-policy` capability。
