# Reference 项目集汇总

生成时间：2026-03-18T18:42:28.416Z
变更：iteration-9-6-fidelity-gates-and-reference-report-hardening
项目集：storybook、dagger
requested_run_mode：cold
skip_init：true

| Project | Status | Runtime | overall | reuse_overage | median_skeleton | median_key_source | missing | collapsed | extra | warm_stable |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| storybook | ready | ready | 98.30% | 129 | 0.29 | 0.05 | 3 | 150 | 151 | true |
| dagger | runtime_incomplete | runtime_incomplete | N/A | N/A | N/A | N/A | N/A | N/A | 0 | false |

## Gate

- storybook：not-pass，overall=98.30% / reuse_overage=129 / median_skeleton=0.29 / median_key_source=0.05 / warm_stable=true
- dagger：fail-hard，runtime_state=runtime_incomplete

## 高频差距

- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足：1 个项目
- evidence block 已进入页面，但覆盖率和密度仍低于 reference：1 个项目
- 单页章节拆分比 reference 粗，主题混杂在同一页里：1 个项目
- docs-backed 页面主章节骨架仍偏离 reference，存在英文原始 docs heading 或泛化模板回退：1 个项目
- 解释层正文密度仍低于 reference：1 个项目
- 高频缺失专题集中在：流程主题：1 个项目
- 仍存在明显 page collapse，同一生成页承担多个 reference 页面：1 个项目
- runtime 仍处于 runtime_incomplete，当前只能做诊断，不能纳入 fidelity 验收基线：1 个项目

## Stability

- storybook：stable，reuse_delta=0 / skeleton_delta=0 / key_source_delta=0
- dagger：unstable，reuse_delta=N/A / skeleton_delta=N/A / key_source_delta=N/A
