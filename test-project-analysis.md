# Test Project Analysis

生成时间：2026-03-11T03:24:39.614Z
基线命令：`node scripts/run-test-projects.mjs --jobs 1`
说明：项目集脚本当前直接调用 release binary，不会协商 Agent LLM bridge；但如果目标 repo 根存在可用的 `wiki.dev.yaml` provider 配置，core 仍会优先走 provider 直连。当前这份报告默认描述的是未提供 provider dev 覆盖时的 deterministic baseline，LLM 增强正文、provider 优先和双向协议桥接由 `crates/wiki-core/tests/llm_runtime.rs` 与 `agents/codebuddy/src/runtime/invokeCore.test.ts` 单独覆盖。

## 总览

| Project | Pages | Avg lines/page | Avg prose/page | Graph pages | Mermaid | Reference delta |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| aLocal | 7 | 85.6 | 1 | 7/7 | 0 | -79 |
| axum | 7 | 468 | 1 | 7/7 | 0 | -104 |
| bat | 14 | 96.1 | 1 | 14/14 | 0 | -65 |
| chi | 5 | 76.6 | 1 | 5/5 | 0 | -60 |
| cobra | 4 | 69 | 1 | 4/4 | 0 | -34 |
| dagger | 28 | 787.9 | 1 | 28/28 | 0 | -37 |
| django-ninja | 5 | 118.8 | 1 | 5/5 | 0 | n/a |
| docker-mailserver | 4 | 24.5 | 1 | 1/4 | 0 | n/a |
| fastapi | 7 | 164 | 1 | 7/7 | 0 | n/a |
| gin | 7 | 132.3 | 1 | 7/7 | 0 | n/a |
| httpx | 5 | 93.6 | 1 | 5/5 | 0 | n/a |
| leakcanary | 10 | 297.6 | 1 | 10/10 | 0 | n/a |
| pinia | 16 | 86.3 | 1 | 16/16 | 0 | -45 |
| restaurant-app | 15 | 105.5 | 1 | 15/15 | 0 | -112 |
| spec-wiki | 8 | 226.1 | 1 | 8/8 | 0 | n/a |
| spring-petclinic | 4 | 39.5 | 1 | 4/4 | 0 | n/a |
| storybook | 68 | 503.3 | 1 | 68/68 | 0 | -108 |
| wot-starter | 8 | 70.4 | 1 | 8/8 | 0 | n/a |
| zustand | 4 | 42.8 | 1 | 4/4 | 0 | -93 |

## aLocal

- 页面密度：7 页，平均 85.6 行/页（密度高），其中段落 1 行/页、列表 69.4 行/页；最长页面是 `模块：spider`（191 行）。
- 图事实落页：7/7 页面包含 graph facts，总计 381 行；概述=76、架构=9、工作流=22，Mermaid 0 个。
- Query/图验证：符号 query 以 `get_html` 为样本，命中 24 个符号、6 个页面；图 query 以 `getLog` 为样本，扩展 14 条图边、5 个社区、0 个流程。
- Reference 对照：与 reference 相比明显压缩（7 vs 86，-79 页），当前仍以 repo 级总览 + 模块页为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## axum

- 页面密度：7 页，平均 468 行/页（密度高），其中段落 1 行/页、列表 451.9 行/页；最长页面是 `模块：axum`（1248 行）。
- 图事实落页：7/7 页面包含 graph facts，总计 3038 行；概述=443、架构=21、工作流=227，Mermaid 0 个。
- Query/图验证：符号 query 以 `echo_app` 为样本，命中 24 个符号、7 个页面；图 query 以 `as_mut` 为样本，扩展 110 条图边、14 个社区、0 个流程。
- Reference 对照：与 reference 相比明显压缩（7 vs 111，-104 页），当前仍以 repo 级总览 + 模块页为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## bat

- 页面密度：14 页，平均 96.1 行/页（密度高），其中段落 1 行/页、列表 79.5 行/页；最长页面是 `项目概述`（273 行）。
- 图事实落页：14/14 页面包含 graph facts，总计 949 行；概述=224、架构=9、工作流=67，Mermaid 0 个。
- Query/图验证：符号 query 以 `assert_c` 为样本，命中 24 个符号、14 个页面；图 query 以 `detect` 为样本，扩展 352 条图边、6 个社区、1 个流程。
- Reference 对照：与 reference 相比明显压缩（14 vs 79，-65 页），当前仍以 repo 级总览 + 模块页为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## chi

- 页面密度：5 页，平均 76.6 行/页（密度中），其中段落 1 行/页、列表 60.8 行/页；最长页面是 `项目概述`（178 行）。
- 图事实落页：5/5 页面包含 graph facts，总计 189 行；概述=91、架构=6、工作流=36，Mermaid 0 个。
- Query/图验证：符号 query 以 `paginate` 为样本，命中 1 个符号、4 个页面；图 query 以 `paginate` 为样本，扩展 2 条图边、2 个社区、2 个流程。
- Reference 对照：与 reference 相比明显压缩（5 vs 65，-60 页），当前仍以 repo 级总览 + 模块页为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## cobra

