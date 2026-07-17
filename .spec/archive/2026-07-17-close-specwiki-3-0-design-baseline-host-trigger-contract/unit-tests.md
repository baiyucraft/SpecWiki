# close-specwiki-3-0-design-baseline-host-trigger-contract 单元测试设计

## 测试总览

本文件定义 TDD Red-Green-Refactor 蓝图。UT-001..UT-008 分别覆盖 trigger 对象语言、语义 reducer、corpus、host role/capabilities、Codex-first skills、CodeBuddy parser、可执行 hook 与跨层 drift guard；所有用例回链 `system-tests.md` 和 `tasks.md`。

## 单元测试用例

### UT-001 TriggerDecision invariant 与 action policy

**目标行为**

共享合同暴露三态、闭集 reason/evidence 和 query/status semantic-or-explicit、mutation explicit-only policy，并拒绝非法 target 组合。

**关联**

- Design: 共享 trigger kernel、Action policy 与 reducer precedence、Decision invariants
- 系统测试用例: ST-002、ST-006
- Tasks: 1.1 Red / 1.2 Green / 1.3 Refactor

**Files**

- Test: `packages/spec-wiki/src/agents/shared/triggerContract.test.ts`
- Modify: `packages/spec-wiki/src/agents/shared/triggerContract.ts`
- Reference: `packages/spec-wiki/src/wikiActions.ts`

**测试代码蓝图**

```typescript
test('exposes closed policies and rejects invalid decisions', () => {
  expect(getTriggerActionPolicy('query')).toBe('semantic_or_explicit');
  expect(getTriggerActionPolicy('rebuild')).toBe('explicit_only');
  expect(() => validateTriggerDecision({ decision: 'should_trigger', targetAction: null, ...base }))
    .toThrow(/targetAction/);
});
```

**测试数据 / Fixture / Mock 边界**

- 使用 `PublicWikiAction` 真实闭集；不 mock evaluator。

**运行命令**

`pnpm --dir packages/spec-wiki test -- src/agents/shared/triggerContract.test.ts -t "exposes closed policies"`

**预期 Red 失败**

- 失败测试名: `exposes closed policies and rejects invalid decisions`
- 关键错误 / 断言差异: 无法导入 `triggerContract.ts` 或缺少 policy/invariant API。
- 失败原因: 共享 trigger 对象语言尚未实现。

**Green 通过条件**

- policy 与非法 decision 断言全部通过，action 闭集不复制。

**Refactor 守卫**

- 公开类型保持 readonly、闭集常量与 `PublicWikiAction` 单一来源不变。

### UT-002 Normalization、precedence 与中英文语义分类

**目标行为**

`evaluateHostTrigger` 确定性处理显式 action、语义 query/status、opt-out、冲突、缺 term、近碰撞和 invalid input。

**关联**

- Design: 共享 trigger kernel、Action policy 与 reducer precedence
- 系统测试用例: ST-002、ST-003、ST-006
- Tasks: 1.4 Red / 1.5 Green / 1.6 Refactor

**Files**

- Test: `packages/spec-wiki/src/agents/shared/triggerContract.test.ts`
- Modify: `packages/spec-wiki/src/agents/shared/triggerContract.ts`
- Reference: `design.md` reducer precedence

**测试代码蓝图**

```typescript
test.each([
  ['帮我定位认证模块调用路径', 'should_trigger', 'query'],
  ['show whether the repo wiki is ready', 'should_trigger', 'status'],
  ['do not query the wiki', 'should_not_trigger', null],
  ['module.exports', 'should_not_trigger', null],
])('classifies %s', (text, decision, targetAction) => {
  expect(evaluateHostTrigger({ kind: 'user_prompt', text })).toMatchObject({ decision, targetAction });
});
```

**测试数据 / Fixture / Mock 边界**

- 纯字符串 table；无文件、CLI、host mock。

**运行命令**

`pnpm --dir packages/spec-wiki test -- src/agents/shared/triggerContract.test.ts -t "classifies"`

**预期 Red 失败**

- 失败测试名: `classifies ...`
- 关键错误 / 断言差异: evaluator 不存在或所有输入无法产生预期三态。
- 失败原因: normalization/rules/reducer 尚未实现。

