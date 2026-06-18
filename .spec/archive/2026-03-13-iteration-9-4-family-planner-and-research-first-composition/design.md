## Context

9.3 之后，`spec-wiki` 已经具备 `TargetedSnippet`、`section_plan`、`overview/architecture` research 和 archetype topic，但页面主链的主导权仍然在 `planner -> page_context -> renderer` 这一套 deterministic 页面模板上。当前实现虽然能把 research 结果写进页面，却还没有形成类似 CodeWiki 和 deepwiki-rs 那样的“先研究对象、再成页”的生成方式。

这个问题在 `storybook` 上最明显。reference 并不是单纯的模块页和专题页集合，而更像一棵产品知识树：

```text
项目概述
核心概念
插件系统
多框架支持
构建系统
API参考
部署和 CI/CD
高级功能
故障排除
主题和外观
```

而当前实现仍主要围绕：

```text
overview
architecture
workflow
module
topic
```

这说明当前差距已经不再是“正文够不够像 reference”，而是“页面本体是否正确”。9.4 需要把 planner 上游再抬一层：先识别内容家族和研究域，再决定正式页面集合。9.4 的主目标，是让 `storybook + dagger` 这两个验收样本的生成结果在页面树、主题边界、API/config/docs 命中和出处密度上尽可能贴近各自 reference。

结合本地参考仓库的真实实现，可以归纳出两条可直接借鉴的主线：

- CodeWiki：`leaf-first + source-fed + parent-doc-rollup`
  - 叶子模块直接吃一手源码，父模块再基于子文档汇总，见 [documentation_generator.py](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/documentation_generator.py)、[prompt_template.py](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/prompt_template.py)。
- deepwiki-rs：`research-first + domain-oriented compose`
  - 先产出系统/架构/模块研究结果，再由 editor 按页面类型成页，见 [workflow.rs](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/workflow.rs)、[orchestrator.rs](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/research/orchestrator.rs)、[compose/mod.rs](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/compose/mod.rs)。
- Qoder 中间索引：`tree / graph / memory / vector / completion` 分层持久化
  - 本地目录 [index](C:/Users/baiyucraft/AppData/Roaming/Qoder/SharedClientCache/index) 说明它把目录树、关系图、记忆网络和消费层缓存拆开存，不把“树事实”和“会话/检索结果”揉进同一层。9.4 需要借这一点约束 family planner 与 research digest 的边界。

9.4 要做的是把这两条主线落到当前 deterministic 主链之内，而不是抛弃现有 runtime、managed section 和增量更新 contract。

在 `storybook` 专项验证之后，9.4 的剩余差距已经更具体地暴露出来；而 `dagger` 的 warm 对照又补充暴露出另一类问题：

- family 树已经出现，但 `family-child` 仍然折叠了过多 reference 子页
- compose 仍主要停留在“section 级组织”，还没有形成真正的叶子文档单元
- evidence 已进入页面，但仍然主要以“页面附属块”表达，而不是章节级引用计划
- docs / API / config / type surface 已进入 dossier，但还没有真正主导下一层页面拆分
- 对 runtime-heavy / compiler-heavy 仓库，当前页面树仍过度依赖模块页和专题页，尚不足以覆盖 reference 中的 API、框架、测试和教程页族

因此 9.4 后半程不再只是“做出 family”，而是要把 family 继续拆成更接近 reference 的叶子文档层，并让 citation 真正进入 compose 主链。

## Goals / Non-Goals

**Goals:**

- 让 `storybook + dagger` 的生成产物在页面树、主题边界、API/config/docs 覆盖、citation 密度和页面折叠度上尽可能贴近 reference。
- 在模块树之上引入稳定的 `content family planner`，让产品文档型仓库可以生成 family index 和 family child 页，而不是继续把不同知识域折叠进少数 module/topic 页。
- 引入 `leaf-first/source-fed` 路径，让叶子 family 页、叶子模块页和高置信主题页可以优先消费一手源码、public API、config surface 和 docs anchors。
- 把 research 升级为正式 compose 前置层，使 `overview / architecture / family / module / topic` 页都先形成研究材料，再由 compose 层组织页面结构。
- 让父页真正消费子页结果，而不是只消费 `child summary` 这类薄 rollup。
- 针对 `storybook + dagger` 增加 reference 对齐指标，重点观察 family coverage、模块/API 拆分、page collapse、API/config/docs 命中和 citation 密度；9.4 的测试和验证范围只收敛到这两个样本。

