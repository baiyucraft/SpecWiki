## Context

当前仓库已经实现了 `Repository Scan -> ModuleTree -> Page Planner -> Wiki Assembler` 的 baseline 主链，但距离“迭代 1 完成”还差最后一轮收口。现状的主要缺口有三类：

- `ModuleTree` 仍偏向“根模块 + 一层子模块”，递归层级、稳定排序和 `child_ids` 回填还不够完整。
- `Page Planner` 虽然具备父子页面概念，但页面路径和页面父子关系还没有完全建立在真实递归模块树之上。
- `query` 已能返回页面、模块、源码信息，但主命中路径还混有较重的 Markdown 检索，结构优先语义不够清晰。

本次设计以 [.wiki/06-设计文档/00-总体设计.md](E:/project/!byAI/spec-wiki/.wiki/06-设计文档/00-总体设计.md) 的“迭代 1：Deterministic Structural Baseline”为直接边界，并遵守 [AGENTS.md](E:/project/!byAI/spec-wiki/AGENTS.md) 中的开发参考：

- [tmp/codewiki-upstream/codewiki](E:/project/!byAI/spec-wiki/tmp/codewiki-upstream/codewiki) 主要用来参考依赖分析、递归模块拆分和自底向上的层级页面组织。
- [tmp/deepwiki-rs-upstream](E:/project/!byAI/spec-wiki/tmp/deepwiki-rs-upstream) 主要用来参考 deterministic preprocess、workflow、context 和 cache 分层。
- [tmp/reference-zh/content](E:/project/!byAI/spec-wiki/tmp/reference-zh/content) 与 [tmp/reference-zh/meta/repowiki-metadata.json](E:/project/!byAI/spec-wiki/tmp/reference-zh/meta/repowiki-metadata.json) 主要用来对齐 `.wiki` 页面组织和 metadata 导出形态。

约束保持不变：

- 事实层继续保持 deterministic，不引入 LLM。
- 当前宿主和平台边界不扩展，仍以现有 Windows + CodeBuddy 集成为准。
- 不在本次变更中引入 `WikiState` 主模型迁移；那属于后续“State Kernel”迭代。

## Goals / Non-Goals

**Goals:**

- 让 `ModuleTree` 从一层结构收口到可递归、可稳定重建的层级模型。
- 让结构化解析能力只围绕“建树正确性”补强，而不是提前扩展解释层。
- 让页面规划、页面路径和 metadata 导出与递归模块树完全对齐。
- 让 `query` 以结构化索引为主命中来源，并返回基础 provenance。
- 建立一组能代表 baseline 完成度的验收夹具，而不是只靠零散单元测试判断完成。

**Non-Goals:**

- 不在本次变更中实现 `WikiState` 成为内部唯一主模型。
- 不在本次变更中实现真正的 dirty graph、局部重建或增量更新。
- 不在本次变更中引入 LLM 总结、RAG、TOON 或图示生成。
- 不在本次变更中扩展新的宿主、平台或版本控制抽象。

## Decisions

### 决策 1：递归模块树继续建立在“归一化扫描事实”之上，而不是直接递归目录

模块树的输入继续来自 `Repository Scan` 和结构化解析结果，但会先对 workspace、manifest、入口、目录角色和依赖线索做归一化，再进入递归拆分。模块发现顺序、稳定 ID 和 `child_ids` 回填都以归一化后的模块候选集合为准，不允许页面规划直接读取原始目录树自行推断。

这样做的原因是：

- 递归目录虽然更容易写，但会把“目录存在”误当成“模块存在”。
- 归一化 facts 更容易测试，也更容易在后续做 cache 和状态迁移。
- 这与 [tmp/codewiki-upstream/codewiki](E:/project/!byAI/spec-wiki/tmp/codewiki-upstream/codewiki) 里“依赖分析 -> 模块聚类 -> 层级生成”的参考原则一致。

备选方案：

- 直接按目录递归生成模块树：实现简单，但会让非模块目录、基础设施目录和 workspace 边界混在一起。
- 先引入 AST 或 LLM 再建树：精度可能更高，但成本和不确定性都超出本迭代边界。

### 决策 2：页面身份、页面路径和父子关系必须由模块祖先链唯一导出

总览页和架构页继续作为顶层固定页面；模块页则统一从模块祖先链导出页面 ID、相对路径和父页面关系。路径生成必须是 Windows 安全的，并处理模块重名、非法字符和同名祖先路径冲突。

