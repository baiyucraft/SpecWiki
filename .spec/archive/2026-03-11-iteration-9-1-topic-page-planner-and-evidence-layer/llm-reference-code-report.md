# LLM 调用方式代码对比报告

生成时间：2026-03-11

## 1. 范围与结论先行

这份报告只基于真实源码阅读，不基于 README、产品页或说明文档。

我重点阅读了以下代码：

- 当前仓库 `crates/wiki-core/src/llm/mod.rs`
- 当前仓库 `crates/wiki-core/src/generation/context.rs`
- deepwiki-rs `src/generator/preprocess/agents/code_analyze.rs`
- deepwiki-rs `src/generator/preprocess/agents/code_purpose_analyze.rs`
- deepwiki-rs `src/generator/preprocess/agents/relationships_analyze.rs`
- deepwiki-rs `src/generator/step_forward_agent.rs`
- deepwiki-rs `src/generator/compose/agents/overview_editor.rs`
- CodeWiki `src/be/documentation_generator.py`
- CodeWiki `src/be/agent_orchestrator.py`
- CodeWiki `src/be/prompt_template.py`
- CodeWiki `src/be/cluster_modules.py`
- CodeWiki `src/be/llm_services.py`
- CodeWiki `src/be/agent_tools/read_code_components.py`
- deepwiki-open `api/rag.py`
- deepwiki-open `api/prompts.py`
- deepwiki-open `api/simple_chat.py`
- deepwiki-open `api/websocket_wiki.py`
- GitNexus `gitnexus/src/core/ingestion/pipeline.ts`
- GitNexus `gitnexus/src/core/ingestion/cluster-enricher.ts`
- GitNexus `gitnexus-web/src/core/llm/agent.ts`
- GitNexus `gitnexus-web/src/core/llm/context-builder.ts`

先说结论：

1. 当前仓库把 LLM 主要放在 `uncertainty_gate + page_enrichment`，位置偏后，输入也偏“聚合后的 facts/summary/evidence”。
2. deepwiki-rs 和 CodeWiki 的共同点，不是“提示词写得更华丽”，而是让 LLM 更早接触结构材料、代码内容、子模块文档和研究结果。
3. deepwiki-open 和 GitNexus 的共同点，是把 LLM 放在“检索消费层”或“图解释层”，而不是用它直接承担整条 Wiki 生成主链。
4. 所以你现在觉得“生成 wiki 的逻辑不对”，核心问题大概率不是某几句 prompt，而是 LLM 介入的阶段和送入材料的粒度不对。

## 2. 当前仓库：LLM 现在是怎么用的

### 2.1 调用位置

当前仓库的 LLM 调用主要集中在 `crates/wiki-core/src/llm/mod.rs`，分成两类：

1. `uncertainty_gate`
2. `page_enrichment`

`uncertainty_gate` 主要包括：

- `classify_file_purpose`
- `classify_file_purposes`
- `decide_top_level_promotion`
- `classify_module_kind`
- 低置信度依赖辅助判断

`page_enrichment` 则是在页面上下文已经构造完成之后，对页面正文做结构化增强。

### 2.2 给模型的数据

`FilePurposeAssistInput` 的真实字段是：

- `path`
- `kind`
- `language`
- `file_size`
- `deterministic`
- `preview`

也就是说，文件角色判断并没有把整文件源码、调用图、导入图交给模型，只给了“路径 + 语言 + 粗分类 + 截断预览”。

`PageEnrichmentInput` 的真实字段是：

- `page_id`
- `page_type`
- `title`
- `scope`
- `section_titles`
- `facts`
- `summary_inputs`
- `hints`
- `child_summaries`
- `evidence_groups`
- `diagram_inputs`
- `allow_mermaid`

这部分输入由 `crates/wiki-core/src/generation/context.rs` 组装。真实数据形态不是“代码块 + 符号图 + 调用链原文”，而是已经被压缩成：

- `技术栈：...`
- `核心源码：...`
- `图热点：...`
- `流程：...`
- `社区：...`
- `循环：...`
- `入口：...`
- `依赖：...`
- `主题摘要：...`

`evidence_groups` 和 `diagram_inputs` 是本轮 9.1 新加的稳定输入，但本质上仍然是 facts layer 的解释层包装，不是大段源码或原始 symbol graph。

