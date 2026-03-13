# content-family-planner Specification

## Purpose
TBD - created by archiving change iteration-9-4-family-planner-and-research-first-composition. Update Purpose after archive.
## Requirements
### Requirement: planner 必须支持稳定的内容家族发现与索引页/子页规划
系统 MUST 在现有 `overview / architecture / workflow / module / topic` 之外，支持稳定的 `family` 页面规划。family 代表产品知识域或公共能力域，而不是简单文件簇；第一版至少 MUST 支持 `concept / addon / framework / builder / api / config / ops / troubleshooting / theme` 这些 family 类型。family 规划 MUST 基于 deterministic 规则，从 docs anchors、public API surface、config surface、manifest/workspace 元数据、入口文件和已有模块/专题结果中发现候选，而不是让 LLM 直接决定页面集合。

#### Scenario: docs-heavy 或 platform 仓库生成 family index 与 child 页面
- **WHEN** 仓库存在稳定的 docs/API/config/plugin/framework 信号，且这些信号可被归并到某个 family 类型
- **THEN** planner MUST 生成对应的 `family-index` 页面
- **THEN** planner MUST 为高信号子主题生成 `family-child` 页面
- **THEN** family 页面 MUST 进入正式页面集合，而不是只作为临时 research 对象

#### Scenario: family 规划保持 deterministic
- **WHEN** 同一仓库在 family 边界未变化的情况下重复执行 `init` 或 `rebuild`
- **THEN** 相同 family index 和 child 页面 MUST 保持稳定 `page_id`、相对路径和父子关系
- **THEN** 页面集合不得因为模型输出差异而抖动

### Requirement: family 页面必须具备稳定的收编、去重与父子关系规则
系统 MUST 明确 family 页、topic 页和 module 页之间的收编与去重规则。若某个 family child 与现有 topic/module 页在主题、关键来源和 section 结构上高度重叠，planner MUST 优先收编或抑制重复页面，而不是同时保留多个近似页面。family index 页 MUST 作为其 child 页的直接父页，并允许挂到 overview、architecture 或特定 module/family 父页之下。

#### Scenario: family child 与 module/topic 页重叠时抑制重复页面
- **WHEN** 某个 family child 候选与现有 module/topic 页在关键来源和页面定位上高度重叠
- **THEN** planner MUST 抑制其中一个页面或把其内容收编到更合适的父页
- **THEN** 系统不得同时生成两个近乎重复的正式页面

#### Scenario: family index 与 child 层级稳定
- **WHEN** 某个 family 被正式规划
- **THEN** 该 family 的 index 页 MUST 成为对应 child 页的稳定父页
- **THEN** 子页不得直接挂到 overview 或 architecture，除非 planner 明确判定该 family 不需要独立 index 页

### Requirement: planner 必须支持 `family-leaf-doc` 叶子文档单元
系统 MUST 在 `family-child` 之下支持稳定的 `family-leaf-doc` 叶子文档页，用于承接 docs-heavy / platform archetype 中继续细分的 API、config、addon、framework、troubleshooting 等 reference 子页。`family-leaf-doc` 必须由 deterministic 规则规划，不得由样本仓库名直接驱动。

#### Scenario: family child 被 docs/API/config/type 信号继续拆分
- **WHEN** 某个 `family-child` 下命中了多个稳定的 docs anchors、public API surface、config surface 或 type surface 子簇
- **THEN** planner MUST 继续生成对应的 `family-leaf-doc` 页面
- **THEN** 这些 leaf 页面 MUST 挂在对应 `family-child` 之下，而不是直接挂到 `family-index`

#### Scenario: `family-leaf-doc` 规划保持通用
- **WHEN** 当前仓库只是 `storybook` 这一类 docs-heavy/platform 仓库中的一个样本
- **THEN** planner MUST 依赖 archetype signal、family profile 与 surface clustering 规划 leaf 页面
- **THEN** 系统不得引入基于仓库名、reference 标题或固定目录路径的硬编码分支

