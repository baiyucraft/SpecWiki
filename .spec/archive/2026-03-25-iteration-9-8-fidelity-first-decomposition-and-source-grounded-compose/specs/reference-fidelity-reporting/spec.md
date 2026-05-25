## ADDED Requirements

### Requirement: gap ledger 必须把 fidelity 缺口映射回 planner / research / compose contract
系统 MUST 在 reference fidelity 报告中为每个 `missing page`、`collapsed page`、`skeleton shortfall` 和 `key source shortfall` 输出对应的 contract hypothesis，并把它映射到 `planner`、`research` 或 `compose` 中的明确断点。报告 MUST 不得再只给症状和分数，而要直接回答哪一层 contract 失真。

#### Scenario: missing page 与 collapsed page 回指 decomposition contract
- **WHEN** 报告脚本发现 reference 页面缺失或被 coarse reuse 吞并
- **THEN** gap ledger MUST 把该缺口映射到对应的 `typed surface bundle`、`leaf decomposition policy` 或 `collapse guard`
- **THEN** 报告 MUST 能指出这更接近 planner 断点，而不是把它笼统归为页面质量差

#### Scenario: skeleton 与 key source shortfall 回指 research / compose contract
- **WHEN** 报告脚本发现最终 Markdown 存在 `skeleton shortfall` 或 `key source shortfall`
- **THEN** gap ledger MUST 指出问题更偏向 `research` 的 `skeleton_profile / key_source_clusters` 缺失，还是 `compose` 的 `section_grounding_refs / child digest contract` 未落页
- **THEN** 报告 MUST 保留对应 contract hypothesis，便于后续专项验收直接回看主链断点