**Non-Goals:**

- 不在 9.4 内追求对全量 19 个测试项目同时收敛。
- 不在 9.4 解决迭代 11 的 CodeBuddy Agent bridge 或宿主可用性问题。
- 不在 9.4 重做 query/RAG/embedding 路线；这仍属于迭代 10 范围。
- 不让 LLM 直接输出最终 Markdown 页面，也不引入自由命名页面集合。
- 不试图逐页复刻 reference 的产品目录；目标是让页面本体和主题边界明显向 reference 靠拢，而不是一比一复制站点结构。

## Decisions

### 决策 1：在 `topic` 之上增加 `family` 规划层，而不是继续横向扩张 `topic_kind`

9.4 引入新的 `family` 页面家族层，位置在当前 `module/topic` 之上，作用是表达“产品知识域”而不是“代码文件簇”。family 候选来源包括：

- docs 目录主题树
- package/workspace 中的 public API 与类型入口
- `main.ts / preview.ts / manager.ts / preset.ts` 这类配置/扩展入口
- builder/framework/addon 目录与命名约定
- 当前已有 module/topic page 的高密度聚类结果

family page 分为两类：

- `family-index`：家族索引页，例如“插件系统”“多框架支持”“API参考”
- `family-child`：家族子页，例如“React 框架支持”“Preview API”“Themes Addon”

不继续横向增加 `topic_kind` 的原因是：

- `topic` 仍然适合表达实现机制、流程或 repo archetype 主题，不适合表达“产品知识树”。
- 把 family 继续塞进 `topic_kind` 会让 dedupe、父子关系和 index/child 组织越来越混乱。

备选方案：

- 继续扩张 `topic_kind`：实现成本低，但无法表达 index/child 结构，也很难解决 storybook 的“页面折叠”问题。
- 让 LLM 直接决定 family 集合：违背当前 deterministic planner 边界。

### 决策 2：引入 `FamilyDossier`，并让 dossier 承载 docs/API/config surface

当前 dossier 更偏代码实现片段。9.4 要新增 `FamilyDossier`，并同时扩展现有 `RepoDossier / ModuleDossier / TopicDossier`，让它们可以携带：

- `docs_anchors`
- `public_api_surfaces`
- `config_surfaces`
- `type_surfaces`
- `child_page_results`
- `family_scoped_evidence`

其中 `public_api_surfaces` 与 `config_surfaces` 的提取将优先依赖：

- 入口/导出文件
- 类型定义文件
- 约定配置文件
- manifest / package 元数据

选择这一方案，是因为 storybook 这类 reference 的大量页面并不是靠实现片段支撑，而是靠：

- API 暴露面
- 配置面
- docs 入口
- 示例与约定

只继续加强 `TargetedSnippet` 还不够。

### 决策 3：research-first compose 继续保留 deterministic renderer，但降低 renderer 的主导权

9.4 不会让 LLM 直接写最终 Markdown 文件，但会把当前 renderer 的角色从“主导章节结构”降为“按 compose plan 落盘”。新的页面生成顺序为：

```text
planner
-> dossier builders
-> research builders
-> compose plans
-> deterministic renderer
```

其中 `compose plan` 至少要包含：

- page positioning
- section order
- section summaries
- evidence refs
- diagram refs
- child page refs

这比 9.3 的 `section_plan` 更进一步：不只是 section 列表，而是“正式成页计划”。

保留 deterministic renderer 的原因：

- 现有 runtime 仍依赖 managed marker、stable `section_id` 和增量更新。
- 用户编辑保留语义不能被自由 Markdown 输出破坏。

### 决策 3.2：在 `family-child` 之下增加通用 `family-leaf-doc` 层，而不是继续让子页膨胀成大页

9.4 后半程引入 `family-leaf-doc` 页面类型，位置位于 `family-child` 之下。它不是样本仓库特判，而是 docs-heavy / platform archetype 的通用“叶子文档单元”：

- `family-index`
  - 负责家族总览与导航
- `family-child`
  - 负责稳定子主题分组
- `family-leaf-doc`
  - 负责 reference 中真正细粒度的 API / addon / config / docs / troubleshooting 叶子文档

`family-leaf-doc` 的候选来源 MUST 优先来自：

- docs anchors 的子目录 / 文件名聚类
- public API surface 的导出入口聚类
- config surface 的配置入口聚类
- type surface 的类型面聚类
- child/page digest 中已验证的高信号主题

