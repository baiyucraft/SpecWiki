## Context

当前迭代 1 已完成结构 baseline 的主链，但真实产物仍暴露出两类收口问题：

- 模块页 `关键源码` 仍采用“模块内前几个文件”的近似策略，导致日志、锁文件、纯文档和通用配置容易进入摘要。
- 跨模块关系虽然已经有结构，但在前后端混合仓库和基础设施目录中仍然偏保守，模块页与 query 经常只能给出“无依赖”的弱结果。

这次变更继续遵守 [.wiki/06-设计文档/00-总体设计.md](E:/project/!byAI/spec-wiki/.wiki/06-设计文档/00-总体设计.md) 的迭代 1 边界，不引入 `WikiState`、增量更新或 LLM。参考物仍以 [tmp/codewiki-upstream/codewiki](E:/project/!byAI/spec-wiki/tmp/codewiki-upstream/codewiki)、[tmp/deepwiki-rs-upstream](E:/project/!byAI/spec-wiki/tmp/deepwiki-rs-upstream) 和 [tmp/reference-zh](E:/project/!byAI/spec-wiki/tmp/reference-zh) 为主，但实现边界以当前设计和 [AGENTS.md](E:/project/!byAI/spec-wiki/AGENTS.md) 为准。

## Goals / Non-Goals

**Goals:**

- 让模块页 `关键源码` 稳定偏向入口、依赖线索、核心实现文件，而不是低信号文件。
- 让跨模块关系在混合仓库样本上更容易被识别并进入模块页和 query 结果。
- 让模块页摘要和 query 命中说明建立在同一套高信号 facts 上。
- 把 `aLocal` 的 `.wiki` 产物快照纳入 baseline 对照流程，保证任务和实现能被真实产物反向校正。

**Non-Goals:**

- 不改动页面总体类型，不新增新的知识主题页体系。
- 不迁移 `WikiState` 主模型，不触碰迭代 2 的状态内核边界。
- 不引入 LLM 总结、RAG 或 diagram 能力。

## Decisions

### 决策 1：关键源码改成“信号排序”而不是“简单截断”

模块上下文中的 `key_sources` 不再直接取模块内前几个文件，而是先按信号排序，再截取少量候选。高信号优先级依次包括：入口文件、显式依赖证据文件、模块根下的主源码、具备高频技术后缀的实现文件；低信号文件如日志、锁文件、临时文件、纯文档和辅助配置会被降权或直接排除。

这样做的原因是：

- 当前产物中的主要噪声不是“缺文件”，而是“选错文件”。
- 只改排序和过滤即可提升模块页质量，不需要引入新模型。

备选方案：

- 继续沿用当前顺序截断：实现最简单，但产物噪声会持续存在。
- 由渲染层再做二次过滤：实现分散，且 query 与页面会使用不同事实源。

### 决策 2：跨模块关系继续保持 deterministic，但扩展启发式来源

跨模块关系仍然只接受 deterministic 线索，不引入语义推理；但来源会扩展到 manifest 依赖、导入别名、入口引用和典型前后端/基础设施路径约定。关系证据必须继续能回溯到文件路径或清晰来源。

这样做的原因是：

- 迭代 1 需要的是“更可靠的基础关系”，不是“更聪明但不可控的关系”。
- 这能直接增强模块页和 query，而不影响后续状态内核设计。

备选方案：

- 只依赖 manifest：精度更高，但会漏掉大量非包管理器驱动的仓库关系。
- 引入更重的 AST 分析：收益可能更高，但超出本次收口范围。

### 决策 3：模块页摘要与 query 共用同一组结构事实

模块页的 `模块角色 / 关键源码 / 依赖模块 / 被依赖模块` 和 `query` 返回的 `reasons / provenance` 必须尽量共用同一组模块事实，避免页面和查询各自做不同的信号判断。

这样做的原因是：

- baseline 阶段最怕“页面说一套，query 说一套”。
- 共用事实能让 `aLocal` 对照时更容易定位差异来源。

备选方案：

- 页面和 query 各自维护筛选逻辑：短期更快，但后续维护成本会快速上升。

### 决策 4：`aLocal` 对照流程作为验收步骤，而不是新的导出契约

`E:\\project\\aLocal` 的 `init` 产物会固定快照到 `tmp/` 下，并与 [tmp/reference-zh](E:/project/!byAI/spec-wiki/tmp/reference-zh) 做差异对照；但这个对照只作为“发现内容组织和信号质量差距”的验收步骤，不把 `reference-zh` 直接当成强制 schema 或逐页 golden output。

这样做的原因是：

- `reference-zh` 是成熟参考样例，不是当前系统必须字节级对齐的输出协议。
- 用它做方向校准是有价值的，但不能让它反客为主替代设计边界。

## Risks / Trade-offs

- [关键源码过滤过严] -> 保留入口和高信号配置作为白名单，避免页面过度瘦身。
- [关系启发式扩展后引入误报] -> 要求每条关系继续保留证据路径，并在 fixture 上固定期望。
- [reference 对照导致范围膨胀] -> 明确只对照页面结构、信号质量和 metadata 组织，不追逐 reference 的主题页广度。

## Migration Plan

1. 先调整模块上下文中的 `key_sources` 选择和低信号文件过滤。
2. 再扩展跨模块关系提取，并同步收紧模块页摘要内容。
3. 最后补 query 验证、`aLocal` 快照对照和对应测试。

## Open Questions

- 某些配置文件是否应作为高信号保留，需要以 `aLocal` 与 fixture 的实际产物来校准。
- 前端调用后端 API 的关系是否需要细分成专门关系类型，本次先不强行新增类型，只先提高召回。