### 2.3 提示词形态

当前仓库 provider 侧真正送给模型的是一个结构化 JSON message：

- `prompt_type`
- `prompt_version`
- `input_hash`
- `instruction`
- `response_schema`
- `input`

`page_enrichment` 的 instruction 明确要求：

- 只能改写解释层
- 不能虚构 facts
- 优先把稳定 facts 组织成正文
- 只能在已有 `diagram_inputs` 上生成 Mermaid

这条链路的优点是：

- 非常稳
- 容易缓存
- 容易回退
- 不容易胡编

但它的缺点也很明显：

- LLM 接触源码太晚
- 接触到的已经不是“研究材料”，而是“被 core 先解释了一遍的材料”
- 生成页更像“事实整理器”，不像“研究驱动的文档作者”

### 2.4 对当前仓库的判断

当前仓库的 LLM 更像一个：

- 扫描阶段的保守分类器
- 页面阶段的结构化润色器

它还不是一个真正意义上的：

- 代码研究器
- 模块洞察生成器
- 自底向上文档编织器

这就是当前页内容和 reference 总有“密度还行，但味道不对”的根因。

## 3. deepwiki-rs：LLM 主要用在研究链，而不是只做最终润色

### 3.1 调用位置

deepwiki-rs 的 LLM 调用贯穿三层：

1. preprocess
2. research
3. compose

其中最关键的是 preprocess 和 research。

`workflow.rs` 的主链是：

- `PreProcessAgent`
- `ResearchOrchestrator`
- `DocumentationComposer`

也就是说，文档生成前已经有一层 AI 研究结果沉淀在 memory 里。

### 3.2 具体 prompt 和输入

#### A. `code_purpose_analyze.rs`

这条链是“文件角色判断”。

system prompt 很直接：

- 你是代码架构分析师，专门分析代码文件的组件类型

user prompt 的真实输入是：

- 文件路径
- 文件名
- 文件内容预览

模板里还明确要求模型根据以下因素判断：

- 路径和目录结构
- 文件名和扩展名
- 文件内容和代码结构
- import/export
- 函数和类定义
- 业务逻辑特征

这和当前仓库 `FilePurposeAssistInput` 很像，但它更强调“代码结构和导入导出”，而不是只是保守枚举兜底。

#### B. `code_analyze.rs`

这条链更关键。

它先做规则分析，再把以下内容一起交给 LLM：

- component name
- file path
- component type
- importance score
- responsibilities
- interface count
- dependency count
- LOC
- cyclomatic complexity
- source summary
- dependency component code snippets

这里有一个非常重要的差异：

deepwiki-rs 不只是把当前文件的摘要给模型，还会把依赖组件的代码片段一起给模型。

所以它让模型分析的是：

- 当前文件是什么
- 它依赖谁
- 它在系统里处在什么位置

而不是只让模型在页面阶段对“事实列表”做语句改写。

#### C. `relationships_analyze.rs`

这条链做项目级依赖关系分析。

system prompt 明确是：

- 你是项目级代码依赖关系图分析师

user prompt 的真实输入是压缩后的 `code insights` 列表，重点字段包括：

- 文件名
- code purpose
- path
- importance
- complexity
- dependencies

然后要求生成：

- core module dependencies
- key data flows
- architecture hierarchy
- circular dependency warnings

也就是说，deepwiki-rs 在“文档还没开始写”之前，已经先让 LLM 看过项目级结构关系。

### 3.3 compose 阶段怎么用

`step_forward_agent.rs` 会把多类研究材料拼进 prompt：

- 项目结构
- code insights
- README
- dependency analysis
- 既有 research results
- 外部 knowledge

`overview_editor.rs` 这种 compose agent 不是直接从扫描 facts 出发，而是基于：

- system context research
- domain modules research
- 可选外部知识

来写最终文档。

### 3.4 deepwiki-rs 的关键特征

它的 LLM 不是“末端改写器”，而是“中前段研究器”。

它给模型的材料明显更靠近：

- 原始代码
- 依赖代码片段
- 项目结构
- research memory

不是只给“页面 facts”。

## 4. CodeWiki：LLM 直接读模块源码，自底向上生成父子文档

### 4.1 调用位置

CodeWiki 的 LLM 主要有三类用法：

