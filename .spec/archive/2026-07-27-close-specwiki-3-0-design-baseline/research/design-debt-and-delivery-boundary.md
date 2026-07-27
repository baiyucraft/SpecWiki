# 设计债务与交付边界调研

## 调研目的

- 阶段：explore
- 关联 change：close-specwiki-3-0-design-baseline
- 服务边界：split
- 要回答的问题：当前项目为什么不能声明设计全部完成，应按哪些独立合同域拆分重新设计，以及各 child 的顺序和验收边界是什么。
- 停止条件：能够明确 problem、goals、non-goals、deliveryShape、child 边界、依赖、风险和 deferred-to-proposal 内容。

## 结论摘要

- 当前问题不是缺一份总设计，而是缺少唯一产品基线、跨文档一致合同和可验证完成定义。
- 设计索引、场景状态、版本口径、Runtime / Agents query DTO 和 roadmap 之间存在可证实漂移。
- 9 个 capability Purpose 占位只是表面信号；质量门禁冲突、历史 page-first 术语、provider session 命名和未证明的 decomposition 完成性更需要重新设计边界。
- 采用一个 parent + 六个 child 的 multi-change，先产品基线和 Runtime / Query，再场景、可靠性、宿主，最后文档收口。

## 已读取资料

| 路径 / 来源 | 目的 | 关键发现 |
| --- | --- | --- |
| `.wiki/06-设计文档/INDEX.md` | 核对稳定设计声明 | INDEX 声明只保存稳定设计，但核心与扩展场景正文仍标为草案。 |
| `.wiki/06-设计文档/00-总体设计.md` | 核对产品定位和完成定义 | 已有四包和 KnowledgeUnit-first 原则，但缺少 release scope、非目标和跨文档完成门槛。 |
| `.wiki/06-设计文档/01-Runtime设计.md` | 核对 Runtime / Query 当前与延期合同 | 同时存在 v0.2.0 minimal runtime 与 v0.1.0 延期口径；richer query 尚未形成稳定 schema。 |
| `.wiki/06-设计文档/02-Agents设计.md` | 核对宿主消费和触发边界 | 宿主稳定字段与 Runtime 当前 DTO 有漂移；统一 trigger、消费模板和语料体系被延期。 |
| `.wiki/06-设计文档/03-核心场景.md` | 核对核心闭环可验收性 | 9 个场景有叙述和成功标准，但未绑定正式产物、状态、错误恢复和 verification fixture。 |
| `.wiki/06-设计文档/04-扩展场景.md` | 核对未来范围 | 10 个主题混合可靠性、治理、产品功能和运维场景，没有优先级或 non-goal 分类。 |
| `.wiki/05-规格基线/capabilities/**/spec.md` | 核对 capability 设计债务 | 9 个 Purpose 为归档占位；部分规格仍使用历史 page/family 术语或高估当前 agent bridge 边界。 |
| `.wiki/05-规格基线/capabilities/workflow-verification/spec.md` | 核对测试与验收合同 | 多轮 requirement 对 storybook、dagger 和 19 项目门禁角色存在互相冲突的正式要求。 |
| `.docs/roadmap/implementation-roadmap.md` | 核对实施路线 freshness | 已归档并落地的迭代仍被标为计划中。 |
| `.docs/roadmap/knowledge-system-completeness-roadmap.md` | 核对 program authority | 仍指向已经归档的 active parent 路径。 |
| `.docs/quality/knowledge-quality-gates-acceptance.md` | 核对质量 gate 沉淀 | formal gate 分析尚未完整进入稳定测试与验收 SSOT。 |
| `.spec/archive/2026-07-13-refactor-specwiki-around-contract-closure/**` | 核对既有 program 边界 | parent 与八个 child 已归档，证明合同收口 program 完成，但不等于整个 3.0 产品设计完成。 |
| 用户确认 | 确认交付方向 | 用户确认采用 multi-change 重新设计并创建 exploration artifacts。 |

## 子代理调研

| 子代理 / 角色 | 调研主题 | 结论 | 主会话复核 |
| --- | --- | --- | --- |
| design-scope auditor | 产品、Runtime、Agents 和场景闭环 | 建议按 baseline、canonical contracts、场景验收、可靠性/生命周期、扩展分类和 host trigger 拆分；指出版本与 DTO 漂移。 | accepted；将 canonical query 和 host trigger 保持为独立 child。 |
| capability-roadmap auditor | capability、质量门禁和 roadmap 一致性 | 指出 9 个 Purpose 不能一刀切处理，workflow-verification 存在合同冲突，旧 roadmap 和 authority 指针失效。 | accepted；将质量门禁纳入场景验收，将 capability / roadmap 放到最终 closure。 |

## 关键发现

### 稳定设计声明与草案状态矛盾

- 证据：`.wiki/06-设计文档/INDEX.md`、`03-核心场景.md`、`04-扩展场景.md`。
- 说明：INDEX 把目录定义为稳定 SSOT，但场景正文仍明确标记为草案或未来场景。
- 影响：product-contract 必须定义 design status 和完成标准，documentation-closure 最后同步状态。

### 缺少唯一版本和合同真相

