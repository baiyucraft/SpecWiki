# 文档收口现状与迁移边界审计

## 调研目的

- 阶段：design
- 关联 change：close-specwiki-3-0-design-baseline-documentation-closure
- 服务边界：design
- 要回答的问题：前五个 child 归档后，哪些当前 Wiki、capability、公开入口和 `.docs` 材料仍与 canonical contract 冲突，应如何分类迁移并建立自动化门禁。
- 停止条件：能够形成逐类迁移矩阵、唯一 authority 落点、非脆弱测试边界和风险处置方案。

## 结论摘要

- 父 split 最初识别 9 个 Purpose 占位，前置 child 已处理其中 5 个；当前精确剩余 4 个，不能继续用旧数量作为实现断言。
- 问题不止是 Purpose 占位。`wiki-bm25-query`、`repo-wiki-runtime` 和 `.docs/release/v0-2-0.md` 仍公开旧 `matched_* / matched_pages / provenance_summary` 合同，与 `route_groups` authority 直接冲突。
- `.docs/design/**` 的 5 个文件已是纯迁移存根；两份 roadmap 与两份 release gap 材料继续冒充当前队列；稳定结论已经有 Wiki 或 archive 落点，应删除这些重复入口。
- v0.2.0 仍需要一个长期 versioned release contract，但必须与 registry、tag、checksum、staged publish 等 release evidence 正交；落点应进入 `.wiki/04-对外方法/`。
- family 可以保留为 deterministic signal 或 projection style，不能继续作为与 KnowledgeUnit 平行的 capability 主抽象；`content-family-planner` 应合并进 `knowledge-unit-decomposition`。
- 新门禁应解析明确结构和有限 authority roots，不做全库关键词快照，也不扫描只读 archive 制造误报。

## 已读取资料

| 路径 / 来源 | 目的 | 关键发现 |
| --- | --- | --- |
| `proposal.md` | 固定目标与非目标 | 只做文档投影、authority 和一致性门禁，不改行为合同 |
| parent `split.md` 与 research | 核对验收边界 | closure 必须执行材料迁移矩阵并处理 capability、roadmap、状态和引用 |
| product-contract 归档 `design.md` | 读取有界迁移矩阵 | 当前设计、capability、公开 surface、release、`.docs`、manifests 与 archive 均有明确角色 |
| `.wiki/06-设计文档/05-产品基线与设计治理.md` | 核对版本与状态 authority | architecture、product-release、artifact 与 evidence 必须正交 |
| `.wiki/06-设计文档/06-Runtime查询合同.md` | 核对 query authority | `route_groups` 是唯一结果 authority；旧顶层字段已删除 |
| `.wiki/05-规格基线/capabilities/**` | 核对 Purpose 和正文 | 4 个 Purpose 占位；多个旧 query、family 和 release evidence 表述仍冲突 |
| `.wiki/06-设计文档/**` | 核对状态与交叉引用 | INDEX 把 adopted 与 implemented 混用；部分场景和 Agents 实现事实已过时 |
| `README.md`、`README-CN.md` | 核对公开入口 | 旧 Claude `/wiki:*` identity、archive surface、Codex-first 和 release evidence 口径漂移 |
| `.docs/**` | 形成逐文件迁移矩阵 | 多数文件已迁移、已完成或仅为历史分析，不应继续并列当前 authority |
| `scripts/tests/*contract*.test.ts`、`scripts/run-tests.mjs` | 选择测试模式 | root Vitest 已自动纳入 `scripts/tests`，适合新增聚焦合同测试 |

## 子代理调研

| 子代理 / 角色 | 调研主题 | 结论 | 主会话复核 |
| --- | --- | --- | --- |
| capability/Wiki auditor | Purpose、query、family、release 与设计状态 | 识别 4 个占位、两处 query 硬冲突、family 平行抽象和 release evidence 混用 | accepted；以 canonical query 和产品基线逐项复核 |
| docs/test auditor | `.docs`、README、公开 surface 和测试结构 | 建议删除迁移存根/旧 roadmap/gap，建立 Wiki release authority，并采用结构化有限扫描 | accepted；保留外部理念 research 作为无 authority reference |

## 关键发现

### Query capability 与 canonical transport 冲突

- 证据：`.wiki/05-规格基线/capabilities/wiki-bm25-query/spec.md`、`repo-wiki-runtime/spec.md`、`.docs/release/v0-2-0.md`、`.wiki/06-设计文档/06-Runtime查询合同.md`。
- 说明：当前 capability 和 release 文档仍暴露旧顶层 matched 字段或顺序 fallback 心智，canonical contract 已改为 formal layer fusion、route-local ranking 与 `route_groups` 唯一 authority。
- 影响：closure 不能只补 Purpose，必须同步当前 capability 正文和 release projection。

### Capability inventory 需要删除平行抽象