1. `cluster_modules.py` 做模块聚类
2. `agent_orchestrator.py` 做叶子模块文档生成
3. `documentation_generator.py` 做父模块和仓库 overview

### 4.2 具体 prompt 和输入

#### A. 模块聚类 `cluster_modules.py`

这里调用 `call_llm(prompt, config, model=config.cluster_model)`。

prompt 来自 `format_cluster_prompt(...)`，输入是真实的：

- potential core components 列表
- 模块树

它要求模型输出 `<GROUPED_COMPONENTS>` JSON。

注意这里不是让模型写文案，而是让模型参与模块划分。

#### B. 叶子模块文档 `agent_orchestrator.py + prompt_template.py`

CodeWiki 给 agent 的 system prompt 非常明确：

- 任务是基于 module name 和 core code components 生成系统文档
- 需要写主文档
- 复杂模块需要递归生成 sub-module docs
- 要用 Mermaid
- 要避免和其他模块重复

更重要的是 user prompt 的真实输入。

`format_user_prompt(...)` 会按文件分组，把每个 core component 所在文件的完整内容直接塞进去：

- module tree
- core component ids
- grouped file content
- 每个文件里有哪些 core components

这和当前仓库差异很大。当前仓库 page prompt 基本不直接喂完整源码，而 CodeWiki 是直接把模块关键文件源码送给模型。

#### C. 父模块文档 `documentation_generator.py`

父模块 overview 不是重新扫源码生成，而是构造一个 `repo_structure` JSON，其中明确附带：

- 目标模块位置
- 1-depth children
- child docs 内容

然后用 `MODULE_OVERVIEW_PROMPT` 或 `REPO_OVERVIEW_PROMPT` 让模型写 overview。

这点非常关键：父页显式消费子页文档。

### 4.3 Tool 使用方式

复杂模块 agent 还可以用工具：

- `read_code_components`
- `str_replace_editor`
- `generate_sub_module_documentation`

其中 `read_code_components` 能按 component id 把源码再读给模型。

也就是说，CodeWiki 的 LLM 在运行时还可以主动拉取更多代码上下文，不是一次性只吃固定 facts。

### 4.4 CodeWiki 的关键特征

它的 LLM 真正承担了：

- 模块划分
- 叶子页生成
- 父页汇总

而且输入材料是：

- 模块树
- 组件列表
- 真实文件内容
- 已生成的子页文档

这比当前仓库更像“自底向上的文档编排器”。

## 5. deepwiki-open：LLM 主要在检索问答和对话消费层

### 5.1 调用位置

deepwiki-open 的重点不在离线 Wiki 生成，而在：

- `rag.py`
- `simple_chat.py`
- `websocket_wiki.py`

### 5.2 prompt 和输入

`prompts.py` 里的 `RAG_TEMPLATE` 真实输入包括：

- `system_prompt`
- `output_format_str`
- `conversation_history`
- `contexts`
- `input_str`

其中 `contexts` 是检索回来的文档块，每块都带：

- `File Path`
- `Content`

`simple_chat.py` 和 `websocket_wiki.py` 在真正发请求前，又会把 prompt 组装成：

- `system_prompt`
- `conversation_history`
- `currentFileContent` 可选
- `<START_OF_CONTEXT>...<END_OF_CONTEXT>`
- `<query>`

并且上下文会先按 `file path` 分组。

### 5.3 deepwiki-open 的关键特征

它让模型消费的是：

- conversation history
- file-scoped retrieved context
- exact file content
- current query

这不是 Wiki 主生成链，而是 query/RAG 消费链。

所以它可以借鉴的是：

- 文件路径分组的上下文组织
- query 任务下的 prompt 结构
- citation/上下文约束

不能直接拿来当当前仓库 Wiki 生成的模板。

## 6. GitNexus：LLM 主要在图解释层和 Graph RAG agent

### 6.1 ingestion 主链里 LLM 的位置

GitNexus 的核心 ingestion pipeline 基本是 deterministic：

- `processStructure`
- `processParsing`
- `processImports`
- `processCalls`
- `processHeritage`
- `processCommunities`
- `processProcesses`

这条主链把真实结构事实先建成 graph。

LLM 不是先进去决定事实，而是后置地解释 graph。

### 6.2 `cluster-enricher.ts`

这里的 LLM prompt 很典型：

