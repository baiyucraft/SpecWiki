# build-specwiki-lite 单元测试设计

## 测试总览

以下 TDD 蓝图覆盖核心纯函数和文件系统边界；ST-001/ST-008 的完整打包与自举由系统测试承担。

## 单元测试用例

### UT-001 资产同步遵守 ownership

**目标行为**

默认只补缺失，force 只覆盖 managed baseline，Skills 始终同步，自定义页面始终保留。

**关联**

- Design: Init 与 Update
- 系统测试用例: ST-002
- Tasks: 1.1 Red / 1.2 Green / 1.3 Refactor

**Files**

- Test: `packages/spec-wiki-lite/src/core/assets/sync.test.ts`
- Modify: `packages/spec-wiki-lite/src/core/assets/sync.ts`
- Reference: `design.md`

**测试代码蓝图**

```typescript
test("preserves scaffold and user pages while force only replaces managed baselines", async () => {
  const root = await createTempProject();
  await syncProjectAssets(root, { force: false });
  await writeFile(join(root, ".wiki/custom.md"), "custom");
  const report = await syncProjectAssets(root, { force: true });
  expect(await readFile(join(root, ".wiki/custom.md"), "utf8")).toBe("custom");
  expect(report.preserved).toContain(".wiki/custom.md");
});
```

**测试数据 / Fixture / Mock 边界**

- 真实临时目录，不 mock fs。

**运行命令**

`pnpm --dir packages/spec-wiki-lite vitest run src/core/assets/sync.test.ts`

**预期 Red 失败**

- 失败测试名: `preserves scaffold and user pages while force only replaces managed baselines`
- 关键错误 / 断言差异: 模块不存在或自定义页面被覆盖。
- 失败原因: 新 ownership 同步器尚未实现。

**Green 通过条件**

- created/updated/unchanged/preserved 分类与文件内容一致。

**Refactor 守卫**

- init/update 复用同一同步器，不增加宿主分支。

### UT-002 Wiki inspector 返回稳定 issue

**目标行为**

INDEX、frontmatter、链接、orphan 和重复 SSOT 均产生可定位 issue。

**关联**

- Design: Wiki Status
- 系统测试用例: ST-003
- Tasks: 2.1 Red / 2.2 Green / 2.3 Refactor

**Files**

- Test: `packages/spec-wiki-lite/src/core/wiki/inspect.test.ts`
- Modify: `packages/spec-wiki-lite/src/core/wiki/inspect.ts`
- Reference: `design.md`

**测试代码蓝图**

```typescript
test("reports structural wiki issues without runtime fields", async () => {
  const report = await inspectWiki(await createBrokenWikiFixture());
  expect(report.issues.map(issue => issue.kind)).toEqual(expect.arrayContaining([
    "missing_index", "invalid_frontmatter", "broken_link", "orphan_page", "duplicate_ssot",
  ]));
  expect(report).not.toHaveProperty("runtime_readiness");
});
```

**测试数据 / Fixture / Mock 边界**

- 真实 Markdown fixtures；不解析源码。

**运行命令**

`pnpm --dir packages/spec-wiki-lite vitest run src/core/wiki/inspect.test.ts`

**预期 Red 失败**

- 失败测试名: `reports structural wiki issues without runtime fields`
- 关键错误 / 断言差异: inspector 模块不存在。
- 失败原因: Lite Wiki inspector 尚未实现。

**Green 通过条件**

- 五类 issue 均稳定返回路径和消息。

**Refactor 守卫**

- 排除旧隐藏 runtime 目录，不把 archive 当 current Wiki。

### UT-003 Metadata required artifacts 随 stage 单调增加

**目标行为**

每个 stage 只要求已经到达的 artifact，并拒绝未知 stage/shape。

**关联**

- Design: `.spec` Workflow
- 系统测试用例: ST-004
- Tasks: 3.1 Red / 3.2 Green / 3.3 Refactor

**Files**

- Test: `packages/spec-wiki-lite/src/core/change/validate.test.ts`
- Modify: `packages/spec-wiki-lite/src/core/change/validate.ts`
- Reference: `.wiki/00-文档约定/02-UniSpec开发规范.md`

**测试代码蓝图**

