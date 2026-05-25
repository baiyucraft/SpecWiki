## Why

`iteration-12-9-7-formalize-quality-gates-and-acceptance-contract` 已经为 scripts、primary gate 和 baseline guard 建立了统一 summary contract，但它主要解决的是“怎么统一收测试结果”，而不是“怎么从 knowledge system 角度判断这次 release 是否真的可放行”。当前仍缺少 knowledge-level metrics、program-level blocker 与 release evidence 规则，导致团队能看到 gate 结果，却还不能稳定回答：哪些 stale / blocked / conflict / missing provenance 已经严重到必须阻断 release。

因此这轮要继续 formalize 的不是脚本接口本身，而是建立 knowledge governance metrics 与 release gates，让 `quality gates` 之上再长出一层面向 knowledge system 的放行标准与证据闭环。

## What Changes

- 新增 `knowledge-governance-metrics-and-release-gates` capability，定义 knowledge-level metrics、program-level blocker 与 release evidence contract。
- 将 `stale rate`、`blocked rate`、`conflict rate`、`missing provenance rate`、`degraded answer rate` 等指标纳入正式 knowledge governance summary。
- 为 `storybook + dagger` primary sample 与 `19` 项目 baseline batch 定义 release blocker、waiver 与 evidence 归档规则。
- 让 gate closeout 能区分“脚本执行成功”与“knowledge system 实际可放行”，并输出面向 program 的 release judgment。
- **BREAKING**：若当前 closeout 只要脚本成功就可被视为通过，本轮会将 release 判断收紧为“脚本结果 + knowledge-level metrics + blocker 评估”三者共同成立。

## Capabilities

### New Capabilities
- `knowledge-governance-metrics-and-release-gates`: 定义 knowledge-level metrics、blocker、waiver 与 release evidence 的正式 contract。

### Modified Capabilities
- `workflow-verification`: 生命周期与批量测试需要额外产出 knowledge-level metrics 与 blocker judgment。
- `reference-fidelity-reporting`: 样本报告需要补充 knowledge governance summary，而不只对比页面结构与 gate 状态。
- `knowledge-runtime-health-signals`: health signals 需要支持聚合成稳定的 release-facing governance metrics。

## Impact

- `scripts/testing/**`、`scripts/run-test-projects.mjs`、`scripts/test-wiki-lifecycle.mjs`、`scripts/collect-reference-project-reports.mjs`
- `.docs/**` 与 `openspec/**/reference-project-reports/*.md` 的 release evidence 结构
- `crates/wiki-runtime/**` 对 health summary、metrics 聚合与 recommended action 的暴露面
- release closeout 对 primary sample、batch baseline 与 waiver 的判定方式
