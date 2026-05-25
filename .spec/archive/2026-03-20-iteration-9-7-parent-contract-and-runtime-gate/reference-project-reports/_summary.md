# Reference 项目集汇总

生成时间：2026-03-19T23:04:31.750Z
变更：iteration-9-7-parent-contract-and-runtime-gate
项目集：storybook、dagger
requested_run_mode：cold
skip_init：true

| Project | Status | Runtime | overall | reuse_overage | median_skeleton | median_key_source | missing | collapsed | extra | warm_stable |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| storybook | ready | ready | 92.05% | 123 | 0.13 | 0.03 | 14 | 137 | 156 | true |
| dagger | ready | ready | 95.38% | 25 | 0.08 | 0.07 | 3 | 35 | 27 | true |

## Gate

- storybook：not-pass，overall=92.05% / reuse_overage=123 / median_skeleton=0.13 / median_key_source=0.03 / warm_stable=true
- dagger：not-pass，overall=95.38% / reuse_overage=25 / median_skeleton=0.08 / median_key_source=0.07 / warm_stable=true

## 高频差距

- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足：2 个项目
- evidence block 已进入页面，但覆盖率和密度仍低于 reference：2 个项目
- 单页章节拆分比 reference 粗，主题混杂在同一页里：2 个项目
- docs-backed 页面主章节骨架仍偏离 reference，存在英文原始 docs heading 或泛化模板回退：2 个项目
- 解释层正文密度仍低于 reference：2 个项目
- 高频缺失专题集中在：流程主题、核心机制主题：1 个项目
- 高频缺失专题集中在：流程主题：1 个项目

## Stability

- storybook：stable，reuse_delta=0 / skeleton_delta=0 / key_source_delta=0
- dagger：stable，reuse_delta=0 / skeleton_delta=0 / key_source_delta=0