不继续扩大 `family-child` section 数量的原因是：

- 这会进一步加剧 `page collapse`
- reference 的差距已经明确表现为“缺少叶子文档页”，而不是“某个 family child 少几个小节”
- `storybook` 只是当前验收样本，但 docs-heavy archetype 本身就需要这层抽象

### 决策 3.3：采用 `leaf-first compose`，父页消费叶子文档结果，而不是继续直接消费 surface 列表

9.4 后半程明确借鉴 CodeWiki 的 leaf-first 路线：

- `family-leaf-doc`
  - 优先消费一手 docs / API / config / type / targeted source 材料
- `family-child`
  - 优先消费 leaf doc 的结构化结果
- `family-index / overview / architecture`
  - 优先消费 child / leaf 的结构化结果

这意味着当前的 `parent consume child` 还要再向前推一层：

- 从“父页消费 child rollup”
- 升级到“父页消费 leaf-first 结果链”

系统仍不直接读取完整 Markdown 正文做 parent compose，但 MUST 让 leaf doc 的 `compose plan / citation plan / key sources` 进入 parent digest。

### 决策 3.4：把 evidence 从页面附属块升级到 `section-scoped citation plan`

9.4 后半程不再把 citation 视为“页面末尾附加块”，而是把它收进 compose 计划本身：

- 每个 compose section SHOULD 显式给出 `evidence_refs`
- renderer MUST 依据 section 计划把来源块绑定到对应 section
- parent digest MUST 保留子页 section 的 citation 指向，而不是只保留摘要

这样做的原因是：

- 当前 citation 密度远低于 reference，不是因为没有 evidence，而是 evidence 没有进入正确的 section 作用域
- reference 的大量页面更接近“章节级引用文档”，而不是“整页统一引用文档”
- 这也更符合 deepwiki-rs 的 research -> compose 分层：研究结果不只决定写什么，也决定“每节以哪些来源为支撑”

### 决策 3.1：保持 `tree / graph / dossier / research-digest` 分层，不把样本规则写进 renderer

9.4 参考 Qoder 的索引目录后，明确采用分层约束：

- `tree / graph`
  - 继续承载扫描结果、模块树、关系边和稳定事实。
- `family planner / dossier`
  - 只消费树、图、docs/API/config/type 信号，产出可复用的页面规划与研究输入。
- `research / child digest`
  - 只承载 LLM 生成的 compose 计划、证据 rollup 和 parent consume child 的中间结果。
- `renderer`
  - 只负责按 compose plan 落盘，不反向引入样本仓库的硬编码目录规则。

这样做的原因是：

- `storybook` 只是 9.4 的验收样本，不应该被写成 core 的长期事实模型。
- family planner 的样本经验可以沉淀为 archetype profile/catalog，但不能直接混入 renderer 或 session 逻辑。
- 这也更符合 CodeWiki 的 leaf-first 输入面和 deepwiki-rs 的 research/compose 分层。

### 决策 4：实现真正的 `parent consume child docs/results`，而不是只消费轻量 rollup

9.4 会把当前 `ChildPageRollup` 升级为两层：

- `child_result_rollup`
  - 面向 planner/research，携带 `summary`、`compose_plan`、`evidence_rollup`、`diagram_rollup`
- `child_page_digest`
  - 面向 parent compose，携带压缩后的子页结构和关键段落表达

父页 compose 时优先使用这两层对象，而不是再次重扫同一批 facts。

这个决策直接借鉴 CodeWiki 的：

- 叶子模块先生成文档
- 父模块读取子文档，再生成 overview

但在当前仓库里不会直接读取完整 Markdown 全文，而是优先读取结构化 digest，避免 token 失控。

### 决策 5：对 `storybook` 按 archetype 引入专用 family rules，并以 reference 贴近度验收

9.4 不做完全通用的“任何仓库都能自动长出任意 family 树”。第一版只明确支持高价值 archetype：

- 文档平台 / UI 工具平台
- Web framework / backend framework
- SDK / library
- CLI / developer tool
- ops / deployment-heavy repo

其中 `storybook` 将作为 family planner 的首要目标仓库。第一版 family rules 至少覆盖：

- concept
- addon
- framework
- builder
- api
- config
- ops
- troubleshooting
- theme

