# Reference 对比后的优化收敛

## 当前差距不是 prompt 文风问题

10 个 reference 项目的逐项目报告都指向同一个结论：当前生成结果和 reference 的主要差距，不在于 LLM 句子写得不够“像文档”，而在于 core 的页面规划、来源落页和图表达还不够细。

共性差距：

- planner 粒度明显粗于 reference。大量 reference 页面被折叠到少量 overview / architecture / module 页里。
- 页面缺少源码引用/出处层。当前正文能提到文件名，但还没有把关键事实稳定落到“来自哪个文件/模块”的引用结构里。
- Mermaid 表达不足。虽然 graph facts 已经进入 state/context，但页面里很少真正展开成受控图块。
- 单页章节过粗。很多不同主题被挤在同一页，导致正文密度上去后仍然不像 reference 的专题页。
- 大仓库缺少“主题页”而只有“目录页”。`axum`、`storybook`、`dagger` 这类项目尤其明显。

## 下一轮最值钱的优化顺序

### 1. 先改 page planner，不要继续只调 prompt

最优先要做的是把“目录页 planner”升级成“目录页 + 主题页 planner”。

建议优先增加三类可规划专题页：

- 核心机制页：例如 `chi` 的 `context / tree / chain / pattern`，`axum` 的 `extractor / response / router / middleware`
- API/能力簇页：把同一模块下高频协作文件按能力簇拆页，而不是全部折叠到一个模块页
- 运行/流程页：把已有 `processes`、入口链和部署线索单独提升成 workflow 子页

原因：

- 当前 `chi` 只有 5 页，却要映射 65 个 reference 页面，说明不是“写得不够长”，而是页面主题没有被拆出来。
- `axum`、`restaurant-app`、`storybook` 的缺失页，也主要是专题页缺位，而不是 facts 不够。

### 2. 给页面补来源层，而不是只补正文层

reference 普遍有更强的“出处感”。下一轮应把已有 facts 的来源一起落页：

- section 内为关键结论补 `来源文件` / `关键源码` 小块
- 让 context builder 输出更稳定的 `evidence files` 列表，而不是只给 LLM 一堆 facts 字符串
- overview / architecture / module 页都至少能显示 3 到 8 个关键文件出处

这件事比继续增加段落更值钱，因为它同时提升可读性、可信度和 Agent 可消费性。

### 3. Mermaid 不应交给 LLM 自由发挥，而应更多由 facts 驱动

当前图少，不是因为 LLM 不会画，而是 planner/context 没把“该画什么图”明确喂出来。

建议下一轮优先支持三类 deterministic-first 图输入：

- 模块依赖图
- 父子模块/目录结构图
- 请求或执行流程图

LLM 只负责把图节点和说明组织成更可读的 block，不负责凭空发明图结构。

### 4. 根目录核心源码要能晋升成页面主题

多份报告都显示根级核心源码被低估：

- `chi` 的 `context.go / mux.go / tree.go / chain.go`
- `axum` 的 `lib.rs / extract/* / response/* / routing/*`

当前 module tree 更擅长目录模块，不擅长“根目录核心文件簇”。下一轮要允许这类高信号文件簇晋升为专题页或虚拟模块页。

## 项目类型上的优先策略

不同项目的缺口重点不同，可以按项目类型分三条线推进：

- 框架型仓库：`chi`、`axum`、`cobra`、`zustand`
  重点做 API/机制专题页和根级核心文件簇晋升
- 工具链/大型工程仓库：`dagger`、`storybook`
  重点做子系统级 planner 和 process/workflow 主题页
- 应用型仓库：`restaurant-app`、`aLocal`
  重点做来源层、流程图和跨模块协作页

## 结论

下一轮优化的核心不是“把 LLM 再写得更像 reference”，而是：

1. 让 planner 产出更细的页面主题
2. 让 context 带着来源证据进入页面
3. 让图表达更多地由现有 facts 驱动

如果这三件事不先做，单纯继续调 prompt，只会把当前几页写得更长，不会真正接近 reference 的页面结构。
