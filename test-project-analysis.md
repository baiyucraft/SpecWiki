# Test Project Analysis

生成时间：2026-03-12T09:44:48.960Z
基线命令：`node scripts/run-test-projects.mjs --jobs 1`
说明：项目集脚本当前直接调用 release binary，不会协商 Agent LLM bridge；如果目标 repo 根存在可用的 `wiki.dev.yaml` provider 配置，core 会优先走 provider 直连。这份报告按当前 `tmp/test/*/.wiki` 实际产物统计 section plan、research 命中、精准 evidence 与 reference 差异。

## 总览

| Project | Pages | Avg lines/page | Avg prose/page | Topic pages | Section-plan pages | Core research | Precise evidence | Mermaid | Reference delta |
| --- | ---: | ---: | ---: | ---: | ---: | --- | ---: | ---: | --- |
| aLocal | 21 | 40.9 | 7.4 | 14 | 6 | - | 71/71 | 14 | -65 |
| axum | 16 | 60.2 | 7.8 | 9 | 6 | - | 64/64 | 12 | -95 |
| bat | 27 | 50.6 | 7.3 | 13 | 6 | - | 73/73 | 26 | -52 |
| chi | 16 | 39.8 | 6.9 | 12 | 6 | - | 71/71 | 3 | -49 |
| cobra | 17 | 34.3 | 6.1 | 13 | 6 | - | 27/27 | 9 | -21 |
| dagger | 52 | 75.4 | 7.6 | 24 | 6 | - | 246/246 | 44 | -13 |
| django-ninja | 18 | 43.4 | 7.3 | 13 | 6 | - | 50/50 | 10 | -29 |
| docker-mailserver | 5 | 35.2 | 11.8 | 2 | 4 | overview/architecture | 5/5 | 0 | -74 |
| fastapi | 15 | 43.1 | 6.9 | 9 | 6 | - | 62/62 | 3 | -87 |
| gin | 20 | 40.3 | 6.9 | 14 | 6 | - | 71/71 | 9 | -68 |
| httpx | 15 | 40.4 | 7.1 | 11 | 6 | - | 21/21 | 8 | -20 |
| leakcanary | 23 | 48.4 | 7 | 14 | 6 | - | 74/74 | 15 | n/a |
| pinia | 33 | 51.7 | 8.9 | 17 | 5 | - | 145/145 | 37 | -28 |
| restaurant-app | 35 | 51 | 8.1 | 20 | 6 | - | 179/179 | 35 | -92 |
| spec-wiki | 23 | 71.3 | 7.6 | 15 | 5 | - | 89/89 | 20 | n/a |
| spring-petclinic | 16 | 26.9 | 5.4 | 12 | 6 | - | 19/19 | 3 | n/a |
| storybook | 124 | 94.9 | 8.4 | 57 | 6 | - | 707/707 | 120 | -52 |
| wot-starter | 22 | 39 | 7 | 14 | 6 | - | 93/93 | 6 | n/a |
| zustand | 14 | 39.1 | 7.9 | 10 | 6 | - | 20/20 | 7 | -83 |

## aLocal

- 页面密度：21 页，平均 40.9 行/页（密度中），其中段落 7.4 行/页、列表 14.2 行/页；最长页面是 `项目概述`（82 行）。
- 主题与 evidence：专题页 14 个，evidence 落页 15 页/21 页，总计 19 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：71/71 条 evidence 带真实行号，密度 1。
- 图事实落页：8/21 页面包含 graph facts，总计 60 行；概述=8、架构=7、工作流=22，Mermaid 14 个，落在 11 页。
- Query/图验证：符号 query 以 `get_html` 为样本，命中 24 个符号、18 个页面；图 query 以 `getLog` 为样本，扩展 14 条图边、5 个社区、0 个流程。
- LLM cache：file_purpose(81)、page_research(6)、page_enrichment(2)
- Reference 对照：与 reference 相比明显压缩（21 vs 86，-65 页），当前仍以 repo 级总览 + 模块页为主。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## axum