原因是这类规则已经在 `storybook` 的 reference 和真实代码结构里反复出现，收益最大，也最便于直接拿 reference 对照。

### 决策 6：reference 验证从“页数/命中”升级为“页面折叠度与 family 覆盖”，并以 `storybook` 为单一验收对象

9.4 的验证不再只统计 `generated / matched / missing`，而要新增：

- `familyCoverage`
- `familyIndexCount`
- `familyChildCount`
- `apiSurfaceHitRate`
- `configSurfaceHitRate`
- `docsAnchorHitRate`
- `pageCollapseRatio`

其中 `pageCollapseRatio` 用于衡量一个生成页映射到多少个 reference 页；`storybook` 目前正是这个指标过高，说明页面本体不对。9.4 的验收重点不是“功能都实现了”，而是这些指标是否让 `storybook` 更贴近 reference。

9.4 不把这套验证立即扩到全量 19 项目。原因是：

- 当前 change 的唯一验收对象就是 `storybook`，要先把这类 docs-heavy/platform 仓库的页面本体问题收住。
- 如果一开始就回跑 19 项目，调试成本会被全局噪音放大，不利于收敛 family planner 的第一版规则。
- 等 `storybook` 路径收稳后，再决定是否单开后续 change 做全量项目集回归。

### 决策 6.1：`storybook` 继续作为验收样本，但不得回流为 `storybook_*` 特判实现

9.4 后半程允许继续用 `storybook` 暴露缺口、指导 refactor，但 core 内的正式抽象 MUST 保持通用：

- archetype signal
- family profile
- leaf decomposition rule
- citation plan

系统不得把 `storybook` 的标题、目录、路径或仓库名判断直接硬编码进 planner / renderer / session 主链。`storybook` 只是验收样本，不是长期事实模型。

## Risks / Trade-offs

- [风险：family planner 过于依赖命名约定，导致误判] → 第一版只支持高价值 archetype，并要求 family 候选至少命中多个独立信号源。
- [风险：dossier 输入进一步变重，导致 cold run 更慢] → 继续使用 phase budget、family page 优先级和 leaf-first 顺序，先控制高价值页的研究材料质量，而不是无限加料。
- [风险：parent consume child results 让缓存关系更复杂] → 把 child result/digest identity 纳入现有 cache key 和 change set 传播链，而不是在 compose 阶段隐式读取页面文件。
- [风险：family 页与 module/topic 页重复] → planner 中明确 family/topic/module 的收编和抑制规则，优先避免重复页，而不是允许多页竞争同一主题。
- [风险：docs/API/config 入口在不同语言仓库差异很大] → 第一版只保证代表性 archetype； unsupported 仓库继续回退到现有 module/topic 主链。
- [风险：leaf doc 拆分过细导致页面暴涨或内容稀释] → 第一版只在 docs-heavy / platform archetype 内启用，且要求 leaf 候选命中多个 signal 或稳定 docs/API/config 子域。
- [风险：section-scoped citation plan 增加 renderer 和 parent digest 复杂度] → 继续使用结构化 `evidence_refs`，不引入自由 inline citation 语法。

## Migration Plan

1. 在 planner 中增加 family candidate discovery、family page identity 和 parent-child 规则。
2. 扩展 dossier builder，增加 `FamilyDossier` 以及 docs/API/config/type surface 输入。
3. 在 family-child 之下增加通用 `family-leaf-doc` 规划与 leaf decomposition。
4. 调整 research contract，把 family 页、leaf doc 与 parent consume child digest 纳入正式 compose 流。
5. 下沉 renderer 到 compose plan 之后，并让 citation/evidence 进入 section 级作用域。
6. 升级 reference 脚本与专项分析，先只以 `storybook` 做重点对照。

当前处于测试开发阶段，不保留旧页面拓扑兼容。对于因 family planner 引起的页面路径和父子关系变化，允许在 9.4 内直接替换旧 contract。

## Open Questions

- `FamilyDossier` 是否需要单独持久化为独立 cache 表，还是继续复用现有 page context cache。
- `child_page_digest` 第一版是直接从结构化 compose plan 生成，还是允许有限摘录子页正文。
- storybook 之外，第二个最值得优先支持的 archetype 是 `fastapi` 这类 docs-heavy framework，还是 `docker-mailserver` 这类 ops-heavy repo。
- `family-leaf-doc` 是否需要单独 dossier 类型，还是先复用 `FamilyDossier` 并在后续迭代拆分。
