# workflow-verification Specification

## Purpose
定义 Repo Wiki workflow 的验证面，确保主链行为、graph 能力、progress 协议和热路径优化都能被自动化测试覆盖。
## Requirements
### Requirement: 系统必须提供端到端验证适配层到 Wiki 产物的主链路
系统 MUST 提供自动化测试，验证适配层调用 Rust core 后能够在目标仓库完成 Wiki 初始化、手工编辑同步、增量更新、强制重建并执行查询，且生成的 `.wiki/` 产物与返回结果符合预期。针对迭代 7 收口，验证 MUST 继续覆盖 page identity 稳定性（增删少量源文件后核心页面 `page_id` 不变）、steering 配置生效（忽略路径、模块提升/降级、合并阈值）、页面合并/拆分正确性、父子关系按模块树层级分配、扩展 section 模板的内容密度，并新增覆盖多语言 symbol parsing 质量、`symbols` / `symbols_fts` 写盘、一致性的增量重解析、symbol BM25 query 和解析失败隔离。验证脚本还 MUST 使用与当前源码一致的 release binary，并允许对 `init / update / rebuild` 这类重 workflow 使用更长超时，避免把大型 monorepo 的正常初始化误判为失败。每轮与迭代 7 相关的 tasks 设计、实现或测试时，还 MUST 对 `DESIGN.md § 测试项目集` 的完整项目集执行 `init` 分析；如果目标仓库存在 reference，则必须对照 `.wiki/*.md` 与 `wiki.metadata.json`。项目集分析报告 MUST 按项目逐个输出，而不是只给总表或总括结论。

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
- **THEN** 必须对 `DESIGN2.0.md § 测试项目集` 的完整项目集执行 `init` 分析
- **THEN** 必须重点关注多语言 symbol 提取质量、symbol query 命中、增量重解析结果以及既有页面拓扑不变量
- **THEN** 如果存在 reference，必须对照 `.wiki/*.md` 与 `wiki.metadata.json`
- **THEN** 必须输出 `test-project-analysis.md`
- **THEN** 报告必须参考迭代 5 与迭代 6 的项目集报告形式，按项目逐个分析、逐个输出，至少覆盖每个项目的 `init` 结果、页面/模块规模、symbol 提取表现、symbol query 命中表现，以及与 reference 的差异说明或“无 reference”说明

#### Scenario: 大型 monorepo 的验证脚本不因工具层超时而误判失败
- **WHEN** 验证脚本对 `storybook` 这类大型 monorepo 执行 `init`、`update` 或 `rebuild`
- **THEN** 验证脚本必须确保使用的是当前源码对应的 release binary
- **THEN** 验证脚本必须为这类重 workflow 提供足够超时
- **THEN** 系统不得因为验证脚本层的固定短超时把正常 workflow 误记为失败

### Requirement: 端到端验证必须覆盖 symbol resolution 与 graph analysis 生命周期
系统 MUST 提供自动化测试，验证适配层调用 Rust core 后能够在目标仓库完成 symbol resolution、graph analysis、增量 edge refresh 和 graph query，而不仅是 symbol definitions 的写盘。验证 MUST 覆盖 `edges`、`communities`、`community_members`、`processes`、`process_steps` 的生成、一致性与增量清理，并继续确保页面 runtime、不变页面和 user section 保持稳定。

#### Scenario: init 后 graph tables 生成真实数据
- **WHEN** 测试在包含可解析 import/call/heritage 关系的临时仓库中执行 `init`
- **THEN** 测试 MUST 观察到 `edges` 表存在真实业务 rows
- **THEN** 当图分析输入充分时，测试 MUST 观察到 `communities` 和 `processes` 相关表存在真实 rows

#### Scenario: update 后 graph rows 增量刷新
- **WHEN** 测试在 `init` 后修改或删除某个已跟踪源码文件并执行 `update`
- **THEN** 测试 MUST 观察到该文件对应的 symbol rows 和 edge rows 被刷新或清理
- **THEN** 测试 MUST 观察到 graph-derived 结果与新的 edges 保持一致

