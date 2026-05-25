## MODIFIED Requirements

### Requirement: parent contract 摘要必须进入可复用 runtime/cache 主链
系统 MUST 将 parent compose 需要的最小 contract 摘要写入现有 runtime/cache 主链，以支持 `update`、`rebuild`、reference 报告与 runtime 诊断复用。对 parent unit 而言，摘要 MUST 至少覆盖 `child_unit_ids`、`child_page_ids`、child digest 引用、citation/diagram 摘要引用、readiness 状态以及当前 parent `UnitResearch` 对应的最小身份线索。系统 MUST NOT 继续只为这类页面写入最小 `source_ids`。

#### Scenario: parent page context 能回溯 child contract 输入
- **WHEN** 系统为某个 parent unit 写入正式 runtime cache
- **THEN** 后续读取方 MUST 能从 cache/state 中回溯该页面对应的 child unit 集合与最小 compose contract 摘要
- **THEN** 报告、诊断和增量更新 MUST 不需要依赖最终 Markdown 反推这些输入

#### Scenario: 高层 parent unit cache 能回溯自身 research 身份
- **WHEN** 系统为 `Overview`、`Architecture`、`DomainIndex` 或 `config_surface` parent unit 写入 runtime/cache
- **THEN** cache/state MUST 能区分“该页面已有自己的 parent `UnitResearch`”与“仅有 system/domain seed”
- **THEN** 系统 MUST NOT 把高层父页继续记成只消费 seed 的页面
