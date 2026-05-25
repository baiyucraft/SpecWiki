## MODIFIED Requirements

### Requirement: 系统必须提供端到端验证适配层到 Wiki 产物的主链路
系统 MUST 提供自动化测试，验证适配层调用 Rust core 后能够在目标仓库完成 Wiki 初始化、手工编辑同步、增量更新、强制重建并执行查询，且生成的 `.wiki/` 产物与返回结果符合预期。针对迭代 6 收口，验证 MUST 覆盖 page identity 稳定性（增删少量源文件后核心页面 page_id 不变）、steering 配置生效（忽略路径、模块提升/降级、合并阈值）、页面合并/拆分正确性、父子关系按模块树层级分配、扩展 section 模板的内容密度，以及测试项目集 `init` 分析。每轮与迭代 6 相关的 tasks 设计、实现或测试时，还 MUST 对 `DESIGN.md § 测试项目集` 的完整项目集执行 `init` 分析；如果目标仓库存在 reference，则必须对照 `.wiki/*.md` 与 `wiki.metadata.json`。

#### Scenario: page identity 稳定性验证
- **WHEN** 测试在临时仓库中执行 `init`，然后增加一个源文件并执行 `update`
- **THEN** 测试必须观察到核心模块页面的 `page_id` 保持不变
- **THEN** 测试必须观察到 overview 和 architecture 页面的 `page_id` 保持不变

#### Scenario: steering 配置忽略路径生效
- **WHEN** 测试在临时仓库中创建 `.wiki/wiki.steering.yaml` 并声明全局忽略路径和按语言忽略路径，然后执行 `init`
- **THEN** 测试必须观察到全局忽略路径下的文件不出现在 `wiki.metadata.json` 的 source_files 中
- **THEN** 测试必须观察到当仓库主语言匹配时，按语言忽略路径下的文件也不出现在 source_files 中
- **THEN** 测试必须观察到被忽略路径下的模块不生成独立页面

#### Scenario: steering 配置模块提升/降级生效
- **WHEN** 测试在临时仓库中通过 steering 配置 promote 一个低权重模块
- **THEN** 测试必须观察到该模块生成了独立页面
- **WHEN** 测试通过 steering 配置 demote 一个高权重模块
- **THEN** 测试必须观察到该模块被合并到父模块页面

#### Scenario: 小模块合并验证
- **WHEN** 测试在临时仓库中创建一个只有 1-2 个源文件的模块
- **THEN** 测试必须观察到该模块不生成独立页面
- **THEN** 测试必须观察到该模块的内容出现在父模块页面的子模块概述 section 中

#### Scenario: 父子关系按模块树层级分配
- **WHEN** 测试在临时仓库中创建嵌套模块结构并执行 `init`
- **THEN** 测试必须观察到嵌套模块页面的 `parent_id` 指向父模块页面
- **THEN** 测试必须观察到顶层模块页面的 `parent_id` 指向 overview 页面

#### Scenario: 扩展 section 模板内容密度
- **WHEN** 测试在临时仓库中执行 `init`
- **THEN** 测试必须观察到 overview 页面包含技术栈 section
- **THEN** 测试必须观察到 architecture 页面包含模块结构的层级化文本表达

#### Scenario: 初始化后页面包含 managed marker
- **WHEN** 端到端或集成测试在临时仓库中执行 `init`
- **THEN** 测试必须观察到 `.wiki/` 目录和 `wiki.metadata.json` 已生成
- **THEN** 测试必须观察到页面中的 runtime 托管区段带有 managed marker

#### Scenario: 手工区段在 update 后仍被保留
- **WHEN** 测试在 `sync` 之后修改相关源码并执行 `update`
- **THEN** 测试必须观察到 `updated_pages` 只包含受影响页面
- **THEN** 测试必须观察到同页 user section 在最终 Markdown 中仍然存在

#### Scenario: rebuild 保留同页 user section
- **WHEN** 测试在同步手工区段后执行 `rebuild`
- **THEN** 测试必须观察到系统重新生成 managed sections
- **THEN** 测试必须观察到同页 user section 在最终 Markdown 中仍然存在

#### Scenario: 测试项目集全量 init 分析
- **WHEN** 迭代 6 的 tasks 设计或测试阶段
- **THEN** 必须对 `DESIGN.md § 测试项目集` 的完整项目集执行 `init` 分析
- **THEN** 必须重点关注页面拓扑稳定性、合并策略效果和父子关系正确性
- **THEN** 如果存在 reference，必须对照 `.wiki/*.md` 与 `wiki.metadata.json`
