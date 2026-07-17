# close-specwiki-3-0-design-baseline-documentation-closure 单元测试设计

## 测试总览

本文件定义单个 root Vitest 合同文件中的六组 TDD 蓝图。每组先在当前漂移状态上得到可解释 Red，再通过最小文档迁移得到 Green，最后在不扩大扫描范围的前提下重构 helper。测试直接读取真实仓库文件，不 mock 文件系统或 UniSpec 状态。

## 单元测试用例

### UT-001 Capability Purpose 与 inventory 闭合

**目标行为**

全部 capability 具有有效 Purpose，INDEX 与目录一致，family 不再是平行 capability identity。

**关联**

- Design: `功能设计 / 1. Capability inventory 收口`
- 系统测试用例: ST-001
- Tasks: 1.1 Red / 1.2 Green / 1.3 Refactor

**Files**

- Test: `scripts/tests/documentation-closure-contract.test.ts`
- Modify: `.wiki/05-规格基线/INDEX.md`、`.wiki/05-规格基线/capabilities/**`
- Reference: `scripts/tests/product-baseline-contract.test.ts`

**测试代码蓝图**

```typescript
test("capability purposes and inventory are closed", () => {
  expect(collectCapabilityPurposeIssues()).toEqual([]);
  expect(collectCapabilityInventoryIssues()).toEqual([]);
  expect(capabilityIds()).not.toContain("content-family-planner");
});
```

**测试数据 / Fixture / Mock 边界**

- 直接读取真实 capability directories、`spec.md` 和 INDEX；不使用 fixture 或 mock。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/documentation-closure-contract.test.ts -t "capability purposes and inventory"`

**预期 Red 失败**

- 失败测试名: `capability purposes and inventory are closed`
- 关键错误 / 断言差异: 报告 4 个归档 Purpose 占位、`content-family-planner` 仍存在或 INDEX/目录差异。
- 失败原因: documentation closure 尚未迁移 capability inventory。

**Green 通过条件**

- 所有 Purpose 合法、INDEX/目录相等、family 平行 capability 已删除且有效规则已迁移。

**Refactor 守卫**

- helper 只解析首个 Purpose section 和实际链接集合，不把正文措辞固化为快照。

### UT-002 Query 与 release contract 正交

**目标行为**

当前 query capability 只投影 canonical route groups，v0.2.0 release contract 有唯一 Wiki authority 且不冒充 released evidence。

**关联**

- Design: `功能设计 / 1. Capability inventory 收口`、`2. 长期 Wiki authority 收口`
- 系统测试用例: ST-002
- Tasks: 2.1 Red / 2.2 Green / 2.3 Refactor

**Files**

- Test: `scripts/tests/documentation-closure-contract.test.ts`
- Modify: `.wiki/05-规格基线/capabilities/wiki-bm25-query/spec.md`、`repo-wiki-runtime/spec.md`、`adapter-distribution/spec.md`、`repo-wiki-workflow/spec.md`、`.wiki/04-对外方法/02-v0.2.0发布合同.md`
- Reference: `scripts/tests/runtime-query-contract.test.ts`、`scripts/tests/product-baseline-contract.test.ts`

**测试代码蓝图**

```typescript
test("query and release projections use canonical authorities", () => {
  expect(collectQueryProjectionIssues()).toEqual([]);
  expect(collectReleaseContractIssues()).toEqual([]);
});
```

**测试数据 / Fixture / Mock 边界**

- 读取真实 Wiki authority、package/Cargo manifests；不访问 registry 或网络。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/documentation-closure-contract.test.ts -t "query and release"`

**预期 Red 失败**

- 失败测试名: `query and release projections use canonical authorities`
- 关键错误 / 断言差异: 缺少 release authority；query capability 仍存在旧正向 matched 字段；staging 被描述为真实发布。
- 失败原因: canonical contracts 尚未投影到 capability/release 文档。

**Green 通过条件**

- route groups 为唯一公开结果 authority；release contract/manifest/staging/released evidence 分层明确。

**Refactor 守卫**

- 不扩大为 Runtime 源码重构；runtime query 专题测试继续通过。

### UT-003 设计状态与 Codex-first 当前事实

**目标行为**

设计 INDEX 只表达 adopted authority，场景和 Agents 页面准确投影已实现的 Codex-first trigger 事实。

**关联**

- Design: `功能设计 / 2. 长期 Wiki authority 收口`
- 系统测试用例: ST-003
- Tasks: 3.1 Red / 3.2 Green / 3.3 Refactor

**Files**

- Test: `scripts/tests/documentation-closure-contract.test.ts`
- Modify: `.wiki/06-设计文档/INDEX.md`、`00-总体设计.md`、`02-Agents设计.md`、`03-核心场景.md`、`05-产品基线与设计治理.md`
- Reference: `scripts/tests/host-trigger-contract.test.ts`、`scripts/tests/core-scenario-wiki-contract.test.ts`

**测试代码蓝图**

```typescript
test("design authorities separate adoption from delivery evidence", () => {
  expect(collectDesignStatusIssues()).toEqual([]);
  expect(collectHostProjectionIssues()).toEqual([]);
});
```

**测试数据 / Fixture / Mock 边界**