- 证据：`content-family-planner/spec.md`、`knowledge-unit-decomposition/spec.md`、`page-evidence-layer/spec.md`、`page-research-dossier/spec.md`、`research-driven-page-composition/spec.md`。
- 说明：family 已被前置合同降为 decomposition signal 或 page projection style，但 capability 名称和多处对象名仍把它表达成平行页面体系。
- 影响：删除 `content-family-planner` capability，将仍有效的 deterministic discovery、dedup 和 projection 规则合并到 KnowledgeUnit decomposition；其它 capability 改用 domain/unit/section/projection scope。

### `.docs` 已从阶段输入变成重复 authority 风险

- 证据：`.docs/INDEX.md`、`.docs/design/**`、`.docs/roadmap/**`、`.docs/release/**`、`.docs/quality/**`。
- 说明：5 个 design 文件只是 migrated stub；roadmap 仍把已完成迭代标为计划中；release gap 仍描述已完成修复；quality 分析的稳定规则已有 Wiki/测试落点。
- 影响：删除重复材料，仅保留外部理念 research，并让 `.docs/INDEX.md` 成为明确的 reference inventory。

### 公开入口与当前 CLI/宿主合同漂移

- 证据：`README.md`、`README-CN.md`、`.wiki/04-对外方法/00-CLI.md`、`.wiki/06-设计文档/02-Agents设计.md`。
- 说明：README 遗留 Claude `/wiki:*` identity、缺 archive 模式与退出语义、未表达 Codex-first，并把 staging target 近似写成真实发布。
- 影响：README 只投影一级 CLI、repo-local skills 和 Codex-first host roles；release 状态回链新的 versioned release contract。

### adopted authority 与 delivery evidence 被 INDEX 再次混用

- 证据：`.wiki/06-设计文档/INDEX.md`、`02-Agents设计.md`、`04-扩展场景.md`、`05-产品基线与设计治理.md`。
- 说明：INDEX 以“未实现”排除设计材料，但产品基线明确 decision、implementation、verification、release 四轴正交；adopted next/non-goal 仍可以是稳定 authority。
- 影响：INDEX 状态只表达 authority/decision 角色，不推导实现或发布；具体 evidence 由专题页和归档报告提供。

## 方案与取舍

| 选项 | 优点 | 缺点 | 结论 |
| --- | --- | --- | --- |
| 只补 4 个 Purpose | 改动小 | query、family、release 和 roadmap 冲突继续存在 | 不采用 |
| 所有 `.docs` 原位加免责声明 | 保留深链 | 重复入口和过时正文仍可被误读为当前事实 | 不采用 |
| 删除已迁移/已完成材料，仅保留登记过的 reference | authority 最清晰，符合项目文档边界 | 外部深链可能失效 | 采用；库内链接由测试覆盖 |
| 保留 `content-family-planner` 并改 Purpose | 避免 capability 删除 | 继续保留平行抽象名称 | 不采用；测试开发阶段不保留兼容层 |
| 新建 Wiki versioned release contract | product-release 有唯一长期入口，能与 evidence 分层 | 需要同步 README 与索引 | 采用 |
| 全库关键词禁止列表 | 实现简单 | 历史引用、否定表述和 generic active path 会误报 | 不采用；使用结构解析和有限 roots |

## 风险与未知项

| 项目 | 影响 | 处理方式 |
| --- | --- | --- |
| 删除 `.docs` 破坏外部深链 | 外部引用不可自动发现 | 接受测试开发阶段不兼容；保证库内链接无断链 |
| family 全局替换误伤合法投影语义 | 丢失有用模型 | 只迁移身份层；允许 signal/projection style 的限定用法 |
| staging 被误报为 released | 违反正交 evidence | release contract 明确 adopted contract 与 evidence checklist |
| capability index 与目录漂移 | 新增/删除 capability 后入口错误 | 测试比较目录 inventory 与 INDEX 链接集合 |
| 历史关键词产生假阳性 | 门禁脆弱 | 只扫描当前 authority 与登记 survivor，archive 永不扫描 |

## 对当前 artifact 的影响

- 应写入：`design.md`
- 影响内容：
  - 明确 `.docs` 删除/保留矩阵。
  - 建立 v0.2.0 release contract 长期落点。
  - 定义 capability 合并、Purpose、query/family/release 迁移范围。
  - 定义 README、设计索引、场景和产品基线同步项。
  - 定义结构化合同测试与 archive 只读边界。
- 后续阶段处理：
  - `unispec-plan` 将设计点映射为 system tests、unit tests 和 TDD tasks。

## 未采纳内容

- 不修改 `.spec/archive/**` 中的旧字段、状态或链接；它们是只读历史证据。
- 不实现 registry/tag/checksum 采集；本 change 只定义 release evidence 边界。
- 不重构 Runtime query、CLI parser 或 HostAdapter；当前行为合同由前置 child 拥有。