- 证据：Runtime、Agents、release gap 和 capability 文档同时使用 v0.1.0、v0.2.0、v0.3.0 与 3.0。
- 说明：architecture baseline、product / CLI release 与 package / crate artifact version 是不同版本域；数值不同本身不是漂移，但 authority 和映射关系未定义会导致“当前承诺”无法唯一判断。
- 影响：product-contract 必须先建立版本域 authority 与映射规则，不要求跨版本域数值拉齐。

### 设计决策状态与交付证据被混用

- 证据：现有材料使用“当前”“稳定”“草案”“已归档”等不同词汇，但没有区分设计是否被采纳与实现是否被验证或发布。
- 说明：`adopted` 不能自动推出 `implemented`，`implemented` 也不能自动推出 `verified` 或 `released`；混成一个枚举会再次把文档采纳误判成项目完成。
- 影响：product-contract 必须要求设计决策状态与 delivery evidence 正交，具体字段与取值 deferred-to-design。

### Runtime 与 Agents query 消费漂移

- 证据：Runtime 已以 route groups、results 和逐结果 provenance 为主合同；Agents 仍列出 matched pages 和 provenance summary 作为稳定消费字段。
- 说明：宿主可能固化过时 DTO，并在 Runtime 外自行推断 query 语义。
- 影响：runtime-query-contract 先于 host-trigger-contract。

### 场景和质量门禁不可执行

- 证据：核心场景没有 command/API、formal artifacts、状态、恢复和 fixture 映射；workflow-verification 对样本 gate 的要求互相冲突。
- 说明：自然语言成功标准无法证明设计已经完成，也无法形成唯一 verification decision。
- 影响：core-scenario-acceptance 必须同时完成场景矩阵和 gate 决策模型。

### Capability 占位背后存在边界问题

- 证据：9 个 Purpose 为归档占位；agent-session-bridge 实际以 provider 直连为首落地；content-family 和 composition 仍带历史 page-first 词汇；decomposition 完成性证据不足。
- 说明：简单补 Purpose 会把历史命名和未验证能力继续包装成稳定合同。
- 影响：前置 children 先重定合同，documentation-closure 最后改名、迁移、补 Purpose 或拆分 capability。

## 方案与取舍

| 选项 | 优点 | 缺点 | 结论 |
| --- | --- | --- | --- |
| 单一大 change 重写全部设计 | 文件集中、表面进度快 | 目标不可独立验收，依赖混乱，容易把未来功能一起承诺 | 不采用 |
| 只清理 TBD、状态和 roadmap | 改动小、可快速完成 | 无法解决 DTO、门禁、场景和生命周期合同冲突 | 不采用 |
| 产品基线驱动的 multi-change | 每个合同域可独立 proposal/design/review，依赖清楚 | 周期更长，需要严格维护 parent 状态 | 采用 |
| 把全部扩展场景纳入 3.0 baseline | 看似能声明“全部设计” | 权限、外部 PR、onboarding 等会造成范围失控 | 不采用；允许 next / non-goal |

## 风险与未知项

| 项目 | 影响 | 处理方式 |
| --- | --- | --- |
| 3.0 与 semver 的映射尚未明确 | 当前/延期合同继续漂移 | 写入 product-contract proposal 和 design |
| 不同版本域被误要求数值拉齐 | 破坏独立发布单元的版本治理 | product-contract 明确版本域 authority，只要求关系可解释 |
| 设计状态与交付证据混为一谈 | adopted 被错误报告为 verified / released | product-contract 定义正交模型，具体 schema deferred-to-design |
| quality gate 历史 requirement 冲突 | verification 无唯一结论 | 写入 core-scenario-acceptance，禁止仅做措辞合并 |
| richer query 是否进入当前公开面 | 影响 DTO、CLI 和宿主 | deferred-to-proposal，由 baseline 和用户价值决定 |
| 大仓 compose 暴露实现缺口 | 设计可能无法被当前代码证明 | reliability child 记录后续实现 change，不伪装完成 |
| capability 改名或拆分影响索引链接 | 文档 closure 可能扩大 | deferred-to-design，并在最终 child 统一迁移链接 |
| 扩展场景范围膨胀 | program 无法结束 | 强制 baseline / next / non-goal 三分法 |

## 对当前 artifact 的影响

- 应写入：`split.md`
- 影响内容：
  - 采用 multi-change。
  - 六个 child 的目标、验收边界、顺序和依赖。
  - 将质量门禁冲突纳入核心场景验收。
  - 将 capability / roadmap 清理放到最后，不提前掩盖设计缺口。
- 后续阶段处理：
  - 每个 child 仍必须进入 `unispec-propose`，重新确认 problem、goals、non-goals 和 success criteria。
  - 接口、状态机、schema、测试矩阵和迁移细节 deferred-to-design / deferred-to-plan。

## 未采纳内容

- 不直接采用任何 upstream 实现；本轮没有新增 upstream 迁移或改写落点。
- 不把 active changes 为空视为设计完成证据。
- 不删除现有阶段材料；删除、迁移或保留策略由 documentation-closure 在前置合同完成后决定。
