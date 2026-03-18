# Reference 对比后的优化收敛

## 当前收敛

- ready 样本：storybook
- runtime_incomplete 样本：dagger

## 高频观察

- storybook：overall=98.30% / reuse_overage=129 / median_skeleton=0.29 / median_key_source=0.05
- dagger：runtime_incomplete，需先解决 已有 knowledge/research 数据，但 assemble 尚未写出 metadata 与 Markdown

## 基线资格说明

- 当前产物通过 `--skip-init` 从已有 runtime 读取；它本身只承担现状定位与 warm 稳定性对照，不单独替代 fresh init。若上游 runtime 已由同轮 fresh init 成功生成，则可与那批 fresh 产物一起构成 9.7-9.9 的验收基线。

## 下一步建议

- 先解决 runtime_incomplete，避免把 assemble 缺口误判成页面质量问题。
- 优先回收 many-to-one reuse，父页只能消费 child digest，不应继续吞并多个 reference 主题。
- 继续收敛 docs-backed 页面骨架，保证 section plan 真正落到最终 Markdown。
- 继续提升 citation / key source grounding，避免只命中结构不命中关键文件。
