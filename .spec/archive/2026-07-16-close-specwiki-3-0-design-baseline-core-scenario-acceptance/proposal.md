# close-specwiki-3-0-design-baseline-core-scenario-acceptance

## 问题

### 背景

Repo Wiki 3.0 已经有一页 9 个核心用户故事，也已经建立产品基线和 Runtime Query canonical 合同。但核心场景页仍标记为草案，场景之间的输入、输出和成功标准使用自然语言描述，未形成统一的可执行验收入口。

同时，`workflow-verification`、reference fidelity、项目集和 lifecycle 脚本分别积累了不同版本的质量门禁语义。formal gate、primary gate、baseline guard 和 diagnostic 的责任、失败归因和退出码没有唯一规则，导致同一个失败可能被重复计为多个 blocker，或在 diagnostic/跳过主流程后仍被报告为 pass。

### 真实问题

当前无法用可追溯证据回答以下问题：9 个核心场景是否逐项具备可验收的行为合同；某个场景失败应由哪个 gate 负责；未覆盖或降级是否应阻断当前验收；以及当前公开 query、声明型知识和协作恢复能力的边界在哪里。继续以场景草案或脚本汇总结果代替这些证据，会把未实现的 richer query 或未证明的冲突检测误报为 3.0 设计完成。

## 目标

- 建立 9 个核心场景的 canonical acceptance matrix，逐项明确 actor、trigger、公开 command/API、formal artifacts、state/readiness、success、failure/degraded、recovery、verification fixture 和 evidence refs。
- 对每个场景明确当前支持、显式降级和延期能力，严格服从产品基线与 Runtime Query canonical 合同。
- 建立唯一的 gate decision 语义：区分 formal quality gate、primary gate、baseline guard 和 diagnostic，保证同一失败只有一个 owning decision。
- 让 diagnostic、未覆盖和跳过主流程的状态在最终聚合及进程退出语义中可观察、不可伪装为 pass。
- 通过自动化 contract tests、负例、CLI 退出码测试和场景 fixture 证明 9/9 覆盖与 gate 归因规则。
- 为 `init`、`status`、`query`、`update`、`sync`、`rebuild` 现有公开入口建立可追溯的场景验收映射，并保持宿主只消费 Runtime canonical DTO。

## 非目标

- 不新增 intent/focus/scope/traversal、独立 entrypoint、owner 或完整 callers/callees/impact API；场景 3/4 只验收现有 symbol/path/module/graph route 及其明确限制。
- 不实现三个宿主的 trigger 语料 parity、HostAdapter capability matrix 或 provider/host bridge；这些属于 `close-specwiki-3-0-design-baseline-host-trigger-contract`。
- 不完成 stale、降级、declared knowledge 生命周期和大仓 compose 的完整设计；这些属于 `close-specwiki-3-0-design-baseline-reliability-lifecycle`。
- 不把任意自然语言的规范与代码语义冲突检测、专用 declared authoring command 或新 CLI 合同写成当前能力；只验收已有结构化 declared/governance 边界，新增入口留待后续设计或 change。
- 不把 reference fidelity 百分比、完整项目集 baseline guard 或单一样本结果替代 9 个场景的正式行为证据。
- 不执行全库 capability Purpose、roadmap、设计索引和旧 authority 的迁移；这些属于最终 `documentation-closure`。
- 不修改历史 `.spec/archive/**`，不采用或迁移 upstream 实现。

## 成功标准

- 9/9 核心场景均有一条 canonical acceptance matrix 记录，且每条记录包含 actor、trigger、公开入口、正式产物、state/readiness、失败或降级、恢复和 verification fixture/evidence。
- 每个场景都明确 `supported`、`degraded` 或 `deferred` 边界；场景 3/4 不承诺 Runtime Query 合同中已延期的 richer query。
- gate 合同固定 level、scope、decision、blocking、evidence refs、required companion gates 和组合规则；`formal quality gate`、`primary gate`、`baseline guard`、`diagnostic` 不再互相冒充。
- 自动化负例证明同一 failure identity 只产生一个 owning formal/primary blocker；无关 gates 不被总失败数连带污染。
- 当主流程被 diagnostic 或跳过条件短路时，相关 gate 只能是 `not_covered` 或整体 `incomplete`，不能伪造 `pass`；decision 与 CLI 退出码一致。
- primary fixture、baseline guard 和 required companion gates 由当前场景/验收计划显式声明，不再由脚本全局硬编码 `storybook + dagger` 或其他历史样本。
- 场景验收只使用 Runtime canonical `route_groups`、逐结果 provenance、readiness、query trust 和 recommended action，拒绝旧的公开 `provenance_summary` 等字段。
- 场景 6/7/8 的自动化证据覆盖已存在的 pitfall/policy/convention 和结构化 governance conflict 边界；未证明的自然语言语义检测和新 authoring API 明确标为 deferred/unknown。
- 场景 9 至少有 A/B 双工作副本恢复 fixture：B 只复制正式 `.wiki/.knowledge`、metadata 和页面产物，不复制 cache；恢复、漂移拒绝、状态和后续 action 均可观察。具体最小产物集合在 design 阶段确定。
- proposal、design、plan、实现、full review 和 verification 产物能够引用同一矩阵及 gate evidence，不以文档存在性替代行为证据。

