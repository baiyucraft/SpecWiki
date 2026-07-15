# close-specwiki-3-0-design-baseline-product-contract

## 问题

当前仓库同时使用 `Repo Wiki Design 3.0`、产品发布 `v0.1.0 / v0.2.0 / v0.3.0`、npm package `0.2.0` 和 Rust crate `0.1.0` 等版本表达，但没有正式说明这些版本域各自代表什么、由哪个材料授权、相互之间如何映射。版本数值不同本身并不必然构成漂移；真正的问题是 authority、适用范围和映射规则不明确，导致“当前承诺”无法唯一判断。

设计材料还混用了“当前”“稳定”“草案”“已归档”等状态，却没有区分设计决策是否被采纳与实现是否完成、验证或发布。结果是 contract-closure 已归档、active changes 为空时，项目仍可能把“设计已采纳”误报为“整个项目设计和交付均已完成”。

本 change 需要先建立可被后续 Runtime、场景、可靠性、宿主和文档收口 children 共同引用的 canonical product baseline contract，而不是立即迁移全库文档或统一所有版本号。

## 目标

- 建立唯一、可引用的 Repo Wiki 3.0 canonical architecture baseline，并明确其产品目标、适用范围和非目标。
- 定义 architecture baseline、product / CLI release、package / crate artifact version 三类版本域的 authority 和映射规则，明确不同版本域不要求数值强制一致。
- 定义设计决策状态与 implementation / verification / release evidence 的正交关系，避免用单一状态同时表达“已采纳”和“已交付”。
- 定义单个合同域完成与全项目设计完成两层判定规则；第一个 child 只定义规则，不单独宣称全项目完成。
- 定义设计材料的 authority、适用范围、依赖、非目标和可验证条件要求，并产出供后续 children 使用的规范输入。
- 对现存当前材料形成分类与迁移清单，供最终 `documentation-closure` 执行全库迁移和一致性收口。

## 非目标

- 不在本 change 中统一 Runtime、CLI 与 Agents 的 query DTO、ranking、provenance 或降级语义。
- 不把 9 个核心场景改写为验收矩阵，也不处理历史 quality gate 冲突。
- 不设计跨宿主 trigger taxonomy、语料体系或 HostAdapter capability matrix。
- 不执行全库状态迁移、设计 INDEX 修正、旧 authority 链接清理、capability Purpose 补齐或 roadmap 删除。
- 不要求 npm package、Rust crates、产品 release 和 Design 3.0 使用相同版本号。
- 不修改历史 `.spec/archive/**`，不把历史 artifact 重新包装为当前 authority。
- 不实现产品或 Runtime 代码。

## 成功标准

- 存在一个明确、唯一且可引用的 Repo Wiki 3.0 canonical product baseline contract，后续五个 child 能将其作为规范输入。
- 版本真相模型至少覆盖 architecture baseline、product / CLI release、package / crate artifact version，并为每个版本域定义 authority、适用范围和允许的映射关系。
- 合同明确规定不同版本域数值不同不自动构成漂移；只有 authority、适用范围或声明关系冲突才被判定为设计问题。
- 设计决策状态与交付证据被定义为两个正交维度，能够表达“已采纳但未实现”“已实现但未验证”“已验证但未发布”等不同事实。
- 单个合同域完成清单至少要求 authority、范围、非目标、依赖和可验证条件明确；全项目设计完成必须要求 parent 六个 child 全部完成且最终一致性门禁通过。
- 现存当前材料被纳入一份有界的分类与迁移清单，至少覆盖 `.wiki` 当前设计与 capability、当前 release contract、package / crate manifests 和非权威 `.docs` 阶段材料；`.spec/archive/**` 明确只作为不可改历史证据。
- 本 change 的验收不依赖全库材料已经迁移；全库引用、状态和 INDEX 收口仍由 `documentation-closure` 独立验收。

## 影响范围

### 包含内容

- Repo Wiki 3.0 架构基线的产品级 authority 和完成定义。
- semver 产品发布线与独立发布单元版本的治理边界。
- 设计决策状态、交付证据和 authority 的项目级语义。
- 当前材料分类、迁移责任和后续 child 输入边界。
- 设计变更如何声明适用范围、依赖、非目标和验证条件的治理要求。

### 不包含内容

- Runtime Query 的具体输入输出字段和 transport schema。
- 核心场景的 command/API、formal artifacts 和 fixture 映射。
- reliability、knowledge lifecycle、host trigger 的具体合同。
- capability 重命名、文档移动、旧 roadmap 删除和一致性检查实现。

### 受影响角色

- 架构与产品维护者：需要用统一基线判断当前承诺和延期边界。
- Runtime、Agents 与 capability 维护者：后续设计必须引用同一 authority 和状态语义。
- Reviewer 与 Agent：需要区分设计采纳、实现、验证、发布和全项目完成状态。

## 交付形态

single-change

本 change 是 `close-specwiki-3-0-design-baseline` parent 下的第一个 child，只交付 canonical product baseline 和治理判定规则。它没有前置 child，并为后续五个 child 提供共同输入。

## 风险

- 如果状态模型继续把设计决策与交付证据混为一谈，后续仍会把 adopted 误报为 verified 或 released。
- 如果版本真相模型只追求版本号一致，会破坏 npm package、Rust crates 等独立发布单元的版本治理。
- 如果当前材料分类范围无限扩张，本 change 会退化为全库文档迁移；必须把实际迁移留给 `documentation-closure`。
- 如果 3.0 被直接等同于某个 semver release，架构长期目标和当前发布能力会再次混淆。

## 未知项

- canonical baseline contract 的具体文件位置、文档结构和引用方式留到 design 阶段确定。
- 设计决策状态与交付证据的具体字段名、取值集合及是否进入 frontmatter 留到 design 阶段确定。
- 版本域 authority matrix 的具体表达形式和机械校验入口留到 design / plan 阶段确定。
- 现存材料分类与迁移清单的持久化位置及最终 closure 消费方式留到 design 阶段确定。
- 本 change 是否需要增加最小权威入口，或只产出供最终 closure 迁移的正式合同，由 design 阶段在不承担全库迁移的前提下决定。

## 参考资料

- [Parent 拆分方案](../close-specwiki-3-0-design-baseline/split.md)
- [设计债务与交付边界调研](../close-specwiki-3-0-design-baseline/research/design-debt-and-delivery-boundary.md)
- [Repo Wiki Design 3.0](../../../.wiki/06-设计文档/00-总体设计.md)
- [v0.2.0 版本说明](../../../.docs/release/v0-2-0.md)