#### Scenario: query 返回 graph context
- **WHEN** 测试执行 `query` 并使用某个已知 symbol 名作为检索词
- **THEN** 测试 MUST 观察到返回结果除了 `matched_symbols` 外，还包含 graph relation 或 process/community 上下文
- **THEN** 测试 MUST 观察到 provenance 区分 BM25 与 graph 命中来源

### Requirement: 包装语言与项目集验证必须覆盖 Vue / Svelte graph 解析
系统 MUST 在 fixture、integration test 或项目集分析中覆盖 Vue / Svelte 单文件组件的 script wrapper 解析，验证定义类符号、raw relation captures 与 graph resolution 都能映射回原始组件文件。每轮与迭代 8 相关的 tasks 设计、实现或测试时，还 MUST 对 `DESIGN2.0.md § 测试项目集` 的完整项目集执行 `init` 分析，并在 `test-project-analysis.md` 中按项目输出 graph facts、graph query 命中表现与 reference 差异。

#### Scenario: Vue 或 Svelte wrapper 解析通过
- **WHEN** 测试在包含 Vue 或 Svelte 单文件组件的 fixture 或项目中执行 `init`
- **THEN** 测试 MUST 观察到这些组件的 definitions 或 graph relations 能映射回原始组件路径
- **THEN** 测试 MUST 观察到对应 diagnostics 不会把这些文件误记为 unsupported parser

#### Scenario: 测试项目集分析输出 graph 事实表现
- **WHEN** 迭代 8 的 tasks 设计或测试阶段执行完整项目集 `init` 分析
- **THEN** 报告 MUST 按项目逐个说明 edges / communities / processes 的生成表现
- **THEN** 报告 MUST 说明 graph query 的命中表现以及与 reference 的差异或“无 reference”状态

### Requirement: 端到端验证必须覆盖 progress 事件流
系统 MUST 提供自动化测试，验证 `wiki-core --json` 在执行 `init`、`update` 或 `rebuild` 时会输出 progress 事件流。验证 MUST 同时覆盖 core 直接调用与 CodeBuddy Agent 流式消费路径。

#### Scenario: core 直接调用时输出 progress 与最终 result
- **WHEN** 测试执行 `init`、`update` 或 `rebuild`
- **THEN** 测试 MUST 观察到至少一个 `progress` 事件和一个最终 `result` 或 `error` 事件
- **THEN** 测试 MUST 观察到最终终态事件可恢复出现有 `CoreResponse` 语义

#### Scenario: Agent 流式消费 progress 后仍返回最终结果
- **WHEN** 测试通过 CodeBuddy Agent 调用长流程 workflow
- **THEN** 测试 MUST 观察到 Agent 能消费 progress 事件而不报协议错误
- **THEN** 测试 MUST 观察到 Agent 最终仍返回与终态事件等价的最终结果

### Requirement: 验证必须覆盖 parser 热路径优化的一致性与局部读取行为
系统 MUST 提供自动化测试，验证 parse/query 复用与有限并行不会改变 symbol parsing 结果，并验证 `update` 在小范围 graph 变化下优先走局部 symbol/edge 读取，而不是固定回退为全量读取。

#### Scenario: parse 工件复用不改变 symbol 语义
- **WHEN** 测试对包含 definitions 与 raw relation captures 的代表性源码执行 symbol parsing
- **THEN** 测试 MUST 观察到优化后的 symbols、imports、calls 和 heritage 结果与基线语义一致
- **THEN** 测试 MUST 观察到诊断隔离行为保持不变

#### Scenario: 不同并行度下 symbol parsing 结果一致
- **WHEN** 测试分别以单工作单元和默认并行度执行同一批源码的 symbol parsing
- **THEN** 测试 MUST 观察到相同的 symbol IDs、raw captures 和诊断集合
- **THEN** 测试 MUST 观察到结果排序保持一致

