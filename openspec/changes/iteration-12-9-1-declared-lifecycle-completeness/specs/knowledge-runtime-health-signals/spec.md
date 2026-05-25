## ADDED Requirements

### Requirement: declared 删除写回必须继续暴露下游 stale 与健康诊断
系统 MUST 将 declared 删除写回视为会影响下游 derived / projection 的正式 lifecycle 变化。合法删除后，系统 MUST 继续生成可供 `status`、`update` 或等价 workflow 消费的 stale scope 与 health diagnostics，而不是因为当前页 declared records 为空就停止传播。

#### Scenario: 删除 declared 后仍产生 stale / health scope
- **WHEN** 某页合法删除了此前存在的 declared records
- **THEN** sync 结果 MUST 继续暴露受影响 `stale_unit_ids`、`stale_projection_ids` 或等价稳定引用
- **THEN** runtime MUST 继续生成与 declared 删除相匹配的 health diagnostics

#### Scenario: 合法删除不得误报 illegal drift
- **WHEN** 某页只发生了合法 declared snapshot 删除，且页面其它部分保持受管合法
- **THEN** 系统 MUST NOT 将这次变化误报为 `illegal_drift`
- **THEN** 相关 health diagnostics MUST 表达“等待下游刷新”而不是“页面 truth 非法漂移”
