# storybook Reference Fidelity Report

- project：storybook
- status：ready
- generated_pages：195
- reference_pages：176

## Run Metrics

- run_label：reuse
- cache_mode：preserve
- usage：requests=0, total_tokens=0, page_research=0, page_enrichment=0

## Runtime Metrics

- runtime_state：ready
- baseline_class：acceptance_candidate
- incomplete_reason：n/a
- db_counts：knowledge_units=195, knowledge_domains=14, research_cache=210, page_digests=195, page_drafts=195, unit_runtime_gates=195, wiki_pages=195, pipeline_checkpoint=0
- pipeline_runtime_summary：state=completed, researched=0, compose_ready=0, composed=0, assembled=195
- unit_runtime_gates：total=195, compose_ready=0, compose_pending=0, compose_blocked=0, assemble_done=195
- parent_contract：parents=7, compose_ready_parents=7, child_digest_parents=7, missing_readiness_parents=0
- stop_reasons：completed(23)、no_further_tool_calls(170)、not_run(2)

## Fidelity Gate

- decision：not-pass
- reason：overall=92.05% / reuse_overage=123 / median_skeleton=0.13 / median_key_source=0.03 / warm_stable=true
- overall_match_rate：92.05%
- reuse_overage：123
- median_skeleton_fidelity：0.13
- median_key_source_coverage：0.03

## 四个专项问题

- 页数是否接近 reference：matched 162/176，missing=14
- 是否存在 coarse page reuse：reuse_pages=14，severe_reuse_pages=10，reuse_overage=123
- docs-backed 页面是否具备 reference 式骨架：median=0.13，shortfall=162
- 正文是否覆盖关键文件：median=0.03，shortfall=162

## Top Reuse Offenders

- 项目概述.md：reuse_count=62；unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- 插件生态/pseudo-states.md：reuse_count=14；unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- 多框架支持/多框架支持.md：reuse_count=14；unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- 系统架构.md：reuse_count=11；unit_id=unit-3ce08b39bd6b, unit_type=architecture, domain_id=system, readiness=compose_ready, child_digests=5
- 多框架支持/react-vitest-3.md：reuse_count=6；unit_id=unit-ed51639dbf62, unit_type=module_doc, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=0
- 配置参考/主题和外观.md：reuse_count=6；unit_id=unit-bebeb3781961, unit_type=config_doc, domain_id=domain-e15151050742, readiness=compose_ready, child_digests=3
- 主题系统/themes.md：reuse_count=6；unit_id=unit-e858919ec1df, unit_type=module_doc, domain_id=domain-9cda34dbf62b, readiness=compose_ready, child_digests=0
- 平台绑定-Web/web-components.md：reuse_count=4；unit_id=unit-e7060a424501, unit_type=module_doc, domain_id=domain-c86d32da5da8, readiness=compose_ready, child_digests=0
- 构建系统/构建系统.md：reuse_count=3；unit_id=unit-6733528f117a, unit_type=domain_index, domain_id=domain-f0fd06748cfc, readiness=compose_ready, child_digests=6
- 构建系统/builder-webpack5.md：reuse_count=3；unit_id=unit-e23b550d9f21, unit_type=module_doc, domain_id=domain-f0fd06748cfc, readiness=compose_ready, child_digests=0

## Skeleton Lowest Pages

- 高级功能/高级功能.md -> 多框架支持/多框架支持.md：skeleton=0.00
- 高级功能/扩展开发/预设开发.md -> 多框架支持/多框架支持.md：skeleton=0.00
- 高级功能/预设配置.md -> 多框架支持/多框架支持.md：skeleton=0.00
- 构建系统/构建系统.md -> 构建系统/构建系统.md：skeleton=0.00
- 多框架支持/React框架支持.md -> 多框架支持/多框架支持.md：skeleton=0.04

## Key Source Lowest Pages

- 部署和CI_CD/静态部署.md -> 插件生态/pseudo-states.md：coverage=0.00，missing=.circleci/config.yml、.circleci/config.yml#L1-L55、.circleci/config.yml#L31-L49、code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L129-L183
- 插件系统/第三方Addons集成.md -> 概念指南/addons.md：coverage=0.00，missing=code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L107-L116、code/.storybook/main.ts#L117-L120、code/.storybook/main.ts#L129-L132、code/.storybook/main.ts#L133-L139
- 插件系统/核心Addons详解/核心Addons详解.md -> 插件生态/pseudo-states.md：coverage=0.00，missing=.yarnrc.yml、code/.storybook/main.ts、code/.storybook/manager.tsx、code/.storybook/preview.tsx、code/addons/a11y/package.json、code/addons/a11y/package.json#L1-L90
- 插件系统/核心Addons详解/Controls Addon（参数控制）.md -> 概念指南/essentials/Controls.md：coverage=0.00，missing=CHANGELOG.md、MIGRATION.md、MIGRATION.md#L984、code/addons/docs/src/blocks/controls/ArgControl.tsx、code/addons/docs/src/blocks/controls/ArgControl.tsx#L1-L20、code/addons/onboarding/src/constants.ts
- 插件系统/核心Addons详解/Outline_Measure_Toolbar Addons（辅助工具）.md -> 概念指南/addons.md：coverage=0.00，missing=code/core/src/components/components/Tabs/Tabs.hooks.tsx、code/core/src/core-server/presets/common-manager.ts、code/core/src/core-server/presets/common-preset.ts、code/core/src/manager-api/modules/layout.ts、code/core/src/manager-api/modules/layout.ts#L306-L314、code/core/src/manager-api/modules/layout.ts#L464-L471

## Warm Stability

- stable：true
- delta_reuse_overage：0
- delta_median_skeleton：0.00
- delta_median_key_source：0.00

## 覆盖统计

- topic coverage：generated 5 / reference 8
- evidence coverage：generated 194 / reference 176
- citation density：generated 17.66 / reference 40.2
- diagram coverage：generated 190 / reference 176
- 高频缺失专题：流程主题(3)、核心机制主题(1)

## Decomposition 命中

- generated：runtime(43)、compiler-pipeline(12)、api-surface(54)、config-surface(110)、docs-guide(195)、testing(83)、example-tutorial(57)、troubleshooting(119)、integration-platform(112)
- reference：runtime(168)、compiler-pipeline(60)、api-surface(125)、config-surface(168)、docs-guide(176)、testing(120)、example-tutorial(127)、troubleshooting(176)、integration-platform(136)
- 高频缺口：troubleshooting(156)、api-surface(99)、compiler-pipeline(48)、example-tutorial(43)、runtime(20)、testing(12)

## 项目结论

- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- evidence block 已进入页面，但覆盖率和密度仍低于 reference
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- docs-backed 页面主章节骨架仍偏离 reference，存在英文原始 docs heading 或泛化模板回退
- 解释层正文密度仍低于 reference
- 高频缺失专题集中在：流程主题、核心机制主题

## 逐文件详情

### API参考/API参考.md

- reference 标题：API参考
- 生成页：多框架支持/react-vitest-3.md（react-vitest-3）
- KnowledgeUnit：unit_id=unit-ed51639dbf62, unit_type=module_doc, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=0
- reuse_count：6
- skeleton_score：0.11
- key_source_coverage：0.04
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L107-L183、code/.storybook/main.ts#L169-L180、code/.storybook/main.ts#L19-L183、code/.storybook/main.ts#L20-L106、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L415、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L341-L355、code/.storybook/preview.tsx#L402-L415、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L1-L30、code/.storybook/storybook.setup.ts#L11-L29、code/addons/a11y/src/typings.d.ts、code/addons/a11y/src/typings.d.ts#L1-L2、code/addons/docs/src/typings.d.ts、code/addons/docs/src/typings.d.ts#L1-L13、code/addons/links/src/manager.ts、code/addons/links/src/manager.ts#L1-L10、code/addons/links/src/manager.ts#L5-L9、code/addons/vitest/src/typings.d.ts、code/addons/vitest/src/typings.d.ts#L1-L16、code/core/src/common/utils/get-renderer-name.ts、code/core/src/common/utils/get-renderer-name.ts#L11-L58、code/core/src/manager-api/root.tsx、code/core/src/manager-api/root.tsx#L397-L433、code/core/src/manager-api/typings.d.ts、code/core/src/manager-api/typings.d.ts#L1-L13、code/core/src/preview-api/modules/addons/hooks.ts、code/core/src/preview-api/modules/addons/hooks.ts#L96-L140、code/core/src/preview/typings.d.ts、code/core/src/preview/typings.d.ts#L1-L6、code/core/src/typings.d.ts、code/core/src/typings.d.ts#L1-L63、code/frameworks/react-vite/src/index.ts、code/frameworks/react-vite/src/index.ts#L1-L4、code/frameworks/react-webpack5/src/index.ts、code/frameworks/react-webpack5/src/index.ts#L1-L3、code/frameworks/web-components-vite/src/index.ts、code/frameworks/web-components-vite/src/index.ts#L1-L4、code/lib/cli-storybook/package.json、code/lib/cli-storybook/package.json#L1-L69、code/lib/cli-storybook/src/automigrate/helpers/mainConfigFile.ts、code/lib/cli-storybook/src/automigrate/helpers/mainConfigFile.ts#L48-L77、code/renderers/react/src/typings.d.ts、code/renderers/react/src/typings.d.ts#L1-L5、scripts/utils/cli-step.ts、scripts/utils/cli-step.ts#L1-L143、scripts/utils/cli-step.ts#L98-L143
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L107-L183、main.ts#L169-L180、main.ts#L19-L183、main.ts#L20-L106、manager.tsx、manager.tsx#L1-L9、manager.tsx#L4-L8

### API参考/CLI命令参考.md

- reference 标题：CLI命令参考
- 生成页：核心运行时/core.md（core）
- KnowledgeUnit：unit_id=unit-9aa179a17370, unit_type=module_doc, domain_id=domain-6226b0bb2dd4, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.21
- key_source_coverage：0.10
- missing_key_sources：CHANGELOG.md、MIGRATION.md、MIGRATION.md#L2385-L2386、MIGRATION.md#L3210-L3229、code/core/src/bin/core.ts#L172-L192、code/core/src/bin/core.ts#L172-L213、code/core/src/bin/core.ts#L176-L213、code/core/src/bin/core.ts#L182-L189、code/core/src/bin/core.ts#L186-L189、code/core/src/bin/core.ts#L194-L213、code/core/src/bin/dispatcher.ts#L1-L88、code/core/src/bin/dispatcher.ts#L23-L34、code/core/src/bin/dispatcher.ts#L23-L88、code/core/src/bin/dispatcher.ts#L36-L54、code/core/src/common/utils/get-storybook-configuration.ts、code/core/src/common/utils/get-storybook-configuration.ts#L1-L25、code/lib/cli-sb/index.js、code/lib/cli-sb/index.js#L1-L4、code/lib/create-storybook/src/initiate.ts、code/lib/create-storybook/src/initiate.ts#L1-L227、code/lib/create-storybook/src/initiate.ts#L126-L132、code/lib/create-storybook/src/initiate.ts#L139-L160、code/lib/create-storybook/src/initiate.ts#L163-L226、code/lib/create-storybook/src/initiate.ts#L198-L211、code/lib/create-storybook/src/initiate.ts#L32-L124、package.json、scripts/utils/cli-step.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CHANGELOG.md、MIGRATION.md、MIGRATION.md#L2385-L2386、MIGRATION.md#L3210-L3229、core.ts#L172-L192、core.ts#L172-L213、core.ts#L176-L213、core.ts#L182-L189

### API参考/开发API参考/Addon API.md

- reference 标题：插件API
- 生成页：API-参考/开发API参考/插件API.md（插件API）
- KnowledgeUnit：unit_id=unit-aca9da519bfe, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.14
- key_source_coverage：0.00
- missing_key_sources：code/addons/a11y/package.json、code/addons/a11y/package.json#L37-L52、code/addons/a11y/package.json#L82-L88、code/addons/a11y/src/types.ts、code/addons/a11y/src/types.ts#L1-L68、code/addons/docs/package.json、code/addons/docs/package.json#L125-L131、code/addons/docs/package.json#L40-L80、code/addons/docs/src/blocks/blocks/types.ts、code/addons/docs/src/blocks/blocks/types.ts#L1-L16、code/addons/links/package.json、code/addons/links/package.json#L29-L47、code/addons/links/package.json#L74-L80、code/addons/themes/package.json、code/addons/themes/package.json#L36-L50、code/addons/themes/package.json#L74-L80、code/addons/vitest/package.json、code/addons/vitest/package.json#L131-L137、code/addons/vitest/package.json#L40-L66、code/core/src/channels/index.test.ts、code/core/src/channels/index.test.ts#L55-L104、code/core/src/manager-api/lib/addons.ts、code/core/src/manager-api/lib/addons.ts#L100-L109、code/core/src/manager-api/lib/addons.ts#L56-L111、code/core/src/manager-api/modules/addons.ts、code/lib/cli-storybook/src/postinstallAddon.ts、code/lib/cli-storybook/src/postinstallAddon.ts#L1-L49、docs/_snippets/storybook-addons-api-getqueryparam.md、docs/_snippets/storybook-addons-api-getqueryparam.md#L1-L6、docs/_snippets/storybook-addons-api-selectincurrentkind.md、docs/_snippets/storybook-addons-api-selectincurrentkind.md#L1-L6
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L37-L52、package.json#L82-L88、types.ts、types.ts#L1-L68、package.json、package.json#L125-L131、package.json#L40-L80

### API参考/开发API参考/CSF API.md

- reference 标题：CSF API
- 生成页：API-参考/开发API参考/CSF-API.md（CSF API）
- KnowledgeUnit：unit_id=unit-7a28a946024d, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.06
- key_source_coverage：0.00
- missing_key_sources：CHANGELOG.md、CHANGELOG.md#L115-L172、CHANGELOG.md#L479-L538、CHANGELOG.md#L628-L651、code/core/src/core-server/utils/new-story-templates/csf-factory-template.test.ts、code/core/src/core-server/utils/new-story-templates/csf-factory-template.ts、code/core/src/core-server/utils/new-story-templates/csf-factory-template.ts#L1-L46、code/core/src/core-server/utils/new-story-templates/csf-factory-template.ts#L18-L45、code/core/src/csf-tools/README.md、code/core/src/csf-tools/README.md#L1-L14、code/core/src/csf/core-annotations.ts、code/core/src/csf/core-annotations.ts#L1-L55、code/core/src/csf/core-annotations.ts#L33-L54、code/core/src/csf/csf-factories.test.ts、code/core/src/csf/csf-factories.test.ts#L18-L47、code/core/src/csf/csf-factories.ts、code/core/src/csf/csf-factories.ts#L1-L286、code/core/src/csf/csf-factories.ts#L119-L137、code/core/src/csf/csf-factories.ts#L139-L171、code/core/src/csf/csf-factories.ts#L15-L21、code/core/src/csf/csf-factories.ts#L180-L192、code/core/src/csf/csf-factories.ts#L214-L244、code/core/src/csf/csf-factories.ts#L222-L233、code/core/src/csf/csf-factories.ts#L245-L274、code/core/src/csf/csf-factories.ts#L26-L137、code/core/src/csf/csf-factories.ts#L26-L171、code/core/src/csf/csf-factories.ts#L45-L137、code/core/src/csf/csf-factories.ts#L45-L72、code/core/src/preview-api/modules/store/csf/csf-factory-utils.ts、code/core/src/preview-api/modules/store/csf/csf-factory-utils.ts#L1-L26、code/core/src/preview-api/modules/store/csf/csf-factory-utils.ts#L10-L25、code/presets/server-webpack/src/loader.ts、code/presets/server-webpack/src/loader.ts#L1-L12、code/renderers/react/src/__test__/Button.csf4.stories.tsx、code/renderers/react/src/__test__/Button.csf4.stories.tsx#L1-L300、docs/_snippets/play-function-composition.md、docs/_snippets/play-function-composition.md#L278-L311、docs/_snippets/play-function-composition.md#L278-L386
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CHANGELOG.md、CHANGELOG.md#L115-L172、CHANGELOG.md#L479-L538、CHANGELOG.md#L628-L651、csf-factory-template.test.ts、csf-factory-template.ts、csf-factory-template.ts#L1-L46、csf-factory-template.ts#L18-L45

### API参考/开发API参考/Decorators API.md

- reference 标题：装饰器API
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- KnowledgeUnit：unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：14
- skeleton_score：0.15
- key_source_coverage：0.09
- missing_key_sources：code/addons/pseudo-states/src/preview.ts#L1-L8、code/addons/pseudo-states/src/preview.ts#L4-L8、code/addons/pseudo-states/src/preview/withPseudoState.ts#L1-L13、code/addons/pseudo-states/src/preview/withPseudoState.ts#L1-L223、code/addons/pseudo-states/src/preview/withPseudoState.ts#L125-L175、code/addons/pseudo-states/src/preview/withPseudoState.ts#L177-L222、code/core/src/actions/models/DecoratorFunction.ts、code/core/src/actions/models/DecoratorFunction.ts#L1-L1、code/core/src/preview-api/modules/addons/make-decorator.test.ts、code/core/src/preview-api/modules/addons/make-decorator.test.ts#L1-L170、code/core/src/preview-api/modules/addons/make-decorator.test.ts#L158-L168、code/core/src/preview-api/modules/addons/make-decorator.test.ts#L23-L168、code/core/src/preview-api/modules/addons/make-decorator.ts、code/core/src/preview-api/modules/addons/make-decorator.ts#L1-L14、code/core/src/preview-api/modules/addons/make-decorator.ts#L1-L91、code/core/src/preview-api/modules/addons/make-decorator.ts#L44-L66、code/core/src/preview-api/modules/addons/make-decorator.ts#L44-L90、code/core/src/preview-api/modules/addons/make-decorator.ts#L50-L66、code/core/src/preview-api/modules/addons/make-decorator.ts#L54-L60、code/core/src/preview-api/modules/addons/make-decorator.ts#L9-L49、code/core/src/preview-api/modules/preview-web/Preview.tsx、code/core/src/preview-api/modules/preview-web/Preview.tsx#L1-L510、code/core/src/preview-api/modules/preview-web/Preview.tsx#L1-L52、code/core/src/preview-api/modules/preview-web/Preview.tsx#L175-L223、code/core/src/preview-api/modules/preview-web/PreviewWeb.tsx、code/core/src/preview-api/modules/preview-web/PreviewWeb.tsx#L1-L10、code/core/src/preview-api/modules/preview-web/PreviewWeb.tsx#L1-L22、code/core/src/preview-api/modules/preview-web/PreviewWeb.tsx#L11-L21、code/core/src/preview-api/modules/preview-web/index.ts#L1-L22
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：preview.ts#L1-L8、preview.ts#L4-L8、withPseudoState.ts#L1-L13、withPseudoState.ts#L1-L223、withPseudoState.ts#L125-L175、withPseudoState.ts#L177-L222、DecoratorFunction.ts、DecoratorFunction.ts#L1-L1

### API参考/开发API参考/Hooks API.md

- reference 标题：Hooks API
- 生成页：无
- 问题：缺少对应生成页面

### API参考/开发API参考/Preview API.md

- reference 标题：Preview API
- 生成页：API-参考/开发API参考/Preview-API.md（Preview API）
- KnowledgeUnit：unit_id=unit-c5c20b0bb950, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0
- reuse_count：2
- skeleton_score：0.15
- key_source_coverage：0.05
- missing_key_sources：code/core/src/instrumenter/preview-api.ts、code/core/src/instrumenter/preview-api.ts#L6、code/core/src/preview-api/modules/preview-web/PreviewWeb.tsx、code/core/src/preview-api/modules/preview-web/WebView.ts、code/core/src/preview-api/modules/preview-web/WebView.ts#L115-L124、code/core/src/preview-api/modules/preview-web/WebView.ts#L137-L150、code/core/src/preview-api/modules/preview-web/WebView.ts#L164-L184、code/core/src/preview-api/modules/preview-web/WebView.ts#L42-L209、code/core/src/preview-api/modules/preview-web/WebView.ts#L69-L198、code/core/src/preview-api/modules/preview-web/WebView.ts#L69-L78、code/core/src/preview-api/modules/preview-web/WebView.ts#L69-L97、code/core/src/preview-api/modules/preview-web/index.ts#L1-L22、code/core/src/preview-api/modules/preview-web/index.ts#L21-L22、code/core/src/preview-api/modules/preview-web/index.ts#L5-L14、code/core/src/preview-api/modules/preview-web/simulate-pageload.ts、code/core/src/preview-api/modules/preview-web/simulate-pageload.ts#L31-L115、code/core/src/preview-api/modules/preview-web/simulate-pageload.ts#L72-L81、code/core/src/preview-api/modules/preview-web/simulate-pageload.ts#L83-L115、code/e2e-tests/preview-api.spec.ts、code/e2e-tests/preview-api.spec.ts#L1-L95
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：preview-api.ts、preview-api.ts#L6、PreviewWeb.tsx、WebView.ts、WebView.ts#L115-L124、WebView.ts#L137-L150、WebView.ts#L164-L184、WebView.ts#L42-L209

### API参考/开发API参考/Store API.md

- reference 标题：Store API
- 生成页：API-参考/开发API参考/Store-API.md（Store API）
- KnowledgeUnit：unit_id=unit-c9d01c4740e6, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.13
- key_source_coverage：0.03
- missing_key_sources：code/core/core/src/preview-api/modules/store/StoryIndexStore.ts、code/core/src/manager-api/lib/store-setup.ts、code/core/src/manager-api/lib/store-setup.ts#L1-L20、code/core/src/manager-api/modules/url.ts、code/core/src/manager-api/root.tsx、code/core/src/manager-api/root.tsx#L130-L197、code/core/src/manager-api/store.ts、code/core/src/manager-api/store.ts#L48-L121、code/core/src/manager-api/store.ts#L75-L121、code/core/src/preview-api/modules/preview-web/UrlStore.ts、code/core/src/preview-api/modules/preview-web/UrlStore.ts#L69-L113、code/core/src/preview-api/modules/preview-web/UrlStore.ts#L69-L92、code/core/src/preview-api/modules/preview-web/UrlStore.ts#L94-L113、code/core/src/preview-api/modules/store/ArgsStore.ts、code/core/src/preview-api/modules/store/ArgsStore.ts#L11-L68、code/core/src/preview-api/modules/store/ArgsStore.ts#L16-L22、code/core/src/preview-api/modules/store/ArgsStore.ts#L16-L67、code/core/src/preview-api/modules/store/GlobalsStore.ts、code/core/src/preview-api/modules/store/GlobalsStore.ts#L39-L71、code/core/src/preview-api/modules/store/GlobalsStore.ts#L7-L72、code/core/src/preview-api/modules/store/StoryIndexStore.ts#L19-L59、code/core/src/preview-api/modules/store/StoryIndexStore.ts#L26-L58、code/core/src/preview-api/modules/store/csf/portable-stories.ts、code/core/src/preview-api/modules/store/csf/portable-stories.ts#L90-L269、code/core/src/preview-api/modules/store/csf/portable-stories.ts#L90-L302、code/core/src/preview-api/modules/store/csf/portable-stories.ts#L97-L100、code/core/src/preview-api/modules/store/csf/prepareStory.ts、code/core/src/preview-api/modules/store/csf/prepareStory.ts#L131-L133、code/core/src/preview-api/modules/store/csf/prepareStory.ts#L37-L170、code/core/src/shared/universal-store/index.ts#L571-L663、code/core/src/shared/universal-store/index.ts#L82-L695
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：StoryIndexStore.ts、store-setup.ts、store-setup.ts#L1-L20、url.ts、root.tsx、root.tsx#L130-L197、store.ts、store.ts#L48-L121

### API参考/开发API参考/Types API.md

- reference 标题：Types API
- 生成页：API-参考/api.md（Api）
- KnowledgeUnit：unit_id=unit-55cebbb98db4, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=5
- reuse_count：1
- skeleton_score：0.38
- key_source_coverage：0.00
- missing_key_sources：MIGRATION.md、MIGRATION.md#L3819-L3820、code/core/src/csf/story.ts、code/core/src/csf/story.ts#L1-L500、code/core/src/csf/story.ts#L451-L500、code/core/src/csf/story.ts#L8-L9-L451-L500、code/core/src/docs-tools/argTypes/types.ts、code/core/src/docs-tools/argTypes/types.ts#L1-L10、code/core/src/manager-api/modules/stories.ts、code/core/src/manager-api/modules/stories.ts#L110-L136、code/core/src/preview-api/modules/preview-web/PreviewWeb.tsx、code/core/src/preview-api/modules/store/args.test.ts、code/core/src/preview-api/modules/store/args.test.ts#L80-L224、code/core/src/preview-api/modules/store/inferArgTypes.test.ts、code/core/src/preview-api/modules/store/inferArgTypes.test.ts#L46-L101、code/core/src/preview-api/modules/store/inferArgTypes.test.ts#L83-L101、code/core/src/preview-api/modules/store/types.ts、code/core/src/typings.d.ts、code/core/src/typings.d.ts#L1-L63、code/frameworks/angular/src/client/compodoc.ts、code/frameworks/angular/src/client/compodoc.ts#L248-L307、code/frameworks/angular/src/client/compodoc.ts#L299-L307、docs/api/arg-types.mdx、docs/api/arg-types.mdx#L321-L360、docs/api/arg-types.mdx#L343-L360
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：MIGRATION.md、MIGRATION.md#L3819-L3820、story.ts、story.ts#L1-L500、story.ts#L451-L500、story.ts#L8-L9-L451-L500、types.ts、types.ts#L1-L10

### API参考/开发API参考/开发API参考.md

- reference 标题：开发API参考
- 生成页：API-参考/开发API参考.md（开发API参考）
- KnowledgeUnit：unit_id=unit-e5aa6f8808f2, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=4
- reuse_count：1
- skeleton_score：0.15
- key_source_coverage：0.06
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L19-L183、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L157-L192、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L402-L415、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L1-L30、code/.storybook/storybook.setup.ts#L11-L29、code/.storybook/storybook.setup.ts#L16-L27、code/core/src/manager-api/index.ts#L1-L27、code/core/src/manager-api/modules/addons.ts、code/core/src/manager-api/modules/addons.ts#L55-L74、code/core/src/manager-api/modules/addons.ts#L66-L74、code/core/src/manager-api/modules/shortcuts.ts、code/core/src/manager-api/modules/shortcuts.ts#L180-L210、code/core/src/preview-api/addons.ts、code/core/src/preview-api/addons.ts#L1-L4、code/core/src/preview-api/index.ts#L1-L90、code/core/src/preview-api/index.ts#L78-L88、code/core/src/preview-api/modules/addons/hooks.ts#L217-L242、code/core/src/preview-api/preview-web.ts、code/core/src/preview-api/preview-web.ts#L1-L4、code/core/src/test/preview.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L1-L186、main.ts#L149-L181、main.ts#L19-L183、manager.tsx、manager.tsx#L1-L9、manager.tsx#L4-L8、preview.tsx

### API参考/类型定义参考/API类型定义.md

- reference 标题：API类型定义
- 生成页：API-参考/类型定义参考/API类型定义.md（API类型定义）
- KnowledgeUnit：unit_id=unit-26477bd3916f, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.15
- key_source_coverage：0.00
- missing_key_sources：code/core/src/channels/types.ts、code/core/src/channels/types.ts#L1-L60、code/core/src/channels/types.ts#L13-L16、code/core/src/channels/types.ts#L13-L50、code/core/src/channels/types.ts#L36-L50、code/core/src/common/telemetry/sendTelemetryError.ts、code/core/src/manager-api/typings.d.ts、code/core/src/manager-api/typings.d.ts#L1-L13、code/core/src/manager-api/typings.d.ts#L4-L12、code/core/src/manager/typings.d.ts、code/core/src/manager/typings.d.ts#L1-L38、code/core/src/manager/typings.d.ts#L10-L35、code/core/src/manager/typings.d.ts#L31-L34、code/core/src/preview/typings.d.ts、code/core/src/preview/typings.d.ts#L1-L6、code/core/src/preview/typings.d.ts#L3-L5、code/core/src/preview/typings.d.ts#L5-L5、code/e2e-tests/preview-api.spec.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：types.ts、types.ts#L1-L60、types.ts#L13-L16、types.ts#L13-L50、types.ts#L36-L50、sendTelemetryError.ts、typings.d.ts、typings.d.ts#L1-L13

### API参考/类型定义参考/工具类型定义.md

- reference 标题：工具类型定义
- 生成页：API-参考/类型定义参考/工具类型定义.md（工具类型定义）
- KnowledgeUnit：unit_id=unit-33cb1a901ca2, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.10
- key_source_coverage：0.07
- missing_key_sources：code/core/src/channels/types.ts、code/core/src/channels/types.ts#L1-L60、code/core/src/channels/types.ts#L36-L60、code/core/src/manager-api/root.tsx、code/core/src/manager-api/root.tsx#L1-L516、code/core/src/manager-api/store.ts、code/core/src/manager-api/store.ts#L1-L123、code/core/src/manager-api/store.ts#L27-L44、code/core/src/manager-api/store.ts#L48-L122、code/core/src/manager-api/store.ts#L75-L121、code/core/src/preview-api/index.ts#L1-L90、code/core/src/preview-api/modules/addons/hooks.ts、code/core/src/preview-api/modules/addons/hooks.ts#L1-L660、code/core/src/preview-api/modules/addons/hooks.ts#L131-L215、code/core/src/preview-api/modules/addons/hooks.ts#L220-L224、code/core/src/preview-api/modules/addons/hooks.ts#L23-L54、code/core/src/preview-api/modules/addons/hooks.ts#L244-L292、code/core/src/preview-api/modules/addons/hooks.ts#L36-L129、code/core/src/shared/universal-store/index.ts#L1-L695、code/core/src/shared/universal-store/index.ts#L374-L419、code/core/src/shared/universal-store/index.ts#L528-L663、code/core/src/shared/universal-store/index.ts#L614-L621、code/core/src/shared/universal-store/index.ts#L82-L127、code/core/src/shared/universal-store/index.ts#L82-L695、code/core/src/shared/universal-store/types.ts、code/core/src/shared/universal-store/types.ts#L10-L58、code/core/src/types/modules/features.ts、code/core/src/types/modules/features.ts#L1-L6
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：types.ts、types.ts#L1-L60、types.ts#L36-L60、root.tsx、root.tsx#L1-L516、store.ts、store.ts#L1-L123、store.ts#L27-L44

### API参考/类型定义参考/插件类型定义.md

- reference 标题：插件类型定义
- 生成页：无
- 问题：缺少对应生成页面

### API参考/类型定义参考/构建器类型定义.md

- reference 标题：构建器类型定义
- 生成页：无
- 问题：缺少对应生成页面

### API参考/类型定义参考/核心类型定义.md

- reference 标题：核心类型定义
- 生成页：平台绑定-Web/web-components.md（web-components）
- KnowledgeUnit：unit_id=unit-e7060a424501, unit_type=module_doc, domain_id=domain-c86d32da5da8, readiness=compose_ready, child_digests=0
- reuse_count：4
- skeleton_score：0.11
- key_source_coverage：0.07
- missing_key_sources：code/core/src/core-server/utils/save-story/mocks/unsupported-csf-variances.stories.tsx、code/core/src/csf/SBType.ts、code/core/src/csf/SBType.ts#L1-L59、code/core/src/csf/core-annotations.ts、code/core/src/csf/csf-factories.ts、code/core/src/csf/csf-factories.ts#L1-L25、code/core/src/csf/csf-factories.ts#L1-L286、code/core/src/csf/csf-factories.ts#L26-L171、code/core/src/csf/csf-factories.ts#L88-L276、code/core/src/csf/story.ts、code/core/src/csf/story.ts#L1-L604、code/core/src/csf/story.ts#L1-L8、code/core/src/csf/story.ts#L32-L170、code/core/src/csf/story.ts#L331-L339、code/core/src/csf/story.ts#L347-L541、code/core/src/csf/story.ts#L421-L541、code/core/src/csf/story.ts#L8-L30、code/core/src/preview-api/modules/addons/make-decorator.ts、code/core/src/preview-api/modules/store/decorators.ts、code/core/src/preview-api/modules/store/decorators.ts#L1-L95、code/core/src/preview-api/modules/store/decorators.ts#L10-L21、code/core/src/preview-api/modules/store/decorators.ts#L49-L95、code/renderers/react/src/preview.tsx、code/renderers/react/src/preview.tsx#L140-L212、code/renderers/react/src/preview.tsx#L147-L212、code/renderers/web-components/src/preview.ts#L198-L229
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：unsupported-csf-variances.stories.tsx、SBType.ts、SBType.ts#L1-L59、core-annotations.ts、csf-factories.ts、csf-factories.ts#L1-L25、csf-factories.ts#L1-L286、csf-factories.ts#L26-L171

### API参考/类型定义参考/框架类型定义.md

- reference 标题：框架类型定义
- 生成页：API-参考/类型定义参考/框架类型定义.md（框架类型定义）
- KnowledgeUnit：unit_id=unit-4968f588a5fc, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.20
- key_source_coverage：0.03
- missing_key_sources：code/core/src/common/utils/framework.ts、code/core/src/common/utils/framework.ts#L41-L64、code/core/src/csf/story.ts、code/core/src/csf/story.ts#L193-L214、code/core/src/csf/story.ts#L268-L281、code/core/src/types/modules/frameworks.ts、code/core/src/types/modules/frameworks.ts#L1-L26、code/core/src/types/modules/renderers.ts、code/core/src/types/modules/renderers.ts#L1-L16、code/frameworks/angular/src/client/angular-beta/AbstractRenderer.ts、code/frameworks/angular/src/client/angular-beta/AbstractRenderer.ts#L196-L238、code/frameworks/angular/src/client/angular-beta/AbstractRenderer.ts#L35-L240、code/frameworks/angular/src/client/angular-beta/AbstractRenderer.ts#L35-L50、code/frameworks/angular/src/client/angular-beta/CanvasRenderer.ts、code/frameworks/angular/src/client/angular-beta/CanvasRenderer.ts#L4-L18、code/frameworks/angular/src/client/angular-beta/DocsRenderer.ts、code/frameworks/angular/src/client/angular-beta/DocsRenderer.ts#L25-L35、code/frameworks/angular/src/client/angular-beta/DocsRenderer.ts#L8-L38、code/frameworks/angular/src/client/angular-beta/DocsRenderer.ts#L8-L53、code/frameworks/angular/src/client/angular-beta/RendererFactory.ts、code/frameworks/angular/src/client/angular-beta/RendererFactory.ts#L11-L34、code/frameworks/angular/src/client/angular-beta/RendererFactory.ts#L20-L34、code/frameworks/angular/src/client/angular-beta/RendererFactory.ts#L6-L42、code/frameworks/angular/src/client/types.ts、code/frameworks/angular/src/client/types.ts#L40-L43、code/frameworks/angular/src/client/types.ts#L40-L50、code/frameworks/angular/src/client/types.ts#L45-L47、code/lib/create-storybook/src/bin/modernInputs.ts、code/lib/create-storybook/src/bin/modernInputs.ts#L52-L73、code/lib/create-storybook/src/generators/NEXTJS/index.ts#L73-L102、docs/_snippets/button-component-with-proptypes.md、docs/_snippets/button-component-with-proptypes.md#L1-L93、docs/_snippets/button-implementation.md、docs/_snippets/button-implementation.md#L79-L151
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：framework.ts、framework.ts#L41-L64、story.ts、story.ts#L193-L214、story.ts#L268-L281、frameworks.ts、frameworks.ts#L1-L26、renderers.ts

### API参考/类型定义参考/类型定义参考.md

- reference 标题：类型定义参考
- 生成页：API-参考/类型定义参考.md（类型定义参考）
- KnowledgeUnit：unit_id=unit-60c68a68653f, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=3
- reuse_count：1
- skeleton_score：0.10
- key_source_coverage：0.19
- missing_key_sources：code/addons/a11y/src/types.ts#L1-L68、code/addons/a11y/src/types.ts#L7-L68、code/addons/docs/src/types.ts#L1-L257、code/addons/docs/src/types.ts#L1-L5、code/addons/docs/src/types.ts#L151-L256、code/core/src/manager/typings.d.ts#L1-L38、code/core/src/preview-api/modules/addons/main.ts、code/core/src/preview-api/modules/addons/main.ts#L1-L52、code/core/src/preview-api/modules/addons/main.ts#L44-L52、code/core/src/preview-api/modules/addons/main.ts#L7-L39、code/core/src/preview/typings.d.ts#L1-L6、code/core/src/types/modules/builders.ts、code/core/src/types/modules/builders.ts#L1-L8、code/core/src/types/modules/core-common.ts、code/core/src/types/modules/core-common.ts#L1-L11、code/core/src/types/modules/core-common.ts#L1-L735、code/core/src/types/modules/core-common.ts#L23-L670、code/core/src/types/modules/core-common.ts#L390-L670、code/core/src/types/modules/core-common.ts#L672-L683、code/core/src/types/modules/frameworks.ts、code/core/src/types/modules/frameworks.ts#L1-L27、code/core/src/types/modules/frameworks.ts#L2-L26、code/core/src/types/modules/renderers.ts、code/core/src/types/modules/renderers.ts#L1-L16、code/core/src/typings.d.ts#L1-L63、code/core/src/typings.d.ts#L3-L33
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：types.ts#L1-L68、types.ts#L7-L68、types.ts#L1-L257、types.ts#L1-L5、types.ts#L151-L256、typings.d.ts#L1-L38、main.ts、main.ts#L1-L52

### API参考/配置API参考/main.js配置.md

- reference 标题：main.js配置
- 生成页：配置参考/configure/integration.md（Integration）
- KnowledgeUnit：unit_id=unit-09120ecb41af, unit_type=config_doc, domain_id=domain-e15151050742, readiness=compose_ready, child_digests=6
- reuse_count：1
- skeleton_score：0.11
- key_source_coverage：0.08
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L107-L116、code/.storybook/main.ts#L117-L120、code/.storybook/main.ts#L121-L128、code/.storybook/main.ts#L129-L132、code/.storybook/main.ts#L133-L139、code/.storybook/main.ts#L140-L142、code/.storybook/main.ts#L143-L147、code/.storybook/main.ts#L148、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L170-L171、code/.storybook/main.ts#L170-L172、code/.storybook/main.ts#L172、code/.storybook/main.ts#L19-L185、code/.storybook/main.ts#L20-L106、docs/_snippets/main-config-typical.md、docs/_snippets/main-config-typical.md#L1-L128、docs/addons/addon-types.mdx、docs/addons/addon-types.mdx#L64-L83、docs/api/main-config/main-config-build.mdx、docs/api/main-config/main-config-build.mdx#L1-L122、docs/api/main-config/main-config-core.mdx、docs/api/main-config/main-config-core.mdx#L1-L179、docs/api/main-config/main-config-features.mdx、docs/api/main-config/main-config-features.mdx#L1-L196、docs/api/main-config/main-config-features.mdx#L119-L129、docs/configure/index.mdx#L28-L44、docs/configure/index.mdx#L45-L96、docs/configure/index.mdx#L66-L96、docs/configure/integration/frameworks.mdx#L14-L46、docs/configure/integration/frameworks.mdx#L27-L46、docs/configure/integration/images-and-assets.mdx#L22-L60、docs/configure/integration/images-and-assets.mdx#L72-L90、docs/sharing/storybook-composition.mdx、docs/sharing/storybook-composition.mdx#L21-L59
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L1-L186、main.ts#L107-L116、main.ts#L117-L120、main.ts#L121-L128、main.ts#L129-L132、main.ts#L133-L139、main.ts#L140-L142

### API参考/配置API参考/manager.js配置.md

- reference 标题：manager.js配置
- 生成页：API-参考/api/main-config/Main-Config.md（Main Config）
- KnowledgeUnit：unit_id=unit-711c4bac651c, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.54
- key_source_coverage：0.00
- missing_key_sources：code/addons/themes/src/manager.tsx、code/addons/themes/src/manager.tsx#L1-L15、code/core/src/manager-api/modules/layout.ts、code/core/src/manager-api/modules/layout.ts#L323-L345、code/core/src/manager-api/tests/layout.test.ts、code/core/src/manager/components/layout/Layout.stories.tsx、code/core/src/manager/components/layout/Layout.stories.tsx#L100-L141、code/core/src/manager/index.tsx、code/core/src/manager/index.tsx#L40-L67、docs/_snippets/main-config-manager-head.md、docs/_snippets/main-config-manager-head.md#L1-L127、docs/_snippets/storybook-main-register-addon.md、docs/_snippets/storybook-main-register-addon.md#L1-L6、docs/api/main-config/main-config-manager-head.mdx、docs/api/main-config/main-config-manager-head.mdx#L1-L27、docs/api/main-config/main-config.mdx、docs/api/main-config/main-config.mdx#L1-L56、docs/api/main-config/main-config.mdx#L29-L56、docs/configure/user-interface/features-and-behavior.mdx、docs/configure/user-interface/features-and-behavior.mdx#L1-L153、docs/configure/user-interface/features-and-behavior.mdx#L16-L32、docs/configure/user-interface/features-and-behavior.mdx#L33-L91、docs/configure/user-interface/features-and-behavior.mdx#L47-L91、docs/configure/user-interface/features-and-behavior.mdx#L54-L90
- 结论：图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：manager.tsx、manager.tsx#L1-L15、layout.ts、layout.ts#L323-L345、layout.test.ts、Layout.stories.tsx、Layout.stories.tsx#L100-L141、index.tsx

### API参考/配置API参考/preview.js配置.md

- reference 标题：preview.js配置
- 生成页：构建系统/构建系统.md（构建系统）
- KnowledgeUnit：unit_id=unit-6733528f117a, unit_type=domain_index, domain_id=domain-f0fd06748cfc, readiness=compose_ready, child_digests=6
- reuse_count：3
- skeleton_score：0.07
- key_source_coverage：0.04
- missing_key_sources：code/addons/docs/src/preview.ts、code/addons/docs/src/preview.ts#L14-L29、code/addons/docs/template/stories/docspage/basic.stories.ts、code/addons/docs/template/stories/docspage/basic.stories.ts#L3-L34、code/addons/themes/src/preview.ts、code/addons/themes/src/preview.ts#L5-L7、code/builders/builder-vite/src/utils/process-preview-annotation.ts、code/builders/builder-vite/src/utils/process-preview-annotation.ts#L6-L25、code/builders/builder-webpack5/src/presets/preview-preset.ts#L7-L25、code/core/src/preview-api/modules/store/csf/composeConfigs.test.ts、code/core/src/preview-api/modules/store/csf/composeConfigs.test.ts#L127-L128、code/core/src/preview-api/modules/store/csf/composeConfigs.test.ts#L93-L170、code/core/src/preview-api/modules/store/csf/getValuesFromGlobalTypes.ts、code/core/src/preview-api/modules/store/csf/getValuesFromGlobalTypes.ts#L3-L9、code/frameworks/nextjs/src/preview.tsx、code/frameworks/nextjs/src/preview.tsx#L57-L62、code/frameworks/nextjs/src/preview.tsx#L57-L92、code/frameworks/nextjs/src/preview.tsx#L76-L92、code/renderers/react/src/preview.tsx、code/renderers/react/src/preview.tsx#L102-L239、code/renderers/react/src/preview.tsx#L55-L85、docs/_snippets/storybook-builder-api-preview-exports.md、docs/_snippets/storybook-builder-api-preview-exports.md#L29-L32、docs/_snippets/storybook-builder-api-preview-exports.md#L56-L58、docs/_snippets/storybook-builder-api-preview-exports.md#L6-L79、docs/_snippets/storybook-builder-api-preview-exports.md#L65-L68
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：preview.ts、preview.ts#L14-L29、basic.stories.ts、basic.stories.ts#L3-L34、preview.ts、preview.ts#L5-L7、process-preview-annotation.ts、process-preview-annotation.ts#L6-L25

### API参考/配置API参考/构建器配置.md

- reference 标题：构建器配置
- 生成页：无
- 问题：缺少对应生成页面

### API参考/配置API参考/配置API参考.md

- reference 标题：配置API参考
- 生成页：多框架支持/多框架支持.md（多框架支持）
- KnowledgeUnit：unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- reuse_count：14
- skeleton_score：0.07
- key_source_coverage：0.09
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L19-L183、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L336-L400、code/.storybook/preview.tsx#L402-L415、code/addons/links/build-config.ts、code/addons/links/build-config.ts#L3-L23、code/core/src/common/utils/sync-main-preview-addons.ts、code/core/src/common/utils/sync-main-preview-addons.ts#L47-L66、code/core/src/csf/csf-factories.ts、code/core/src/csf/csf-factories.ts#L45-L72、code/core/src/types/modules/core-common.ts、code/core/src/types/modules/core-common.ts#L390-L565、code/core/src/types/modules/core-common.ts#L567-L670、code/frameworks/react-vite/src/node/index.ts#L3-L5
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L19-L183、manager.tsx、manager.tsx#L1-L9、manager.tsx#L4-L8、preview.tsx、preview.tsx#L1-L416、preview.tsx#L144-L193

### API参考/配置API参考/预设配置.md

- reference 标题：预设配置
- 生成页：多框架支持/多框架支持.md（多框架支持）
- KnowledgeUnit：unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- reuse_count：14
- skeleton_score：0.07
- key_source_coverage：0.22
- missing_key_sources：code/addons/a11y/src/preset.ts#L1-L200、code/addons/docs/preset.js、code/addons/docs/src/preset.ts#L1-L225、code/addons/onboarding/preset.js、code/addons/onboarding/src/preset.ts#L1-L200、code/addons/vitest/preset.js、code/addons/vitest/src/preset.ts#L1-L200、code/builders/builder-vite/preset.js、code/builders/builder-vite/src/preset.ts#L1-L42、code/core/src/common/presets.test.ts、code/core/src/common/presets.ts、code/core/src/common/presets.ts#L1-L358、code/core/src/common/presets.ts#L156-L243、code/core/src/common/presets.ts#L226-L242、code/core/src/common/presets.ts#L245-L263、code/core/src/common/presets.ts#L265-L316、code/core/src/common/presets.ts#L318-L358、code/core/src/common/presets.ts#L35-L40、code/core/src/core-server/build-static.ts、code/core/src/core-server/build-static.ts#L70-L112、code/core/src/core-server/load.ts、code/core/src/core-server/load.ts#L46-L103、code/core/src/csf-tools/ConfigFile.ts、code/core/src/csf-tools/ConfigFile.ts#L453-L541、code/frameworks/angular/preset.js、code/frameworks/angular/src/preset.ts#L1-L54、code/frameworks/react-vite/src/preset.ts#L1-L51、code/frameworks/vue3-vite/src/preset.ts#L1-L57
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：preset.ts#L1-L200、preset.js、preset.ts#L1-L225、preset.js、preset.ts#L1-L200、preset.js、preset.ts#L1-L200、preset.js

### 主题和外观/主题和外观.md

- reference 标题：主题和外观
- 生成页：配置参考/主题和外观.md（主题和外观）
- KnowledgeUnit：unit_id=unit-bebeb3781961, unit_type=config_doc, domain_id=domain-e15151050742, readiness=compose_ready, child_digests=3
- reuse_count：6
- skeleton_score：0.05
- key_source_coverage：0.00
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L169-L173、code/.storybook/main.ts#L19-L183、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L123-L129、code/.storybook/preview.tsx#L133-L142、code/.storybook/preview.tsx#L220-L301、code/.storybook/preview.tsx#L234-L236、code/.storybook/preview.tsx#L242-L297、code/.storybook/preview.tsx#L282-L297、code/.storybook/preview.tsx#L89-L91、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L1-L30、code/core/src/component-testing/utils.ts、code/core/src/components/components/Form/styles.ts、code/core/src/components/components/Form/styles.ts#L1-L120、code/core/src/components/components/Form/styles.ts#L52-L114、code/core/src/csf/story.ts、code/core/src/manager-api/tests/layout.test.ts、code/core/src/manager/components/preview/utils/components.ts、code/core/src/manager/components/preview/utils/components.ts#L1-L70、code/core/src/manager/globals/exports.ts、code/core/src/manager/globals/exports.ts#L365-L383、code/core/src/shared/utils/categorize-render-errors.test.ts#L127-L152、code/core/src/theming/base.ts、code/core/src/theming/base.ts#L1-L168、code/core/src/theming/base.ts#L104-L167、code/core/src/theming/convert.ts、code/core/src/theming/convert.ts#L1-L206、code/core/src/theming/convert.ts#L113-L184、code/core/src/theming/convert.ts#L163-L165、code/core/src/theming/convert.ts#L181-L203、code/core/src/theming/convert.ts#L78-L205、code/core/src/theming/create.ts、code/core/src/theming/create.ts#L1-L51、code/core/src/theming/create.ts#L18-L23、code/core/src/theming/create.ts#L18-L50、code/core/src/theming/create.ts#L29-L50、code/core/src/theming/index.ts、code/core/src/theming/index.ts#L1-L51、code/core/src/theming/index.ts#L43-L44、code/core/src/theming/index.ts#L6-L20、code/core/src/theming/tests/convert.test.js、code/core/src/theming/tests/convert.test.js#L17-L65、code/core/src/theming/tests/create.test.js、code/core/src/theming/tests/create.test.js#L1-L45、code/core/src/theming/tests/create.test.js#L10-L36
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L1-L186、main.ts#L169-L173、main.ts#L19-L183、manager.tsx、manager.tsx#L1-L9、manager.tsx#L4-L8、preview.tsx

### 主题和外观/主题系统概览.md

- reference 标题：主题系统概览
- 生成页：配置参考/主题和外观/主题系统概览.md（主题系统概览）
- KnowledgeUnit：unit_id=unit-fea4961ba0fd, unit_type=config_doc, domain_id=domain-e15151050742, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.13
- key_source_coverage：0.00
- missing_key_sources：code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L20-L30、code/.storybook/preview.tsx#L220-L301、code/.storybook/preview.tsx#L280-L298、code/addons/themes/docs/api.md、code/addons/themes/docs/api.md#L1-L108、code/addons/themes/src/theme-switcher.tsx、code/addons/themes/src/theme-switcher.tsx#L92-L112、code/core/src/theming/base.ts、code/core/src/theming/base.ts#L1-L168、code/core/src/theming/convert.ts、code/core/src/theming/convert.ts#L1-L206、code/core/src/theming/convert.ts#L78-L205、code/core/src/theming/create.ts、code/core/src/theming/create.ts#L1-L51、code/core/src/theming/create.ts#L18-L50、code/core/src/theming/create.ts#L29-L50、code/core/src/theming/tests/convert.test.js、code/core/src/theming/tests/convert.test.js#L8-L65、code/core/src/theming/tests/create.test.js、code/core/src/theming/tests/create.test.js#L10-L36、code/core/src/theming/themes/dark.ts、code/core/src/theming/themes/dark.ts#L1-L46、code/core/src/theming/themes/dark.ts#L4-L46、code/core/src/theming/themes/light.ts、code/core/src/theming/themes/light.ts#L1-L46、code/core/src/theming/themes/light.ts#L4-L46、code/core/src/theming/types.ts、code/core/src/theming/types.ts#L1-L109、code/core/src/theming/utils.ts、code/core/src/theming/utils.ts#L1-L79、code/core/src/theming/utils.ts#L24-L62、code/core/src/theming/utils.ts#L64-L79
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：manager.tsx、manager.tsx#L1-L9、preview.tsx、preview.tsx#L1-L416、preview.tsx#L20-L30、preview.tsx#L220-L301、preview.tsx#L280-L298、api.md

### 主题和外观/布局和间距设计.md

- reference 标题：布局和间距设计
- 生成页：主题系统/themes.md（themes）
- KnowledgeUnit：unit_id=unit-e858919ec1df, unit_type=module_doc, domain_id=domain-9cda34dbf62b, readiness=compose_ready, child_digests=0
- reuse_count：6
- skeleton_score：0.08
- key_source_coverage：0.07
- missing_key_sources：code/__mocks__/fileMock.js、code/__mocks__/fs-extra.ts、code/__mocks__/fs-extra.ts#L1-L40、code/__mocks__/fs.ts、code/__mocks__/fs/promises.ts、code/__mocks__/fs/promises.ts#L1-L40、code/__mocks__/htmlMock.js、code/__mocks__/lodash-es.js、code/__mocks__/styleMock.js、code/__mocks__/uuid.js、code/addons/themes/src/decorators.ts、code/addons/themes/src/index.ts#L1-L11、code/addons/themes/src/types.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：fileMock.js、fs-extra.ts、fs-extra.ts#L1-L40、fs.ts、promises.ts、promises.ts#L1-L40、htmlMock.js、lodash-es.js

### 主题和外观/自定义主题开发.md

- reference 标题：自定义主题开发
- 生成页：配置参考/主题和外观/自定义主题开发.md（自定义主题开发）
- KnowledgeUnit：unit_id=unit-969c31fcb23f, unit_type=config_doc, domain_id=domain-e15151050742, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.14
- key_source_coverage：0.02
- missing_key_sources：code/addons/themes/package.json#L1-L82、code/addons/themes/package.json#L67-L72、code/addons/themes/src/constants.ts、code/addons/themes/src/constants.ts#L1-L18、code/addons/themes/src/constants.ts#L3-L17、code/addons/themes/src/decorators/index.ts、code/addons/themes/src/decorators/index.ts#L1-L6、code/addons/themes/src/index.ts、code/addons/themes/src/index.ts#L1-L11、code/addons/themes/src/preview.ts、code/addons/themes/src/preview.ts#L1-L8、code/addons/themes/src/preview.ts#L5-L7、code/addons/themes/src/types.ts、code/addons/themes/src/types.ts#L1-L29、code/core/src/theming/base.ts、code/core/src/theming/convert.ts、code/core/src/theming/create.ts、code/core/src/theming/create.ts#L1-L50、code/core/src/theming/create.ts#L18-L50、code/core/src/theming/create.ts#L29-L50、code/core/src/theming/create.ts#L39-L44、code/core/src/theming/create.ts#L45-L50、code/core/src/theming/ensure.ts、code/core/src/theming/ensure.ts#L1-L27、code/core/src/theming/ensure.ts#L10-L27、code/core/src/theming/ensure.ts#L14-L24、code/core/src/theming/themes/dark.ts、code/core/src/theming/themes/dark.ts#L1-L46、code/core/src/theming/themes/dark.ts#L4-L46、code/core/src/theming/themes/light.ts、code/core/src/theming/themes/light.ts#L1-L46、code/core/src/theming/themes/light.ts#L4-L46、code/core/src/theming/types.ts、code/core/src/theming/types.ts#L1-L109、code/core/src/theming/types.ts#L12-L55、code/core/src/theming/types.ts#L39-L55、code/core/src/theming/types.ts#L4-L109、code/core/src/theming/utils.ts、code/core/src/theming/utils.ts#L49-L59、code/core/src/theming/utils.ts#L64-L78
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json#L1-L82、package.json#L67-L72、constants.ts、constants.ts#L1-L18、constants.ts#L3-L17、index.ts、index.ts#L1-L6、index.ts

### 主题和外观/颜色和字体系统.md

- reference 标题：颜色和字体系统
- 生成页：配置参考/主题和外观/颜色和字体系统.md（颜色和字体系统）
- KnowledgeUnit：unit_id=unit-d3de6041cfb7, unit_type=config_doc, domain_id=domain-e15151050742, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.15
- key_source_coverage：0.00
- missing_key_sources：code/__mocks__/fileMock.js、code/__mocks__/fileMock.js#L1-L2、code/__mocks__/styleMock.js、code/__mocks__/styleMock.js#L1-L2、code/addons/a11y/build-config.ts、code/addons/a11y/manager.js、code/addons/a11y/manager.js#L1-L200、code/addons/a11y/preset.js、code/addons/a11y/preview.js、code/addons/a11y/preview.js#L1-L200、code/addons/a11y/src/AccessibilityRuleMaps.ts、code/addons/a11y/src/AccessibilityRuleMaps.ts#L1-L200、code/addons/a11y/src/a11yRunner.test.ts、code/addons/a11y/src/a11yRunner.ts、code/addons/a11y/src/a11yRunner.ts#L1-L200、code/addons/a11y/src/a11yRunnerUtils.ts、code/addons/a11y/src/a11yRunnerUtils.ts#L1-L200、code/addons/a11y/src/axeRuleMappingHelper.ts、code/addons/a11y/src/axeRuleMappingHelper.ts#L1-L200、code/addons/a11y/src/constants.ts、code/addons/a11y/src/constants.ts#L1-L200、code/addons/a11y/src/index.ts、code/addons/a11y/src/params.ts、code/addons/a11y/src/postinstall.ts、code/addons/a11y/src/preset.ts、code/addons/a11y/src/results.mock.ts、code/addons/a11y/src/types.ts、code/addons/a11y/src/types.ts#L1-L200、code/addons/a11y/src/typings.d.ts、code/addons/a11y/src/utils.ts、code/addons/a11y/src/utils.ts#L1-L200、code/addons/a11y/src/visionSimulatorFilters.ts、code/addons/a11y/src/visionSimulatorFilters.ts#L1-L200
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：fileMock.js、fileMock.js#L1-L2、styleMock.js、styleMock.js#L1-L2、build-config.ts、manager.js、manager.js#L1-L200、preset.js

### 多框架支持/Angular框架支持.md

- reference 标题：Angular框架支持
- 生成页：API-参考/Angular框架支持.md（Angular框架支持）
- KnowledgeUnit：unit_id=unit-e74623e69693, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0
- reuse_count：2
- skeleton_score：0.10
- key_source_coverage：0.05
- missing_key_sources：code/frameworks/angular/package.json、code/frameworks/angular/package.json#L1-L124、code/frameworks/angular/package.json#L61-L106、code/frameworks/angular/src/client/decorators.ts、code/frameworks/angular/src/client/decorators.ts#L11-L31、code/frameworks/angular/src/client/decorators.ts#L11-L58、code/frameworks/angular/src/client/decorators.ts#L11-L86、code/frameworks/angular/src/client/decorators.ts#L37-L58、code/frameworks/angular/src/client/index.ts#L1-L11、code/frameworks/angular/src/client/preview.ts、code/frameworks/angular/src/client/preview.ts#L26-L136、code/frameworks/angular/src/client/preview.ts#L26-L258、code/frameworks/angular/src/client/preview.ts#L45-L54、code/frameworks/angular/src/client/preview.ts#L81-L246、code/frameworks/angular/src/client/public-types.ts、code/frameworks/angular/src/client/public-types.ts#L25-L91、code/frameworks/angular/src/client/public-types.ts#L55-L91、code/frameworks/angular/src/client/types.ts、code/frameworks/angular/src/client/types.ts#L9-L50、code/frameworks/angular/src/index.ts#L1-L17、code/frameworks/angular/src/server/framework-preset-angular-cli.ts、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L1-L173、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L119-L159、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L145-L151、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L165-L172、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L17-L173、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L17-L72、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L18-L21、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L38-L71、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L49-L54、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L57-L69、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts#L1-L72、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts#L26-L72、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts#L37-L53、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts#L37-L72、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts#L55-L71、code/frameworks/angular/tsconfig.json、code/frameworks/angular/tsconfig.json#L1-L12、code/frameworks/angular/tsconfig.json#L3-L11
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L124、package.json#L61-L106、decorators.ts、decorators.ts#L11-L31、decorators.ts#L11-L58、decorators.ts#L11-L86、decorators.ts#L37-L58

### 多框架支持/HTML框架支持.md

- reference 标题：HTML框架支持
- 生成页：无
- 问题：缺少对应生成页面

### 多框架支持/React框架支持.md

- reference 标题：React框架支持
- 生成页：多框架支持/多框架支持.md（多框架支持）
- KnowledgeUnit：unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- reuse_count：14
- skeleton_score：0.04
- key_source_coverage：0.17
- missing_key_sources：code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L19-L132、code/.storybook/main.ts#L19-L186、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L402-L415、code/.storybook/storybook.setup.ts、code/frameworks/react-vite/package.json、code/frameworks/react-vite/package.json#L1-L81、code/frameworks/react-vite/package.json#L54-L75、code/frameworks/react-vite/package.json#L70-L75、code/frameworks/react-vite/src/index.ts#L1-L5、code/frameworks/react-vite/src/preset.ts#L1-L51、code/frameworks/react-vite/src/preset.ts#L10-L50、code/frameworks/react-webpack5/package.json、code/frameworks/react-webpack5/package.json#L1-L74、code/frameworks/react-webpack5/package.json#L50-L63、code/frameworks/react-webpack5/package.json#L58-L63、code/frameworks/react-webpack5/src/index.ts#L1-L4、code/frameworks/react-webpack5/src/preset.ts#L1-L46、code/renderers/react/package.json、code/renderers/react/package.json#L1-L97、code/renderers/react/package.json#L54-L86、code/renderers/react/package.json#L81-L91、code/renderers/react/src/preview.tsx、code/renderers/react/src/preview.tsx#L1-L255、code/renderers/react/src/preview.tsx#L101-L255、code/renderers/react/src/preview.tsx#L24-L35、code/renderers/react/src/preview.tsx#L55-L85
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L149-L181、main.ts#L19-L132、main.ts#L19-L186、preview.tsx、preview.tsx#L1-L416、preview.tsx#L195-L334、preview.tsx#L402-L415、storybook.setup.ts

### 多框架支持/Svelte框架支持.md

- reference 标题：Svelte框架支持
- 生成页：多框架支持/svelte-vite.md（svelte-vite）
- KnowledgeUnit：unit_id=unit-663041313566, unit_type=module_doc, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.09
- key_source_coverage：0.12
- missing_key_sources：code/e2e-tests/framework-svelte.spec.ts、code/e2e-tests/framework-svelte.spec.ts#L1-L104、code/frameworks/svelte-vite/package.json、code/frameworks/svelte-vite/package.json#L1-L80、code/frameworks/svelte-vite/package.json#L54-L74、code/frameworks/svelte-vite/src/plugins/svelte-docgen.ts#L1-L147、code/frameworks/svelte-vite/src/plugins/svelte-docgen.ts#L108-L147、code/frameworks/svelte-vite/src/preset.ts、code/frameworks/svelte-vite/src/preset.ts#L1-L32、code/frameworks/svelte-vite/src/preset.ts#L12-L31、code/frameworks/svelte-vite/src/preset.ts#L7-L31、code/frameworks/svelte-vite/src/types.ts、code/frameworks/sveltekit/package.json、code/frameworks/sveltekit/package.json#L1-L79、code/frameworks/sveltekit/package.json#L36-L49、code/frameworks/sveltekit/package.json#L59-L73、code/frameworks/sveltekit/src/index.ts#L1-L5、code/renderers/svelte/package.json、code/renderers/svelte/package.json#L1-L77、code/renderers/svelte/package.json#L55-L71、code/renderers/svelte/src/globals.ts、code/renderers/svelte/src/globals.ts#L1-L2、code/renderers/svelte/src/index.ts#L1-L5、code/renderers/svelte/src/preset.ts、code/renderers/svelte/src/preset.ts#L1-L21、code/renderers/svelte/src/preset.ts#L5-L20、code/renderers/svelte/src/public-types.ts、code/renderers/svelte/src/public-types.ts#L1-L71、code/renderers/svelte/src/public-types.ts#L27-L71
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：framework-svelte.spec.ts、framework-svelte.spec.ts#L1-L104、package.json、package.json#L1-L80、package.json#L54-L74、svelte-docgen.ts#L1-L147、svelte-docgen.ts#L108-L147、preset.ts

### 多框架支持/Vue 3框架支持.md

- reference 标题：Vue 3框架支持
- 生成页：多框架支持/多框架支持.md（多框架支持）
- KnowledgeUnit：unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- reuse_count：14
- skeleton_score：0.07
- key_source_coverage：0.13
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L149-L181、code/frameworks/vue3-vite/package.json、code/frameworks/vue3-vite/package.json#L1-L73、code/frameworks/vue3-vite/package.json#L51-L67、code/frameworks/vue3-vite/src/index.ts#L1-L5、code/frameworks/vue3-vite/src/preset.ts#L1-L57、code/frameworks/vue3-vite/src/preset.ts#L15-L37、code/frameworks/vue3-vite/src/preset.ts#L15-L57、code/frameworks/vue3-vite/src/preset.ts#L24-L31、code/frameworks/vue3-vite/src/types.ts、code/frameworks/vue3-vite/src/types.ts#L1-L105、code/frameworks/vue3-vite/src/types.ts#L17-L48、code/frameworks/vue3-vite/src/vite-plugin.ts、code/frameworks/vue3-vite/src/vite-plugin.ts#L1-L8、code/renderers/vue3/package.json、code/renderers/vue3/package.json#L1-L72、code/renderers/vue3/package.json#L63-L66、code/renderers/vue3/src/globals.ts、code/renderers/vue3/src/globals.ts#L1-L13、code/renderers/vue3/src/globals.ts#L10-L12、code/renderers/vue3/src/globals.ts#L9-L12、code/renderers/vue3/src/index.ts#L1-L10、code/renderers/vue3/src/public-types.ts、code/renderers/vue3/src/public-types.ts#L1-L84、code/renderers/vue3/src/public-types.ts#L29-L63、code/renderers/vue3/src/render.ts#L1-L178、code/renderers/vue3/src/render.ts#L10-L114、code/renderers/vue3/src/render.ts#L10-L178、code/renderers/vue3/src/render.ts#L45-L114、code/renderers/vue3/src/render.ts#L91-L106、code/renderers/vue3/src/types.ts、code/renderers/vue3/src/types.ts#L1-L38、code/renderers/vue3/src/types.ts#L24-L35
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L149-L181、package.json、package.json#L1-L73、package.json#L51-L67、index.ts#L1-L5、preset.ts#L1-L57、preset.ts#L15-L37

### 多框架支持/Web Components框架支持.md

- reference 标题：Web Components框架支持
- 生成页：平台绑定-Web/web-components.md（web-components）
- KnowledgeUnit：unit_id=unit-e7060a424501, unit_type=module_doc, domain_id=domain-c86d32da5da8, readiness=compose_ready, child_digests=0
- reuse_count：4
- skeleton_score：0.10
- key_source_coverage：0.12
- missing_key_sources：code/.storybook/main.ts、code/frameworks/web-components-vite/package.json、code/frameworks/web-components-vite/package.json#L1-L68、code/frameworks/web-components-vite/package.json#L52-L55、code/frameworks/web-components-vite/src/index.ts#L1-L5、code/renderers/web-components/package.json、code/renderers/web-components/package.json#L1-L73、code/renderers/web-components/package.json#L49-L67、code/renderers/web-components/src/framework-api.ts、code/renderers/web-components/src/framework-api.ts#L1-L40、code/renderers/web-components/src/framework-api.ts#L13-L26、code/renderers/web-components/src/framework-api.ts#L28-L40、code/renderers/web-components/src/globals.ts、code/renderers/web-components/src/index.ts#L1-L36、code/renderers/web-components/src/index.ts#L14-L35、code/renderers/web-components/src/preview.ts#L1-L268、code/renderers/web-components/src/preview.ts#L25-L53、code/renderers/web-components/src/preview.ts#L85-L267、code/renderers/web-components/src/public-types.ts、code/renderers/web-components/src/types.ts、code/renderers/web-components/src/types.ts#L1-L25、code/renderers/web-components/src/types.ts#L14-L25
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、package.json、package.json#L1-L68、package.json#L52-L55、index.ts#L1-L5、package.json、package.json#L1-L73、package.json#L49-L67

### 多框架支持/多框架支持.md

- reference 标题：多框架支持
- 生成页：多框架支持/多框架支持.md（多框架支持）
- KnowledgeUnit：unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- reuse_count：14
- skeleton_score：0.07
- key_source_coverage：0.08
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L169-L181、code/.storybook/main.ts#L19-L132、code/.storybook/main.ts#L19-L183、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L195-L415、code/.storybook/preview.tsx#L402-L415、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L11-L29、code/frameworks/angular/package.json、code/frameworks/angular/package.json#L27-L47、code/frameworks/angular/package.json#L61-L68、code/frameworks/html-vite/package.json#L28-L42、code/frameworks/react-vite/package.json、code/frameworks/react-vite/package.json#L28-L44、code/frameworks/react-vite/package.json#L54-L64、code/frameworks/react-vite/src/index.ts#L1-L5、code/frameworks/svelte-vite/package.json、code/frameworks/svelte-vite/package.json#L28-L44、code/frameworks/svelte-vite/package.json#L54-L58、code/frameworks/vue3-vite/package.json、code/frameworks/vue3-vite/package.json#L28-L42、code/frameworks/vue3-vite/package.json#L51-L58、code/frameworks/vue3-vite/src/index.ts#L1-L5、code/frameworks/web-components-vite/package.json、code/frameworks/web-components-vite/package.json#L30-L42、code/frameworks/web-components-vite/package.json#L52-L55、package.json、package.json#L18-L31、package.json#L5-L16
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L149-L181、main.ts#L169-L181、main.ts#L19-L132、main.ts#L19-L183、manager.tsx、manager.tsx#L4-L8、preview.tsx

### 快速开始.md

- reference 标题：快速开始
- 生成页：多框架支持/react-vitest-3.md（react-vitest-3）
- KnowledgeUnit：unit_id=unit-ed51639dbf62, unit_type=module_doc, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=0
- reuse_count：6
- skeleton_score：0.13
- key_source_coverage：0.05
- missing_key_sources：README.md、README.md#L61-L94、code/.storybook/main.ts#L129-L132、code/.storybook/main.ts#L169-L180、code/.storybook/main.ts#L19-L183、code/.storybook/preview.tsx#L144-L415、code/.storybook/preview.tsx#L402-L415、code/addons/docs/src/blocks/blocks/Canvas.stories.tsx、code/addons/docs/src/blocks/blocks/Canvas.stories.tsx#L1-L200、code/addons/docs/src/blocks/blocks/Canvas.stories.tsx#L1-L38、code/addons/docs/src/blocks/blocks/Canvas.stories.tsx#L13-L38、code/addons/docs/src/blocks/blocks/Controls.stories.tsx、code/addons/docs/src/blocks/blocks/Controls.stories.tsx#L1-L162、code/addons/docs/src/blocks/blocks/Controls.stories.tsx#L1-L26、code/addons/docs/src/blocks/blocks/Controls.stories.tsx#L14-L26、code/addons/docs/src/blocks/blocks/Story.stories.tsx、code/addons/docs/src/blocks/blocks/Story.stories.tsx#L1-L185、code/addons/docs/src/blocks/blocks/Story.stories.tsx#L1-L21、code/addons/docs/src/blocks/blocks/Story.stories.tsx#L12-L21、code/core/template/stories/preview.ts、code/frameworks/angular/package.json、code/frameworks/angular/package.json#L61-L106、code/frameworks/angular/package.json#L90-L106、code/frameworks/react-vite/package.json、code/frameworks/react-vite/package.json#L54-L75、code/frameworks/react-vite/package.json#L70-L75、code/frameworks/vue3-vite/package.json、code/frameworks/vue3-vite/package.json#L51-L67、code/frameworks/vue3-vite/package.json#L64-L67、code/package.json、code/package.json#L15-L45、code/package.json#L60-L70、code/package.json#L71-L151、package.json、package.json#L1-L74、package.json#L18-L38、package.json#L5-L17、package.json#L72-L73、scripts/package.json、scripts/package.json#L1-L189、scripts/package.json#L6-L43
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：README.md、README.md#L61-L94、main.ts#L129-L132、main.ts#L169-L180、main.ts#L19-L183、preview.tsx#L144-L415、preview.tsx#L402-L415、Canvas.stories.tsx

### 插件系统/Addons概览.md

- reference 标题：Addons概览
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.03
- missing_key_sources：code/.storybook/main.ts#L107-L116、code/.storybook/main.ts#L19-L116、code/addons/a11y/package.json、code/addons/a11y/package.json#L37-L52、code/addons/a11y/src/index.ts、code/addons/a11y/src/index.ts#L1-L11、code/addons/a11y/src/preview.tsx、code/addons/a11y/src/preview.tsx#L1-L110、code/addons/a11y/src/preview.tsx#L14-L110、code/addons/a11y/src/preview.tsx#L14-L96、code/addons/docs/package.json、code/addons/docs/package.json#L40-L80、code/addons/docs/src/preview.ts、code/addons/docs/src/preview.ts#L1-L30、code/addons/docs/src/preview.ts#L14-L29、code/addons/links/package.json、code/addons/links/package.json#L29-L47、code/addons/links/src/preview.ts、code/addons/links/src/preview.ts#L1-L4、code/addons/onboarding/package.json、code/addons/onboarding/package.json#L33-L41、code/addons/onboarding/src/index.ts、code/addons/onboarding/src/index.ts#L1-L3、code/addons/pseudo-states/package.json、code/addons/pseudo-states/package.json#L35-L47、code/addons/pseudo-states/src/preview.ts、code/addons/pseudo-states/src/preview.ts#L1-L9、code/addons/themes/package.json、code/addons/themes/package.json#L36-L49、code/addons/themes/src/preview.ts、code/addons/themes/src/preview.ts#L1-L8、code/addons/vitest/package.json、code/addons/vitest/package.json#L40-L65、code/addons/vitest/src/index.ts、code/addons/vitest/src/index.ts#L1-L4
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L107-L116、main.ts#L19-L116、package.json、package.json#L37-L52、index.ts、index.ts#L1-L11、preview.tsx、preview.tsx#L1-L110

### 插件系统/插件系统.md

- reference 标题：插件系统
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- KnowledgeUnit：unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：14
- skeleton_score：0.05
- key_source_coverage：0.16
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L107-L116、code/.storybook/main.ts#L19-L116、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L10-L415、code/.storybook/preview.tsx#L16-L16、code/.storybook/preview.tsx#L18-L21、code/.storybook/preview.tsx#L220-L301、code/.storybook/preview.tsx#L358-L384、code/.storybook/preview.tsx#L388-L399、code/.storybook/preview.tsx#L402-L415、code/addons/a11y/package.json、code/addons/a11y/package.json#L37-L51、code/addons/a11y/package.json#L60-L77、code/addons/a11y/src/index.ts#L1-L11、code/addons/a11y/src/preview.tsx、code/addons/a11y/src/preview.tsx#L1-L110、code/addons/a11y/src/preview.tsx#L14-L109、code/addons/a11y/src/preview.tsx#L14-L96、code/addons/docs/package.json、code/addons/docs/package.json#L40-L74、code/addons/docs/package.json#L88-L120、code/addons/docs/src/index.ts#L1-L21、code/addons/docs/src/preview.ts#L1-L30、code/addons/docs/src/preview.ts#L14-L29、code/addons/links/package.json、code/addons/links/package.json#L29-L41、code/addons/links/package.json#L55-L64、code/addons/links/src/index.ts#L1-L8、code/addons/links/src/preview.ts#L1-L4、code/addons/themes/package.json、code/addons/themes/package.json#L36-L49、code/addons/themes/package.json#L58-L69、code/addons/themes/src/index.ts#L1-L11、code/addons/themes/src/preview.ts#L1-L8、code/addons/themes/src/preview.ts#L5-L7
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L107-L116、main.ts#L19-L116、manager.tsx、manager.tsx#L4-L8、preview.tsx、preview.tsx#L10-L415、preview.tsx#L16-L16

### 插件系统/核心Addons详解/A11y Addon（可访问性测试）.md

- reference 标题：A11y Addon（可访问性测试）
- 生成页：插件生态/插件生态.md（插件生态）
- KnowledgeUnit：unit_id=unit-e6f7b25bcf70, unit_type=domain_index, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=9
- reuse_count：1
- skeleton_score：0.07
- key_source_coverage：0.02
- missing_key_sources：code/addons/a11y/package.json、code/addons/a11y/package.json#L60-L74、code/addons/a11y/src/a11yRunner.ts、code/addons/a11y/src/a11yRunner.ts#L1-L136、code/addons/a11y/src/a11yRunner.ts#L18-L121、code/addons/a11y/src/a11yRunner.ts#L18-L22、code/addons/a11y/src/a11yRunner.ts#L43-L121、code/addons/a11y/src/a11yRunnerUtils.ts、code/addons/a11y/src/a11yRunnerUtils.ts#L1-L35、code/addons/a11y/src/a11yRunnerUtils.ts#L10-L34、code/addons/a11y/src/components/A11YPanel.tsx、code/addons/a11y/src/components/A11YPanel.tsx#L1-L235、code/addons/a11y/src/components/A11YPanel.tsx#L146-L235、code/addons/a11y/src/components/A11YPanel.tsx#L56-L235、code/addons/a11y/src/components/A11yContext.tsx、code/addons/a11y/src/components/A11yContext.tsx#L1-L486、code/addons/a11y/src/components/A11yContext.tsx#L275-L289、code/addons/a11y/src/components/A11yContext.tsx#L38-L87、code/addons/a11y/src/components/A11yContext.tsx#L431-L449、code/addons/a11y/src/components/VisionSimulator.tsx、code/addons/a11y/src/components/VisionSimulator.tsx#L1-L68、code/addons/a11y/src/components/VisionSimulator.tsx#L38-L68、code/addons/a11y/src/constants.ts、code/addons/a11y/src/constants.ts#L1-L22、code/addons/a11y/src/index.ts#L1-L11、code/addons/a11y/src/manager.tsx、code/addons/a11y/src/manager.tsx#L1-L67、code/addons/a11y/src/manager.tsx#L50-L66、code/addons/a11y/src/params.ts、code/addons/a11y/src/params.ts#L1-L45、code/addons/a11y/src/params.ts#L20-L44、code/addons/a11y/src/preview.tsx、code/addons/a11y/src/preview.tsx#L1-L110、code/addons/a11y/src/preview.tsx#L14-L110、code/addons/a11y/src/preview.tsx#L14-L96、code/addons/a11y/src/types.ts、code/addons/a11y/src/types.ts#L1-L68、code/addons/a11y/src/utils.ts、code/addons/a11y/src/utils.ts#L1-L18、code/addons/a11y/src/withVisionSimulator.ts、code/addons/a11y/src/withVisionSimulator.ts#L1-L53、code/addons/a11y/src/withVisionSimulator.ts#L10-L53、code/addons/vitest/src/components/TestProviderRender.tsx、code/addons/vitest/src/components/TestProviderRender.tsx#L130-L143、code/addons/vitest/src/node/test-manager.ts、code/addons/vitest/src/node/test-manager.ts#L231-L277、docs/writing-tests/accessibility-testing.mdx、docs/writing-tests/accessibility-testing.mdx#L199-L213、docs/writing-tests/accessibility-testing.mdx#L96-L118
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L60-L74、a11yRunner.ts、a11yRunner.ts#L1-L136、a11yRunner.ts#L18-L121、a11yRunner.ts#L18-L22、a11yRunner.ts#L43-L121、a11yRunnerUtils.ts

### 插件系统/核心Addons详解/Actions Addon（动作记录）.md

- reference 标题：Actions Addon（动作记录）
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- KnowledgeUnit：unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：14
- skeleton_score：0.13
- key_source_coverage：0.05
- missing_key_sources：code/core/core/src/actions/loaders.ts、code/core/src/actions/addArgs.ts、code/core/src/actions/addArgs.ts#L1-L9、code/core/src/actions/addArgs.ts#L5-L9、code/core/src/actions/addArgsHelpers.ts、code/core/src/actions/addArgsHelpers.ts#L1-L62、code/core/src/actions/addArgsHelpers.ts#L18-L62、code/core/src/actions/constants.ts、code/core/src/actions/constants.ts#L1-L6、code/core/src/actions/containers/ActionLogger/index.tsx#L1-L77、code/core/src/actions/containers/ActionLogger/index.tsx#L27-L77、code/core/src/actions/containers/ActionLogger/index.tsx#L51-L55、code/core/src/actions/containers/ActionLogger/index.tsx#L57-L77、code/core/src/actions/decorator.ts、code/core/src/actions/decorator.ts#L1-L72、code/core/src/actions/decorator.ts#L24-L72、code/core/src/actions/decorator.ts#L59-L72、code/core/src/actions/index.ts#L1-L4、code/core/src/actions/loaders.ts#L1-L55、code/core/src/actions/loaders.ts#L9-L55、code/core/src/actions/manager.tsx、code/core/src/actions/manager.tsx#L1-L19、code/core/src/actions/manager.tsx#L9-L18、code/core/src/actions/models/ActionDisplay.ts、code/core/src/actions/models/ActionDisplay.ts#L1-L12、code/core/src/actions/models/ActionOptions.ts、code/core/src/actions/models/ActionOptions.ts#L1-L12、code/core/src/actions/models/ActionsFunction.ts、code/core/src/actions/models/ActionsFunction.ts#L1-L82、code/core/src/actions/models/ActionsFunction.ts#L4-L82、code/core/src/actions/preview.ts#L1-L14、code/core/src/actions/preview.ts#L9-L14、code/core/src/actions/runtime/action.ts、code/core/src/actions/runtime/action.ts#L1-L105、code/core/src/actions/runtime/action.ts#L50-L105、code/core/src/actions/runtime/action.ts#L57-L79、code/core/src/actions/runtime/action.ts#L81-L99、code/core/src/actions/runtime/actions.ts、code/core/src/actions/runtime/actions.ts#L1-L34、code/core/src/actions/runtime/actions.ts#L5-L34、code/e2e-tests/addon-actions.spec.ts、code/e2e-tests/addon-actions.spec.ts#L10-L63
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：loaders.ts、addArgs.ts、addArgs.ts#L1-L9、addArgs.ts#L5-L9、addArgsHelpers.ts、addArgsHelpers.ts#L1-L62、addArgsHelpers.ts#L18-L62、constants.ts

### 插件系统/核心Addons详解/Backgrounds Addon（背景设置）.md

- reference 标题：Backgrounds Addon（背景设置）
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- KnowledgeUnit：unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：14
- skeleton_score：0.11
- key_source_coverage：0.02
- missing_key_sources：code/core/src/backgrounds/components/Tool.tsx、code/core/src/backgrounds/components/Tool.tsx#L1-L112、code/core/src/backgrounds/components/Tool.tsx#L13-L112、code/core/src/backgrounds/components/Tool.tsx#L13-L43、code/core/src/backgrounds/constants.ts、code/core/src/backgrounds/constants.ts#L1-L7、code/core/src/backgrounds/decorator.ts、code/core/src/backgrounds/decorator.ts#L1-L102、code/core/src/backgrounds/decorator.ts#L21-L101、code/core/src/backgrounds/decorator.ts#L37-L53、code/core/src/backgrounds/defaults.ts、code/core/src/backgrounds/defaults.ts#L1-L7、code/core/src/backgrounds/defaults.ts#L3-L6、code/core/src/backgrounds/manager.tsx、code/core/src/backgrounds/manager.tsx#L1-L18、code/core/src/backgrounds/manager.tsx#L8-L17、code/core/src/backgrounds/preview.ts#L1-L37、code/core/src/backgrounds/preview.ts#L14-L27、code/core/src/backgrounds/preview.ts#L31-L36、code/core/src/backgrounds/types.ts、code/core/src/backgrounds/types.ts#L1-L60、code/core/src/backgrounds/types.ts#L21-L59、code/core/src/backgrounds/utils.ts、code/core/src/backgrounds/utils.ts#L1-L70、code/core/src/backgrounds/utils.ts#L45-L69、code/core/template/stories/backgrounds/globals.stories.ts、code/core/template/stories/backgrounds/globals.stories.ts#L1-L105、code/core/template/stories/backgrounds/globals.stories.ts#L19-L105、code/e2e-tests/addon-backgrounds.spec.ts、code/e2e-tests/addon-backgrounds.spec.ts#L1-L63、code/e2e-tests/addon-backgrounds.spec.ts#L18-L61、code/presets/server-webpack/src/lib/compiler/__testfixtures__/backgrounds.json、code/presets/server-webpack/src/lib/compiler/__testfixtures__/backgrounds.json#L1-L17、code/presets/server-webpack/src/lib/compiler/__testfixtures__/backgrounds.json#L3-L8、docs/essentials/backgrounds.mdx、docs/essentials/backgrounds.mdx#L1-L37、docs/essentials/backgrounds.mdx#L12-L37、test-storybooks/server-kitchen-sink/stories/addons/backgrounds.stories.json、test-storybooks/server-kitchen-sink/stories/addons/backgrounds.stories.json#L1-L25、test-storybooks/server-kitchen-sink/stories/addons/backgrounds.stories.json#L2-L9
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：Tool.tsx、Tool.tsx#L1-L112、Tool.tsx#L13-L112、Tool.tsx#L13-L43、constants.ts、constants.ts#L1-L7、decorator.ts、decorator.ts#L1-L102

### 插件系统/核心Addons详解/Controls Addon（参数控制）.md

- reference 标题：Controls Addon（参数控制）
- 生成页：概念指南/essentials/Controls.md（Controls）
- KnowledgeUnit：unit_id=unit-0d705d91e2e7, unit_type=concept_guide, domain_id=domain-be9fa08103e5, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.64
- key_source_coverage：0.00
- missing_key_sources：CHANGELOG.md、MIGRATION.md、MIGRATION.md#L984、code/addons/docs/src/blocks/controls/ArgControl.tsx、code/addons/docs/src/blocks/controls/ArgControl.tsx#L1-L20、code/addons/onboarding/src/constants.ts、code/addons/onboarding/src/constants.ts#L4、code/core/src/controls/README.md、code/core/src/controls/README.md#L1-L23、code/core/src/controls/constants.ts、code/core/src/controls/constants.ts#L1-L4、code/core/src/controls/preview.ts、code/core/src/controls/preview.ts#L1-L10、code/core/src/manager/App.tsx、code/core/src/manager/App.tsx#L63-L81、code/core/src/manager/components/preview/Preview.tsx、code/core/src/manager/components/preview/Preview.tsx#L236-L277、code/core/src/manager/container/Panel.tsx、code/core/src/manager/container/Panel.tsx#L47-L81、code/e2e-tests/addon-controls.spec.ts、code/e2e-tests/addon-controls.spec.ts#L1-L121、code/e2e-tests/addon-controls.spec.ts#L10-L121、code/e2e-tests/addon-controls.spec.ts#L10-L78、code/e2e-tests/addon-controls.spec.ts#L26-L78、docs/api/arg-types.mdx、docs/api/arg-types.mdx#L85-L125、docs/essentials/controls.mdx、docs/essentials/controls.mdx#L282-L297
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CHANGELOG.md、MIGRATION.md、MIGRATION.md#L984、ArgControl.tsx、ArgControl.tsx#L1-L20、constants.ts、constants.ts#L4、README.md

### 插件系统/核心Addons详解/Docs Addon（文档生成）.md

- reference 标题：Docs Addon（文档生成）
- 生成页：平台绑定-Web/web-components.md（web-components）
- KnowledgeUnit：unit_id=unit-e7060a424501, unit_type=module_doc, domain_id=domain-c86d32da5da8, readiness=compose_ready, child_digests=0
- reuse_count：4
- skeleton_score：0.11
- key_source_coverage：0.11
- missing_key_sources：code/addons/docs/package.json、code/addons/docs/package.json#L1-L133、code/addons/docs/package.json#L40-L79、code/addons/docs/package.json#L88-L117、code/addons/docs/src/DocsRenderer.tsx、code/addons/docs/src/DocsRenderer.tsx#L1-L88、code/addons/docs/src/DocsRenderer.tsx#L18-L22、code/addons/docs/src/DocsRenderer.tsx#L24-L46、code/addons/docs/src/DocsRenderer.tsx#L48-L87、code/addons/docs/src/DocsRenderer.tsx#L54-L81、code/addons/docs/src/DocsRenderer.tsx#L59-L62、code/addons/docs/src/blocks.ts、code/addons/docs/src/blocks.ts#L1-L10、code/addons/docs/src/index.ts#L1-L21、code/addons/docs/src/preset.ts#L31-L67、code/addons/docs/src/preset.ts#L46-L58、code/addons/docs/src/preset.ts#L49-L60、code/addons/docs/src/preview.ts#L1-L30、code/addons/docs/src/preview.ts#L14-L29、code/addons/docs/src/preview.ts#L20-L27、code/addons/docs/src/preview.ts#L3-L12、docs/_snippets/addon-docs-options.md、docs/_snippets/addon-docs-options.md#L1-L68、docs/_snippets/storybook-builder-api-mdx.md、docs/_snippets/storybook-builder-api-mdx.md#L1-L18
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L133、package.json#L40-L79、package.json#L88-L117、DocsRenderer.tsx、DocsRenderer.tsx#L1-L88、DocsRenderer.tsx#L18-L22、DocsRenderer.tsx#L24-L46

### 插件系统/核心Addons详解/Links Addon（故事导航）.md

- reference 标题：Links Addon（故事导航）
- 生成页：插件生态/links.md（links）
- KnowledgeUnit：unit_id=unit-78445132ba70, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.11
- key_source_coverage：0.17
- missing_key_sources：code/addons/links/README.md、code/addons/links/README.md#L119-L147、code/addons/links/README.md#L23-L34、code/addons/links/README.md#L57-L76、code/addons/links/README.md#L82-L95、code/addons/links/README.md#L99-L116、code/addons/links/package.json、code/addons/links/package.json#L1-L82、code/addons/links/src/constants.ts、code/addons/links/src/constants.ts#L1-L9、code/addons/links/src/constants.ts#L4-L8、code/addons/links/src/index.ts#L1-L8、code/addons/links/src/manager.ts#L1-L11、code/addons/links/src/manager.ts#L5-L10、code/addons/links/src/preview.ts#L1-L4、code/addons/links/src/react/components/link.test.tsx#L42-L99、code/addons/links/src/react/components/link.test.tsx#L68-L99、code/addons/links/src/react/components/link.tsx#L1-L7、code/addons/links/src/react/components/link.tsx#L1-L88、code/addons/links/src/react/components/link.tsx#L14-L22、code/addons/links/src/react/components/link.tsx#L24-L88、code/addons/links/src/react/components/link.tsx#L36-L88、code/addons/links/src/react/components/link.tsx#L62-L75、code/addons/links/src/utils.test.ts#L26-L81、code/addons/links/src/utils.test.ts#L66-L81、code/addons/links/src/utils.test.ts#L83-L88、code/addons/links/src/utils.ts#L1-L11、code/addons/links/src/utils.ts#L1-L119、code/addons/links/src/utils.ts#L110-L119、code/addons/links/src/utils.ts#L36-L119、code/addons/links/src/utils.ts#L36-L37、code/addons/links/src/utils.ts#L39-L56、code/addons/links/src/utils.ts#L39-L80、code/addons/links/src/utils.ts#L58-L80、code/addons/links/src/utils.ts#L61-L80、code/addons/links/src/utils.ts#L82-L119、code/addons/links/template/stories/decorator.stories.ts、code/addons/links/template/stories/decorator.stories.ts#L26-L54、code/addons/links/template/stories/hrefto.stories.ts、code/addons/links/template/stories/hrefto.stories.ts#L14-L22
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：README.md、README.md#L119-L147、README.md#L23-L34、README.md#L57-L76、README.md#L82-L95、README.md#L99-L116、package.json、package.json#L1-L82

### 插件系统/核心Addons详解/Outline_Measure_Toolbar Addons（辅助工具）.md

- reference 标题：Outline/Measure/Toolbar Addons（辅助工具）
- 生成页：概念指南/addons.md（Addons）
- KnowledgeUnit：unit_id=unit-eade5822b8ba, unit_type=concept_guide, domain_id=domain-be9fa08103e5, readiness=compose_ready, child_digests=8
- reuse_count：2
- skeleton_score：0.53
- key_source_coverage：0.00
- missing_key_sources：code/core/src/components/components/Tabs/Tabs.hooks.tsx、code/core/src/core-server/presets/common-manager.ts、code/core/src/core-server/presets/common-preset.ts、code/core/src/manager-api/modules/layout.ts、code/core/src/manager-api/modules/layout.ts#L306-L314、code/core/src/manager-api/modules/layout.ts#L464-L471、code/core/src/manager-api/modules/layout.ts#L67-L72、code/core/src/manager-api/modules/shortcuts.ts、code/core/src/manager-api/modules/shortcuts.ts#L101-L101、code/core/src/measure/Tool.tsx、code/core/src/measure/Tool.tsx#L1-L49、code/core/src/measure/Tool.tsx#L11-L48、code/core/src/measure/Tool.tsx#L16-L32、code/core/src/measure/Tool.tsx#L24-L32、code/core/src/measure/constants.ts、code/core/src/measure/constants.ts#L1-L10、code/core/src/measure/constants.ts#L5-L9、code/core/src/measure/manager.tsx、code/core/src/measure/manager.tsx#L1-L18、code/core/src/measure/manager.tsx#L13-L13、code/core/src/measure/manager.tsx#L8-L17、code/core/src/measure/manager.tsx#L9-L16、code/core/src/measure/preview.ts、code/core/src/measure/preview.ts#L1-L20、code/core/src/measure/preview.ts#L7-L11、code/core/src/measure/preview.ts#L7-L19、code/core/src/outline/OutlineSelector.tsx、code/core/src/outline/OutlineSelector.tsx#L1-L50、code/core/src/outline/OutlineSelector.tsx#L11-L49、code/core/src/outline/OutlineSelector.tsx#L15-L33、code/core/src/outline/OutlineSelector.tsx#L25-L33、code/core/src/outline/constants.ts、code/core/src/outline/constants.ts#L1-L3、code/core/src/outline/manager.tsx、code/core/src/outline/manager.tsx#L1-L18、code/core/src/outline/manager.tsx#L13-L13、code/core/src/outline/manager.tsx#L8-L17、code/core/src/outline/manager.tsx#L9-L16、code/core/src/outline/preview.ts、code/core/src/outline/preview.ts#L1-L16、code/core/src/outline/preview.ts#L7-L11、code/core/src/outline/preview.ts#L7-L15、code/core/src/outline/types.ts、code/core/src/outline/types.ts#L1-L21、code/core/src/outline/types.ts#L7-L15、code/core/src/toolbar/index.ts、code/core/src/toolbar/index.ts#L1-L3、docs/_snippets/storybook-addon-toolkit-types.md、docs/_snippets/storybook-addon-toolkit-types.md#L1-L46、docs/_snippets/storybook-addon-toolkit-types.md#L22-L30、docs/_snippets/storybook-addons-api-useapi.md、docs/_snippets/storybook-addons-api-useapi.md#L1-L45、docs/_snippets/storybook-addons-api-useapi.md#L20-L28、docs/addons/writing-addons.mdx、docs/addons/writing-addons.mdx#L93-L121、docs/addons/writing-addons.mdx#L93-L136、docs/essentials/measure-and-outline.mdx、docs/essentials/measure-and-outline.mdx#L1-L26、docs/essentials/measure-and-outline.mdx#L10-L21、docs/essentials/measure-and-outline.mdx#L22-L26
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：Tabs.hooks.tsx、common-manager.ts、common-preset.ts、layout.ts、layout.ts#L306-L314、layout.ts#L464-L471、layout.ts#L67-L72、shortcuts.ts

### 插件系统/核心Addons详解/Themes Addon（主题切换）.md

- reference 标题：Themes Addon（主题切换）
- 生成页：主题系统/themes.md（themes）
- KnowledgeUnit：unit_id=unit-e858919ec1df, unit_type=module_doc, domain_id=domain-9cda34dbf62b, readiness=compose_ready, child_digests=0
- reuse_count：6
- skeleton_score：0.19
- key_source_coverage：0.11
- missing_key_sources：code/addons/themes/README.md、code/addons/themes/README.md#L37-L67、code/addons/themes/README.md#L9-L21、code/addons/themes/build-config.ts、code/addons/themes/docs/api.md、code/addons/themes/docs/api.md#L1-L213、code/addons/themes/docs/api.md#L5-L86、code/addons/themes/docs/getting-started/bootstrap.md、code/addons/themes/docs/getting-started/emotion.md、code/addons/themes/docs/getting-started/material-ui.md、code/addons/themes/docs/getting-started/postcss.md、code/addons/themes/docs/getting-started/styled-components.md、code/addons/themes/docs/getting-started/tailwind.md、code/addons/themes/package.json、code/addons/themes/project.json、code/addons/themes/src/constants.ts、code/addons/themes/src/constants.ts#L1-L18、code/addons/themes/src/decorators/class-name.decorator.tsx#L1-L57、code/addons/themes/src/decorators/class-name.decorator.tsx#L19-L56、code/addons/themes/src/decorators/class-name.decorator.tsx#L26-L52、code/addons/themes/src/decorators/class-name.decorator.tsx#L30-L52、code/addons/themes/src/decorators/data-attribute.decorator.tsx、code/addons/themes/src/decorators/data-attribute.decorator.tsx#L1-L42、code/addons/themes/src/decorators/data-attribute.decorator.tsx#L19-L41、code/addons/themes/src/decorators/data-attribute.decorator.tsx#L26-L37、code/addons/themes/src/decorators/data-attribute.decorator.tsx#L30-L37、code/addons/themes/src/decorators/helpers.ts、code/addons/themes/src/decorators/helpers.ts#L1-L40、code/addons/themes/src/decorators/helpers.ts#L16-L32、code/addons/themes/src/decorators/helpers.ts#L34-L39、code/addons/themes/src/decorators/index.ts#L1-L6、code/addons/themes/src/decorators/provider.decorator.tsx#L1-L64、code/addons/themes/src/decorators/provider.decorator.tsx#L23-L63、code/addons/themes/src/decorators/provider.decorator.tsx#L35-L61、code/addons/themes/src/decorators/provider.decorator.tsx#L40-L61、code/addons/themes/src/index.ts#L1-L11、code/addons/themes/src/preview.ts#L1-L8、code/addons/themes/src/types.ts、code/addons/themes/src/types.ts#L1-L29
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：README.md、README.md#L37-L67、README.md#L9-L21、build-config.ts、api.md、api.md#L1-L213、api.md#L5-L86、bootstrap.md

### 插件系统/核心Addons详解/核心Addons详解.md

- reference 标题：核心Addons详解
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- KnowledgeUnit：unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：14
- skeleton_score：0.05
- key_source_coverage：0.00
- missing_key_sources：.yarnrc.yml、code/.storybook/main.ts、code/.storybook/manager.tsx、code/.storybook/preview.tsx、code/addons/a11y/package.json、code/addons/a11y/package.json#L1-L90、code/addons/a11y/package.json#L37-L52、code/addons/docs/package.json、code/addons/docs/package.json#L1-L133、code/addons/docs/package.json#L40-L80、code/addons/links/package.json、code/addons/links/package.json#L1-L82、code/addons/links/package.json#L29-L47、code/addons/themes/package.json、code/addons/themes/package.json#L1-L82、code/addons/themes/package.json#L36-L50、code/e2e-tests/addon-a11y.spec.ts、code/e2e-tests/addon-backgrounds.spec.ts、code/e2e-tests/addon-controls.spec.ts、code/e2e-tests/addon-docs.spec.ts、code/e2e-tests/addon-links.spec.ts、code/e2e-tests/addon-toolbars.spec.ts、docs/addons/docs/、docs/writing-docs/
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：.yarnrc.yml、main.ts、manager.tsx、preview.tsx、package.json、package.json#L1-L90、package.json#L37-L52、package.json

### 插件系统/第三方Addons集成.md

- reference 标题：第三方Addons集成
- 生成页：概念指南/addons.md（Addons）
- KnowledgeUnit：unit_id=unit-eade5822b8ba, unit_type=concept_guide, domain_id=domain-be9fa08103e5, readiness=compose_ready, child_digests=8
- reuse_count：2
- skeleton_score：0.44
- key_source_coverage：0.00
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L107-L116、code/.storybook/main.ts#L117-L120、code/.storybook/main.ts#L129-L132、code/.storybook/main.ts#L133-L139、code/.storybook/main.ts#L133-L147、code/.storybook/main.ts#L149-L181、code/addons/a11y/package.json、code/addons/a11y/package.json#L1-L90、code/addons/a11y/package.json#L37-L52、code/addons/a11y/package.json#L75-L77、code/addons/a11y/package.json#L85-L87、code/addons/a11y/src/index.ts、code/addons/a11y/src/index.ts#L1-L11、code/addons/docs/package.json、code/addons/docs/package.json#L1-L133、code/addons/docs/package.json#L118-L120、code/addons/docs/package.json#L128-L130、code/addons/docs/package.json#L40-L80、code/addons/docs/src/index.ts、code/addons/docs/src/index.ts#L1-L21、code/package.json、code/package.json#L1-L219、code/package.json#L74-L80、docs/addons/addon-types.mdx、docs/addons/addon-types.mdx#L10-L62、docs/addons/index.mdx、docs/addons/index.mdx#L12-L12、docs/addons/index.mdx#L31-L45、docs/addons/writing-addons.mdx、docs/addons/writing-addons.mdx#L16-L35、docs/addons/writing-addons.mdx#L173-L285、package.json、package.json#L1-L74、package.json#L40-L60、package.json#L5-L16
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L1-L186、main.ts#L107-L116、main.ts#L117-L120、main.ts#L129-L132、main.ts#L133-L139、main.ts#L133-L147、main.ts#L149-L181

### 插件系统/自定义Addons开发.md

- reference 标题：自定义Addons开发
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- KnowledgeUnit：unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：14
- skeleton_score：0.13
- key_source_coverage：0.12
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L1-L200、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L200、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L200、code/.storybook/storybook.setup.ts、code/addons/a11y/package.json、code/addons/a11y/package.json#L1-L90、code/addons/a11y/package.json#L37-L52、code/addons/a11y/src/index.ts#L1-L11、code/addons/a11y/src/manager.tsx、code/addons/a11y/src/manager.tsx#L1-L67、code/addons/a11y/src/manager.tsx#L50-L67、code/addons/a11y/src/preview.tsx、code/addons/a11y/src/preview.tsx#L1-L110、code/addons/a11y/src/preview.tsx#L14-L110、code/addons/docs/package.json、code/addons/docs/package.json#L1-L133、code/addons/docs/package.json#L40-L80、code/addons/docs/src/index.ts#L1-L21、code/addons/docs/src/preview.ts#L1-L30、code/addons/docs/src/preview.ts#L14-L30、code/addons/links/package.json、code/addons/links/package.json#L1-L82、code/addons/links/package.json#L29-L47、code/addons/pseudo-states/src/index.ts#L1-L11、code/addons/themes/package.json、code/addons/themes/package.json#L1-L82、code/addons/themes/package.json#L36-L50、code/addons/vitest/src/index.ts#L1-L4、code/e2e-tests/addon-a11y.spec.ts、code/e2e-tests/addon-a11y.spec.ts#L1-L200、code/e2e-tests/manager.spec.ts、code/e2e-tests/manager.spec.ts#L1-L200、code/e2e-tests/preview-api.spec.ts、code/e2e-tests/preview-api.spec.ts#L1-L200
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L1-L200、manager.tsx、manager.tsx#L1-L200、preview.tsx、preview.tsx#L1-L200、storybook.setup.ts、package.json

### 故障排除/兼容性问题.md

- reference 标题：兼容性问题
- 生成页：配置参考/主题和外观.md（主题和外观）
- KnowledgeUnit：unit_id=unit-bebeb3781961, unit_type=config_doc, domain_id=domain-e15151050742, readiness=compose_ready, child_digests=3
- reuse_count：6
- skeleton_score：0.13
- key_source_coverage：0.09
- missing_key_sources：code/.swcrc、code/.swcrc#L1-L24、code/addons/docs/docs/frameworks/ANGULAR.md、code/addons/docs/docs/frameworks/COMMON.md、code/addons/docs/docs/frameworks/EMBER.md、code/addons/docs/docs/frameworks/FRAMEWORKS-OVERVIEW.md、code/addons/docs/docs/frameworks/NEXTJS.md、code/addons/docs/docs/frameworks/PREACT.md、code/addons/docs/docs/frameworks/REACT-VUE-ANGULAR.md、code/addons/docs/docs/frameworks/RENDERERS.md、code/addons/docs/docs/frameworks/SVELTE.md、code/addons/docs/docs/frameworks/WEB-COMPONENTS.md、code/core/src/shared/constants/environments-support.ts、code/core/src/shared/constants/environments-support.ts#L1-L21、code/lib/cli-storybook/src/autoblock/block-node-version.ts、code/lib/cli-storybook/src/autoblock/block-node-version.ts#L1-L21、code/lib/cli-storybook/src/bin/index.ts、code/lib/cli-storybook/src/bin/index.ts#L1-L20、code/lib/cli-storybook/src/doctor/hasMultipleVersions.ts、code/lib/cli-storybook/src/doctor/hasMultipleVersions.ts#L1-L25、code/lib/create-storybook/src/bin/index.ts、code/lib/create-storybook/src/bin/index.ts#L1-L20、code/package.json#L1-L219、code/package.json#L71-L150、code/tsconfig.json#L1-L26、package.json#L1-L74、package.json#L5-L16、test-storybooks/ember-cli/config/targets.js、test-storybooks/ember-cli/config/targets.js#L1-L4
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：.swcrc、.swcrc#L1-L24、ANGULAR.md、COMMON.md、EMBER.md、FRAMEWORKS-OVERVIEW.md、NEXTJS.md、PREACT.md

### 故障排除/安装问题.md

- reference 标题：安装问题
- 生成页：开发工具/开发工具.md（开发工具）
- KnowledgeUnit：unit_id=unit-df10871c9c97, unit_type=domain_index, domain_id=domain-1ad09e98abab, readiness=compose_ready, child_digests=5
- reuse_count：2
- skeleton_score：0.05
- key_source_coverage：0.02
- missing_key_sources：.yarnrc.yml、.yarnrc.yml#L1-L39、.yarnrc.yml#L15-L39、.yarnrc.yml#L21-L39、CONTRIBUTING.md、CONTRIBUTING.md#L25-L48、CONTRIBUTING.md#L47-L52、README.md、code/core/package.json、code/core/package.json#L1-L389、code/core/package.json#L222-L229、code/core/src/common/js-package-manager/NPMProxy.test.ts、code/core/src/common/js-package-manager/NPMProxy.test.ts#L411-L434、code/core/src/common/js-package-manager/NPMProxy.ts、code/core/src/common/js-package-manager/NPMProxy.ts#L128-L161、code/core/src/common/js-package-manager/Yarn2Proxy.test.ts、code/core/src/common/js-package-manager/Yarn2Proxy.test.ts#L270-L293、code/lib/cli-storybook/src/doctor/hasMultipleVersions.ts、code/lib/cli-storybook/src/doctor/hasMultipleVersions.ts#L1-L25、code/lib/cli-storybook/src/sandbox.ts、code/lib/cli-storybook/src/sandbox.ts#L241-L278、code/lib/create-storybook/package.json、code/lib/create-storybook/package.json#L1-L61、code/lib/create-storybook/package.json#L31-L32、code/lib/create-storybook/src/commands/DependencyInstallationCommand.ts、code/lib/create-storybook/src/commands/DependencyInstallationCommand.ts#L78-L101、code/lib/create-storybook/src/commands/FinalizationCommand.ts#L65-L81、code/lib/create-storybook/src/commands/PreflightCheckCommand.ts、code/lib/create-storybook/src/commands/PreflightCheckCommand.ts#L37-L69、code/lib/create-storybook/src/dependency-collector.ts、code/lib/create-storybook/src/dependency-collector.ts#L1-L48、code/lib/create-storybook/src/dependency-collector.ts#L19-L48、code/lib/create-storybook/src/initiate.ts、code/lib/create-storybook/src/initiate.ts#L1-L227、code/lib/create-storybook/src/initiate.ts#L126-L132、code/lib/create-storybook/src/initiate.ts#L32-L124、code/lib/create-storybook/src/initiate.ts#L42-L124、code/package.json、code/package.json#L1-L219、code/package.json#L60-L70、code/package.json#L74-L150、nx.json、nx.json#L1-L220、nx.json#L27-L56、nx.json#L5-L5、package.json、package.json#L1-L74、package.json#L72-L73、scripts/package.json、scripts/package.json#L1-L189、scripts/sandbox/generate.ts、scripts/sandbox/generate.ts#L266-L290、scripts/sandbox/generate.ts#L274-L290
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：.yarnrc.yml、.yarnrc.yml#L1-L39、.yarnrc.yml#L15-L39、.yarnrc.yml#L21-L39、CONTRIBUTING.md、CONTRIBUTING.md#L25-L48、CONTRIBUTING.md#L47-L52、README.md

### 故障排除/故障排除.md

- reference 标题：故障排除
- 生成页：配置参考/主题和外观.md（主题和外观）
- KnowledgeUnit：unit_id=unit-bebeb3781961, unit_type=config_doc, domain_id=domain-e15151050742, readiness=compose_ready, child_digests=3
- reuse_count：6
- skeleton_score：0.10
- key_source_coverage：0.10
- missing_key_sources：README.md、README.md#L91-L94、code/.storybook/main.ts、code/.storybook/main.ts#L117-L149、code/.storybook/main.ts#L169-L173、code/.storybook/main.ts#L174-L178、code/.storybook/main.ts#L19-L183、code/core/src/shared/constants/environments-support.ts、code/core/src/shared/constants/environments-support.ts#L4-L21、code/package.json#L1-L219、code/package.json#L15-L45、code/package.json#L70-L151、code/playwright.config.ts、code/playwright.config.ts#L1-L115、code/playwright.config.ts#L11-L114、code/playwright.config.ts#L49-L51、code/playwright.config.ts#L54-L104、code/vitest.config.ts、code/vitest.config.ts#L1-L57、code/vitest.config.ts#L12-L13、code/vitest.config.ts#L12-L56、code/vitest.config.ts#L33-L56、package.json#L1-L74、package.json#L18-L38、package.json#L5-L17、package.json#L72-L73、scripts/package.json#L1-L189、scripts/package.json#L6-L44、scripts/task.ts、scripts/task.ts#L347-L367、scripts/task.ts#L372-L587、scripts/task.ts#L81-L106、scripts/utils/options.ts、scripts/utils/options.ts#L118-L171、scripts/utils/options.ts#L318-L341
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：README.md、README.md#L91-L94、main.ts、main.ts#L117-L149、main.ts#L169-L173、main.ts#L174-L178、main.ts#L19-L183、environments-support.ts

### 故障排除/构建和性能问题.md

- reference 标题：构建和性能问题
- 生成页：构建系统/builder-webpack5.md（builder-webpack5）
- KnowledgeUnit：unit_id=unit-e23b550d9f21, unit_type=module_doc, domain_id=domain-f0fd06748cfc, readiness=compose_ready, child_digests=0
- reuse_count：3
- skeleton_score：0.12
- key_source_coverage：0.06
- missing_key_sources：code/builders/builder-vite/package.json、code/builders/builder-vite/package.json#L1-L73、code/builders/builder-vite/package.json#L49-L72、code/builders/builder-vite/src/index.ts#L1-L68、code/builders/builder-vite/src/index.ts#L19-L33、code/builders/builder-vite/src/index.ts#L41-L61、code/builders/builder-vite/src/index.ts#L49-L50、code/builders/builder-vite/src/index.ts#L54-L58、code/builders/builder-vite/src/vite-server.ts、code/builders/builder-vite/src/vite-server.ts#L1-L47、code/builders/builder-vite/src/vite-server.ts#L16-L46、code/builders/builder-vite/src/vite-server.ts#L18-L30、code/builders/builder-vite/src/vite-server.ts#L21-L38、code/builders/builder-vite/src/vite-server.ts#L40-L46、code/builders/builder-vite/src/vite-server.ts#L9-L46、code/builders/builder-webpack5/package.json、code/builders/builder-webpack5/package.json#L1-L93、code/builders/builder-webpack5/package.json#L54-L92、code/builders/builder-webpack5/src/index.ts#L1-L349、code/builders/builder-webpack5/src/index.ts#L121-L227、code/builders/builder-webpack5/src/index.ts#L132-L134、code/builders/builder-webpack5/src/index.ts#L151-L179、code/builders/builder-webpack5/src/index.ts#L187-L200、code/builders/builder-webpack5/src/index.ts#L229-L243、code/builders/builder-webpack5/src/index.ts#L251-L319、code/builders/builder-webpack5/src/index.ts#L303-L319、code/builders/builder-webpack5/src/preview/base-webpack.config.ts#L1-L113、code/builders/builder-webpack5/src/preview/base-webpack.config.ts#L64-L94、code/builders/builder-webpack5/src/preview/base-webpack.config.ts#L67-L84、code/builders/builder-webpack5/src/preview/base-webpack.config.ts#L75-L84、code/core/src/common/utils/file-cache.ts、code/core/src/common/utils/file-cache.ts#L1-L162、code/core/src/common/utils/file-cache.ts#L120-L133、code/renderers/react/src/componentManifest/utils.ts、code/renderers/react/src/componentManifest/utils.ts#L37-L100、package.json、package.json#L1-L74、scripts/bench/bench-packages.ts、scripts/bench/bench-packages.ts#L1-L52、scripts/bench/bench-packages.ts#L46-L52、scripts/bench/safe-args.ts、scripts/bench/safe-args.ts#L1-L8、scripts/build/utils/generate-bundle.ts、scripts/build/utils/generate-bundle.ts#L1-L270、scripts/build/utils/generate-bundle.ts#L80-L121、scripts/build/utils/generate-bundle.ts#L86-L121、scripts/build/utils/generate-bundle.ts#L86-L94
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L73、package.json#L49-L72、index.ts#L1-L68、index.ts#L19-L33、index.ts#L41-L61、index.ts#L49-L50、index.ts#L54-L58

### 故障排除/调试工具和技巧.md

- reference 标题：调试工具和技巧
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- KnowledgeUnit：unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：14
- skeleton_score：0.14
- key_source_coverage：0.00
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L107-L183、code/.storybook/main.ts#L149-L183、code/.storybook/main.ts#L169-L180、code/.storybook/main.ts#L19-L183、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L402-L415、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L11-L29、code/.storybook/storybook.setup.ts#L9-L29、code/.storybook/storybook.setup.ts#L9-L9、code/addons/docs/src/blocks/blocks/Canvas.stories.tsx、code/addons/docs/src/blocks/blocks/Canvas.stories.tsx#L13-L36、code/core/src/instrumenter/instrumenter.ts、code/core/src/instrumenter/instrumenter.ts#L259-L286、code/core/src/instrumenter/instrumenter.ts#L440-L490、code/core/src/instrumenter/instrumenter.ts#L676-L740、code/core/src/instrumenter/instrumenter.ts#L749-L780、code/core/src/instrumenter/instrumenter.ts#L94-L286、code/core/src/node-logger/logger/console.ts、code/core/src/node-logger/logger/console.ts#L26-L304、code/core/src/node-logger/logger/console.ts#L486-L551、code/e2e-tests/util.ts、code/e2e-tests/util.ts#L117-L148、code/e2e-tests/util.ts#L32-L74、code/e2e-tests/util.ts#L8-L74、code/e2e-tests/util.ts#L96-L115、code/lib/eslint-plugin/docs/rules/context-in-play-function.md、code/lib/eslint-plugin/docs/rules/context-in-play-function.md#L1-L63、code/lib/eslint-plugin/src/rules/context-in-play-function.ts、code/lib/eslint-plugin/src/rules/context-in-play-function.ts#L48-L99
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L107-L183、main.ts#L149-L183、main.ts#L169-L180、main.ts#L19-L183、manager.tsx、manager.tsx#L4-L8、preview.tsx

### 故障排除/运行时错误.md

- reference 标题：运行时错误
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除/配置错误.md

- reference 标题：配置错误
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- KnowledgeUnit：unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：14
- skeleton_score：0.09
- key_source_coverage：0.00
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L169-L173、code/.storybook/main.ts#L19-L183、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L415、code/.storybook/preview.tsx#L195-L415、code/.storybook/preview.tsx#L402-L415、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L1-L30、code/.storybook/storybook.setup.ts#L11-L29、code/.storybook/storybook.setup.ts#L16-L27、code/core/src/common/utils/normalize-stories.ts、code/core/src/common/utils/normalize-stories.ts#L1-L115、code/core/src/common/utils/normalize-stories.ts#L25-L90、code/core/src/common/utils/normalize-stories.ts#L38-L100、code/core/src/common/utils/paths.ts、code/core/src/common/utils/paths.ts#L1-L89、code/core/src/common/utils/paths.ts#L11-L57、code/core/src/core-server/utils/server-statics.ts、code/core/src/core-server/utils/server-statics.ts#L195-L208、code/core/src/core-server/utils/server-statics.ts#L198-L208、code/core/src/csf-tools/ConfigFile.ts、code/core/src/csf-tools/ConfigFile.ts#L453-L493、code/lib/cli-storybook/src/automigrate/helpers/mainConfigFile.ts、code/lib/cli-storybook/src/automigrate/helpers/mainConfigFile.ts#L172-L211、code/package.json、code/package.json#L71-L151
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L1-L186、main.ts#L149-L181、main.ts#L169-L173、main.ts#L19-L183、manager.tsx、manager.tsx#L1-L9、manager.tsx#L4-L8

### 构建系统/Vite构建器详解.md

- reference 标题：Vite构建器详解
- 生成页：构建系统/builder-vite.md（builder-vite）
- KnowledgeUnit：unit_id=unit-7786830d55a0, unit_type=module_doc, domain_id=domain-f0fd06748cfc, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.20
- key_source_coverage：0.14
- missing_key_sources：code/builders/builder-vite/input/iframe.html、code/builders/builder-vite/package.json、code/builders/builder-vite/package.json#L1-L73、code/builders/builder-vite/package.json#L49-L67、code/builders/builder-vite/src/build.ts、code/builders/builder-vite/src/build.ts#L1-L11、code/builders/builder-vite/src/build.ts#L1-L99、code/builders/builder-vite/src/build.ts#L17-L98、code/builders/builder-vite/src/build.ts#L21-L40、code/builders/builder-vite/src/build.ts#L46-L62、code/builders/builder-vite/src/build.ts#L77-L87、code/builders/builder-vite/src/index.ts#L1-L17、code/builders/builder-vite/src/index.ts#L1-L68、code/builders/builder-vite/src/index.ts#L19-L33、code/builders/builder-vite/src/index.ts#L41-L61、code/builders/builder-vite/src/index.ts#L41-L65、code/builders/builder-vite/src/index.ts#L54-L58、code/builders/builder-vite/src/plugins/code-generator-plugin.ts#L1-L97、code/builders/builder-vite/src/plugins/code-generator-plugin.ts#L29-L38、code/builders/builder-vite/src/plugins/code-generator-plugin.ts#L30-L37、code/builders/builder-vite/src/plugins/code-generator-plugin.ts#L44-L52、code/builders/builder-vite/src/plugins/code-generator-plugin.ts#L89-L94、code/builders/builder-vite/src/plugins/csf-plugin.ts#L1-L23、code/builders/builder-vite/src/plugins/csf-plugin.ts#L7-L22、code/builders/builder-vite/src/plugins/index.ts#L1-L13
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：iframe.html、package.json、package.json#L1-L73、package.json#L49-L67、build.ts、build.ts#L1-L11、build.ts#L1-L99、build.ts#L17-L98

### 构建系统/Webpack构建器详解.md

- reference 标题：Webpack构建器详解
- 生成页：构建系统/构建系统.md（构建系统）
- KnowledgeUnit：unit_id=unit-6733528f117a, unit_type=domain_index, domain_id=domain-f0fd06748cfc, readiness=compose_ready, child_digests=6
- reuse_count：3
- skeleton_score：0.08
- key_source_coverage：0.07
- missing_key_sources：code/builders/builder-webpack5/package.json、code/builders/builder-webpack5/package.json#L1-L93、code/builders/builder-webpack5/package.json#L54-L69、code/builders/builder-webpack5/src/index.ts#L121-L227、code/builders/builder-webpack5/src/index.ts#L132-L144、code/builders/builder-webpack5/src/index.ts#L151-L179、code/builders/builder-webpack5/src/index.ts#L208-L221、code/builders/builder-webpack5/src/index.ts#L229-L243、code/builders/builder-webpack5/src/index.ts#L277-L291、code/builders/builder-webpack5/src/index.ts#L57-L113、code/builders/builder-webpack5/src/index.ts#L57-L82、code/builders/builder-webpack5/src/presets/custom-webpack-preset.ts、code/builders/builder-webpack5/src/presets/custom-webpack-preset.ts#L34-L89、code/builders/builder-webpack5/src/presets/custom-webpack-preset.ts#L66-L89、code/builders/builder-webpack5/src/presets/custom-webpack-preset.ts#L7-L14、code/builders/builder-webpack5/src/presets/custom-webpack-preset.ts#L82-L88、code/builders/builder-webpack5/src/presets/preview-preset.ts#L1-L29、code/builders/builder-webpack5/src/presets/preview-preset.ts#L5-L29、code/builders/builder-webpack5/src/preview/iframe-webpack.config.ts、code/builders/builder-webpack5/src/preview/iframe-webpack.config.ts#L113-L273、code/builders/builder-webpack5/src/preview/iframe-webpack.config.ts#L140-L191、code/builders/builder-webpack5/src/preview/iframe-webpack.config.ts#L181-L200、code/builders/builder-webpack5/src/preview/iframe-webpack.config.ts#L192-L223、code/builders/builder-webpack5/src/preview/iframe-webpack.config.ts#L196-L222、code/builders/builder-webpack5/src/preview/iframe-webpack.config.ts#L232-L273、code/builders/builder-webpack5/src/preview/iframe-webpack.config.ts#L31-L118、code/builders/builder-webpack5/src/types.ts、code/builders/builder-webpack5/src/types.ts#L11-L53、code/frameworks/nextjs/src/css/webpack.ts、code/frameworks/nextjs/src/css/webpack.ts#L16-L56、code/frameworks/nextjs/src/font/webloader/webpack-loader.ts、code/frameworks/nextjs/src/font/webloader/webpack-loader.ts#L18-L44、code/lib/core-webpack/src/index.ts#L1-L7、code/lib/core-webpack/src/load-custom-webpack-config.ts、code/lib/core-webpack/src/load-custom-webpack-config.ts#L1-L9、code/lib/core-webpack/src/load-custom-webpack-config.ts#L7-L9、code/lib/core-webpack/src/merge-webpack-config.ts、code/lib/core-webpack/src/merge-webpack-config.ts#L69-L81、docs/_snippets/my-component-story-import-static-asset.md#L1-L6、package.json、package.json#L1-L74
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L93、package.json#L54-L69、index.ts#L121-L227、index.ts#L132-L144、index.ts#L151-L179、index.ts#L208-L221、index.ts#L229-L243

### 构建系统/构建器概览.md

- reference 标题：构建器概览
- 生成页：构建系统/builder-webpack5.md（builder-webpack5）
- KnowledgeUnit：unit_id=unit-e23b550d9f21, unit_type=module_doc, domain_id=domain-f0fd06748cfc, readiness=compose_ready, child_digests=0
- reuse_count：3
- skeleton_score：0.13
- key_source_coverage：0.13
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/builders/builder-vite/package.json、code/builders/builder-vite/package.json#L1-L73、code/builders/builder-vite/src/build.ts、code/builders/builder-vite/src/build.ts#L1-L99、code/builders/builder-vite/src/index.ts#L1-L68、code/builders/builder-vite/src/types.ts、code/builders/builder-vite/src/types.ts#L1-L25、code/builders/builder-vite/src/vite-config.ts、code/builders/builder-vite/src/vite-config.ts#L1-L94、code/builders/builder-vite/src/vite-server.ts、code/builders/builder-vite/src/vite-server.ts#L1-L47、code/builders/builder-webpack5/package.json、code/builders/builder-webpack5/package.json#L1-L93、code/builders/builder-webpack5/src/index.ts#L1-L349、code/builders/builder-webpack5/src/preview/virtual-module-mapping.ts#L1-L85、code/builders/builder-webpack5/src/types.ts、code/builders/builder-webpack5/src/types.ts#L1-L53、code/core/src/types/modules/builders.ts、code/core/src/types/modules/builders.ts#L1-L7
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L1-L186、package.json、package.json#L1-L73、build.ts、build.ts#L1-L99、index.ts#L1-L68、types.ts

### 构建系统/构建系统.md

- reference 标题：构建系统
- 生成页：构建系统/构建系统.md（构建系统）
- KnowledgeUnit：unit_id=unit-6733528f117a, unit_type=domain_index, domain_id=domain-f0fd06748cfc, readiness=compose_ready, child_digests=6
- reuse_count：3
- skeleton_score：0.00
- key_source_coverage：0.06
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L149-L181、code/builders/builder-vite/package.json、code/builders/builder-vite/package.json#L1-L73、code/builders/builder-vite/package.json#L49-L67、code/builders/builder-vite/src/build.ts、code/builders/builder-vite/src/build.ts#L1-L99、code/builders/builder-vite/src/build.ts#L17-L98、code/builders/builder-vite/src/build.ts#L42-L62、code/builders/builder-vite/src/build.ts#L64-L75、code/builders/builder-vite/src/index.ts#L1-L68、code/builders/builder-vite/src/index.ts#L41-L67、code/builders/builder-vite/src/index.ts#L54-L58、code/builders/builder-vite/src/types.ts、code/builders/builder-vite/src/types.ts#L1-L25、code/builders/builder-vite/src/vite-config.ts、code/builders/builder-vite/src/vite-config.ts#L1-L94、code/builders/builder-vite/src/vite-config.ts#L37-L78、code/builders/builder-vite/src/vite-server.ts、code/builders/builder-vite/src/vite-server.ts#L1-L47、code/builders/builder-vite/src/vite-server.ts#L18-L28、code/builders/builder-vite/src/vite-server.ts#L9-L46、code/builders/builder-webpack5/package.json、code/builders/builder-webpack5/package.json#L1-L93、code/builders/builder-webpack5/package.json#L54-L82、code/builders/builder-webpack5/src/index.ts#L1-L349、code/builders/builder-webpack5/src/index.ts#L121-L227、code/builders/builder-webpack5/src/index.ts#L132-L134、code/builders/builder-webpack5/src/index.ts#L181-L227、code/builders/builder-webpack5/src/index.ts#L208-L221、code/builders/builder-webpack5/src/index.ts#L303-L318、code/builders/builder-webpack5/src/index.ts#L321-L341、code/builders/builder-webpack5/src/presets/custom-webpack-preset.ts、code/builders/builder-webpack5/src/presets/custom-webpack-preset.ts#L1-L93、code/builders/builder-webpack5/src/presets/custom-webpack-preset.ts#L34-L64、code/builders/builder-webpack5/src/presets/custom-webpack-preset.ts#L66-L89、code/builders/builder-webpack5/src/presets/preview-preset.ts#L1-L29、code/builders/builder-webpack5/src/preview/virtual-module-mapping.ts、code/builders/builder-webpack5/src/preview/virtual-module-mapping.ts#L1-L85、code/builders/builder-webpack5/src/preview/virtual-module-mapping.ts#L51-L52、code/builders/builder-webpack5/src/types.ts、code/builders/builder-webpack5/src/types.ts#L1-L53、code/builders/builder-webpack5/src/types.ts#L45-L48
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L1-L186、main.ts#L149-L181、package.json、package.json#L1-L73、package.json#L49-L67、build.ts、build.ts#L1-L99

### 构建系统/构建配置优化.md

- reference 标题：构建配置优化
- 生成页：多框架支持/多框架支持.md（多框架支持）
- KnowledgeUnit：unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- reuse_count：14
- skeleton_score：0.07
- key_source_coverage：0.03
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L19-L183、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L416、code/.storybook/preview.tsx#L402-L416、code/builders/builder-vite/src/build.ts、code/builders/builder-vite/src/build.ts#L17-L40、code/builders/builder-vite/src/build.ts#L22-L38、code/chromatic.config.json、code/chromatic.config.json#L1-L10、code/core/build-config.ts、code/core/src/bin/core.ts、code/core/src/bin/core.ts#L172-L213、code/core/src/core-server/standalone.ts、code/core/src/core-server/standalone.ts#L5-L35、code/core/tsconfig.json、code/core/tsconfig.json#L1-L11、code/package.json、code/package.json#L15-L44、code/scripts/build-package.ts、code/scripts/build-package.ts#L27-L183、code/tsconfig.json、code/tsconfig.json#L1-L26、docs/_snippets/storybook-builder-api-build-server.md、docs/_snippets/storybook-builder-api-build-server.md#L8-L33
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L149-L181、main.ts#L19-L183、manager.tsx、manager.tsx#L1-L9、manager.tsx#L4-L8、preview.tsx、preview.tsx#L1-L416

### 核心概念/Addons系统架构/Addons开发指南.md

- reference 标题：Addons开发指南
- 生成页：系统架构.md（系统架构）
- KnowledgeUnit：unit_id=unit-3ce08b39bd6b, unit_type=architecture, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：11
- skeleton_score：0.17
- key_source_coverage：0.02
- missing_key_sources：code/.storybook/manager.tsx、code/.storybook/preview.tsx、code/.storybook/storybook.setup.ts、code/addons/a11y/package.json、code/addons/a11y/package.json#L1-L90、code/addons/a11y/package.json#L37-L52、code/addons/a11y/package.json#L60-L77、code/addons/a11y/package.json#L75-L77、code/addons/a11y/package.json#L78-L88、code/addons/a11y/src/constants.ts、code/addons/a11y/src/constants.ts#L1-L22、code/addons/a11y/src/index.ts、code/addons/a11y/src/index.ts#L1-L11、code/addons/a11y/src/manager.tsx、code/addons/a11y/src/manager.tsx#L14-L48、code/addons/a11y/src/manager.tsx#L50-L67、code/addons/a11y/src/preview.tsx、code/addons/a11y/src/preview.tsx#L10-L12、code/addons/a11y/src/preview.tsx#L14-L110、code/addons/a11y/src/preview.tsx#L14-L96、code/addons/a11y/src/preview.tsx#L81-L95、code/addons/a11y/src/types.ts、code/addons/a11y/src/types.ts#L55-L68、code/addons/a11y/src/types.ts#L7-L31、code/addons/docs/package.json、code/addons/docs/package.json#L1-L133、code/addons/docs/package.json#L118-L120、code/addons/docs/package.json#L121-L132、code/addons/docs/package.json#L40-L80、code/addons/docs/package.json#L88-L120、code/addons/docs/src/index.ts、code/addons/docs/src/index.ts#L1-L21、code/addons/docs/src/preview.ts、code/addons/docs/src/preview.ts#L1-L30、code/addons/docs/src/preview.ts#L14-L30、code/addons/docs/src/types.ts、code/addons/docs/src/types.ts#L151-L256、code/addons/links/package.json、code/addons/links/package.json#L55-L64、code/addons/links/package.json#L61-L64、code/addons/links/package.json#L70-L81、code/addons/links/src/index.ts、code/addons/links/src/index.ts#L1-L8、code/addons/links/src/index.ts#L5-L7、code/addons/themes/package.json、code/addons/themes/package.json#L1-L82、code/addons/themes/package.json#L58-L69、code/addons/themes/package.json#L67-L69、code/addons/themes/package.json#L70-L81、code/addons/themes/package.json#L74-L80、code/addons/vitest/package.json、code/addons/vitest/package.json#L1-L139、code/addons/vitest/package.json#L107-L127、code/addons/vitest/package.json#L128-L138、code/addons/vitest/package.json#L40-L66、code/addons/vitest/package.json#L78-L127、code/e2e-tests/manager.spec.ts、code/e2e-tests/preview-api.spec.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：manager.tsx、preview.tsx、storybook.setup.ts、package.json、package.json#L1-L90、package.json#L37-L52、package.json#L60-L77、package.json#L75-L77

### 核心概念/Addons系统架构/Addons架构设计.md

- reference 标题：Addons架构设计
- 生成页：系统架构.md（系统架构）
- KnowledgeUnit：unit_id=unit-3ce08b39bd6b, unit_type=architecture, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：11
- skeleton_score：0.07
- key_source_coverage：0.00
- missing_key_sources：code/addons/a11y/package.json、code/addons/a11y/package.json#L37-L52、code/addons/a11y/package.json#L60-L63、code/addons/docs/package.json、code/addons/docs/package.json#L40-L80、code/addons/docs/package.json#L88-L96、code/addons/links/package.json、code/addons/links/package.json#L29-L47、code/addons/links/package.json#L55-L57、code/addons/onboarding/package.json、code/addons/onboarding/package.json#L33-L41、code/addons/onboarding/package.json#L50-L57、code/addons/pseudo-states/package.json、code/addons/pseudo-states/package.json#L35-L47、code/addons/pseudo-states/package.json#L55-L59、code/addons/themes/package.json、code/addons/themes/package.json#L36-L49、code/addons/themes/package.json#L58-L60、code/addons/vitest/package.json、code/addons/vitest/package.json#L40-L65、code/addons/vitest/package.json#L78-L81、code/core/src/manager-api/index.ts#L1-L27、code/core/src/manager-api/index.ts#L3-L27、code/core/src/manager-api/modules/channel.ts、code/core/src/manager-api/modules/channel.ts#L1-L2、code/core/src/manager-api/modules/channel.ts#L1-L78、code/core/src/manager-api/modules/channel.ts#L52-L77、code/core/src/manager-api/modules/channel.ts#L62-L74、code/core/src/manager-api/modules/channel.ts#L7-L48、code/core/src/preview-api/index.ts、code/core/src/preview-api/index.ts#L1-L90、code/core/src/preview-api/index.ts#L4-L18、code/core/src/preview-api/index.ts#L46-L68
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L37-L52、package.json#L60-L63、package.json、package.json#L40-L80、package.json#L88-L96、package.json、package.json#L29-L47

### 核心概念/Addons系统架构/Addons系统架构.md

- reference 标题：Addons系统架构
- 生成页：系统架构.md（系统架构）
- KnowledgeUnit：unit_id=unit-3ce08b39bd6b, unit_type=architecture, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：11
- skeleton_score：0.16
- key_source_coverage：0.00
- missing_key_sources：code/addons/a11y/manager.js、code/addons/docs/manager.js、code/addons/links/manager.js、code/addons/onboarding/manager.js、code/addons/pseudo-states/manager.js、code/addons/themes/manager.js、code/addons/vitest/manager.js、code/core/src/common/utils/remove.ts、code/core/src/common/utils/remove.ts#L50-L63、code/core/src/manager-api/modules/addons.ts、code/core/src/manager-api/modules/addons.ts#L1-L134、code/core/src/manager-api/modules/addons.ts#L13-L134、code/core/src/manager-api/modules/addons.ts#L18-L134、code/core/src/manager-api/modules/addons.ts#L18-L74、code/core/src/manager-api/modules/addons.ts#L93-L134、code/core/src/preview-api/addons.ts、code/core/src/preview-api/addons.ts#L1-L4、code/core/src/types/modules/addons.ts、code/core/src/types/modules/addons.ts#L324-L475、code/core/src/types/modules/addons.ts#L493-L522、code/lib/cli-storybook/src/postinstallAddon.ts、code/lib/cli-storybook/src/postinstallAddon.ts#L1-L49、code/lib/cli-storybook/src/postinstallAddon.ts#L10-L49、code/lib/cli-storybook/src/postinstallAddon.ts#L39-L49、docs/_snippets/storybook-addon-disable-addon.md、docs/_snippets/storybook-addon-disable-addon.md#L1-L11、docs/_snippets/storybook-addons-api-selectstory.md、docs/_snippets/storybook-addons-api-selectstory.md#L1-L6、docs/_snippets/storybook-addons-api-togglepanel.md、docs/_snippets/storybook-addons-api-togglepanel.md#L1-L13、docs/_snippets/storybook-main-register-addon.md、docs/_snippets/storybook-main-register-addon.md#L1-L6、docs/addons/index.mdx、docs/addons/index.mdx#L1-L46、docs/addons/index.mdx#L14-L30、docs/addons/index.mdx#L31-L46
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：manager.js、manager.js、manager.js、manager.js、manager.js、manager.js、manager.js、remove.ts

### 核心概念/Addons系统架构/Addons通信机制.md

- reference 标题：Addons通信机制
- 生成页：系统架构.md（系统架构）
- KnowledgeUnit：unit_id=unit-3ce08b39bd6b, unit_type=architecture, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：11
- skeleton_score：0.14
- key_source_coverage：0.02
- missing_key_sources：code/core/src/channels/index.test.ts、code/core/src/channels/index.test.ts#L106-L169、code/core/src/channels/index.test.ts#L55-L169、code/core/src/channels/index.ts、code/core/src/channels/index.ts#L1-L60、code/core/src/channels/index.ts#L32-L51、code/core/src/channels/main.ts#L1-L10、code/core/src/channels/main.ts#L1-L148、code/core/src/channels/main.ts#L22-L148、code/core/src/channels/main.ts#L22-L51、code/core/src/channels/main.ts#L57-L146、code/core/src/channels/main.ts#L62-L82、code/core/src/channels/postmessage/getEventSourceUrl.ts、code/core/src/channels/postmessage/getEventSourceUrl.ts#L1-L53、code/core/src/channels/postmessage/getEventSourceUrl.ts#L3-L52、code/core/src/channels/postmessage/index.ts、code/core/src/channels/postmessage/index.ts#L139-L192、code/core/src/channels/postmessage/index.ts#L29-L240、code/core/src/channels/postmessage/index.ts#L29-L47、code/core/src/channels/postmessage/index.ts#L66-L129、code/core/src/channels/postmessage/index.ts#L7-L8、code/core/src/channels/types.ts、code/core/src/channels/types.ts#L1-L60、code/core/src/channels/types.ts#L13-L50、code/core/src/channels/websocket/index.ts、code/core/src/channels/websocket/index.ts#L23-L109、code/core/src/channels/websocket/index.ts#L23-L75、code/core/src/channels/websocket/index.ts#L36-L42、code/core/src/channels/websocket/index.ts#L44-L75、code/core/src/channels/websocket/index.ts#L6-L7、code/core/src/channels/websocket/index.ts#L81-L107、code/core/src/channels/websocket/index.ts#L96-L100、code/core/src/channels/websocket/index.ts#L96-L99、code/core/src/core-server/utils/__tests__/server-channel.test.ts、code/core/src/core-server/utils/__tests__/server-channel.test.ts#L42-L90、code/core/src/core-server/utils/get-server-channel.ts、code/core/src/core-server/utils/get-server-channel.ts#L26-L85、code/core/src/core-server/utils/get-server-channel.ts#L26-L98、code/core/src/core-server/utils/get-server-channel.ts#L34-L59、code/core/src/core-server/utils/get-server-channel.ts#L6-L7、code/core/src/core-server/utils/get-server-channel.ts#L61-L97、code/core/src/core-server/utils/get-server-channel.ts#L69-L84、code/core/src/core-server/utils/getAccessControlMiddleware.ts、code/core/src/core-server/utils/getAccessControlMiddleware.ts#L1-L14、code/core/src/core-server/utils/getAccessControlMiddleware.ts#L3-L12
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：index.test.ts、index.test.ts#L106-L169、index.test.ts#L55-L169、index.ts、index.ts#L1-L60、index.ts#L32-L51、main.ts#L1-L10、main.ts#L1-L148

### 核心概念/Addons系统架构/Addons配置管理.md

- reference 标题：Addons配置管理
- 生成页：系统架构.md（系统架构）
- KnowledgeUnit：unit_id=unit-3ce08b39bd6b, unit_type=architecture, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：11
- skeleton_score：0.24
- key_source_coverage：0.03
- missing_key_sources：code/.storybook/main.ts#L107-L116、code/.storybook/main.ts#L117-L185、code/.storybook/main.ts#L169-L173、code/.storybook/main.ts#L171-L172、code/.storybook/main.ts#L172-L172、code/.storybook/main.ts#L174-L179、code/.storybook/main.ts#L19-L116、code/.storybook/main.ts#L19-L185、code/addons/a11y/package.json、code/addons/a11y/package.json#L37-L51、code/addons/a11y/src/preset.ts、code/addons/a11y/src/preset.ts#L1-L4、code/addons/docs/package.json、code/addons/docs/package.json#L37-L51、code/addons/docs/src/preset.ts、code/addons/docs/src/preset.ts#L138-L153、code/addons/docs/src/preset.ts#L155-L157、code/addons/docs/src/preset.ts#L159-L193、code/addons/docs/src/preset.ts#L214-L221、code/addons/themes/package.json、code/addons/themes/package.json#L36-L49、code/core/package.json、code/core/src/common/presets.ts、code/core/src/common/presets.ts#L126-L144、code/core/src/common/presets.ts#L156-L243、code/core/src/common/presets.ts#L187-L197、code/core/src/common/presets.ts#L226-L243、code/core/src/common/presets.ts#L265-L328、code/core/src/common/presets.ts#L285-L304、code/core/src/common/presets.ts#L318-L357、code/core/src/common/presets.ts#L330-L357、code/core/src/common/presets.ts#L68-L116、code/core/src/common/presets.ts#L68-L144
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L107-L116、main.ts#L117-L185、main.ts#L169-L173、main.ts#L171-L172、main.ts#L172-L172、main.ts#L174-L179、main.ts#L19-L116、main.ts#L19-L185

### 核心概念/Addons系统架构/核心Addons详解/A11y Addon.md

- reference 标题：A11y Addon
- 生成页：插件生态/a11y.md（a11y）
- KnowledgeUnit：unit_id=unit-f17d66e30682, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.13
- key_source_coverage：0.09
- missing_key_sources：code/addons/a11y/package.json、code/addons/a11y/package.json#L1-L90、code/addons/a11y/package.json#L60-L77、code/addons/a11y/src/AccessibilityRuleMaps.ts、code/addons/a11y/src/AccessibilityRuleMaps.ts#L1-L596、code/addons/a11y/src/a11yRunner.ts、code/addons/a11y/src/a11yRunner.ts#L1-L136、code/addons/a11y/src/a11yRunner.ts#L18-L22、code/addons/a11y/src/a11yRunner.ts#L24-L41、code/addons/a11y/src/a11yRunner.ts#L43-L121、code/addons/a11y/src/a11yRunner.ts#L57-L84、code/addons/a11y/src/a11yRunner.ts#L88-L91、code/addons/a11y/src/a11yRunner.ts#L96-L119、code/addons/a11y/src/components/A11YPanel.stories.tsx、code/addons/a11y/src/components/Report/Report.stories.tsx、code/addons/a11y/src/constants.ts、code/addons/a11y/src/constants.ts#L1-L22、code/addons/a11y/src/constants.ts#L18-L18、code/addons/a11y/src/index.ts、code/addons/a11y/src/index.ts#L1-L11、code/addons/a11y/src/manager.tsx、code/addons/a11y/src/manager.tsx#L1-L67、code/addons/a11y/src/manager.tsx#L14-L66、code/addons/a11y/src/manager.tsx#L50-L56、code/addons/a11y/src/manager.tsx#L50-L66、code/addons/a11y/src/params.ts、code/addons/a11y/src/params.ts#L1-L45、code/addons/a11y/src/params.ts#L20-L44、code/addons/a11y/src/preview.tsx、code/addons/a11y/src/preview.tsx#L1-L110、code/addons/a11y/src/preview.tsx#L14-L110、code/addons/a11y/src/preview.tsx#L14-L96、code/addons/a11y/src/preview.tsx#L25-L29、code/addons/a11y/src/preview.tsx#L25-L96、code/addons/a11y/src/preview.tsx#L63-L74、code/addons/a11y/src/preview.tsx#L81-L94、code/addons/a11y/src/types.ts、code/addons/a11y/src/types.ts#L1-L68、code/addons/a11y/src/types.ts#L16-L31、code/addons/a11y/src/utils.ts、code/addons/a11y/src/utils.ts#L1-L18、code/addons/a11y/src/withVisionSimulator.ts、code/addons/a11y/src/withVisionSimulator.ts#L1-L53、code/addons/a11y/src/withVisionSimulator.ts#L10-L52、code/addons/a11y/src/withVisionSimulator.ts#L13-L27、code/lib/cli-storybook/src/automigrate/fixes/addon-a11y-addon-test.test.ts、code/lib/cli-storybook/src/automigrate/fixes/addon-a11y-addon-test.test.ts#L52-L86、docs/_snippets/test-runner-axe-playwright.md、docs/_snippets/test-runner-axe-playwright.md#L1-L12
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L90、package.json#L60-L77、AccessibilityRuleMaps.ts、AccessibilityRuleMaps.ts#L1-L596、a11yRunner.ts、a11yRunner.ts#L1-L136、a11yRunner.ts#L18-L22

### 核心概念/Addons系统架构/核心Addons详解/Actions Addon.md

- reference 标题：Actions Addon
- 生成页：系统架构.md（系统架构）
- KnowledgeUnit：unit_id=unit-3ce08b39bd6b, unit_type=architecture, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：11
- skeleton_score：0.20
- key_source_coverage：0.00
- missing_key_sources：CHANGELOG.md、CHANGELOG.md#L1936-L1936、CHANGELOG.md#L3339-L3339、CHANGELOG.md#L4020-L4021、CHANGELOG.md#L5065-L5065、CHANGELOG.md#L5361-L5361、CHANGELOG.md#L945-L945、MIGRATION.md、MIGRATION.md#L907-L984、code/core/src/actions/constants.ts、code/core/src/actions/constants.ts#L1-L7、code/core/src/actions/constants.ts#L4-L6、code/core/src/actions/containers/ActionLogger/index.tsx、code/core/src/actions/containers/ActionLogger/index.tsx#L32-L77、code/core/src/actions/containers/ActionLogger/index.tsx#L37-L49、code/core/src/actions/containers/ActionLogger/index.tsx#L51-L55、code/core/src/actions/containers/ActionLogger/index.tsx#L57-L65、code/core/src/actions/index.ts、code/core/src/actions/index.ts#L1-L4、code/core/src/actions/loaders.ts、code/core/src/actions/loaders.ts#L1-L54、code/core/src/actions/loaders.ts#L12-L31、code/core/src/actions/loaders.ts#L47-L48、code/core/src/actions/loaders.ts#L9-L52、code/core/src/actions/manager.tsx、code/core/src/actions/manager.tsx#L1-L18、code/core/src/actions/manager.tsx#L9-L17、code/core/src/instrumenter/instrumenter.test.ts、code/core/src/instrumenter/instrumenter.test.ts#L580-L620、code/core/src/instrumenter/instrumenter.ts、code/core/src/instrumenter/instrumenter.ts#L354-L386、code/core/src/instrumenter/instrumenter.ts#L377-L386、code/core/src/manager-api/lib/addons.ts、code/core/src/manager-api/lib/addons.ts#L56-L66、code/core/src/manager-api/lib/addons.ts#L87-L98、code/e2e-tests/addon-actions.spec.ts、code/e2e-tests/addon-actions.spec.ts#L31-L63、code/e2e-tests/addon-actions.spec.ts#L37-L62、code/e2e-tests/module-mocking.spec.ts、code/e2e-tests/module-mocking.spec.ts#L44-L68、code/e2e-tests/module-mocking.spec.ts#L57-L67
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CHANGELOG.md、CHANGELOG.md#L1936-L1936、CHANGELOG.md#L3339-L3339、CHANGELOG.md#L4020-L4021、CHANGELOG.md#L5065-L5065、CHANGELOG.md#L5361-L5361、CHANGELOG.md#L945-L945、MIGRATION.md

### 核心概念/Addons系统架构/核心Addons详解/Backgrounds Addon.md

- reference 标题：Backgrounds Addon
- 生成页：系统架构.md（系统架构）
- KnowledgeUnit：unit_id=unit-3ce08b39bd6b, unit_type=architecture, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：11
- skeleton_score：0.14
- key_source_coverage：0.00
- missing_key_sources：code/core/src/backgrounds/components/Tool.tsx、code/core/src/backgrounds/components/Tool.tsx#L1-L112、code/core/src/backgrounds/components/Tool.tsx#L13-L112、code/core/src/backgrounds/components/Tool.tsx#L17-L28、code/core/src/backgrounds/components/Tool.tsx#L22-L28、code/core/src/backgrounds/components/Tool.tsx#L74-L78、code/core/src/backgrounds/constants.ts、code/core/src/backgrounds/constants.ts#L1-L8、code/core/src/backgrounds/decorator.ts、code/core/src/backgrounds/decorator.ts#L1-L102、code/core/src/backgrounds/decorator.ts#L19-L48、code/core/src/backgrounds/decorator.ts#L21-L101、code/core/src/backgrounds/decorator.ts#L21-L48、code/core/src/backgrounds/decorator.ts#L46-L48、code/core/src/backgrounds/decorator.ts#L55-L59、code/core/src/backgrounds/decorator.ts#L76-L95、code/core/src/backgrounds/defaults.ts、code/core/src/backgrounds/defaults.ts#L1-L7、code/core/src/backgrounds/defaults.ts#L3-L6、code/core/src/backgrounds/manager.tsx、code/core/src/backgrounds/manager.tsx#L1-L18、code/core/src/backgrounds/manager.tsx#L8-L17、code/core/src/backgrounds/preview.ts、code/core/src/backgrounds/preview.ts#L1-L37、code/core/src/backgrounds/preview.ts#L14-L23、code/core/src/backgrounds/preview.ts#L25-L27、code/core/src/backgrounds/types.ts、code/core/src/backgrounds/types.ts#L1-L60、code/core/src/backgrounds/types.ts#L10-L16、code/core/src/backgrounds/utils.ts、code/core/src/backgrounds/utils.ts#L1-L70、code/core/src/backgrounds/utils.ts#L45-L70、code/core/template/stories/backgrounds/globals.stories.ts、code/core/template/stories/backgrounds/globals.stories.ts#L1-L105、code/core/template/stories/backgrounds/globals.stories.ts#L10-L16、code/core/template/stories/backgrounds/globals.stories.ts#L19-L29、code/core/template/stories/backgrounds/globals.stories.ts#L31-L42、code/core/template/stories/backgrounds/globals.stories.ts#L54-L60、code/core/template/stories/backgrounds/globals.stories.ts#L74-L104、code/e2e-tests/addon-backgrounds.spec.ts、code/e2e-tests/addon-backgrounds.spec.ts#L1-L63、code/e2e-tests/addon-backgrounds.spec.ts#L18-L34、docs/essentials/backgrounds.mdx、docs/essentials/backgrounds.mdx#L1-L187
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：Tool.tsx、Tool.tsx#L1-L112、Tool.tsx#L13-L112、Tool.tsx#L17-L28、Tool.tsx#L22-L28、Tool.tsx#L74-L78、constants.ts、constants.ts#L1-L8

### 核心概念/Addons系统架构/核心Addons详解/Controls Addon.md

- reference 标题：Controls Addon
- 生成页：系统架构.md（系统架构）
- KnowledgeUnit：unit_id=unit-3ce08b39bd6b, unit_type=architecture, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：11
- skeleton_score：0.21
- key_source_coverage：0.00
- missing_key_sources：code/addons/docs/src/blocks/components/ArgsTable/ArgControl.tsx、code/addons/docs/src/blocks/components/ArgsTable/ArgControl.tsx#L1-L90、code/addons/docs/src/blocks/components/ArgsTable/ArgControl.tsx#L26-L90、code/addons/docs/src/blocks/components/ArgsTable/ArgControl.tsx#L46-L90、code/addons/docs/src/blocks/components/ArgsTable/ArgControl.tsx#L71-L84、code/addons/docs/src/blocks/components/ArgsTable/ArgsTable.tsx、code/addons/docs/src/blocks/components/ArgsTable/ArgsTable.tsx#L363-L399、code/addons/docs/src/blocks/controls/Boolean.tsx、code/addons/docs/src/blocks/controls/Color.tsx、code/addons/docs/src/blocks/controls/Date.tsx、code/addons/docs/src/blocks/controls/Files.tsx、code/addons/docs/src/blocks/controls/Number.tsx、code/addons/docs/src/blocks/controls/Object.tsx、code/addons/docs/src/blocks/controls/Range.tsx、code/addons/docs/src/blocks/controls/Text.tsx、code/addons/docs/src/blocks/controls/helpers.ts、code/core/core/src/manager/components/preview/tools/addons.tsx、code/core/src/controls/constants.ts、code/core/src/controls/constants.ts#L1-L3、code/core/src/manager/components/panel/Panel.tsx、code/core/src/manager/components/panel/Panel.tsx#L110-L154、code/core/src/manager/components/preview/tools/addons.tsx#L40-L68、code/e2e-tests/addon-controls.spec.ts、code/e2e-tests/addon-controls.spec.ts#L1-L121、code/e2e-tests/addon-controls.spec.ts#L10-L66、code/e2e-tests/addon-controls.spec.ts#L110-L120、code/e2e-tests/addon-controls.spec.ts#L68-L78、code/e2e-tests/addon-controls.spec.ts#L80-L108、docs/api/arg-types.mdx、docs/api/arg-types.mdx#L85-L125、docs/essentials/controls.mdx、docs/essentials/controls.mdx#L282-L297
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ArgControl.tsx、ArgControl.tsx#L1-L90、ArgControl.tsx#L26-L90、ArgControl.tsx#L46-L90、ArgControl.tsx#L71-L84、ArgsTable.tsx、ArgsTable.tsx#L363-L399、Boolean.tsx

### 核心概念/Addons系统架构/核心Addons详解/Docs Addon.md

- reference 标题：Docs Addon
- 生成页：系统架构.md（系统架构）
- KnowledgeUnit：unit_id=unit-3ce08b39bd6b, unit_type=architecture, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：11
- skeleton_score：0.19
- key_source_coverage：0.00
- missing_key_sources：code/addons/docs/package.json、code/addons/docs/package.json#L1-L133、code/addons/docs/package.json#L88-L117、code/addons/docs/src/DocsRenderer.tsx、code/addons/docs/src/DocsRenderer.tsx#L1-L88、code/addons/docs/src/DocsRenderer.tsx#L24-L46、code/addons/docs/src/DocsRenderer.tsx#L59-L62、code/addons/docs/src/blocks.ts、code/addons/docs/src/blocks.ts#L1-L10、code/addons/docs/src/blocks/blocks/DocsStory.tsx、code/addons/docs/src/blocks/blocks/DocsStory.tsx#L1-L39、code/addons/docs/src/index.ts、code/addons/docs/src/index.ts#L1-L21、code/addons/docs/src/manager.tsx、code/addons/docs/src/manager.tsx#L1-L108、code/addons/docs/src/manager.tsx#L46-L50、code/addons/docs/src/manager.tsx#L87-L87、code/addons/docs/src/manager.tsx#L87-L96、code/addons/docs/src/manifest.ts、code/addons/docs/src/manifest.ts#L52-L79、code/addons/docs/src/manifest.ts#L52-L93、code/addons/docs/src/mdx-react-shim.ts、code/addons/docs/src/mdx-react-shim.ts#L1-L2、code/addons/docs/src/types.ts、code/addons/docs/src/types.ts#L1-L257、code/addons/docs/src/types.ts#L151-L251、code/addons/docs/src/types.ts#L191-L208、code/addons/docs/src/types.ts#L66-L99、code/core/src/core-server/utils/StoryIndexGenerator.test.ts、code/core/src/core-server/utils/StoryIndexGenerator.test.ts#L1276-L1291、code/core/src/core-server/utils/__tests__/autoName.test.ts、code/core/src/core-server/utils/__tests__/autoName.test.ts#L1-L11、code/core/src/core-server/utils/autoName.ts、code/core/src/core-server/utils/autoName.ts#L1-L27、docs/writing-docs/mdx.mdx、docs/writing-docs/mdx.mdx#L38-L71、docs/writing-docs/mdx.mdx#L51-L51
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L133、package.json#L88-L117、DocsRenderer.tsx、DocsRenderer.tsx#L1-L88、DocsRenderer.tsx#L24-L46、DocsRenderer.tsx#L59-L62、blocks.ts

### 核心概念/Addons系统架构/核心Addons详解/Themes Addon.md

- reference 标题：Themes Addon
- 生成页：主题系统/themes.md（themes）
- KnowledgeUnit：unit_id=unit-e858919ec1df, unit_type=module_doc, domain_id=domain-9cda34dbf62b, readiness=compose_ready, child_digests=0
- reuse_count：6
- skeleton_score：0.23
- key_source_coverage：0.23
- missing_key_sources：.wiki/核心模块/code/addons/themes.md、.wiki/核心模块/code/addons/themes.md#L1-L18、code/addons/themes/src/constants.ts、code/addons/themes/src/constants.ts#L1-L18、code/addons/themes/src/decorators/helpers.ts、code/addons/themes/src/decorators/helpers.ts#L1-L40、code/addons/themes/src/decorators/helpers.ts#L34-L39、code/addons/themes/src/decorators/index.ts#L1-L6、code/addons/themes/src/decorators/provider.decorator.tsx#L1-L64、code/addons/themes/src/decorators/provider.decorator.tsx#L13-L18、code/addons/themes/src/decorators/provider.decorator.tsx#L23-L64、code/addons/themes/src/decorators/provider.decorator.tsx#L47-L62、code/addons/themes/src/index.ts#L1-L11、code/addons/themes/src/manager.tsx#L1-L15、code/addons/themes/src/preview.ts#L1-L8、code/addons/themes/src/theme-switcher.tsx#L1-L113、code/addons/themes/src/theme-switcher.tsx#L69-L71、code/addons/themes/src/theme-switcher.tsx#L73-L90、code/addons/themes/src/types.ts、code/addons/themes/src/types.ts#L1-L29
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：themes.md、themes.md#L1-L18、constants.ts、constants.ts#L1-L18、helpers.ts、helpers.ts#L1-L40、helpers.ts#L34-L39、index.ts#L1-L6

### 核心概念/Addons系统架构/核心Addons详解/Toolbars Addon.md

- reference 标题：工具栏插件（Toolbars Addon）
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- KnowledgeUnit：unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：14
- skeleton_score：0.13
- key_source_coverage：0.05
- missing_key_sources：code/core/src/toolbar/components/ToolbarManager.tsx、code/core/src/toolbar/components/ToolbarManager.tsx#L1-L30、code/core/src/toolbar/components/ToolbarManager.tsx#L11-L29、code/core/src/toolbar/components/ToolbarManager.tsx#L13-L17、code/core/src/toolbar/components/ToolbarMenuSelect.tsx、code/core/src/toolbar/components/ToolbarMenuSelect.tsx#L1-L148、code/core/src/toolbar/components/ToolbarMenuSelect.tsx#L103-L124、code/core/src/toolbar/components/ToolbarMenuSelect.tsx#L131-L146、code/core/src/toolbar/components/ToolbarMenuSelect.tsx#L135-L146、code/core/src/toolbar/components/ToolbarMenuSelect.tsx#L18-L27、code/core/src/toolbar/components/ToolbarMenuSelect.tsx#L28-L40、code/core/src/toolbar/components/ToolbarMenuSelect.tsx#L42-L146、code/core/src/toolbar/components/ToolbarMenuSelect.tsx#L49-L59、code/core/src/toolbar/components/ToolbarMenuSelect.tsx#L63-L95、code/core/src/toolbar/constants.ts、code/core/src/toolbar/constants.ts#L1-L3、code/core/src/toolbar/index.ts#L1-L4、code/core/src/toolbar/manager.tsx、code/core/src/toolbar/manager.tsx#L1-L17、code/core/src/toolbar/manager.tsx#L8-L16、code/core/src/toolbar/preview.ts#L1-L3、code/core/src/toolbar/types.ts、code/core/src/toolbar/types.ts#L1-L62、code/core/src/toolbar/types.ts#L16-L36、code/core/src/toolbar/utils/get-selected.ts、code/core/src/toolbar/utils/get-selected.ts#L1-L11、code/core/src/toolbar/utils/normalize-toolbar-arg-type.ts、code/core/src/toolbar/utils/normalize-toolbar-arg-type.ts#L1-L38、code/core/src/toolbar/utils/normalize-toolbar-arg-type.ts#L8-L37、code/core/src/toolbar/utils/register-shortcuts.ts、code/core/src/toolbar/utils/register-shortcuts.ts#L1-L40、code/core/src/toolbar/utils/register-shortcuts.ts#L12-L39、code/core/template/stories/toolbars/globals.stories.ts、code/core/template/stories/toolbars/globals.stories.ts#L24-L31、code/e2e-tests/addon-toolbars.spec.ts、code/e2e-tests/addon-toolbars.spec.ts#L14-L36、docs/essentials/toolbars-and-globals.mdx、docs/essentials/toolbars-and-globals.mdx#L104-L121、docs/essentials/toolbars-and-globals.mdx#L20-L27、docs/essentials/toolbars-and-globals.mdx#L36-L78、docs/essentials/toolbars-and-globals.mdx#L80-L98
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ToolbarManager.tsx、ToolbarManager.tsx#L1-L30、ToolbarManager.tsx#L11-L29、ToolbarManager.tsx#L13-L17、ToolbarMenuSelect.tsx、ToolbarMenuSelect.tsx#L1-L148、ToolbarMenuSelect.tsx#L103-L124、ToolbarMenuSelect.tsx#L131-L146

### 核心概念/Addons系统架构/核心Addons详解/Viewport Addon.md

- reference 标题：视口插件（Viewport Addon）
- 生成页：系统架构.md（系统架构）
- KnowledgeUnit：unit_id=unit-3ce08b39bd6b, unit_type=architecture, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：11
- skeleton_score：0.19
- key_source_coverage：0.03
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L107-L116、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L402-L415、code/addons/vitest/src/vitest-plugin/viewports.test.ts、code/addons/vitest/src/vitest-plugin/viewports.test.ts#L1-L297、code/addons/vitest/src/vitest-plugin/viewports.test.ts#L257-L278、code/addons/vitest/src/vitest-plugin/viewports.test.ts#L261-L278、code/addons/vitest/src/vitest-plugin/viewports.test.ts#L261-L297、code/addons/vitest/src/vitest-plugin/viewports.test.ts#L280-L297、code/addons/vitest/src/vitest-plugin/viewports.test.ts#L49-L63、code/addons/vitest/src/vitest-plugin/viewports.test.ts#L49-L93、code/addons/vitest/src/vitest-plugin/viewports.test.ts#L78-L96、code/addons/vitest/src/vitest-plugin/viewports.ts、code/addons/vitest/src/vitest-plugin/viewports.ts#L1-L97、code/addons/vitest/src/vitest-plugin/viewports.ts#L12-L28、code/addons/vitest/src/vitest-plugin/viewports.ts#L36-L54、code/addons/vitest/src/vitest-plugin/viewports.ts#L56-L96、code/addons/vitest/src/vitest-plugin/viewports.ts#L77-L81、code/core/src/viewport/defaults.ts、code/core/src/viewport/defaults.ts#L1-L268、code/core/src/viewport/defaults.ts#L234-L267、code/core/src/viewport/defaults.ts#L3-L228、code/core/src/viewport/index.ts、code/core/src/viewport/index.ts#L1-L5、code/e2e-tests/addon-viewport.spec.ts、code/e2e-tests/addon-viewport.spec.ts#L1-L60、code/e2e-tests/addon-viewport.spec.ts#L14-L23、code/e2e-tests/addon-viewport.spec.ts#L14-L58、code/e2e-tests/addon-viewport.spec.ts#L25-L43、code/e2e-tests/addon-viewport.spec.ts#L45-L58、code/e2e-tests/addon-viewport.spec.ts#L55-L58
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L107-L116、preview.tsx、preview.tsx#L1-L416、preview.tsx#L402-L415、viewports.test.ts、viewports.test.ts#L1-L297、viewports.test.ts#L257-L278

### 核心概念/Addons系统架构/核心Addons详解/核心Addons详解.md

- reference 标题：核心Addons详解
- 生成页：系统架构.md（系统架构）
- KnowledgeUnit：unit_id=unit-3ce08b39bd6b, unit_type=architecture, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：11
- skeleton_score：0.18
- key_source_coverage：0.00
- missing_key_sources：code/addons/a11y/package.json、code/addons/a11y/package.json#L1-L90、code/addons/a11y/package.json#L37-L52、code/addons/a11y/package.json#L60-L74、code/addons/docs/package.json、code/addons/docs/package.json#L1-L133、code/addons/docs/package.json#L40-L80、code/addons/docs/package.json#L88-L117、code/addons/themes/package.json、code/addons/themes/package.json#L1-L82、code/addons/themes/package.json#L36-L50、code/addons/themes/package.json#L58-L66、code/e2e-tests/addon-a11y.spec.ts、code/e2e-tests/addon-docs.spec.ts、code/e2e-tests/addon-themes.spec.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L90、package.json#L37-L52、package.json#L60-L74、package.json、package.json#L1-L133、package.json#L40-L80、package.json#L88-L117

### 核心概念/CSF格式规范.md

- reference 标题：CSF格式规范
- 生成页：概念指南/writing-stories.md（Writing Stories）
- KnowledgeUnit：unit_id=unit-8e98c6dd7f17, unit_type=concept_guide, domain_id=domain-be9fa08103e5, readiness=compose_ready, child_digests=11
- reuse_count：1
- skeleton_score：0.53
- key_source_coverage：0.00
- missing_key_sources：code/__mocks__/inject-decorator.ts.csf.txt、code/__mocks__/inject-decorator.ts.csf.txt#L1-L32、code/__mocks__/inject-decorator.ts.csf3.txt、code/__mocks__/inject-decorator.ts.csf3.txt#L1-L16、code/core/src/csf-tools/CsfFile.test.ts、code/core/src/csf-tools/CsfFile.test.ts#L1314-L1355、code/core/src/csf-tools/CsfFile.ts、code/core/src/csf-tools/CsfFile.ts#L1-L1113、code/core/src/csf-tools/CsfFile.ts#L210-L256、code/core/src/csf-tools/CsfFile.ts#L258-L293、code/core/src/csf-tools/CsfFile.ts#L280-L980、code/core/src/csf-tools/CsfFile.ts#L335-L351、code/core/src/csf-tools/CsfFile.ts#L353-L411、code/core/src/csf-tools/CsfFile.ts#L435-L980、code/core/src/csf-tools/CsfFile.ts#L538-L742、code/core/src/csf-tools/CsfFile.ts#L743-L782、code/core/src/csf-tools/CsfFile.ts#L818-L829、code/core/src/csf-tools/README.md、code/lib/codemod/build-config.ts、code/lib/codemod/src/transforms/csf-2-to-3.ts、code/lib/codemod/src/transforms/csf-2-to-3.ts#L1-L377、code/lib/codemod/src/transforms/csf-2-to-3.ts#L115-L222、code/lib/codemod/src/transforms/csf-2-to-3.ts#L15-L17、code/lib/csf-plugin/project.json、code/lib/eslint-plugin/docs/rules/csf-component.md、code/lib/eslint-plugin/docs/rules/csf-component.md#L1-L37、docs/_snippets/migrate-csf-2-to-3.md、docs/_snippets/migrate-csf-2-to-3.md#L1-L12、docs/_snippets/storybook-migrate-csf-2-to-3.md、docs/_snippets/storybook-migrate-csf-2-to-3.md#L1-L15、docs/api/csf/csf-next.mdx、docs/api/csf/index.mdx、docs/api/csf/index.mdx#L1-L314、docs/api/csf/index.mdx#L16-L314、docs/api/csf/index.mdx#L203-L314、docs/writing-docs/mdx.mdx、docs/writing-docs/mdx.mdx#L38-L71、docs/writing-stories/index.mdx、docs/writing-stories/index.mdx#L1-L422、docs/writing-stories/index.mdx#L51-L101
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：inject-decorator.ts.csf.txt、inject-decorator.ts.csf.txt#L1-L32、inject-decorator.ts.csf3.txt、inject-decorator.ts.csf3.txt#L1-L16、CsfFile.test.ts、CsfFile.test.ts#L1314-L1355、CsfFile.ts、CsfFile.ts#L1-L1113

### 核心概念/主题和参数系统.md

- reference 标题：主题和参数系统
- 生成页：主题系统/themes.md（themes）
- KnowledgeUnit：unit_id=unit-e858919ec1df, unit_type=module_doc, domain_id=domain-9cda34dbf62b, readiness=compose_ready, child_digests=0
- reuse_count：6
- skeleton_score：0.13
- key_source_coverage：0.00
- missing_key_sources：code/addons/a11y/template/stories/parameters.stories.ts、code/addons/a11y/template/stories/parameters.stories.ts#L1-L165、code/addons/a11y/template/stories/parameters.stories.ts#L8-L47、code/addons/docs/template/stories/docspage/basic.stories.ts、code/addons/docs/template/stories/docspage/basic.stories.ts#L1-L35、code/addons/docs/template/stories/docspage/basic.stories.ts#L7-L8、code/addons/themes/src/constants.ts、code/addons/themes/src/constants.ts#L1-L18、code/addons/themes/src/constants.ts#L3-L17、code/addons/themes/src/decorators/helpers.ts、code/addons/themes/src/decorators/helpers.ts#L1-L39、code/addons/themes/src/decorators/helpers.ts#L16-L32、code/addons/themes/src/decorators/helpers.ts#L16-L39、code/addons/themes/src/decorators/helpers.ts#L34-L39、code/addons/themes/src/types.ts、code/addons/themes/src/types.ts#L1-L29、code/core/src/preview-api/modules/store/GlobalsStore.ts、code/core/src/preview-api/modules/store/GlobalsStore.ts#L1-L73、code/core/src/preview-api/modules/store/GlobalsStore.ts#L15-L71、code/core/src/preview-api/modules/store/GlobalsStore.ts#L25-L71、code/core/src/preview-api/modules/store/GlobalsStore.ts#L39-L50、code/core/src/preview-api/modules/store/GlobalsStore.ts#L7-L73、code/core/src/preview-api/modules/store/csf/getValuesFromGlobalTypes.ts、code/core/src/preview-api/modules/store/csf/getValuesFromGlobalTypes.ts#L1-L10、code/core/src/preview-api/modules/store/csf/getValuesFromGlobalTypes.ts#L3-L9、code/core/src/preview-api/modules/store/parameters.ts、code/core/src/preview-api/modules/store/parameters.ts#L1-L43、code/core/src/preview-api/modules/store/parameters.ts#L11-L42、code/core/src/theming/convert.ts、code/core/src/theming/convert.ts#L113-L179、code/core/src/theming/create.ts、code/core/src/theming/create.ts#L1-L50、code/core/src/theming/create.ts#L29-L50、code/core/src/theming/tests/create.test.js、code/core/src/theming/tests/create.test.js#L1-L45、docs/writing-stories/decorators.mdx、docs/writing-stories/decorators.mdx#L58-L67
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：parameters.stories.ts、parameters.stories.ts#L1-L165、parameters.stories.ts#L8-L47、basic.stories.ts、basic.stories.ts#L1-L35、basic.stories.ts#L7-L8、constants.ts、constants.ts#L1-L18

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- KnowledgeUnit：unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：14
- skeleton_score：0.14
- key_source_coverage：0.08
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L169-L173、code/.storybook/main.ts#L19-L183、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L144-L334、code/.storybook/preview.tsx#L144-L415、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L195-L415、code/.storybook/preview.tsx#L208-L215、code/.storybook/preview.tsx#L220-L298、code/.storybook/preview.tsx#L336-L400、code/.storybook/preview.tsx#L402-L415、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L1-L30、code/.storybook/storybook.setup.ts#L11-L29、code/addons/themes/docs/api.md、code/addons/themes/docs/api.md#L79-L147、code/core/src/channels/postmessage/index.ts#L200-L239、code/core/src/channels/types.ts、code/core/src/channels/types.ts#L1-L59、code/core/src/channels/types.ts#L18-L50、code/core/src/core-server/utils/save-story/mocks/unsupported-csf-variances.stories.tsx、code/core/src/core-server/utils/save-story/mocks/unsupported-csf-variances.stories.tsx#L1-L20、code/core/src/manager-api/lib/addons.ts、code/core/src/manager-api/lib/addons.ts#L100-L109、code/core/src/manager-api/lib/addons.ts#L56-L111、code/core/src/manager-api/lib/addons.ts#L87-L111、code/core/src/preview-api/index.ts#L1-L90、code/core/src/theming/index.ts#L1-L51、code/core/src/theming/index.ts#L10-L51、code/core/template/stories/preview.ts#L1-L109、code/core/template/stories/preview.ts#L36-L109、code/renderers/react/template/components/index.js、code/renderers/react/template/stories/csf4.mdx、code/renderers/react/template/stories/csf4.mdx#L1-L8、docs/writing-stories/decorators.mdx、docs/writing-stories/decorators.mdx#L69-L94
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L1-L186、main.ts#L149-L181、main.ts#L169-L173、main.ts#L19-L183、manager.tsx、manager.tsx#L1-L9、manager.tsx#L4-L8

### 核心概念/组件故事（Stories）.md

- reference 标题：组件故事（Stories）
- 生成页：API-参考/组件故事-Stories.md（组件故事（Stories））
- KnowledgeUnit：unit_id=unit-6d187323f017, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.13
- key_source_coverage：0.00
- missing_key_sources：code/addons/docs/src/blocks/blocks/Canvas.stories.tsx、code/addons/docs/src/blocks/blocks/Canvas.stories.tsx#L1-L315、code/addons/docs/src/blocks/blocks/Canvas.stories.tsx#L13-L36、code/addons/docs/src/blocks/blocks/Canvas.stories.tsx#L40-L315、code/addons/docs/src/blocks/blocks/Story.stories.tsx、code/addons/docs/src/blocks/blocks/Story.stories.tsx#L1-L185、code/addons/docs/src/blocks/blocks/Story.stories.tsx#L12-L20、code/addons/docs/src/blocks/blocks/Story.stories.tsx#L146-L172、code/addons/docs/src/blocks/blocks/Story.stories.tsx#L24-L185、code/addons/docs/src/blocks/blocks/Story.stories.tsx#L97-L110、code/core/src/csf-tools/CsfFile.ts、code/core/src/csf-tools/CsfFile.ts#L632-L690、code/core/src/csf-tools/getStorySortParameter.ts、code/core/src/csf-tools/getStorySortParameter.ts#L102-L143、code/core/src/manager-api/modules/stories.ts、code/core/src/manager-api/modules/stories.ts#L1-L800、code/core/src/manager-api/modules/stories.ts#L172-L195、code/core/src/manager-api/modules/stories.ts#L183-L212、code/core/src/manager-api/modules/stories.ts#L381-L422、code/core/src/manager-api/modules/stories.ts#L550-L565、code/core/src/manager-api/modules/stories.ts#L79-L294、code/core/src/preview-api/README-store.md、code/core/src/preview-api/README-store.md#L38-L74、code/core/src/preview-api/README-store.md#L44-L74、code/core/src/preview-api/README-store.md#L70-L74、code/frameworks/react-vite/package.json、code/frameworks/react-vite/package.json#L1-L81、code/lib/cli-storybook/package.json、code/lib/cli-storybook/package.json#L1-L69、code/presets/create-react-app/package.json、code/presets/create-react-app/package.json#L1-L55、code/renderers/react/package.json、code/renderers/react/package.json#L1-L97
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：Canvas.stories.tsx、Canvas.stories.tsx#L1-L315、Canvas.stories.tsx#L13-L36、Canvas.stories.tsx#L40-L315、Story.stories.tsx、Story.stories.tsx#L1-L185、Story.stories.tsx#L12-L20、Story.stories.tsx#L146-L172

### 核心概念/装饰器和全局状态.md

- reference 标题：装饰器和全局状态
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- KnowledgeUnit：unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：14
- skeleton_score：0.13
- key_source_coverage：0.06
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L117-L120、code/.storybook/main.ts#L19-L120、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L220-L301、code/.storybook/preview.tsx#L220-L333、code/.storybook/preview.tsx#L309-L333、code/.storybook/preview.tsx#L402-L415、code/addons/themes/docs/api.md、code/addons/themes/docs/api.md#L1-L32、code/addons/themes/src/decorators/helpers.ts、code/addons/themes/src/decorators/helpers.ts#L1-L40、code/addons/themes/src/decorators/helpers.ts#L34-L39、code/addons/themes/src/decorators/provider.decorator.tsx、code/addons/themes/src/decorators/provider.decorator.tsx#L1-L64、code/addons/themes/src/decorators/provider.decorator.tsx#L23-L63、code/core/src/preview-api/modules/addons/hooks.ts、code/core/src/preview-api/modules/addons/hooks.ts#L649-L659、code/core/src/theming/index.ts#L10-L20、code/core/template/stories/preview.ts#L1-L109、code/core/template/stories/preview.ts#L56-L108、code/frameworks/nextjs-vite/src/styledJsx/decorator.tsx、code/frameworks/nextjs-vite/src/styledJsx/decorator.tsx#L1-L9、code/frameworks/nextjs/src/styledJsx/decorator.tsx、code/frameworks/nextjs/src/styledJsx/decorator.tsx#L1-L9
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L1-L186、main.ts#L117-L120、main.ts#L19-L120、manager.tsx、manager.tsx#L4-L8、preview.tsx、preview.tsx#L1-L416

### 核心概念/预览和管理界面.md

- reference 标题：预览和管理界面
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- KnowledgeUnit：unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：14
- skeleton_score：0.13
- key_source_coverage：0.00
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L107-L182、code/.storybook/main.ts#L19-L182、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L144-L415、code/.storybook/preview.tsx#L402-L415、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L11-L29、code/core/src/channels/main.ts、code/core/src/channels/main.ts#L22-L147、code/core/src/channels/main.ts#L62-L82、code/core/src/channels/types.ts、code/core/src/channels/types.ts#L1-L59、code/core/src/channels/types.ts#L18-L50、code/core/src/manager-api/lib/addons.ts、code/core/src/manager-api/lib/addons.ts#L100-L111、code/core/src/manager/App.tsx、code/core/src/manager/App.tsx#L24-L81、code/core/src/manager/App.tsx#L31-L61、code/core/src/manager/App.tsx#L44-L61、code/core/src/manager/App.tsx#L63-L80、code/core/src/manager/components/layout/Layout.tsx、code/core/src/manager/components/layout/Layout.tsx#L108-L112、code/core/src/manager/components/layout/Layout.tsx#L145-L210、code/core/src/manager/components/layout/MainAreaContainer.tsx、code/core/src/manager/components/layout/MainAreaContainer.tsx#L72-L91、code/core/src/manager/components/layout/MainAreaContainer.tsx#L82-L85、code/core/src/manager/components/panel/Panel.tsx、code/core/src/manager/components/panel/Panel.tsx#L86-L195、code/core/src/manager/components/sidebar/Sidebar.tsx、code/core/src/manager/components/sidebar/Sidebar.tsx#L105-L246、code/core/src/preview-api/modules/preview-web/Preview.tsx、code/core/src/preview-api/modules/preview-web/Preview.tsx#L422-L468、code/core/src/preview-api/modules/preview-web/PreviewWithSelection.tsx、code/core/src/preview-api/modules/preview-web/PreviewWithSelection.tsx#L510-L525、code/core/src/preview-api/modules/preview-web/render/StoryRender.ts、code/core/src/preview-api/modules/preview-web/render/StoryRender.ts#L59-L103
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L107-L182、main.ts#L19-L182、manager.tsx、manager.tsx#L4-L8、preview.tsx、preview.tsx#L144-L415、preview.tsx#L402-L415

### 测试框架/Playwright测试.md

- reference 标题：Playwright测试
- 生成页：多框架支持/react-vitest-3.md（react-vitest-3）
- KnowledgeUnit：unit_id=unit-ed51639dbf62, unit_type=module_doc, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=0
- reuse_count：6
- skeleton_score：0.12
- key_source_coverage：0.04
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L169-L173、code/.storybook/main.ts#L19-L186、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L195-L416、code/.storybook/preview.tsx#L220-L299、code/.storybook/preview.tsx#L37-L45、code/chromatic.config.json、code/chromatic.config.json#L1-L10、code/chromatic.config.json#L4-L8、code/e2e-tests/manager.spec.ts、code/e2e-tests/manager.spec.ts#L1-L346、code/e2e-tests/manager.spec.ts#L10-L346、code/e2e-tests/manager.spec.ts#L17-L279、code/e2e-tests/manager.spec.ts#L281-L344、code/e2e-tests/manager.spec.ts#L6-L8、code/e2e-tests/navigation.spec.ts、code/e2e-tests/navigation.spec.ts#L1-L24、code/e2e-tests/navigation.spec.ts#L8-L23、code/e2e-tests/preview-api.spec.ts、code/e2e-tests/preview-api.spec.ts#L1-L95、code/e2e-tests/preview-api.spec.ts#L11-L95、code/e2e-tests/preview-api.spec.ts#L18-L93、code/e2e-tests/preview-api.spec.ts#L6-L7、code/e2e-tests/util.ts、code/e2e-tests/util.ts#L1-L300、code/e2e-tests/util.ts#L117-L148、code/e2e-tests/util.ts#L176-L187、code/e2e-tests/util.ts#L252-L273、code/e2e-tests/util.ts#L8-L274、code/e2e-tests/util.ts#L96-L148、code/package.json、code/package.json#L15-L44、code/playwright.config.ts、code/playwright.config.ts#L1-L115、code/playwright.config.ts#L11-L115、code/playwright.config.ts#L22-L30、code/playwright.config.ts#L31-L51、code/playwright.config.ts#L54-L104、code/playwright.config.ts#L6-L8、code/vitest.config.ts、code/vitest.config.ts#L1-L57、code/vitest.config.ts#L12-L13、code/vitest.config.ts#L33-L56、docs/writing-stories/mocking-data-and-modules/mocking-network-requests.mdx#L1-L36
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L169-L173、main.ts#L19-L186、preview.tsx#L1-L416、preview.tsx#L195-L334、preview.tsx#L195-L416、preview.tsx#L220-L299、preview.tsx#L37-L45

### 测试框架/Vitest集成.md

- reference 标题：Vitest集成
- 生成页：无
- 问题：缺少对应生成页面

### 测试框架/可访问性测试.md

- reference 标题：可访问性测试
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- KnowledgeUnit：unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：14
- skeleton_score：0.11
- key_source_coverage：0.02
- missing_key_sources：code/addons/a11y/package.json、code/addons/a11y/package.json#L1-L90、code/addons/a11y/package.json#L60-L77、code/addons/a11y/src/a11yRunner.ts、code/addons/a11y/src/a11yRunner.ts#L1-L136、code/addons/a11y/src/a11yRunner.ts#L123-L135、code/addons/a11y/src/a11yRunner.ts#L18-L22、code/addons/a11y/src/a11yRunner.ts#L24-L41、code/addons/a11y/src/a11yRunner.ts#L43-L121、code/addons/a11y/src/a11yRunner.ts#L57-L84、code/addons/a11y/src/a11yRunner.ts#L86-L93、code/addons/a11y/src/a11yRunnerUtils.ts、code/addons/a11y/src/a11yRunnerUtils.ts#L1-L35、code/addons/a11y/src/a11yRunnerUtils.ts#L10-L34、code/addons/a11y/src/constants.ts、code/addons/a11y/src/constants.ts#L1-L22、code/addons/a11y/src/constants.ts#L6-L18、code/addons/a11y/src/index.ts#L1-L11、code/addons/a11y/src/manager.tsx、code/addons/a11y/src/manager.tsx#L1-L67、code/addons/a11y/src/manager.tsx#L14-L66、code/addons/a11y/src/manager.tsx#L17-L30、code/addons/a11y/src/manager.tsx#L50-L66、code/addons/a11y/src/params.ts、code/addons/a11y/src/params.ts#L1-L45、code/addons/a11y/src/params.ts#L20-L44、code/addons/a11y/src/params.ts#L21-L27、code/addons/a11y/src/params.ts#L35-L39、code/addons/a11y/src/preview.tsx、code/addons/a11y/src/preview.tsx#L1-L110、code/addons/a11y/src/preview.tsx#L14-L96、code/addons/a11y/src/preview.tsx#L63-L75、code/addons/a11y/src/types.ts、code/addons/a11y/src/types.ts#L1-L68、code/addons/a11y/src/types.ts#L16-L31、code/addons/a11y/src/utils.ts、code/addons/a11y/src/utils.ts#L1-L18、code/addons/a11y/src/utils.ts#L10-L17、code/e2e-tests/addon-a11y.spec.ts、code/e2e-tests/addon-a11y.spec.ts#L14-L81、code/lib/create-storybook/src/commands/AddonConfigurationCommand.ts、code/lib/create-storybook/src/commands/AddonConfigurationCommand.ts#L109-L153、docs/_snippets/addon-a11y-config-in-preview.md、docs/_snippets/addon-a11y-config-in-preview.md#L1-L32、docs/writing-tests/accessibility-testing.mdx、docs/writing-tests/accessibility-testing.mdx#L60-L66
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L90、package.json#L60-L77、a11yRunner.ts、a11yRunner.ts#L1-L136、a11yRunner.ts#L123-L135、a11yRunner.ts#L18-L22、a11yRunner.ts#L24-L41

### 测试框架/测试框架.md

- reference 标题：测试框架
- 生成页：无
- 问题：缺少对应生成页面

### 测试框架/组件测试.md

- reference 标题：组件测试
- 生成页：多框架支持/react-vitest-3.md（react-vitest-3）
- KnowledgeUnit：unit_id=unit-ed51639dbf62, unit_type=module_doc, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=0
- reuse_count：6
- skeleton_score：0.20
- key_source_coverage：0.03
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L37-L45、code/.storybook/preview.tsx#L402-L416、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L1-L30、code/.storybook/storybook.setup.ts#L11-L29、code/addons/vitest/README.md、code/core/core/src/mock/extract.ts、code/core/src/component-testing/components/test-fn.stories.tsx、code/core/src/component-testing/components/test-fn.stories.tsx#L1-L178、code/core/src/component-testing/components/test-fn.stories.tsx#L103-L125、code/core/src/component-testing/components/test-fn.stories.tsx#L11-L26、code/core/src/component-testing/components/test-fn.stories.tsx#L132-L178、code/core/src/component-testing/components/test-fn.stories.tsx#L36-L101、code/core/src/component-testing/components/test-fn.stories.tsx#L36-L47、code/core/src/component-testing/components/test-fn.stories.tsx#L37-L47、code/core/src/component-testing/components/test-fn.stories.tsx#L89-L101、code/core/src/mock/extract.ts#L57-L78、code/core/src/mock/mocker-runtime.js、code/core/src/mock/mocker-runtime.js#L52-L89、code/e2e-tests/sb-module-mocking.spec.ts、code/e2e-tests/sb-module-mocking.spec.ts#L32-L47、code/vitest-setup.ts、code/vitest-setup.ts#L1-L123、code/vitest-setup.ts#L43-L51、code/vitest-setup.ts#L55-L56、code/vitest-setup.ts#L58-L72、code/vitest.config.storybook.ts、code/vitest.config.storybook.ts#L1-L56、code/vitest.config.storybook.ts#L20-L55、code/vitest.config.storybook.ts#L21-L55、code/vitest.config.storybook.ts#L41-L51、code/vitest.config.ts、code/vitest.config.ts#L1-L57、code/vitest.config.ts#L12-L13、code/vitest.config.ts#L15-L31、code/vitest.config.ts#L43-L54、code/vitest.helpers.ts、code/vitest.helpers.ts#L1-L37、code/vitest.helpers.ts#L18-L37、code/vitest.shared.ts、code/vitest.shared.ts#L1-L16、docs/_snippets/individual-snapshot-tests-portable-stories.md、docs/_snippets/individual-snapshot-tests-portable-stories.md#L146-L205、docs/_snippets/individual-snapshot-tests-portable-stories.md#L178-L204、docs/_snippets/portable-stories-vitest-compose-stories.md、docs/_snippets/portable-stories-vitest-compose-stories.md#L55-L80、docs/_snippets/portable-stories-vitest-snapshot-test.md、docs/_snippets/portable-stories-vitest-snapshot-test.md#L134-L172、docs/_snippets/portable-stories-vitest-snapshot-test.md#L84-L174、docs/_snippets/portable-stories-vitest-with-play-function.md、docs/_snippets/portable-stories-vitest-with-play-function.md#L1-L44、docs/writing-stories/mocking-data-and-modules/mocking-modules.mdx、docs/writing-stories/mocking-data-and-modules/mocking-modules.mdx#L164-L177
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、preview.tsx#L1-L416、preview.tsx#L195-L334、preview.tsx#L37-L45、preview.tsx#L402-L416、storybook.setup.ts、storybook.setup.ts#L1-L30、storybook.setup.ts#L11-L29

### 测试框架/视觉回归测试.md

- reference 标题：视觉回归测试
- 生成页：多框架支持/react-vitest-3.md（react-vitest-3）
- KnowledgeUnit：unit_id=unit-ed51639dbf62, unit_type=module_doc, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=0
- reuse_count：6
- skeleton_score：0.13
- key_source_coverage：0.07
- missing_key_sources：code/.storybook/main.ts#L19-L183、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L220-L301、code/.storybook/preview.tsx#L402-L415、code/chromatic.config.json、code/chromatic.config.json#L1-L9、code/chromatic.config.json#L4-L8、code/chromatic.config.json#L5-L6、code/e2e-tests/component-tests.spec.ts、code/e2e-tests/component-tests.spec.ts#L1-L189、code/e2e-tests/component-tests.spec.ts#L17-L24、code/e2e-tests/component-tests.spec.ts#L53-L56、code/e2e-tests/component-tests.spec.ts#L9-L146、code/package.json、code/package.json#L15-L44、code/package.json#L39-L39、code/package.json#L72-L149、code/playwright.config.ts、code/playwright.config.ts#L11-L114、code/playwright.config.ts#L31-L41、code/vitest.config.storybook.ts、code/vitest.config.storybook.ts#L21-L55、code/vitest.config.storybook.ts#L36-L37、code/vitest.config.storybook.ts#L41-L51、code/vitest.config.ts、code/vitest.config.ts#L12-L13、code/vitest.config.ts#L33-L56
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L19-L183、preview.tsx#L195-L334、preview.tsx#L220-L301、preview.tsx#L402-L415、chromatic.config.json、chromatic.config.json#L1-L9、chromatic.config.json#L4-L8、chromatic.config.json#L5-L6

### 贡献指南.md

- reference 标题：贡献指南
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.12
- key_source_coverage：0.03
- missing_key_sources：.github/PULL_PR_TEMPLATE.md、.github/PULL_REQUEST_TEMPLATE.md#L1-L80、.github/PULL_REQUEST_TEMPLATE.md#L41-L48、.github/PULL_REQUEST_TEMPLATE.md#L49-L66、.yarnrc.yml、.yarnrc.yml#L15-L38、CODE_OF_CONDUCT.md、CODE_OF_CONDUCT.md#L1-L135、CONTRIBUTING.md#L15-L21、CONTRIBUTING.md#L157-L218、CONTRIBUTING.md#L209-L218、CONTRIBUTING.md#L220-L238、CONTRIBUTING.md#L25-L48、SECURITY.md#L1-L30、code/package.json、code/package.json#L15-L44、code/playwright.config.ts、code/playwright.config.ts#L11-L114、code/vitest.config.ts、code/vitest.config.ts#L12-L13、code/vitest.config.ts#L15-L41、code/vitest.config.ts#L33-L56、nx.json、nx.json#L1-L220、nx.json#L24-L198、nx.json#L36-L49、nx.json#L4、nx.json#L5-L5、nx.json#L57-L97、package.json、package.json#L1-L74、package.json#L18-L32、package.json#L18-L38、package.json#L27-L31、package.json#L5-L16、scripts/package.json、scripts/package.json#L12-L13、scripts/package.json#L14-L23、scripts/package.json#L6-L43
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：PULL_PR_TEMPLATE.md、PULL_REQUEST_TEMPLATE.md#L1-L80、PULL_REQUEST_TEMPLATE.md#L41-L48、PULL_REQUEST_TEMPLATE.md#L49-L66、.yarnrc.yml、.yarnrc.yml#L15-L38、CODE_OF_CONDUCT.md、CODE_OF_CONDUCT.md#L1-L135

### 部署和CI_CD/CI_CD集成.md

- reference 标题：CI/CD集成
- 生成页：无
- 问题：缺少对应生成页面

### 部署和CI_CD/Chromatic集成.md

- reference 标题：Chromatic集成
- 生成页：无
- 问题：缺少对应生成页面

### 部署和CI_CD/环境配置.md

- reference 标题：环境配置
- 生成页：无
- 问题：缺少对应生成页面

### 部署和CI_CD/部署和CI_CD.md

- reference 标题：部署和CI/CD
- 生成页：开发工具/开发工具.md（开发工具）
- KnowledgeUnit：unit_id=unit-df10871c9c97, unit_type=domain_index, domain_id=domain-1ad09e98abab, readiness=compose_ready, child_digests=5
- reuse_count：2
- skeleton_score：0.07
- key_source_coverage：0.07
- missing_key_sources：.circleci/config.yml、.circleci/config.yml#L1-L55、.circleci/config.yml#L42-L49、.nx/workflows/distribution-config.yaml、.nx/workflows/distribution-config.yaml#L1-L36、README.md、README.md#L1-L244、README.md#L79-L88、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L129-L181、code/.storybook/main.ts#L170-L172、code/.storybook/main.ts#L170-L179、code/chromatic.config.json、code/chromatic.config.json#L1-L10、code/chromatic.config.json#L4-L7、code/package.json、code/package.json#L1-L219、code/package.json#L15-L44、code/package.json#L37-L40、code/package.json#L38-L38、code/package.json#L39-L39、code/project.json、code/project.json#L1-L31、package.json、package.json#L1-L74、scripts/ci/main.ts#L1-L156、scripts/ci/main.ts#L149-L155、scripts/ci/main.ts#L39-L139
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：config.yml、config.yml#L1-L55、config.yml#L42-L49、distribution-config.yaml、distribution-config.yaml#L1-L36、README.md、README.md#L1-L244、README.md#L79-L88

### 部署和CI_CD/静态部署.md

- reference 标题：静态部署
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- KnowledgeUnit：unit_id=unit-1c199b5b37f5, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：14
- skeleton_score：0.11
- key_source_coverage：0.00
- missing_key_sources：.circleci/config.yml、.circleci/config.yml#L1-L55、.circleci/config.yml#L31-L49、code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L129-L183、code/.storybook/main.ts#L169-L173、code/.storybook/main.ts#L19-L183、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L195-L415、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L1-L30、code/.storybook/storybook.setup.ts#L11-L29、code/.storybook/storybook.setup.ts#L17-L27、code/chromatic.config.json、code/chromatic.config.json#L1-L10、code/chromatic.config.json#L2-L8、code/package.json、code/package.json#L15-L44、code/package.json#L37-L42、package.json、package.json#L1-L74、package.json#L5-L17、scripts/build-package.ts、scripts/build-package.ts#L1-L189、scripts/build-package.ts#L152-L182、scripts/build-package.ts#L27-L182
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：config.yml、config.yml#L1-L55、config.yml#L31-L49、main.ts、main.ts#L1-L186、main.ts#L129-L183、main.ts#L169-L173、main.ts#L19-L183

### 项目概述/使用场景和案例.md

- reference 标题：使用场景和案例
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.14
- key_source_coverage：0.18
- missing_key_sources：README.md、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L169-L179、code/.storybook/main.ts#L19-L129、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L336-L400、code/package.json、code/package.json#L15-L45、code/package.json#L74-L106、docs/get-started/index.mdx#L1-L6、docs/index.mdx#L6-L36、docs/index.mdx#L8-L36、docs/writing-docs/index.mdx#L9-L19、docs/writing-stories/index.mdx#L22-L50、docs/writing-stories/index.mdx#L283-L331、docs/writing-stories/index.mdx#L51-L82、docs/writing-stories/index.mdx#L75-L81、docs/writing-tests/index.mdx#L114-L153、docs/writing-tests/index.mdx#L13-L29、docs/writing-tests/index.mdx#L137-L153、docs/writing-tests/index.mdx#L17-L29、docs/writing-tests/index.mdx#L35-L64、docs/writing-tests/index.mdx#L59-L113、package.json、package.json#L5-L16
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：README.md、main.ts#L149-L181、main.ts#L169-L179、main.ts#L19-L129、preview.tsx、preview.tsx#L144-L193、preview.tsx#L195-L334、preview.tsx#L336-L400

### 项目概述/基本概念.md

- reference 标题：基本概念
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.16
- missing_key_sources：README.md、README.md#L47-L94、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L106-L148、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L19-L132、code/.storybook/main.ts#L19-L182、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L415、code/.storybook/preview.tsx#L157-L193、code/.storybook/preview.tsx#L195-L415、code/core/src/components/components/Button/Button.stories.tsx、code/core/src/components/components/Button/Button.stories.tsx#L1-L409、code/core/src/components/components/Button/Button.stories.tsx#L11-L409、docs/api/csf/index.mdx#L1-L314、docs/api/csf/index.mdx#L12-L140、docs/configure/user-interface/storybook-addons.mdx、docs/configure/user-interface/storybook-addons.mdx#L1-L28、docs/configure/user-interface/storybook-addons.mdx#L10-L20、docs/configure/user-interface/storybook-addons.mdx#L8-L28、docs/get-started/index.mdx#L1-L6、docs/writing-docs/index.mdx#L1-L20、docs/writing-docs/index.mdx#L9-L20、docs/writing-stories/index.mdx#L1-L422、docs/writing-stories/index.mdx#L22-L321、docs/writing-stories/index.mdx#L33-L101、docs/writing-tests/index.mdx#L1-L240、docs/writing-tests/index.mdx#L35-L156、docs/writing-tests/index.mdx#L9-L156、docs/writing-tests/index.mdx#L9-L30
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：README.md、README.md#L47-L94、main.ts#L1-L186、main.ts#L106-L148、main.ts#L149-L181、main.ts#L19-L132、main.ts#L19-L182、preview.tsx

### 项目概述/技术架构/Monorepo设计.md

- reference 标题：Monorepo设计
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.00
- missing_key_sources：code/addons/a11y/package.json、code/addons/a11y/package.json#L1-L90、code/addons/a11y/package.json#L37-L52、code/addons/a11y/package.json#L60-L63、code/builders/builder-vite/package.json、code/builders/builder-vite/package.json#L1-L73、code/builders/builder-vite/package.json#L30-L39、code/builders/builder-vite/package.json#L49-L52、code/builders/builder-vite/package.json#L64-L67、code/core/package.json、code/core/package.json#L1-L389、code/core/package.json#L230-L243、code/core/package.json#L38-L221、code/core/package.json#L48-L221、code/frameworks/react-vite/package.json、code/frameworks/react-vite/package.json#L1-L81、code/frameworks/react-vite/package.json#L28-L45、code/frameworks/react-vite/package.json#L54-L64、code/frameworks/react-vite/package.json#L70-L75、code/lib/create-storybook/package.json、code/lib/create-storybook/package.json#L1-L61、code/package.json、code/package.json#L1-L219、code/package.json#L15-L45、code/package.json#L74-L106、code/package.json#L74-L151、code/project.json、code/renderers/react/package.json、code/renderers/react/package.json#L1-L97、code/renderers/react/package.json#L24-L44、code/renderers/react/package.json#L54-L59、code/renderers/react/package.json#L81-L86、nx.json、nx.json#L1-L220、nx.json#L24-L198、nx.json#L5-L218、nx.json#L92-L149、nx.json#L92-L198、package.json、package.json#L1-L74、package.json#L40-L60、package.json#L5-L17、scripts/package.json、scripts/package.json#L1-L189、scripts/package.json#L21-L23、scripts/package.json#L6-L44
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L90、package.json#L37-L52、package.json#L60-L63、package.json、package.json#L1-L73、package.json#L30-L39、package.json#L49-L52

### 项目概述/技术架构/技术架构.md

- reference 标题：技术架构
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.07
- key_source_coverage：0.00
- missing_key_sources：code/addons/docs/package.json、code/addons/docs/package.json#L1-L133、code/addons/docs/package.json#L40-L79、code/core/package.json、code/core/package.json#L1-L389、code/core/package.json#L48-L221、code/core/package.json#L48-L222、code/core/src/bin/dispatcher.ts、code/core/src/bin/dispatcher.ts#L1-L88、code/core/src/bin/dispatcher.ts#L23-L34、code/core/src/bin/dispatcher.ts#L23-L85、code/core/src/bin/dispatcher.ts#L36-L85、code/core/src/cli/index.ts、code/core/src/cli/index.ts#L1-L10、code/core/src/core-server/index.ts、code/core/src/core-server/index.ts#L1-L35、code/core/src/core-server/index.ts#L5-L13、code/core/src/manager-api/index.ts、code/core/src/manager-api/index.ts#L1-L27、code/core/src/manager/manager-stores.ts、code/core/src/preview-api/index.ts、code/core/src/preview-api/index.ts#L1-L90、code/core/src/preview-api/index.ts#L46-L88、code/frameworks/react-vite/package.json、code/frameworks/react-vite/package.json#L1-L81、code/frameworks/react-vite/package.json#L28-L44、code/lib/cli-sb/package.json、code/lib/cli-sb/package.json#L1-L36、code/lib/cli-sb/package.json#L27-L30、code/lib/cli-storybook/package.json、code/lib/cli-storybook/package.json#L1-L69、code/lib/cli-storybook/package.json#L29-L32、code/package.json、code/package.json#L1-L219、code/package.json#L15-L45、code/renderers/react/package.json、code/renderers/react/package.json#L1-L97、code/renderers/react/package.json#L23-L44、package.json、package.json#L1-L74、package.json#L5-L16
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L133、package.json#L40-L79、package.json、package.json#L1-L389、package.json#L48-L221、package.json#L48-L222、dispatcher.ts

### 项目概述/技术架构/插件系统架构/插件架构设计.md

- reference 标题：插件架构设计
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.07
- key_source_coverage：0.00
- missing_key_sources：code/addons/a11y/package.json、code/addons/a11y/package.json#L1-L90、code/addons/a11y/package.json#L37-L52、code/addons/a11y/src/index.ts、code/addons/a11y/src/index.ts#L1-L11、code/addons/a11y/src/index.ts#L10-L11、code/addons/docs/package.json、code/addons/docs/package.json#L1-L133、code/addons/docs/package.json#L40-L80、code/addons/docs/src/index.ts、code/addons/docs/src/index.ts#L1-L21、code/addons/docs/src/index.ts#L20-L21、code/core/src/common/utils/get-addon-annotations.ts、code/core/src/common/utils/get-addon-annotations.ts#L1-L48、code/core/src/common/utils/get-addon-annotations.ts#L12-L48、code/core/src/common/utils/get-addon-annotations.ts#L27-L48、code/core/src/common/utils/sync-main-preview-addons.ts、code/core/src/common/utils/sync-main-preview-addons.ts#L47-L100、code/core/src/csf/csf-factories.ts、code/core/src/csf/csf-factories.ts#L1-L286、code/core/src/csf/csf-factories.ts#L26-L82、code/core/src/csf/csf-factories.ts#L45-L82、code/core/src/manager-api/modules/addons.ts、code/core/src/manager-api/modules/addons.ts#L1-L134、code/core/src/manager-api/modules/addons.ts#L13-L134、code/core/src/manager-api/modules/addons.ts#L18-L74、code/core/src/manager-api/modules/addons.ts#L93-L134、code/core/src/preview-api/addons.ts、code/core/src/preview-api/addons.ts#L1-L3、code/core/src/types/modules/addons.ts、code/core/src/types/modules/addons.ts#L1-L522、code/core/src/types/modules/addons.ts#L26-L522、code/core/src/types/modules/addons.ts#L329-L450、code/core/src/types/modules/addons.ts#L493-L522、docs/_snippets/storybook-addon-load-external-addons-preset.md、docs/_snippets/storybook-addon-load-external-addons-preset.md#L1-L15
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L90、package.json#L37-L52、index.ts、index.ts#L1-L11、index.ts#L10-L11、package.json、package.json#L1-L133

### 项目概述/技术架构/插件系统架构/插件注册机制.md

- reference 标题：插件注册机制
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.15
- key_source_coverage：0.00
- missing_key_sources：code/addons/docs/package.json、code/addons/docs/package.json#L125-L131、code/addons/docs/package.json#L40-L80、code/addons/links/package.json、code/addons/links/package.json#L29-L47、code/addons/links/package.json#L74-L80、code/addons/themes/package.json、code/addons/themes/package.json#L36-L50、code/addons/themes/package.json#L74-L80、code/core/src/manager-api/lib/addons.ts、code/core/src/manager-api/lib/addons.ts#L120-L129、code/core/src/manager-api/lib/addons.ts#L121-L123、code/core/src/manager-api/lib/addons.ts#L31-L148、code/lib/cli-storybook/src/postinstallAddon.ts、code/lib/cli-storybook/src/postinstallAddon.ts#L10-L49、code/lib/cli-storybook/src/postinstallAddon.ts#L39-L42、code/lib/eslint-plugin/src/rules/no-uninstalled-addons.ts、code/lib/eslint-plugin/src/rules/no-uninstalled-addons.ts#L196-L200、code/lib/eslint-plugin/src/rules/no-uninstalled-addons.ts#L97-L130、docs/_snippets/storybook-addon-load-external-addons-preset.md、docs/_snippets/storybook-addon-load-external-addons-preset.md#L1-L15、docs/_snippets/storybook-addons-api-getqueryparam.md、docs/_snippets/storybook-addons-api-getqueryparam.md#L1-L6、docs/_snippets/storybook-addons-api-selectincurrentkind.md、docs/_snippets/storybook-addons-api-selectincurrentkind.md#L1-L6、docs/_snippets/storybook-addons-root-preset-manager-entries.md、docs/_snippets/storybook-addons-root-preset-manager-entries.md#L1-L6、docs/_snippets/storybook-main-register-addon.md、docs/_snippets/storybook-main-register-addon.md#L1-L6、docs/addons/addon-knowledge-base.mdx、docs/addons/addon-knowledge-base.mdx#L114-L124
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L125-L131、package.json#L40-L80、package.json、package.json#L29-L47、package.json#L74-L80、package.json、package.json#L36-L50

### 项目概述/技术架构/插件系统架构/插件生命周期管理.md

- reference 标题：插件生命周期管理
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.15
- key_source_coverage：0.04
- missing_key_sources：code/.storybook/main.ts#L107-L116、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L19-L183、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L402-L415、code/core/src/builder-manager/index.ts、code/core/src/builder-manager/index.ts#L174-L239、code/core/src/builder-manager/index.ts#L36-L121、code/core/src/common/utils/remove.ts#L46-L63、code/core/src/csf/csf-factories.ts、code/core/src/csf/csf-factories.ts#L45-L72、code/core/src/instrumenter/instrumenter.test.ts#L696-L735、code/core/src/instrumenter/instrumenter.ts、code/core/src/instrumenter/instrumenter.ts#L749-L766、code/core/src/manager-api/index.ts、code/core/src/manager-api/index.ts#L1-L27、code/core/src/manager/components/preview/Preview.tsx、code/core/src/manager/components/preview/Preview.tsx#L176-L197、code/core/src/preview-api/modules/preview-web/render/StoryRender.test.ts、code/core/src/preview-api/modules/preview-web/render/StoryRender.test.ts#L111-L163、code/core/src/preview-api/modules/preview-web/render/StoryRender.test.ts#L444-L476、code/lib/cli-storybook/src/postinstallAddon.ts、code/lib/cli-storybook/src/postinstallAddon.ts#L10-L49
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L107-L116、main.ts#L149-L181、main.ts#L19-L183、manager.tsx、manager.tsx#L1-L9、preview.tsx、preview.tsx#L144-L193、preview.tsx#L402-L415

### 项目概述/技术架构/插件系统架构/插件系统架构.md

- reference 标题：插件系统架构
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.14
- key_source_coverage：0.00
- missing_key_sources：code/addons/a11y/package.json、code/addons/a11y/package.json#L1-L90、code/addons/a11y/package.json#L37-L52、code/addons/a11y/src/index.ts、code/addons/a11y/src/index.ts#L1-L11、code/addons/a11y/src/index.ts#L6-L11、code/addons/docs/package.json、code/addons/docs/package.json#L1-L133、code/addons/docs/package.json#L40-L74、code/addons/docs/src/index.ts、code/addons/docs/src/index.ts#L1-L21、code/addons/links/package.json、code/addons/links/package.json#L1-L82、code/addons/links/package.json#L29-L47、code/addons/links/src/index.ts、code/addons/links/src/index.ts#L1-L8、code/core/src/manager-api/lib/addons.ts、code/core/src/manager-api/lib/addons.ts#L1-L149、code/core/src/manager-api/lib/addons.ts#L120-L125、code/core/src/manager-api/lib/addons.ts#L120-L129、code/core/src/manager-api/lib/addons.ts#L31-L149、code/core/src/manager-api/lib/addons.ts#L50-L66、code/core/src/manager-api/modules/addons.ts、code/core/src/manager-api/modules/addons.ts#L1-L134、code/core/src/manager-api/modules/addons.ts#L103-L121、code/core/src/manager-api/modules/addons.ts#L18-L134、code/core/src/manager-api/modules/addons.ts#L93-L134、code/core/src/manager-api/root.tsx、code/core/src/manager-api/root.tsx#L1-L200、code/core/src/manager-api/root.tsx#L127-L128、code/core/src/manager-api/root.tsx#L130-L197、code/core/src/manager-api/root.tsx#L170-L187、code/core/src/manager-api/root.tsx#L170-L197、code/core/src/manager-api/store.ts、code/core/src/manager-api/store.ts#L1-L123、code/core/src/manager-api/store.ts#L48-L123、code/core/src/manager-api/store.ts#L75-L121、code/core/src/types/modules/addons.ts、code/core/src/types/modules/addons.ts#L493-L521、docs/_snippets/storybook-addon-load-external-addons-preset.md、docs/_snippets/storybook-addon-load-external-addons-preset.md#L1-L15、docs/_snippets/storybook-addons-api-getqueryparam.md、docs/_snippets/storybook-addons-api-getqueryparam.md#L1-L6、docs/_snippets/storybook-addons-api-on.md、docs/_snippets/storybook-addons-api-on.md#L1-L7、docs/_snippets/storybook-addons-root-preset.md、docs/_snippets/storybook-addons-root-preset.md#L1-L8
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L90、package.json#L37-L52、index.ts、index.ts#L1-L11、index.ts#L6-L11、package.json、package.json#L1-L133

### 项目概述/技术架构/插件系统架构/插件通信协议.md

- reference 标题：插件通信协议
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.14
- key_source_coverage：0.03
- missing_key_sources：code/core/src/channels/index.test.ts、code/core/src/channels/index.test.ts#L268-L346、code/core/src/channels/index.test.ts#L40-L346、code/core/src/channels/index.ts、code/core/src/channels/index.ts#L1-L60、code/core/src/channels/index.ts#L19-L51、code/core/src/channels/index.ts#L32-L51、code/core/src/channels/main.ts#L1-L148、code/core/src/channels/main.ts#L22-L148、code/core/src/channels/main.ts#L57-L82、code/core/src/channels/main.ts#L62-L82、code/core/src/channels/main.ts#L84-L86、code/core/src/channels/postmessage/index.ts、code/core/src/channels/postmessage/index.ts#L1-L240、code/core/src/channels/postmessage/index.ts#L194-L238、code/core/src/channels/postmessage/index.ts#L29-L240、code/core/src/channels/postmessage/index.ts#L66-L129、code/core/src/channels/postmessage/index.ts#L66-L137、code/core/src/channels/postmessage/index.ts#L66-L192、code/core/src/channels/types.ts、code/core/src/channels/types.ts#L1-L60、code/core/src/channels/types.ts#L13-L50、code/core/src/channels/types.ts#L13-L60、code/core/src/channels/types.ts#L18-L50、code/core/src/channels/websocket/index.ts、code/core/src/channels/websocket/index.ts#L1-L109、code/core/src/channels/websocket/index.ts#L23-L109、code/core/src/channels/websocket/index.ts#L36-L75、code/core/src/channels/websocket/index.ts#L44-L75、code/core/src/channels/websocket/index.ts#L60-L74、code/core/src/channels/websocket/index.ts#L81-L101
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：index.test.ts、index.test.ts#L268-L346、index.test.ts#L40-L346、index.ts、index.ts#L1-L60、index.ts#L19-L51、index.ts#L32-L51、main.ts#L1-L148

### 项目概述/技术架构/构建系统架构/Vite构建器.md

- reference 标题：Vite构建器
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.08
- key_source_coverage：0.03
- missing_key_sources：code/builders/builder-vite/input/iframe.html、code/builders/builder-vite/package.json、code/builders/builder-vite/preset.js、code/builders/builder-vite/preset.js#L1-L2、code/builders/builder-vite/src/build.ts、code/builders/builder-vite/src/build.ts#L1-L99、code/builders/builder-vite/src/build.ts#L17-L98、code/builders/builder-vite/src/build.ts#L46-L62、code/builders/builder-vite/src/build.ts#L77-L87、code/builders/builder-vite/src/codegen-modern-iframe-script.ts、code/builders/builder-vite/src/codegen-modern-iframe-script.ts#L1-L71、code/builders/builder-vite/src/codegen-modern-iframe-script.ts#L10-L71、code/builders/builder-vite/src/codegen-modern-iframe-script.ts#L23-L41、code/builders/builder-vite/src/index.ts、code/builders/builder-vite/src/index.ts#L1-L68、code/builders/builder-vite/src/index.ts#L19-L33、code/builders/builder-vite/src/index.ts#L19-L61、code/builders/builder-vite/src/index.ts#L41-L61、code/builders/builder-vite/src/plugins/csf-plugin.ts、code/builders/builder-vite/src/plugins/csf-plugin.ts#L1-L23、code/builders/builder-vite/src/plugins/csf-plugin.ts#L7-L22、code/builders/builder-vite/src/plugins/index.ts、code/builders/builder-vite/src/plugins/index.ts#L1-L13、code/builders/builder-vite/src/preset.ts#L1-L42、code/builders/builder-vite/src/preset.ts#L19-L41、code/builders/builder-vite/src/vite-config.ts、code/builders/builder-vite/src/vite-config.ts#L1-L94、code/builders/builder-vite/src/vite-config.ts#L38-L78、code/builders/builder-vite/src/vite-config.ts#L38-L93、code/builders/builder-vite/src/vite-config.ts#L80-L93、code/builders/builder-vite/src/vite-server.ts、code/builders/builder-vite/src/vite-server.ts#L1-L47、code/builders/builder-vite/src/vite-server.ts#L32-L38、code/builders/builder-vite/src/vite-server.ts#L9-L46
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：iframe.html、package.json、preset.js、preset.js#L1-L2、build.ts、build.ts#L1-L99、build.ts#L17-L98、build.ts#L46-L62

### 项目概述/技术架构/构建系统架构/Webpack构建器.md

- reference 标题：Webpack构建器
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.15
- key_source_coverage：0.00
- missing_key_sources：code/builders/builder-webpack5/package.json、code/builders/builder-webpack5/package.json#L1-L93、code/builders/builder-webpack5/package.json#L54-L69、code/builders/builder-webpack5/src/index.ts、code/builders/builder-webpack5/src/index.ts#L1-L349、code/builders/builder-webpack5/src/index.ts#L121-L227、code/builders/builder-webpack5/src/index.ts#L132-L134、code/builders/builder-webpack5/src/index.ts#L139-L144、code/builders/builder-webpack5/src/index.ts#L147-L179、code/builders/builder-webpack5/src/index.ts#L208-L220、code/builders/builder-webpack5/src/index.ts#L251-L319、code/builders/builder-webpack5/src/index.ts#L277-L291、code/builders/builder-webpack5/src/index.ts#L304-L314、code/builders/builder-webpack5/src/index.ts#L57-L113、code/builders/builder-webpack5/src/index.ts#L57-L66、code/builders/builder-webpack5/src/preview/virtual-module-mapping.ts、code/builders/builder-webpack5/src/preview/virtual-module-mapping.ts#L1-L85、code/builders/builder-webpack5/src/preview/virtual-module-mapping.ts#L19-L84、code/builders/builder-webpack5/src/preview/virtual-module-mapping.ts#L51-L52、code/builders/builder-webpack5/src/types.ts、code/builders/builder-webpack5/src/types.ts#L1-L53、code/builders/builder-webpack5/src/types.ts#L11-L52、code/builders/builder-webpack5/templates/preview.ejs、code/builders/builder-webpack5/templates/preview.ejs#L1-L83、code/builders/builder-webpack5/templates/virtualModuleModernEntry.js、code/builders/builder-webpack5/templates/virtualModuleModernEntry.js#L1-L50
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L93、package.json#L54-L69、index.ts、index.ts#L1-L349、index.ts#L121-L227、index.ts#L132-L134、index.ts#L139-L144

### 项目概述/技术架构/构建系统架构/构建器架构设计.md

- reference 标题：构建器架构设计
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.07
- key_source_coverage：0.00
- missing_key_sources：code/builders/builder-vite/package.json、code/builders/builder-vite/package.json#L1-L73、code/builders/builder-vite/src/build.ts、code/builders/builder-vite/src/build.ts#L1-L99、code/builders/builder-vite/src/index.ts、code/builders/builder-vite/src/index.ts#L1-L68、code/builders/builder-vite/src/index.ts#L19-L33、code/builders/builder-vite/src/index.ts#L41-L65、code/builders/builder-vite/src/types.ts、code/builders/builder-vite/src/types.ts#L1-L25、code/builders/builder-vite/src/types.ts#L10、code/builders/builder-webpack5/package.json、code/builders/builder-webpack5/package.json#L1-L93、code/builders/builder-webpack5/src/index.ts、code/builders/builder-webpack5/src/index.ts#L1-L349、code/builders/builder-webpack5/src/index.ts#L121-L227、code/builders/builder-webpack5/src/index.ts#L129-L243、code/builders/builder-webpack5/src/index.ts#L251-L319、code/builders/builder-webpack5/src/index.ts#L86-L113、code/builders/builder-webpack5/src/types.ts、code/builders/builder-webpack5/src/types.ts#L1-L53、code/core/src/common/utils/framework.ts、code/core/src/common/utils/framework.ts#L41-L64、code/core/src/core-server/build-static.ts、code/core/src/core-server/build-static.ts#L70-L112、code/core/src/core-server/load.ts、code/core/src/core-server/load.ts#L46-L80、code/core/src/core-server/load.ts#L65-L78、code/core/src/types/modules/core-common.ts、code/core/src/types/modules/core-common.ts#L255-L279、code/lib/create-storybook/src/services/FrameworkDetectionService.ts、code/lib/create-storybook/src/services/FrameworkDetectionService.ts#L30-L78、docs/_snippets/storybook-builder-api-interface.md、docs/_snippets/storybook-builder-api-interface.md#L1-L24、docs/builders/builder-api.mdx、docs/builders/builder-api.mdx#L30-L38
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L73、build.ts、build.ts#L1-L99、index.ts、index.ts#L1-L68、index.ts#L19-L33、index.ts#L41-L65

### 项目概述/技术架构/构建系统架构/构建系统架构.md

- reference 标题：构建系统架构
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.17
- key_source_coverage：0.00
- missing_key_sources：code/builders/builder-vite/package.json、code/builders/builder-vite/package.json#L1-L73、code/builders/builder-vite/package.json#L49-L67、code/builders/builder-vite/src/build.ts、code/builders/builder-vite/src/build.ts#L1-L99、code/builders/builder-vite/src/build.ts#L17-L98、code/builders/builder-vite/src/build.ts#L81-L87、code/builders/builder-vite/src/index.ts、code/builders/builder-vite/src/index.ts#L1-L68、code/builders/builder-vite/src/index.ts#L19-L67、code/builders/builder-vite/src/index.ts#L41-L67、code/builders/builder-vite/src/index.ts#L54-L58、code/builders/builder-vite/src/index.ts#L67-L68、code/builders/builder-vite/src/plugins/index.ts、code/builders/builder-vite/src/plugins/index.ts#L1-L13、code/builders/builder-vite/src/types.ts、code/builders/builder-vite/src/types.ts#L10-L24、code/builders/builder-vite/src/utils/has-vite-plugins.ts、code/builders/builder-vite/src/utils/has-vite-plugins.ts#L1-L26、code/builders/builder-vite/src/utils/has-vite-plugins.ts#L13-L25、code/builders/builder-vite/src/utils/without-vite-plugins.ts、code/builders/builder-vite/src/utils/without-vite-plugins.ts#L1-L24、code/builders/builder-vite/src/utils/without-vite-plugins.ts#L2-L23、code/builders/builder-vite/src/vite-config.ts、code/builders/builder-vite/src/vite-config.ts#L1-L94、code/builders/builder-vite/src/vite-config.ts#L38-L78、code/builders/builder-vite/src/vite-config.ts#L38-L93、code/builders/builder-vite/src/vite-server.ts、code/builders/builder-vite/src/vite-server.ts#L1-L47、code/builders/builder-vite/src/vite-server.ts#L9-L46、code/builders/builder-webpack5/package.json、code/builders/builder-webpack5/package.json#L1-L93、code/builders/builder-webpack5/package.json#L54-L69、code/builders/builder-webpack5/src/index.ts、code/builders/builder-webpack5/src/index.ts#L1-L349、code/builders/builder-webpack5/src/index.ts#L121-L227、code/builders/builder-webpack5/src/index.ts#L132-L134、code/builders/builder-webpack5/src/index.ts#L208-L221、code/builders/builder-webpack5/src/index.ts#L251-L319、code/builders/builder-webpack5/src/index.ts#L321-L341、code/builders/builder-webpack5/src/index.ts#L343-L348、code/builders/builder-webpack5/src/index.ts#L57-L113、code/builders/builder-webpack5/src/index.ts#L68-L82、code/builders/builder-webpack5/src/preview/virtual-module-mapping.ts、code/builders/builder-webpack5/src/preview/virtual-module-mapping.ts#L1-L85、code/builders/builder-webpack5/src/preview/virtual-module-mapping.ts#L19-L84、code/builders/builder-webpack5/src/types.ts、code/builders/builder-webpack5/src/types.ts#L19-L52
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L73、package.json#L49-L67、build.ts、build.ts#L1-L99、build.ts#L17-L98、build.ts#L81-L87、index.ts

### 项目概述/技术架构/核心引擎架构/CLI分发器.md

- reference 标题：CLI分发器
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.00
- missing_key_sources：code/core/src/cli/build.ts、code/core/src/cli/build.ts#L1-L21、code/core/src/cli/build.ts#L4-L20、code/core/src/cli/buildIndex.ts、code/core/src/cli/buildIndex.ts#L1-L31、code/core/src/cli/buildIndex.ts#L11-L30、code/core/src/cli/detect.ts、code/core/src/cli/detect.ts#L1-L7、code/core/src/cli/detect.ts#L4-L6、code/core/src/cli/dev.ts、code/core/src/cli/dev.ts#L1-L65、code/core/src/cli/dev.ts#L37-L64、code/core/src/cli/dev.ts#L8-L35、code/core/src/cli/globalSettings.ts、code/core/src/cli/globalSettings.ts#L1-L120、code/core/src/cli/globalSettings.ts#L114-L118、code/core/src/cli/globalSettings.ts#L61-L119、code/core/src/cli/helpers.ts、code/core/src/cli/helpers.ts#L1-L253、code/core/src/cli/helpers.ts#L25-L253、code/core/src/cli/helpers.ts#L25-L50、code/core/src/cli/index.ts、code/core/src/cli/index.ts#L1-L10、code/lib/cli-storybook/src/upgrade.ts、code/lib/cli-storybook/src/upgrade.ts#L321-L537
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：build.ts、build.ts#L1-L21、build.ts#L4-L20、buildIndex.ts、buildIndex.ts#L1-L31、buildIndex.ts#L11-L30、detect.ts、detect.ts#L1-L7

### 项目概述/技术架构/核心引擎架构/全局设置.md

- reference 标题：全局设置
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.14
- key_source_coverage：0.03
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L19-L183、code/core/src/cli/NpmOptions.ts、code/core/src/cli/NpmOptions.ts#L1-L4、code/core/src/cli/globalSettings.test.ts、code/core/src/cli/globalSettings.test.ts#L1-L107、code/core/src/cli/globalSettings.test.ts#L31-L67、code/core/src/cli/globalSettings.ts、code/core/src/cli/globalSettings.ts#L1-L120、code/core/src/cli/globalSettings.ts#L108-L119、code/core/src/cli/globalSettings.ts#L114-L117、code/core/src/cli/globalSettings.ts#L21-L58、code/core/src/cli/globalSettings.ts#L61-L80、code/core/src/cli/globalSettings.ts#L66-L77、code/core/src/cli/globalSettings.ts#L91-L119、code/core/src/common/js-package-manager/index.ts、code/core/src/common/js-package-manager/index.ts#L1-L4、code/core/src/common/utils/get-storybook-info.ts、code/core/src/common/utils/get-storybook-info.ts#L105-L137、code/core/src/common/utils/get-storybook-info.ts#L111-L137、code/core/src/common/utils/load-manager-or-addons-file.ts、code/core/src/common/utils/load-manager-or-addons-file.ts#L1-L26、code/core/src/common/utils/load-manager-or-addons-file.ts#L17-L23、code/core/src/common/utils/load-manager-or-addons-file.ts#L9-L26、code/lib/cli-storybook/src/automigrate/helpers/mainConfigFile.ts、code/lib/cli-storybook/src/automigrate/helpers/mainConfigFile.ts#L142-L146、code/lib/cli-storybook/src/util.ts、code/lib/cli-storybook/src/util.ts#L793-L822、code/package.json、code/package.json#L1-L219
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L19-L183、NpmOptions.ts、NpmOptions.ts#L1-L4、globalSettings.test.ts、globalSettings.test.ts#L1-L107、globalSettings.test.ts#L31-L67、globalSettings.ts

### 项目概述/技术架构/核心引擎架构/核心引擎架构.md

- reference 标题：核心引擎架构
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.08
- key_source_coverage：0.00
- missing_key_sources：code/core/README.md、code/core/README.md#L1-L39、code/core/build-config.ts、code/core/build-config.ts#L1-L214、code/core/build-config.ts#L22-L210、code/core/build-config.ts#L33-L44、code/core/build-config.ts#L58-L83、code/core/package.json、code/core/package.json#L1-L389、code/core/package.json#L101-L105、code/core/package.json#L3-L389、code/core/package.json#L69-L105、code/core/tsconfig.json、code/core/tsconfig.json#L1-L11、code/core/tsconfig.json#L3-L8
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：README.md、README.md#L1-L39、build-config.ts、build-config.ts#L1-L214、build-config.ts#L22-L210、build-config.ts#L33-L44、build-config.ts#L58-L83、package.json

### 项目概述/技术架构/核心引擎架构/核心服务器.md

- reference 标题：核心服务器
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.15
- key_source_coverage：0.00
- missing_key_sources：code/builders/builder-vite/src/preview.ts、code/builders/builder-vite/src/preview.ts#L1-L120、code/builders/builder-webpack5/src/preview.ts、code/builders/builder-webpack5/src/preview.ts#L1-L120、code/core/build-config.ts、code/core/build-config.ts#L1-L56、code/core/src/builder-manager/index.ts、code/core/src/builder-manager/index.ts#L200-L320、code/core/src/builder-manager/index.ts#L240-L296、code/core/src/cli/build.ts、code/core/src/cli/build.ts#L1-L200、code/core/src/cli/dev.ts、code/core/src/cli/dev.ts#L1-L200、code/core/src/core-server/index.ts、code/core/src/core-server/index.ts#L1-L200、code/core/src/core-server/utils/server-address.ts、code/core/src/core-server/utils/server-address.ts#L1-L120、code/core/src/core-server/utils/server-errors.ts、code/core/src/core-server/utils/server-errors.ts#L1-L120、code/core/src/core-server/utils/server-statics.ts、code/core/src/core-server/utils/server-statics.ts#L1-L200、code/core/src/core-server/utils/server-statics.ts#L115-L168、code/core/src/core-server/utils/server-statics.ts#L170-L196、code/core/src/core-server/utils/server-statics.ts#L181-L188、code/frameworks/react-webpack5/src/preview.tsx、code/frameworks/react-webpack5/src/preview.tsx#L1-L120、code/presets/server-webpack/src/index.ts、code/presets/server-webpack/src/index.ts#L1-L120、code/renderers/react/src/client/index.tsx、code/renderers/react/src/client/index.tsx#L1-L120、docs/_snippets/storybook-builder-api-dev-server.md、docs/_snippets/storybook-builder-api-dev-server.md#L1-L21、docs/_snippets/storybook-builder-api-interface.md、docs/_snippets/storybook-builder-api-interface.md#L1-L24、docs/_snippets/storybook-builder-api-shutdown-server.md、docs/_snippets/storybook-builder-api-shutdown-server.md#L1-L19、docs/builders/webpack.mdx、docs/builders/webpack.mdx#L14-L25
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：preview.ts、preview.ts#L1-L120、preview.ts、preview.ts#L1-L120、build-config.ts、build-config.ts#L1-L56、index.ts、index.ts#L200-L320

### 项目概述/技术架构/核心引擎架构/版本管理.md

- reference 标题：版本管理
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.03
- missing_key_sources：CONTRIBUTING/RELEASING.md、CONTRIBUTING/RELEASING.md#L100-L133、CONTRIBUTING/RELEASING.md#L214-L230、CONTRIBUTING/RELEASING.md#L320-L323、CONTRIBUTING/RELEASING.md#L324-L332、CONTRIBUTING/RELEASING.md#L591-L596、CONTRIBUTING/RELEASING.md#L63-L98、MIGRATION.md#L776-L799、RESOLUTIONS.md、RESOLUTIONS.md#L1-L8、code/core/src/common/js-package-manager/JsPackageManager.ts、code/core/src/common/js-package-manager/JsPackageManager.ts#L513-L679、code/core/src/common/js-package-manager/JsPackageManagerFactory.test.ts、code/core/src/common/js-package-manager/JsPackageManagerFactory.test.ts#L123-L316、code/core/src/common/js-package-manager/Yarn1Proxy.ts、code/core/src/common/js-package-manager/Yarn1Proxy.ts#L101-L143、code/core/src/common/js-package-manager/Yarn2Proxy.ts、code/core/src/common/js-package-manager/Yarn2Proxy.ts#L80-L135、code/lib/cli-storybook/src/doctor/hasMultipleVersions.ts、code/lib/cli-storybook/src/doctor/hasMultipleVersions.ts#L1-L25、code/lib/create-storybook/src/dependency-collector.test.ts、code/lib/create-storybook/src/dependency-collector.test.ts#L92-L129、code/lib/create-storybook/src/dependency-collector.ts、code/lib/create-storybook/src/dependency-collector.ts#L50-L89、docs/versions/latest.json、docs/versions/latest.json#L1-L1、docs/versions/next.json、docs/versions/next.json#L1-L1、package.json、package.json#L1-L74
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：RELEASING.md、RELEASING.md#L100-L133、RELEASING.md#L214-L230、RELEASING.md#L320-L323、RELEASING.md#L324-L332、RELEASING.md#L591-L596、RELEASING.md#L63-L98、MIGRATION.md#L776-L799

### 项目概述/技术架构/框架适配器架构/Angular框架适配器.md

- reference 标题：Angular框架适配器
- 生成页：多框架支持/多框架支持.md（多框架支持）
- KnowledgeUnit：unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- reuse_count：14
- skeleton_score：0.05
- key_source_coverage：0.21
- missing_key_sources：code/frameworks/angular/README.md、code/frameworks/angular/build-config.ts、code/frameworks/angular/build-config.ts#L1-L66、code/frameworks/angular/package.json、code/frameworks/angular/package.json#L1-L124、code/frameworks/angular/package.json#L61-L106、code/frameworks/angular/project.json、code/frameworks/angular/project.json#L1-L10、code/frameworks/angular/src/builders/utils/error-handler.ts、code/frameworks/angular/src/builders/utils/run-compodoc.ts、code/frameworks/angular/src/builders/utils/standalone-options.ts、code/frameworks/angular/src/client/index.ts#L1-L11、code/frameworks/angular/src/index.ts#L1-L17、code/frameworks/angular/src/node/index.ts#L1-L8、code/frameworks/angular/src/preset.ts#L1-L54、code/frameworks/angular/src/server/framework-preset-angular-cli.ts、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L1-L173、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L165-L173、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L17-L72、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts#L1-L72、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts#L26-L72
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：README.md、build-config.ts、build-config.ts#L1-L66、package.json、package.json#L1-L124、package.json#L61-L106、project.json、project.json#L1-L10

### 项目概述/技术架构/框架适配器架构/HTML框架适配器.md

- reference 标题：HTML框架适配器
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.00
- missing_key_sources：code/builders/builder-vite/project.json、code/builders/builder-vite/src/index.ts、code/builders/builder-vite/src/index.ts#L1-L47、code/builders/builder-vite/src/index.ts#L19-L33、code/builders/builder-vite/src/index.ts#L41-L47、code/core/src/builder-manager/index.ts、code/core/src/builder-manager/index.ts#L166-L238、code/core/src/builder-manager/index.ts#L205-L219、code/core/src/common/utils/framework.ts、code/core/src/common/utils/framework.ts#L41-L64、code/core/src/common/utils/interpolate.ts、code/core/src/common/utils/interpolate.ts#L1-L15、code/core/src/common/versions.ts、code/core/src/common/versions.ts#L14-L14、code/core/src/types/modules/frameworks.ts、code/core/src/types/modules/frameworks.ts#L5-L5、code/frameworks/html-vite/package.json、code/frameworks/html-vite/package.json#L1-L66、code/frameworks/html-vite/package.json#L50-L60、code/frameworks/html-vite/src/index.ts、code/frameworks/html-vite/src/index.ts#L1-L3、code/frameworks/html-vite/src/types.ts、code/frameworks/html-vite/src/types.ts#L1-L39、code/frameworks/html-vite/src/types.ts#L11-L39、code/frameworks/html-vite/src/types.ts#L15-L39、code/frameworks/html-vite/template/cli/ts/Button.stories.ts、code/frameworks/html-vite/template/cli/ts/Button.stories.ts#L0-L0、code/frameworks/html-vite/template/cli/ts/Header.stories.ts、code/frameworks/html-vite/template/cli/ts/Header.stories.ts#L0-L0、code/frameworks/html-vite/template/cli/ts/Page.stories.ts、code/frameworks/html-vite/template/cli/ts/Page.stories.ts#L0-L0、code/lib/cli-storybook/src/sandbox-templates.ts、code/lib/create-storybook/src/bin/modernInputs.ts、code/lib/eslint-plugin/src/rules/no-renderer-packages.ts、code/lib/eslint-plugin/src/rules/no-renderer-packages.ts#L19-L19、code/renderers/html/template/components/Html.js、code/renderers/html/template/components/Html.js#L1-L1
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：project.json、index.ts、index.ts#L1-L47、index.ts#L19-L33、index.ts#L41-L47、index.ts、index.ts#L166-L238、index.ts#L205-L219

### 项目概述/技术架构/框架适配器架构/React框架适配器.md

- reference 标题：React框架适配器
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.10
- key_source_coverage：0.07
- missing_key_sources：code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L169-L173、code/.storybook/main.ts#L174-L179、code/frameworks/react-vite/package.json、code/frameworks/react-vite/package.json#L1-L81、code/frameworks/react-vite/package.json#L54-L75、code/frameworks/react-vite/package.json#L70-L75、code/frameworks/react-vite/src/index.ts、code/frameworks/react-vite/src/index.ts#L1-L5、code/frameworks/react-vite/src/preset.ts#L1-L51、code/frameworks/react-vite/src/preset.ts#L10-L50、code/frameworks/react-vite/src/preset.ts#L5-L8、code/frameworks/react-vite/src/types.ts、code/frameworks/react-vite/src/types.ts#L1-L74、code/frameworks/react-vite/src/types.ts#L14-L27、code/frameworks/react-vite/src/types.ts#L14-L74、code/renderers/react/src/index.ts、code/renderers/react/src/preview.tsx、code/renderers/react/src/preview.tsx#L1-L255、code/renderers/react/src/preview.tsx#L101-L138、code/renderers/react/src/preview.tsx#L55-L85、code/renderers/react/src/public-types.ts、code/renderers/react/src/public-types.ts#L1-L81、code/renderers/react/src/types.ts、code/renderers/react/src/types.ts#L8-L38
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L149-L181、main.ts#L169-L173、main.ts#L174-L179、package.json、package.json#L1-L81、package.json#L54-L75、package.json#L70-L75、index.ts

### 项目概述/技术架构/框架适配器架构/Svelte框架适配器.md

- reference 标题：Svelte框架适配器
- 生成页：多框架支持/多框架支持.md（多框架支持）
- KnowledgeUnit：unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- reuse_count：14
- skeleton_score：0.06
- key_source_coverage：0.10
- missing_key_sources：code/e2e-tests/framework-svelte.spec.ts、code/frameworks/svelte-vite/package.json、code/frameworks/svelte-vite/package.json#L1-L80、code/frameworks/svelte-vite/package.json#L54-L68、code/frameworks/svelte-vite/package.json#L54-L74、code/frameworks/svelte-vite/src/plugins/generateDocgen.ts、code/frameworks/svelte-vite/src/plugins/generateDocgen.ts#L1-L527、code/frameworks/svelte-vite/src/plugins/generateDocgen.ts#L259-L526、code/frameworks/svelte-vite/src/plugins/generateDocgen.ts#L331-L343、code/frameworks/svelte-vite/src/plugins/generateDocgen.ts#L408-L526、code/frameworks/svelte-vite/src/plugins/generateDocgen.ts#L450-L458、code/frameworks/svelte-vite/src/plugins/generateDocgen.ts#L51-L58、code/frameworks/svelte-vite/src/plugins/generateDocgen.ts#L7-L62、code/frameworks/svelte-vite/src/plugins/svelte-docgen.ts#L1-L147、code/frameworks/svelte-vite/src/plugins/svelte-docgen.ts#L108-L146、code/frameworks/svelte-vite/src/plugins/svelte-docgen.ts#L117-L144、code/frameworks/svelte-vite/src/preset.ts#L1-L32、code/frameworks/svelte-vite/src/preset.ts#L12-L31、code/frameworks/svelte-vite/src/preset.ts#L21-L23、code/frameworks/svelte-vite/src/preset.ts#L7-L31、code/frameworks/svelte-vite/src/types.ts、code/frameworks/svelte-vite/src/types.ts#L1-L47、code/frameworks/svelte-vite/src/types.ts#L11-L21、code/frameworks/svelte-vite/src/utils.ts、code/frameworks/svelte-vite/src/utils.ts#L1-L32、code/frameworks/svelte-vite/src/utils.ts#L13-L31、code/frameworks/svelte-vite/src/utils.ts#L28-L30、code/renderers/svelte/package.json、code/renderers/svelte/package.json#L1-L77、code/renderers/svelte/package.json#L55-L66、code/renderers/svelte/package.json#L55-L71、code/renderers/svelte/src/index.ts#L1-L5、code/renderers/svelte/src/portable-stories.ts、code/renderers/svelte/src/portable-stories.ts#L1-L201、code/renderers/svelte/src/portable-stories.ts#L118-L160、code/renderers/svelte/src/public-types.ts、code/renderers/svelte/src/public-types.ts#L1-L71、code/renderers/svelte/src/public-types.ts#L27-L71
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：framework-svelte.spec.ts、package.json、package.json#L1-L80、package.json#L54-L68、package.json#L54-L74、generateDocgen.ts、generateDocgen.ts#L1-L527、generateDocgen.ts#L259-L526

### 项目概述/技术架构/框架适配器架构/Vue 3框架适配器.md

- reference 标题：Vue 3框架适配器
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.12
- key_source_coverage：0.03
- missing_key_sources：code/frameworks/vue3-vite/package.json、code/frameworks/vue3-vite/package.json#L1-L73、code/frameworks/vue3-vite/package.json#L51-L67、code/frameworks/vue3-vite/src/index.ts、code/frameworks/vue3-vite/src/index.ts#L1-L5、code/frameworks/vue3-vite/src/plugins/vue-component-meta.ts、code/frameworks/vue3-vite/src/plugins/vue-component-meta.ts#L1-L305、code/frameworks/vue3-vite/src/plugins/vue-component-meta.ts#L172-L199、code/frameworks/vue3-vite/src/plugins/vue-docgen.ts、code/frameworks/vue3-vite/src/plugins/vue-docgen.ts#L1-L33、code/frameworks/vue3-vite/src/plugins/vue-template.ts、code/frameworks/vue3-vite/src/plugins/vue-template.ts#L3-L14、code/frameworks/vue3-vite/src/preset.ts#L1-L57、code/frameworks/vue3-vite/src/preset.ts#L15-L37、code/frameworks/vue3-vite/src/types.ts、code/frameworks/vue3-vite/src/types.ts#L1-L105、code/frameworks/vue3-vite/src/types.ts#L14-L48、code/frameworks/vue3-vite/src/types.ts#L50-L73、code/frameworks/vue3-vite/src/vite-plugin.ts、code/frameworks/vue3-vite/src/vite-plugin.ts#L1-L8、code/renderers/vue3/package.json、code/renderers/vue3/package.json#L1-L72、code/renderers/vue3/package.json#L51-L66、code/renderers/vue3/src/index.ts、code/renderers/vue3/src/preview.ts、code/renderers/vue3/src/public-types.ts、code/renderers/vue3/src/public-types.ts#L1-L84、code/renderers/vue3/src/render.ts、code/renderers/vue3/src/render.ts#L1-L178、code/renderers/vue3/src/render.ts#L45-L114、code/renderers/vue3/src/types.ts、code/renderers/vue3/src/types.ts#L1-L38、code/renderers/vue3/src/types.ts#L24-L38
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L73、package.json#L51-L67、index.ts、index.ts#L1-L5、vue-component-meta.ts、vue-component-meta.ts#L1-L305、vue-component-meta.ts#L172-L199

### 项目概述/技术架构/框架适配器架构/Web Components框架适配器.md

- reference 标题：Web Components框架适配器
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.08
- key_source_coverage：0.00
- missing_key_sources：code/frameworks/web-components-vite/package.json、code/frameworks/web-components-vite/package.json#L1-L68、code/frameworks/web-components-vite/package.json#L52-L62、code/frameworks/web-components-vite/src/index.ts、code/frameworks/web-components-vite/src/index.ts#L1-L5、code/frameworks/web-components-vite/src/node/index.ts、code/frameworks/web-components-vite/src/node/index.ts#L1-L8、code/frameworks/web-components-vite/src/types.ts、code/frameworks/web-components-vite/src/types.ts#L1-L39、code/renderers/web-components/package.json、code/renderers/web-components/package.json#L1-L73、code/renderers/web-components/package.json#L49-L67、code/renderers/web-components/src/framework-api.ts、code/renderers/web-components/src/framework-api.ts#L1-L40、code/renderers/web-components/src/framework-api.ts#L13-L26、code/renderers/web-components/src/framework-api.ts#L3-L11、code/renderers/web-components/src/globals.ts、code/renderers/web-components/src/index.ts、code/renderers/web-components/src/index.ts#L1-L36、code/renderers/web-components/src/index.ts#L14-L35、code/renderers/web-components/src/preview.ts、code/renderers/web-components/src/preview.ts#L1-L268、code/renderers/web-components/src/preview.ts#L44-L53、code/renderers/web-components/src/preview.ts#L85-L146、code/renderers/web-components/src/public-types.ts、code/renderers/web-components/src/public-types.ts#L1-L43、code/renderers/web-components/src/types.ts、code/renderers/web-components/src/types.ts#L1-L25、code/renderers/web-components/src/types.ts#L14-L17
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L68、package.json#L52-L62、index.ts、index.ts#L1-L5、index.ts、index.ts#L1-L8、types.ts

### 项目概述/技术架构/框架适配器架构/框架适配器架构.md

- reference 标题：框架适配器架构
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.07
- key_source_coverage：0.06
- missing_key_sources：code/frameworks/angular/package.json、code/frameworks/angular/package.json#L1-L124、code/frameworks/angular/package.json#L61-L67、code/frameworks/angular/package.json#L90-L106、code/frameworks/angular/src/client/index.ts、code/frameworks/angular/src/client/index.ts#L1-L11、code/frameworks/angular/src/index.ts、code/frameworks/angular/src/index.ts#L1-L17、code/frameworks/nextjs/package.json、code/frameworks/nextjs/package.json#L1-L157、code/frameworks/nextjs/package.json#L105-L123、code/frameworks/nextjs/package.json#L34-L81、code/frameworks/react-vite/package.json、code/frameworks/react-vite/package.json#L1-L81、code/frameworks/react-vite/package.json#L54-L64、code/frameworks/react-vite/src/index.ts、code/frameworks/react-vite/src/index.ts#L1-L5、code/frameworks/react-vite/src/node/index.ts、code/frameworks/react-vite/src/node/index.ts#L1-L8、code/frameworks/react-vite/src/plugins/react-docgen.ts、code/frameworks/react-vite/src/plugins/react-docgen.ts#L1-L132、code/frameworks/react-vite/src/plugins/react-docgen.ts#L40-L99、code/frameworks/react-vite/src/plugins/react-docgen.ts#L90-L96、code/frameworks/react-vite/src/preset.ts#L1-L51、code/frameworks/react-vite/src/preset.ts#L10-L50、code/frameworks/react-vite/src/preset.ts#L5-L50、code/frameworks/react-vite/src/types.ts、code/frameworks/react-vite/src/types.ts#L1-L74、code/frameworks/react-vite/src/types.ts#L14-L74、code/frameworks/react-vite/src/types.ts#L66-L74、code/frameworks/react-webpack5/package.json、code/frameworks/react-webpack5/package.json#L1-L74、code/frameworks/react-webpack5/package.json#L50-L54、code/frameworks/svelte-vite/package.json、code/frameworks/svelte-vite/package.json#L1-L80、code/frameworks/svelte-vite/package.json#L54-L59、code/frameworks/svelte-vite/src/index.ts、code/frameworks/svelte-vite/src/preset.ts#L1-L32、code/frameworks/svelte-vite/src/preset.ts#L12-L31、code/frameworks/svelte-vite/src/preset.ts#L25-L26、code/frameworks/svelte-vite/src/preset.ts#L7-L31、code/frameworks/vue3-vite/package.json、code/frameworks/vue3-vite/package.json#L1-L73、code/frameworks/vue3-vite/package.json#L51-L58、code/frameworks/vue3-vite/src/index.ts、code/frameworks/vue3-vite/src/preset.ts#L1-L57、code/frameworks/vue3-vite/src/preset.ts#L10-L37、code/frameworks/vue3-vite/src/preset.ts#L15-L57
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L124、package.json#L61-L67、package.json#L90-L106、index.ts、index.ts#L1-L11、index.ts、index.ts#L1-L17

### 项目概述/技术架构/渲染系统架构/HTML渲染器.md

- reference 标题：HTML渲染器
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.00
- missing_key_sources：code/renderers/html/package.json、code/renderers/html/package.json#L1-L61、code/renderers/html/package.json#L27-L49、code/renderers/html/src/entry-preview.ts、code/renderers/html/src/entry-preview.ts#L1-L11、code/renderers/html/src/entry-preview.ts#L1-L2、code/renderers/html/src/entry-preview.ts#L8-L10、code/renderers/html/src/globals.ts、code/renderers/html/src/globals.ts#L1-L6、code/renderers/html/src/index.ts、code/renderers/html/src/index.ts#L1-L5、code/renderers/html/src/portable-stories.ts、code/renderers/html/src/portable-stories.ts#L1-L43、code/renderers/html/src/portable-stories.ts#L33-L42、code/renderers/html/src/public-types.ts、code/renderers/html/src/public-types.ts#L1-L43、code/renderers/html/src/public-types.ts#L18-L42、code/renderers/html/src/public-types.ts#L23-L42、code/renderers/html/src/render.ts、code/renderers/html/src/render.ts#L1-L11、code/renderers/html/src/render.ts#L1-L70、code/renderers/html/src/render.ts#L12-L42、code/renderers/html/src/render.ts#L12-L70、code/renderers/html/src/render.ts#L3-L6、code/renderers/html/src/render.ts#L37-L42、code/renderers/html/src/render.ts#L44-L70、code/renderers/html/src/render.ts#L62-L69、code/renderers/html/src/types.ts、code/renderers/html/src/types.ts#L1-L38、code/renderers/html/src/types.ts#L17-L37
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L61、package.json#L27-L49、entry-preview.ts、entry-preview.ts#L1-L11、entry-preview.ts#L1-L2、entry-preview.ts#L8-L10、globals.ts

### 项目概述/技术架构/渲染系统架构/React渲染器.md

- reference 标题：React渲染器
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.15
- key_source_coverage：0.11
- missing_key_sources：code/addons/docs/src/DocsRenderer.tsx、code/addons/docs/src/DocsRenderer.tsx#L10-L10、code/addons/docs/src/preset.ts#L155-L155、code/core/src/common/versions.ts、code/core/src/common/versions.ts#L33-L33、code/lib/react-dom-shim/package.json、code/lib/react-dom-shim/package.json#L44-L48、code/lib/react-dom-shim/src/react-18.tsx、code/lib/react-dom-shim/src/react-18.tsx#L18-L33、code/lib/react-dom-shim/src/react-18.tsx#L48-L60、code/lib/react-dom-shim/src/react-18.tsx#L62-L69、code/lib/react-dom-shim/src/react-18.tsx#L7-L81、code/renderers/react/package.json、code/renderers/react/src/preset.ts#L25-L25、code/renderers/react/src/renderToCanvas.tsx、code/renderers/react/src/renderToCanvas.tsx#L74-L74
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：DocsRenderer.tsx、DocsRenderer.tsx#L10-L10、preset.ts#L155-L155、versions.ts、versions.ts#L33-L33、package.json、package.json#L44-L48、react-18.tsx

### 项目概述/技术架构/渲染系统架构/Svelte渲染器.md

- reference 标题：Svelte渲染器
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.14
- key_source_coverage：0.05
- missing_key_sources：code/e2e-tests/framework-svelte.spec.ts、code/renderers/svelte/package.json、code/renderers/svelte/package.json#L1-L77、code/renderers/svelte/package.json#L26-L44、code/renderers/svelte/src/globals.ts、code/renderers/svelte/src/globals.ts#L1-L2、code/renderers/svelte/src/index.ts、code/renderers/svelte/src/index.ts#L1-L5、code/renderers/svelte/src/mount.ts、code/renderers/svelte/src/mount.ts#L1-L16、code/renderers/svelte/src/portable-stories.ts、code/renderers/svelte/src/portable-stories.ts#L1-L201、code/renderers/svelte/src/preset.ts#L1-L21、code/renderers/svelte/src/public-types.ts、code/renderers/svelte/src/public-types.ts#L1-L71、code/renderers/svelte/src/render.ts、code/renderers/svelte/src/render.ts#L29-L114、code/renderers/svelte/src/types.ts、code/renderers/svelte/src/types.ts#L1-L35
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：framework-svelte.spec.ts、package.json、package.json#L1-L77、package.json#L26-L44、globals.ts、globals.ts#L1-L2、index.ts、index.ts#L1-L5

### 项目概述/技术架构/渲染系统架构/Vue3渲染器.md

- reference 标题：Vue3渲染器
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.08
- key_source_coverage：0.00
- missing_key_sources：code/renderers/vue3/package.json、code/renderers/vue3/package.json#L1-L72、code/renderers/vue3/package.json#L51-L66、code/renderers/vue3/src/entry-preview.ts、code/renderers/vue3/src/entry-preview.ts#L1-L21、code/renderers/vue3/src/entry-preview.ts#L1-L7、code/renderers/vue3/src/entry-preview.ts#L11-L21、code/renderers/vue3/src/entry-preview.ts#L7-L21、code/renderers/vue3/src/index.ts、code/renderers/vue3/src/index.ts#L1-L10、code/renderers/vue3/src/render.ts、code/renderers/vue3/src/render.ts#L1-L178、code/renderers/vue3/src/render.ts#L10-L114、code/renderers/vue3/src/render.ts#L10-L19、code/renderers/vue3/src/render.ts#L117-L124、code/renderers/vue3/src/render.ts#L12-L16、code/renderers/vue3/src/render.ts#L144-L157、code/renderers/vue3/src/render.ts#L168-L177、code/renderers/vue3/src/render.ts#L4-L8、code/renderers/vue3/src/render.ts#L45-L114、code/renderers/vue3/src/render.ts#L91-L106
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L72、package.json#L51-L66、entry-preview.ts、entry-preview.ts#L1-L21、entry-preview.ts#L1-L7、entry-preview.ts#L11-L21、entry-preview.ts#L7-L21

### 项目概述/技术架构/渲染系统架构/渲染系统架构.md

- reference 标题：渲染系统架构
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.08
- key_source_coverage：0.06
- missing_key_sources：code/.storybook/manager.tsx、code/.storybook/preview.tsx、code/core/package.json、code/core/package.json#L1-L389、code/core/package.json#L48-L221、code/core/src/manager/manager-stores.ts、code/core/src/manager/manager-stores.ts#L1-L11、code/core/src/preview-api/index.ts、code/core/src/preview-api/index.ts#L1-L90、code/core/src/preview/globals.ts、code/core/src/preview/globals.ts#L1-L2、code/core/src/preview/globals/globals.ts、code/core/src/preview/globals/globals.ts#L1-L23、code/renderers/react/src/index.ts、code/renderers/react/src/index.ts#L1-L10、code/renderers/vue3/src/index.ts、code/renderers/vue3/src/index.ts#L1-L10
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：manager.tsx、preview.tsx、package.json、package.json#L1-L389、package.json#L48-L221、manager-stores.ts、manager-stores.ts#L1-L11、index.ts

### 项目概述/核心特性/主题定制系统/主题切换机制.md

- reference 标题：主题切换机制
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.06
- key_source_coverage：0.04
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L133-L142、code/.storybook/preview.tsx#L220-L301、code/.storybook/preview.tsx#L341-L354、code/addons/themes/src/index.ts、code/addons/themes/src/index.ts#L1-L11、code/addons/themes/src/preview.ts、code/addons/themes/src/preview.ts#L1-L8、code/addons/themes/src/preview.ts#L5-L7、code/core/src/theming/create.ts、code/core/src/theming/create.ts#L1-L51、code/core/src/theming/create.ts#L18-L23、code/core/src/theming/create.ts#L29-L50、code/core/src/theming/index.ts、code/core/src/theming/index.ts#L1-L51、code/core/src/theming/utils.ts、code/core/src/theming/utils.ts#L1-L79、code/core/src/theming/utils.ts#L66-L79
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、manager.tsx、manager.tsx#L1-L9、manager.tsx#L4-L8、preview.tsx、preview.tsx#L1-L416、preview.tsx#L133-L142、preview.tsx#L220-L301

### 项目概述/核心特性/主题定制系统/主题定制系统.md

- reference 标题：主题定制系统
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.03
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L133-L142、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L195-L204、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L22-L30、code/.storybook/preview.tsx#L220-L301、code/.storybook/preview.tsx#L228-L236、code/.storybook/preview.tsx#L280-L299、code/.storybook/preview.tsx#L50-L92、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L1-L30、code/core/src/theming/create.ts、code/core/src/theming/create.ts#L1-L51、code/core/src/theming/create.ts#L18-L23、code/core/src/theming/create.ts#L2-L5、code/core/src/theming/create.ts#L29-L50、code/core/src/theming/index.ts、code/core/src/theming/index.ts#L1-L51、code/core/src/theming/index.ts#L10-L44、code/core/src/theming/index.ts#L36-L44、code/core/src/theming/index.ts#L43-L44、code/core/src/theming/types.ts、code/core/src/theming/types.ts#L1-L109、code/core/src/theming/types.ts#L24-L27、code/core/src/theming/types.ts#L71-L108、code/core/src/theming/types.ts#L78-L79、code/core/src/theming/types.ts#L8-L55、code/core/src/theming/types.ts#L90-L94
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、manager.tsx、manager.tsx#L1-L9、manager.tsx#L4-L8、preview.tsx、preview.tsx#L1-L416、preview.tsx#L133-L142、preview.tsx#L144-L193

### 项目概述/核心特性/主题定制系统/主题扩展开发.md

- reference 标题：主题扩展开发
- 生成页：主题系统/themes.md（themes）
- KnowledgeUnit：unit_id=unit-e858919ec1df, unit_type=module_doc, domain_id=domain-9cda34dbf62b, readiness=compose_ready, child_digests=0
- reuse_count：6
- skeleton_score：0.20
- key_source_coverage：0.18
- missing_key_sources：code/addons/themes/docs/api.md、code/addons/themes/docs/api.md#L1-L213、code/addons/themes/docs/api.md#L110-L129、code/addons/themes/docs/api.md#L149-L213、code/addons/themes/docs/getting-started/bootstrap.md、code/addons/themes/docs/getting-started/bootstrap.md#L1-L86、code/addons/themes/package.json、code/addons/themes/package.json#L1-L82、code/addons/themes/src/constants.ts、code/addons/themes/src/constants.ts#L1-L18、code/addons/themes/src/decorators/class-name.decorator.tsx#L1-L57、code/addons/themes/src/decorators/data-attribute.decorator.tsx、code/addons/themes/src/decorators/data-attribute.decorator.tsx#L1-L42、code/addons/themes/src/decorators/helpers.ts、code/addons/themes/src/decorators/helpers.ts#L1-L40、code/addons/themes/src/decorators/provider.decorator.tsx#L1-L64、code/addons/themes/src/index.ts#L1-L11、code/addons/themes/src/preview.ts#L1-L8
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：api.md、api.md#L1-L213、api.md#L110-L129、api.md#L149-L213、bootstrap.md、bootstrap.md#L1-L86、package.json、package.json#L1-L82

### 项目概述/核心特性/主题定制系统/字体系统.md

- reference 标题：字体系统
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.06
- key_source_coverage：0.00
- missing_key_sources：code/addons/themes/src/constants.ts、code/addons/themes/src/constants.ts#L1-L18、code/addons/themes/src/constants.ts#L3-L17、code/addons/themes/src/constants.ts#L3-L6、code/core/src/theming/base.ts、code/core/src/theming/base.ts#L59-L102、code/core/src/theming/global.ts、code/core/src/theming/global.ts#L1-L164、code/core/src/theming/global.ts#L101-L163、code/core/src/theming/global.ts#L25-L99、code/core/src/theming/global.ts#L32-L35、code/core/src/theming/global.ts#L42-L97、code/core/src/theming/global.ts#L74-L97、code/core/src/theming/themes/dark.ts、code/core/src/theming/themes/dark.ts#L1-L46、code/core/src/theming/themes/dark.ts#L19-L21、code/core/src/theming/themes/light.ts、code/core/src/theming/themes/light.ts#L1-L46、code/core/src/theming/themes/light.ts#L19-L21
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：constants.ts、constants.ts#L1-L18、constants.ts#L3-L17、constants.ts#L3-L6、base.ts、base.ts#L59-L102、global.ts、global.ts#L1-L164

### 项目概述/核心特性/主题定制系统/布局系统.md

- reference 标题：布局系统
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.14
- key_source_coverage：0.00
- missing_key_sources：code/.storybook/preview.tsx、code/.storybook/preview.tsx#L238-L274、code/addons/docs/docs/theming.md、code/addons/docs/docs/theming.md#L1-L98、code/addons/docs/docs/theming.md#L10-L98、code/addons/themes/src/theme-switcher.tsx、code/addons/themes/src/theme-switcher.tsx#L1-L113、code/addons/themes/src/theme-switcher.tsx#L24-L112、code/addons/themes/src/theme-switcher.tsx#L7-L16、code/core/src/components/components/spaced/Spaced.stories.tsx、code/core/src/components/components/spaced/Spaced.stories.tsx#L1-L64、code/core/src/components/components/spaced/Spaced.tsx、code/core/src/components/components/spaced/Spaced.tsx#L1-L71、code/core/src/components/components/spaced/Spaced.tsx#L13-L54、code/core/src/components/components/spaced/Spaced.tsx#L3-L4、code/core/src/components/components/spaced/Spaced.tsx#L56-L71、code/core/src/components/components/spaced/Spaced.tsx#L7-L54、code/core/src/manager-api/modules/layout.ts、code/core/src/manager-api/modules/layout.ts#L1-L557、code/core/src/manager-api/modules/layout.ts#L124-L148、code/core/src/manager-api/modules/layout.ts#L13-L14、code/core/src/manager-api/modules/layout.ts#L173-L185、code/core/src/manager-api/modules/layout.ts#L187-L556、code/core/src/manager-api/modules/layout.ts#L189-L345、code/core/src/manager-api/modules/layout.ts#L38-L120、code/core/src/manager-api/tests/layout.test.ts、code/core/src/manager-api/tests/layout.test.ts#L16-L202、code/core/src/manager-api/tests/layout.test.ts#L53-L202、code/core/src/manager-api/tests/layout.test.ts#L544-L680、code/core/src/measure/box-model/visualizer.ts、code/core/src/measure/box-model/visualizer.ts#L109-L165、code/core/src/measure/box-model/visualizer.ts#L109-L218、code/core/src/measure/box-model/visualizer.ts#L124-L218、code/core/src/measure/box-model/visualizer.ts#L167-L218、code/core/template/stories/layout.stories.ts、code/core/template/stories/layout.stories.ts#L1-L72、code/core/template/stories/layout.stories.ts#L14-L17、code/core/template/stories/layout.stories.ts#L19-L72
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：preview.tsx、preview.tsx#L238-L274、theming.md、theming.md#L1-L98、theming.md#L10-L98、theme-switcher.tsx、theme-switcher.tsx#L1-L113、theme-switcher.tsx#L24-L112

### 项目概述/核心特性/主题定制系统/颜色系统.md

- reference 标题：颜色系统
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.08
- key_source_coverage：0.00
- missing_key_sources：code/addons/themes/package.json、code/addons/themes/package.json#L1-L82、code/core/src/node-logger/logger/colors.ts、code/core/src/theming/themes/dark.ts、code/core/src/theming/themes/dark.ts#L1-L46、code/core/src/theming/themes/light.ts、code/core/src/theming/themes/light.ts#L1-L46
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L82、colors.ts、dark.ts、dark.ts#L1-L46、light.ts、light.ts#L1-L46

### 项目概述/核心特性/多框架支持/Angular框架支持.md

- reference 标题：Angular框架支持
- 生成页：API-参考/Angular框架支持.md（Angular框架支持）
- KnowledgeUnit：unit_id=unit-e74623e69693, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0
- reuse_count：2
- skeleton_score：0.11
- key_source_coverage：0.02
- missing_key_sources：code/frameworks/angular/package.json、code/frameworks/angular/package.json#L1-L124、code/frameworks/angular/package.json#L27-L47、code/frameworks/angular/package.json#L61-L88、code/frameworks/angular/package.json#L90-L106、code/frameworks/angular/src/client/config.ts、code/frameworks/angular/src/client/config.ts#L11-L18、code/frameworks/angular/src/client/config.ts#L11-L20、code/frameworks/angular/src/client/docs/config.ts、code/frameworks/angular/src/client/docs/config.ts#L1-L16、code/frameworks/angular/src/client/docs/config.ts#L6-L15、code/frameworks/angular/src/client/preview.ts、code/frameworks/angular/src/client/preview.ts#L175-L246、code/frameworks/angular/src/client/preview.ts#L175-L258、code/frameworks/angular/src/client/preview.ts#L26-L136、code/frameworks/angular/src/client/preview.ts#L45-L54、code/frameworks/angular/src/client/preview.ts#L96-L136、code/frameworks/angular/src/client/public-types.ts、code/frameworks/angular/src/client/public-types.ts#L55-L90、code/frameworks/angular/src/client/types.ts、code/frameworks/angular/src/client/types.ts#L9-L26、code/frameworks/angular/src/client/types.ts#L9-L43、code/frameworks/angular/src/index.ts#L1-L17、code/frameworks/angular/src/server/framework-preset-angular-cli.ts、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L119-L159、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L147-L154、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L165-L172、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L17-L72、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L18-L21、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L38-L69、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L57-L69、code/frameworks/angular/src/server/framework-preset-angular-cli.ts#L94-L116、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts#L26-L53、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts#L37-L53、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts#L37-L71、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts#L38-L43、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts#L55-L71、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts#L64-L70、code/frameworks/angular/src/server/preset-options.ts、code/frameworks/angular/src/server/preset-options.ts#L1-L15、code/frameworks/angular/src/server/preset-options.ts#L6-L14
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L124、package.json#L27-L47、package.json#L61-L88、package.json#L90-L106、config.ts、config.ts#L11-L18、config.ts#L11-L20

### 项目概述/核心特性/多框架支持/Next.js框架支持.md

- reference 标题：Next.js框架支持
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.06
- key_source_coverage：0.00
- missing_key_sources：code/e2e-tests/framework-nextjs.spec.ts、code/frameworks/nextjs-vite/src/index.ts、code/frameworks/nextjs-vite/src/index.ts#L1-L35、code/frameworks/nextjs-vite/src/index.ts#L24-L35、code/frameworks/nextjs-vite/src/preview.tsx、code/frameworks/nextjs-vite/src/preview.tsx#L57-L58、code/frameworks/nextjs-vite/src/preview.tsx#L60-L95、code/frameworks/nextjs-vite/src/preview.tsx#L67-L77、code/frameworks/nextjs-vite/src/types.ts、code/frameworks/nextjs-vite/src/types.ts#L42-L68、code/frameworks/nextjs/package.json、code/frameworks/nextjs/package.json#L1-L157、code/frameworks/nextjs/package.json#L137-L142、code/frameworks/nextjs/package.json#L28-L81、code/frameworks/nextjs/src/index.ts、code/frameworks/nextjs/src/index.ts#L1-L27、code/frameworks/nextjs/src/index.ts#L16-L27、code/frameworks/nextjs/src/preview.tsx、code/frameworks/nextjs/src/preview.tsx#L1-L93、code/frameworks/nextjs/src/preview.tsx#L22-L55、code/frameworks/nextjs/src/preview.tsx#L29-L55、code/frameworks/nextjs/src/preview.tsx#L57-L92、code/frameworks/nextjs/src/preview.tsx#L64-L74、code/frameworks/nextjs/src/routing/decorator.tsx、code/frameworks/nextjs/src/routing/decorator.tsx#L1-L52、code/frameworks/nextjs/src/routing/decorator.tsx#L16-L51、code/frameworks/nextjs/src/types.ts、code/frameworks/nextjs/src/types.ts#L53-L79、code/frameworks/nextjs/src/types.ts#L59-L74
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：framework-nextjs.spec.ts、index.ts、index.ts#L1-L35、index.ts#L24-L35、preview.tsx、preview.tsx#L57-L58、preview.tsx#L60-L95、preview.tsx#L67-L77

### 项目概述/核心特性/多框架支持/React框架支持.md

- reference 标题：React框架支持
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.12
- key_source_coverage：0.04
- missing_key_sources：code/frameworks/react-vite/package.json、code/frameworks/react-vite/package.json#L1-L81、code/frameworks/react-vite/package.json#L54-L64、code/frameworks/react-vite/package.json#L54-L75、code/frameworks/react-vite/package.json#L70-L75、code/frameworks/react-vite/src/index.ts、code/frameworks/react-vite/src/index.ts#L1-L5、code/frameworks/react-vite/src/plugins/react-docgen.ts、code/frameworks/react-vite/src/plugins/react-docgen.ts#L1-L132、code/frameworks/react-vite/src/plugins/react-docgen.ts#L101-L131、code/frameworks/react-vite/src/plugins/react-docgen.ts#L40-L99、code/frameworks/react-vite/src/preset.ts#L1-L51、code/frameworks/react-vite/src/preset.ts#L10-L51、code/frameworks/react-vite/src/preset.ts#L5-L51、code/frameworks/react-webpack5/package.json、code/frameworks/react-webpack5/package.json#L1-L74、code/frameworks/react-webpack5/package.json#L50-L54、code/frameworks/react-webpack5/package.json#L50-L63、code/frameworks/react-webpack5/package.json#L58-L63、code/frameworks/react-webpack5/src/index.ts、code/frameworks/react-webpack5/src/index.ts#L1-L4、code/frameworks/react-webpack5/src/preset.ts#L1-L46、code/frameworks/react-webpack5/src/preset.ts#L13-L24、code/frameworks/react-webpack5/src/preset.ts#L13-L46、code/frameworks/react-webpack5/src/preset.ts#L26-L45、code/frameworks/react-webpack5/src/preset.ts#L9-L46、code/lib/react-dom-shim/package.json、code/lib/react-dom-shim/package.json#L1-L54、code/lib/react-dom-shim/package.json#L24-L32、code/lib/react-dom-shim/package.json#L44-L47、code/lib/react-dom-shim/package.json#L44-L48、code/renderers/react/package.json、code/renderers/react/package.json#L1-L97、code/renderers/react/package.json#L54-L58、code/renderers/react/package.json#L54-L85、code/renderers/react/package.json#L81-L86、code/renderers/react/src/index.ts、code/renderers/react/src/index.ts#L1-L10、code/renderers/react/src/preview.tsx、code/renderers/react/src/preview.tsx#L102-L138、code/renderers/react/src/preview.tsx#L55-L255、code/renderers/react/src/preview.tsx#L55-L85、code/renderers/react/src/public-types.ts、code/renderers/react/src/public-types.ts#L29-L66、code/renderers/react/src/public-types.ts#L29-L81、code/renderers/react/src/public-types.ts#L77-L81、code/renderers/react/src/types.ts、code/renderers/react/src/types.ts#L8-L12、code/renderers/react/src/types.ts#L8-L38
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L81、package.json#L54-L64、package.json#L54-L75、package.json#L70-L75、index.ts、index.ts#L1-L5、react-docgen.ts

### 项目概述/核心特性/多框架支持/Svelte框架支持.md

- reference 标题：Svelte框架支持
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.05
- missing_key_sources：code/frameworks/svelte-vite/package.json、code/frameworks/svelte-vite/package.json#L1-L80、code/frameworks/svelte-vite/package.json#L54-L74、code/frameworks/svelte-vite/package.json#L69-L74、code/frameworks/svelte-vite/src/index.ts、code/frameworks/svelte-vite/src/index.ts#L1-L3、code/frameworks/svelte-vite/src/plugins/svelte-docgen.ts、code/frameworks/svelte-vite/src/plugins/svelte-docgen.ts#L1-L147、code/frameworks/svelte-vite/src/plugins/svelte-docgen.ts#L108-L146、code/frameworks/svelte-vite/src/plugins/svelte-docgen.ts#L115-L128、code/frameworks/svelte-vite/src/preset.ts#L1-L32、code/frameworks/svelte-vite/src/preset.ts#L20-L23、code/frameworks/svelte-vite/src/preset.ts#L7-L31、code/frameworks/svelte-vite/src/types.ts、code/frameworks/svelte-vite/src/types.ts#L11-L21、code/frameworks/sveltekit/package.json、code/frameworks/sveltekit/package.json#L1-L79、code/frameworks/sveltekit/package.json#L59-L73、code/frameworks/sveltekit/package.json#L69-L73、code/frameworks/sveltekit/src/index.ts、code/frameworks/sveltekit/src/index.ts#L1-L5、code/frameworks/sveltekit/src/plugins/config-overrides.ts、code/frameworks/sveltekit/src/plugins/config-overrides.ts#L1-L13、code/frameworks/sveltekit/src/plugins/config-overrides.ts#L3-L12、code/frameworks/sveltekit/src/plugins/config-overrides.ts#L8-L10、code/frameworks/sveltekit/src/preset.ts#L1-L44、code/frameworks/sveltekit/src/preset.ts#L12-L36、code/frameworks/sveltekit/src/preset.ts#L12-L43、code/frameworks/sveltekit/src/preset.ts#L21-L36、code/frameworks/sveltekit/src/preset.ts#L27-L35、code/frameworks/sveltekit/src/preset.ts#L38-L43、code/renderers/svelte/package.json、code/renderers/svelte/package.json#L1-L77、code/renderers/svelte/package.json#L55-L71、code/renderers/svelte/package.json#L68-L71、code/renderers/svelte/src/globals.ts、code/renderers/svelte/src/globals.ts#L1-L2、code/renderers/svelte/src/public-types.ts、code/renderers/svelte/src/public-types.ts#L1-L71、code/renderers/svelte/src/public-types.ts#L27-L71
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L80、package.json#L54-L74、package.json#L69-L74、index.ts、index.ts#L1-L3、svelte-docgen.ts、svelte-docgen.ts#L1-L147

### 项目概述/核心特性/多框架支持/Vue框架支持.md

- reference 标题：Vue框架支持
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.17
- key_source_coverage：0.00
- missing_key_sources：code/frameworks/vue3-vite/package.json、code/frameworks/vue3-vite/package.json#L1-L73、code/frameworks/vue3-vite/package.json#L28-L42、code/frameworks/vue3-vite/package.json#L51-L58、code/frameworks/vue3-vite/package.json#L51-L67、code/frameworks/vue3-vite/package.json#L64-L67、code/frameworks/vue3-vite/src/index.ts、code/frameworks/vue3-vite/src/index.ts#L1-L5、code/renderers/vue3/package.json、code/renderers/vue3/package.json#L1-L72、code/renderers/vue3/package.json#L27-L42、code/renderers/vue3/package.json#L51-L55、code/renderers/vue3/package.json#L51-L66、code/renderers/vue3/package.json#L63-L66、code/renderers/vue3/src/index.ts、code/renderers/vue3/src/index.ts#L1-L10
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L73、package.json#L28-L42、package.json#L51-L58、package.json#L51-L67、package.json#L64-L67、index.ts、index.ts#L1-L5

### 项目概述/核心特性/多框架支持/Web Components框架支持.md

- reference 标题：Web Components框架支持
- 生成页：多框架支持/多框架支持.md（多框架支持）
- KnowledgeUnit：unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- reuse_count：14
- skeleton_score：0.10
- key_source_coverage：0.28
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L149-L181、code/frameworks/web-components-vite/build-config.ts、code/frameworks/web-components-vite/package.json、code/frameworks/web-components-vite/package.json#L1-L68、code/frameworks/web-components-vite/package.json#L52-L55、code/frameworks/web-components-vite/preset.js、code/frameworks/web-components-vite/template/cli/js/Button.js、code/frameworks/web-components-vite/template/cli/js/Button.stories.js、code/renderers/web-components/package.json、code/renderers/web-components/package.json#L1-L73、code/renderers/web-components/package.json#L29-L40、code/renderers/web-components/package.json#L49-L53
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L149-L181、build-config.ts、package.json、package.json#L1-L68、package.json#L52-L55、preset.js、Button.js

### 项目概述/核心特性/多框架支持/其他框架支持.md

- reference 标题：其他框架支持
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.00
- missing_key_sources：code/frameworks/ember/package.json、code/frameworks/ember/package.json#L1-L74、code/frameworks/ember/package.json#L50-L55、code/frameworks/ember/package.json#L50-L68、code/frameworks/ember/package.json#L60-L68、code/frameworks/ember/src/index.ts、code/frameworks/ember/src/index.ts#L1-L2、code/frameworks/html-vite/package.json、code/frameworks/html-vite/package.json#L1-L66、code/frameworks/html-vite/package.json#L50-L60、code/frameworks/html-vite/src/index.ts、code/frameworks/html-vite/src/index.ts#L1-L3、code/frameworks/preact-vite/package.json、code/frameworks/preact-vite/package.json#L1-L69、code/frameworks/preact-vite/package.json#L50-L53、code/frameworks/preact-vite/package.json#L50-L63、code/frameworks/preact-vite/package.json#L60-L63、code/frameworks/preact-vite/src/index.ts、code/frameworks/preact-vite/src/index.ts#L1-L3、code/frameworks/react-native-web-vite/package.json、code/frameworks/react-native-web-vite/package.json#L1-L77、code/frameworks/react-native-web-vite/package.json#L53-L58、code/frameworks/react-native-web-vite/package.json#L53-L71、code/frameworks/react-native-web-vite/package.json#L64-L71、code/frameworks/react-native-web-vite/src/index.ts、code/frameworks/react-native-web-vite/src/index.ts#L1-L5、code/renderers/preact/package.json、code/renderers/preact/package.json#L1-L63、code/renderers/preact/package.json#L46-L57、docs/frameworks.js、docs/frameworks.js#L1-L272
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L74、package.json#L50-L55、package.json#L50-L68、package.json#L60-L68、index.ts、index.ts#L1-L2、package.json

### 项目概述/核心特性/多框架支持/多框架支持.md

- reference 标题：多框架支持
- 生成页：多框架支持/多框架支持.md（多框架支持）
- KnowledgeUnit：unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- reuse_count：14
- skeleton_score：0.06
- key_source_coverage：0.31
- missing_key_sources：code/e2e-tests/framework-nextjs.spec.ts、code/e2e-tests/framework-svelte.spec.ts、code/frameworks/angular/package.json、code/frameworks/angular/package.json#L1-L124、code/frameworks/angular/package.json#L61-L106、code/frameworks/angular/package.json#L90-L106、code/frameworks/angular/src/client/config.ts、code/frameworks/angular/src/client/docs/config.ts、code/frameworks/angular/src/client/preview-prod.tsx、code/frameworks/angular/src/client/preview.tsx、code/frameworks/angular/src/index.ts#L1-L17、code/frameworks/angular/src/node/preview.tsx、code/frameworks/angular/src/server/framework-preset-angular-cli.ts、code/frameworks/angular/src/server/framework-preset-angular-ivy.ts、code/frameworks/angular/src/types.ts、code/frameworks/react-vite/package.json、code/frameworks/react-vite/package.json#L1-L81、code/frameworks/react-vite/package.json#L54-L75、code/frameworks/react-vite/src/index.ts#L1-L5、code/frameworks/react-vite/src/types.ts、code/frameworks/svelte-vite/package.json、code/frameworks/svelte-vite/package.json#L1-L80、code/frameworks/svelte-vite/package.json#L54-L74、code/frameworks/svelte-vite/src/index.ts#L1-L3、code/frameworks/svelte-vite/src/types.ts、code/frameworks/vue3-vite/package.json、code/frameworks/vue3-vite/package.json#L1-L73、code/frameworks/vue3-vite/package.json#L51-L67、code/frameworks/vue3-vite/src/index.ts#L1-L5、code/frameworks/vue3-vite/src/types.ts、code/frameworks/web-components-vite/package.json、code/frameworks/web-components-vite/package.json#L1-L68、code/frameworks/web-components-vite/package.json#L52-L62、code/frameworks/web-components-vite/src/index.ts#L1-L5、code/frameworks/web-components-vite/src/types.ts、code/renderers/react/package.json、code/renderers/react/package.json#L1-L97、code/renderers/react/package.json#L54-L86、code/renderers/react/package.json#L81-L86、code/renderers/react/src/globals.ts、code/renderers/react/src/index.ts#L1-L10、code/renderers/react/src/portable-stories.ts、code/renderers/react/src/preview.tsx、code/renderers/react/src/public-types.ts、code/renderers/svelte/package.json、code/renderers/svelte/package.json#L1-L77、code/renderers/svelte/package.json#L55-L71、code/renderers/svelte/package.json#L68-L71、code/renderers/svelte/src/globals.ts、code/renderers/svelte/src/index.ts#L1-L5、code/renderers/svelte/src/portable-stories.ts、code/renderers/svelte/src/preview.tsx、code/renderers/svelte/src/public-types.ts、code/renderers/vue3/package.json、code/renderers/vue3/package.json#L1-L72、code/renderers/vue3/package.json#L51-L66、code/renderers/vue3/package.json#L63-L66、code/renderers/vue3/src/globals.ts、code/renderers/vue3/src/index.ts#L1-L10、code/renderers/vue3/src/portable-stories.ts、code/renderers/vue3/src/preview.tsx、code/renderers/vue3/src/public-types.ts、code/renderers/web-components/src/preview.tsx、docs/frameworks.js、docs/index.mdx
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：framework-nextjs.spec.ts、framework-svelte.spec.ts、package.json、package.json#L1-L124、package.json#L61-L106、package.json#L90-L106、config.ts、config.ts

### 项目概述/核心特性/插件系统架构/Addon API设计.md

- reference 标题：Addon API设计
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.08
- key_source_coverage：0.05
- missing_key_sources：code/addons/a11y/src/index.ts、code/addons/a11y/src/index.ts#L1-L11、code/addons/a11y/src/params.ts、code/addons/a11y/src/params.ts#L20-L44、code/addons/a11y/src/preset.ts#L1-L4、code/addons/a11y/src/types.ts、code/addons/a11y/src/types.ts#L1-L68、code/addons/a11y/src/types.ts#L41-L68、code/addons/a11y/src/types.ts#L5-L68、code/addons/a11y/src/types.ts#L7-L31、code/addons/a11y/src/types.ts#L7-L68、code/addons/docs/src/DocsRenderer.tsx、code/addons/docs/src/DocsRenderer.tsx#L48-L87、code/addons/docs/src/DocsRenderer.tsx#L54-L86、code/addons/docs/src/DocsRenderer.tsx#L83-L86、code/addons/links/manager.js、code/addons/links/manager.js#L1-L2、code/addons/links/preview.js、code/addons/links/preview.js#L1-L2、code/addons/themes/manager.js、code/addons/themes/manager.js#L1-L2
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：index.ts、index.ts#L1-L11、params.ts、params.ts#L20-L44、preset.ts#L1-L4、types.ts、types.ts#L1-L68、types.ts#L41-L68

### 项目概述/核心特性/插件系统架构/Manager API.md

- reference 标题：Manager API
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.14
- key_source_coverage：0.00
- missing_key_sources：code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/core/src/channels/postmessage/index.ts、code/core/src/channels/postmessage/index.ts#L200-L239、code/core/src/channels/types.ts、code/core/src/channels/types.ts#L1-L59、code/core/src/manager-api/lib/addons.ts、code/core/src/manager-api/lib/addons.ts#L1-L149、code/core/src/manager-api/lib/addons.ts#L120-L125、code/core/src/manager-api/lib/addons.ts#L120-L129、code/core/src/manager-api/lib/addons.ts#L136-L146、code/core/src/manager-api/lib/addons.ts#L31-L134、code/core/src/manager-api/lib/addons.ts#L31-L146、code/core/src/manager-api/lib/addons.ts#L59-L59、code/core/src/manager-api/root.tsx、code/core/src/manager-api/root.tsx#L1-L200、code/core/src/manager-api/root.tsx#L130-L197、code/core/src/manager-api/root.tsx#L137-L197、code/core/src/manager-api/root.tsx#L139-L211、code/core/src/manager-api/root.tsx#L170-L197、code/core/src/manager-api/root.tsx#L229-L243、code/core/src/manager-api/root.tsx#L80-L124、code/core/src/manager-api/root.tsx#L80-L243、code/core/src/manager-api/root.tsx#L82-L110、code/core/src/manager-api/root.tsx#L82-L197、code/core/src/manager-api/store.ts、code/core/src/manager-api/store.ts#L1-L123、code/core/src/manager-api/store.ts#L48-L121、code/core/src/manager-api/store.ts#L48-L65、code/core/src/manager-api/store.ts#L75-L121、code/core/src/manager/components/error-boundary/ManagerErrorBoundary.tsx、code/core/src/manager/components/error-boundary/ManagerErrorBoundary.tsx#L171-L211、code/core/src/manager/index.tsx、code/core/src/manager/index.tsx#L40-L67、docs/_snippets/storybook-addons-root-preset-manager-entries.md、docs/_snippets/storybook-addons-root-preset-manager-entries.md#L1-L6、docs/_snippets/storybook-main-register-addon.md、docs/_snippets/storybook-main-register-addon.md#L1-L6
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：manager.tsx、manager.tsx#L1-L9、index.ts、index.ts#L200-L239、types.ts、types.ts#L1-L59、addons.ts、addons.ts#L1-L149

### 项目概述/核心特性/插件系统架构/Preview API.md

- reference 标题：Preview API
- 生成页：API-参考/开发API参考/Preview-API.md（Preview API）
- KnowledgeUnit：unit_id=unit-c5c20b0bb950, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0
- reuse_count：2
- skeleton_score：0.14
- key_source_coverage：0.00
- missing_key_sources：code/.storybook/main.ts、code/.storybook/preview.tsx、code/core/src/core-server/presets/wsToken.ts、code/core/src/manager-api/modules/stories.ts、code/core/src/preview-api/modules/preview-web/PreviewWeb.test.ts、code/e2e-tests/preview-api.spec.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、preview.tsx、wsToken.ts、stories.ts、PreviewWeb.test.ts、preview-api.spec.ts

### 项目概述/核心特性/插件系统架构/插件生命周期管理.md

- reference 标题：插件生命周期管理
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.07
- key_source_coverage：0.00
- missing_key_sources：code/addons/a11y/src/manager.tsx、code/addons/a11y/src/manager.tsx#L1-L67、code/addons/a11y/src/manager.tsx#L14-L48、code/addons/a11y/src/manager.tsx#L50-L66、code/core/src/manager-api/lib/addons.ts、code/core/src/manager-api/lib/addons.ts#L1-L149、code/core/src/manager-api/lib/addons.ts#L120-L129、code/core/src/manager-api/lib/addons.ts#L31-L134、code/core/src/manager-api/root.tsx、code/core/src/manager-api/root.tsx#L130-L244、code/core/src/manager-api/root.tsx#L170-L187、code/core/src/manager-api/root.tsx#L170-L244、code/core/src/manager-api/root.tsx#L329-L339、code/core/src/manager-api/root.tsx#L341-L456、code/core/src/manager-api/root.tsx#L366-L456、code/core/src/manager-api/stores/status.ts、code/core/src/manager-api/stores/status.ts#L1-L17、code/lib/cli-storybook/src/postinstallAddon.ts、code/lib/cli-storybook/src/postinstallAddon.ts#L1-L49、code/lib/cli-storybook/src/postinstallAddon.ts#L10-L49、docs/_snippets/storybook-addons-api-getqueryparam.md、docs/_snippets/storybook-addons-api-getqueryparam.md#L1-L6、docs/_snippets/storybook-addons-api-selectincurrentkind.md、docs/_snippets/storybook-addons-api-selectincurrentkind.md#L1-L6、docs/_snippets/storybook-addons-api-togglepanel.md、docs/_snippets/storybook-addons-api-togglepanel.md#L1-L13、docs/_snippets/storybook-addons-root-preset-manager-entries.md、docs/_snippets/storybook-addons-root-preset-manager-entries.md#L1-L6、docs/_snippets/storybook-main-register-addon.md、docs/_snippets/storybook-main-register-addon.md#L1-L6
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：manager.tsx、manager.tsx#L1-L67、manager.tsx#L14-L48、manager.tsx#L50-L66、addons.ts、addons.ts#L1-L149、addons.ts#L120-L129、addons.ts#L31-L134

### 项目概述/核心特性/插件系统架构/插件系统架构.md

- reference 标题：插件系统架构
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.06
- key_source_coverage：0.02
- missing_key_sources：code/addons/a11y/package.json、code/addons/a11y/package.json#L37-L52、code/addons/a11y/src/manager.tsx、code/addons/a11y/src/manager.tsx#L50-L67、code/addons/docs/package.json#L40-L80、code/addons/docs/src/preview.ts、code/addons/docs/src/preview.ts#L1-L30、code/addons/docs/src/preview.ts#L14-L30、code/builders/builder-vite/src/codegen-set-addon-channel.ts、code/builders/builder-vite/src/codegen-set-addon-channel.ts#L1-L14、code/core/src/channels/main.ts#L22-L148、code/core/src/channels/main.ts#L62-L82、code/core/src/channels/types.ts、code/core/src/channels/types.ts#L1-L59、code/core/src/instrumenter/instrumenter.test.ts、code/core/src/instrumenter/instrumenter.test.ts#L716-L736、code/core/src/manager-api/lib/addons.ts、code/core/src/manager-api/lib/addons.ts#L100-L111、code/core/src/manager-api/lib/addons.ts#L120-L129、code/core/src/manager-api/lib/addons.ts#L31-L149、code/core/src/manager-api/lib/addons.ts#L59-L66、code/core/src/manager-api/lib/addons.ts#L87-L111、code/core/src/manager-api/lib/addons.ts#L87-L129、code/core/src/manager-api/root.tsx、code/core/src/manager-api/root.tsx#L80-L135、code/core/src/preview-api/addons.ts、code/core/src/preview-api/addons.ts#L1-L4、code/core/src/preview-api/modules/addons/hooks.ts、code/core/src/preview-api/modules/addons/hooks.ts#L119-L128、code/core/src/preview-api/modules/addons/hooks.ts#L96-L140、code/core/src/shared/universal-store/index.ts、code/core/src/shared/universal-store/index.ts#L310-L526、code/core/src/shared/universal-store/index.ts#L507-L513、code/core/src/tabs/modules/addons.ts、code/core/src/types/modules/addons.ts、code/core/src/types/modules/addons.ts#L324-L522、code/core/src/types/modules/addons.ts#L493-L522、code/lib/cli-storybook/src/postinstallAddon.ts、code/lib/cli-storybook/src/postinstallAddon.ts#L1-L49、code/lib/cli-storybook/src/postinstallAddon.ts#L10-L49
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L37-L52、manager.tsx、manager.tsx#L50-L67、package.json#L40-L80、preview.ts、preview.ts#L1-L30、preview.ts#L14-L30

### 项目概述/核心特性/插件系统架构/通信机制.md

- reference 标题：通信机制
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.08
- key_source_coverage：0.03
- missing_key_sources：code/core/src/channels/main.ts#L1-L148、code/core/src/channels/main.ts#L22-L147、code/core/src/channels/main.ts#L62-L82、code/core/src/channels/main.ts#L76-L82、code/core/src/channels/postmessage/getEventSourceUrl.ts、code/core/src/channels/postmessage/getEventSourceUrl.ts#L1-L53、code/core/src/channels/postmessage/getEventSourceUrl.ts#L3-L52、code/core/src/channels/postmessage/index.ts、code/core/src/channels/postmessage/index.ts#L1-L240、code/core/src/channels/postmessage/index.ts#L194-L238、code/core/src/channels/postmessage/index.ts#L29-L239、code/core/src/channels/postmessage/index.ts#L66-L129、code/core/src/channels/postmessage/index.ts#L66-L96、code/core/src/channels/types.ts、code/core/src/channels/types.ts#L1-L60、code/core/src/channels/types.ts#L11-L60、code/core/src/channels/types.ts#L13-L16、code/core/src/channels/types.ts#L18-L22、code/core/src/channels/websocket/index.ts、code/core/src/channels/websocket/index.ts#L1-L109、code/core/src/channels/websocket/index.ts#L23-L108、code/core/src/channels/websocket/index.ts#L44-L75、code/core/src/channels/websocket/index.ts#L65-L74、code/core/src/channels/websocket/index.ts#L81-L107、code/core/src/manager-api/lib/addons.ts、code/core/src/manager-api/lib/addons.ts#L1-L111、code/core/src/manager-api/lib/addons.ts#L100-L109、code/core/src/manager-api/lib/addons.ts#L56-L111、code/core/src/manager/App.tsx、code/core/src/manager/App.tsx#L1-L81、code/core/src/manager/App.tsx#L44-L61
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L148、main.ts#L22-L147、main.ts#L62-L82、main.ts#L76-L82、getEventSourceUrl.ts、getEventSourceUrl.ts#L1-L53、getEventSourceUrl.ts#L3-L52、index.ts

### 项目概述/核心特性/文档生成功能/Doc Blocks API.md

- reference 标题：Doc Blocks API
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.08
- key_source_coverage：0.00
- missing_key_sources：code/addons/docs/src/blocks/blocks/ArgTypes.tsx、code/addons/docs/src/blocks/blocks/ArgTypes.tsx#L1-L200、code/addons/docs/src/blocks/blocks/Canvas.tsx、code/addons/docs/src/blocks/blocks/Canvas.tsx#L1-L200、code/addons/docs/src/blocks/blocks/Controls.tsx、code/addons/docs/src/blocks/blocks/Controls.tsx#L1-L200、code/addons/docs/src/blocks/blocks/Description.tsx、code/addons/docs/src/blocks/blocks/Description.tsx#L1-L200、code/addons/docs/src/blocks/blocks/DocsPage.test.ts、code/addons/docs/src/blocks/blocks/DocsPage.test.ts#L1-L100、code/addons/docs/src/blocks/blocks/Meta.tsx、code/addons/docs/src/blocks/blocks/Meta.tsx#L1-L200、code/addons/docs/src/blocks/blocks/Primary.tsx、code/addons/docs/src/blocks/blocks/Primary.tsx#L1-L200、code/addons/docs/src/blocks/blocks/Source.tsx、code/addons/docs/src/blocks/blocks/Source.tsx#L1-L200、code/addons/docs/src/blocks/blocks/Stories.tsx、code/addons/docs/src/blocks/blocks/Stories.tsx#L1-L200、code/addons/docs/src/blocks/blocks/Story.tsx、code/addons/docs/src/blocks/blocks/Story.tsx#L1-L200、code/addons/docs/src/blocks/blocks/Subtitle.tsx、code/addons/docs/src/blocks/blocks/Subtitle.tsx#L1-L200、code/addons/docs/src/blocks/blocks/TableOfContents.tsx、code/addons/docs/src/blocks/blocks/TableOfContents.tsx#L1-L200、code/addons/docs/src/blocks/blocks/Title.tsx、code/addons/docs/src/blocks/blocks/Title.tsx#L1-L200、code/addons/docs/src/blocks/blocks/Typeset.tsx、code/addons/docs/src/blocks/blocks/Typeset.tsx#L1-L200、code/addons/docs/src/blocks/blocks/Unstyled.mdx、code/addons/docs/src/blocks/blocks/Unstyled.mdx#L1-L200、code/addons/docs/src/blocks/blocks/external/ExternalDocsContext.ts、code/addons/docs/src/blocks/blocks/external/ExternalDocsContext.ts#L1-L100、code/addons/docs/src/blocks/blocks/external/ExternalPreview.ts、code/addons/docs/src/blocks/blocks/external/ExternalPreview.ts#L1-L100、code/addons/docs/src/blocks/components/ColorPalette.tsx、code/addons/docs/src/blocks/components/ColorPalette.tsx#L1-L200、code/addons/docs/src/blocks/components/DocsPageExampleCaption.mdx、code/addons/docs/src/blocks/components/DocsPageExampleCaption.mdx#L1-L200、code/addons/docs/src/blocks/components/IconGallery.tsx、code/addons/docs/src/blocks/components/IconGallery.tsx#L1-L200、code/core/src/components/brand/colorpalette.mdx、code/core/src/components/brand/colorpalette.mdx#L1-L200、code/core/src/components/brand/typography.mdx、code/core/src/components/brand/typography.mdx#L1-L200、code/core/src/components/components/Button/Docs.mdx、code/core/src/components/components/Button/Docs.mdx#L1-L200
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ArgTypes.tsx、ArgTypes.tsx#L1-L200、Canvas.tsx、Canvas.tsx#L1-L200、Controls.tsx、Controls.tsx#L1-L200、Description.tsx、Description.tsx#L1-L200

### 项目概述/核心特性/文档生成功能/MDX文档编写.md

- reference 标题：MDX文档编写
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.21
- key_source_coverage：0.05
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L107-L116、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L336-L357、code/addons/docs/package.json、code/addons/docs/package.json#L1-L133、code/addons/docs/package.json#L40-L80、code/addons/docs/src/DocsRenderer.tsx、code/addons/docs/src/DocsRenderer.tsx#L1-L88、code/addons/docs/src/DocsRenderer.tsx#L15-L22、code/addons/docs/src/DocsRenderer.tsx#L54-L81、code/addons/docs/src/blocks.ts、code/addons/docs/src/blocks.ts#L1-L10、code/addons/docs/src/blocks/components/ArgsTable/ArgsTable.tsx、code/addons/docs/src/blocks/components/ArgsTable/ArgsTable.tsx#L1-L200、code/addons/docs/src/index.ts、code/addons/docs/src/index.ts#L1-L21、code/addons/docs/src/index.ts#L11-L18、code/addons/docs/template/stories/docs2/MetaOf.mdx、code/addons/docs/template/stories/docs2/MetaOf.mdx#L1-L13、code/addons/docs/template/stories/docs2/Title.mdx、code/addons/docs/template/stories/docs2/Title.mdx#L1-L10、docs/writing-docs/build-documentation.mdx、docs/writing-docs/build-documentation.mdx#L1-L62、docs/writing-docs/build-documentation.mdx#L10-L62、docs/writing-docs/build-documentation.mdx#L28-L33、docs/writing-docs/doc-blocks.mdx、docs/writing-docs/doc-blocks.mdx#L1-L295、docs/writing-docs/doc-blocks.mdx#L128-L272、docs/writing-docs/doc-blocks.mdx#L18-L50、docs/writing-docs/doc-blocks.mdx#L54-L96、docs/writing-docs/doc-blocks.mdx#L77-L96、docs/writing-docs/index.mdx#L1-L20、docs/writing-docs/mdx.mdx、docs/writing-docs/mdx.mdx#L1-L316、docs/writing-docs/mdx.mdx#L247-L316、docs/writing-docs/mdx.mdx#L40-L115
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L107-L116、preview.tsx、preview.tsx#L1-L416、preview.tsx#L144-L193、preview.tsx#L336-L357、package.json、package.json#L1-L133

### 项目概述/核心特性/文档生成功能/文档生成功能.md

- reference 标题：文档生成功能
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.14
- key_source_coverage：0.04
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L107-L116、code/.storybook/main.ts#L149-L181、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L220-L301、code/.storybook/preview.tsx#L336-L400、code/addons/docs/package.json、code/addons/docs/package.json#L1-L133、code/addons/docs/package.json#L40-L80、code/addons/docs/src/blocks.ts、code/addons/docs/src/blocks.ts#L1-L10、code/addons/docs/src/blocks/blocks/index.ts、code/addons/docs/src/blocks/blocks/index.ts#L1-L32、code/addons/docs/src/preview.ts、code/addons/docs/src/preview.ts#L1-L30、code/addons/docs/src/preview.ts#L14-L29、code/addons/docs/template/stories/docs2/MetaOf.mdx、code/addons/docs/template/stories/docs2/MetaOf.mdx#L1-L13、code/addons/docs/template/stories/docs2/Title.mdx、code/addons/docs/template/stories/docs2/Title.mdx#L1-L10、code/core/package.json、code/core/package.json#L1-L389、code/core/package.json#L48-L221、code/core/src/components/components/Button/Button.stories.tsx、code/core/src/components/components/Button/Button.stories.tsx#L1-L409、code/core/src/components/components/Button/Button.stories.tsx#L11-L16、code/core/src/components/components/Button/Button.stories.tsx#L22-L409、docs/writing-docs/autodocs.mdx、docs/writing-docs/autodocs.mdx#L1-L279、docs/writing-docs/autodocs.mdx#L111-L144、docs/writing-docs/autodocs.mdx#L14-L86、docs/writing-docs/autodocs.mdx#L160-L208、docs/writing-docs/autodocs.mdx#L180-L198、docs/writing-docs/autodocs.mdx#L229-L279、docs/writing-docs/index.mdx#L1-L20、docs/writing-docs/mdx.mdx、docs/writing-docs/mdx.mdx#L1-L316、docs/writing-docs/mdx.mdx#L115-L190、docs/writing-docs/mdx.mdx#L14-L113、docs/writing-docs/mdx.mdx#L191-L213、docs/writing-docs/mdx.mdx#L217-L245、docs/writing-docs/mdx.mdx#L247-L316
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L107-L116、main.ts#L149-L181、preview.tsx、preview.tsx#L1-L416、preview.tsx#L144-L193、preview.tsx#L195-L334、preview.tsx#L220-L301

### 项目概述/核心特性/文档生成功能/自动文档系统.md

- reference 标题：自动文档系统
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.14
- key_source_coverage：0.03
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L19-L116、code/.storybook/main.ts#L19-L186、code/addons/docs/README.md、code/addons/docs/README.md#L1-L157、code/addons/docs/README.md#L122-L142、code/addons/docs/README.md#L85-L142、code/addons/docs/README.md#L9-L40、code/addons/docs/docs/docspage.md、code/addons/docs/docs/docspage.md#L1-L176、code/addons/docs/docs/docspage.md#L152-L168、code/addons/docs/docs/docspage.md#L7-L30、code/addons/docs/docs/mdx.md、code/addons/docs/docs/mdx.md#L1-L201、code/addons/docs/docs/mdx.md#L7-L58、code/builders/builder-vite/src/plugins/csf-plugin.ts、code/builders/builder-vite/src/plugins/csf-plugin.ts#L1-L23、code/builders/builder-vite/src/plugins/csf-plugin.ts#L17-L22、code/builders/builder-vite/src/plugins/csf-plugin.ts#L7-L22、code/core/package.json、code/core/package.json#L1-L389、code/core/package.json#L107-L121、code/core/package.json#L117-L121、code/core/src/csf/index.ts、code/core/src/csf/index.ts#L1-L97、code/core/src/csf/index.ts#L26-L91、code/core/src/docs-tools/index.ts、code/core/src/docs-tools/shared.ts、code/core/src/docs-tools/shared.ts#L1-L26
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L19-L116、main.ts#L19-L186、README.md、README.md#L1-L157、README.md#L122-L142、README.md#L85-L142、README.md#L9-L40

### 项目概述/核心特性/文档生成功能/自定义文档页面.md

- reference 标题：自定义文档页面
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.06
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L107-L116、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L19-L132、code/.storybook/main.ts#L19-L148、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L144-L334、code/.storybook/preview.tsx#L157-L193、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L208-L215、code/.storybook/preview.tsx#L220-L301、code/.storybook/preview.tsx#L309-L333、code/.storybook/preview.tsx#L336-L400、code/.storybook/preview.tsx#L340-L355、code/.storybook/preview.tsx#L402-L415、code/addons/docs/template/stories/docs2/MetaOf.mdx、code/addons/docs/template/stories/docs2/MetaOf.mdx#L1-L13、code/addons/docs/template/stories/docs2/Title.mdx、code/addons/docs/template/stories/docs2/Title.mdx#L1-L10、code/core/src/theming/create.ts、code/core/src/theming/create.ts#L1-L51、code/core/src/theming/create.ts#L20-L50、code/core/src/theming/create.ts#L29-L50、code/core/src/theming/index.ts、code/core/src/theming/index.ts#L1-L51、code/core/src/theming/index.ts#L10-L51、code/core/src/theming/index.ts#L30-L51、docs/index.mdx#L1-L37
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L107-L116、main.ts#L149-L181、main.ts#L19-L132、main.ts#L19-L148、manager.tsx、manager.tsx#L1-L9、manager.tsx#L4-L8

### 项目概述/核心特性/核心特性.md

- reference 标题：核心特性
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.07
- key_source_coverage：0.05
- missing_key_sources：.storybook/main.ts#L1-L186、.storybook/main.ts#L129-L181、.storybook/main.ts#L169-L181、.storybook/main.ts#L19-L132、.storybook/manager.tsx、.storybook/manager.tsx#L1-L9、.storybook/manager.tsx#L4-L8、.storybook/preview.tsx、.storybook/preview.tsx#L1-L416、.storybook/preview.tsx#L10-L21、.storybook/preview.tsx#L144-L193、.storybook/preview.tsx#L195-L334、.storybook/preview.tsx#L336-L400、.storybook/preview.tsx#L402-L415、code/addons/a11y/src/preset.ts#L1-L3、code/addons/docs/src/manifest.test.ts、code/addons/docs/src/manifest.test.ts#L179-L205、code/addons/links/project.json、code/addons/links/project.json#L1-L9、code/addons/themes/project.json、code/addons/themes/project.json#L1-L8、code/core/package.json、code/core/package.json#L1-L389、code/core/package.json#L48-L221、code/core/src/components/components/Tabs/Tabs.hooks.tsx、code/core/src/components/components/Tabs/Tabs.hooks.tsx#L57-L99、code/core/src/controls/constants.ts、code/core/src/controls/constants.ts#L1-L3、code/core/src/core-server/utils/manifests/render-components-manifest.ts、code/core/src/core-server/utils/manifests/render-components-manifest.ts#L804-L850、code/core/src/manager/container/Menu.tsx、code/core/src/manager/container/Menu.tsx#L203-L245、code/e2e-tests/addon-docs.spec.ts、code/e2e-tests/addon-docs.spec.ts#L84-L109、docs/_snippets/storybook-preview-extended-theme-variables.md、docs/_snippets/storybook-preview-extended-theme-variables.md#L1-L7、docs/faq.mdx、docs/faq.mdx#L174-L175、package.json、package.json#L5-L16
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L129-L181、main.ts#L169-L181、main.ts#L19-L132、manager.tsx、manager.tsx#L1-L9、manager.tsx#L4-L8、preview.tsx

### 项目概述/核心特性/测试支持功能/Playwright测试.md

- reference 标题：Playwright测试
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.11
- key_source_coverage：0.00
- missing_key_sources：code/chromatic.config.json、code/chromatic.config.json#L1-L10、code/e2e-tests/addon-a11y.spec.ts、code/e2e-tests/addon-a11y.spec.ts#L1-L83、code/e2e-tests/addon-a11y.spec.ts#L14-L81、code/e2e-tests/component-tests.spec.ts、code/e2e-tests/component-tests.spec.ts#L1-L189、code/e2e-tests/component-tests.spec.ts#L47-L56、code/e2e-tests/component-tests.spec.ts#L60-L61、code/e2e-tests/component-tests.spec.ts#L9-L146、code/e2e-tests/manager.spec.ts、code/e2e-tests/manager.spec.ts#L1-L346、code/e2e-tests/manager.spec.ts#L17-L344、code/e2e-tests/navigation.spec.ts、code/e2e-tests/navigation.spec.ts#L1-L24、code/e2e-tests/navigation.spec.ts#L9-L22、code/e2e-tests/util.ts、code/e2e-tests/util.ts#L1-L300、code/e2e-tests/util.ts#L117-L148、code/e2e-tests/util.ts#L154-L161、code/e2e-tests/util.ts#L18-L74、code/e2e-tests/util.ts#L252-L273、code/e2e-tests/util.ts#L32-L42、code/e2e-tests/util.ts#L44-L74、code/e2e-tests/util.ts#L8-L274、code/e2e-tests/util.ts#L96-L115、code/package.json、code/package.json#L1-L219、code/package.json#L37-L42、code/package.json#L71-L149、code/playwright.config.ts、code/playwright.config.ts#L1-L115、code/playwright.config.ts#L11-L115、code/playwright.config.ts#L110-L114、code/playwright.config.ts#L22-L30、code/playwright.config.ts#L25-L30、code/playwright.config.ts#L57-L60、code/playwright.config.ts#L6-L8
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：chromatic.config.json、chromatic.config.json#L1-L10、addon-a11y.spec.ts、addon-a11y.spec.ts#L1-L83、addon-a11y.spec.ts#L14-L81、component-tests.spec.ts、component-tests.spec.ts#L1-L189、component-tests.spec.ts#L47-L56

### 项目概述/核心特性/测试支持功能/Vitest集成.md

- reference 标题：Vitest集成
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.15
- key_source_coverage：0.02
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L107-L116、code/.storybook/main.ts#L19-L183、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L336-L400、code/.storybook/preview.tsx#L402-L415、code/.storybook/preview.tsx#L413-L413、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L1-L30、code/.storybook/storybook.setup.ts#L11-L29、code/addons/vitest/vitest.config.ts、code/addons/vitest/vitest.config.ts#L1-L11、code/core/vitest.config.ts、code/core/vitest.config.ts#L1-L16、code/core/vitest.config.ts#L8-L13、code/frameworks/react-vite/vitest.config.ts、code/frameworks/react-vite/vitest.config.ts#L1-L11、code/lib/cli-sb/vitest.config.ts、code/lib/cli-sb/vitest.config.ts#L1-L11、code/vitest-setup.ts、code/vitest-setup.ts#L1-L123、code/vitest-setup.ts#L58-L123、code/vitest-setup.ts#L58-L72、code/vitest-setup.ts#L6-L49、code/vitest.config.storybook.ts、code/vitest.config.storybook.ts#L1-L56、code/vitest.config.storybook.ts#L21-L55、code/vitest.config.storybook.ts#L30-L40、code/vitest.config.storybook.ts#L32-L40、code/vitest.config.storybook.ts#L41-L51、code/vitest.config.ts、code/vitest.config.ts#L1-L57、code/vitest.config.ts#L12-L13、code/vitest.config.ts#L12-L31、code/vitest.config.ts#L12-L56、code/vitest.config.ts#L15-L31、code/vitest.config.ts#L33-L56、code/vitest.config.ts#L43-L54、code/vitest.helpers.ts、code/vitest.helpers.ts#L1-L37、code/vitest.shared.ts、code/vitest.shared.ts#L1-L16、code/vitest.shared.ts#L5-L15
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L107-L116、main.ts#L19-L183、preview.tsx、preview.tsx#L1-L416、preview.tsx#L195-L334、preview.tsx#L336-L400、preview.tsx#L402-L415

### 项目概述/核心特性/测试支持功能/可访问性测试.md

- reference 标题：可访问性测试
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.11
- key_source_coverage：0.00
- missing_key_sources：code/addons/a11y/package.json、code/addons/a11y/package.json#L1-L90、code/addons/a11y/package.json#L60-L63、code/addons/a11y/package.json#L60-L88、code/addons/a11y/src/constants.ts、code/addons/a11y/src/constants.ts#L1-L22、code/addons/a11y/src/index.ts、code/addons/a11y/src/index.ts#L1-L10、code/addons/a11y/src/index.ts#L1-L11、code/addons/a11y/src/index.ts#L6-L10、code/addons/a11y/src/manager.tsx、code/addons/a11y/src/manager.tsx#L1-L67、code/addons/a11y/src/manager.tsx#L14-L48、code/addons/a11y/src/manager.tsx#L14-L66、code/addons/a11y/src/manager.tsx#L5-L12、code/addons/a11y/src/manager.tsx#L50-L66、code/addons/a11y/src/params.ts、code/addons/a11y/src/params.ts#L1-L4、code/addons/a11y/src/params.ts#L1-L45、code/addons/a11y/src/params.ts#L20-L27、code/addons/a11y/src/params.ts#L20-L44、code/addons/a11y/src/params.ts#L42-L44、code/addons/a11y/src/preview.tsx、code/addons/a11y/src/preview.tsx#L1-L110、code/addons/a11y/src/preview.tsx#L14-L109、code/addons/a11y/src/preview.tsx#L14-L96、code/addons/a11y/src/preview.tsx#L25-L41、code/addons/a11y/src/preview.tsx#L3-L12、code/addons/a11y/src/preview.tsx#L41-L96、code/addons/a11y/src/preview.tsx#L63-L75、code/addons/a11y/src/preview.tsx#L98-L109、code/addons/a11y/src/types.ts、code/addons/a11y/src/types.ts#L1-L6、code/addons/a11y/src/types.ts#L1-L68、code/addons/a11y/src/types.ts#L33-L68、code/addons/a11y/src/types.ts#L41-L58、code/addons/a11y/src/types.ts#L49-L58、code/e2e-tests/addon-a11y.spec.ts、code/e2e-tests/addon-a11y.spec.ts#L14-L81
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L1-L90、package.json#L60-L63、package.json#L60-L88、constants.ts、constants.ts#L1-L22、index.ts、index.ts#L1-L10

### 项目概述/核心特性/测试支持功能/测试支持功能.md

- reference 标题：测试支持功能
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.11
- key_source_coverage：0.02
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L107-L116、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L402-L415、code/.storybook/preview.tsx#L413-L415、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L1-L30、code/.storybook/storybook.setup.ts#L11-L29、code/e2e-tests/addon-a11y.spec.ts、code/e2e-tests/addon-a11y.spec.ts#L1-L83、code/e2e-tests/component-tests.spec.ts、code/e2e-tests/component-tests.spec.ts#L1-L189、code/e2e-tests/util.ts、code/e2e-tests/util.ts#L1-L300、code/e2e-tests/util.ts#L8-L274、code/e2e-tests/util.ts#L96-L148、code/playwright.config.ts、code/playwright.config.ts#L1-L115、code/playwright.config.ts#L11-L115、code/playwright.config.ts#L23-L29、code/playwright.config.ts#L31-L41、code/playwright.config.ts#L31-L51、code/vitest-setup.ts、code/vitest-setup.ts#L1-L123、code/vitest-setup.ts#L58-L72、code/vitest-setup.ts#L6-L49、code/vitest.config.storybook.ts、code/vitest.config.storybook.ts#L1-L56、code/vitest.config.storybook.ts#L21-L55、code/vitest.config.storybook.ts#L22-L29、code/vitest.config.storybook.ts#L22-L54、code/vitest.config.storybook.ts#L32-L51、code/vitest.config.ts、code/vitest.config.ts#L1-L57、code/vitest.config.ts#L12-L13、code/vitest.config.ts#L12-L31、code/vitest.config.ts#L15-L31、code/vitest.config.ts#L33-L56、code/vitest.config.ts#L43-L54、code/vitest.helpers.ts、code/vitest.helpers.ts#L1-L37、code/vitest.helpers.ts#L18-L37、code/vitest.shared.ts、code/vitest.shared.ts#L1-L16、code/vitest.shared.ts#L5-L15
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L107-L116、preview.tsx、preview.tsx#L1-L416、preview.tsx#L144-L193、preview.tsx#L195-L334、preview.tsx#L402-L415、preview.tsx#L413-L415

### 项目概述/核心特性/测试支持功能/组件测试.md

- reference 标题：组件测试
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.03
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L107-L116、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L37-L45、code/.storybook/preview.tsx#L402-L416、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L1-L30、code/.storybook/storybook.setup.ts#L11-L29、code/addons/vitest/vitest.config.ts、code/addons/vitest/vitest.config.ts#L1-L11、code/e2e-tests/component-tests.spec.ts、code/e2e-tests/component-tests.spec.ts#L1-L189、code/e2e-tests/component-tests.spec.ts#L17-L24、code/e2e-tests/component-tests.spec.ts#L9-L146、code/playwright.config.ts、code/playwright.config.ts#L1-L115、code/vitest-setup.ts、code/vitest-setup.ts#L1-L123、code/vitest-setup.ts#L1-L2、code/vitest-setup.ts#L43-L72、code/vitest-setup.ts#L74-L122、code/vitest.config.storybook.ts、code/vitest.config.storybook.ts#L1-L56、code/vitest.config.storybook.ts#L21-L55、code/vitest.config.storybook.ts#L8-L19、code/vitest.config.ts、code/vitest.config.ts#L1-L57、code/vitest.config.ts#L12-L13、code/vitest.config.ts#L15-L31、code/vitest.config.ts#L39-L41、code/vitest.config.ts#L43-L54、code/vitest.config.ts#L45-L54、code/vitest.shared.ts、code/vitest.shared.ts#L1-L16
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L107-L116、preview.tsx、preview.tsx#L1-L416、preview.tsx#L144-L193、preview.tsx#L37-L45、preview.tsx#L402-L416、storybook.setup.ts

### 项目概述/核心特性/组件开发环境/交互式调试.md

- reference 标题：交互式调试
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.07
- key_source_coverage：0.05
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L19-L183、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L220-L301、code/.storybook/preview.tsx#L309-L333、code/.storybook/preview.tsx#L336-L400、code/core/src/actions/components/ActionLogger/index.tsx、code/core/src/actions/components/ActionLogger/index.tsx#L40-L81、code/core/src/actions/containers/ActionLogger/index.tsx、code/core/src/actions/containers/ActionLogger/index.tsx#L32-L77、code/core/src/actions/manager.tsx、code/core/src/actions/manager.tsx#L1-L18、code/core/src/actions/manager.tsx#L9-L17、code/core/src/controls/constants.ts、code/core/src/controls/constants.ts#L1-L3、code/core/src/manager/components/panel/Panel.tsx、code/core/src/manager/components/panel/Panel.tsx#L110-L154
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L149-L181、main.ts#L19-L183、manager.tsx、manager.tsx#L1-L9、preview.tsx、preview.tsx#L1-L416、preview.tsx#L220-L301

### 项目概述/核心特性/组件开发环境/实时预览.md

- reference 标题：实时预览
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.02
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L169-L173、code/.storybook/main.ts#L19-L183、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L157-L192、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L195-L415、code/.storybook/preview.tsx#L402-L415、code/addons/docs/src/blocks/components/Story.tsx、code/addons/docs/src/blocks/components/Story.tsx#L43-L67、code/addons/docs/src/blocks/components/Story.tsx#L43-L88、code/addons/docs/src/blocks/components/Story.tsx#L62-L67、code/core/src/channels/postmessage/getEventSourceUrl.ts、code/core/src/channels/postmessage/getEventSourceUrl.ts#L3-L52、code/core/src/channels/postmessage/index.ts、code/core/src/channels/postmessage/index.ts#L131-L137、code/core/src/channels/postmessage/index.ts#L194-L194、code/core/src/channels/postmessage/index.ts#L29-L194、code/core/src/channels/postmessage/index.ts#L49-L129、code/core/src/channels/postmessage/index.ts#L66-L129、code/core/src/channels/postmessage/index.ts#L92-L109、code/core/src/components/components/Zoom/ZoomIFrame.tsx、code/core/src/components/components/Zoom/ZoomIFrame.tsx#L38-L65、code/core/src/manager-api/modules/provider.ts、code/core/src/manager-api/modules/provider.ts#L9-L17、code/core/src/preview-api/modules/preview-web/Preview.tsx、code/core/src/preview-api/modules/preview-web/Preview.tsx#L422-L424、code/core/src/preview-api/modules/preview-web/Preview.tsx#L422-L468、code/core/src/preview-api/modules/preview-web/Preview.tsx#L431-L460、code/core/src/preview-api/modules/preview-web/render/CsfDocsRender.ts、code/core/src/preview-api/modules/preview-web/render/CsfDocsRender.ts#L33-L97、code/core/src/preview-api/modules/preview-web/render/CsfDocsRender.ts#L63-L89、code/core/src/preview-api/modules/store/StoryStore.ts、code/core/src/preview-api/modules/store/StoryStore.ts#L123-L242、code/core/src/preview-api/modules/store/StoryStore.ts#L140-L146、code/core/src/preview-api/modules/store/StoryStore.ts#L148-L159、code/core/src/preview-api/modules/store/StoryStore.ts#L228-L242、code/renderers/html/src/render.ts、code/renderers/html/src/render.ts#L12-L42、code/renderers/react/src/preview.tsx、code/renderers/react/src/preview.tsx#L55-L85
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L149-L181、main.ts#L169-L173、main.ts#L19-L183、manager.tsx、manager.tsx#L1-L9、manager.tsx#L4-L8、preview.tsx

### 项目概述/核心特性/组件开发环境/开发服务器.md

- reference 标题：开发服务器
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.00
- missing_key_sources：code/builders/builder-vite/src/index.ts、code/builders/builder-vite/src/index.ts#L1-L68、code/builders/builder-vite/src/index.ts#L19-L33、code/builders/builder-vite/src/index.ts#L19-L61、code/builders/builder-vite/src/index.ts#L41-L61、code/builders/builder-vite/src/logger.ts、code/builders/builder-vite/src/logger.ts#L1-L27、code/builders/builder-vite/src/vite-config.ts、code/builders/builder-vite/src/vite-config.ts#L1-L94、code/builders/builder-vite/src/vite-config.ts#L38-L78、code/builders/builder-vite/src/vite-server.ts、code/builders/builder-vite/src/vite-server.ts#L1-L47、code/builders/builder-vite/src/vite-server.ts#L16-L38、code/builders/builder-vite/src/vite-server.ts#L9-L46、code/builders/builder-webpack5/src/index.ts、code/builders/builder-webpack5/src/index.ts#L1-L349、code/builders/builder-webpack5/src/index.ts#L121-L227、code/builders/builder-webpack5/src/index.ts#L181-L200、code/builders/builder-webpack5/src/preview/iframe-webpack.config.ts、code/builders/builder-webpack5/src/preview/iframe-webpack.config.ts#L231-L273、code/core/src/common/utils/file-cache.ts、code/core/src/common/utils/file-cache.ts#L52-L162、code/core/src/core-server/dev-server.ts、code/core/src/core-server/dev-server.ts#L113-L137、code/core/src/core-server/dev-server.ts#L25-L190、code/core/src/core-server/dev-server.ts#L31-L157、code/core/src/core-server/dev-server.ts#L74-L88、code/core/src/core-server/dev-server.ts#L84-L142、code/core/src/core-server/dev-server.ts#L84-L88、code/core/src/core-server/utils/get-caching-middleware.ts、code/core/src/core-server/utils/get-caching-middleware.ts#L1-L9、code/core/src/core-server/utils/middleware.ts、code/core/src/core-server/utils/middleware.ts#L1-L18、code/renderers/react/src/componentManifest/utils.ts、code/renderers/react/src/componentManifest/utils.ts#L37-L72、docs/_snippets/storybook-builder-api-configuration-options.md、docs/_snippets/storybook-builder-api-configuration-options.md#L1-L35、docs/_snippets/storybook-builder-api-dev-server.md、docs/_snippets/storybook-builder-api-dev-server.md#L1-L21、docs/_snippets/storybook-builder-api-interface.md、docs/_snippets/storybook-builder-api-interface.md#L1-L24、docs/_snippets/storybook-builder-api-interface.md#L2-L22、docs/_snippets/storybook-builder-api-shutdown-server.md、docs/_snippets/storybook-builder-api-shutdown-server.md#L1-L19、docs/builders/webpack.mdx、docs/builders/webpack.mdx#L14-L25
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：index.ts、index.ts#L1-L68、index.ts#L19-L33、index.ts#L19-L61、index.ts#L41-L61、logger.ts、logger.ts#L1-L27、vite-config.ts

### 项目概述/核心特性/组件开发环境/组件开发环境.md

- reference 标题：组件开发环境
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.13
- key_source_coverage：0.03
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L169-L181、code/.storybook/main.ts#L174-L178、code/.storybook/main.ts#L19-L186、code/.storybook/main.ts#L20-L106、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L195-L415、code/.storybook/preview.tsx#L220-L301、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L1-L30、code/.storybook/storybook.setup.ts#L16-L29、code/addons/a11y/template/tests.stories.ts、code/addons/a11y/template/tests.stories.ts#L1-L81、code/core/src/components/components/Button/Button.stories.tsx、code/core/src/components/components/Button/Button.stories.tsx#L1-L409、code/core/src/components/components/Button/Button.stories.tsx#L11-L145、code/core/src/components/components/Button/Button.stories.tsx#L11-L24、code/core/src/shared/constants/environments-support.ts、code/core/src/shared/constants/environments-support.ts#L1-L21、code/core/src/shared/constants/environments-support.ts#L4-L11、code/frameworks/react-vite/src/index.ts、code/frameworks/react-vite/src/index.ts#L1-L5、code/frameworks/react-vite/src/types.ts、code/frameworks/react-vite/src/types.ts#L1-L74、code/package.json、code/package.json#L15-L44、code/renderers/react/src/index.ts、code/renderers/react/src/index.ts#L1-L10
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L149-L181、main.ts#L169-L181、main.ts#L174-L178、main.ts#L19-L186、main.ts#L20-L106、manager.tsx、manager.tsx#L1-L9

### 项目概述/核心特性/组件开发环境/组件生命周期.md

- reference 标题：组件生命周期
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.07
- key_source_coverage：0.03
- missing_key_sources：code/core/src/channels/README.md、code/core/src/channels/README.md#L1-L33、code/core/src/channels/postmessage/index.ts、code/core/src/channels/postmessage/index.ts#L1-L240、code/core/src/channels/postmessage/index.ts#L194-L238、code/core/src/channels/postmessage/index.ts#L214-L238、code/core/src/channels/postmessage/index.ts#L29-L47、code/core/src/channels/postmessage/index.ts#L66-L129、code/core/src/manager-api/store.ts、code/core/src/manager-api/store.ts#L1-L123、code/core/src/manager-api/store.ts#L48-L122、code/core/src/preview-api/modules/addons/hooks.ts、code/core/src/preview-api/modules/addons/hooks.ts#L1-L25、code/core/src/preview-api/modules/addons/hooks.ts#L1-L660、code/core/src/preview-api/modules/addons/hooks.ts#L102-L117、code/core/src/preview-api/modules/addons/hooks.ts#L131-L215、code/core/src/preview-api/modules/addons/hooks.ts#L184-L215、code/core/src/preview-api/modules/addons/hooks.ts#L206-L211、code/core/src/preview-api/modules/addons/hooks.ts#L220-L224、code/core/src/preview-api/modules/addons/hooks.ts#L36-L129、code/core/src/preview-api/modules/addons/hooks.ts#L371-L422、code/core/src/preview-api/modules/addons/hooks.ts#L498-L504、code/core/src/preview-api/modules/addons/hooks.ts#L59-L128、code/core/src/preview-api/modules/addons/hooks.ts#L614-L633、code/core/src/preview-api/modules/preview-web/render/StoryRender.ts、code/core/src/preview-api/modules/preview-web/render/StoryRender.ts#L255-L310、code/core/src/shared/universal-store/index.ts、code/core/src/shared/universal-store/index.ts#L1-L695、code/core/src/shared/universal-store/index.ts#L374-L419、code/core/src/shared/universal-store/index.ts#L426-L482、code/core/src/shared/universal-store/index.ts#L507-L554、code/core/src/shared/universal-store/index.ts#L538-L553、code/core/src/shared/universal-store/index.ts#L82-L134、code/core/src/shared/universal-store/types.ts、code/core/src/shared/universal-store/types.ts#L1-L78、code/core/src/shared/universal-store/types.ts#L23-L63、code/frameworks/angular/src/client/angular-beta/StorybookWrapperComponent.ts、code/frameworks/angular/src/client/angular-beta/StorybookWrapperComponent.ts#L133-L151
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：README.md、README.md#L1-L33、index.ts、index.ts#L1-L240、index.ts#L194-L238、index.ts#L214-L238、index.ts#L29-L47、index.ts#L66-L129

### 项目概述/项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=5
- reuse_count：62
- skeleton_score：0.07
- key_source_coverage：0.09
- missing_key_sources：CONTRIBUTING.md#L1-L243、CONTRIBUTING.md#L220-L239、CONTRIBUTING.md#L53-L96、README.md、README.md#L47-L94、README.md#L77-L94、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L19-L133、code/addons/a11y/package.json、code/addons/a11y/package.json#L37-L52、code/addons/a11y/package.json#L60-L63、code/addons/docs/package.json、code/addons/docs/package.json#L40-L80、code/addons/docs/package.json#L88-L96、code/core/package.json、code/core/package.json#L230-L243、code/core/package.json#L48-L221、code/frameworks/react-vite/package.json、code/frameworks/react-vite/package.json#L28-L45、code/frameworks/react-vite/package.json#L54-L64、code/package.json、code/package.json#L15-L45、code/package.json#L71-L106、code/renderers/react/package.json、code/renderers/react/package.json#L23-L45、code/renderers/react/package.json#L54-L59、docs/index.mdx#L1-L37、docs/index.mdx#L8-L14、package.json、package.json#L40-L60、package.json#L5-L16
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CONTRIBUTING.md#L1-L243、CONTRIBUTING.md#L220-L239、CONTRIBUTING.md#L53-L96、README.md、README.md#L47-L94、README.md#L77-L94、main.ts#L149-L181、main.ts#L19-L133

### 高级功能/工具集成/CI_CD集成.md

- reference 标题：CI/CD集成
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/工具集成/Codemod工具.md

- reference 标题：Codemod工具
- 生成页：核心模块/codemod.md（codemod）
- KnowledgeUnit：unit_id=unit-32fb8f61cd90, unit_type=module_doc, domain_id=domain-8984fdaabf39, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.17
- key_source_coverage：0.06
- missing_key_sources：code/lib/cli-storybook/src/bin/run.ts、code/lib/cli-storybook/src/bin/run.ts#L220-L240、code/lib/cli-storybook/src/codemod/csf-factories.ts、code/lib/cli-storybook/src/codemod/csf-factories.ts#L1-L145、code/lib/cli-storybook/src/codemod/csf-factories.ts#L17-L145、code/lib/cli-storybook/src/codemod/helpers/config-to-csf-factory.ts、code/lib/cli-storybook/src/codemod/helpers/csf-factories-utils.ts、code/lib/cli-storybook/src/codemod/helpers/remove-unused-types.ts、code/lib/cli-storybook/src/codemod/helpers/story-to-csf-factory.ts、code/lib/cli-storybook/src/codemod/helpers/story-to-csf-factory.ts#L1-L520、code/lib/cli-storybook/src/codemod/helpers/story-to-csf-factory.ts#L234-L426、code/lib/cli-storybook/src/codemod/helpers/story-to-csf-factory.ts#L26-L518、code/lib/cli-storybook/src/migrate.ts、code/lib/cli-storybook/src/migrate.ts#L1-L25、code/lib/cli-storybook/src/migrate.ts#L16-L24、code/lib/codemod/README.md、code/lib/codemod/README.md#L1-L112、code/lib/codemod/README.md#L51-L112、code/lib/codemod/README.md#L6-L21、code/lib/codemod/build-config.ts、code/lib/codemod/build-config.ts#L1-L34、code/lib/codemod/build-config.ts#L3-L32、code/lib/codemod/package.json、code/lib/codemod/package.json#L1-L64、code/lib/codemod/package.json#L24-L35、code/lib/codemod/src/index.test.ts#L29-L52、code/lib/codemod/src/index.ts#L1-L116、code/lib/codemod/src/index.ts#L16-L116、code/lib/codemod/src/index.ts#L28-L116、code/lib/codemod/src/index.ts#L38-L106、code/lib/codemod/src/lib/utils.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：run.ts、run.ts#L220-L240、csf-factories.ts、csf-factories.ts#L1-L145、csf-factories.ts#L17-L145、config-to-csf-factory.ts、csf-factories-utils.ts、remove-unused-types.ts

### 高级功能/工具集成/ESLint集成.md

- reference 标题：ESLint集成
- 生成页：插件生态/eslint-plugin.md（eslint-plugin）
- KnowledgeUnit：unit_id=unit-15ada7f1fc82, unit_type=module_doc, domain_id=domain-006c7002a0fc, readiness=compose_ready, child_digests=0
- reuse_count：1
- skeleton_score：0.10
- key_source_coverage：0.13
- missing_key_sources：code/core/.eslintrc.json、code/core/.eslintrc.json#L1-L7、code/lib/eslint-plugin/package.json、code/lib/eslint-plugin/package.json#L1-L73、code/lib/eslint-plugin/package.json#L45-L67、code/lib/eslint-plugin/package.json#L64-L67、code/lib/eslint-plugin/src/configs/csf.ts#L6-L29、code/lib/eslint-plugin/src/configs/recommended.ts#L10-L11、code/lib/eslint-plugin/src/configs/recommended.ts#L10-L31、code/lib/eslint-plugin/src/configs/recommended.ts#L6-L34、code/lib/eslint-plugin/src/configs/recommended.ts#L8-L32、code/lib/eslint-plugin/src/index.ts#L1-L70、code/lib/eslint-plugin/src/index.ts#L10-L14、code/lib/eslint-plugin/src/index.ts#L33-L69、code/lib/eslint-plugin/src/index.ts#L7-L14、code/lib/eslint-plugin/src/rules/await-interactions.ts、code/lib/eslint-plugin/src/rules/await-interactions.ts#L169-L217、code/lib/eslint-plugin/src/rules/await-interactions.ts#L27-L220、code/lib/eslint-plugin/src/rules/await-interactions.ts#L27-L44、code/lib/eslint-plugin/src/rules/csf-component.ts#L16-L30、code/lib/eslint-plugin/src/rules/csf-component.ts#L16-L70、code/lib/eslint-plugin/src/rules/story-exports.ts、code/lib/eslint-plugin/src/rules/story-exports.ts#L23-L113、code/lib/eslint-plugin/src/rules/story-exports.ts#L23-L41、code/lib/eslint-plugin/src/rules/story-exports.ts#L81-L109、code/lib/eslint-plugin/src/rules/use-storybook-expect.ts、code/lib/eslint-plugin/src/rules/use-storybook-expect.ts#L19-L35、code/lib/eslint-plugin/src/rules/use-storybook-expect.ts#L19-L94
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：.eslintrc.json、.eslintrc.json#L1-L7、package.json、package.json#L1-L73、package.json#L45-L67、package.json#L64-L67、csf.ts#L6-L29、recommended.ts#L10-L11

### 高级功能/工具集成/IDE配置优化.md

- reference 标题：IDE配置优化
- 生成页：配置参考/主题和外观.md（主题和外观）
- KnowledgeUnit：unit_id=unit-bebeb3781961, unit_type=config_doc, domain_id=domain-e15151050742, readiness=compose_ready, child_digests=3
- reuse_count：6
- skeleton_score：0.13
- key_source_coverage：0.07
- missing_key_sources：.editorconfig、.editorconfig#L1-L12、code/.eslintrc.js、code/.eslintrc.js#L1-L234、code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L149-L181、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/storybook.setup.ts、code/.storybook/storybook.setup.ts#L1-L200、code/tsconfig.json#L1-L26、code/vitest.config.storybook.ts、code/vitest.config.storybook.ts#L1-L56、code/vitest.config.ts、code/vitest.config.ts#L1-L57、package.json#L1-L74、prettier.config.mjs、prettier.config.mjs#L1-L3、scripts/prettier.config.js、scripts/prettier.config.js#L1-L57、storybook.code-workspace、storybook.code-workspace#L1-L22
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：.editorconfig、.editorconfig#L1-L12、.eslintrc.js、.eslintrc.js#L1-L234、main.ts、main.ts#L1-L186、main.ts#L149-L181、manager.tsx

### 高级功能/工具集成/工具集成.md

- reference 标题：工具集成
- 生成页：多框架支持/react-vitest-3.md（react-vitest-3）
- KnowledgeUnit：unit_id=unit-ed51639dbf62, unit_type=module_doc, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=0
- reuse_count：6
- skeleton_score：0.11
- key_source_coverage：0.04
- missing_key_sources：.circleci/config.yml、.github/workflows、code/.eslintrc.js、code/.eslintrc.js#L12-L14、code/.eslintrc.js#L5-L15、code/.eslintrc.js#L65-L232、code/.eslintrc.js#L84-L92、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L19-L183、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L144-L415、code/.storybook/preview.tsx#L195-L334、code/.storybook/preview.tsx#L195-L415、code/.storybook/preview.tsx#L336-L400、code/.storybook/preview.tsx#L402-L415、code/lib/codemod/package.json、code/lib/codemod/package.json#L24-L35、code/package.json、code/package.json#L15-L45、code/package.json#L74-L150、code/prettier.config.mjs、code/prettier.config.mjs#L1-L3、code/tsconfig.json、code/tsconfig.json#L3-L23、package.json、package.json#L5-L17、scripts/.eslintrc.cjs、scripts/.eslintrc.cjs#L1-L26、scripts/.eslintrc.cjs#L74-L100、scripts/package.json、scripts/package.json#L53-L177、scripts/package.json#L6-L44、scripts/prettier.config.js、scripts/prettier.config.js#L1-L57、scripts/prettier.config.js#L36-L48、scripts/snippets/codemod.ts、scripts/snippets/codemod.ts#L1-L375、scripts/snippets/codemod.ts#L248-L289、scripts/snippets/codemod.ts#L291-L344、scripts/snippets/codemod.ts#L50-L246、scripts/tsconfig.json、scripts/tsconfig.json#L3-L29
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：config.yml、workflows、.eslintrc.js、.eslintrc.js#L12-L14、.eslintrc.js#L5-L15、.eslintrc.js#L65-L232、.eslintrc.js#L84-L92、main.ts#L149-L181

### 高级功能/工具集成/构建工具优化.md

- reference 标题：构建工具优化
- 生成页：构建系统/builder-webpack5.md（builder-webpack5）
- KnowledgeUnit：unit_id=unit-e23b550d9f21, unit_type=module_doc, domain_id=domain-f0fd06748cfc, readiness=compose_ready, child_digests=0
- reuse_count：3
- skeleton_score：0.13
- key_source_coverage：0.07
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L149-L181、code/.storybook/main.ts#L169-L173、code/builders/builder-vite/package.json、code/builders/builder-vite/package.json#L1-L73、code/builders/builder-vite/package.json#L49-L67、code/builders/builder-vite/src/index.ts#L1-L68、code/builders/builder-vite/src/index.ts#L54-L60、code/builders/builder-vite/src/vite-config.ts、code/builders/builder-vite/src/vite-config.ts#L1-L94、code/builders/builder-vite/src/vite-config.ts#L37-L78、code/builders/builder-vite/src/vite-config.ts#L37-L93、code/builders/builder-vite/src/vite-config.ts#L66-L73、code/builders/builder-vite/src/vite-config.ts#L80-L93、code/builders/builder-vite/src/vite-server.ts、code/builders/builder-vite/src/vite-server.ts#L1-L47、code/builders/builder-vite/src/vite-server.ts#L16-L38、code/builders/builder-vite/src/vite-server.ts#L32-L38、code/builders/builder-vite/src/vite-server.ts#L9-L46、code/builders/builder-webpack5/package.json、code/builders/builder-webpack5/package.json#L1-L93、code/builders/builder-webpack5/package.json#L54-L69、code/builders/builder-webpack5/src/index.ts#L1-L349、code/builders/builder-webpack5/src/index.ts#L121-L227、code/builders/builder-webpack5/src/index.ts#L121-L318、code/builders/builder-webpack5/src/index.ts#L132-L134、code/builders/builder-webpack5/src/index.ts#L151-L179、code/builders/builder-webpack5/src/index.ts#L181-L200、code/builders/builder-webpack5/src/index.ts#L187-L200、code/builders/builder-webpack5/src/index.ts#L303-L318、code/builders/builder-webpack5/src/preview/virtual-module-mapping.ts#L1-L85、code/builders/builder-webpack5/src/preview/virtual-module-mapping.ts#L19-L84、code/core/src/shared/constants/environments-support.ts、code/core/src/shared/constants/environments-support.ts#L1-L21、scripts/.babelrc.cjs、scripts/.babelrc.cjs#L1-L17、scripts/build/utils/generate-bundle.ts、scripts/build/utils/generate-bundle.ts#L1-L270、scripts/build/utils/generate-bundle.ts#L45-L62
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L1-L186、main.ts#L149-L181、main.ts#L169-L173、package.json、package.json#L1-L73、package.json#L49-L67、index.ts#L1-L68

### 高级功能/性能监控.md

- reference 标题：性能监控
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/扩展开发/Addon开发.md

- reference 标题：Addon开发
- 生成页：配置参考/主题和外观.md（主题和外观）
- KnowledgeUnit：unit_id=unit-bebeb3781961, unit_type=config_doc, domain_id=domain-e15151050742, readiness=compose_ready, child_digests=3
- reuse_count：6
- skeleton_score：0.11
- key_source_coverage：0.21
- missing_key_sources：code/addons/a11y/package.json#L37-L52、code/addons/a11y/package.json#L75-L77、code/addons/a11y/src/index.ts、code/addons/a11y/src/index.ts#L1-L11、code/addons/docs/package.json#L118-L120、code/addons/docs/package.json#L40-L80、code/addons/docs/src/preview.ts、code/addons/docs/src/preview.ts#L1-L30、code/addons/links/package.json#L29-L47、code/addons/links/package.json#L61-L64、code/addons/links/src/preview.ts、code/addons/links/src/preview.ts#L1-L4、code/addons/themes/package.json#L36-L50、code/addons/themes/package.json#L67-L69、code/addons/themes/src/preview.ts、code/addons/themes/src/preview.ts#L1-L8、code/addons/vitest/package.json#L107-L113、code/addons/vitest/package.json#L40-L66、code/addons/vitest/src/index.ts、code/addons/vitest/src/index.ts#L1-L4、code/core/package.json#L230-L243、code/core/package.json#L48-L221、code/core/src/manager-api/index.ts、code/core/src/manager-api/index.ts#L1-L27、code/core/src/preview-api/index.ts、code/core/src/preview-api/index.ts#L1-L90、code/lib/cli-storybook/package.json#L29-L32
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json#L37-L52、package.json#L75-L77、index.ts、index.ts#L1-L11、package.json#L118-L120、package.json#L40-L80、preview.ts、preview.ts#L1-L30

### 高级功能/扩展开发/主题开发.md

- reference 标题：主题开发
- 生成页：主题系统/themes.md（themes）
- KnowledgeUnit：unit_id=unit-e858919ec1df, unit_type=module_doc, domain_id=domain-9cda34dbf62b, readiness=compose_ready, child_digests=0
- reuse_count：6
- skeleton_score：0.19
- key_source_coverage：0.11
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/manager.tsx#L1-L9、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L195-L301、code/.storybook/preview.tsx#L220-L301、code/addons/docs/src/blocks/components/ColorPalette.stories.tsx、code/addons/docs/src/blocks/components/ColorPalette.stories.tsx#L1-L79、code/addons/themes/src/decorators/class-name.decorator.tsx#L34-L56、code/addons/themes/src/index.ts#L1-L11、code/core/src/components/brand/colorpalette.mdx、code/core/src/components/brand/colorpalette.mdx#L39-L96、code/core/src/theming/base.ts、code/core/src/theming/base.ts#L1-L168、code/core/src/theming/convert.ts、code/core/src/theming/convert.ts#L1-L206、code/core/src/theming/convert.ts#L36-L205、code/core/src/theming/convert.ts#L78-L205、code/core/src/theming/create.ts、code/core/src/theming/create.ts#L1-L51、code/core/src/theming/create.ts#L29-L50、code/core/src/theming/index.ts#L1-L51、code/core/src/theming/tests/convert.test.js、code/core/src/theming/tests/convert.test.js#L1-L65、code/core/src/theming/tests/create.test.js、code/core/src/theming/tests/create.test.js#L1-L45、code/core/src/theming/utils.ts、code/core/src/theming/utils.ts#L1-L79、code/core/src/theming/utils.ts#L24-L59、code/core/src/theming/utils.ts#L44-L59
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L1-L186、manager.tsx#L1-L9、preview.tsx、preview.tsx#L1-L416、preview.tsx#L195-L301、preview.tsx#L220-L301、ColorPalette.stories.tsx

### 高级功能/扩展开发/扩展开发.md

- reference 标题：扩展开发
- 生成页：配置参考/主题和外观.md（主题和外观）
- KnowledgeUnit：unit_id=unit-bebeb3781961, unit_type=config_doc, domain_id=domain-e15151050742, readiness=compose_ready, child_digests=3
- reuse_count：6
- skeleton_score：0.07
- key_source_coverage：0.22
- missing_key_sources：code/.storybook/main.ts、code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L107-L116、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L402-L415、code/addons/a11y/package.json#L37-L52、code/addons/a11y/package.json#L75-L77、code/addons/docs/package.json#L118-L120、code/addons/docs/package.json#L40-L80、code/addons/links/package.json#L29-L47、code/addons/links/package.json#L61-L64、code/addons/onboarding/package.json#L33-L42、code/addons/onboarding/package.json#L58-L60、code/addons/themes/package.json#L36-L50、code/addons/themes/package.json#L67-L69、code/core/package.json#L1-L389、code/core/package.json#L48-L221、code/core/package.json#L71-L75
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts、main.ts#L1-L186、main.ts#L107-L116、manager.tsx、manager.tsx#L1-L9、preview.tsx、preview.tsx#L1-L416、preview.tsx#L402-L415

### 高级功能/扩展开发/构建器扩展.md

- reference 标题：构建器扩展
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/扩展开发/渲染器扩展.md

- reference 标题：渲染器扩展
- 生成页：多框架支持/多框架支持.md（多框架支持）
- KnowledgeUnit：unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- reuse_count：14
- skeleton_score：0.07
- key_source_coverage：0.11
- missing_key_sources：code/core/src/common/utils/get-renderer-name.ts、code/core/src/common/utils/get-renderer-name.ts#L1-L47、code/core/src/common/utils/get-renderer-name.ts#L11-L21、code/core/src/types/modules/renderers.ts、code/core/src/types/modules/renderers.ts#L1-L16、code/frameworks/angular/src/client/angular-beta/CanvasRenderer.ts、code/frameworks/angular/src/client/angular-beta/CanvasRenderer.ts#L1-L18、code/frameworks/angular/src/client/angular-beta/CanvasRenderer.ts#L15-L17、code/frameworks/angular/src/client/angular-beta/RendererFactory.ts、code/frameworks/angular/src/client/angular-beta/RendererFactory.ts#L1-L42、code/frameworks/angular/src/client/angular-beta/RendererFactory.ts#L11-L18、code/frameworks/angular/src/client/angular-beta/RendererFactory.ts#L20-L34、code/frameworks/angular/src/client/angular-beta/RendererFactory.ts#L28-L34、code/frameworks/angular/src/client/render.ts#L1-L27、code/frameworks/angular/src/renderer.ts、code/frameworks/angular/src/renderer.ts#L1-L7、code/lib/cli-storybook/src/automigrate/fixes/renderer-to-framework.ts、code/lib/eslint-plugin/src/rules/no-renderer-packages.ts、code/renderers/html/src/docs/sourceDecorator.ts、code/renderers/html/src/globals.ts、code/renderers/html/src/index.ts#L1-L5、code/renderers/html/src/portable-stories.ts、code/renderers/html/src/public-types.ts、code/renderers/html/src/render.ts#L1-L71、code/renderers/html/src/render.ts#L12-L42、code/renderers/html/src/render.ts#L12-L70、code/renderers/html/src/render.ts#L37-L42、code/renderers/html/src/render.ts#L44-L70、code/renderers/html/src/render.ts#L53-L69、code/renderers/server/src/entry-preview.ts、code/renderers/server/src/entry-preview.ts#L1-L3、code/renderers/web-components/src/entry-preview.ts、code/renderers/web-components/src/entry-preview.ts#L1-L5
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：get-renderer-name.ts、get-renderer-name.ts#L1-L47、get-renderer-name.ts#L11-L21、renderers.ts、renderers.ts#L1-L16、CanvasRenderer.ts、CanvasRenderer.ts#L1-L18、CanvasRenderer.ts#L15-L17

### 高级功能/扩展开发/预设开发.md

- reference 标题：预设开发
- 生成页：多框架支持/多框架支持.md（多框架支持）
- KnowledgeUnit：unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- reuse_count：14
- skeleton_score：0.00
- key_source_coverage：0.30
- missing_key_sources：code/.storybook/main.ts#L19-L185、code/core/src/core-server/build-static.ts、code/core/src/core-server/build-static.ts#L35-L128、code/core/src/core-server/build-static.ts#L70-L128、code/core/src/core-server/build-static.ts#L74-L106、code/core/src/core-server/load.ts、code/core/src/core-server/load.ts#L18-L103、code/core/src/core-server/load.ts#L28-L32、code/core/src/core-server/load.ts#L46-L103、code/core/src/core-server/load.ts#L55-L93、code/core/src/csf-tools/ConfigFile.ts、code/core/src/csf-tools/ConfigFile.ts#L453-L541、code/lib/core-webpack/src/merge-webpack-config.ts、code/lib/core-webpack/src/merge-webpack-config.ts#L59-L81、code/lib/core-webpack/src/merge-webpack-config.ts#L69-L81、code/presets/create-react-app/package.json、code/presets/create-react-app/package.json#L23-L27、code/presets/react-webpack/package.json、code/presets/react-webpack/package.json#L23-L33
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L19-L185、build-static.ts、build-static.ts#L35-L128、build-static.ts#L70-L128、build-static.ts#L74-L106、load.ts、load.ts#L18-L103、load.ts#L28-L32

### 高级功能/自定义渲染器.md

- reference 标题：自定义渲染器
- 生成页：平台绑定-Web/web-components.md（web-components）
- KnowledgeUnit：unit_id=unit-e7060a424501, unit_type=module_doc, domain_id=domain-c86d32da5da8, readiness=compose_ready, child_digests=0
- reuse_count：4
- skeleton_score：0.12
- key_source_coverage：0.17
- missing_key_sources：code/renderers/html/package.json、code/renderers/html/package.json#L27-L36、code/renderers/html/src/index.ts#L1-L5、code/renderers/preact/package.json、code/renderers/preact/package.json#L27-L36、code/renderers/preact/package.json#L54-L57、code/renderers/preact/src/index.ts#L1-L5、code/renderers/react/package.json、code/renderers/react/package.json#L23-L45、code/renderers/react/package.json#L81-L86、code/renderers/react/src/index.ts#L1-L10、code/renderers/react/src/preview.tsx、code/renderers/react/src/preview.tsx#L102-L138、code/renderers/react/src/preview.tsx#L147-L239、code/renderers/react/src/preview.tsx#L18-L21、code/renderers/react/src/preview.tsx#L24-L35、code/renderers/react/src/preview.tsx#L249-L255、code/renderers/react/src/preview.tsx#L55-L85、code/renderers/server/package.json、code/renderers/server/package.json#L26-L34、code/renderers/server/src/index.ts#L1-L4、code/renderers/svelte/package.json、code/renderers/svelte/package.json#L26-L44、code/renderers/svelte/package.json#L68-L71、code/renderers/svelte/src/index.ts#L1-L5、code/renderers/vue3/package.json、code/renderers/vue3/package.json#L27-L42、code/renderers/vue3/package.json#L63-L66、code/renderers/vue3/src/index.ts#L1-L10、code/renderers/web-components/package.json、code/renderers/web-components/package.json#L29-L39、code/renderers/web-components/package.json#L64-L67、code/renderers/web-components/src/index.ts#L1-L36、code/renderers/web-components/src/index.ts#L14-L35
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：package.json、package.json#L27-L36、index.ts#L1-L5、package.json、package.json#L27-L36、package.json#L54-L57、index.ts#L1-L5、package.json

### 高级功能/预设配置.md

- reference 标题：预设配置
- 生成页：多框架支持/多框架支持.md（多框架支持）
- KnowledgeUnit：unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- reuse_count：14
- skeleton_score：0.00
- key_source_coverage：0.06
- missing_key_sources：code/core/src/common/presets.ts、code/core/src/common/presets.ts#L129-L133、code/core/src/common/presets.ts#L156-L243、code/core/src/common/presets.ts#L156-L328、code/core/src/common/presets.ts#L156-L357、code/core/src/common/presets.ts#L245-L328、code/core/src/common/presets.ts#L265-L316、code/core/src/common/presets.ts#L318-L357、code/core/src/common/presets.ts#L348-L354、code/core/src/common/presets.ts#L35-L55、code/presets/create-react-app/package.json、code/presets/create-react-app/package.json#L1-L55、code/presets/create-react-app/package.json#L46-L49、code/presets/create-react-app/preset.js、code/presets/create-react-app/preset.js#L1-L2、code/presets/create-react-app/src/index.ts#L1-L153、code/presets/create-react-app/src/index.ts#L101-L108、code/presets/create-react-app/src/index.ts#L45-L146、code/presets/create-react-app/src/index.ts#L64-L74、code/presets/react-webpack/package.json、code/presets/react-webpack/package.json#L23-L33、code/presets/react-webpack/package.json#L57-L66、code/presets/react-webpack/src/index.ts#L1-L11、code/presets/react-webpack/src/index.ts#L7-L10、code/presets/react-webpack/src/types.ts、code/presets/react-webpack/src/types.ts#L1-L44、code/presets/react-webpack/src/types.ts#L10-L44、code/presets/react-webpack/src/types.ts#L24-L38、code/presets/server-webpack/package.json、code/presets/server-webpack/package.json#L1-L58、code/presets/server-webpack/package.json#L23-L31
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：presets.ts、presets.ts#L129-L133、presets.ts#L156-L243、presets.ts#L156-L328、presets.ts#L156-L357、presets.ts#L245-L328、presets.ts#L265-L316、presets.ts#L318-L357

### 高级功能/高级功能.md

- reference 标题：高级功能
- 生成页：多框架支持/多框架支持.md（多框架支持）
- KnowledgeUnit：unit_id=unit-13344a044882, unit_type=domain_index, domain_id=domain-6ae554b8afb1, readiness=compose_ready, child_digests=17
- reuse_count：14
- skeleton_score：0.00
- key_source_coverage：0.09
- missing_key_sources：code/.storybook/main.ts#L1-L186、code/.storybook/main.ts#L107-L116、code/.storybook/main.ts#L129-L132、code/.storybook/main.ts#L133-L180、code/.storybook/main.ts#L169-L180、code/.storybook/main.ts#L19-L132、code/.storybook/manager.tsx、code/.storybook/manager.tsx#L1-L9、code/.storybook/manager.tsx#L4-L8、code/.storybook/preview.tsx、code/.storybook/preview.tsx#L1-L416、code/.storybook/preview.tsx#L144-L193、code/.storybook/preview.tsx#L220-L301、code/.storybook/preview.tsx#L336-L400、code/.storybook/preview.tsx#L402-L415、code/addons/docs/package.json、code/addons/docs/package.json#L1-L133、code/addons/docs/package.json#L40-L80、code/addons/docs/src/preset.ts#L1-L225、code/addons/docs/src/preset.ts#L155-L157、code/addons/docs/src/preset.ts#L159-L193、code/addons/docs/src/preset.ts#L164-L187、code/addons/docs/src/preset.ts#L167-L193、code/addons/docs/src/preset.ts#L46-L60、code/builders/builder-vite/package.json、code/builders/builder-vite/package.json#L1-L73、code/builders/builder-vite/package.json#L29-L39、code/builders/builder-vite/package.json#L49-L62、code/builders/builder-vite/src/build.ts、code/builders/builder-vite/src/build.ts#L1-L40、code/builders/builder-vite/src/build.ts#L22-L38、code/builders/builder-vite/src/index.ts#L1-L68、code/builders/builder-vite/src/index.ts#L41-L65、code/core/package.json、code/core/package.json#L1-L389、code/core/package.json#L48-L221、code/core/src/types/index.ts#L1-L23、code/frameworks/react-vite/package.json、code/frameworks/react-vite/package.json#L1-L81、code/frameworks/react-vite/package.json#L28-L44、code/frameworks/react-vite/package.json#L65-L75、code/frameworks/react-vite/src/preset.ts#L1-L51、code/frameworks/react-vite/src/preset.ts#L10-L50、code/frameworks/react-vite/src/preset.ts#L14-L35、code/frameworks/react-vite/src/preset.ts#L5-L8、code/lib/cli-storybook/src/postinstallAddon.ts、code/lib/cli-storybook/src/postinstallAddon.ts#L1-L49、code/lib/cli-storybook/src/postinstallAddon.ts#L10-L49、code/presets/react-webpack/package.json、code/presets/react-webpack/package.json#L1-L72、code/presets/react-webpack/package.json#L23-L33、code/presets/react-webpack/src/index.ts#L1-L11、code/presets/react-webpack/src/index.ts#L7-L10、code/renderers/react/package.json、code/renderers/react/package.json#L1-L97、code/renderers/react/package.json#L23-L45、code/renderers/react/package.json#L54-L59、code/renderers/react/package.json#L81-L91、code/renderers/react/src/preview.tsx、code/renderers/react/src/preview.tsx#L1-L255、code/renderers/react/src/preview.tsx#L55-L85
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：main.ts#L1-L186、main.ts#L107-L116、main.ts#L129-L132、main.ts#L133-L180、main.ts#L169-L180、main.ts#L19-L132、manager.tsx、manager.tsx#L1-L9