#### Scenario: 小范围 update 优先走局部读取
- **WHEN** 测试在 `init` 后只修改少量源码文件并执行 `update`
- **THEN** 测试 MUST 观察到系统优先使用局部 symbol/edge 读取路径
- **THEN** 测试 MUST 观察到系统不会对该类小变更固定执行全量 `symbols / edges` 枚举

### Requirement: 验证必须覆盖 LLM 开关、缓存命中与桥接回退
系统 MUST 提供自动化测试，验证在 LLM 关闭、桥接不可用、缓存命中和增强开启四种典型路径下，`init`、`update` 和 `rebuild` 都能保持可回退性与页面稳定性。验证 MUST 同时覆盖 core 直接执行和 CodeBuddy Agent 桥接执行，不得只测 happy path。

#### Scenario: 未启用 LLM 时 deterministic 主链保持可用
- **WHEN** 测试在未开启 LLM 增强的情况下执行 `init`、`update` 或 `rebuild`
- **THEN** 测试 MUST 观察到 workflow 成功完成
- **THEN** 测试 MUST 观察到页面、状态库和 metadata 与 deterministic 预期保持一致

#### Scenario: 相同输入第二次执行命中 LLM 缓存
- **WHEN** 测试对同一仓库、同一模型和相同输入连续执行两次启用增强的 workflow
- **THEN** 第二次执行 MUST 复用 `llm_cache`
- **THEN** 测试 MUST 观察到真实 LLM 请求次数少于第一次执行

#### Scenario: Agent 桥接不可用时 core 自动回退
- **WHEN** 测试通过 CodeBuddy Agent 执行启用增强的长流程 workflow，但 Agent 对 `llm_request` 返回不可用
- **THEN** 测试 MUST 观察到 core 回退到 deterministic 内容并完成 workflow
- **THEN** 最终结果 MUST 仍然通过终态事件返回

#### Scenario: provider 直连优先于 Agent bridge
- **WHEN** 测试同时提供可用的 provider 直连配置和 Agent bridge
- **THEN** 测试 MUST 观察到 core 优先使用 provider 直连
- **THEN** Agent bridge 不得收到同一请求对应的 `llm_request`

#### Scenario: dev 配置文件可驱动本地 provider 验证
- **WHEN** repo 根存在 `wiki.dev.yaml` 并声明可用的 provider 直连配置
- **THEN** 测试 MUST 观察到 workflow 读取该文件并通过 provider 路径完成请求
- **THEN** 删除该文件后，workflow MUST 回退到共享 steering + Agent/fallback 行为

#### Scenario: provider 并行增强仍保持预算与顺序约束
- **WHEN** 测试为 provider 直连配置开启 `llm.parallel_requests > 1`
- **THEN** 测试 MUST 观察到同深度页面增强可以并行完成
- **THEN** 测试 MUST 同时验证父页晚于子页、总真实调用数不超过预算、缓存命中页不占用真实并行槽

### Requirement: 项目集验证必须覆盖增强后的页面信息密度与 graph 落地
每轮与迭代 9 相关的 tasks 设计、实现或测试时，系统 MUST 对 `DESIGN2.0.md § 测试项目集` 的完整项目集执行 `init` 分析，并在 `test-project-analysis.md` 中按项目输出增强后的页面信息密度、graph facts 是否进入页面正文、workflow/architecture 页面表现，以及与 reference 的差异。9.3 还 MUST 逐项目统计 `section_plan` 覆盖率、overview/architecture research 命中和精准 evidence 引用密度；验证可以按 deterministic baseline 与增强模式做对照，但不得只给总表结论。