- 页面密度：16 页，平均 60.2 行/页（密度中），其中段落 7.8 行/页、列表 27.1 行/页；最长页面是 `工作流与部署`（127 行）。
- 主题与 evidence：专题页 9 个，evidence 落页 11 页/16 页，总计 13 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：64/64 条 evidence 带真实行号，密度 1。
- 图事实落页：7/16 页面包含 graph facts，总计 154 行；概述=8、架构=7、工作流=111，Mermaid 12 个，落在 11 页。
- Query/图验证：符号 query 以 `echo_app` 为样本，命中 24 个符号、11 个页面；图 query 以 `as_mut` 为样本，扩展 110 条图边、14 个社区、0 个流程。
- LLM cache：file_purpose(86)、page_research(6)、page_enrichment(2)
- Reference 对照：与 reference 相比明显压缩（16 vs 111，-95 页），当前仍以 repo 级总览 + 模块页为主。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## bat

- 页面密度：27 页，平均 50.6 行/页（密度中），其中段落 7.3 行/页、列表 12.3 行/页；最长页面是 `流程主题：run flow`（106 行）。
- 主题与 evidence：专题页 13 个，evidence 落页 16 页/27 页，总计 16 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：73/73 条 evidence 带真实行号，密度 1。
- 图事实落页：9/27 页面包含 graph facts，总计 103 行；概述=8、架构=7、工作流=58，Mermaid 26 个，落在 18 页。
- Query/图验证：符号 query 以 `assert_c` 为样本，命中 24 个符号、21 个页面；图 query 以 `detect` 为样本，扩展 352 条图边、6 个社区、1 个流程。
- LLM cache：file_purpose(128)、page_research(6)、page_enrichment(2)
- Reference 对照：与 reference 接近（27 vs 79，-52 页）。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## chi

- 页面密度：16 页，平均 39.8 行/页（密度中），其中段落 6.9 行/页、列表 15.8 行/页；最长页面是 `项目概述`（78 行）。
- 主题与 evidence：专题页 12 个，evidence 落页 14 页/16 页，总计 16 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：71/71 条 evidence 带真实行号，密度 1。
- 图事实落页：5/16 页面包含 graph facts，总计 61 行；概述=8、架构=7、工作流=32，Mermaid 3 个，落在 2 页。
- Query/图验证：符号 query 以 `paginate` 为样本，命中 1 个符号、14 个页面；图 query 以 `paginate` 为样本，扩展 2 条图边、2 个社区、2 个流程。
- LLM cache：file_purpose(28)、page_research(6)、page_enrichment(2)
- Reference 对照：与 reference 相比明显压缩（16 vs 65，-49 页），当前仍以 repo 级总览 + 模块页为主。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## cobra

- 页面密度：17 页，平均 34.3 行/页（密度低），其中段落 6.1 行/页、列表 10.1 行/页；最长页面是 `项目概述`（76 行）。
- 主题与 evidence：专题页 13 个，evidence 落页 5 页/17 页，总计 5 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：27/27 条 evidence 带真实行号，密度 1。
- 图事实落页：6/17 页面包含 graph facts，总计 66 行；概述=8、架构=7、工作流=42，Mermaid 9 个，落在 8 页。
- Query/图验证：符号 query 以 `findFlag` 为样本，命中 1 个符号、7 个页面；图 query 以 `genZshComp` 为样本，扩展 20 条图边、10 个社区、3 个流程。
- LLM cache：file_purpose(42)、page_research(6)、page_enrichment(2)
- Reference 对照：与 reference 接近（17 vs 38，-21 页）。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## dagger

- 页面密度：52 页，平均 75.4 行/页（密度中），其中段落 7.6 行/页、列表 33 行/页；最长页面是 `工作流与部署`（501 行）。
- 主题与 evidence：专题页 24 个，evidence 落页 48 页/52 页，总计 49 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：246/246 条 evidence 带真实行号，密度 1。
- 图事实落页：30/52 页面包含 graph facts，总计 678 行；概述=8、架构=7、工作流=485，Mermaid 44 个，落在 43 页。
- Query/图验证：符号 query 以 `factoryOf` 为样本，命中 1 个符号、9 个页面；图 query 以 `addAll` 为样本，扩展 117 条图边、20 个社区、0 个流程。
- LLM cache：file_purpose(128)、page_research(6)、page_enrichment(2)
- Reference 对照：与 reference 接近（52 vs 65，-13 页）。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## django-ninja