- 输入是 cluster 的 `heuristicLabel`
- 输入是 member list
- member 只包含 `name / filePath / type`
- 输出要求是 JSON

它的目标只是给 cluster 起：

- semantic name
- keywords
- description

而不是直接写 Wiki 页面。

更重要的是，这个文件里已经显式实现了两种模式：

1. 单 cluster 调用
2. `enrichClustersBatch` 批量调用

也就是说，GitNexus 已经明确采用了“同类型批量判断”的思路。

### 6.3 `gitnexus-web/src/core/llm/agent.ts`

Graph RAG agent 的 system prompt 很强，核心要求是：

- 所有事实都要 citation
- 必须 validate
- 先 search/read/trace，再回答
- 用 tables 和 Mermaid

动态上下文来自 `context-builder.ts`，真实注入的是：

- project stats
- hotspots
- folder tree

然后 agent 再通过工具去拿：

- search
- cypher
- grep
- read
- explore
- overview
- impact

### 6.4 GitNexus 的关键特征

GitNexus 的 LLM 主要承担：

- cluster 语义解释
- graph-grounded 问答代理

它并不直接承担“整仓 Wiki 离线写作主链”。

它最值得借鉴的是：

- 先建强图事实，再让 LLM 解释
- 批量 enrichment
- 强 citation / validation 约束

## 7. 对比表：五个实现到底差在哪

| 项目 | LLM 介入阶段 | 主要输入 | 主要输出 | 角色定位 |
| --- | --- | --- | --- | --- |
| 当前仓库 | 扫描兜底 + 页面增强末端 | 文件预览、facts、summary_inputs、evidence_groups、diagram_inputs | 枚举分类、页面正文、Mermaid | 保守分类器 + 正文润色器 |
| deepwiki-rs | preprocess + research + compose | 项目结构、文件内容/摘要、依赖代码片段、研究结果 | code insight、关系分析、研究报告、最终文档 | 中前段研究器 |
| CodeWiki | 聚类 + 叶子文档 + 父文档 | 模块树、组件列表、完整文件内容、子页 docs、tool 读出的源码 | 模块树、模块 docs、overview | 自底向上文档编排器 |
| deepwiki-open | RAG / chat 消费层 | conversation、retrieved contexts、file content、query | 对话回答 | 检索问答器 |
| GitNexus | 图解释层 / agent 层 | graph stats、hotspots、folder tree、cluster members、tool results | cluster label、agent answer | 图解释器 / Graph RAG agent |

## 7.1 生成 page 时，是否把源码给 LLM

这部分单独回答一个非常具体的问题：

“在参考项目里，真正生成 page 的时候，会不会把源码正文给 LLM？”

先给结论：

- 当前仓库：page 阶段基本不给源码正文。
- `CodeWiki`：会，叶子模块文档阶段会直接给完整文件内容。
- `deepwiki-rs`：会，但更集中在 preprocess/research；compose 写 page 时更多消费研究结果。
- `deepwiki-open`：不是典型 page generator，query 时会给检索片段和可选文件正文。
- `GitNexus`：离线生成图阶段不给源码到 LLM；agent 问答时通过工具再读源码。

### 直接对照表

| 项目 | page 生成时是否直接给源码 | 给到什么程度 | 发生阶段 |
| --- | --- | --- | --- |
| 当前仓库 | 基本否 | 路径、facts、summary、evidence、diagram input；无源码正文 | `page_enrichment` |
| deepwiki-rs | 部分是 | 文件内容预览、源码摘要、依赖代码片段 | 主要在 `preprocess/research`，不是每个 compose page 都直接给 |
| CodeWiki | 是 | 叶子模块 prompt 直接包含完整文件内容；还可工具读取组件源码 | `process_module` 叶子文档阶段 |
| deepwiki-open | 不适用 | 检索 context、按文件分组的片段、可选当前文件正文 | `RAG/chat` |
| GitNexus | 否 | cluster member 名称、类型、路径；源码通过 agent 工具另取 | `cluster enrichment` / `Graph RAG agent` |

### 当前仓库

当前仓库 page 阶段送给 LLM 的是 `PageEnrichmentInput`，字段只有：

- `facts`
- `summary_inputs`
- `hints`
- `child_summaries`
- `evidence_groups`
- `diagram_inputs`

