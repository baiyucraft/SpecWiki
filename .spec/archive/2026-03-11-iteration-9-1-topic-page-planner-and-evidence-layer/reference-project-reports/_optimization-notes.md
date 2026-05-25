# Reference 对比后的优化收敛

## 当前收敛

这轮 9.1 已经把专题页、evidence layer 和 deterministic Mermaid 接入主链，但 reference 项目集仍然能看出剩余差距主要集中在 coverage 阈值和页面粒度，而不是简单的正文措辞。

## 高频观察

- 已生成专题页的项目：10/10
- 已落 evidence block 的项目：10/10
- 已落 Mermaid 图的项目：10/10
- 高频缺失专题：中间件主题(2)

## 下一步建议

- 优先继续调 planner 阈值和 topic seed 规则，让根级机制页、流程主题页和模块能力页覆盖更多 reference 高频主题。
- evidence layer 下一步应补“证据分组更细”和“模块页/专题页的 section 内证据密度”，而不是回退到全文文件清单。
- Mermaid 已进入主链，下一步重点是让更多页面拥有 diagram inputs，而不是放宽 LLM 自由生成结构图。