- 页面密度：18 页，平均 43.4 行/页（密度中），其中段落 7.3 行/页、列表 14.6 行/页；最长页面是 `工作流与部署`（92 行）。
- 主题与 evidence：专题页 13 个，evidence 落页 13 页/18 页，总计 16 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：50/50 条 evidence 带真实行号，密度 1。
- 图事实落页：6/18 页面包含 graph facts，总计 89 行；概述=8、架构=7、工作流=59，Mermaid 10 个，落在 9 页。
- Query/图验证：符号 query 以 `clean_db` 为样本，命中 1 个符号、2 个页面；图 query 以 `filter` 为样本，扩展 882 条图边、3 个社区、0 个流程。
- LLM cache：file_purpose(111)、page_research(6)、page_enrichment(2)
- Reference 对照：与 reference 接近（18 vs 47，-29 页）。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## docker-mailserver

- 页面密度：5 页，平均 35.2 行/页（密度中），其中段落 11.8 行/页、列表 9.8 行/页；最长页面是 `项目概述`（59 行）。
- 主题与 evidence：专题页 2 个，evidence 落页 3 页/5 页，总计 7 个 evidence block。
- Research 命中：section-plan 页 4 个，overview=命中，architecture=命中，repo-archetype 专题 0 个。
- 精准 evidence：5/5 条 evidence 带真实行号，密度 1。
- 图事实落页：1/5 页面包含 graph facts，总计 1 行；概述=0、架构=1、工作流=0，Mermaid 0 个，落在 0 页。
- Query/图验证：无可用符号 query 样本。
- LLM cache：file_purpose(36)、page_research(4)、page_enrichment(3)、top_level_promotion(2)
- Reference 对照：与 reference 相比明显压缩（5 vs 79，-74 页），当前仍以 repo 级总览 + 模块页为主。
- 增强观测：已有 4 页缓存了 section plan；overview research=命中，architecture research=命中。

## fastapi

- 页面密度：15 页，平均 43.1 行/页（密度中），其中段落 6.9 行/页、列表 19.7 行/页；最长页面是 `工作流与部署`（110 行）。
- 主题与 evidence：专题页 9 个，evidence 落页 11 页/15 页，总计 12 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：62/62 条 evidence 带真实行号，密度 1。
- 图事实落页：7/15 页面包含 graph facts，总计 115 行；概述=8、架构=7、工作流=78，Mermaid 3 个，落在 2 页。
- Query/图验证：符号 query 以 `add_task` 为样本，命中 24 个符号、4 个页面；图 query 以 `get_db` 为样本，扩展 127 条图边、14 个社区、0 个流程。
- LLM cache：file_purpose(128)、page_research(6)、page_enrichment(2)
- Reference 对照：与 reference 相比明显压缩（15 vs 102，-87 页），当前仍以 repo 级总览 + 模块页为主。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## gin

- 页面密度：20 页，平均 40.3 行/页（密度中），其中段落 6.9 行/页、列表 14.9 行/页；最长页面是 `工作流与部署`（103 行）。
- 主题与 evidence：专题页 14 个，evidence 落页 17 页/20 页，总计 17 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：71/71 条 evidence 带真实行号，密度 1。
- 图事实落页：7/20 页面包含 graph facts，总计 108 行；概述=8、架构=7、工作流=71，Mermaid 9 个，落在 7 页。
- Query/图验证：符号 query 以 `function` 为样本，命中 24 个符号、17 个页面；图 query 以 `mapURI` 为样本，扩展 8 条图边、3 个社区、0 个流程。
- LLM cache：file_purpose(102)、page_research(6)、page_enrichment(2)
- Reference 对照：与 reference 相比明显压缩（20 vs 88，-68 页），当前仍以 repo 级总览 + 模块页为主。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## httpx

