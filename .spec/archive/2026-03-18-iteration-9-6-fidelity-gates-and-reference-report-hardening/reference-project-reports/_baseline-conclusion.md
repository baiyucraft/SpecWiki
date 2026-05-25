# 9.6 Baseline Conclusion

生成时间：2026-03-19T02:41:00Z
change：iteration-9-6-fidelity-gates-and-reference-report-hardening

## 结论口径

- 2026-03-19 已经重新跑通 `storybook` 的 fresh init，并产出 `ready` runtime；随后对同一 runtime 执行 `--skip-init --warm-reruns 2`，验证 warm report 稳定性。
- 因此，本文件记录的是“fresh runtime + warm report 对照”组成的 9.6 基线，而不是早先那次被 core blocker 卡住的中间态。

## 样本状态

- `storybook`：`ready` / `acceptance_candidate`
- `dagger`：`runtime_incomplete` / `diagnostic_only`
  当前只完成到 knowledge planning / research，尚未写出 `wiki.metadata.json` 与最终 Markdown。

## 9.6 四个专项问题

- 页数是否接近 reference：`storybook` 为 `98.30%`，`matched=173/176`、`missing=3`；`dagger` 当前不能进入页级 fidelity。
- 是否存在 coarse page reuse：`storybook reuse_overage=129`，many-to-one reuse 仍然非常严重，是最主要的结构问题。
- docs-backed 页面是否具备 reference 式骨架：`storybook median_skeleton_fidelity=0.2941`，说明 section plan 到最终 Markdown 的骨架落地仍然偏弱。
- 正文是否覆盖关键文件：`storybook median_key_source_coverage=0.0513`，说明 citation / key-source grounding 仍远未收敛。

## Warm Stability

- `storybook`：两次 warm report 的 `reuse_overage / median_skeleton_fidelity / median_key_source_coverage` 波动均为 `0`，可作为 9.7-9.9 的稳定验收基线。
- `dagger`：两次 warm report 都稳定落在 `runtime_incomplete`，这能稳定诊断 assemble 未完成，但不进入 fidelity denominator，也不构成页面质量基线。

## Fresh Runtime Recovery

- 早先 fresh run 失败点来自 core 写盘，而不是 9.6 报告层：
  `write_knowledge_units: unresolved parent/domain dependencies`
- 这轮已在 planner 修复 dangling `parent_unit_id`：
  - docs dedup 后修 orphan parent
  - 全局 prune 后再修 orphan parent，并回退到最近仍存在的祖先或同域 `DomainIndex`
- 修复后 fresh init 的当前 runtime 状态：
  - `storybook`：`ready`，`knowledge_units=195`，`unit_research=188`，`page_drafts=195`，`wiki_pages=195`
  - `dagger`：`runtime_incomplete`，`knowledge_units=63`，`unit_research=37`，`page_drafts=0`，`wiki_pages=0`
- 结论：`storybook` 现在已经具备 fresh baseline 资格，并经 warm rerun 验证稳定；`dagger` 仍然是 assemble blocker，而不是 page fidelity blocker。

## 后续 9.7-9.9 的主要收敛入口

- 首先回收 `many-to-one reuse`，避免父页继续吞并多个 reference 主题。
- 其次提升 `skeleton fidelity`，让 research section plan 真正落到 compose/renderer 的最终骨架上。
- 再提升 `key source coverage`，把 citation / evidence / source grounding 从“有提及”推进到“覆盖 reference 关键文件”。
- `dagger` 需要先解决 `runtime_incomplete`，否则不能把它当成页面质量问题继续分析。
