# storybook init 剩余缺口分析

## 当前结论

- 当前 `storybook` 的 `init fidelity` 仍然 **未达到 9.8 验收目标**。
- 最新专项指标：
  - `overall_match_rate=98.30%`
  - `reuse_overage=106`
  - `median_skeleton_fidelity=0.50`
  - `median_key_source_coverage=0.2353`
- 相比本轮开始前的基线：
  - `reuse_overage: 107 -> 106`
  - `median_key_source_coverage: 0.2308 -> 0.2353`
  - `median_skeleton_fidelity: 0.50 -> 0.50`
- 说明这轮 `research` 收敛有小幅净收益，但还没有把主矛盾打穿。

## 本轮已确认的改善

- `research_engine.rs` 已经从平面打分，收敛到 `role/family + selection policy + fallback relief + unit topic focus`。
- `topic focus` 没有写 `storybook` 专有分支，只使用：
  - `KnowledgeUnit.title`
  - `doc reference citations`
  - `scope.source_ids`
  - `planner grounded paths`
  - `docs anchors`
- `theme-style-focus` 已经把“颜色和字体系统”从 `nextjs preset / router` 这一类明显漂移的文件，拉回到 `addons/themes + core/theming` 一带。
- `type-focus` 已经被收紧为“必须有真实 docs 引文才启用”，避免仅凭标题把整个 API 区域错误重排。

## 仍未达标的核心缺口

### 1. planner 层：某些 unit 的原始输入仍然过泛

- `Types API` 的 unit scope 原始上仍然主要来自一批 `public-types.ts`。
- `颜色和字体系统` 的 unit scope 原始上仍然主要来自：
  - `docs/configure/user-interface/theming.mdx`
  - `docs/configure/styling-and-css.mdx`
  - 少量 config/mock 路径
- 这意味着 `research` 虽然可以在候选内做 rerank，但无法凭空得到 planner 没有送进来的主题主干。

### 2. research 层：profile 对了，但 unit 主题主干还不够强

- `Types API` 仍然没有稳定命中 reference 真正关心的类型主干：
  - `code/core/src/csf/story.ts`
  - `code/core/src/manager-api/modules/stories.ts`
  - `code/core/src/preview-api/modules/preview-web/PreviewWeb.tsx`
  - `code/core/src/preview-api/modules/store/args.test.ts`
  - `code/core/src/preview-api/modules/store/inferArgTypes.test.ts`
  - `docs/api/arg-types.mdx`
- 当前它仍然偏向：
  - `code/renderers/html/src/public-types.ts`
  - `code/lib/core-webpack/src/types.ts`
  - `code/core/src/actions/runtime/index.ts`
  - `code/core/src/router/types.ts`
- 这说明 `Types API` 的候选池里仍是“泛 type 合同文件”占主导，`topic focus` 目前只够抑制最明显的漂移，还不够把 shared type spine 抬到前排。

- `颜色和字体系统` 已经从“明显错源”收回到了 `addons/themes + core/theming`，但 reference 这页当前仍主要落在：
  - `code/addons/a11y/**`
  - `code/__mocks__/styleMock.js`
  - `code/__mocks__/fileMock.js`
- 当前生成页虽然更接近“标题语义”，但与 reference 的 key-source 交集仍很低，`key_source_coverage=0.05`。
- 这页现在暴露的不是单纯“选错 profile”，而是：
  - 参考页本身更偏 `a11y / 可访问性颜色验证`
  - 当前 core 更偏 `themes / theming`
  - 两者之间缺少稳定、可通用的 unit topic 归并规则

### 3. compose 层：骨架和解释密度仍然偏弱

- `median_skeleton_fidelity` 仍停在 `0.50`，没有继续上升。
- 这说明即使 key source 有局部改善，最终 Markdown 仍然大量停留在 reference 式骨架的“壳”上，还没有形成更强的 section-level 解释层。
- 典型表现：
  - reference 更细的主题仍被合并在同一页
  - 章节结构还偏粗
  - 解释性正文密度不足

## 代表性页面判断

### Types API

- 当前状态：比中途错误版本好，但基本回到“泛 types 文件聚集”的旧问题。
- 现在的问题不是“完全选错 profile”，而是：
  - `ApiSurface` 候选里真正的 shared type spine 权重还不够高
  - `public-types.ts / types.ts / typings.d.ts` 这类泛 contract 文件仍然过强
- 对应主链断点：
  - 主断点在 `research`
  - 次断点在 `planner` 的 unit topic grounding 还不够强

### 颜色和字体系统

- 当前状态：比旧版本明显更合理，已经不再被 `nextjs preset / router` 主导。
- 但它还没有达到 reference 的主题语义。
- 当前更像“主题系统 / theming 实现”，reference 更像“颜色与字体的 a11y 验证体系”。
- 对应主链断点：
  - `research` 已经在往正确方向收
  - 但 `planner -> research` 之间还缺一层更强的 unit topic grounding，去决定这页到底属于 `themes`、`theming` 还是 `a11y`

## 为什么还不能算通过

- `overall=98.30%` 很高，但它主要反映“有页可对齐”，不代表页面质量已经过线。
- 当前真正卡住验收的是两项：
  - `reuse_overage=106` 仍然过高
  - `median_key_source_coverage=0.2353` 仍然明显偏低
- 换句话说：
  - 页面集合已经大体成型
  - 但单页主题边界、关键文件命中和正文 grounding 还不够

## 下一步建议

### 优先级 1：加强 planner -> research 的 unit topic grounding

- 不再继续堆更多路径打分补丁。
- 优先让 `planner_signal_bundles.matched_paths / matched_keywords` 更明确地进入 `research` 的 unit-level source selection。
- 目标不是增加更多候选，而是让 unit 自己携带“这一页真正想讲哪一簇源码”的更强约束。

### 优先级 2：压制泛 contract 文件的主导地位

- `public-types.ts / types.ts / typings.d.ts / preset.ts / preview.tsx` 这类文件仍应保留，但只应该作为辅源，不该继续占主导。
- 需要更强的“泛 contract saturation penalty”，而不是继续加局部关键词 boost。

### 优先级 3：把 key-source 改善传到 compose 正文

- 现在 key source 即使局部变好，也还没有显著带动 skeleton 与正文解释层。
- 下一步需要检查 `section_grounding_refs -> compose contract -> final markdown` 这段是否把新的 key source 真正消费进正文，而不是只停留在 evidence block。

## 一句话总结

- 当前 `storybook init` 已经证明：这轮 `research` 收敛方向是对的，但还只是“小幅修正”，还没把 9.8 的主矛盾打穿。
- 真正还欠缺的是：**unit 级 topic grounding 仍然不够 source-grounded，导致 planner 输入过泛、research 选源不够聚焦、compose 正文也就无法稳定贴住 reference 的关键实现主干。**