- 页面密度：4 页，平均 69 行/页（密度中），其中段落 1 行/页、列表 53.5 行/页；最长页面是 `项目概述`（122 行）。
- 图事实落页：4/4 页面包含 graph facts，总计 180 行；概述=98、架构=6、工作流=42，Mermaid 0 个。
- Query/图验证：符号 query 以 `findFlag` 为样本，命中 1 个符号、3 个页面；图 query 以 `genZshComp` 为样本，扩展 20 条图边、10 个社区、3 个流程。
- Reference 对照：与 reference 相比明显压缩（4 vs 38，-34 页），当前仍以 repo 级总览 + 模块页为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## dagger

- 页面密度：28 页，平均 787.9 行/页（密度高），其中段落 1 行/页、列表 771.1 行/页；最长页面是 `项目概述`（1743 行）。
- 图事实落页：28/28 页面包含 graph facts，总计 21029 行；概述=1708、架构=99、工作流=747，Mermaid 0 个。
- Query/图验证：符号 query 以 `factoryOf` 为样本，命中 1 个符号、7 个页面；图 query 以 `addAll` 为样本，扩展 117 条图边、20 个社区、0 个流程。
- Reference 对照：与 reference 接近（28 vs 65，-37 页）。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## django-ninja

- 页面密度：5 页，平均 118.8 行/页（密度高），其中段落 1 行/页、列表 103 行/页；最长页面是 `项目概述`（264 行）。
- 图事实落页：5/5 页面包含 graph facts，总计 438 行；概述=217、架构=8、工作流=71，Mermaid 0 个。
- Query/图验证：符号 query 以 `clean_db` 为样本，命中 1 个符号、2 个页面；图 query 以 `filter` 为样本，扩展 882 条图边、3 个社区、0 个流程。
- Reference 对照：无 reference，对照以 query 与 graph 命中为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## docker-mailserver

- 页面密度：4 页，平均 24.5 行/页（密度低），其中段落 1 行/页、列表 9 行/页；最长页面是 `模块：docs`（29 行）。
- 图事实落页：1/4 页面包含 graph facts，总计 2 行；概述=0、架构=2、工作流=0，Mermaid 0 个。
- Query/图验证：无可用符号 query 样本。
- Reference 对照：无 reference，对照以 query 与 graph 命中为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## fastapi

- 页面密度：7 页，平均 164 行/页（密度高），其中段落 1 行/页、列表 147.9 行/页；最长页面是 `项目概述`（459 行）。
- 图事实落页：7/7 页面包含 graph facts，总计 884 行；概述=360、架构=6、工作流=110，Mermaid 0 个。
- Query/图验证：符号 query 以 `add_task` 为样本，命中 24 个符号、6 个页面；图 query 以 `get_db` 为样本，扩展 127 条图边、14 个社区、0 个流程。
- Reference 对照：无 reference，对照以 query 与 graph 命中为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## gin

- 页面密度：7 页，平均 132.3 行/页（密度高），其中段落 1 行/页、列表 116.1 行/页；最长页面是 `项目概述`（301 行）。
- 图事实落页：7/7 页面包含 graph facts，总计 725 行；概述=266、架构=6、工作流=97，Mermaid 0 个。
- Query/图验证：符号 query 以 `function` 为样本，命中 24 个符号、5 个页面；图 query 以 `mapURI` 为样本，扩展 8 条图边、3 个社区、0 个流程。
- Reference 对照：无 reference，对照以 query 与 graph 命中为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## httpx

- 页面密度：5 页，平均 93.6 行/页（密度高），其中段落 1 行/页、列表 77.8 行/页；最长页面是 `项目概述`（203 行）。
- 图事实落页：5/5 页面包含 graph facts，总计 338 行；概述=174、架构=6、工作流=58，Mermaid 0 个。
- Query/图验证：符号 query 以 `compress` 为样本，命中 1 个符号、4 个页面；图 query 以 `netloc` 为样本，扩展 4 条图边、2 个社区、0 个流程。
- Reference 对照：无 reference，对照以 query 与 graph 命中为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## leakcanary

- 页面密度：10 页，平均 297.6 行/页（密度高），其中段落 1 行/页、列表 281.2 行/页；最长页面是 `项目概述`（852 行）。
- 图事实落页：10/10 页面包含 graph facts，总计 2674 行；概述=821、架构=13、工作流=237，Mermaid 0 个。
- Query/图验证：符号 query 以 `openFile` 为样本，命中 1 个符号、3 个页面；图 query 以 `minRun` 为样本，扩展 5 条图边、1 个社区、1 个流程。
- Reference 对照：无 reference，对照以 query 与 graph 命中为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## pinia