#### Scenario: 项目集分析逐项目输出增强表现
- **WHEN** 迭代 9 的 tasks 设计或测试阶段执行完整项目集 `init` 分析
- **THEN** 报告 MUST 按项目逐个说明 overview、architecture、module、workflow 页的增强表现
- **THEN** 报告 MUST 说明 communities / processes / cycle warnings 是否真正进入正文，而不只是底层 state
- **THEN** 报告 MUST 额外说明 `section_plan` 覆盖率、overview/architecture research 命中和 evidence 引用密度

#### Scenario: 有 reference 的项目继续进行结构对照
- **WHEN** 测试项目存在 `tmp/reference/*`
- **THEN** 项目集分析 MUST 继续对照 `.wiki/*.md` 和 `wiki.metadata.json`
- **THEN** 报告 MUST 明确记录增强后页面结构与 reference 的差异或“无显著差异”

### Requirement: reference 验证必须覆盖专题页、evidence 与图表达
系统 MUST 在 reference 项目验证中显式统计最终 `.wiki/*.md` 中的专题页覆盖率、citation / evidence 落页情况和图表达覆盖率，而不是只统计页面数、行数或 cache 中间态。逐项目报告 MUST 能指出哪些主题未被规划、哪些关键来源未落页、哪些页面仍然缺图，并明确这些统计来自最终 Markdown contract。

#### Scenario: 报告从最终 Markdown 读取 citation / evidence / mermaid
- **WHEN** 测试脚本为带 reference 的项目生成对比报告
- **THEN** 报告 MUST 直接从最终 `.wiki/*.md` 统计 citation、evidence block 和 Mermaid block
- **THEN** 报告不得只依赖 `page_drafts`、`page_context_cache` 或其它中间缓存表给出最终结论

### Requirement: 验证必须覆盖 dossier、provider research session 与 cold/warm 对照
系统 MUST 提供自动化测试和项目集验证，覆盖 targeted dossier snippets、child rollup 稳定性、provider bounded research session、tool schema、phase budget 裁剪和 cold/warm run 差异。测试报告 MUST 明确区分 cold run 与 warm run，而不是把两者混在同一结论里。CodeBuddy Agent 侧验证不属于 9.3 的必做范围。

#### Scenario: 测试区分 cold run 与 warm run
- **WHEN** 验证脚本对同一项目执行两轮启用 LLM 的 workflow
- **THEN** 报告 MUST 明确标记哪一轮是 cold run、哪一轮是 warm run
- **THEN** 报告 MUST 能说明 cache mode 与真实请求数的差异

#### Scenario: 验证 provider bounded research session 与 section-plan 结果
- **WHEN** 测试通过 provider-tools 执行 `overview`、`architecture`、`module` 或 `topic` 页 research session
- **THEN** 测试 MUST 观察到 session 事件、tool 调用和结构化 `PageResearchResult`
- **THEN** 测试 MUST 观察到最终页面仍由 deterministic renderer 落盘
- **THEN** 测试 MUST 观察到结果包含 `section_plan`、精准 snippet 引用和 line-span evidence

### Requirement: 验证必须覆盖实时 usage 输出与同类型 gate 批量化
系统 MUST 在自动化测试和项目报告中覆盖实时 usage 输出、phase budget 生效和同类型 uncertainty gate 批量化行为。普通模式下的 usage 输出 MUST 可被脚本直接消费，不得要求人工去 debug trace 中核对。

#### Scenario: progress/event stream 可观测实时 usage
- **WHEN** 测试脚本执行启用 LLM 的长流程 workflow
- **THEN** 脚本 MUST 观察到 workflow 进行中持续刷新的 usage snapshot
- **THEN** 不得只有 workflow 结束后才一次性看到成本汇总

#### Scenario: file_purpose 等同类型 gate 以批量方式执行
- **WHEN** 测试在存在多条 `file_purpose` 候选的项目上执行 workflow
- **THEN** 报告 MUST 能区分批量 gate 请求与逐条请求
- **THEN** 测试 MUST 继续验证批量化后单条 fallback 和 cache 粒度未被破坏

