# refactor-specwiki-around-contract-closure-cli-product-surface 单元测试设计

## 测试总览

本文件定义一级 CLI 收口的 TDD 蓝图。每个 UT 对应 design 能力点、ST 和 tasks；真实测试在 apply 阶段按 Red-Green-Refactor 写入。

## 单元测试用例

### UT-001 CommandSpec 解析与 help 分层

**目标行为**

默认/完整/per-command help 和未知命令具有固定命令集合与退出语义。

**关联**

- Design: 命令规格、Help 分层
- 系统测试用例: ST-001
- Tasks: 1.1 / 1.2 / 1.3

**Files**

- Test: `packages/spec-wiki/src/cli.test.ts`
- Modify: `packages/spec-wiki/src/cli.ts`, `packages/spec-wiki/src/cli/commandSpec.ts`
- Reference: `packages/spec-wiki/src/wikiActions.ts`

**测试代码蓝图**

```typescript
test('default and full help expose the intended top-level commands', async () => {
  expect(await runCli(['--help'])).toMatchObject({ code: 0 });
  expect(defaultOutput).toContain('init');
  expect(defaultOutput).not.toContain('archive');
  expect(fullOutput).toContain('validate <change-id>');
});
```

**测试数据 / Fixture / Mock 边界**

- mock stdout/stderr；不启动 runtime。

**运行命令**

`pnpm --filter spec-wiki test -- cli.test.ts`

**预期 Red 失败**

- 失败测试名: `default and full help expose the intended top-level commands`
- 关键错误 / 断言差异: 当前 help 仍显示 `wiki <action>` 且无 `--help-all`。
- 失败原因: 一级 CommandSpec 尚未实现。

**Green 通过条件**

- help/usage 断言全部通过。

**Refactor 守卫**

- 命令集合只有一个定义来源，archive 不进入列表。

### UT-002 参数与 usage error

**目标行为**

query、host、machine/help、bridge 参数组合按规格解析并返回 64。

**关联**

- Design: 命令规格、结构化错误
- 系统测试用例: ST-001, ST-002
- Tasks: 1.4 / 1.5 / 1.6

**Files**

- Test: `packages/spec-wiki/src/cli.test.ts`, `packages/spec-wiki/src/selectHosts.test.ts`
- Modify: `packages/spec-wiki/src/cli.ts`, `orchestration/init/selectHosts.ts`
- Reference: `agents/shared/hosts.ts`

**测试代码蓝图**

```typescript
test('machine init never prompts and host flags are mutually exclusive', async () => {
  expect(await runCli(['init', '--json', '--host', 'codex', '--hosts', 'claude'])).toBe(64);
  expect(promptMock).not.toHaveBeenCalled();
});
```

**测试数据 / Fixture / Mock 边界**

- 注入 host detector 与 prompt；不访问真实 HOME。

**运行命令**

`pnpm --filter spec-wiki test -- cli.test.ts selectHosts.test.ts`

**预期 Red 失败**

- 失败测试名: `machine init never prompts and host flags are mutually exclusive`
- 关键错误 / 断言差异: 当前只识别 `--tool/--tools`。
- 失败原因: 新参数合同缺失。

**Green 通过条件**

- 合法参数路由正确，非法组合稳定返回 64。

**Refactor 守卫**

- TTY 仅影响 human host selection，不影响 machine protocol。

### UT-003 CoreResponse errorKind 与 exit policy

**目标行为**

TS 严格解析 errorKind，集中映射 0/2/64/1。

**关联**

- Design: 结构化错误与退出码
- 系统测试用例: ST-002, ST-004
- Tasks: 2.1 / 2.2 / 2.3

**Files**

- Test: `packages/spec-wiki/src/index.test.ts`, `runtime/exitPolicy.test.ts`
- Modify: `runtime/parseResult.ts`, `runtime/exitPolicy.ts`
- Reference: `crates/wiki-runtime/src/transport/dto.rs`

**测试代码蓝图**

```typescript
test('maps usage, partial, invalid validation and failures without string matching', () => {
  expect(exitCodeFor(error('invalid_argument'))).toBe(64);
  expect(exitCodeFor(initPartial())).toBe(2);
  expect(exitCodeFor(validation(false))).toBe(2);
});
```

**测试数据 / Fixture / Mock 边界**

- 使用纯 DTO fixtures。

**运行命令**

`pnpm --filter spec-wiki test -- index.test.ts exitPolicy.test.ts`

**预期 Red 失败**

- 失败测试名: `maps usage, partial, invalid validation and failures without string matching`
- 关键错误 / 断言差异: 当前 CoreResponse 无 errorKind，forwarder 只有 0/1。
- 失败原因: 结构化退出策略未实现。

**Green 通过条件**

- 所有矩阵项按 DTO 映射。

**Refactor 守卫**

- 禁止解析 error message 文本。

### UT-004 NDJSON line parser 与唯一终态

**目标行为**

