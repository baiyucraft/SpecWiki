# close-specwiki-3-0-design-baseline-documentation-closure

## 问题

前五个合同 child 已分别稳定产品基线、Runtime query、核心场景、可靠性生命周期和 Codex-first 宿主触发合同，但全库当前文档尚未完成最终收口。部分 capability 仍保留归档时生成的 Purpose 占位，历史 page/family-first 叙事与 KnowledgeUnit-first 主线并存，设计索引、公开入口、roadmap、阶段材料和 authority 指针仍可能把历史计划、已归档 change 或过时状态描述为当前事实。

如果只依赖人工整理，这些口径会在后续文档更新中再次漂移，项目也无法按照产品基线定义证明六个 child 已形成一致的 3.0 设计基线。

## 目标

- 执行 product-contract 定义的有界材料分类与迁移矩阵，收口当前设计、capability、公开 surface、release/roadmap 和阶段材料的角色与 authority。
- 清除 capability Purpose 占位，并让 capability 名称、Purpose 与正文准确表达当前已采纳合同，不包装未实现能力。
- 统一 KnowledgeUnit-first 主线与仍需保留的 family/page 投影术语，删除或改写把 family/page 当作一级系统本体的历史表述。
- 修正设计索引、状态标签、跨文档链接和 authority 指针，使当前材料只指向合法的长期 authority 或只读历史证据。
- 对失效 roadmap、迁移存根和质量分析材料完成保留、迁移或删除分类，避免阶段材料继续冒充当前计划或稳定合同。
- 建立自动化一致性门禁，阻止 Purpose、版本域、设计状态、active/archive 引用和 authority 关系再次漂移。

## 非目标

- 不重新设计或修改前五个已归档 child 的 Runtime、query、场景、可靠性或宿主触发行为合同。
- 不新增 CLI、Runtime DTO、持久化格式、宿主能力或产品功能。
- 不把 architecture baseline、product/CLI release、npm package 和 Rust crate 版本强制拉齐。
- 不把 release contract、manifest 或已归档 test report 误报为 registry、Git tag、binary checksum 或 staged publish evidence。
- 不改写 `.spec/archive/**` 历史 artifact；历史内容仅作为只读证据和迁移输入。
- 不保留过时状态词、旧 authority 指针或隐式版本映射的兼容层。

## 成功标准

- `.wiki/05-规格基线/capabilities/**/spec.md` 中不存在归档生成的 Purpose 占位；每个 capability 的 Purpose 与其当前合同边界一致。
- 当前设计、规格、公开入口和 `.docs` 材料不再把已归档 change 路径声明为 active authority，也不把已完成能力继续标为计划中。
- 设计 INDEX、场景状态和产品基线的 baseline/next/non-goal 分类一致，不把未实现阶段草案标为当前稳定设计。
- KnowledgeUnit 是正式规划、生命周期和刷新主线；仍保留的 family/page 术语均明确限定为信号、分解方式或页面投影，不形成平行本体。
- product-contract 的材料分类矩阵逐类得到处置证据；`.spec/archive/**` 保持只读，`.docs/**` 不被提升为当前架构 authority。
- 当前 release、manifest 和版本治理叙事能回链唯一 authority，并继续保持不同版本域正交。
- 自动化测试能在 Purpose 占位、失效 active-change 指针、错误设计状态、过时 roadmap 状态或关键 authority 漂移重新出现时失败。
- UniSpec full review、full verification 和归档校验通过，父 change 能记录第六个 child 已归档。

## 影响范围

### 包含内容

- `.wiki/05-规格基线/capabilities/**` 的 Purpose、命名、正文边界和索引。
- `.wiki/06-设计文档/**`、`.wiki/04-对外方法/**`、`.wiki/INDEX.md` 及必要公开入口的状态、authority 和交叉引用。
- `.docs/release/**`、`.docs/roadmap/**`、`.docs/design/**`、`.docs/quality/**` 的阶段材料分类与失效内容处置。
- 与长期文档一致性有关的自动化合同测试和现有测试编排入口。
- parent/child UniSpec metadata 的最终归档证据。

### 不包含内容

- `.wiki/.knowledge/**`、`.wiki/.cache/**`、`.wiki/pages/**` 和 `wiki.metadata.json` 等 runtime 产物。
- `.spec/archive/**` 的任何内容修改。
- 与当前文档收口无关的代码重构、依赖升级或发布动作。

## 交付形态

single-change

这是 parent `close-specwiki-3-0-design-baseline` 下顺序第 6 个 child。前五个依赖均已归档，本 change 只执行最终文档治理和一致性门禁，不重新拆分 parent scope。

## 风险

- capability 的历史名称和 Requirement 可能混合当前合同与未实现承诺；若只补 Purpose 会继续高估能力，必须以代码事实和前置 child 合同为准收口。
- 删除阶段材料可能丢失仍有价值的历史上下文；必须先区分稳定知识、阶段存根和只读历史证据。
- 全文关键词检查可能误伤合法的历史说明或否定性表述；自动化门禁需要限定当前 authority 范围和语义上下文。
- 文档收口触及面广，若顺手修改行为代码会扩大 change；实现必须保持在文档投影与一致性验证边界内。

## 未知项

- `product-release` 长期公开合同是否继续保留在 `.docs/release/**`，或迁入 `.wiki` 的具体落点，留到 design 根据当前发布流程决定。
- capability 的精确改名、合并或仅重写边界，留到 design 按当前代码和前置合同逐项判断。
- 各 `.docs/**` 文件应保留、迁移、改写还是删除的逐文件清单，留到 design 形成迁移矩阵。
- registry、Git tag、binary checksum 和 staged package 等真实发布证据的统一长期落点尚未定义；本 change 只保证不伪造这些证据。

## 参考资料

- [父 change 拆分方案](../close-specwiki-3-0-design-baseline/split.md)
- [设计债务与交付边界调研](../close-specwiki-3-0-design-baseline/research/design-debt-and-delivery-boundary.md)
- [product-contract 归档设计与材料迁移矩阵](../../archive/2026-07-15-close-specwiki-3-0-design-baseline-product-contract/design.md)
- [产品基线与设计治理](../../../.wiki/06-设计文档/05-产品基线与设计治理.md)