```typescript
test.each(["proposal", "design", "cases", "tasks", "implementation", "review", "verification"])(
  "requires the artifact prefix for %s", async (stage) => {
    const result = await validateChange(await createStageFixture(stage));
    expect(result.requiredArtifacts).toEqual(expectedArtifactsFor(stage));
  },
);
```

**测试数据 / Fixture / Mock 边界**

- 临时 `.spec` fixture；YAML 使用真实 parser。

**运行命令**

`pnpm --dir packages/spec-wiki-lite vitest run src/core/change/validate.test.ts`

**预期 Red 失败**

- 失败测试名: `requires the artifact prefix for *`
- 关键错误 / 断言差异: validator 不存在。
- 失败原因: TS stage schema 尚未实现。

**Green 通过条件**

- stage 闭集、required artifacts 和 blocking issues 正确。

**Refactor 守卫**

- show/status/archive 复用同一 schema。

### UT-004 Verification 证据必须 full/pass

**目标行为**

verification change 只有 review/test frontmatter 都是 full/pass 才可归档。

**关联**

- Design: `.spec` Workflow
- 系统测试用例: ST-004, ST-005
- Tasks: 3.4 Red / 3.5 Green / 3.6 Refactor

**Files**

- Test: `packages/spec-wiki-lite/src/core/change/validate.test.ts`
- Modify: `packages/spec-wiki-lite/src/core/change/validate.ts`
- Reference: `.wiki/00-文档约定/02-UniSpec开发规范.md`

**测试代码蓝图**

```typescript
test("blocks verification unless both reports are full pass", async () => {
  const result = await validateChange(await createVerificationFixture({ review: "pass", verification: "skipped" }));
  expect(result.valid).toBe(false);
  expect(result.issues).toContainEqual(expect.objectContaining({ kind: "verification_not_passed" }));
});
```

**测试数据 / Fixture / Mock 边界**

- YAML frontmatter fixtures，无 mock。

**运行命令**

`pnpm --dir packages/spec-wiki-lite vitest run src/core/change/validate.test.ts -t "full pass"`

**预期 Red 失败**

- 失败测试名: `blocks verification unless both reports are full pass`
- 关键错误 / 断言差异: 无 blocking issue。
- 失败原因: report evidence parser 尚未实现。

**Green 通过条件**

- partial/fail/skipped/malformed 均阻止归档。

**Refactor 守卫**

- 使用 YAML parser，不用正则解析 frontmatter 字段。

### UT-005 Archive 原子移动并拒绝冲突

**目标行为**

archive 只在 validate 通过时 rename 到 dated target，目标已存在不覆盖。

**关联**

- Design: `.spec` Workflow、原子性
- 系统测试用例: ST-005
- Tasks: 4.1 Red / 4.2 Green / 4.3 Refactor

**Files**

- Test: `packages/spec-wiki-lite/src/core/change/archive.test.ts`
- Modify: `packages/spec-wiki-lite/src/core/change/archive.ts`
- Reference: `design.md`

**测试代码蓝图**

```typescript
test("moves a valid change once and refuses an existing target", async () => {
  const first = await archiveChange(root, id, fixedClock);
  await expect(archiveChange(root, id, fixedClock)).rejects.toThrow();
  expect(first.archivedTo).toBe(`.spec/archive/2026-07-27-${id}`);
});
```

**测试数据 / Fixture / Mock 边界**

- 注入 clock；真实临时目录。

**运行命令**

`pnpm --dir packages/spec-wiki-lite vitest run src/core/change/archive.test.ts`

**预期 Red 失败**

- 失败测试名: `moves a valid change once and refuses an existing target`
- 关键错误 / 断言差异: archive 模块不存在。
- 失败原因: TS archive core 尚未实现。

**Green 通过条件**

- source 消失、target 字节保留、重复调用失败。

**Refactor 守卫**

- 禁止 copy-then-delete 和覆盖目标。

### UT-006 Child archive 同步 parent

**目标行为**

child 成功归档后，parent metadata 和 split marker 同步；不一致时整体失败。

**关联**

- Design: `.spec` Workflow
- 系统测试用例: ST-005
- Tasks: 4.4 Red / 4.5 Green / 4.6 Refactor

**Files**