### Requirement: reference 与项目集验证必须覆盖 family coverage 和页面折叠度
系统 MUST 在项目集分析和 reference 报告中把 family coverage 扩展为 `knowledge-unit decomposition` 命中指标，并继续统计 page collapse。报告 MUST 说明 docs/API/config/runtime/testing/example/tutorial 等单元类型是否进入正式页面，以及哪些 reference 页面仍被错误折叠进 overview 或大模块页。

#### Scenario: 报告输出 decomposition 命中与 page collapse
- **WHEN** 脚本对带 reference 的 docs-heavy 或 runtime-heavy 仓库生成报告
- **THEN** 报告 MUST 输出各类 KnowledgeUnit 的命中分布和 page collapse 指标
- **THEN** 报告 MUST 指出仍然被错误折叠的主要主题家族或知识单元

### Requirement: storybook 专项验证必须覆盖 leaf doc 与 section citation 收敛
系统 MUST 在 `storybook` 专项验证中额外输出 `family-leaf-doc` 覆盖、leaf-first compose 命中和 section-scoped citation 命中情况，并把它们作为 9.4 后半程的核心验收指标。

#### Scenario: storybook 报告输出 leaf doc 覆盖与 section citation
- **WHEN** 系统对 `storybook` 生成专项 reference 报告
- **THEN** 报告 MUST 说明新增 leaf doc 页面数量与仍然折叠的主要 family
- **THEN** 报告 MUST 说明 citation 是否已从页面级附属块转为 section-scoped 命中

### Requirement: dagger 专项验证必须覆盖 runtime-heavy / compiler-heavy 的知识域拆分
系统 MUST 在 `dagger` 专项验证中输出 `CoreRuntime / Framework / PlatformBinding / CompilerToolchain / ApiReference / TestingInfra / ConceptGuide` 等知识域的发现情况，并记录 API/测试/教程页是否仍被折叠回大模块页。

#### Scenario: dagger 报告输出知识域发现与缺页分布
- **WHEN** 系统对 `dagger` 生成专项 reference 报告
- **THEN** 报告 MUST 说明 runtime-heavy / compiler-heavy 相关知识域是否被稳定发现
- **THEN** 报告 MUST 说明缺失 reference 页面主要集中在哪些 API / 框架 / 测试 / 教程主题

#### Scenario: storybook 与 dagger 的 lifecycle 验证都必须通过
- **WHEN** 运行 `storybook + dagger` 的 lifecycle 脚本
- **THEN** 两个样本的 `init → status → sync → update → rebuild` 断言都 MUST 通过
- **THEN** 报告 MUST 明确记录 token 统计、符号图稳定性和 parent rebuild 传播结果

### Requirement: 9.5 的专项验证范围必须固定为 storybook 与 dagger
系统 MUST 将 `9.5` 的实现验证、reference 对比和 lifecycle 验收范围固定为 `storybook` 与 `dagger` 两个样本。当前 change 的通过条件 MUST 只依赖这两个样本的专项结果，不得再要求同步完成 19 项目集全量回归。

#### Scenario: 9.5 验收不再绑定全量项目集
- **WHEN** 系统执行 `9.5` 的测试、专项分析和验收报告生成
- **THEN** 验收范围 MUST 只包含 `storybook` 与 `dagger`
- **THEN** 报告不得把完整项目集回归结果当作本轮通过前置条件

### Requirement: 9.5 的专项验收必须把 storybook 与 dagger 都收敛到至少 95%
系统 MUST 把 `storybook` 与 `dagger` 对各自 reference 的总体对齐率收敛到 `>=95%`，并把这一结果建立在最终 `.wiki/*.md` 的对比结果上。系统 MUST 同时报告 `missing pages`、`collapsed pages` 与 `low-fidelity matched pages`，避免只靠增页堆高表面命中率。

