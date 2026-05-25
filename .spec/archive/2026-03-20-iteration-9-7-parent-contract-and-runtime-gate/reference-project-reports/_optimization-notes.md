# Reference 对比后的优化收敛

## 当前收敛

- ready 样本：storybook、dagger
- runtime_incomplete 样本：无

## 高频观察

- storybook：overall=92.05% / reuse_overage=123 / median_skeleton=0.13 / median_key_source=0.03
- dagger：overall=95.38% / reuse_overage=25 / median_skeleton=0.08 / median_key_source=0.07

## 基线资格说明

- 当前产物通过 `--skip-init` 从已有 runtime 读取；它本身只承担现状定位与 warm 稳定性对照，不单独替代 fresh init。若上游 runtime 已由同轮 fresh init 成功生成，则可与那批 fresh 产物一起构成 9.7-9.9 的验收基线。

## 下一步建议

- 优先回收 many-to-one reuse，父页只能消费 child digest，不应继续吞并多个 reference 主题。
- 继续收敛 docs-backed 页面骨架，保证 section plan 真正落到最终 Markdown。
- 继续提升 citation / key source grounding，避免只命中结构不命中关键文件。
