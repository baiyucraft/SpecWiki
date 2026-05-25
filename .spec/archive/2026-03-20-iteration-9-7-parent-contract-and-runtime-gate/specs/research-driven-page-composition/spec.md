## ADDED Requirements

### Requirement: 高层父页必须具备 unit-scoped parent research contract
系统 MUST 让 `Overview`、`Architecture`、`DomainIndex` 与 `config_surface` parent unit 在进入正式 compose 前拥有自己的 unit-scoped parent research contract。该 contract MUST 至少表达稳定的 `section_plan`、evidence/diagram 引用和 child-backed research 输入。系统 MUST NOT 再允许这些高层父页仅依赖 `SystemResearch`、`DomainResearch` 或固定 section 模板直接成页。

#### Scenario: overview 不再只靠 system research 直接成页
- **WHEN** 系统生成 `Overview` 页面
- **THEN** 该页面 MUST 先具备面向该 parent unit 的 research contract
- **THEN** compose MUST 依据该 research contract 组织 section，而不是只把 `SystemResearch` 插入固定骨架

#### Scenario: domain index 不再只靠固定骨架消费 child 摘要
- **WHEN** 系统生成某个 domain 的 `DomainIndex`
- **THEN** 该页面 MUST 具备该 parent unit 自己的 `section_plan`
- **THEN** compose MUST 消费 child-backed evidence、citation 或 diagram 摘要，而不是只把 child summary 拼进固定 section

### Requirement: 父页 compose 输入必须按 child rollup 逐层上卷
系统 MUST 让 parent compose 输入以 child rollup 的形式逐层上卷。高层父页 MUST 通过直接 child unit 持久化出的 rollup 消费下层结果，而不是跨层直接抓取更深层 leaf 输入。上卷结果 MUST 至少包含 child digest、section-scoped citation digest、diagram digest、key sources 和 readiness 摘要。

#### Scenario: overview 通过 domain rollup 消费下层结果
- **WHEN** `Overview` 或 `Architecture` 需要消费 domain 下更深层 leaf 的研究结果
- **THEN** 系统 MUST 先由各 `DomainIndex` 或中间 parent unit 上卷出 child rollup
- **THEN** `Overview` 或 `Architecture` MUST 只消费这些逐层上卷结果
- **THEN** 系统 MUST NOT 让高层父页直接越过中间 parent 节点读取深层 leaf 输入

#### Scenario: config surface 父页消费 child rollup 而非子页摘要拼接
- **WHEN** 某个 `config_surface` parent unit 存在多个 child unit
- **THEN** parent compose 输入 MUST 包含这些 child unit 的 rollup 摘要
- **THEN** 系统 MUST NOT 仅通过 `PageDigest.summary` 或 `render_child_digest_summary` 直接拼出父页正文