#### Scenario: storybook 与 dagger 的总体对齐率都达到 95%
- **WHEN** 系统生成 `storybook` 与 `dagger` 的最终专项 reference 报告
- **THEN** 报告 MUST 明确给出两者各自的总体对齐率
- **THEN** `storybook` 的总体对齐率 MUST 大于等于 `95%`
- **THEN** `dagger` 的总体对齐率 MUST 大于等于 `95%`

#### Scenario: 95% 不得通过单纯增页获得
- **WHEN** 某轮专项收敛后总体对齐率提升
- **THEN** 报告 MUST 同时给出 `missing pages`、`collapsed pages`、`extra generated pages` 与 `low-fidelity matched pages`
- **THEN** 系统不得仅通过显著增加额外页面而忽略高频 collapse 或低质量命中问题

### Requirement: storybook 与 dagger 专项验证必须同时覆盖 provider、citation 与 decomposition 收敛
系统 MUST 在 `storybook` 与 `dagger` 专项验证中同时输出 provider-backed research 执行情况、最终 markdown citation / diagram 命中，以及 KnowledgeUnit decomposition 命中情况。专项报告 MUST 直接暴露“provider 已执行但 citation 未落页”或“citation 已落页但 unit 仍被错误折叠”这类链路断点。

#### Scenario: storybook 与 dagger 报告输出链路断点
- **WHEN** 系统为 `storybook` 或 `dagger` 生成专项 reference 报告
- **THEN** 报告 MUST 同时说明 provider research 是否生效、最终 citation/diagram 是否落页，以及 decomposition 是否命中目标主题
- **THEN** 报告 MUST 能明确指出主断点位于 provider、renderer 还是 planner

### Requirement: docs-backed 页面验证必须覆盖本地化命名优先级
系统 MUST 在 docs-heavy 样本验证中检查最终 `.wiki/` 的 docs-backed 页面是否优先采用本地化/派生 docs corpus 的命名与路径。若仓库同时存在英文 raw docs 与本地化/派生 docs 语料，专项报告 MUST 能指出英文 raw docs 文件名是否仍大批残留。

#### Scenario: 仓库同时存在英文 raw docs 与本地化/派生 docs 语料
- **WHEN** 系统为 `storybook` 或 `dagger` 生成最终专项报告
- **THEN** 报告 MUST 指出最终 `.wiki/` 是否仍大量保留英文 raw docs 文件名
- **THEN** 若本地化/派生 docs 语料已存在，系统 MUST 优先以该语料生成 docs-backed 页面路径与标题

### Requirement: docs-backed 页面验证必须覆盖主章节骨架对齐
系统 MUST 在 docs-heavy 样本验证中检查 docs-backed 页面是否稳定保留或收敛到 reference 的主章节骨架，而不是退回英文原始 docs 标题或泛化模板章节。

#### Scenario: reference 页面具有稳定主章节序列
- **WHEN** 某个 docs-backed 页面在 reference 中已经表现为稳定章节序列
- **THEN** 最终 `.wiki/*.md` MUST 优先保留或收敛到对应的主章节骨架
- **THEN** 报告 MUST 能指出该页是否偏离了 `cite / 目录 / 简介 / 项目结构 / 核心组件 / 架构总览 / 详细组件分析 / 依赖关系分析 / 性能考量 / 故障排查指南 / 结论 / 附录` 这类 reference 主结构

### Requirement: 9.6 的专项验收必须把 reuse、skeleton fidelity 与 key source coverage 纳入正式门槛
系统 MUST 将 `reuse`、`skeleton fidelity` 与 `key source coverage` 作为 `storybook + dagger` 专项验收的正式门槛，与 `overall_match_rate` 一起回答“页数是否接近 reference、是否存在 coarse page reuse、docs-backed 页面是否具备 reference 式骨架、正文是否真正覆盖关键文件”这四个问题。`overall_match_rate` MUST 仅作为一级门槛，不能单独决定通过。

