# Reference 对比后的优化收敛

## 当前收敛

这轮 9.3 已经把 targeted dossier、section-plan-driven 页面组装和 archetype 专题页接入主链，但 reference 项目集仍然能看出剩余差距主要集中在 planner 覆盖率、citation 密度和 research 结果对正文结构的主导程度。

## 高频观察

- 已生成专题页的项目：18/18
- 已生成 repo-archetype 专题的项目：18/18
- 已落 evidence block 的项目：18/18
- 已落 Mermaid 图的项目：17/18
- 平均 citation 密度：generated 0.87 / reference 62.3
- 高频缺失专题：核心机制主题(1)、中间件主题(1)

## 下一步建议

- 优先继续调 planner 阈值和 topic seed 规则，让根级机制页、流程主题页、repo-archetype 专题覆盖更多 reference 高频主题。
- evidence layer 下一步应补 citation 密度和 section 内证据命中率，而不是回退到全文文件清单。
- overview/architecture 的 research 结果需要更稳定落页，否则 section-plan 对总览页的收益会被 budget 和 fallback 抵消。
- Mermaid 已进入主链，下一步重点是让更多页面拥有 diagram inputs，而不是放宽 LLM 自由生成结构图。
