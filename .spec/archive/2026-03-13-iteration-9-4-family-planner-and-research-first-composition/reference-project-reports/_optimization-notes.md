# Reference 对比后的优化收敛

## 当前结果

9.4 的主链重构已经把 core 从旧的 `module/topic/family` 混合页面规划，推进到了更接近 [DESIGN-CORE2.0.md](E:/project/!byAI/spec-wiki/DESIGN-CORE2.0.md) 的 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble` 结构。`storybook + dagger` 两个验收样本的 init / lifecycle 都已经能稳定跑通，说明这轮的主要问题已经不再是 runtime 稳定性，而是页面质量、知识单元粒度和最终产物与 reference 的贴近度。

当前专项结果：

- `storybook`: generated `142` / reference `176` / matched `126` / missing `50`
- `dagger`: generated `71` / reference `65` / matched `34` / missing `31`
- `citation(gen/ref)`: `0 / 77.72`（storybook），`0 / 79.69`（dagger）
- `diagram(gen/ref)`: `0 / 176`（storybook），`0 / 65`（dagger）

这说明 9.4 已经解决了“主链架构不对”和“workflow 不稳定”的问题，但和 reference 的主要差距仍然非常大，而且不是简单调 prompt 能补上的。

## 高频观察

- 已生成专题页的项目：1/2
- 已生成 repo-archetype 专题的项目：0/2
- 已落 evidence block 的项目：0/2
- 已落 Mermaid 图的项目：0/2
- 平均 citation 密度：generated 0 / reference 78.7
- 高频缺失专题：无
- `storybook` 的主要差距是 docs-heavy / product-doc-heavy 仓库的中粒度页面仍然折叠过多
- `dagger` 的主要差距是 runtime-heavy / compiler-heavy 仓库的 API / framework / tutorial / testing 页面仍然缺失或被压回大模块页

## DESIGN-CORE2.0 达成情况

### 已经做到的

- Facts 层大体到位：扫描、符号图、图分析、模块树仍然是稳定 deterministic 主链。
- Knowledge Planning 层已经正式落地：`KnowledgeDomain / KnowledgeUnit / KnowledgeTree` 已进入主链，不再只是 proposal 里的概念。
- Leaf-first / parent-consume-child 的处理顺序已经有正式实现：`processing_order`、`PageDigest`、父页消费子页 digest 都已经接入 workflow。
- Workflow / Runtime / Storage 已切到 2.0 主路径：`init / update / rebuild` 现在都走 `knowledge_planning -> research -> compose`，并持久化 `knowledge_domains / knowledge_units / research_cache / page_digests / pipeline_checkpoint`。
- Checkpoint / Resume 已经可用：LLM 或 compose 中断后能保存检查点并恢复。
- storybook + dagger 的 lifecycle 现在都能完整通过，说明 2.0 结构级改造已经可跑、可测、可维护。

### 部分做到，但还没有真正达标的

- Research 层只完成了结构壳子，还没有变成真正的 provider-first、research-first 内容主驱动。当前主线更多还是 `StructuralResearchProvider` 在兜底，LLM research 的地位没有完全变成 2.0 设计里说的“硬前置主驱动器”。
- Compose 层虽然已经有 `PageDraft / PageDigest / section_plan`，但最终页面还没有做到“完全由 research 结果主导”，仍有明显的 deterministic section/template 痕迹。
- Knowledge-unit driven 已经比 9.3 前强很多，但还没有彻底摆脱旧的 `topic/family/module` 时代的心智模型。很多 profile / helper 仍然带着旧页面体系的影子。

### 明确还没做到的

- `LLM-required Research / Compose` 没有真正实现。2.0 设计里说的是 Research 和 Compose 必须接 LLM，不支持退化，但当前真实运行并没有完全达到这个状态。
- `Citation-driven Content` 没有做到。内部虽然已经有 citation 数据结构，但最终 Markdown 没有把 citation 变成 reference 那种正文内可见、可统计、可追溯的出处层，所以报告里仍然是 `0`。
- `Diagram-driven pages` 没做到。图输入结构存在，但没有稳定落成可统计的正式页面图表达，所以 diagram 覆盖还是 `0`。
- `storybook / dagger` 的知识单元粒度仍不够。当前页数和命中率表明我们已经比旧架构更接近 reference，但仍然没有拆到 reference 的中粒度页面树。
- `Research object split` 还没有真正做成 deepwiki-rs 那种分层研究对象。现在还没有稳定分出 `system / domain / api / config / runtime / testing / guide` 各自独立的 research object 主路径。

结论：

- 2.0 的“架构重构”已经大体做到。
- 2.0 的“内容质量承诺”还没有做到。

## 为什么系统会越来越复杂

当前复杂度上升的根因，不是功能做多了，而是把几类本来应该分开的事情揉进了一条页面主链：

- 事实层：扫描、模块树、符号图、流程图
- 拆页层：到底该生成哪些知识单元
- 研究层：LLM 应该研究什么
- 成页层：章节、引用、图怎么落
- 消费层：query / session / agent

在当前代码里，这几层虽然名义上已经拆开，但实际边界还不够硬，导致：

- `page_enrichment` 和 `page_research` 这种历史过渡语义曾长期并存
- `topic / family / module` 几套页面语义互相挤压
- `PageResearchResult / PageComposePlan / PageDigest / child digest` 等对象曾长期重复搬运类似信息
- provider / agent / session / tools 这类消费层能力过早影响主生成链

真正该收住的主线应该是：

1. `Facts Layer`
2. `Knowledge Unit Planner`
3. `Research Layer`
4. `Compose Layer`

不是继续把“页面类型”当一等抽象，而是把“知识单元”当一等抽象，把页面当结果。

## 四个参考库真正该借什么

### CodeWiki

真正值得借的是：

- leaf-first
- source-fed
- parent-consume-child-docs

它的关键不是 prompt 写法，而是：先确定文档单元，叶子先吃源码生成文档，父页再消费子文档结果。当前 core 还没有把这件事做彻底。

### deepwiki-rs

真正值得借的是：

- research-first
- research object split
- compose 只消费研究结果

它不是一个薄薄的 `page_research` 包天下，而是把 `system / architecture / workflow / key modules / boundaries` 先拆成不同研究对象。这正是当前 `storybook` 和 `dagger` 都还缺的一层。

### GitNexus

真正值得借的是：

- 厚 facts 层
- 图事实优先
- 解析/调用/社区/流程分层处理

它提醒我们：复杂度应更多放在事实层与图层，而不是把页面猜测做得越来越复杂。

### deepwiki-open

真正值得借的是：

- 生成与消费分离
- query/session 属于消费层，不该过早绑进生成主链

这意味着 agent/session/tooling 的复杂度不应该继续成为 9.4 的主矛盾。

## 当前和 reference 的真正差距

### storybook

`storybook` 的 reference 本质上不是代码模块树，而是：

- 产品知识树
- API 树
- 配置树
- 插件树
- 多框架树
- 主题/故障排除/测试树

当前 9.4 已经长出了 family 树，但仍然缺：

- `family-child` 之下更细的 leaf 文档单元
- citation 真正进正文
- diagram 真正进页面
- child page 结果对父页的更强主导

### dagger

`dagger` 暴露的是另一类问题：

- runtime-heavy / compiler-heavy / framework-heavy 仓库仍然缺乏足够细的知识单元拆分
- 大量 reference 页被压回 `dagger-runtime` 这类大模块页
- API / Framework / Testing / Guide 还没有形成稳定的中粒度知识页树

所以 9.4 现在不是“只差一点点”，而是：

- `storybook` 偏“页树方向对了，但 citation/diagram/leaf 粒度不够”
- `dagger` 偏“知识单元类型和拆分策略还不够”

## 下一步收敛方向

基于当前 9.4 的实现和 reference 对比，下一步不应该继续优先追加新的 page type 或 agent/session 能力，而应收敛到三件事：

1. **Section-scoped citation rendering**
   - citation 不能继续停在内部结构里
   - 必须真正进入最终 Markdown 正文
   - 否则统计永远是 `0`，页面也永远不像 reference

2. **Diagram draft -> markdown 的正式落盘**
   - 图输入已经有了，但没有正式成为页面主表达
   - 需要把 diagram draft 稳定渲染进页面并进入报告统计

3. **Knowledge unit 的中粒度继续拆分**
   - 对 `storybook`：继续补 `family-child -> family-leaf-doc`
   - 对 `dagger`：补 `runtime / api / framework / testing / guide` 的中粒度知识单元
   - 不再围绕旧的 topic/family 页面语义修修补补

## 方向结论

当前 9.4 的正确结论不是“继续把 page pipeline 做复杂”，而是：

- 保留 deterministic facts
- 保留 knowledge planning 主链
- 保留 leaf-first / parent-consume-child
- 继续向 `knowledge-unit driven` 收敛
- 把 citation / diagram / 中粒度 knowledge unit 作为下一阶段质量主战场

一句话总结：

**9.4 已经把 core 拉到了更合理的 2.0 架构上，但距离 reference 的主要差距，已经不在 workflow 稳定性，而在“知识单元拆解 + research 内容主导 + citation/diagram 正式落页”这三件事。**
