# topic-page-planner Specification

## Purpose
定义 Repo Wiki 在正式页面规划阶段发现、去重和稳定落盘专题页的约束。
## Requirements
### Requirement: planner 必须支持基于高信号主题的专题页发现
系统 MUST 在现有 `overview / architecture / module / workflow` 页面之外，支持基于高信号主题发现专题页。专题候选 MUST 至少覆盖三类现有来源：根级核心机制文件簇、模块内能力簇、以及由 detected processes 归纳出的流程主题；9.3 还 MUST 按仓库 archetype 扩展稳定专题页族，例如路由/入口链路页、配置与运行时页、协议/数据模型页、命令树页和部署流程页。专题页的发现 MUST 由 deterministic 规则和现有 graph/module/context 输入驱动，而不是让 LLM 直接决定页面集合。

#### Scenario: 根级核心文件簇生成专题页
- **WHEN** 仓库根级存在一组高信号源码文件，且它们通过依赖证据、graph hotspot 或命名模式共同指向同一核心机制
- **THEN** planner MUST 允许为该文件簇生成专题页
- **THEN** 该专题页不得要求这些文件先被提升为独立目录模块

#### Scenario: archetype 高频主题进入专题页集合
- **WHEN** 仓库被识别为 Web、CLI、Library/SDK、全栈应用或运维脚本等 archetype，且对应稳定事实已经存在
- **THEN** planner MUST 允许生成该 archetype 对应的高频专题页族
- **THEN** 专题页命名和父子关系 MUST 继续由稳定规则导出，而不是由模型自由命名

#### Scenario: 流程主题页消费 detected processes
- **WHEN** graph summary 中存在稳定 detected process，且这些流程并不适合仅由单个 workflow 总览页承载
- **THEN** planner MUST 允许生成流程主题页
- **THEN** 这些页面 MUST 以流程主题而不是文件目录命名

### Requirement: 专题页必须保持稳定页面身份与去重策略
系统 MUST 为专题页生成稳定的页面身份、父子关系和覆盖去重规则。专题页 identity MUST 基于稳定 `topic_kind + topic_key + parent_scope` 导出，而不是取决于本轮发现顺序。若某主题与现有模块页、兄弟专题页或 archetype 专题页高度重叠，planner MUST 优先合并或抑制重复页面，而不是复制同一组内容。

#### Scenario: 同一主题重复规划时页面身份稳定
- **WHEN** 同一仓库在未发生主题边界变化的情况下重复执行 `init` 或 `rebuild`
- **THEN** 同一专题页 MUST 保持稳定 `page_id` 和相对路径
- **THEN** 该页面不得因为候选排序变化而改变父页面

#### Scenario: 主题与模块页或兄弟专题页重叠时抑制重复页面
- **WHEN** 某个专题候选与现有模块页或兄弟专题页在 evidence、关键源码和 section 主题上高度重叠
- **THEN** planner MUST 允许抑制该专题页或把其内容合并回父页面
- **THEN** 系统不得同时生成两个近乎重复的正式页面