这里没有源码内容字段，见 `crates/wiki-core/src/llm/mod.rs` 的 `PageEnrichmentInput`。

`evidence_groups` 里也只有：

- `label`
- `path`
- `source_id`
- `note`

没有源码正文，见 `crates/wiki-core/src/domain/context.rs` 的 `PageEvidenceItem`。

唯一和源码正文接近的，是扫描阶段 `FilePurposeAssistInput.preview`，但那属于 `uncertainty_gate`，不是 page 生成阶段。

### deepwiki-rs

deepwiki-rs 会把源码材料给 LLM，但要分阶段看。

`code_purpose_analyze.rs` 给的是：

- 文件路径
- 文件名
- 文件内容预览

`code_analyze.rs` 给的是：

- source summary
- dependency component code snippets

`key_modules_insight.rs` 会把筛出来的相关代码 insight 拼进 domain prompt，其中包含源码摘要。

所以 deepwiki-rs 的模式是：

- 先让 LLM 看代码做 insight / relationship / research
- 再让 compose agent 基于 research 结果写 page

它不是像 CodeWiki 那样在每个最终 page prompt 里都直接塞完整源码。

### CodeWiki

CodeWiki 是最明确“page 生成时直接给源码”的参考实现。

`format_user_prompt(...)` 会按文件分组，把当前模块 core components 所在文件的完整内容直接拼进 prompt：

- `# File: ...`
- `## Core Components in this file`
- `## File Content`
- fenced code block

见 `tmp/upstream/codewiki/codewiki/src/be/prompt_template.py`。

同时复杂模块 agent 还能在运行中通过 `read_code_components` 工具继续读取指定组件的源码，见：

- `tmp/upstream/codewiki/codewiki/src/be/agent_tools/read_code_components.py`
- `tmp/upstream/codewiki/codewiki/src/be/agent_orchestrator.py`

但要注意，父模块 / repo overview 阶段更多是吃：

- `repo_structure`
- child docs

这时不一定再直接给源码正文。

所以 CodeWiki 的特点是：

- 叶子页直接吃源码
- 父页主要吃子页结果

### deepwiki-open

deepwiki-open 更像 query/RAG 系统，不是离线 wiki page 生成器。

它给模型的上下文包括：

- conversation history
- retrieved contexts
- current file content 可选

而 `contexts` 会按文件路径分组，见：

- `tmp/upstream/deepwiki-open/api/prompts.py`
- `tmp/upstream/deepwiki-open/api/simple_chat.py`
- `tmp/upstream/deepwiki-open/api/websocket_wiki.py`

所以它属于：

- 查询回答时会给源码片段
- 不适合直接类比“生成 page 时是否给源码”

### GitNexus

GitNexus 的离线 ingestion 主链是 deterministic 的：

- `processStructure`
- `processParsing`
- `processImports`
- `processCalls`
- `processHeritage`
- `processCommunities`
- `processProcesses`

见 `tmp/upstream/GitNexus/gitnexus/src/core/ingestion/pipeline.ts`。

`cluster-enricher.ts` 给 LLM 的只是：

- heuristic label
- member names
- member types
- file paths

没有源码正文。

但 `gitnexus-web` 的 Graph RAG agent 可以通过工具：

- `read`
- `search`
- `cypher`

去再读源码，见 `tmp/upstream/GitNexus/gitnexus-web/src/core/llm/agent.ts`。

所以 GitNexus 是：

- 建图阶段不给源码到 LLM
- 交互问答阶段按需读源码

### 对当前仓库最直接的启发

如果只围绕“page 生成时要不要给源码”这一个问题，最有参考价值的是两条路线：

1. `CodeWiki` 路线  
   叶子页直接给关键文件源码，父页吃子页结果。

2. `deepwiki-rs` 路线  
   不一定在最终 page 时给完整源码，但必须在 page 之前先形成足够强的 code insight / research layer。

当前仓库现在两条都没完全做到：

- page 阶段不给源码正文
- page 前也还没有足够强的 dossier / insight 层

所以它才会落在一个比较尴尬的位置：

- 比 deterministic 模板强
- 但比 research-driven 文档弱

## 8. 为什么当前仓库会让人感觉“生成 wiki 的逻辑不对”

结合上面四个参考项目，我认为当前仓库最主要的问题有四个。

