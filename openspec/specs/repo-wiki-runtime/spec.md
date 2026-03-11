# repo-wiki-runtime Specification

## Purpose
定义 Repo Wiki 页面运行时、页面状态、managed section 与 steering 配置消费的持久化约束。

## Requirements

### Requirement: 正式 Wiki 页面必须稳定映射到模块结构
系统 MUST 让正式 Wiki 页面与模块结构保持稳定映射，以便后续 `query`、`sync`、`update` 和 `rebuild` 可以围绕层级化页面工作。模块页的关键源码表达 MUST 优先选择入口、依赖证据和核心实现文件，并抑制日志、锁文件、纯文档和低价值配置噪声。模块页的页面标识或路径必须由模块 root_path 稳定导出，并记录基础 provenance。页面状态还 MUST 记录稳定 section 标识、section 级 generated/observed hash，以及 user section 的锚点信息，使 runtime 能在保留人工内容的同时局部重组页面。页面的父子关系 MUST 按模块树层级分配。被合并的小模块 MUST 作为父模块页面的子模块概述 section 出现，而不是生成独立页面。steering 配置 MUST 作为 runtime 的一部分被读取和消费。

#### Scenario: 写入模块页
- **WHEN** 系统为模块生成正式 Wiki 页面
- **THEN** 页面状态中必须记录对应模块标识
- **THEN** 页面来源中必须记录该页面依赖的源码集合
- **THEN** 同一模块在重复生成时必须使用稳定页面标识或路径
- **THEN** 页面标识必须锚定到模块 root_path，而不是模块名或发现顺序

#### Scenario: 写入递归模块页
- **WHEN** 模块页对应多级嵌套模块
- **THEN** 系统必须为该页面写入稳定的父页面关系，父页面 MUST 是模块树中父模块对应的页面
- **THEN** 系统必须避免因 Windows 非法路径字符或同名模块导致落盘冲突

#### Scenario: 导出高信号关键源码
- **WHEN** 系统生成模块页摘要和 metadata 页面来源
- **THEN** 关键源码列表必须优先包含入口、依赖证据或核心实现文件
- **THEN** 关键源码列表不得被日志、锁文件、纯文档或低价值配置主导

#### Scenario: 重复生成页面时 section 身份稳定
- **WHEN** 同一页面在未发生对应 section 输入变化的情况下被重复生成
- **THEN** 页面中的 section 标识必须保持稳定
- **THEN** 未变化 section 的 generated hash 和 provenance 映射必须保持可复用

#### Scenario: 页面写盘时包含 managed section marker
- **WHEN** 系统在 `init`、`update` 或 `rebuild` 中写出正式 Wiki 页面
- **THEN** 所有 runtime 托管 section 必须以稳定 marker 包裹
- **THEN** 页面文件仍必须保持为正常 Markdown 文档

#### Scenario: 被合并模块作为父模块页面的子模块概述
- **WHEN** 某模块因合并策略不生成独立页面
- **THEN** 该模块的内容 MUST 作为父模块页面的"子模块概述" managed section 出现
- **THEN** 该 section MUST 包含被合并模块的名称、角色和关键源码信息

#### Scenario: steering 配置作为 runtime 的一部分
- **WHEN** `.wiki/wiki.steering.yaml` 存在
- **THEN** 系统 MUST 在 `init`、`update`、`rebuild` 时读取该配置
- **THEN** 配置 MUST 影响 planner 的页面规划决策

#### Scenario: 扩展的 section 模板提升内容密度
- **WHEN** 系统生成 overview 或 architecture 页面
- **THEN** overview 页面 MUST 包含技术栈和入口与构建 section
- **THEN** architecture 页面 MUST 包含模块树的文本化层级表达
- **THEN** module 页面 MUST 包含关键源码列表和依赖关系 section

### Requirement: 正式 Wiki 页面必须支持可回退的 LLM 增强 section 与图内容
系统 MUST 允许 overview、architecture、module 和 workflow 页面在现有 managed sections 中承载 LLM 增强正文和 Mermaid 图内容。增强内容 MUST 复用现有 `page_id`、`section_id` 和 managed marker，而不是创建新的运行时层或脱离页面状态表的 sidecar 文件。增强内容不可用时，系统 MUST 回退到 deterministic section body，并继续写出合法 Markdown 页面。

#### Scenario: LLM 增强不会改变 section 身份
- **WHEN** 同一页面在启用 LLM 增强的情况下被重复生成，且对应 section 的输入哈希未变化
- **THEN** 页面中的 `section_id` 和 managed marker MUST 保持稳定
- **THEN** 该 section 允许更新正文，但不得更换 section 身份

#### Scenario: 增强内容失效时页面仍写出合法 Markdown
- **WHEN** 某个页面的增强正文或 Mermaid 图未通过校验，或当前运行环境不提供增强结果
- **THEN** 系统 MUST 写出 deterministic 的 section 正文
- **THEN** 页面文件 MUST 仍是包含 managed marker 的正常 Markdown 文档

### Requirement: runtime 必须持久化专题页与 evidence block 的稳定身份
系统 MUST 在现有页面 runtime 中持久化专题页和 evidence block 的稳定身份。专题页 MUST 与现有 overview / architecture / module / workflow 页面共用同一套页面状态、managed section 和缓存 contract；evidence block MUST 复用现有 page/section runtime，而不是写入新的 sidecar 层。

#### Scenario: 专题页进入正式 runtime
- **WHEN** planner 生成专题页
- **THEN** runtime MUST 为其写入正式页面状态、input hash 和 managed sections
- **THEN** 该页面 MUST 与其他正式页面一样进入 `.wiki/*.md` 和状态库

#### Scenario: evidence block 复用现有 runtime contract
- **WHEN** 页面 section 中存在 evidence block
- **THEN** 这些 block MUST 继续受现有 managed section contract 管理
- **THEN** 系统不得为 evidence 单独引入新的正式 runtime 目录

### Requirement: debug trace 模式必须通过显式配置开启且不得污染正式协议
系统 MUST 提供可选的 debug trace 模式，并允许通过启动参数或 steering 配置显式开启。该模式写出的调试信息 MUST 与现有 stdout JSON IPC 正式协议隔离，不能改变 `result/error/progress` 的编码 contract。

#### Scenario: 启动参数开启 debug trace
- **WHEN** 宿主以启动参数显式传入 debug trace 目录或开启标记
- **THEN** 系统 MUST 在指定目录写出调试 trace
- **THEN** stdout 上的正式 JSON 协议 MUST 保持不变

#### Scenario: steering 开启 debug trace
- **WHEN** repo 根配置显式开启 debug trace
- **THEN** `init/update/rebuild` MUST 为本次 workflow 写出调试 trace
- **THEN** debug trace 关闭时系统不得额外写出调试产物
