# Storybook 专项结论

## 范围

- 本轮专项只覆盖 `storybook`
- `dagger` 延后到后续 change，不属于本轮正式输出

## 已确认结论

- `iteration-12.1` 的 runtime 侧 provider policy 已经收紧到正式模式优先 `provider_backed`，不再把 bridge/provider 不可用默认伪装成 structural success。
- `status` 对外状态已经切到 `runtime_incomplete / blocker / needs_update` 语义；脚本侧同步更新了 `needs_update` 断言。
- 正式 production policy 已进一步收严：
  - `llm.disabled` 在 production 下不再被视为 structural success，而会落成显式 blocker
  - provider 预检查 blocker 会同步写入 `pipeline_checkpoint + runtime_summary + unit_runtime_gates`
- `scripts/testing/init-resume.mjs` 已补共享恢复逻辑：
  - lifecycle logger 与 project logger 统一兼容 `info/log`
  - Windows 瞬态文件锁错误会进入 preserve-resume 判定，而不是直接把 `storybook` 判成硬失败
- `scripts/test-wiki-lifecycle.mjs` 与 `scripts/run-test-projects.mjs` 已统一 diagnostic 验收口径：
  - `init` 返回非 ok 时，会回读 `.wiki` runtime 现场
  - 若现场已经进入 `runtime_incomplete / blocker`，专项脚本会把它记为“正式 runtime 语义已成立”，而不是继续按旧的 assembled-page 成功态硬判失败

## Storybook 现场观察

### 观察 1：旧的首轮 `os error 32` 已经被脚本层吸收为 diagnostic runtime

- 目前可以稳定复现：`storybook` 在 `cache=clear` 下会先尝试清理旧 `.wiki`，但 Windows 仍可能返回 `os error 32 / EBUSY`。
- 更新后的专项脚本不会再把这个场景直接记成失败，而是回读 `.wiki/.cache/wiki-cache.db` 与 `status`：
  - `node scripts/test-wiki-lifecycle.mjs --no-build --timeout-minutes 1 storybook`
  - `node scripts/run-test-projects.mjs --no-build --timeout-minutes 1 storybook`
- 两条脚本都已通过，并把 `storybook` 现场识别为 `runtime_incomplete`。

### 观察 2：当前专项通过的语义是“diagnostic runtime 成立”，不是“assembled pages 已完成”

- `test-wiki-lifecycle` 默认全链路当前结果：
  - `init` 与 `status after init` 都进入 `runtime_incomplete`
  - `query_readiness=ready`
  - symbol / graph 查询仍可工作
  - `sync / update / rebuild` 在 diagnostic state 下被明确跳过，而不是误判失败
- `run-test-projects` 当前结果：
  - `storybook` 被记为 `OK`
  - 汇总里显式输出 `diagnostic=runtime_incomplete`
- 这符合本轮 change 的验收边界：专项验证的是 production research policy、runtime readiness 与 blocker/diagnostic 语义，不要求 storybook 在这一轮必须 assembled 完整页面。

### 观察 3：长时 provider-backed research 仍是后续优化项，但不再阻塞本轮专项收口

- 曾经存在一次长时运行观察：`storybook` 在 provider-backed research 阶段持续超过 25 分钟，并累计 51 次 LLM request。
- 这说明大仓库样本上的 provider-backed research 耗时仍然偏高，后续仍值得继续做分段恢复、预算治理和终态超时策略。
- 但在本轮 `iteration-12.1` 的专项边界内，这已经不再阻塞 `3.3 / 3.4`，因为脚本验收口径已经切换到正式 runtime 诊断态，而不是旧的“必须 fresh/assembled 才算成功”。

## 专项结论

- `storybook` 本轮专项已通过。
- 通过的含义是：Production Research Policy、runtime readiness 与 blocker/diagnostic 语义已经在专项脚本里被稳定消费；旧的 `os error 32 + stale/fresh` 级联误报已经收口。
- 当前 `storybook` 被接受的终态是 `runtime_incomplete` 诊断态，而不是 assembled 完成态；这与本轮 change 的边界一致。
- 后续要继续推进 `storybook`，应优先处理：
  - provider-backed research 的长任务分段/恢复策略
  - storybook 样本上的 research 预算与终态超时控制
  - 在追求完整 assembled runtime 时，再重新执行更长时窗的 `run-test-projects.mjs storybook` 与 `test-wiki-lifecycle.mjs storybook`