### 8.1 LLM 介入太晚

当前仓库让 LLM 主要在 `PageEnrichmentInput` 上工作。

这意味着模型拿到的已经是：

- 被 planner 筛过
- 被 context builder 压过
- 被 facts layer 解释过

的二手材料。

模型很难在这个阶段再产出像 reference 那样的“研究感”和“专题感”。

### 8.2 给模型的数据过于聚合

当前仓库 page prompt 里最常见的是：

- `图热点：...`
- `流程：...`
- `社区：...`
- `源码：...`

这种短句摘要。

它们适合稳定渲染，不适合做深度文档生成。

deepwiki-rs 和 CodeWiki 则明显更多地给模型：

- 真正的文件内容
- 依赖代码片段
- 组件级材料
- 子页文档

### 8.3 父页没有真正消费子页产物

CodeWiki 很清楚地把 child docs 喂给 parent overview。

当前仓库虽然有 `child_summaries`，但还远远不是“父页建立在子页之上”。

现在更像：

- 每一页都各自基于一份 facts 再写一遍

这会导致：

- 重复
- 缺乏层级递进
- 父页不像真正的汇总页

### 8.4 缺一个 LLM 可消费的中间研究层

deepwiki-rs 有：

- code insights
- relationship analysis
- research reports

CodeWiki 有：

- cluster module tree
- leaf module docs
- parent docs

当前仓库在 `facts -> page` 之间还缺一个真正的：

- dossier
- insight
- research memo

层。

所以 LLM 现在只能在“页面末端”发力，而不是在“研究中段”发力。

## 9. 对当前仓库最值得做的改法

这部分不是实现方案，只是基于代码对比得出的方向判断。

### 9.1 不建议继续只调 `page_enrichment` prompt

继续改当前 prompt 会有收益，但收益上限不高。

原因很简单：

- 输入材料本身就不够“研究态”
- 模型拿不到足够多的一手结构材料

### 9.2 应该补一个 `Topic/Module Dossier` 中间层

建议在当前 `facts -> planned page -> page_enrichment` 之间增加一层可缓存、可回退、可复用的 dossier：

- source cluster
- key symbols
- cross-module edges
- process candidates
- evidence rollup
- child page rollup
- raw code snippets 或 symbol summaries

这层才应该成为 LLM 的主要输入，而不是直接拿现在的 `PageContext` 去写页。

### 9.3 让父页消费子页结构化结果，而不是只吃轻量摘要

当前 `child_summaries` 太轻。

更合理的是让父页消费：

- child summary
- child evidence rollup
- child diagrams
- child key modules / key sources

这会更接近 CodeWiki 的 parent overview 方式。

### 9.4 给模型更多“有限的一手材料”

不是要把整仓源码全喂进去，而是要像 deepwiki-rs 一样给有限但高价值的一手材料：

- 关键文件正文或稳定截断片段
- 依赖代码片段
- 关键 symbol 列表
- entry-point trace

这比继续堆 `图热点：...` 这种二手短句更有价值。

### 9.5 保留当前结构化 JSON contract，但把输入层升级

当前仓库的一个优点不能丢：

- `prompt_type`
- `input_hash`
- `response_schema`
- 强缓存
- 强回退

这个 contract 本身是对的。

要改的不是 contract，而是 `input` 的材料层级。

## 10. 最终判断

如果只看真实代码，我对五个项目的判断是：

1. 当前仓库最强调“稳定事实 + 结构化回退”，这部分是对的。
2. 但当前仓库把 LLM 放得太后，给的数据又太像压缩过的 facts，所以输出自然更像“整理稿”，不像“研究驱动的 Wiki”。
3. deepwiki-rs 最值得借鉴的是“研究链前置”和“让模型接触依赖代码片段”。
4. CodeWiki 最值得借鉴的是“直接给模块关键源码”和“父页显式消费子页文档”。
5. GitNexus 最值得借鉴的是“先建图事实，再做图解释”和“同类型批量 enrichment”。
6. deepwiki-open 更适合借鉴到 query/RAG 层，不适合直接当 Wiki 生成链模板。

如果基于这份报告收敛成一句最重要的话，那就是：

当前仓库的问题不主要是“提示词差”，而主要是“LLM 接触真实代码和结构材料的时机太晚、粒度太粗”。
