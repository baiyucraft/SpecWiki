# close-specwiki-3-0-design-baseline-documentation-closure 系统测试用例

## 用例总览

本用例集验证 3.0 六个 child 的最终文档收口：capability inventory、Runtime query 与 release authority、设计状态和 Codex-first 投影、`.docs` 分类、README/CLI 一致性以及跨文档链接与 archive 边界。全部用例通过 root Vitest 和 UniSpec 校验自动执行，不需要浏览器或人工验收。

## 系统测试用例

### ST-001 Capability Purpose、inventory 与 KnowledgeUnit-first 边界一致

- 关联成功标准: 所有 capability Purpose 非占位；KnowledgeUnit 为正式主线；capability INDEX 与目录一致。
- 覆盖设计点: Capability inventory 收口；删除 `content-family-planner`；family 仅为 signal/projection style。
- 前置条件: 当前 `.wiki/05-规格基线/capabilities/**` 和 INDEX 可读。
- 操作 / 触发: 运行 documentation closure 合同测试，解析全部 capability Purpose、目录集合和 INDEX 链接，并检查 family identity 迁移后的关键 authority。
- 期望结果: Purpose 全部非空且无归档占位；INDEX 与目录一一对应；不存在平行 family capability；保留的 family 语义明确受 KnowledgeUnit/domain/projection scope 约束。
- 验证方式: root Vitest `scripts/tests/documentation-closure-contract.test.ts`，并复跑相关 capability/root contract tests。

### ST-002 Query 与 v0.2.0 release authority 唯一且正交

- 关联成功标准: 当前 query 投影不再暴露旧顶层字段；版本治理回链唯一 authority；release contract 不冒充 release evidence。
- 覆盖设计点: `wiki-bm25-query`、`repo-wiki-runtime` 收口；新增 `.wiki/04-对外方法/02-v0.2.0发布合同.md`。
- 前置条件: canonical Runtime query authority、package manifests 和 capability specs 可读。
- 操作 / 触发: 对当前 query/release authority 运行结构化合同断言，并读取 manifest 显式映射。
- 期望结果: `route_groups` 保持唯一结果 authority；旧 matched/provenance 字段只可出现在明确删除/禁止的说明中；release contract 区分 adopted contract、staging/dry-run 和 released evidence。
- 验证方式: documentation closure、runtime query、product baseline 和 distribution 合同测试。

### ST-003 设计状态、场景与 Codex-first 宿主投影一致

- 关联成功标准: INDEX 不把 adopted 与 implemented 混用；baseline/next/non-goal 一致；当前宿主事实不再标为延期。
- 覆盖设计点: 设计 INDEX、总体设计、Agents、核心/扩展场景和产品基线同步。
- 前置条件: `.wiki/06-设计文档/**` 与 host trigger authority 可读。
- 操作 / 触发: 运行状态/宿主结构断言，核对 adopted authority、独立 evidence 轴、Codex reference role 和 trigger 当前代码事实。
- 期望结果: INDEX 只表达 adopted authority；扩展场景是当前分类 authority；Codex 是唯一 reference host；compatibility role、trigger capability、asset validator 和 CodeBuddy structured parser 被标为当前事实。
- 验证方式: documentation closure、core scenario、host trigger 和 product baseline 合同测试。

### ST-004 `.docs` 只保留登记的无 authority reference

- 关联成功标准: product-contract 材料迁移矩阵逐类处置；旧 roadmap/迁移存根/quality 分析已迁移或删除；`.docs` 不作为当前 authority。
- 覆盖设计点: `.docs` 删除/保留矩阵与 survivor front matter。
- 前置条件: `.docs` 文件树和 INDEX 可读。
- 操作 / 触发: 比较实际 `.docs` Markdown 文件与允许 inventory，解析 survivor front matter 和 INDEX 登记。
- 期望结果: 只存在 `.docs/INDEX.md` 与已登记的外部理念 reference；reference 声明 `authority: none`，不包含 active backlog/authority 指针；被删除路径无当前库内链接。
- 验证方式: documentation closure 合同测试和 `git diff --check`。