**Green 通过条件**

- NFKC、大小写、空白、标点、词边界与冲突 precedence 全部稳定。

**Refactor 守卫**

- evaluator 保持无 I/O、无 host id、evidence 稳定排序且不保存 raw prompt。

### UT-003 Corpus schema、版本与三宿主语义 conformance

**目标行为**

严格解析 v1 corpus，并对全部 semantic cases 运行相同 decision 断言。

**关联**

- Design: 版本化 corpus 与分层 conformance
- 系统测试用例: ST-003
- Tasks: 2.1 Red / 2.2 Green / 2.3 Refactor

**Files**

- Test: `packages/spec-wiki/src/agents/shared/triggerCorpus.test.ts`
- Modify: `packages/spec-wiki/src/agents/shared/triggerCorpus.ts`、`packages/spec-wiki/tests/fixtures/host-trigger-corpus.v1.json`
- Reference: `packages/spec-wiki/src/agents/shared/triggerContract.ts`

**测试代码蓝图**

```typescript
test('validates v1 corpus and applies every semantic case to every host', () => {
  const corpus = parseHostTriggerCorpus(readFixture());
  for (const testCase of corpus.semanticCases)
    for (const host of HOSTS)
      expect(projectTriggerDecision(host.id, testCase.input)).toMatchObject(testCase.expected);
});
```

**测试数据 / Fixture / Mock 边界**

- 真实 JSON fixture；禁止运行时宽松 coercion。

**运行命令**

`pnpm --dir packages/spec-wiki test -- src/agents/shared/triggerCorpus.test.ts`

**预期 Red 失败**

- 失败测试名: `validates v1 corpus and applies every semantic case to every host`
- 关键错误 / 断言差异: parser/fixture/projector 不存在。
- 失败原因: 版本化 corpus 合同尚未落地。

**Green 通过条件**

- zh-CN/en、三态、近碰撞、冲突、缺 term 与 invalid adapter cases 可解析且断言一致。

**Refactor 守卫**

- schema major、contractVersion、revision 职责分离；semantic 与 adapter cases 不混层。

### UT-004 Host compatibility role、capabilities 与 asset validator

**目标行为**

Registry 恰有一个 Codex reference，并在 production asset build 中拒绝 capability/asset 漂移。

**关联**

- Design: Host compatibility 与 capability matrix
- 系统测试用例: ST-001、ST-008
- Tasks: 2.4 Red / 2.5 Green / 2.6 Refactor

**Files**

- Test: `packages/spec-wiki/src/agents/shared/hostAssets.test.ts`
- Modify: `packages/spec-wiki/src/agents/shared/hosts.ts`、`hostAssets.ts`
- Reference: 三宿主 `assets.ts`

**测试代码蓝图**

```typescript
test('declares Codex as the only reference and validates assets', () => {
  expect(HOSTS.filter(host => host.compatibilityRole === 'reference').map(host => host.id)).toEqual(['codex']);
  for (const host of HOSTS)
    expect(() => buildHostBootstrapAssets('E:\\demo', process.env, host.id)).not.toThrow();
  expect(() => validateHostAssetsAgainstCapabilities(codebuddy, skillsOnly)).toThrow(/hooks/);
});
```

**测试数据 / Fixture / Mock 边界**

- 临时/内存 asset arrays；不写真实仓库。

**运行命令**

`pnpm --dir packages/spec-wiki test -- src/agents/shared/hostAssets.test.ts`

**预期 Red 失败**

- 失败测试名: `declares Codex as the only reference and validates assets`
- 关键错误 / 断言差异: `compatibilityRole/triggerCapabilities/validator` 不存在。
- 失败原因: registry/capability matrix 尚未实现。

**Green 通过条件**

- Codex/Claude/CodeBuddy 声明与真实 skill/hook/settings assets 一致，build 返回前执行 validator。

**Refactor 守卫**

- 保留当前最小 host switch，不扩大为完整 HostAdapter 重构。

### UT-005 Codex reference skills 与 compatible guidance parity

**目标行为**

Codex skills 先满足 reference guidance；Claude/CodeBuddy 复用同一 policy，mutation skills 明确 explicit-only，query 只含 canonical fields。

**关联**

