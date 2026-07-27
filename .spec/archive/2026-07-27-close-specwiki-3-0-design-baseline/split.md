# close-specwiki-3-0-design-baseline 拆分方案

## 问题概述

当前 contract-closure program 已归档，但项目仍不能证明“3.0 设计完成”。稳定设计索引与场景草案状态矛盾，v0.1.0、v0.2.0、v0.3.0 和 3.0 架构口径并存，Runtime 与 Agents 对 query 稳定字段的描述不一致，核心场景缺少可执行验收映射，扩展场景没有 baseline / next / non-goal 分类，部分 capability 和 roadmap 仍保留过时或互相冲突的合同。

这些问题跨越产品基线、Runtime、Agents、场景验收、可靠性、规格基线和文档治理，无法通过单一文档清理 change 独立验收。

## 整体目标

- 建立唯一的 Repo Wiki 3.0 架构设计基线，并明确其与 semver 发布线的映射规则。
- 为“设计完成”定义可验证标准，而不是以 active change 数量或文档存在性代替完成证据。
- 统一 Runtime、CLI、Agents、场景和 capability 对公开合同的描述。
- 将当前承诺、下一阶段和明确非目标分开，停止使用无归类的“后续应支持”。
- 让核心场景、质量门禁和跨宿主触发规则具备可追溯验收入口。
- 在最终收口时清除失效 roadmap、错误 authority 指针和 capability 文档债务。

## 非目标

- 本 parent 不直接实现代码，也不替代 child 的 proposal、design、plan 或 review。
- 不把所有未来设想一次性纳入 3.0 产品承诺。
- 不重写历史 archive 来伪造新的设计结论。
- 不把 reference fidelity 百分比或单一样本表现当作全部产品设计完成标准。
- 不在 Runtime 正式合同稳定前让宿主自行扩展 query、ranking、readiness 或状态机语义。

## 交付形态

multi-change

问题包含六个可独立验收的合同域，并存在明确依赖：先建立产品基线，再统一 Runtime / Query；核心场景、可靠性和宿主触发消费稳定合同；最后进行 capability、roadmap 和设计文档收口。

## 子 change

### 1. close-specwiki-3-0-design-baseline-product-contract

- 目标：发布唯一、可引用的 Repo Wiki 3.0 canonical baseline contract，定义产品目标、非目标、设计完成规则、版本域 authority、正交状态语义和设计治理规则。
- 验收边界：明确区分 architecture baseline、product / CLI release 和 package / crate artifact version，禁止把不同版本域误解为数值必须一致；定义设计决策状态与 implementation / verification / release evidence 的正交关系；定义单个合同域完成和全项目设计完成的不同判定条件；产出供后续 children 使用的规范输入和现存材料分类 / 迁移清单，但不承担全库引用迁移、INDEX 状态修正或旧 authority 清理。
- 依赖：无。
- 归档状态：[x] archived
- 后续阶段：使用 `unispec-propose` 创建该 child 的 `proposal.md`

### 2. close-specwiki-3-0-design-baseline-runtime-query-contract

- 目标：统一 query input / output、route groups、ranking、provenance、readiness、trust、错误和降级语义，并明确 intent-aware query、owner / entrypoint / impact 的当前或延期边界。
- 验收边界：Runtime、CLI 与 Agents 对稳定字段逐项一致；term-only 与 richer query 的关系明确；每类结果都有排序、截断、来源和质量解释；宿主不再依赖过时的 matched-pages-only 合同。
- 依赖：close-specwiki-3-0-design-baseline-product-contract。
- 归档状态：[x] archived
- 后续阶段：使用 `unispec-propose` 创建该 child 的 `proposal.md`

### 3. close-specwiki-3-0-design-baseline-core-scenario-acceptance

- 目标：把 9 个核心场景转成可验收设计合同，并统一 formal quality gate、primary gate、baseline guard 和 diagnostic 的决策语义。
- 验收边界：9/9 场景均绑定 actor、trigger、command/API、formal artifacts、state/readiness、failure/degraded、recovery 和 verification fixture；workflow-verification 中互相冲突的样本门禁被消解；同一失败只映射到一个明确 gate decision。
- 依赖：close-specwiki-3-0-design-baseline-product-contract、close-specwiki-3-0-design-baseline-runtime-query-contract。
- 归档状态：[x] archived
- 后续阶段：使用 `unispec-propose` 创建该 child 的 `proposal.md`