### ST-005 README EN/CN 与一级 CLI、Codex-first 和 release contract 对齐

- 关联成功标准: 公开入口只引用合法 authority；旧 roadmap/命令 identity 不再作为当前合同；版本叙事与 release evidence 正交。
- 覆盖设计点: README 公开入口投影与 `.wiki/04` authority。
- 前置条件: README EN/CN、CLI authority 和 release contract 可读。
- 操作 / 触发: 分别检查两份 README 的宿主角色、一级命令、archive modes/exit semantics、release authority link 和旧 `/wiki:*` 禁止项。
- 期望结果: 两份 README 结构一致；Codex-first 明确；archive surface 完整；不存在旧 command identity 或“staging 等于已发布”表述。
- 验证方式: documentation closure、CLI surface、distribution tests 和 package build。

### ST-006 当前 authority 链接有效且 archive 保持只读

- 关联成功标准: 当前材料不指向已归档 active change；archive 只作历史证据；full review/verification 和归档校验通过。
- 覆盖设计点: Link/authority integrity 与 archive 排除边界。
- 前置条件: 当前 Wiki、README、`.docs/INDEX.md`、active changes 和 archive 目录可读。
- 操作 / 触发: 解析有限 current roots 的相对 Markdown 链接；校验具体 `.spec/changes/<id>` target 真实 active；记录 archive tree 基线并执行 full verification。
- 期望结果: 当前链接均存在；具体 active target 不指向已归档 change；archive 链接只出现在 history/evidence 语境；实现阶段未修改历史 archive；UniSpec validate 无阻塞。
- 验证方式: documentation closure 合同测试、`git diff --check`、`pnpm test`、UniSpec full review/verification/archive checks。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| Capability Purpose 全部有效 | ST-001 | Purpose section parser + inventory test |
| 当前材料无失效 active/roadmap 状态 | ST-003、ST-004、ST-006 | 状态 marker、docs inventory、link resolver |
| 设计状态与 baseline/next/non-goal 一致 | ST-003 | design authority contract |
| KnowledgeUnit-first，family/page 仅为受限投影 | ST-001 | capability identity contract |
| 材料迁移矩阵完成且 archive 只读 | ST-004、ST-006 | docs inventory + git/archive evidence |
| Release/manifest/版本治理回链唯一 authority | ST-002、ST-005 | release contract + README parity |
| 自动化门禁阻止再次漂移 | ST-001 至 ST-006 | root Vitest full suite |
| Full review、verification、归档和 parent marker 通过 | ST-006 | UniSpec reports/validate/archive |

## 边界与异常

- `.spec/archive/**` 中的旧字段、旧链接和历史状态不参与 current contract scan，也不修改。
- Generic `.spec/changes/**` 约定不是具体 active pointer，不应被门禁误伤。
- 旧 query 字段可以出现在明确“已删除/禁止”的 canonical 说明中，不能作为正向响应字段出现。
- family 可以作为 deterministic source signal 或 projection style 出现，不能成为 durable identity 或平行树。
- registry/tag/checksum evidence 缺失不是本 change 失败，但 release contract 必须明确缺口且不得宣称 released。

## 验证数据与环境

- 当前仓库 Markdown、package/Cargo manifests、active/archived UniSpec directories。
- Node.js、pnpm、Vitest、Rust toolchain 和现有工作区测试入口。
- 不使用网络、浏览器、mock 外部发布系统或修改 runtime 产物。

## 未覆盖项

- 无法验证仓库外部持有的已删除 `.docs` 深链；项目处于测试开发阶段，明确接受该不兼容变更。
- 不验证 registry、Git tag、binary checksum 或真实 publish；它们属于未来 release evidence 实现。

## 参考资料

- [proposal](./proposal.md)
- [design](./design.md)
- [文档收口现状与迁移边界审计](./research/documentation-closure-audit.md)
