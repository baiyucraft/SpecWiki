# content-family-planner Specification

## Purpose
TBD - created by archiving change iteration-9-4-family-planner-and-research-first-composition. Update Purpose after archive.
## Requirements
### Requirement: planner 必须支持稳定的内容家族发现与索引页/子页规划
系统 MUST 把 `family` 降级为 KnowledgeUnit decomposition 的一种信号来源，而不是继续作为独立页面体系主抽象。family 代表产品知识域或公共能力域，但正式页面集合 MUST 先映射到 `KnowledgeDomain / KnowledgeUnit`，再决定是否呈现为 index/child 结构。family 候选仍 MUST 基于 deterministic 规则，从 docs anchors、public API surface、config surface、manifest/workspace 元数据、入口文件和已有模块/专题结果中发现，而不是让 LLM 直接决定页面集合。

#### Scenario: docs-heavy 或 platform 仓库生成 family 风格页面时仍先映射到 KnowledgeUnit
- **WHEN** 仓库存在稳定的 docs/API/config/plugin/framework 信号，且这些信号可被归并到某个 family 类型
- **THEN** planner MUST 先为其生成对应的 `KnowledgeDomain / KnowledgeUnit`
- **THEN** 只有在该 domain 需要稳定 index/child 结构时，系统才 MAY 输出 family 风格目录页
- **THEN** family 不得绕过 KnowledgeUnit 主线单独决定正式页面集合

#### Scenario: family 信号保持 deterministic
- **WHEN** 同一仓库在 family 相关 signal 未变化的情况下重复执行 `init` 或 `rebuild`
- **THEN** 相同 family 来源映射出的 KnowledgeUnit id、相对路径和父子关系 MUST 保持稳定
- **THEN** 页面集合不得因为模型输出差异而抖动

### Requirement: family 页面必须具备稳定的收编、去重与父子关系规则
系统 MUST 明确 family 信号生成的页面与其它 KnowledgeUnit 之间的收编与去重规则。若某个 family child 候选与现有 API / config / module / guide unit 在主题、关键来源和 section 结构上高度重叠，planner MUST 优先收编或抑制重复页面，而不是同时保留多个近似页面。family 风格 index 页若存在，MUST 只是对应 domain/index unit 的页面投影，而不是一条独立于 KnowledgeTree 的父子链。

#### Scenario: family 信号与其它 unit 重叠时抑制重复页面
- **WHEN** 某个 family child 候选与现有 API、config、module 或 guide 类 KnowledgeUnit 在关键来源和页面定位上高度重叠
- **THEN** planner MUST 抑制其中一个页面或把其内容收编到更合适的 unit
- **THEN** 系统不得同时生成两个近乎重复的正式页面

#### Scenario: family 风格 index 仍服从 KnowledgeTree
- **WHEN** 某个 family 信号被正式规划为 index/child 结构
- **THEN** 该结构 MUST 作为 KnowledgeTree 中现有 domain/index/child 关系的页面投影
- **THEN** family 风格页面不得形成一套脱离 KnowledgeTree 的额外父子链

