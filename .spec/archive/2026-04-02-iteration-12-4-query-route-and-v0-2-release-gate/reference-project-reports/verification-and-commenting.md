# 12.4 验证与注释检查

## 已完成验证

- `cargo test -p wiki-runtime`
  - 结果：通过
  - 说明：覆盖 runtime workflow、query route、transport contract、acceptance、hierarchy、repo、symbols 等测试。
- `cargo build -p wiki-runtime --bin wiki-runtime --target-dir target/e2e`
  - 结果：通过
  - 说明：为 CLI e2e 刷新 runtime 二进制。
- `pnpm vitest run scripts/tests/test-wiki-lifecycle-preflight.test.ts scripts/tests/e2e.test.ts`
  - 结果：通过
  - 说明：验证 warm restore preflight、CLI forwarding、`status/query` forwarding 与 `provenance_summary` route tags。
- `pnpm test`（`packages/spec-wiki`）
  - 结果：通过
  - 说明：覆盖 CLI、forwardCore、invokeCore、command assets 与宿主工具映射。
- `node scripts/test-wiki-lifecycle.mjs --no-build --phase steady --run-mode cold --timeout-minutes 45 chi zustand`
  - 结果：通过，`50/50` assertions 通过
  - 说明：`chi + zustand` smoke gate 已完成，覆盖 `init / status / sync / query / update(no-op)` 闭环。
- `node scripts/test-wiki-lifecycle.mjs --no-build --phase bootstrap --run-mode warm --timeout-minutes 120 storybook`
  - 结果：通过，`10/10` assertions 通过
  - 说明：证明 `storybook` 可从 partial runtime 续跑收敛到 `fresh`，并满足当前收口后的 primary gate runtime 门槛。
- `target/debug/wiki-runtime.exe --json` + `query(term=addon)`（`tmp/test/storybook`）
  - 结果：通过
  - 说明：`query_mode=index_first`、`provenance_summary=index_hit`，满足本轮 query route 可观测性要求。

## 非阻塞参考项

- `chi + zustand` smoke gate
  - 已通过
  - 当前结果与结论见 `smoke-gate-chi-zustand-2026-04-02.md`
- `dagger`
  - 已移出本轮强制验收范围
  - 当前只保留为后续参考项，不影响 `12.4` 收口
- `node scripts/run-test-projects.mjs` 全量 baseline guard
  - 已移出本轮强制验收范围
  - 当前只保留为后续可选参考项；详见 `baseline-guard-2026-04-02.md`

## 关键实现结论

- 已移除 `SPEC_WIKI_V0_1_INDEX_ONLY` / `index_only` 的公开成功短路。
- `status` 与 `query` 对 `runtime_incomplete` 统一改为 `needs_update / update` 语义，不再包装成 `ready / none`。
- `query` 结果内部已收口到 `index -> knowledge -> page fallback`。
- `provenance_summary` 已切到稳定 route tags：
  - `index_hit`
  - `knowledge_hit`
  - `page_fallback`
- `spec-wiki` CLI forwarding 修复了 `gate_summary.blockers` 缺省时把 `status` 误判为失败的问题。

## .wiki/02-开发指南/00-代码注释规范.md 检查

本轮新增或修改的关键文件已人工复核：

- `crates/wiki-runtime/src/workflows/release_scope.rs`
- `crates/wiki-runtime/src/workflows/init.rs`
- `crates/wiki-runtime/src/workflows/update.rs`
- `crates/wiki-runtime/src/workflows/query.rs`
- `crates/wiki-runtime/src/workflows/status.rs`
- `crates/wiki-runtime/src/domain/runtime_profile.rs`
- `packages/spec-wiki/src/runtime/parseResult.ts`
- `scripts/test-wiki-lifecycle.mjs`

检查结论：

- `crates/wiki-runtime/src/domain/runtime_profile.rs` 与 `crates/wiki-runtime/src/workflows/query.rs` 已补齐 `//!` 文件头注释。
- 模块级文件使用 `//!` 或 `/** ... */` 文件注释，满足仓库规范。
- 新增逻辑没有引入英文业务注释；注释语言满足中文规范。
- 注释聚焦在状态投影、query route 与 transport 协议边界，没有机械复述代码。
- 当前无需为简单字段补充额外注释；新增 helper 主要复用现有命名语义，可读性可接受。

## 边界复核

- 本轮没有新增公开 capability。
- 本轮没有把 `sync / rebuild` 扩成新的公开 surface。
- 本轮没有引入新的外部 query payload。
- 本轮没有把 `runtime_incomplete` 重新包装成发布成功态。