## 影响范围

### 涉及角色 / 利益相关者

- 使用 `init/status/query/update/sync/rebuild` 的仓库用户。
- 依赖项目规则、定位结果、影响上下文和恢复状态的 Agent 宿主。
- 维护 Rust runtime、TypeScript CLI、测试编排和质量报告的开发者。
- 负责 3.0 设计基线、后续 reliability、host trigger 和 documentation closure 的维护者。

### 业务场景

- 首次 `init` 后理解项目概览。
- Agent 开工前读取规则、找到改动入口并分析影响。
- 代码变更后的 `update`、bug 避坑记录、规范声明和结构化冲突处理。
- 协作用户只取得已提交正式 Wiki 产物后恢复本地 runtime。

### 功能范围

#### 包含内容

- 核心场景 acceptance matrix 和 gate ownership/decision 规则。
- 现有 Runtime、CLI、Agents、脚本和测试入口的验收映射与 fail-closed 证据。
- pitfall/declared/governance 的当前可执行边界，以及 A/B restore 验收 fixture。

#### 不包含内容

- 新的 query 维度、宿主触发体系、完整可靠性生命周期和全库文档迁移。

### 数据需求

- 验收记录需要引用正式 `.wiki/.knowledge`、metadata、页面、runtime state/readiness、query canonical payload、gate summary、failure identity 和 recovery evidence。
- 本 proposal 不规定数据库表、传输字段扩展或 fixture 的具体存储结构；这些 deferred 到 design/plan。

### 非功能期望

- 验收决策必须确定性、可重放、fail closed，并能从 evidence refs 追溯到唯一 owning gate。
- diagnostic、degraded 和未覆盖状态不得被静默压成成功。
- 现有历史 archive 保持只读，公开 query authority 保持单一。

## 交付形态

single-change

这是 `close-specwiki-3-0-design-baseline` 下的第 3 个 child change，依赖已归档的 product-contract 和 runtime-query-contract；它只负责核心场景验收合同，不替代其他 child 的 proposal、design 或实现。

## 风险

- 历史 `workflow-verification` requirement 分散且存在多个 `MUST`，若只修改最后一段文字可能继续保留冲突。
- 取消全局固定 primary 样本语义可能影响已有 reference snapshot、release evidence 和脚本测试，需要在 design/plan 阶段逐项迁移当前 authority，不改历史 archive。
- 场景 7 的自然语言“规范与代码冲突”尚无确定性实现；错误承诺会把诊断能力伪装成已完成产品能力。
- 场景 9 与 reliability child 共享 restore/stale 语境；本 child 必须保持验收映射边界，避免重复定义生命周期策略。
- gate 语义变化会影响多层脚本和退出码，需以失败身份和覆盖状态做兼容迁移，而非只改报告文案。

## 未知项

- acceptance matrix 和 gate result 的最终字段、版本化方式及聚合入口留待 `design.md` 确定。
- A/B restore fixture 所需的最小正式产物集合、manifest/page digest 关系和漂移拒绝顺序留待 design/plan 验证。
- 场景 6/8 是否需要专用 declared authoring command，当前不作决定；若需要，应另立实现范围或由后续 change 承担。
- primary fixture 的具体样本组合由当前验收计划决定，不在 proposal 阶段把历史 `storybook + dagger` 固化为产品规则。

## 参考资料

- [核心场景验收与质量门禁调研](./research/core-scenario-acceptance-audit.md)
- [parent 拆分方案](../close-specwiki-3-0-design-baseline/split.md)
- [产品基线与设计治理](../../archive/2026-07-15-close-specwiki-3-0-design-baseline-product-contract/design.md)
- [Runtime Query 合同设计](../../archive/2026-07-15-close-specwiki-3-0-design-baseline-runtime-query-contract/design.md)
- `.wiki/06-设计文档/03-核心场景.md`
- `.wiki/05-规格基线/capabilities/workflow-verification/spec.md`