流处理按完整行解析并拒绝零/多/terminal 后事件。

**关联**

- Design: 事件流与输出
- 系统测试用例: ST-002, ST-003
- Tasks: 2.4 / 2.5 / 2.6

**Files**

- Test: `packages/spec-wiki/src/runtime/coreEventStream.test.ts`
- Modify: `runtime/coreEventStream.ts`, `runtime/forwardCore.ts`
- Reference: `runtime/invokeCore.ts`

**测试代码蓝图**

```typescript
test('rejects progress after terminal across arbitrary chunks', async () => {
  await expect(runChunks([resultChunk, progressChunk])).rejects.toThrow(/after terminal/);
});
```

**测试数据 / Fixture / Mock 边界**

- fake child stdout chunks；bridge stdin 使用 PassThrough。

**运行命令**

`pnpm --filter spec-wiki test -- coreEventStream.test.ts forwardCore.test.ts`

**预期 Red 失败**

- 失败测试名: `rejects progress after terminal across arbitrary chunks`
- 关键错误 / 断言差异: 当前 forwardCore 原样输出 chunk 并倒扫最后事件。
- 失败原因: 专用 stream parser 缺失。

**Green 通过条件**

- chunk、事件顺序和唯一终态测试通过。

**Refactor 守卫**

- bridge 保持机器模式和事件原样转发。

### UT-005 BootstrapReport 保留部分写入事实

**目标行为**

宿主资产中途失败返回 partial、已完成文件和 recovery hint。

**关联**

- Design: Host bootstrap
- 系统测试用例: ST-003
- Tasks: 3.1 / 3.2 / 3.3

**Files**

- Test: `packages/spec-wiki/src/bootstrap.test.ts`
- Modify: `orchestration/init/runInit.ts`
- Reference: `agents/shared/hostAssets.ts`

**测试代码蓝图**

```typescript
test('returns partial bootstrap facts when a later asset write fails', async () => {
  const report = await runBootstrapInit(failingWriterOptions);
  expect(report.outcome).toBe('partial');
  expect(report.hosts[0].files).toHaveLength(1);
});
```

**测试数据 / Fixture / Mock 边界**

- 临时目录；注入文件 writer 触发第二次失败。

**运行命令**

`pnpm --filter spec-wiki test -- bootstrap.test.ts`

**预期 Red 失败**

- 失败测试名: `returns partial bootstrap facts when a later asset write fails`
- 关键错误 / 断言差异: 当前函数直接 throw，无法返回已写事实。
- 失败原因: 结构化 bootstrap report 未实现。

**Green 通过条件**

- ready/partial/failed fixtures 均通过。

**Refactor 守卫**

- 写入仍幂等，不新增回滚。

### UT-006 Rust cli_init outcome 矩阵

**目标行为**

内部 `cli_init` 保持唯一终态并由 Rust 计算 ready/partial/failed。

**关联**

- Design: Unified init
- 系统测试用例: ST-003
- Tasks: 3.4 / 3.5 / 3.6

**Files**

- Test: `crates/wiki-runtime/tests/acceptance/command_contract.rs`
- Modify: `transport/dto.rs`, `transport/cli.rs`, `transport/json_rpc.rs`
- Reference: `workflows/init.rs`, `workflows/status.rs`

**测试代码蓝图**

```rust
#[test]
fn cli_init_projects_bootstrap_and_landing_into_one_outcome() {
    // dispatch cli_init with ready/partial bootstrap fixtures and assert one terminal DTO.
}
```

**测试数据 / Fixture / Mock 边界**

- Rust temp repo fixtures；不调用外部 LLM。

**运行命令**

`cargo test -p wiki-runtime --test acceptance command_contract`

**预期 Red 失败**

- 失败测试名: `cli_init_projects_bootstrap_and_landing_into_one_outcome`
- 关键错误 / 断言差异: unsupported_action:cli_init。
- 失败原因: 内部 action 和 DTO 未实现。

**Green 通过条件**

- outcome 矩阵、bootstrap validation、失败 data 测试通过。

**Refactor 守卫**

- 原 `init` action DTO 不变，`cli_init` 不进入公开 JS action。

### UT-007 Governance 单次 evaluation reports

**目标行为**

changes/change/validate 从同一次 live evaluation 构造 envelope。

**关联**

- Design: Governance transport
- 系统测试用例: ST-004
- Tasks: 4.1 / 4.2 / 4.3

**Files**

- Test: `crates/wiki-runtime/tests/governance_workflows.rs`
- Modify: `workflows/governance.rs`
- Reference: `wiki-model/src/domain/governance.rs`

**测试代码蓝图**

```rust
#[test]
fn governance_reports_share_one_live_summary() {
    // assert report summary/fingerprint and changes originate from one evaluation.
}
```

**测试数据 / Fixture / Mock 边界**

- 复用 governance temp fixtures。

**运行命令**

`cargo test -p wiki-runtime --test governance_workflows`