- Test: `packages/spec-wiki-lite/src/core/change/archive.test.ts`
- Modify: `packages/spec-wiki-lite/src/core/change/archive.ts`
- Reference: `design.md`

**测试代码蓝图**

```typescript
test("archives a child and records the same target in its active parent", async () => {
  await archiveChange(root, childId, fixedClock);
  const parent = await readMetadata(root, parentId);
  expect(parent.multiChange.children[0].archiveStatus).toBe("archived");
});
```

**测试数据 / Fixture / Mock 边界**

- parent/child fixture；真实 YAML document。

**运行命令**

`pnpm --dir packages/spec-wiki-lite vitest run src/core/change/archive.test.ts -t "active parent"`

**预期 Red 失败**

- 失败测试名: `archives a child and records the same target in its active parent`
- 关键错误 / 断言差异: parent 未更新。
- 失败原因: parent synchronization 尚未实现。

**Green 通过条件**

- meta/split/target 三方一致。

**Refactor 守卫**

- 未知字段和 YAML comments 尽量保留；失败不能留下半更新 parent。

### UT-007 Path guard 拒绝所有逃逸

**目标行为**

非法 id、绝对路径、`..` 和 symlink escape 在 I/O 前被拒绝。

**关联**

- Design: 路径安全
- 系统测试用例: ST-006
- Tasks: 5.1 Red / 5.2 Green / 5.3 Refactor

**Files**

- Test: `packages/spec-wiki-lite/src/core/path.test.ts`
- Modify: `packages/spec-wiki-lite/src/core/path.ts`
- Reference: `design.md`

**测试代码蓝图**

```typescript
test.each(["../escape", "/absolute", "C:\\escape", "wiki/../../escape"])(
  "rejects unsafe identifier or path %s", (input) => expect(() => resolveSafePath(root, input)).toThrow(),
);
```

**测试数据 / Fixture / Mock 边界**

- 平台条件化 symlink/junction fixture；无外部目录写入。

**运行命令**

`pnpm --dir packages/spec-wiki-lite vitest run src/core/path.test.ts`

**预期 Red 失败**

- 失败测试名: `rejects unsafe identifier or path *`
- 关键错误 / 断言差异: unsafe path 被返回。
- 失败原因: canonical guard 尚未实现。

**Green 通过条件**

- 所有逃逸抛出稳定 domain error。

**Refactor 守卫**

- lexical 与 realpath containment 都保留。

### UT-008 CLI 只暴露 Lite 合同

**目标行为**

help/parser 只接受六个新命令和 Codex host，并返回固定退出码。

**关联**

- Design: CLI
- 系统测试用例: ST-001, ST-002, ST-004
- Tasks: 6.1 Red / 6.2 Green / 6.3 Refactor

**Files**

- Test: `packages/spec-wiki-lite/src/cli.test.ts`
- Modify: `packages/spec-wiki-lite/src/cli.ts`
- Reference: `design.md`

**测试代码蓝图**

```typescript
test("help exposes only the SpecWiki Lite commands", async () => {
  const output = await runForOutput(["--help"]);
  expect(output.stdout).toContain("spec-wiki-lite init");
  expect(output.stdout).not.toMatch(/query|sync|rebuild/);
});
```

**测试数据 / Fixture / Mock 边界**

- 注入 CLI I/O；文件行为使用临时目录。

**运行命令**

`pnpm --dir packages/spec-wiki-lite vitest run src/cli.test.ts`

**预期 Red 失败**

- 失败测试名: `help exposes only the SpecWiki Lite commands`
- 关键错误 / 断言差异: 仍显示 `spec-wiki query` 等旧命令。
- 失败原因: CLI 尚未重写和改名。

**Green 通过条件**

- 六命令、参数和退出码全部符合设计。

**Refactor 守卫**

- human/JSON 使用同一 domain result，不复制业务判断。

## 测试辅助边界

- 允许新增共享 temp project、fixture writer 和 injected clock helper。
- 禁止 mock 掉 YAML parser、真实 fs rename 或 package inventory。

## 不纳入单元测试的内容

- tarball 安装和 bin 执行由 ST-001 系统测试覆盖。
- current Wiki/源码残留扫描由 ST-007 root contract test 覆盖。
- 自举归档由 ST-008 最终 verification 覆盖。
