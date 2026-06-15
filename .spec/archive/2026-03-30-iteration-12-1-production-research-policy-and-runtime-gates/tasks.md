## 1. Provider Policy 收口

- [x] 1.1 调整 `crates/wiki-runtime/src/workflows/research_provider.rs`，把正式模式下的 provider 选择收紧为 provider-backed success 或显式 blocker failure
- [x] 1.2 将 `StructuralResearchProvider` 限制到测试、fixture 和显式开发模式，移除正式 `init / update / rebuild` 的默认 structural fallback 成功语义
- [x] 1.3 统一 `init / update / rebuild` 的 provider 选择与失败处理入口，确保三条 workflow 不再出现漂移

## 2. Runtime Gate 与状态投影

- [x] 2.1 调整 checkpoint / runtime gate 写入逻辑，覆盖 provider 缺失、provider 不可用、provider research 失败和 compose 失败
- [x] 2.2 收口 `crates/wiki-runtime/src/workflows/status.rs`、runtime summary 与 blocker hint，对外明确表达 `runtime_incomplete / blocker / needs_update`
- [x] 2.3 清理仍把 structural fallback 视为正式完成态的旧状态分支、旧推荐动作和旧成功摘要

## 3. Tests And Verification

- [x] 3.1 为 provider policy 与 runtime gate 新增 Rust 测试，覆盖正式模式失败、显式开发模式例外、checkpoint/gate 联动与 `status` 投影
- [x] 3.2 更新现有 LLM/runtime 测试，移除“正式模式下 bridge/provider 不可用仍可 structural success”的旧断言
- [x] 3.3 运行 `node scripts/run-test-projects.mjs storybook`，验证 `storybook` 的 production research policy、runtime readiness 与 blocker 诊断
- [x] 3.4 运行 `node scripts/test-wiki-lifecycle.mjs storybook`，确认生命周期脚本能消费新的状态与 blocker 语义
- [x] 3.5 输出本轮 `storybook` 专项结论到 `.spec/changes/iteration-12-1-production-research-policy-and-runtime-gates/reference-project-reports/*.md`，并在结论中注明 `dagger` 延后到后续 change

## 4. Commenting And Change Hygiene

- [x] 4.1 检查本 change 涉及代码的注释是否符合 `.wiki/02-开发指南/00-代码注释规范.md`
- [x] 4.2 核对实现、design、specs 与 tasks 的边界一致，确保没有把 `.wiki/.knowledge/**`、knowledge-first update 或 release 验收混入本轮