- Design: Codex reference projection、Shared guidance projection
- 系统测试用例: ST-004、ST-006
- Tasks: 3.1 Red / 3.2 Green / 3.3 Refactor

**Files**

- Test: `packages/spec-wiki/src/agents/shared/commandAssets.test.ts`
- Modify: `packages/spec-wiki/src/agents/shared/commandAssets.ts`
- Reference: `triggerContract.ts`、三宿主 `assets.ts`

**测试代码蓝图**

```typescript
test('projects Codex-first trigger guidance without host-specific policy drift', () => {
  const query = renderHostActionSkill('query');
  const rebuild = renderHostActionSkill('rebuild');
  expect(query).toContain('semantic or explicit');
  expect(rebuild).toContain('explicit request');
  expect(renderCodeBuddyActionSkill('query')).toContain('`route_groups`');
  expect(query).not.toMatch(/matched_pages|intent|scope|traversal/);
});
```

**测试数据 / Fixture / Mock 边界**

- 使用真实 renderer；不做 snapshot 大段复制。

**运行命令**

`pnpm --dir packages/spec-wiki test -- src/agents/shared/commandAssets.test.ts`

**预期 Red 失败**

- 失败测试名: `projects Codex-first trigger guidance without host-specific policy drift`
- 关键错误 / 断言差异: 当前 renderer 未消费 trigger policy，mutation explicit-only marker 缺失。
- 失败原因: guidance projection 尚未接入共享 contract。

**Green 通过条件**

- 六 action guidance 从共享 policy 生成，query/status 与 mutation 边界清晰。

**Refactor 守卫**

- Codex/Claude 共用 renderer；CodeBuddy wrapper 只改变合法外壳，不复制 policy。

### UT-006 CodeBuddy v1 event parser 与 delivery

**目标行为**

Parser 只接受 `hook_event_name=UserPromptSubmit` 和 string `user_prompt`，并把 decision 映射为 action/clarification/none。

**关联**

- Design: CodeBuddy structured parser 与 delivery
- 系统测试用例: ST-005、ST-006
- Tasks: 3.4 Red / 3.5 Green / 3.6 Refactor

**Files**

- Test: `packages/spec-wiki/src/agents/codebuddy/triggerAdapter.test.ts`
- Modify: `packages/spec-wiki/src/agents/codebuddy/triggerAdapter.ts`
- Reference: CodeBuddy 官方 hook-development `user_prompt` 资料

**测试代码蓝图**

```typescript
test.each([
  [{ hook_event_name: 'UserPromptSubmit', user_prompt: '帮我定位认证入口' }, 'action_context'],
  [{ hook_event_name: 'UserPromptSubmit', user_prompt: '不要查询 wiki' }, 'none'],
  [{ hook_event_name: 'UserPromptSubmit', prompt: 'query auth' }, 'none'],
])('parses only the supported envelope', (event, delivery) => {
  expect(evaluateCodeBuddyUserPrompt(JSON.stringify(event)).delivery).toBe(delivery);
});
```

**测试数据 / Fixture / Mock 边界**

- 使用官方字段形状的最小 JSON；不读取 transcript/session id。

**运行命令**

`pnpm --dir packages/spec-wiki test -- src/agents/codebuddy/triggerAdapter.test.ts -t "parses only"`

**预期 Red 失败**

- 失败测试名: `parses only the supported envelope`
- 关键错误 / 断言差异: adapter 不存在，当前 hook 扫描 raw stdin substring。
- 失败原因: 结构化 parser/delivery 尚未实现。

**Green 通过条件**

- 只接受 `user_prompt`；malformed/unknown/non-string 返回 invalid/none；不抛宿主阻断错误。

**Refactor 守卫**

- 不新增 `prompt/userPrompt` fallback，不携带 session identity。

### UT-007 生成 CodeBuddy hook 与 shared evaluator 全 corpus 对拍

**目标行为**

真实生成 `.mjs` 对合法/negative/ambiguous/malformed input 的输出与库 decision delivery 一致，SessionStart 仅 orientation。

**关联**

- Design: CodeBuddy executable hook、SessionStart 分离
- 系统测试用例: ST-005、ST-006、ST-008
- Tasks: 4.1 Red / 4.2 Green / 4.3 Refactor

