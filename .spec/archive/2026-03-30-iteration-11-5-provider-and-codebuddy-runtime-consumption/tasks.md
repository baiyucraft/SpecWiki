## 1. runtime 当前阶段消费 profile

- [x] 1.1 在 `crates/wiki-runtime` 中定义当前阶段宿主消费所需的 profile 投影对象，覆盖 `facts_ready`、`query_readiness`、`recommended_action`、可选 `runtime_summary / gate_summary`、预判型 `llm_mode_hint`。
- [x] 1.2 收口 `status` 输出，使其返回正式 preflight 结果，并明确可选摘要缺失时仍然成功。
- [x] 1.3 收口 `query` 输出，使其显式表达 `query_mode`、`query_trust`、`recommended_action` 与 page fallback provenance。

## 2. 长流程终态与事件流

- [x] 2.1 调整 `init / update / rebuild` 的终态结果，在适用时稳定回传 `runtime_summary`。
- [x] 2.2 仅在实际跑过 LLM/runtime 路径的长流程终态中回传真实 `llm_execution_mode`，避免把该字段错误扩散到 `status / query / sync`。
- [x] 2.3 补齐事件流中的 `usage`、runtime 摘要消费路径与错误路径 blocker 线索，保持现有 NDJSON 外壳兼容。

## 3. CodeBuddy 薄消费改造

- [x] 3.1 更新 `agents/codebuddy/src/runtime/parseResult.ts`，让其能解析当前阶段 runtime profile、`usage`、`runtime_summary` 与真实 `llm_execution_mode`。
- [x] 3.2 更新 `agents/codebuddy/src/runtime/invokeCore.ts` 与相关 tool 类型，使 CodeBuddy 薄消费 preflight、query 和长流程终态，但不引入新的 Wiki 业务状态机。
- [x] 3.3 收口 `wikiStatus`、`wikiQuery` 及长流程工具的返回类型，确保可选摘要缺失和 provider direct 路径都能被正确透传。

## 4. 测试、场景验证与注释检查

- [x] 4.1 为 `status`、`query` 与长流程终态增加 Rust 测试，覆盖 `recommended_action`、可选 `runtime_summary / gate_summary`、`query_trust` 与 `llm_execution_mode` 的边界。
- [x] 4.2 为 CodeBuddy 增加 TS 测试，覆盖 preflight 结果解析、query trust 透传、`usage` 消费与“只在终态里消费真实执行路径”的约束。
- [x] 4.3 运行与当前变更相关的 Rust/TS 测试，并补一份 11.5 的场景验证说明，明确 knowledge/page 正式消费仍留在后续迭代。
- [x] 4.4 单独执行一轮 `COMMENTING.md` 合规检查，确认本轮新增或修改代码的注释风格与粒度符合要求。
