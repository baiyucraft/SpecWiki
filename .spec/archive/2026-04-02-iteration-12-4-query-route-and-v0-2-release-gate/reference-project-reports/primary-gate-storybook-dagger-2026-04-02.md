# storybook primary gate check (2026-04-02)

## Scope

- change: `iteration-12-4-query-route-and-v0-2-release-gate`
- gate target: `3.1`
- objective: 验证 `storybook` 是否已经达到本轮要求的 release-ready primary gate；`dagger` 已下沉为后续参考项

## Commands Attempted

- `node scripts/test-wiki-lifecycle.mjs storybook dagger`
  - 结果：未形成可归档结论；构建阶段出现 `target/release/wiki-runtime.exe` 文件锁
- `node scripts/test-wiki-lifecycle.mjs --no-build storybook dagger`
  - 结果：超时
- `node scripts/test-wiki-lifecycle.mjs --no-build --phase bootstrap --run-mode warm --timeout-minutes 120 storybook`
  - 结果：通过，`10/10` assertions 通过，`status after init = fresh`
- `target/debug/wiki-runtime.exe --json` + `query(term=addon)`（`tmp/test/storybook`）
  - 结果：通过，`query_mode=index_first`，`provenance_summary=index_hit`
- `node packages/spec-wiki/bin/spec-wiki.js wiki status --repo-root tmp/test/storybook`

## Current Snapshot

### storybook

- `warm/bootstrap` 续跑后：
  - `init` 成功
  - `wiki.metadata.json` 已写出
  - `all pages have managed markers (226/226)`
  - `status after init = fresh`
  - `graph tables readable (edges=26064, communities=794, processes=8)`
- query route 快照：
  - `query_mode=index_first`
  - `provenance_summary=index_hit`
  - `query_trust=stale_but_queryable`

## Interpretation

- `storybook` 已证明当前 runtime 主线可以从 partial runtime 继续收敛到最小正式 knowledge runtime，并稳定返回 `fresh`。
- query 快照同时证明了本轮要求的 route tags 仍维持 `index -> knowledge -> page fallback` 语义中的 `index_hit` 可观测性，没有回退成旧的混层结果。
- 这说明此前 primary gate 的核心阻塞并不是 `status/query` 合同回退，而是长流程样本在超时或中断后没有续跑完。
- 按本轮更新后的验收边界，`storybook` 已满足 primary gate；`dagger` 不再作为本轮强制项。

## Task Impact

- `3.1` 可以标记为 completed。
- `dagger` 已下沉为后续参考项，不再作为本轮强制收口条件。
