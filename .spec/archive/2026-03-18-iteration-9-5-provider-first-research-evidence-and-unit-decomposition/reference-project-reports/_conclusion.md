# 9.5 最终结论摘要

## 最终结果

- `storybook`：`overall_match_rate = 176/176 = 100%`
- `dagger`：`overall_match_rate = 65/65 = 100%`
- 两个样本都已稳定满足 `overall_match_rate >= 95%`
- 但当前仍不能勾 `6.4`，因为 citation / 主章节骨架 / 正文密度 / extra generated pages 仍是主要尾差

## 差距收敛过程

- `storybook`
  - 结构对齐已从缺页/折叠问题收敛到 `missing=0 / collapsed=0`
  - citation 密度已从此前阶段性的 `12.24 -> 15.24 -> 24` 提升到当前 `46.71`
  - `extra generated pages` 已从更早的 `314` 回落，经 `270` 收敛到当前 `210`
  - broad docs corpus 触发 raw source pruning 的方案曾把结构命中打回 `159/176`，已验证失败并撤回
  - `steady lifecycle (warm)` 已跑通，说明 storybook 已满足 `2.4`
- `dagger`
  - 结构对齐已收敛到 `missing=0 / collapsed=0`
  - citation 密度已从此前阶段性的 `33.03 -> 55.16` 提升到当前 `57.09`
  - `extra generated pages` 已从 `74 -> 73 -> 72` 继续回落
  - workflow 已能稳定完成 warm report，但仍保留 `provider_failed_pages = 3`
  - lifecycle 仍被长跑超时阻塞，`3.4` 还不能完成

## 当前剩余尾差

- `storybook`
  - `low-fidelity matched pages = 176`
  - `extra generated pages = 210`
  - `citation density = 46.71 / 77.72`
  - `diagram coverage = 59 / 176`
  - `main outline shortfall = 168`
- `dagger`
  - `low-fidelity matched pages = 65`
  - `extra generated pages = 72`
  - `citation density = 57.09 / 79.69`
  - `diagram coverage = 69 / 65`
  - `main outline shortfall = 63`
  - `provider_failed_pages = 3`

## 判断

- `6.3` 可以完成：最终专项报告、`_summary.md`、`_optimization-notes.md` 和本结论摘要都已落地，并明确记录了最终匹配率、三类差距收敛过程与剩余尾差
- `6.4` 仍不能完成：主短板已从结构命中切换到 citation / 正文密度 / 主章节骨架 / extra generated pages / provider error 尾差
