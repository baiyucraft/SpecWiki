## ADDED Requirements

### Requirement: 验证必须覆盖 dossier、provider research session 与 cold/warm 对照
系统 MUST 提供自动化测试和项目集验证，覆盖 dossier/child rollup 稳定性、provider bounded research session、tool schema、phase budget 裁剪和 cold/warm run 差异。测试报告 MUST 明确区分 cold run 与 warm run，而不是把两者混在同一结论里。CodeBuddy Agent 侧验证不属于 9.2 的必做范围。

#### Scenario: 测试区分 cold run 与 warm run
- **WHEN** 验证脚本对同一项目执行两轮启用 LLM 的 workflow
- **THEN** 报告 MUST 明确标记哪一轮是 cold run、哪一轮是 warm run
- **THEN** 报告 MUST 能说明 cache mode 与真实请求数的差异

#### Scenario: 验证 provider bounded research session 与 tool schema
- **WHEN** 测试通过 provider-tools 执行 `module` / `topic` 页 research session
- **THEN** 测试 MUST 观察到 session 事件、tool 调用和结构化 `PageResearchResult`
- **THEN** 测试 MUST 观察到最终页面仍由 deterministic renderer 落盘

### Requirement: 验证必须覆盖实时 usage 输出与同类型 gate 批量化
系统 MUST 在自动化测试和项目报告中覆盖实时 usage 输出、phase budget 生效和同类型 uncertainty gate 批量化行为。普通模式下的 usage 输出 MUST 可被脚本直接消费，不得要求人工去 debug trace 中核对。

#### Scenario: progress/event stream 可观测实时 usage
- **WHEN** 测试脚本执行启用 LLM 的长流程 workflow
- **THEN** 脚本 MUST 观察到 workflow 进行中持续刷新的 usage snapshot
- **THEN** 不得只有 workflow 结束后才一次性看到成本汇总

#### Scenario: file_purpose 等同类型 gate 以批量方式执行
- **WHEN** 测试在存在多条 `file_purpose` 候选的项目上执行 workflow
- **THEN** 报告 MUST 能区分批量 gate 请求与逐条请求
- **THEN** 测试 MUST 继续验证批量化后单条 fallback 和 cache 粒度未被破坏