- 真实 Wiki files；只检查稳定 headings/markers 和禁止的过时时态。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/documentation-closure-contract.test.ts -t "design authorities"`

**预期 Red 失败**

- 失败测试名: `design authorities separate adoption from delivery evidence`
- 关键错误 / 断言差异: INDEX 仍以未实现筛选；Agents/核心场景仍把已实现 trigger 标为延期；总体设计仍链接旧 roadmap。
- 失败原因: 前置 child 事实尚未完成最终 Wiki 投影。

**Green 通过条件**

- adopted/evidence 正交、Codex-first current facts 与场景分类均对齐。

**Refactor 守卫**

- 不把完整 HostAdapter 或 task-aware scope 误写成已实现。

### UT-004 `.docs` reference inventory 唯一

**目标行为**

`.docs` 只保留 INDEX 与登记的无 authority 外部 reference，已迁移/已完成材料全部退出当前树。

**关联**

- Design: `功能设计 / 3. .docs 迁移与删除矩阵`
- 系统测试用例: ST-004
- Tasks: 4.1 Red / 4.2 Green / 4.3 Refactor

**Files**

- Test: `scripts/tests/documentation-closure-contract.test.ts`
- Modify: `.docs/INDEX.md`、`.docs/research/karpathy-llm-wiki-analysis.md`，并删除 design/roadmap/release/quality 旧文件
- Reference: `.wiki/00-文档约定/04-文档盘点与沉淀规则.md`

**测试代码蓝图**

```typescript
test("docs contains only registered non-authoritative references", () => {
  expect(collectDocsInventoryIssues()).toEqual([]);
});
```

**测试数据 / Fixture / Mock 边界**

- 真实 `.docs` tree/front matter/INDEX；不扫描 archive。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/documentation-closure-contract.test.ts -t "docs contains"`

**预期 Red 失败**

- 失败测试名: `docs contains only registered non-authoritative references`
- 关键错误 / 断言差异: 列出未允许的 design/roadmap/release/quality 文件，reference 缺少 `authority: none`。
- 失败原因: 材料迁移矩阵尚未执行。

**Green 通过条件**

- survivor 集合精确、INDEX 唯一登记、reference 无 active backlog/authority。

**Refactor 守卫**

- 新 survivor 必须显式更新 inventory，不能通过泛化目录允许列表绕过分类。

### UT-005 README、CLI 与 release parity

**目标行为**

README EN/CN 同步投影一级 CLI、archive modes、Codex-first roles 与 release authority。

**关联**

- Design: `功能设计 / 4. 公开入口投影`
- 系统测试用例: ST-005
- Tasks: 5.1 Red / 5.2 Green / 5.3 Refactor

**Files**

- Test: `scripts/tests/documentation-closure-contract.test.ts`
- Modify: `README.md`、`README-CN.md`、`.wiki/04-对外方法/INDEX.md`、`.wiki/INDEX.md`
- Reference: `.wiki/04-对外方法/00-CLI.md`、`scripts/tests/cli-surface.test.ts`

**测试代码蓝图**

```typescript
test("public readmes project the current CLI and host contract", () => {
  for (const readme of readPublicReadmes())
    expect(collectReadmeProjectionIssues(readme)).toEqual([]);
});
```

**测试数据 / Fixture / Mock 边界**

- 真实 README/CLI/release Wiki；不调用外部 host。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/documentation-closure-contract.test.ts -t "public readmes"`

**预期 Red 失败**

- 失败测试名: `public readmes project the current CLI and host contract`
- 关键错误 / 断言差异: 旧 `/wiki:*` identity、缺 archive markers、缺 Codex-first/release authority。
- 失败原因: README 仍投影历史公开面。

**Green 通过条件**

- 两份 README 的结构 marker 与 authority link 一致，旧命令 identity 消失。

**Refactor 守卫**

- 不要求中英文全文相同，只比较稳定 surface 与链接。

### UT-006 Current links 与 active/archive authority 完整

**目标行为**

有限 current roots 的相对链接均有效，具体 active change pointer 不指向 archive，archive 仅作为 history/evidence。

**关联**

- Design: `功能设计 / 5. 一致性测试`
- 系统测试用例: ST-006
- Tasks: 6.1 Red / 6.2 Green / 6.3 Refactor

**Files**

- Test: `scripts/tests/documentation-closure-contract.test.ts`
- Modify: 当前 Wiki、README、`.docs/INDEX.md` 中发现的失效链接
- Reference: `.spec/changes/**/meta.yaml`、`.spec/archive/**`

**测试代码蓝图**

```typescript
test("current authority links resolve without archived active pointers", () => {
  expect(collectCurrentLinkIssues()).toEqual([]);
  expect(collectConcreteActivePointerIssues()).toEqual([]);
});
```

**测试数据 / Fixture / Mock 边界**

- 只扫描 README、`.wiki` current pages 和 `.docs/INDEX.md`；archive 内容不读取。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/documentation-closure-contract.test.ts -t "current authority links"`

**预期 Red 失败**

- 失败测试名: `current authority links resolve without archived active pointers`
- 关键错误 / 断言差异: 报告删除材料的现存链接或 `.docs` 中已归档 active pointer。
- 失败原因: authority/链接收口尚未完成。

**Green 通过条件**

- current links 全部解析，具体 change target 状态匹配，archive 只用于证据链接。

**Refactor 守卫**

- Generic `.spec/changes/**` 规范不误报；archive tree 不进入 scan roots。

## 测试辅助边界

- 允许在 `documentation-closure-contract.test.ts` 内新增 Markdown section、front matter、relative link 和 inventory helpers。
- helper 必须返回 issues 数组以生成可定位失败，不引入生产模块或外部 parser 依赖。
- 不 mock 文件系统、不访问网络、不写 runtime 产物。

## 不纳入单元测试的内容

- UniSpec archive move、parent marker 和 commit 由 `unispec-archive` 与 git 命令验证。
- Registry/tag/checksum 等真实发布证据不在本 change 实现。
- 仓库外部深链无法自动化验证，作为已接受兼容风险记录。
