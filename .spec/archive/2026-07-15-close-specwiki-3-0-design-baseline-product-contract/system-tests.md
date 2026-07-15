# close-specwiki-3-0-design-baseline-product-contract 系统测试用例

## 用例总览

本组用例覆盖 canonical product baseline 的唯一入口、版本 authority、显式映射、正交状态证据、两级完成规则和有界材料分类。验证以工作区级合同测试和 UniSpec artifact 校验为主，不涉及 Runtime、CLI 或浏览器交互。

## 系统测试用例

### ST-001 Canonical baseline 唯一且可达

- 关联成功标准: 存在明确、唯一且可引用的 Repo Wiki 3.0 canonical product baseline contract。
- 覆盖设计点: Canonical baseline 页面、最小导航与职责优先级。
- 前置条件: 仓库包含 `.wiki/06-设计文档/00-总体设计.md` 和 `INDEX.md`。
- 操作 / 触发: 运行 product baseline 合同测试，读取固定 canonical 路径和两个导航入口。
- 期望结果: canonical 页面存在；设计索引和总体设计均链接该页面；页面明确 `05` 负责版本、状态和完成定义，`00` 负责架构内容。
- 验证方式: `pnpm exec vitest run --config vitest.config.mjs scripts/tests/product-baseline-contract.test.ts -t "canonical product baseline is uniquely reachable"`。

### ST-002 三类版本域具有独立 authority 和显式关系

- 关联成功标准: 版本模型覆盖 architecture、product / CLI release、package / crate artifact，并定义 authority、scope 和映射。
- 覆盖设计点: VersionDomain、VersionRelation、manifest authority 与独立版本演进。
- 前置条件: 主包和四个 Rust crate manifest 存在。
- 操作 / 触发: 运行合同测试并对照固定 manifest 清单读取当前版本。
- 期望结果: 三类版本域均有 authority 和 scope；产品与主包关系必须显式声明；页面当前版本值与 manifest 一致；不同 artifact 不被要求版本相等。
- 验证方式: `pnpm exec vitest run --config vitest.config.mjs scripts/tests/product-baseline-contract.test.ts -t "version domains keep independent authorities and explicit mappings"`。

### ST-003 设计决策与交付证据保持正交

- 关联成功标准: 合同能表达 adopted 但未实现、已实现但未验证、验证通过但未发布。
- 覆盖设计点: `decisionStatus`、implementation / verification / release evidence 和证据引用要求。
- 前置条件: canonical 页面存在。
- 操作 / 触发: 运行合同测试读取状态模型和示例。
- 期望结果: 四个维度独立出现；页面明确 adopted 不推导实现、验证或发布；三个代表性组合可表达。
- 验证方式: `pnpm exec vitest run --config vitest.config.mjs scripts/tests/product-baseline-contract.test.ts -t "decision status is orthogonal to delivery evidence"`。

### ST-004 单域与全项目完成规则不混淆

- 关联成功标准: 单域完成具备 authority、范围、非目标、依赖和可验证条件；全项目完成要求六个 child 和最终一致性门禁。
- 覆盖设计点: 单合同域完成与 Repo Wiki 3.0 全项目设计完成。
- 前置条件: canonical 页面存在。
- 操作 / 触发: 运行合同测试读取两级完成规则。
- 期望结果: 单域完成不要求产品实现或发布；全项目完成明确要求六个 child 实际完成、documentation-closure 门禁和无 blocking conflict；不存在用 active changes 为空替代的表述。
- 验证方式: `pnpm exec vitest run --config vitest.config.mjs scripts/tests/product-baseline-contract.test.ts -t "completion rules separate one contract domain from the whole program"`。

### ST-005 材料分类有界且历史 archive 只读

- 关联成功标准: 迁移清单覆盖当前设计、capability、release contract、manifests、`.docs` 和只读 `.spec/archive/**`；本 change 不依赖全库迁移。
- 覆盖设计点: 稳定材料分类、后续 child 输入和 documentation-closure 边界。
- 前置条件: canonical 页面存在。
- 操作 / 触发: 运行合同测试读取固定分类段，不递归扫描全库。
- 期望结果: 所有要求类别都有明确角色；archive 只作为不可改历史证据；全库状态、INDEX 和旧链接收口明确延期到 documentation-closure。
- 验证方式: `pnpm exec vitest run --config vitest.config.mjs scripts/tests/product-baseline-contract.test.ts -t "material boundaries preserve history and defer repository-wide closure"`。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| 唯一、可引用的 canonical baseline | ST-001 | 固定路径、双入口和职责断言 |
| 三类版本域 authority、scope 和映射 | ST-002 | 固定表格语义和 manifest 对照 |
| 数值差异不自动构成漂移 | ST-002 | 独立版本规则断言 |
| 决策与交付证据正交 | ST-003 | 四维状态与合法组合断言 |
| 单域 / 全项目两级完成规则 | ST-004 | 完成条件和禁止替代断言 |
| 有界材料分类与 archive 边界 | ST-005 | 固定类别和延期责任断言 |
| 验收不依赖全库迁移 | ST-005 | documentation-closure 边界断言 |

## 边界与异常

- canonical 页面缺失、导航不可达或职责重复时必须失败。
- manifest 路径缺失或页面显式声明的当前版本与 manifest 不一致时必须失败。
- 测试不得要求 npm 与 Rust crate、不同 Rust crate 或 architecture / release 版本数值相等。
- 测试不得递归扫描 `.spec/archive/**`、`.docs/release/**`、`dist`、`target` 或其它历史 / 生成目录来判断当前合同漂移。
- 本用例集不验证 Runtime/CLI/Agents 行为，也不替代最后一个 child 的全库一致性门禁。

## 验证数据与环境

- Node.js、pnpm 和根级 Vitest 配置可用。
- 固定输入：canonical 页面、设计 `INDEX.md`、总体设计、`packages/spec-wiki/package.json`、四个 `crates/*/Cargo.toml`。
- 全部验证在仓库根目录执行，无网络、登录态或浏览器依赖。

## 未覆盖项

- registry、Git tag、binary checksum 与 staged package 的真实发布证据落点尚未定义，由后续 release / documentation-closure change 处理。
- 全库旧版本叙事、状态标签和 authority 指针迁移由 `close-specwiki-3-0-design-baseline-documentation-closure` 独立验收。

## 参考资料

- `./proposal.md`
- `./design.md`
- `.wiki/02-开发指南/01-测试与验收.md`
- `.wiki/02-开发指南/02-脚本与工作流.md`