这样做的原因是：

- 只有把页面身份绑定到模块祖先链，才能保证递归层级重建后页面不会随扫描顺序漂移。
- 这可以直接对照 [tmp/reference-zh/content](E:/project/!byAI/spec-wiki/tmp/reference-zh/content) 的层级页面组织方式，而不是继续生成一组平铺模块页。

备选方案：

- 保留当前较平铺的模块页路径：迁移更轻，但会让层级页面与模块树脱节。
- 让页面生成器自行命名路径：实现灵活，但 ID 稳定性最差。

### 决策 3：`query` 改为结构优先命中，Markdown 仅作回退

`query` 的主索引来源收敛为页面、模块、源码和关系四类结构化对象。只有在结构化索引没有足够命中时，才允许回退到 Markdown 内容匹配；即使发生回退，结果中也必须显式标出命中原因和 provenance。

这样做的原因是：

- 结构优先检索才能和后续 `State Kernel`、`Incremental Runtime` 保持一致事实源。
- 这与 [tmp/deepwiki-open-upstream](E:/project/!byAI/spec-wiki/tmp/deepwiki-open-upstream) 的消费层思路一致，即把检索对象和出处显式建模，而不是把文本搜索当成唯一能力。

备选方案：

- 继续维持 metadata 与 Markdown 混合检索：迁移最小，但后续很难解释命中原因。
- 完全移除 Markdown 回退：行为更纯粹，但会降低 baseline 阶段的可用性。

### 决策 4：metadata 继续承担 baseline 结构导出，但不抢跑状态内核

本次会扩展 `wiki.metadata.json` 的层级字段和 provenance 字段，让它足以支撑结构优先 query 和页面重建判断；但不会在本次引入 `WikiState` 主模型替换现有 workflow。也就是说，这一轮做的是“让导出足够表达 baseline 结构”，而不是“重写内部状态内核”。

这样做的原因是：

- 迭代 1 的收口目标是把结构输入和结构输出做稳，而不是同时引入另一轮运行时迁移。
- 这能避免把“事实层收口”和“状态层收口”混在一个 change 中，导致范围失控。

备选方案：

- 直接把 `WikiState` 迁移一起做：长期更整洁，但会和迭代 2 边界重叠。

### 决策 5：验收以中型仓库夹具为主，而不是继续堆单点测试

会补一组以中型本地代码目录为目标的验收夹具，覆盖 monorepo、混合目录、非 workspace 和基础设施目录等形态，并围绕 `init -> metadata/cache -> query` 做整体断言。单元测试继续存在，但不再作为“baseline 已完成”的唯一判断口径。

这样做的原因是：

- 当前缺口主要出在“多个步骤串起来之后是否仍然结构稳定”，不是单个函数能否返回值。
- 这也更接近 [tmp/reference-zh](E:/project/!byAI/spec-wiki/tmp/reference-zh) 作为 `.wiki` 产物参考时的真实验收方式。

备选方案：

- 只补单元测试：成本更低，但很难验证结构优先 query 和页面层级是否真实协同工作。

## Risks / Trade-offs

- [模块拆分规则仍然偏启发式] -> 先把稳定顺序、递归层级和测试夹具做实，避免在本迭代引入更重的解析体系。
- [页面路径规则一旦调整会带来已有产物变化] -> 将路径生成规则集中在 planner/runtime 层，并用 fixture 固定期望输出。
- [结构优先 query 可能导致部分旧的文本命中下降] -> 保留 Markdown 回退，并在返回结构中显式区分主命中与回退命中。
- [baseline 验收夹具维护成本上升] -> 只保留少量代表性 fixture，不把每一种语言或框架都变成独立大样本。

## Migration Plan

1. 先收口 `ModuleTree` 和结构化解析，使递归建树结果稳定。
2. 再让 `Context Builder`、`Page Planner`、metadata 导出和页面路径全部切换到递归模块树。
3. 最后切换 `query` 到结构优先路径，并补齐 baseline 验收夹具和整体测试。

本次不涉及发布层迁移；如果页面路径发生变化，以 `rebuild` 重新生成 `.wiki/` 为主要恢复路径。

## Open Questions

- 基础设施目录在递归模块树中默认归为独立模块，还是优先并入最近的业务父模块，目前仍需以 fixture 结果来校准。
- 结构优先 query 的命中排序是否需要区分“页面命中优先”还是“模块命中优先”，本次先只要求命中来源清晰，不强行固定复杂排序策略。
