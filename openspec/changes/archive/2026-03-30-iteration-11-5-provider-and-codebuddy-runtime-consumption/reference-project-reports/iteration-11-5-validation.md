# 11.5 场景验证说明

## 本轮范围

- 本轮只验证 `Provider / CodeBuddy` 如何消费当前阶段 `wiki-runtime`。
- 本轮已收口：
  - `status` preflight
  - `query` trust/profile
  - 长流程终态的 `runtime_summary / llm_execution_mode`
  - CodeBuddy 对 `usage / runtime_summary / blocker_hint` 的薄消费
- 本轮未纳入：
  - knowledge 正式消费
  - page 正式消费
  - CodeBuddy TS 层新的 Wiki 业务状态机

## 运行的验证

- Rust 编译检查：
  - `cargo test -p wiki-runtime --no-run`
- Rust 运行时集成测试：
  - `cargo test -p wiki-runtime --test runtime`
- CodeBuddy 测试：
  - `pnpm test` in `agents/codebuddy`
- 样本仓库 init 验证：
  - `node scripts/run-test-projects.mjs storybook dagger`

## 结果

- `cargo test -p wiki-runtime --no-run` 通过
- `cargo test -p wiki-runtime --test runtime` 通过，`96` 个测试全部通过
- `agents/codebuddy` 的 `15` 个测试全部通过
- `storybook` init 通过：
  - `226` pages
  - `6790` symbols
  - `26064` edges
- `dagger` init 通过：
  - `81` pages
  - `17054` symbols
  - `75773` edges

## 结论

- 当前阶段宿主已经可以只消费 `wiki-runtime` 的 preflight、query profile 和长流程终态 profile，而不需要在 TS 层重建 Wiki 业务语义。
- `status` 没有被扩成“真实执行路径探针”；真实 `llm_execution_mode` 仍只出现在 `init / update / rebuild` 终态。
- knowledge/page 的正式消费仍明确留在后续迭代，不在 11.5 内提前定版。

## 注释检查

- 本轮新增和修改的 Rust/TS 公共类型、字段与关键函数均补了中文注释。
- 过程内注释只用于解释 profile 投影、终态收口和错误路径，不存在机械复述代码的注释。