- 页面密度：15 页，平均 40.4 行/页（密度中），其中段落 7.1 行/页、列表 9.8 行/页；最长页面是 `流程主题：async_auth_flow flow`（88 行）。
- 主题与 evidence：专题页 11 个，evidence 落页 11 页/15 页，总计 12 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：21/21 条 evidence 带真实行号，密度 1。
- 图事实落页：7/15 页面包含 graph facts，总计 72 行；概述=7、架构=7、工作流=49，Mermaid 8 个，落在 5 页。
- Query/图验证：符号 query 以 `compress` 为样本，命中 1 个符号、2 个页面；图 query 以 `netloc` 为样本，扩展 4 条图边、2 个社区、0 个流程。
- LLM cache：file_purpose(46)、page_research(6)、top_level_promotion(3)、page_enrichment(2)
- Reference 对照：与 reference 接近（15 vs 35，-20 页）。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## leakcanary

- 页面密度：23 页，平均 48.4 行/页（密度中），其中段落 7 行/页、列表 21 行/页；最长页面是 `工作流与部署`（186 行）。
- 主题与 evidence：专题页 14 个，evidence 落页 19 页/23 页，总计 20 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：74/74 条 evidence 带真实行号，密度 1。
- 图事实落页：13/23 页面包含 graph facts，总计 233 行；概述=8、架构=7、工作流=170，Mermaid 15 个，落在 14 页。
- Query/图验证：符号 query 以 `openFile` 为样本，命中 1 个符号、3 个页面；图 query 以 `minRun` 为样本，扩展 5 条图边、1 个社区、1 个流程。
- LLM cache：file_purpose(128)、page_research(6)、page_enrichment(2)
- Reference 对照：无 reference，对照以 query 与 graph 命中为主。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## pinia

- 页面密度：33 页，平均 51.7 行/页（密度中），其中段落 8.9 行/页、列表 18 行/页；最长页面是 `模块：pinia`（108 行）。
- 主题与 evidence：专题页 17 个，evidence 落页 29 页/33 页，总计 35 个 evidence block。
- Research 命中：section-plan 页 5 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：145/145 条 evidence 带真实行号，密度 1。
- 图事实落页：13/33 页面包含 graph facts，总计 109 行；概述=7、架构=7、工作流=33，Mermaid 37 个，落在 26 页。
- Query/图验证：符号 query 以 `copyFile` 为样本，命中 1 个符号、6 个页面；图 query 以 `toYAML` 为样本，扩展 4 条图边、2 个社区、1 个流程。
- LLM cache：file_purpose(128)、page_research(5)、page_enrichment(2)
- Reference 对照：与 reference 接近（33 vs 61，-28 页）。
- 增强观测：已有 5 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## restaurant-app

- 页面密度：35 页，平均 51 行/页（密度中），其中段落 8.1 行/页、列表 19.7 行/页；最长页面是 `工作流与部署`（106 行）。
- 主题与 evidence：专题页 20 个，evidence 落页 30 页/35 页，总计 34 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：179/179 条 evidence 带真实行号，密度 1。
- 图事实落页：15/35 页面包含 graph facts，总计 145 行；概述=8、架构=7、工作流=49，Mermaid 35 个，落在 26 页。
- Query/图验证：符号 query 以 `get_dish` 为样本，命中 24 个符号、23 个页面；图 query 以 `rocket` 为样本，扩展 2 条图边、1 个社区、0 个流程。
- LLM cache：file_purpose(128)、page_research(6)、page_enrichment(2)
- Reference 对照：与 reference 接近（35 vs 127，-92 页）。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## spec-wiki

- 页面密度：23 页，平均 71.3 行/页（密度中），其中段落 7.6 行/页、列表 13.9 行/页；最长页面是 `流程主题：run_init flow`（239 行）。
- 主题与 evidence：专题页 15 个，evidence 落页 18 页/23 页，总计 22 个 evidence block。
- Research 命中：section-plan 页 5 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：89/89 条 evidence 带真实行号，密度 1。
- 图事实落页：6/23 页面包含 graph facts，总计 71 行；概述=6、架构=7、工作流=43，Mermaid 20 个，落在 16 页。
- Query/图验证：符号 query 以 `callCore` 为样本，命中 2 个符号、10 个页面；图 query 以 `sample` 为样本，扩展 22 条图边、3 个社区、3 个流程。
- LLM cache：file_purpose(43)、page_research(6)、page_enrichment(2)
- Reference 对照：无 reference，对照以 query 与 graph 命中为主。
- 增强观测：已有 5 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## spring-petclinic

