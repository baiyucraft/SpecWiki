# Reference 对比后的优化收敛

## 当前收敛

这轮 9.5 的验收口径已经固定为 storybook + dagger，目标不是泛化地“更像 reference”，而是让两个样本都在最终 `.wiki/*.md` 上达到 `overall_match_rate >= 95%`，同时把差距拆成 missing / collapsed / low-fidelity 三类来收敛。

## 高频观察

- 已生成专题页的项目：1/1
- 已落 evidence block 的项目：1/1
- 已落 Mermaid 图的项目：1/1
- 平均 citation 密度：generated 83.64 / reference 79.69
- 总体对齐率：dagger=100%
- 三类差距：dagger[missing=0, collapse=0, low-fidelity=65]
- 高频缺失专题：无
- 章节骨架短板：dagger=61
- 英文 raw docs 残留：dagger=matched 0 / extra 0
- extra generated：dagger=63
- stop reasons：dagger[budget=0, stalled=0, invalid=0, failed=1]

## 下一步建议

- 继续压低 low-fidelity：优先补 citation 密度、Mermaid 覆盖率和关键文件提及，避免只命中结构不命中正文。
- 把 invalid_output / provider_error 单独排查，避免把 provider 失败误判成页面内容质量问题。
- docs-backed 页面继续按 reference 骨架和本地化命名收敛，避免回退到英文 raw docs 标题或目录。
- 若 overall 已稳定，优先回收拆分阈值和派生页面预算，先压 extra generated pages，再做尾差润色。
