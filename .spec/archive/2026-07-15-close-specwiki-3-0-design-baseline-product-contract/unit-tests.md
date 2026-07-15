# close-specwiki-3-0-design-baseline-product-contract 单元测试设计

## 测试总览

本文件定义 `scripts/tests/product-baseline-contract.test.ts` 的 TDD 蓝图。每个 UT 聚焦一个文档合同行为，使用固定文件清单和结构化诊断数组，避免全文快照、全库扫描和版本 lockstep。所有测试由根 Vitest 配置执行。

## 单元测试用例

### UT-001 Canonical baseline 唯一可达且职责分离

**目标行为**

固定 canonical 页面存在，且从设计索引和总体设计可达；两个页面的治理职责不重叠。

**关联**

- Design: `Canonical baseline 页面`、`最小导航与延期收口边界`
- 系统测试用例: ST-001
- Tasks: 1.1 Red / 1.2 Green / 1.3 Refactor

**Files**

- Test: `scripts/tests/product-baseline-contract.test.ts`
- Modify: `.wiki/06-设计文档/05-产品基线与设计治理.md`、`.wiki/06-设计文档/INDEX.md`、`.wiki/06-设计文档/00-总体设计.md`
- Reference: `scripts/tests/cli-surface.test.ts`

**测试代码蓝图**

```typescript
test("canonical product baseline is uniquely reachable", () => {
  const missingRequirements = collectCanonicalBaselineRequirements();
  expect(missingRequirements).toEqual([]);
});
```

**测试数据 / Fixture / Mock 边界**

- 只读取三个固定 Markdown 路径，不递归扫描其它页面，不使用 mock。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/product-baseline-contract.test.ts -t "canonical product baseline is uniquely reachable"`

**预期 Red 失败**

- 失败测试名: `canonical product baseline is uniquely reachable`
- 关键错误 / 断言差异: `Expected []`，实际包含 `.wiki/06-设计文档/05-产品基线与设计治理.md: missing canonical baseline`。
- 失败原因: canonical 页面和导航尚未实现；不是导入、语法或环境错误。

**Green 通过条件**

- canonical 页面存在，两个入口均使用相对链接指向它，并明确 `05` 与 `00` 的职责边界。

**Refactor 守卫**

- 固定路径和必要语义检查保持通过，不引入递归扫描或全文快照。

### UT-002 三类版本域独立且显式映射

**目标行为**

版本表覆盖 architecture、product-release、artifact，并以显式 relation 连接而非比较数值推导。

**关联**

- Design: `版本域与显式关系`
- 系统测试用例: ST-002
- Tasks: 2.1 Red / 2.2 Green / 2.3 Refactor

**Files**

- Test: `scripts/tests/product-baseline-contract.test.ts`
- Modify: `.wiki/06-设计文档/05-产品基线与设计治理.md`
- Reference: `packages/spec-wiki/package.json`、`crates/wiki-model/Cargo.toml`、`crates/wiki-index/Cargo.toml`、`crates/wiki-knowledge/Cargo.toml`、`crates/wiki-runtime/Cargo.toml`

**测试代码蓝图**

```typescript
test("version domains keep independent authorities and explicit mappings", () => {
  const baseline = readBaseline();
  expect(collectVersionContractIssues(baseline)).toEqual([]);
});
```

**测试数据 / Fixture / Mock 边界**

- 使用固定 manifest 清单；用 JSON.parse 读取主包版本，用受限 TOML 行匹配读取各 crate package version；不引入 TOML 依赖，不比较版本相等性。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/product-baseline-contract.test.ts -t "version domains keep independent authorities and explicit mappings"`

**预期 Red 失败**

- 失败测试名: `version domains keep independent authorities and explicit mappings`
- 关键错误 / 断言差异: `Expected []`，实际至少包含 `missing authority: architecture`、`missing relation: distributed-by`。
- 失败原因: canonical 页面尚未提供三域 authority 和显式映射。

**Green 通过条件**

- 三域、authority、scope、relation、独立版本规则和所有显式当前 manifest 值均通过检查。

**Refactor 守卫**

- helper 保持有限输入；不得强制任何两个域或 artifact 版本相等。

### UT-003 决策状态与交付证据正交

**目标行为**

合同使用 `decisionStatus` 加三类 delivery evidence 表达状态，并明确各轴不能互相推导。

**关联**

- Design: `设计决策与交付证据`
- 系统测试用例: ST-003
- Tasks: 3.1 Red / 3.2 Green / 3.3 Refactor

**Files**

- Test: `scripts/tests/product-baseline-contract.test.ts`
- Modify: `.wiki/06-设计文档/05-产品基线与设计治理.md`
- Reference: `./design.md`

**测试代码蓝图**

