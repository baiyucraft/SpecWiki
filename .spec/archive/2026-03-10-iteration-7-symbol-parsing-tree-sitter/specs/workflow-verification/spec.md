## MODIFIED Requirements

### Requirement: 系统必须提供端到端验证适配层到 Wiki 产物的主链路
系统 MUST 提供自动化测试，验证适配层调用 Rust core 后能够在目标仓库完成 Wiki 初始化、手工编辑同步、增量更新、强制重建并执行查询，且生成的 `.wiki/` 产物与返回结果符合预期。针对迭代 7 收口，验证 MUST 继续覆盖 page identity 稳定性（增删少量源文件后核心页面 `page_id` 不变）、steering 配置生效（忽略路径、模块提升/降级、合并阈值）、页面合并/拆分正确性、父子关系按模块树层级分配、扩展 section 模板的内容密度，并新增覆盖多语言 symbol parsing 质量、`symbols` / `symbols_fts` 写盘、一致性的增量重解析、symbol BM25 query 和解析失败隔离。验证脚本还 MUST 使用与当前源码一致的 release binary，并允许对 `init / update / rebuild` 这类重 workflow 使用更长超时，避免把大型 monorepo 的正常初始化误判为失败。每轮与迭代 7 相关的 tasks 设计、实现或测试时，还 MUST 对 `.wiki/06-设计文档/00-总体设计.md § 测试项目集` 的完整项目集执行 `init` 分析；如果目标仓库存在 reference，则必须对照 `.wiki/*.md` 与 `wiki.metadata.json`。项目集分析报告 MUST 按项目逐个输出，而不是只给总表或总括结论。

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

#### Scenario: 多语言 symbol parsing 写盘
- **WHEN** 测试在包含 Rust、Go、Python、Java、JavaScript/TypeScript 等代表性语言源码的临时仓库中执行 `init`
- **THEN** 测试必须观察到 `symbols` 表包含函数、类、方法、结构体、接口或枚举等定义类符号
- **THEN** 测试必须观察到 `symbols_fts` 可按 symbol 名称或文件路径命中这些符号

#### Scenario: symbol BM25 query 返回结构化符号结果
- **WHEN** 测试执行 `query` 并使用某个已知符号名作为检索词
- **THEN** 测试必须观察到返回结果中存在 `matched_symbols`
- **THEN** 测试必须观察到该 symbol 命中同时回填相关源码或页面上下文

#### Scenario: update 增量重解析受影响源码文件
- **WHEN** 测试在 `init` 后修改某个已存在源码文件中的定义并执行 `update`
- **THEN** 测试必须观察到该文件对应的 symbol rows 被刷新
- **THEN** 测试必须观察到未受影响文件的 symbol rows 不被无谓重写

#### Scenario: 单文件解析失败隔离
- **WHEN** 测试在临时仓库中引入一个存在语法错误的源码文件并执行 `init` 或 `update`
- **THEN** 测试必须观察到 workflow 仍然完成
- **THEN** 测试必须观察到该坏文件不会留下陈旧 symbol rows

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

#### Scenario: 分阶段生命周期脚本验证 symbol snapshot 一致性
- **WHEN** 验证脚本使用拆分后的 lifecycle phase 入口分别执行 `bootstrap`、`steady`、`mutation` 与 `rebuild`
- **THEN** `bootstrap` 必须验证 `.wiki/`、managed marker 和 `wiki.metadata.json` 正常生成
- **THEN** `steady`、`mutation` 与 `rebuild` 必须验证 `symbols` 表可读且 symbol count 在对应阶段保持预期稳定
- **THEN** 对存在代表性 symbol 且 `.wiki` 可直接查询的项目，`steady`、`mutation` 与 `rebuild` 必须验证 exact symbol hit 仍然存在
- **THEN** 对无 symbols 项目或 real-repo `.wiki` 不位于 `repoRoot` 的项目，脚本 MAY 显式跳过 symbol query，但 MUST 继续验证 workflow 返回成功且 runtime state 保持 `fresh`

#### Scenario: 测试项目集全量 init 分析
- **WHEN** 迭代 7 的 tasks 设计或测试阶段
- **THEN** 必须对 `.wiki/06-设计文档/00-总体设计.md § 测试项目集` 的完整项目集执行 `init` 分析
- **THEN** 必须重点关注多语言 symbol 提取质量、symbol query 命中、增量重解析结果以及既有页面拓扑不变量
- **THEN** 如果存在 reference，必须对照 `.wiki/*.md` 与 `wiki.metadata.json`
- **THEN** 必须输出 `test-project-analysis.md`
- **THEN** 报告必须参考迭代 5 与迭代 6 的项目集报告形式，按项目逐个分析、逐个输出，至少覆盖每个项目的 `init` 结果、页面/模块规模、symbol 提取表现、symbol query 命中表现，以及与 reference 的差异说明或“无 reference”说明

#### Scenario: 大型 monorepo 的验证脚本不因工具层超时而误判失败
- **WHEN** 验证脚本对 `storybook` 这类大型 monorepo 执行 `init`、`update` 或 `rebuild`
- **THEN** 验证脚本必须确保使用的是当前源码对应的 release binary
- **THEN** 验证脚本必须为这类重 workflow 提供足够超时
- **THEN** 系统不得因为验证脚本层的固定短超时把正常 workflow 误记为失败
