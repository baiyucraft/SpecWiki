# Test Project Analysis

生成时间：2026-03-19T23:04:23.157Z
说明：当前报告基于 2.0 的 `knowledge_units / research_cache / page_drafts / unit_runtime_gates / wiki_pages`、`runtime_meta.pipeline_runtime_summary` 与最终 `.wiki/*.md` 读取，不再依赖旧 `page_context_cache.context.research_result` 或 `topic_dossier`。

## 总览

| Project | Runtime | Pages | KnowledgeUnits | UnitResearch | SectionPlan | PageDrafts | WikiPages | TopicPages | Reference Delta |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| storybook | ready | 195 | 195 | 195 | 195 | 195 | 195 | 5 | generated=195 / reference=176（delta=+19） |
| dagger | ready | 64 | 64 | 64 | 64 | 64 | 64 | 7 | generated=64 / reference=65（delta=-1） |

## storybook

- runtime：ready / acceptance_candidate / ready
- runtime gate：summary_state=completed，gate_total=195，compose_ready=0，compose_blocked=0，assemble_done=195
- research runtime：current=n/a，last_elapsed=37444ms，provider=unit=unit-e50a8961f419, stop=no_further_tool_calls, turns=1, tools=0, elapsed=37444ms, cache_hit=false, mode=native_tools
- parent contract：parent_pages=7，compose_ready_parents=7，child_digest_parents=7，missing_readiness_parents=0
- 2.0 pipeline：knowledge_units=195，knowledge_domains=14，unit_research=195，section_plan_units=195，page_drafts=195，wiki_pages=195，markdown=195
- 覆盖率：research=1，compose=1，assemble=1
- UnitType：ConceptGuide(66)、ModuleDoc(46)、ConfigDoc(26)、ExampleDoc(19)、ApiDoc(18)、IntegrationDoc(9)、DomainIndex(5)、TestDoc(3)
- DomainType：CoreRuntime(2)、ApiReference(1)、BuildSystem(1)、ConceptGuide(1)、ConfigReference(1)、DevTooling(1)、Framework(1)、MultiFramework(1)
- ResearchType：unit(195)、domain(14)、system(1)
- 页面质量：topic_pages=5，evidence_pages=194，mermaid_pages=190，avg_lines=186.24，avg_prose=90.1
- 页面分解信号：docs-guide(195)、troubleshooting(119)、integration-platform(112)、config-surface(110)、testing(83)、example-tutorial(57)、api-surface(54)、runtime(43)
- 图事实/符号：symbols=6790，edges=26064，communities=794，processes=8
- 语言分布：typescript(6461)、javascript(329)
- reference：generated=195 / reference=176（delta=+19）

## dagger

- runtime：ready / acceptance_candidate / ready
- runtime gate：summary_state=completed，gate_total=64，compose_ready=0，compose_blocked=0，assemble_done=64
- research runtime：current=n/a，last_elapsed=19402ms，provider=unit=unit-e50a8961f419, stop=no_further_tool_calls, turns=1, tools=0, elapsed=19402ms, cache_hit=false, mode=native_tools
- parent contract：parent_pages=3，compose_ready_parents=3，child_digest_parents=3，missing_readiness_parents=0
- 2.0 pipeline：knowledge_units=64，knowledge_domains=11，unit_research=64，section_plan_units=64，page_drafts=64，wiki_pages=64，markdown=64
- 覆盖率：research=1，compose=1，assemble=1
- UnitType：ModuleDoc(35)、ApiDoc(15)、TestDoc(8)、ConceptGuide(2)、Architecture(1)、ConfigDoc(1)、DomainIndex(1)、Overview(1)
- DomainType：CoreRuntime(2)、ApiReference(1)、BuildSystem(1)、CompilerToolchain(1)、ConceptGuide(1)、ConfigReference(1)、Framework(1)、PlatformBinding(1)
- ResearchType：unit(64)、domain(11)、system(1)
- 页面质量：topic_pages=7，evidence_pages=63，mermaid_pages=63，avg_lines=167.34，avg_prose=65.53
- 页面分解信号：docs-guide(64)、integration-platform(54)、compiler-pipeline(51)、testing(46)、runtime(36)、api-surface(30)、example-tutorial(29)、config-surface(8)
- 图事实/符号：symbols=17054，edges=75773，communities=982，processes=8
- 语言分布：java(13108)、kotlin(3934)、python(12)
- reference：generated=64 / reference=65（delta=-1）