### 4. close-specwiki-3-0-design-baseline-reliability-lifecycle

- 目标：完成 stale、降级、declared knowledge 生命周期、page projection 治理和大仓 compose 可恢复性边界，并对 10 个扩展场景逐项分类。
- 验收边界：A1 / A6 / A8 / A9 具有状态、authority、恢复、降级和非目标合同；A2 / A3 / A5 / A7 / A10 明确归类为 baseline / next / non-goal，并记录依赖与升级条件；knowledge-unit decomposition 和 provider session 的实际边界不再被历史命名高估。
- 依赖：close-specwiki-3-0-design-baseline-product-contract、close-specwiki-3-0-design-baseline-runtime-query-contract。
- 归档状态：[x] archived
- 后续阶段：使用 `unispec-propose` 创建该 child 的 `proposal.md`

### 5. close-specwiki-3-0-design-baseline-host-trigger-contract

- 目标：建立 Codex、Claude、CodeBuddy 共用的 trigger taxonomy、HostAdapter capability matrix、query 消费模板和触发测试语料。
- 验收边界：存在单一 should-trigger / should-not-trigger / ambiguous 规范源；三个宿主对同一语料的决策可比较；宿主只薄消费正式 Runtime DTO；provider research session 与未来 host-agent bridge 明确分层。
- 依赖：close-specwiki-3-0-design-baseline-runtime-query-contract。
- 归档状态：[x] archived
- 后续阶段：使用 `unispec-propose` 创建该 child 的 `proposal.md`

### 6. close-specwiki-3-0-design-baseline-documentation-closure

- 目标：在前五个合同稳定后收口 capability Purpose、规格命名、roadmap、阶段材料、设计索引和跨文档一致性门禁。
- 验收边界：执行 product-contract 产出的分类 / 迁移清单；9 个 capability Purpose 不再是归档占位；历史 page/family 术语与当前 KnowledgeUnit-first 合同一致；设计 INDEX 不再把草案标成稳定设计；当前材料只引用合法 authority，不再指向已归档 active change；旧 roadmap 不再把已完成能力标为计划中；阶段存根和质量分析已迁移或删除；一致性检查可阻止版本和状态再次漂移。
- 依赖：前五个 child 全部完成。
- 归档状态：[x] archived
- 后续阶段：使用 `unispec-propose` 创建该 child 的 `proposal.md`

## 顺序与依赖

- 先执行 product-contract，建立唯一 baseline 和完成定义。
- runtime-query-contract 在 product-contract 后执行，并作为 core-scenario-acceptance 与 host-trigger-contract 的正式输入。
- core-scenario-acceptance、reliability-lifecycle 和 host-trigger-contract 在依赖满足后可并行推进。
- documentation-closure 最后执行，只同步已经确认和验收的合同，不提前掩盖设计缺口。

## 风险与未知项

- 3.0 是架构设计基线，不自动等同于某个 semver release；具体映射由 product-contract 明确。
- npm package、Rust crates 与产品 release 属于不同版本域，版本数值不同本身不构成漂移；只有 authority 或映射关系冲突才构成设计问题。
- product-contract 只定义 canonical 规则和迁移清单，全库材料迁移由 documentation-closure 执行。
- `workflow-verification` 的历史门禁互相冲突，不能只做措辞整理，必须重新定义唯一判定模型。
- 9 个 Purpose 占位不全是纯文档债务；部分 capability 需要改名、重划当前与延期边界或补完成性证据。
- intent-aware query 与 host trigger 都可能扩大公开合同，必须按依赖顺序处理。
- 大仓 compose 和 knowledge-unit decomposition 可能暴露实现缺口；设计 change 应记录后续实现 change，而不是把未验证能力写成已完成。
- 扩展场景若全部进入 baseline 会造成范围失控，必须允许明确 non-goal。

## 后续执行

- 首先使用 `unispec-propose` 为 `close-specwiki-3-0-design-baseline-product-contract` 创建 proposal。
- 其余 child 在依赖满足后逐个进入 proposal，不在 parent 中预写设计结论。

## 参考资料

- [设计债务与交付边界调研](./research/design-debt-and-delivery-boundary.md)