- 页面密度：16 页，平均 86.3 行/页（密度高），其中段落 1 行/页、列表 69.6 行/页；最长页面是 `模块：pinia`（240 行）。
- 图事实落页：16/16 页面包含 graph facts，总计 875 行；概述=104、架构=24、工作流=43，Mermaid 0 个。
- Query/图验证：符号 query 以 `copyFile` 为样本，命中 1 个符号、4 个页面；图 query 以 `toYAML` 为样本，扩展 4 条图边、2 个社区、1 个流程。
- Reference 对照：与 reference 接近（16 vs 61，-45 页）。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## restaurant-app

- 页面密度：15 页，平均 105.5 行/页（密度高），其中段落 1 行/页、列表 88.9 行/页；最长页面是 `项目概述`（288 行）。
- 图事实落页：15/15 页面包含 graph facts，总计 1023 行；概述=199、架构=19、工作流=54，Mermaid 0 个。
- Query/图验证：符号 query 以 `get_dish` 为样本，命中 24 个符号、14 个页面；图 query 以 `rocket` 为样本，扩展 2 条图边、1 个社区、0 个流程。
- Reference 对照：与 reference 相比明显压缩（15 vs 127，-112 页），当前仍以 repo 级总览 + 模块页为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## spec-wiki

- 页面密度：8 页，平均 226.1 行/页（密度高），其中段落 1 行/页、列表 209.9 行/页；最长页面是 `模块：wiki-core`（1199 行）。
- 图事实落页：8/8 页面包含 graph facts，总计 1558 行；概述=174、架构=9、工作流=45，Mermaid 0 个。
- Query/图验证：符号 query 以 `callCore` 为样本，命中 1 个符号、3 个页面；图 query 以 `sample` 为样本，扩展 15 条图边、3 个社区、3 个流程。
- Reference 对照：无 reference，对照以 query 与 graph 命中为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## spring-petclinic

- 页面密度：4 页，平均 39.5 行/页（密度中），其中段落 1 行/页、列表 24 行/页；最长页面是 `项目概述`（66 行）。
- 图事实落页：4/4 页面包含 graph facts，总计 58 行；概述=38、架构=6、工作流=11，Mermaid 0 个。
- Query/图验证：符号 query 以 `findById` 为样本，命中 1 个符号、3 个页面；图 query 以 `findPet` 为样本，扩展 13 条图边、1 个社区、0 个流程。
- Reference 对照：无 reference，对照以 query 与 graph 命中为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## storybook

- 页面密度：68 页，平均 503.3 行/页（密度高），其中段落 1 行/页、列表 486.4 行/页；最长页面是 `模块：core`（2952 行）。
- 图事实落页：68/68 页面包含 graph facts，总计 31015 行；概述=1172、架构=495、工作流=376，Mermaid 0 个。
- Query/图验证：符号 query 以 `addStats` 为样本，命中 1 个符号、24 个页面；图 query 以 `delete` 为样本，扩展 56 条图边、19 个社区、1 个流程。
- Reference 对照：与 reference 接近（68 vs 176，-108 页）。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## wot-starter

- 页面密度：8 页，平均 70.4 行/页（密度中），其中段落 1 行/页、列表 54.1 行/页；最长页面是 `项目概述`（148 行）。
- 图事实落页：8/8 页面包含 graph facts，总计 296 行；概述=81、架构=6、工作流=35，Mermaid 0 个。
- Query/图验证：符号 query 以 `callback` 为样本，命中 1 个符号、5 个页面；图 query 以 `imgTap` 为样本，扩展 7 条图边、1 个社区、0 个流程。
- Reference 对照：无 reference，对照以 query 与 graph 命中为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

## zustand

- 页面密度：4 页，平均 42.8 行/页（密度中），其中段落 1 行/页、列表 27.3 行/页；最长页面是 `项目概述`（70 行）。
- 图事实落页：4/4 页面包含 graph facts，总计 69 行；概述=41、架构=6、工作流=15，Mermaid 0 个。
- Query/图验证：符号 query 以 `external` 为样本，命中 1 个符号、4 个页面；图 query 以 `reviver` 为样本，扩展 3 条图边、1 个社区、1 个流程。
- Reference 对照：与 reference 相比明显压缩（4 vs 97，-93 页），当前仍以 repo 级总览 + 模块页为主。
- 增强观测：项目集仍以 deterministic fallback 页面为主，段落化增强和 Mermaid 没有在这条验证路径中出现；这通常意味着目标 repo 没有提供可用的 provider 直连配置，同时脚本本身也没有协商 Agent LLM bridge。