#### Scenario: 9.6 专项报告生成最终门禁结论
- **WHEN** 系统生成 `storybook` 或 `dagger` 的 9.6 专项 reference 报告
- **THEN** 报告 MUST 同时给出 `overall_match_rate`、`reuse`、`skeleton fidelity` 与 `key source coverage`
- **THEN** 报告 MUST 明确回答页数、reuse、章节骨架与关键文件覆盖这四个专项问题
- **THEN** 系统 MUST 不得仅凭 `overall_match_rate >= 95%` 就判定通过

### Requirement: 9.6 的项目分析脚本必须消费 2.0 的 KnowledgeUnit/Research 数据面
系统 MUST 让项目分析脚本基于 `knowledge_units`、`knowledge_domains`、`research_cache`、`wiki_pages`、`page_digests` 等 2.0 runtime 数据进行统计，而不能继续依赖旧的 `page_context_cache.context.research_result`、`topic_dossier` 或旧 `page_type` 语义来判断 research 命中和页面类型。

#### Scenario: 项目分析脚本统计 storybook 或 dagger 的 research 命中
- **WHEN** 系统执行 `collect-test-project-analysis` 或等价项目分析脚本
- **THEN** 脚本 MUST 从 `knowledge_units / research_cache / wiki_pages` 读取研究与页面统计
- **THEN** 脚本 MUST 不得再因为旧 `PageContext` 字段缺失而把已完成的 research 统计为 0
- **THEN** 输出的页面类型与 decomposition 统计 MUST 与 2.0 的 KnowledgeUnit 主线一致

### Requirement: 9.6 的专项报告必须验证 warm report 稳定性
系统 MUST 对同一项目至少执行两次 warm report，并比较 `reuse_overage`、`median skeleton fidelity` 与 `median key source coverage` 的波动。若指标抖动超出允许范围，报告 MUST 标记该项目当前口径不稳定，不能作为后续迭代基线。

#### Scenario: warm report 重跑同一项目
- **WHEN** 系统对 `storybook` 或 `dagger` 连续执行至少两次 warm report
- **THEN** 报告 MUST 输出两次结果的 `reuse_overage`、`median skeleton fidelity` 与 `median key source coverage`
- **THEN** 报告 MUST 判断这些指标的波动是否处于允许范围
- **THEN** 若波动超出范围，系统 MUST 将该快照标记为不稳定，而不是直接作为 9.7-9.9 的验收基线

### Requirement: 9.7 专项验证必须同时覆盖父页 contract 与 runtime readiness
系统 MUST 在 `storybook + dagger` 专项验证中同时验证高层父页 contract 与 runtime readiness，而不是继续只看最终 Markdown 匹配率。验证 MUST 直接读取最终 `.wiki/*.md`、runtime SQLite 状态与 parent contract 摘要，确认高层父页是否消费 child-backed rollup，及 runtime incomplete 是否能定位到具体 gate。

#### Scenario: storybook 专项验证高层父页 contract
- **WHEN** 系统对 `storybook` 运行 9.7 专项验证
- **THEN** 报告 MUST 指出高层 `Overview`、`Architecture`、`DomainIndex` 或 `config_surface` parent unit 的 reuse 收敛情况
- **THEN** 验证 MUST 证明这些父页存在 child-backed compose contract 或对应的 runtime 摘要
- **THEN** 系统 MUST NOT 仅凭 `overall_match_rate` 或页面数量判断通过

#### Scenario: dagger 专项验证 runtime readiness
- **WHEN** 系统对 `dagger` 运行 9.7 专项验证
- **THEN** 验证 MUST 指出 workflow 当前停在 `research`、`compose` 还是 `assemble` 阶段
- **THEN** 验证 MUST 输出对应 unit 的 gate/readiness 原因
- **THEN** 当 workflow 成功完成时，验证 MUST 观察到 `page_drafts`、最终 wiki 页面和 `wiki.metadata.json` 一并落盘