**预期 Red 失败**

- 失败测试名: `governance_reports_share_one_live_summary`
- 关键错误 / 断言差异: report API 不存在。
- 失败原因: 当前 service 方法分别 evaluate_live。

**Green 通过条件**

- 三类 report 和 not_enabled 语义通过。

**Refactor 守卫**

- policy 规则只存在于 GovernancePolicy。

### UT-008 Governance transport 与 TS parser

**目标行为**

changeId、envelope 和结构化错误跨 Rust/TS 一致。

**关联**

- Design: CoreCommand、Governance transport
- 系统测试用例: ST-004
- Tasks: 4.4 / 4.5 / 4.6

**Files**

- Test: `crates/wiki-runtime/tests/acceptance/command_contract.rs`, `packages/spec-wiki/src/index.test.ts`
- Modify: Rust transport、`runtime/parseResult.ts`
- Reference: `workflows/governance.rs`

**测试代码蓝图**

```typescript
test('parses governance changes/change/validate envelopes strictly', () => {
  expect(parseResult(changesFixture).data).toHaveProperty('changes');
});
```

**测试数据 / Fixture / Mock 边界**

- JSON fixtures 与 Rust temp repos。

**运行命令**

`cargo test -p wiki-runtime --test acceptance && pnpm --filter spec-wiki test -- index.test.ts`

**预期 Red 失败**

- 失败测试名: `parses governance changes/change/validate envelopes strictly`
- 关键错误 / 断言差异: CoreCommand 无 changeId，parser 不识别 envelope。
- 失败原因: transport/parser 未扩展。

**Green 通过条件**

- Rust/TS fixtures 对齐，invalid/not_found 可区分。

**Refactor 守卫**

- 不使用 `any` 或字段启发式识别。

### UT-009 Human renderer 不重算 DTO

**目标行为**

human renderer 对已知 DTO 做稳定翻译并保留 Rust outcome/action。

**关联**

- Design: 事件流与输出
- 系统测试用例: ST-002, ST-003, ST-004
- Tasks: 2.7 / 2.8 / 2.9

**Files**

- Test: `packages/spec-wiki/src/runtime/humanRenderer.test.ts`
- Modify: `runtime/humanRenderer.ts`
- Reference: `runtime/parseResult.ts`

**测试代码蓝图**

```typescript
test('renders provided outcome and recommended action verbatim', () => {
  expect(render(unifiedInitPartial)).toContain('partial');
  expect(render(statusFixture)).toContain('update');
});
```

**测试数据 / Fixture / Mock 边界**

- typed DTO fixtures。

**运行命令**

`pnpm --filter spec-wiki test -- humanRenderer.test.ts`

**预期 Red 失败**

- 失败测试名: `renders provided outcome and recommended action verbatim`
- 关键错误 / 断言差异: renderer 模块不存在。
- 失败原因: 当前 CLI 只透传 JSON。

**Green 通过条件**

- 各命令文本稳定且无重新计算分支。

**Refactor 守卫**

- renderer 输入必须先通过 parser。

### UT-010 宿主资产与旧命令扫描门禁

**目标行为**

生成资产使用一级命令，保留 identity，当前产品面无旧调用残留。

**关联**

- Design: 宿主资产、迁移门禁
- 系统测试用例: ST-005, ST-006
- Tasks: 5.1 / 5.2 / 5.3

**Files**

- Test: `packages/spec-wiki/src/bootstrap.test.ts`, `scripts/tests/cli-surface.test.ts`
- Modify: `workflowSemantics.ts`, `commandAssets.ts`, README/Wiki/tests
- Reference: `tools.ts`, host asset modules

**测试代码蓝图**

```typescript
test('generated host assets call top-level CLI while preserving identities', async () => {
  expect(asset).toContain('spec-wiki query "$ARGUMENTS"');
  expect(asset).not.toContain('spec-wiki wiki');
});
```

**测试数据 / Fixture / Mock 边界**

- 扫描排除 archive/upstream/release/build/deps。

**运行命令**

`pnpm --filter spec-wiki test -- bootstrap.test.ts && pnpm vitest run scripts/tests/cli-surface.test.ts`

**预期 Red 失败**

- 失败测试名: `generated host assets call top-level CLI while preserving identities`
- 关键错误 / 断言差异: 当前生成 `spec-wiki wiki ... --term`。
- 失败原因: 调用文本尚未迁移。

**Green 通过条件**

- asset 与 scan gate 全部通过。

**Refactor 守卫**

- `wikiInit/wiki-*` 等 identity 断言保持通过。

## 测试辅助边界

- 允许新增纯 DTO fixture、临时 repo helper、fake child process 和注入式文件 writer。
- 不 mock GovernancePolicy 结果；使用真实 fixture evaluation。

## 不纳入单元测试的内容

- README/Wiki 文案可读性由 review 检查；旧调用残留由静态扫描自动验证。
