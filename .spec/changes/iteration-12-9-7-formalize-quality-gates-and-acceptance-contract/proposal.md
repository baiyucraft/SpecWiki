## Why

当前 knowledge system 的前置 formal contracts 正在逐步补齐，但“什么叫通过、什么会阻断收口、哪些样本和脚本属于正式门禁”仍然没有被单独定义。若继续把验收规则散落在脚本、口头约定和历史 change 里，后续很容易出现 schema 已变但 gate 仍按旧口径通过，或者把性能、平台化治理、全量达标承诺一股脑塞进 hardening。

因此这轮需要单独收敛 `quality gates + acceptance contract`：先把正式质量闸门、样本门禁、批量回归口径和 capability-to-test matrix 固定下来，让后续 capability 的“实现完成”与“验收通过”拥有同一套正式定义。

## What Changes

- 新增 `knowledge-quality-gates` capability，定义最小正式质量闸门，至少覆盖 `artifact validity`、`restore validity`、`query route contract` 与 `status/recommended_action stability`。
- 修改 `workflow-verification`，把 `storybook + dagger` 作为当前 primary gate 样本，把批量项目集 `init` 与 lifecycle 脚本收进正式 acceptance harness，而不是继续依赖临时测试习惯。
- 修改 `reference-fidelity-reporting`，明确 reference 报告在本轮是验收输入之一，但只服务于正式 gate，不等同于泛化“页面质量越高越好”的无限 hardening。
- 固定 capability-to-test matrix，要求后续 capability 至少映射到 model/artifact/workflow/sample 四类测试面。
- 固定本轮非目标：不做性能优化专项、不做 provider 稳定性攻坚、不承诺 19 项目全量达标、不做 dashboard 或 infra/platform 化测试治理系统。

## Capabilities

### New Capabilities
- `knowledge-quality-gates`: 定义 knowledge system 的最小正式质量闸门、阻断条件与 capability-to-test matrix。

### Modified Capabilities
- `workflow-verification`: 收紧当前脚本级验证合同，明确 primary gate 样本、批量回归入口与阻断语义。
- `reference-fidelity-reporting`: 收紧 reference fidelity 报告在正式验收中的角色与输出边界。

## Impact

- 主要影响 `scripts/run-test-projects.mjs`、`scripts/test-wiki-lifecycle.mjs`、`scripts/collect-reference-project-reports.mjs` 等验收入口与其后续输出口径。
- 影响 `wiki-runtime` 后续 change 的验收方式，但本轮不直接修改 capability 自身的 schema 或业务实现。
- 为后续 `answer assembly`、`query route`、`governance` 等 change 提供统一 gate 与收口标准。