- 页面密度：16 页，平均 26.9 行/页（密度低），其中段落 5.4 行/页、列表 6.7 行/页；最长页面是 `项目概述`（65 行）。
- 主题与 evidence：专题页 12 个，evidence 落页 5 页/16 页，总计 5 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：19/19 条 evidence 带真实行号，密度 1。
- 图事实落页：6/16 页面包含 graph facts，总计 29 行；概述=5、架构=7、工作流=11，Mermaid 3 个，落在 3 页。
- Query/图验证：符号 query 以 `findById` 为样本，命中 1 个符号、5 个页面；图 query 以 `findPet` 为样本，扩展 13 条图边、1 个社区、0 个流程。
- LLM cache：file_purpose(91)、page_research(6)、page_enrichment(2)、top_level_promotion(1)
- Reference 对照：无 reference，对照以 query 与 graph 命中为主。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## storybook

- 页面密度：124 页，平均 94.9 行/页（密度高），其中段落 8.4 行/页、列表 37 行/页；最长页面是 `系统架构`（869 行）。
- 主题与 evidence：专题页 57 个，evidence 落页 120 页/124 页，总计 122 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：707/707 条 evidence 带真实行号，密度 1。
- 图事实落页：67/124 页面包含 graph facts，总计 831 行；概述=8、架构=7、工作流=308，Mermaid 120 个，落在 109 页。
- Query/图验证：符号 query 以 `addStats` 为样本，命中 1 个符号、26 个页面；图 query 以 `delete` 为样本，扩展 56 条图边、19 个社区、1 个流程。
- LLM cache：file_purpose(128)、page_research(6)、page_enrichment(2)
- Reference 对照：与 reference 接近（124 vs 176，-52 页）。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## wot-starter

- 页面密度：22 页，平均 39 行/页（密度中），其中段落 7 行/页、列表 14.9 行/页；最长页面是 `项目概述`（80 行）。
- 主题与 evidence：专题页 14 个，evidence 落页 18 页/22 页，总计 23 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：93/93 条 evidence 带真实行号，密度 1。
- 图事实落页：7/22 页面包含 graph facts，总计 71 行；概述=8、架构=7、工作流=34，Mermaid 6 个，落在 5 页。
- Query/图验证：符号 query 以 `callback` 为样本，命中 1 个符号、7 个页面；图 query 以 `imgTap` 为样本，扩展 7 条图边、1 个社区、0 个流程。
- LLM cache：file_purpose(84)、page_research(6)、page_enrichment(2)
- Reference 对照：无 reference，对照以 query 与 graph 命中为主。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

## zustand

- 页面密度：14 页，平均 39.1 行/页（密度中），其中段落 7.9 行/页、列表 9.6 行/页；最长页面是 `流程主题：persistImpl flow`（71 行）。
- 主题与 evidence：专题页 10 个，evidence 落页 10 页/14 页，总计 16 个 evidence block。
- Research 命中：section-plan 页 6 个，overview=未命中，architecture=未命中，repo-archetype 专题 0 个。
- 精准 evidence：20/20 条 evidence 带真实行号，密度 1。
- 图事实落页：6/14 页面包含 graph facts，总计 39 行；概述=8、架构=7、工作流=15，Mermaid 7 个，落在 4 页。
- Query/图验证：符号 query 以 `external` 为样本，命中 1 个符号、5 个页面；图 query 以 `reviver` 为样本，扩展 3 条图边、1 个社区、1 个流程。
- LLM cache：file_purpose(18)、page_research(6)、page_enrichment(2)、top_level_promotion(1)
- Reference 对照：与 reference 相比明显压缩（14 vs 97，-83 页），当前仍以 repo 级总览 + 模块页为主。
- 增强观测：已有 6 页缓存了 section plan；overview research=未命中，architecture research=未命中。

