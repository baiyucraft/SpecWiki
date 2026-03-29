## ADDED Requirements

### Requirement: `v0.1.0` 的 index-only release scope 必须显式收口 `init/update` 公开语义
当 `SPEC_WIKI_V0_1_INDEX_ONLY` 生效时，本 requirement MUST 作为 `v0.1.0` release scope 下对外 contract 的显式例外，覆盖完整 runtime 路径中“`update` 刷到 `fresh`”的公开承诺。该 release scope 下，`init` 和 `update` 的正式承诺 MUST 收敛为“完成 facts/index snapshot 并返回 `index_only` 终态”。`update` MAY 退化为重做当前 index-only runtime，但 MUST 显式暴露这一语义，而不是继续宣称完整 runtime refresh 已完成。

#### Scenario: index-only init 在 facts snapshot 后短路
- **WHEN** 用户执行开启 `SPEC_WIKI_V0_1_INDEX_ONLY` 的 `init`
- **THEN** workflow MUST 在 facts/index 提交后结束
- **THEN** workflow MUST 返回 `index_only` 终态
- **THEN** workflow MUST NOT 把自己声明为完整 page/runtime 已完成

#### Scenario: index-only update 明确暴露当前真实动作
- **WHEN** 用户执行开启 `SPEC_WIKI_V0_1_INDEX_ONLY` 的 `update`
- **THEN** workflow MAY 回退到一次 full index refresh
- **THEN** `update` 返回中的 `state` MUST 为 `index_only`
- **THEN** 面向宿主的动作说明和测试 contract MUST 明确它刷新的是当前 index-only runtime
