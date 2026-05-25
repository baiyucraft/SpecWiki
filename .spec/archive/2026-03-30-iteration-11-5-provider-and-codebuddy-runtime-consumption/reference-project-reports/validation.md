# iteration-11-5 场景验证说明

## 本轮目标

本轮只验证 Provider / CodeBuddy 如何消费当前阶段 `wiki-runtime` 的 runtime profile：

- `status` preflight
- `query` trust/profile
- 长流程 `progress / result / error` 中的 `usage / runtime_summary / llm_execution_mode`

本轮明确不包含：

- knowledge 正式消费
- page 正式消费
- CodeBuddy TS 层新增 Wiki 业务状态机

这些能力仍留在后续迭代处理。

## 代码侧验证结论

### wiki-runtime

- `status` 现在稳定返回：
  - `facts_ready`
  - `query_readiness`
  - `recommended_action`
  - 可选 `runtime_summary`
  - 可选 `gate_summary`
  - 预判型 `llm_mode_hint`
- `query` 现在稳定返回：
  - `runtime_state`
  - `query_mode`
  - `query_trust`
  - `recommended_action`
  - 现有 provenance 与 `matches`
- `init / update / rebuild` 终态现在稳定返回：
  - 可选 `runtime_summary`
  - 真实 `llm_execution_mode`
- 长流程错误路径现在尽量附带：
  - `runtime_summary`
  - `blocker_hint`

### CodeBuddy

- `parseResult.ts` 已能薄解析：
  - preflight profile
  - query trust/profile
  - `usage`
  - `runtime_summary`
  - 真实 `llm_execution_mode`
- `invokeCore.ts` 继续只做 IPC 和 `llm_request` 桥接，不在 TS 层推导 Wiki 业务状态。
- `wikiStatus / wikiQuery / wikiInit / wikiUpdate / wikiRebuild` 的返回类型已收口为 typed response，不再是无差别 `unknown`。

## 已执行测试

### Rust

执行命令：

```bash
cargo test -p wiki-runtime --tests --no-run
cargo test -p wiki-runtime --test runtime status_
cargo test -p wiki-runtime --test runtime query_
cargo test -p wiki-runtime --test runtime progress_
```

关注结论：

- `status` 覆盖了 `missing / fresh / needs_rebuild` 的 preflight 与摘要投影
- `query` 覆盖了 `page_fallback / mixed / stale_but_queryable`
- 长流程 NDJSON 覆盖了终态 `runtime_summary / llm_execution_mode`

### TypeScript

执行命令：

```bash
pnpm -C agents/codebuddy exec tsc --noEmit
pnpm -C agents/codebuddy test
```

关注结论：

- CodeBuddy 能解析 preflight 和 query trust/profile
- CodeBuddy 能消费 `usage`
- CodeBuddy 只从终态读取真实 `llm_execution_mode`
- provider direct 路径下不会因为没有 `llm_request` 而丢失执行路径信号

## 当前边界结论

- 现在可以让 Provide / CodeBuddy 薄消费当前阶段 `wiki-runtime`
- 但“消费 knowledge 和 page”仍未在本轮定版
- 后续若要继续推进，应在新迭代里单独定义：
  - knowledge 消费 contract
  - page 消费 contract
  - 是否需要新的宿主工具或更高层 route