**Files**

- Test: `packages/spec-wiki/src/agents/codebuddy/assets.test.ts`
- Modify: `packages/spec-wiki/src/agents/shared/commandAssets.ts`、`packages/spec-wiki/src/agents/codebuddy/assets.ts`
- Reference: `triggerAdapter.ts`、trigger corpus

**测试代码蓝图**

```typescript
test('executes generated hooks against adapter corpus', () => {
  const script = writeTempHook(renderCodeBuddyHookScript('UserPromptSubmit'));
  for (const testCase of corpus.adapterCases) {
    const run = spawnSync(process.execPath, [script], { input: testCase.stdin, encoding: 'utf8' });
    expect(JSON.parse(run.stdout)).toEqual(expectedHookPayload(testCase));
  }
});
```

**测试数据 / Fixture / Mock 边界**

- 临时 `.mjs` 和 Node 子进程；不 mock stdin/stdout，不访问网络。

**运行命令**

`pnpm --dir packages/spec-wiki test -- src/agents/codebuddy/assets.test.ts -t "executes generated hooks"`

**预期 Red 失败**

- 失败测试名: `executes generated hooks against adapter corpus`
- 关键错误 / 断言差异: 当前 raw substring hook 会误触发字段/近碰撞并无法识别 malformed envelope。
- 失败原因: generated hook 尚未使用结构化 v1 rules。

**Green 通过条件**

- 所有 adapter cases exit 0、输出唯一 JSON；decision/delivery parity 成立。

**Refactor 守卫**

- 生成脚本不调用 CLI、不读取文件、不形成第二份宿主私有关键词表。

### UT-008 Bootstrap 与跨层 drift guard

**目标行为**

三宿主 bootstrap/distribution 保持幂等，旧 hook被覆盖，源码/fixtures/assets 不泄漏旧 query 或 session 字段。

**关联**

- Design: 迁移与兼容、验证思路、三层正交
- 系统测试用例: ST-007、ST-008
- Tasks: 4.4 Red / 4.5 Green / 4.6 Refactor

**Files**

- Test: `scripts/tests/host-trigger-contract.test.ts`、`packages/spec-wiki/src/bootstrap.test.ts`
- Modify: `scripts/tests/host-trigger-contract.test.ts`、相关 asset/registry 实现
- Reference: Runtime Query 与 reliability capability Wiki

**测试代码蓝图**

```typescript
test('keeps trigger artifacts free of runtime-private and durable session fields', () => {
  for (const file of triggerOwnedFiles()) {
    const text = readFileSync(file, 'utf8');
    expect(text).not.toMatch(/matched_pages|provenance_summary|session_summary|recent_turns|tool_artifact_refs/);
  }
});
```

**测试数据 / Fixture / Mock 边界**

- 只扫描明确列出的 trigger-owned source/fixture/generated strings；不扫描历史 archive。

**运行命令**

`pnpm exec vitest run scripts/tests/host-trigger-contract.test.ts packages/spec-wiki/src/bootstrap.test.ts`

**预期 Red 失败**

- 失败测试名: `keeps trigger artifacts free of runtime-private and durable session fields`
- 关键错误 / 断言差异: 缺少 contract test；当前 generated raw hook/capability metadata 尚未受 drift gate 约束。
- 失败原因: 跨层自动化门禁尚未实现。

**Green 通过条件**

- bootstrap、asset validation、query/session forbidden markers 和 Codex-first role 全部通过。

**Refactor 守卫**

- 历史 `.spec/archive/**` 保持排除；基础 bridge transport 不被误删或误报为 production research bridge。

## 测试辅助边界

- 新增一个版本化 JSON corpus、最小读取 helper 和 Node executable-hook helper。
- 可使用临时目录与 `spawnSync(process.execPath, ...)`；不得访问网络、用户真实 CodeBuddy transcript 或 provider。
- Red 证据以精确 focused command 的失败输出记录到 tasks/checklist 过程，不写入本蓝图。

## 不纳入单元测试的内容

- 外部宿主模型实际自动选 skill 的准确率；由项目资产 conformance 替代。
- production research bridge 与 provider session 实现；本 change 只做静态分层守卫。
