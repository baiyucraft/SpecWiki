## Why

`iteration-12-9-6-formalize-answer-assembly-contract` 已经收稳 `AnswerEnvelope`、supporting refs 与 degraded answer policy，但这仍主要是 formal contract 层面的收口，不等价于 answer / compose runtime 已在真实仓库、真实 restore、真实 provider-backed compose 下稳定可靠。只要 supporting refs、trust/provenance 和 compose substrate 的一致性还会在长流程里漂移，当前 answer surface 就仍然是“可用但未硬化”的状态。

因此这轮不再重做 answer contract formalization，而是把重点放在 runtime / pipeline hardening：让 answer assembly 真正建立在稳定的 declared / derived / projection substrate 之上，并在 `storybook / dagger` 等样本上验证 compose 与 answer 在长流程中的一致性。

## What Changes

- 新增 `answer-compose-runtime-hardening` capability，定义 answer / compose runtime 的稳定性、supporting refs 完整性与 restore 后 substrate 一致性要求。
- 收紧 answer assembly 对 declared / derived / projection / readiness / health 的消费边界，防止 page fallback 或临时 cache 再次成为主 answer substrate。
- 为 supporting refs、trust、provenance 和 degraded reason 建立更严格的一致性 contract，覆盖 cold restore、rebuild、provider-backed compose 与长流程 query。
- 将 `storybook + dagger` 的 provider-backed compose 稳定性验证纳入这轮 primary evidence，而不是只验证短路径 query route。
- **BREAKING**：若当前 host 或 runtime 默认接受缺 provenance、缺 supporting refs 或 substrate 漂移的 answer，本轮会将其收紧为显式 degraded / refuse，而不是继续静默返回“看起来可用”的答案。

## Capabilities

### New Capabilities
- `answer-compose-runtime-hardening`: 定义 answer / compose runtime 在 restore、provider-backed compose 与长流程 query 下的稳定性与一致性要求。

### Modified Capabilities
- `research-driven-page-composition`: compose 输出需要稳定暴露可供 answer 复用的 provenance、citation、projection substrate。
- `repo-wiki-workflow`: `query / rebuild / restore` 需要在 answer substrate 漂移时给出一致的 degraded / refuse 语义。
- `workflow-verification`: 生命周期验证需要加入 answer / compose runtime 的稳定性与证据要求。

## Impact

- `crates/wiki-runtime/**` 的 query、page_render、restore、research_provider 与 answer transport
- `crates/wiki-knowledge/**` 的 compose / research substrate 暴露面
- `scripts/test-wiki-lifecycle.mjs`、`scripts/collect-reference-project-reports.mjs` 与样本报告
- host 对 answer envelope、supporting refs、trust / provenance 的消费预期
