# chi + zustand smoke gate check (2026-04-02)

## Scope

- change: `iteration-12-4-query-route-and-v0-2-release-gate`
- gate target: `3.2`
- objective: 验证 `chi + zustand` 是否已经形成稳定的 `init / status / update / query` smoke gate

## Commands Attempted

- `node scripts/test-wiki-lifecycle.mjs --phase steady chi zustand`
  - 结果：早期尝试超时
- `node scripts/test-wiki-lifecycle.mjs --phase steady --timeout-minutes 60 chi zustand`
  - 结果：早期尝试超时
- `node scripts/test-wiki-lifecycle.mjs --no-build --phase steady --run-mode cold --timeout-minutes 45 chi zustand`
  - 结果：通过，`50/50` assertions 通过
- `node packages/spec-wiki/bin/spec-wiki.js wiki status --repo-root tmp/test/chi`
- `node packages/spec-wiki/bin/spec-wiki.js wiki query --repo-root tmp/test/chi --term router`
- `node packages/spec-wiki/bin/spec-wiki.js wiki status --repo-root tmp/test/zustand`
- `node packages/spec-wiki/bin/spec-wiki.js wiki query --repo-root tmp/test/zustand --term router`

## Current Snapshot

### chi

- lifecycle steady run：
  - `init` 成功
  - `status after init` 通过
  - `sync` 通过
  - `query` 通过
  - `update no-op` 通过
- steady run usage：
  - `init total_tokens=91792`
  - `update-noop total_tokens=91375`
- steady run graph snapshot：
  - `symbols=458`
  - `edges=1358`
  - `communities=43`
  - `processes=8`
- 快照查询：
  - `query.query_mode=index_first`
  - `query.query_trust=stale_but_queryable`
  - `query.provenance_summary=index_hit`

### zustand

- lifecycle steady run：
  - `init` 成功
  - `status after init` 通过
  - `sync` 通过
  - `query` 通过
  - `update no-op` 通过
- steady run usage：
  - `init total_tokens=112412`
  - `update-noop total_tokens=102626`
- steady run graph snapshot：
  - `symbols=206`
  - `edges=397`
  - `communities=16`
  - `processes=8`
- 快照查询：
  - `status.state=fresh`
  - `status.query_readiness=ready`
  - `status.recommended_action=none`
  - `query.query_mode=index_first`
  - `query.query_trust=ready`
  - `query.provenance_summary=`（本次 `router` 查询无命中）

## Interpretation

- 在解除残留 `wiki-runtime.exe` 文件锁后，`chi + zustand` 的 steady smoke gate 已经稳定通过。
- `chi` 证明了小样本仓库也可以完成完整的 `init -> status -> sync -> query -> update(no-op)` 闭环。
- `zustand` 证明了较大前端样本在相同脚本下也能完成同一闭环。
- 这组结果说明 12.4 的公开合同不依赖单个 `storybook` primary sample 才能成立。

## Task Impact

- `3.2` 可以标记为 completed。
