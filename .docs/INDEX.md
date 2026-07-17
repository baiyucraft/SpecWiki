# 阶段与参考材料索引

`.docs/` 只保留明确登记、无 authority 的外部理念或阶段参考。当前稳定项目知识进入 `.wiki/`，change 决策与验证证据进入 `.spec/changes/**` 或 `.spec/archive/**`；未登记文件不得把本目录重新变成 active queue、设计 SSOT 或 release authority。

## 当前材料

| 文件 | 状态 | Authority | Adopted refs |
| --- | --- | --- | --- |
| [Karpathy LLM Wiki 分析](./research/karpathy-llm-wiki-analysis.md) | `reference` | `none` | `.wiki/06-设计文档/00-总体设计.md` |

## 维护规则

- 每个 survivor 必须在 front matter 中声明 `status`、`authority: none` 和 `adoptedRefs`，并在本索引登记一次。
- Reference 中的建议、路线和未来设想不构成 active backlog；采纳内容必须通过 UniSpec change 进入当前 Wiki authority。
- 已迁移设计、已完成 roadmap、release gap 和质量分析不保留重复存根；历史原文与 review/test evidence 由 `.spec/archive/**` 保存。