```typescript
test("decision status is orthogonal to delivery evidence", () => {
  expect(collectStatusModelIssues(readBaseline())).toEqual([]);
});
```

**测试数据 / Fixture / Mock 边界**

- 只读取 canonical 页面固定状态模型段，不使用 fixture 或 mock。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/product-baseline-contract.test.ts -t "decision status is orthogonal to delivery evidence"`

**预期 Red 失败**

- 失败测试名: `decision status is orthogonal to delivery evidence`
- 关键错误 / 断言差异: `Expected []`，实际包含 `missing state dimension: decisionStatus` 和交付证据维度缺口。
- 失败原因: canonical 页面尚未实现正交状态合同。

**Green 通过条件**

- 四个维度、各自闭集值、evidence refs 和三个代表性合法组合均存在。

**Refactor 守卫**

- 不把状态模型收缩为单一阶段或完成布尔值，示例继续证明正交性。

### UT-004 单域和全项目完成判定分离

**目标行为**

单域设计完成与全项目设计完成使用不同证据集合，且不从 active change 数量推导。

**关联**

- Design: `完成判定`
- 系统测试用例: ST-004
- Tasks: 3.4 Red / 3.5 Green / 3.6 Refactor

**Files**

- Test: `scripts/tests/product-baseline-contract.test.ts`
- Modify: `.wiki/06-设计文档/05-产品基线与设计治理.md`
- Reference: `../close-specwiki-3-0-design-baseline/split.md`

**测试代码蓝图**

```typescript
test("completion rules separate one contract domain from the whole program", () => {
  expect(collectCompletionRuleIssues(readBaseline())).toEqual([]);
});
```

**测试数据 / Fixture / Mock 边界**

- 只读取 canonical 页面；不依赖 active change 当前数量，避免测试随流程推进失效。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/product-baseline-contract.test.ts -t "completion rules separate one contract domain from the whole program"`

**预期 Red 失败**

- 失败测试名: `completion rules separate one contract domain from the whole program`
- 关键错误 / 断言差异: `Expected []`，实际包含 `missing completion rule: contract domain`、`missing completion rule: six children`。
- 失败原因: 两级完成合同尚未写入 canonical 页面。

**Green 通过条件**

- 单域五项条件、六个 child、实际归档、documentation-closure、blocking conflict 和禁止 active-count 替代均存在。

**Refactor 守卫**

- 单域完成继续与 implementation/release 完成解耦；全项目规则不引用瞬时 active change 数量作为正证据。

### UT-005 材料边界保留历史并延期全库收口

**目标行为**

canonical 页面稳定分类当前设计、capability、release、manifest、`.docs` 和 `.spec/archive/**`，并将全库迁移交给 documentation-closure。

**关联**

- Design: `有界材料分类与迁移矩阵`
- 系统测试用例: ST-005
- Tasks: 4.1 Red / 4.2 Green / 4.3 Refactor

**Files**

- Test: `scripts/tests/product-baseline-contract.test.ts`
- Modify: `.wiki/06-设计文档/05-产品基线与设计治理.md`
- Reference: `.wiki/00-文档约定/04-文档盘点与沉淀规则.md`

**测试代码蓝图**

```typescript
test("material boundaries preserve history and defer repository-wide closure", () => {
  expect(collectMaterialBoundaryIssues(readBaseline())).toEqual([]);
});
```

**测试数据 / Fixture / Mock 边界**

- 只检查 canonical 页面中的固定分类词和责任边界，不扫描被分类目录内容。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/product-baseline-contract.test.ts -t "material boundaries preserve history and defer repository-wide closure"`

**预期 Red 失败**

- 失败测试名: `material boundaries preserve history and defer repository-wide closure`
- 关键错误 / 断言差异: `Expected []`，实际包含 `missing material class: capabilities`、`missing archive rule: immutable history`。
- 失败原因: 稳定分类与延期责任尚未写入 canonical 页面。

**Green 通过条件**

- 六类材料、只读 archive、非 authority `.docs` 和 documentation-closure 责任均通过检查。

**Refactor 守卫**

- 测试保持有界，不递归扫描历史、阶段或生成目录；页面不复制一次性迁移任务。

## 测试辅助边界

- 允许在 `scripts/tests/product-baseline-contract.test.ts` 内新增小型纯函数：固定路径读取、Markdown 必要标记收集、JSON manifest 读取、受限 Cargo package version 读取。
- 不新增生产模块、第三方依赖、全库 Markdown parser 或通用文档 lint 框架。

## 不纳入单元测试的内容

- 全库状态和引用迁移：由 documentation-closure 的系统验收覆盖。
- registry、tag、binary 和 publish evidence：当前 change 只定义证据边界，不生成这些证据。
- Runtime/CLI/Agents 行为：本 change 不修改这些接口。
