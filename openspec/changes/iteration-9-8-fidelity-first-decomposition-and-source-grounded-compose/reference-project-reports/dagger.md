# dagger Reference Fidelity Report

- project：dagger
- status：ready
- generated_pages：83
- reference_pages：65

## Run Metrics

- run_label：reuse
- cache_mode：preserve
- baseline_mode：warm_runtime_reuse
- usage：requests=0, total_tokens=0, page_research=0, page_enrichment=0

## Runtime Metrics

- runtime_state：ready
- baseline_class：acceptance_candidate
- incomplete_reason：n/a
- db_counts：knowledge_units=83, knowledge_domains=12, research_cache=96, page_digests=83, page_drafts=83, unit_runtime_gates=83, wiki_pages=83, pipeline_checkpoint=0
- pipeline_runtime_summary：state=completed, researched=0, compose_ready=0, composed=0, assembled=83
- unit_runtime_gates：total=83, compose_ready=0, compose_pending=0, compose_blocked=0, assemble_done=83
- parent_contract：parents=3, compose_ready_parents=3, child_digest_parents=3, missing_readiness_parents=0
- compose_diagnostics：digest_pages=83, skeleton_profile_pages=83, grounding_ref_pages=83, planned_key_source_pages=83, grounded_key_source_pages=83, grounding_gap_pages=81
- stop_reasons：no_further_tool_calls(1)、provider_error(55)、not_run(27)

## Fidelity Gate

- decision：not-pass
- reason：overall=96.92% / reuse_overage=21 / median_skeleton=0.17 / median_key_source=0.24 / warm_stable=true
- overall_match_rate：96.92%
- reuse_overage：21
- median_skeleton_fidelity：0.17
- median_key_source_coverage：0.24

## 四个专项问题

- 页数是否接近 reference：matched 63/65，missing=2
- 是否存在 coarse page reuse：reuse_pages=10，severe_reuse_pages=4，reuse_overage=21
- docs-backed 页面是否具备 reference 式骨架：median=0.17，shortfall=63
- 正文是否覆盖关键文件：median=0.24，shortfall=63

## Top Reuse Offenders

- 核心模块/核心模块.md：reuse_count=6；unit_id=unit-73b5821a1486, unit_type=domain_index, domain_id=domain-8984fdaabf39, readiness=compose_ready, child_digests=5, planned_key_sources=36, grounded_key_sources=11, grounding_refs=13
- API-参考/Android-API.md：reuse_count=6；unit_id=unit-9b35e673356d, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- 核心模块/dagger-kythe.md：reuse_count=4；unit_id=unit-adec100c2ce2, unit_type=module_doc, domain_id=domain-8984fdaabf39, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- API-参考/Provides-与--Binds-注解详解.md：reuse_count=3；unit_id=unit-d14dba3d1de2, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- 编译工具链/编译时处理机制.md：reuse_count=2；unit_id=unit-9471ea7609c1, unit_type=module_doc, domain_id=domain-7f3e824e1e4c, readiness=compose_ready, child_digests=5, planned_key_sources=10, grounded_key_sources=3, grounding_refs=5
- 编译工具链/SPI扩展机制.md：reuse_count=2；unit_id=unit-0793a2423091, unit_type=module_doc, domain_id=domain-7f3e824e1e4c, readiness=compose_ready, child_digests=0, planned_key_sources=10, grounded_key_sources=3, grounding_refs=4
- 核心运行时/高级特性与扩展.md：reuse_count=2；unit_id=unit-a6b2c5f6a900, unit_type=module_doc, domain_id=domain-6226b0bb2dd4, readiness=compose_ready, child_digests=2, planned_key_sources=8, grounded_key_sources=3, grounding_refs=5
- 核心运行时/核心概念.md：reuse_count=2；unit_id=unit-aa6ae00aecab, unit_type=module_doc, domain_id=domain-6226b0bb2dd4, readiness=compose_ready, child_digests=3, planned_key_sources=8, grounded_key_sources=3, grounding_refs=5
- API-参考/异步处理与生产者.md：reuse_count=2；unit_id=unit-067191dc7b9e, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- API-参考/hilt-core.md：reuse_count=2；unit_id=unit-ce9d5b292b4c, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12

## Skeleton Lowest Pages

- Hilt框架详解/Hilt框架详解.md -> 框架集成-Hilt/Hilt框架详解.md：skeleton=0.05
- 测试策略与最佳实践/Android测试.md -> 测试策略与最佳实践/Android测试.md：skeleton=0.06
- Hilt框架详解/@HiltAndroidApp与注解使用.md -> 框架集成-Hilt/Hilt框架详解/HiltAndroidApp与注解使用.md：skeleton=0.06
- 编译时处理机制/代码生成策略.md -> 编译工具链/编译时处理机制/代码生成策略.md：skeleton=0.06
- 高级特性与扩展/多值绑定高级用法.md -> 核心运行时/高级特性与扩展/多值绑定高级用法.md：skeleton=0.06

## Key Source Lowest Pages

- 测试策略与最佳实践/单元测试.md -> 测试策略与最佳实践/单元测试.md：coverage=0.00，missing=dagger-runtime/test/javatests/dagger/internal/DoubleCheckTest.java、dagger-runtime/test/javatests/dagger/internal/InstanceFactoryTest.java、dagger-runtime/test/javatests/dagger/internal/MapProviderFactoryTest.java、dagger-runtime/test/javatests/dagger/internal/SetFactoryTest.java、dagger-testing/main/java/dagger/model/testing/BindingGraphSubject.java、examples/bazel/javatests/example/hilt/CoffeeAppFakeHeaterTest.java
- 核心概念/核心注解详解/@Inject 注解详解.md -> 核心运行时/核心概念/Inject-注解详解.md：coverage=0.00，missing=dagger-compiler/main/java/dagger/internal/codecgen/binding/InjectionAnnotations.java、dagger-compiler/main/java/dagger/internal/codecgen/binding/InjectionBinding.java、dagger-compiler/main/java/dagger/internal/codecgen/binding/MembersInjectionBinding.java、dagger-compiler/main/java/dagger/internal/codecgen/binding/MembersInjectorBinding.java、dagger-compiler/main/java/dagger/internal/codecgen/processingstep/InjectProcessingStep.java、dagger-compiler/main/java/dagger/internal/codecgen/writing/MembersInjectorGenerator.java
- 核心概念/核心注解详解/@Provides 与 @Binds 注解详解.md -> API-参考/Provides-与--Binds-注解详解.md：coverage=0.00，missing=dagger-android/main/java/dagger/android/AndroidInjector.java、dagger-compiler/main/java/dagger/internal/codecgen/DelegateComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/DelegateComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/validation/BindsInstanceElementValidator.java、dagger-compiler/main/java/dagger/internal/codegen/validation/BindsMethodValidator.java、dagger-compiler/main/java/dagger/internal/codegen/validation/ProvidesMethodValidator.java
- Hilt框架详解/编译时处理与代码生成.md -> 框架集成-Hilt/Hilt框架详解/编译时处理与代码生成.md：coverage=0.00，missing=hilt-compiler/main/java/dagger/hilt/android/processor/internal/androidentrypoint/AndroidEntryPointProcessingStep.java、hilt-compiler/main/java/dagger/hilt/android/processor/internal/androidentrypoint/AndroidEntryPointProcessor.java、hilt-compiler/main/java/dagger/hilt/android/processor/internal/bindvalue/BindValueProcessingStep.java、hilt-compiler/main/java/dagger/hilt/android/processor/internal/bindvalue/BindValueProcessor.java、hilt-compiler/main/java/dagger/hilt/processor/internal/BaseProcessingStep.java、hilt-compiler/main/java/dagger/hilt/processor/internal/JavacBaseProcessingStepProcessor.java
- Hilt框架详解/测试支持与模拟.md -> 框架集成-Hilt/Hilt框架详解/测试支持与模拟.md：coverage=0.00，missing=examples/bazel/javatests/example/hilt/CoffeeAppFakeHeaterTest.java、hilt-android-testing/main/java/dagger/hilt/android/internal/testing/EarlySingletonComponentCreator.java、hilt-android-testing/main/java/dagger/hilt/android/internal/testing/MarkThatRulesRanRule.java、hilt-android-testing/main/java/dagger/hilt/android/internal/testing/TestApplicationComponentManager.java、hilt-android-testing/main/java/dagger/hilt/android/internal/testing/TestComponentDataSupplier.java、hilt-android-testing/main/java/dagger/hilt/android/testing/BindValue.java

## Warm Stability

- stable：true
- delta_reuse_overage：0
- delta_median_skeleton：0.00
- delta_median_key_source：0.00

## 覆盖统计

- topic coverage：generated 11 / reference 7
- evidence coverage：generated 83 / reference 65
- citation density：generated 15.98 / reference 17.43
- diagram coverage：generated 83 / reference 65
- 高频缺失专题：流程主题(1)

## Decomposition 命中

- generated：runtime(44)、compiler-pipeline(67)、api-surface(25)、config-surface(5)、docs-guide(83)、testing(22)、example-tutorial(14)、troubleshooting(31)、integration-platform(64)
- reference：runtime(32)、compiler-pipeline(47)、api-surface(22)、config-surface(8)、docs-guide(65)、testing(30)、example-tutorial(47)、troubleshooting(65)、integration-platform(37)
- 高频缺口：example-tutorial(37)、troubleshooting(36)、testing(18)、compiler-pipeline(9)、api-surface(7)、runtime(6)

## 项目结论

- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- docs-backed 页面主章节骨架仍偏离 reference，存在英文原始 docs heading 或泛化模板回退
- 解释层正文密度仍低于 reference
- 高频缺失专题集中在：流程主题

## 逐文件详情

### API参考文档/API参考文档.md

- reference 标题：API参考文档
- 生成页：核心模块/核心模块.md（核心模块）
- KnowledgeUnit：unit_id=unit-73b5821a1486, unit_type=domain_index, domain_id=domain-8984fdaabf39, readiness=compose_ready, child_digests=5, planned_key_sources=36, grounded_key_sources=11, grounding_refs=13
- reuse_count：6
- skeleton_score：0.47
- key_source_coverage：0.24
- missing_key_sources：dagger-android/main/java/dagger/android/AndroidInjector.java、dagger-android/main/java/dagger/android/HasAndroidInjector.java、dagger-producers/main/java/dagger/producers/CancellationPolicy.java、dagger-producers/main/java/dagger/producers/ProducerModule.java、dagger-producers/main/java/dagger/producers/Production.java、dagger-producers/main/java/dagger/producers/ProductionComponent.java、dagger-producers/main/java/dagger/producers/ProductionScope.java、dagger-runtime/main/java/dagger/Binds.java、dagger-runtime/main/java/dagger/BindsInstance.java、dagger-runtime/main/java/dagger/Lazy.java、dagger-runtime/main/java/dagger/MapKey.java、dagger-runtime/main/java/dagger/MembersInjector.java、dagger-runtime/main/java/dagger/Provides.java、dagger-runtime/main/java/dagger/Reusable.java、dagger-spi/main/java/d......
</cite>

## 目录
1. [简介](#简介、dagger-spi/main/java/dagger/model/BindingGraph.java、dagger-spi/main/java/dagger/model/BindingKind.java、dagger-spi/main/java/dagger/model/ComponentPath.java、dagger-spi/main/java/dagger/model/DependencyRequest.java、dagger-spi/main/java/dagger/model/RequestKind.java、dagger-spi/main/java/dagger/spi/BindingGraphPlugin.java、dagger-spi/main/java/dagger/spi/DiagnosticReporter.java、hilt-core/main/java/dagger/hilt/DefineComponent.java、hilt-core/main/java/dagger/hilt/EntryPoint.java、hilt-core/main/java/dagger/hilt/EntryPoints.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：AndroidInjector.java、HasAndroidInjector.java、CancellationPolicy.java、ProducerModule.java、Production.java、ProductionComponent.java、ProductionScope.java、Binds.java

### API参考文档/Android API.md

- reference 标题：Android API
- 生成页：API-参考/Android-API.md（Android API）
- KnowledgeUnit：unit_id=unit-9b35e673356d, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：6
- skeleton_score：0.37
- key_source_coverage：0.33
- missing_key_sources：README.md、dagger-android-support/main/java/dagger/android/support/AndroidSupportInjection.java、dagger-android-support/main/java/dagger/android/support/AndroidSupportInjectionModule.java、dagger-android/main/java/dagger/android/AndroidInjectionModule.java、dagger-android/main/java/dagger/android/ContributesAndroidInjector.java、dagger-android/main/java/dagger/android/DaggerActivity.java、dagger-android/main/java/dagger/android/DaggerApplication.java、dagger-android/main/java/dagger/android/DaggerFragment.java、dagger-android/main/java/dagger/android/DaggerService.java、dagger-android/main/java/dagger/android/DispatchingAndroidInjector.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：README.md、AndroidSupportInjection.java、AndroidSupportInjectionModule.java、AndroidInjectionModule.java、ContributesAndroidInjector.java、DaggerActivity.java、DaggerApplication.java、DaggerFragment.java

### API参考文档/Hilt API.md

- reference 标题：Hilt API
- 生成页：API-参考/Hilt-API.md（Hilt API）
- KnowledgeUnit：unit_id=unit-4c2bb0b84c4a, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：1
- skeleton_score：0.44
- key_source_coverage：0.14
- missing_key_sources：hilt-android/main/java/dagger/hilt/android/HiltAndroidApp.java、hilt-core/main/java/dagger/hilt/DefineComponent.java、hilt-core/main/java/dagger/hilt/EntryPoint.java、hilt-core/main/java/dagger/hilt/EntryPoints.java、hilt-core/main/java/dagger/hilt/GeneratesRootInput.java、hilt-core/main/java/dagger/hilt/InstallIn.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：HiltAndroidApp.java、DefineComponent.java、EntryPoint.java、EntryPoints.java、GeneratesRootInput.java、InstallIn.java

### API参考文档/异步处理API.md

- reference 标题：异步处理API
- 生成页：核心模块/dagger-producers.md（dagger-producers）
- KnowledgeUnit：unit_id=unit-9052a93a6afb, unit_type=module_doc, domain_id=domain-8984fdaabf39, readiness=compose_ready, child_digests=0, planned_key_sources=23, grounded_key_sources=6, grounding_refs=4
- reuse_count：1
- skeleton_score：0.15
- key_source_coverage：0.24
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/ProvisionDependencyOnProducerBindingValidator.java、dagger-compiler/main/java/dagger/internal/codegen/writing/DependencyMethodProducerCreationExpression.java、dagger-compiler/main/java/dagger/internal/codegen/writing/ProducerCreationExpression.java、dagger-producers/main/java/dagger/producers/CancellationPolicy.java、dagger-producers/main/java/dagger/producers/ProductionComponent.java、dagger-producers/main/java/dagger/producers/ProductionScope.java、dagger-producers/main/java/dagger/producers/ProductionSubcomponent.java、dagger-producers/main/java/dagger/producers/internal/AbstractProducer.java、dagger-producers/main/java/dagger/producers/internal/DependencyMethodProducer.java、dagger-producers/main/java/dagger/producers/internal/ProductionImplementation.java、javatests/dagger/functional/producers/ProducerFactoryTest.java、javatests/dagger/functional/producers/cancellation/ProducerCancellationTest.java、javatests/dagger/functional/producers/cancellation/ProducerSubcomponentCancellationTest.java、javatests/dagger/functional/producers/subcomponent/ProducerModuleWithSubcomponentsTest.java、javatests/dagger/producers/internal/AbstractProducerTest.java、javatests/dagger/producers/internal/MapProducerTest.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ProvisionDependencyOnProducerBindingValidator.java、DependencyMethodProducerCreationExpression.java、ProducerCreationExpression.java、CancellationPolicy.java、ProductionComponent.java、ProductionScope.java、ProductionSubcomponent.java、AbstractProducer.java

### API参考文档/编译时API.md

- reference 标题：编译时API
- 生成页：API-参考/编译时API.md（编译时API）
- KnowledgeUnit：unit_id=unit-2d1e0853dcb9, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：1
- skeleton_score：0.53
- key_source_coverage：0.27
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/binding/ComponentDescriptor.java、dagger-spi/main/java/dagger/model/BindingGraph.java、dagger-spi/main/java/dagger/model/ComponentPath.java、dagger-spi/main/java/dagger/model/DependencyRequest.java、dagger-spi/main/java/dagger/model/RequestKind.java、dagger-spi/main/java/dagger/spi/BindingGraphPlugin.java、dagger-spi/main/java/dagger/spi/DiagnosticReporter.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ComponentProcessor.java、ComponentDescriptor.java、BindingGraph.java、ComponentPath.java、DependencyRequest.java、RequestKind.java、BindingGraphPlugin.java、DiagnosticReporter.java

### API参考文档/运行时API.md

- reference 标题：运行时API
- 生成页：API-参考/运行时API.md（运行时API）
- KnowledgeUnit：unit_id=unit-88d067d2e809, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：1
- skeleton_score：0.35
- key_source_coverage：0.21
- missing_key_sources：dagger-runtime/main/java/dagger/Binds.java、dagger-runtime/main/java/dagger/BindsInstance.java、dagger-runtime/main/java/dagger/Lazy.java、dagger-runtime/main/java/dagger/MapKey.java、dagger-runtime/main/java/dagger/MembersInjector.java、dagger-runtime/main/java/dagger/Provides.java、dagger-runtime/main/java/dagger/Reusable.java、dagger-runtime/main/java/dagger/assisted/Assisted.java、dagger-runtime/main/java/dagger/assisted/AssistedFactory.java、dagger-runtime/main/java/dagger/multibindings/IntoMap.java、dagger-runtime/main/java/dagger/multibindings/IntoSet.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：Binds.java、BindsInstance.java、Lazy.java、MapKey.java、MembersInjector.java、Provides.java、Reusable.java、Assisted.java

### Android集成开发/@ContributesAndroidInjector注解.md

- reference 标题：@ContributesAndroidInjector注解
- 生成页：编译工具链/dagger-android-processor.md（dagger-android-processor）
- KnowledgeUnit：unit_id=unit-beeb37dec637, unit_type=module_doc, domain_id=domain-7f3e824e1e4c, readiness=compose_ready, child_digests=0, planned_key_sources=10, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.07
- key_source_coverage：0.40
- missing_key_sources：dagger-android-processor/main/java/dagger/android/processor/ContributesAndroidInjectorProcessingStep.java、dagger-android-processor/main/java/dagger/android/processor/XTypeNames.java、dagger-android/main/java/dagger/android/AndroidInjector.java、dagger-android/main/java/dagger/android/ContributesAndroidInjector.java、dagger-android/main/java/dagger/android/DispatchingAndroidInjector.java、dagger-android/test/javatests/dagger/android/AndroidInjectionTest.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ContributesAndroidInjectorProcessingStep.java、XTypeNames.java、AndroidInjector.java、ContributesAndroidInjector.java、DispatchingAndroidInjector.java、AndroidInjectionTest.java

### Android集成开发/Activity注入.md

- reference 标题：Activity注入
- 生成页：API-参考/Android-API.md（Android API）
- KnowledgeUnit：unit_id=unit-9b35e673356d, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：6
- skeleton_score：0.56
- key_source_coverage：0.36
- missing_key_sources：dagger-android-support/main/java/dagger/android/support/DaggerAppCompatActivity.java、dagger-android/main/java/dagger/android/AndroidInjectionModule.java、dagger-android/main/java/dagger/android/ContributesAndroidInjector.java、dagger-android/main/java/dagger/android/DaggerActivity.java、dagger-android/main/java/dagger/android/DispatchingAndroidInjector.java、dagger-android/test/javatests/dagger/android/AndroidInjectionTest.java、javatests/artifacts/dagger-android/simple/app/src/main/java/dagger/android/simple/SimpleActivity.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：DaggerAppCompatActivity.java、AndroidInjectionModule.java、ContributesAndroidInjector.java、DaggerActivity.java、DispatchingAndroidInjector.java、AndroidInjectionTest.java、SimpleActivity.java

### Android集成开发/Android生命周期管理.md

- reference 标题：Android生命周期管理
- 生成页：API-参考/Android-API.md（Android API）
- KnowledgeUnit：unit_id=unit-9b35e673356d, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：6
- skeleton_score：0.50
- key_source_coverage：0.31
- missing_key_sources：dagger-android-support/main/java/dagger/android/support/DaggerAppCompatActivity.java、dagger-android/main/java/dagger/android/AndroidInjectionModule.java、dagger-android/main/java/dagger/android/ContributesAndroidInjector.java、dagger-android/main/java/dagger/android/DaggerActivity.java、dagger-android/main/java/dagger/android/DaggerApplication.java、dagger-android/main/java/dagger/android/DaggerFragment.java、dagger-android/main/java/dagger/android/DispatchingAndroidInjector.java、dagger-android/test/javatests/dagger/android/AndroidInjectionTest.java、dagger-android/test/javatests/dagger/android/DispatchingAndroidInjectorTest.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：DaggerAppCompatActivity.java、AndroidInjectionModule.java、ContributesAndroidInjector.java、DaggerActivity.java、DaggerApplication.java、DaggerFragment.java、DispatchingAndroidInjector.java、AndroidInjectionTest.java

### Android集成开发/Android集成开发.md

- reference 标题：Android集成开发
- 生成页：API-参考/Android-API.md（Android API）
- KnowledgeUnit：unit_id=unit-9b35e673356d, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：6
- skeleton_score：0.41
- key_source_coverage：0.33
- missing_key_sources：dagger-android-support/main/java/dagger/android/support/DaggerAppCompatActivity.java、dagger-android-support/main/java/dagger/android/support/DaggerFragment.java、dagger-android/main/java/dagger/android/AndroidInjectionKey.java、dagger-android/main/java/dagger/android/AndroidInjectionModule.java、dagger-android/main/java/dagger/android/ContributesAndroidInjector.java、dagger-android/main/java/dagger/android/DispatchingAndroidInjector.java、dagger-android/test/javatests/dagger/android/AndroidInjectionTest.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeApp.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：DaggerAppCompatActivity.java、DaggerFragment.java、AndroidInjectionKey.java、AndroidInjectionModule.java、ContributesAndroidInjector.java、DispatchingAndroidInjector.java、AndroidInjectionTest.java、CoffeeApp.java

### Android集成开发/Fragment注入.md

- reference 标题：Fragment注入
- 生成页：API-参考/Android-API.md（Android API）
- KnowledgeUnit：unit_id=unit-9b35e673356d, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：6
- skeleton_score：0.53
- key_source_coverage：0.33
- missing_key_sources：dagger-android-support/main/java/dagger/android/support/AndroidSupportInjection.java、dagger-android-support/main/java/dagger/android/support/DaggerFragment.java、dagger-android/main/java/dagger/android/AndroidInjectionModule.java、dagger-android/main/java/dagger/android/ContributesAndroidInjector.java、dagger-android/main/java/dagger/android/DaggerFragment.java、dagger-android/main/java/dagger/android/DispatchingAndroidInjector.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：AndroidSupportInjection.java、DaggerFragment.java、AndroidInjectionModule.java、ContributesAndroidInjector.java、DaggerFragment.java、DispatchingAndroidInjector.java

### Android集成开发/Service注入.md

- reference 标题：Service注入
- 生成页：API-参考/dagger-android.md（API：dagger-android）
- KnowledgeUnit：unit_id=unit-952d81a48c23, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：1
- skeleton_score：0.60
- key_source_coverage：0.57
- missing_key_sources：dagger-android/main/java/dagger/android/ContributesAndroidInjector.java、dagger-android/main/java/dagger/android/DispatchingAndroidInjector.java、dagger-android/test/javatests/dagger/android/AndroidInjectionTest.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ContributesAndroidInjector.java、DispatchingAndroidInjector.java、AndroidInjectionTest.java

### Android集成开发/其他组件注入.md

- reference 标题：其他组件注入
- 生成页：API-参考/Android-API.md（Android API）
- KnowledgeUnit：unit_id=unit-9b35e673356d, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：6
- skeleton_score：0.53
- key_source_coverage：0.30
- missing_key_sources：dagger-android/main/java/dagger/android/AndroidInjectionModule.java、dagger-android/main/java/dagger/android/ContributesAndroidInjector.java、dagger-android/main/java/dagger/android/DaggerBroadcastReceiver.java、dagger-android/main/java/dagger/android/DaggerContentProvider.java、dagger-android/main/java/dagger/android/DaggerIntentService.java、dagger-android/main/java/dagger/android/DispatchingAndroidInjector.java、dagger-android/test/javatests/dagger/android/AndroidInjectionTest.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：AndroidInjectionModule.java、ContributesAndroidInjector.java、DaggerBroadcastReceiver.java、DaggerContentProvider.java、DaggerIntentService.java、DispatchingAndroidInjector.java、AndroidInjectionTest.java

### Hilt框架详解/@HiltAndroidApp与注解使用.md

- reference 标题：@HiltAndroidApp与注解使用
- 生成页：框架集成-Hilt/Hilt框架详解/HiltAndroidApp与注解使用.md（@HiltAndroidApp与注解使用）
- KnowledgeUnit：unit_id=unit-bac2fbebfa8c, unit_type=module_doc, domain_id=domain-3de8724b9bfa, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.06
- key_source_coverage：0.44
- missing_key_sources：examples/bazel/java/example/hilt/CoffeeApp.java、hilt-android/main/java/dagger/hilt/android/HiltAndroidApp.java、hilt-core/main/java/dagger/hilt/DefineComponent.java、hilt-core/main/java/dagger/hilt/EntryPoints.java、hilt-core/main/java/dagger/hilt/GeneratesRootInput.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CoffeeApp.java、HiltAndroidApp.java、DefineComponent.java、EntryPoints.java、GeneratesRootInput.java

### Hilt框架详解/Hilt核心概念与架构.md

- reference 标题：Hilt核心概念与架构
- 生成页：API-参考/hilt-core.md（API：hilt-core）
- KnowledgeUnit：unit_id=unit-ce9d5b292b4c, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：2
- skeleton_score：0.57
- key_source_coverage：0.63
- missing_key_sources：hilt-core/main/java/dagger/hilt/README.md、hilt-core/main/java/dagger/hilt/components/SingletonComponent.java、hilt-core/main/java/dagger/hilt/internal/definecomponent/DefineComponentNoParent.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：README.md、SingletonComponent.java、DefineComponentNoParent.java

### Hilt框架详解/Hilt框架详解.md

- reference 标题：Hilt框架详解
- 生成页：框架集成-Hilt/Hilt框架详解.md（Hilt框架详解）
- KnowledgeUnit：unit_id=unit-4e41870d3312, unit_type=module_doc, domain_id=domain-3de8724b9bfa, readiness=compose_ready, child_digests=5, planned_key_sources=8, grounded_key_sources=3, grounding_refs=5
- reuse_count：1
- skeleton_score：0.05
- key_source_coverage：0.40
- missing_key_sources：hilt-android/main/java/dagger/hilt/android/HiltAndroidApp.java、hilt-core/main/java/dagger/hilt/DefineComponent.java、hilt-core/main/java/dagger/hilt/GeneratesRootInput.java、hilt-core/main/java/dagger/hilt/components/SingletonComponent.java、hilt-core/main/java/dagger/hilt/internal/GeneratedComponent.java、hilt-core/main/java/dagger/hilt/internal/GeneratedComponentManager.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：HiltAndroidApp.java、DefineComponent.java、GeneratesRootInput.java、SingletonComponent.java、GeneratedComponent.java、GeneratedComponentManager.java

### Hilt框架详解/模块安装与依赖管理.md

- reference 标题：模块安装与依赖管理
- 生成页：框架集成-Hilt/Hilt框架详解/模块安装与依赖管理.md（模块安装与依赖管理）
- KnowledgeUnit：unit_id=unit-78b9ea123a1d, unit_type=module_doc, domain_id=domain-3de8724b9bfa, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.07
- key_source_coverage：0.13
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/DuplicateBindingsValidator.java、dagger-compiler/main/java/dagger/internal/codegen/processingstep/ModuleProcessingStep.java、dagger-compiler/main/java/dagger/internal/codegen/validation/ModuleValidator.java、hilt-android/main/java/dagger/hilt/android/components/ActivityComponent.java、hilt-core/main/java/dagger/hilt/DefineComponent.java、hilt-core/main/java/dagger/hilt/components/SingletonComponent.java、hilt-core/main/java/dagger/hilt/internal/definecomponent/DefineComponentNoParent.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：DuplicateBindingsValidator.java、ModuleProcessingStep.java、ModuleValidator.java、ActivityComponent.java、DefineComponent.java、SingletonComponent.java、DefineComponentNoParent.java

### Hilt框架详解/测试支持与模拟.md

- reference 标题：测试支持与模拟
- 生成页：框架集成-Hilt/Hilt框架详解/测试支持与模拟.md（测试支持与模拟）
- KnowledgeUnit：unit_id=unit-19e426c373ce, unit_type=module_doc, domain_id=domain-3de8724b9bfa, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.13
- key_source_coverage：0.00
- missing_key_sources：examples/bazel/javatests/example/hilt/CoffeeAppFakeHeaterTest.java、hilt-android-testing/main/java/dagger/hilt/android/internal/testing/EarlySingletonComponentCreator.java、hilt-android-testing/main/java/dagger/hilt/android/internal/testing/MarkThatRulesRanRule.java、hilt-android-testing/main/java/dagger/hilt/android/internal/testing/TestApplicationComponentManager.java、hilt-android-testing/main/java/dagger/hilt/android/internal/testing/TestComponentDataSupplier.java、hilt-android-testing/main/java/dagger/hilt/android/testing/BindValue.java、hilt-android-testing/main/java/dagger/hilt/android/testing/CustomTestApplication.java、hilt-android-testing/main/java/dagger/hilt/android/testing/HiltAndroidRule.java、hilt-android-testing/main/java/dagger/hilt/android/testing/HiltTestApplication.java、hilt-android-testing/main/java/dagger/hilt/testing/TestInstallIn.java、hilt-compiler/main/java/dagger/hilt/android/processor/internal/bindvalue/BindValueProcessingStep.java、hilt-compiler/main/java/dagger/hilt/processor/internal/root/TestRootMetadata.java、hilt-compiler/main/java/dagger/hilt/processor/internal/uninstallmodules/UninstallModulesProcessingStep.java、javatests/artifacts/hilt-android/simple/app/src/sharedTest/java/dagger/hilt/android/simple/ActivityScenarioRuleTest.java、javatests/artifacts/hilt-android/simple/app/src/sharedTest/java/dagger/hilt/android/simple/BindValueTest.java、javatests/artifacts/hilt-android/simple/app/src/sharedTest/java/dagger/hilt/android/simple/SimpleActivityTest.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CoffeeAppFakeHeaterTest.java、EarlySingletonComponentCreator.java、MarkThatRulesRanRule.java、TestApplicationComponentManager.java、TestComponentDataSupplier.java、BindValue.java、CustomTestApplication.java、HiltAndroidRule.java

### Hilt框架详解/组件树结构与生命周期.md

- reference 标题：组件树结构与生命周期
- 生成页：框架集成-Hilt/Hilt框架详解/组件树结构与生命周期.md（组件树结构与生命周期）
- KnowledgeUnit：unit_id=unit-4d47b378ca3d, unit_type=module_doc, domain_id=domain-3de8724b9bfa, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.07
- key_source_coverage：0.50
- missing_key_sources：hilt-core/main/java/dagger/hilt/DefineComponent.java、hilt-core/main/java/dagger/hilt/components/SingletonComponent.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：DefineComponent.java、SingletonComponent.java

### Hilt框架详解/编译时处理与代码生成.md

- reference 标题：编译时处理与代码生成
- 生成页：框架集成-Hilt/Hilt框架详解/编译时处理与代码生成.md（编译时处理与代码生成）
- KnowledgeUnit：unit_id=unit-c3cbe0283c09, unit_type=module_doc, domain_id=domain-3de8724b9bfa, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.08
- key_source_coverage：0.00
- missing_key_sources：hilt-compiler/main/java/dagger/hilt/android/processor/internal/androidentrypoint/AndroidEntryPointProcessingStep.java、hilt-compiler/main/java/dagger/hilt/android/processor/internal/androidentrypoint/AndroidEntryPointProcessor.java、hilt-compiler/main/java/dagger/hilt/android/processor/internal/bindvalue/BindValueProcessingStep.java、hilt-compiler/main/java/dagger/hilt/android/processor/internal/bindvalue/BindValueProcessor.java、hilt-compiler/main/java/dagger/hilt/processor/internal/BaseProcessingStep.java、hilt-compiler/main/java/dagger/hilt/processor/internal/JavacBaseProcessingStepProcessor.java、hilt-compiler/main/java/dagger/hilt/processor/internal/aggregateddeps/AggregatedDepsProcessingStep.java、hilt-compiler/main/java/dagger/hilt/processor/internal/aggregateddeps/AggregatedDepsProcessor.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：AndroidEntryPointProcessingStep.java、AndroidEntryPointProcessor.java、BindValueProcessingStep.java、BindValueProcessor.java、BaseProcessingStep.java、JavacBaseProcessingStepProcessor.java、AggregatedDepsProcessingStep.java、AggregatedDepsProcessor.java

### 异步处理与生产者.md

- reference 标题：异步处理与生产者
- 生成页：API-参考/异步处理与生产者.md（异步处理与生产者）
- KnowledgeUnit：unit_id=unit-067191dc7b9e, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：2
- skeleton_score：0.32
- key_source_coverage：0.21
- missing_key_sources：dagger-producers/main/java/dagger/producers/CancellationPolicy.java、dagger-producers/main/java/dagger/producers/Produced.java、dagger-producers/main/java/dagger/producers/ProducerModule.java、dagger-producers/main/java/dagger/producers/ProductionScope.java、dagger-producers/main/java/dagger/producers/ProductionSubcomponent.java、dagger-producers/main/java/dagger/producers/internal/AbstractProducer.java、dagger-producers/main/java/dagger/producers/internal/DependencyMethodProducer.java、dagger-producers/main/java/dagger/producers/monitoring/ProducerMonitor.java、dagger-producers/main/java/dagger/producers/monitoring/ProductionComponentMonitor.java、javatests/dagger/functional/producers/cancellation/ProducerCancellationTest.java、javatests/dagger/producers/internal/AbstractProducerTest.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CancellationPolicy.java、Produced.java、ProducerModule.java、ProductionScope.java、ProductionSubcomponent.java、AbstractProducer.java、DependencyMethodProducer.java、ProducerMonitor.java

### 快速开始.md

- reference 标题：快速开始
- 生成页：快速开始.md（快速开始）
- KnowledgeUnit：unit_id=unit-0e6c274eb4ff, unit_type=example_doc, domain_id=domain-be9fa08103e5, readiness=compose_ready, child_digests=0, planned_key_sources=7, grounded_key_sources=4, grounding_refs=12
- reuse_count：1
- skeleton_score：0.40
- key_source_coverage：0.55
- missing_key_sources：examples/maven/coffee/pom.xml、examples/maven/coffee/src/main/java/example/dagger/CoffeeApp.java、examples/maven/coffee/src/main/java/example/dagger/HeaterModule.java、examples/maven/coffee/src/main/java/example/dagger/PumpModule.java、examples/maven/coffee/src/main/java/example/dagger/Thermosiphon.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：pom.xml、CoffeeApp.java、HeaterModule.java、PumpModule.java、Thermosiphon.java

### 故障排除与调试.md

- reference 标题：故障排除与调试
- 生成页：编译工具链/编译时处理机制.md（编译时处理机制）
- KnowledgeUnit：unit_id=unit-9471ea7609c1, unit_type=module_doc, domain_id=domain-7f3e824e1e4c, readiness=compose_ready, child_digests=5, planned_key_sources=10, grounded_key_sources=3, grounding_refs=5
- reuse_count：2
- skeleton_score：0.10
- key_source_coverage：0.33
- missing_key_sources：CONTRIBUTING.md、README.md、dagger-compiler/main/java/dagger/internal/codegen/binding/ErrorMessages.java、dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/DependencyCycleValidator.java、dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/DuplicateBindingsValidator.java、dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/MissingBindingValidator.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CONTRIBUTING.md、README.md、ErrorMessages.java、DependencyCycleValidator.java、DuplicateBindingsValidator.java、MissingBindingValidator.java

### 构建与部署.md

- reference 标题：构建与部署
- 生成页：配置参考/主题和外观.md（主题和外观）
- KnowledgeUnit：unit_id=unit-bebeb3781961, unit_type=config_doc, domain_id=domain-e15151050742, readiness=compose_ready, child_digests=3, planned_key_sources=10, grounded_key_sources=4, grounding_refs=13
- reuse_count：1
- skeleton_score：0.47
- key_source_coverage：0.19
- missing_key_sources：.github/workflows/ci.yml、CONTRIBUTING.md、MODULE.bazel、buildSrc/src/main/kotlin/dagger/gradle/build/DaggerConventionPlugin.kt、examples/maven/coffee/pom.xml、gradle.properties、gradle/libs.versions.toml、tools/bazel_compat.bzl、tools/maven/maven.bzl、util/deploy-all.sh、util/deploy-to-maven-central.sh、util/install-local-snapshot.sh、util/run-local-tests.sh
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ci.yml、CONTRIBUTING.md、MODULE.bazel、DaggerConventionPlugin.kt、pom.xml、gradle.properties、libs.versions.toml、bazel_compat.bzl

### 核心概念/作用域与生命周期管理.md

- reference 标题：作用域与生命周期管理
- 生成页：核心模块/核心模块.md（核心模块）
- KnowledgeUnit：unit_id=unit-73b5821a1486, unit_type=domain_index, domain_id=domain-8984fdaabf39, readiness=compose_ready, child_digests=5, planned_key_sources=36, grounded_key_sources=11, grounding_refs=13
- reuse_count：6
- skeleton_score：0.50
- key_source_coverage：0.50
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/IncompatiblyScopedBindingsValidator.java、dagger-runtime/main/java/dagger/Reusable.java、dagger-runtime/main/java/dagger/internal/DoubleCheck.java、dagger-runtime/main/java/dagger/internal/SingleCheck.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：IncompatiblyScopedBindingsValidator.java、Reusable.java、DoubleCheck.java、SingleCheck.java

### 核心概念/依赖注入基础理论.md

- reference 标题：依赖注入基础理论
- 生成页：核心运行时/高级特性与扩展.md（高级特性与扩展）
- KnowledgeUnit：unit_id=unit-a6b2c5f6a900, unit_type=module_doc, domain_id=domain-6226b0bb2dd4, readiness=compose_ready, child_digests=2, planned_key_sources=8, grounded_key_sources=3, grounding_refs=5
- reuse_count：2
- skeleton_score：0.13
- key_source_coverage：0.25
- missing_key_sources：dagger-runtime/main/java/dagger/Binds.java、dagger-runtime/main/java/dagger/Component.java、dagger-runtime/main/java/dagger/MembersInjector.java、dagger-runtime/main/java/dagger/Provides.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeApp.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeLogger.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeMaker.java、examples/maven/coffee/src/main/java/example/dagger/HeaterModule.java、examples/maven/coffee/src/main/java/example/dagger/PumpModule.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：Binds.java、Component.java、MembersInjector.java、Provides.java、CoffeeApp.java、CoffeeLogger.java、CoffeeMaker.java、HeaterModule.java

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：核心运行时/核心概念.md（核心概念）
- KnowledgeUnit：unit_id=unit-aa6ae00aecab, unit_type=module_doc, domain_id=domain-6226b0bb2dd4, readiness=compose_ready, child_digests=3, planned_key_sources=8, grounded_key_sources=3, grounding_refs=5
- reuse_count：2
- skeleton_score：0.07
- key_source_coverage：0.08
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/base/Scopes.java、dagger-compiler/main/java/dagger/internal/codegen/binding/ComponentDescriptor.java、dagger-compiler/main/java/dagger/internal/codegen/model/Scope.java、dagger-runtime/main/java/dagger/Binds.java、dagger-runtime/main/java/dagger/BindsInstance.java、dagger-runtime/main/java/dagger/Component.java、dagger-runtime/main/java/dagger/Provides.java、dagger-runtime/main/java/dagger/Reusable.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeApp.java、examples/maven/coffee/src/main/java/example/dagger/HeaterModule.java、examples/maven/coffee/src/main/java/example/dagger/PumpModule.java、examples/maven/coffee/src/main/java/example/dagger/Thermosiphon.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：Scopes.java、ComponentDescriptor.java、Scope.java、Binds.java、BindsInstance.java、Component.java、Provides.java、Reusable.java

### 核心概念/核心注解详解/@Component 注解详解.md

- reference 标题：@Component 注解详解
- 生成页：核心模块/核心模块.md（核心模块）
- KnowledgeUnit：unit_id=unit-73b5821a1486, unit_type=domain_index, domain_id=domain-8984fdaabf39, readiness=compose_ready, child_digests=5, planned_key_sources=36, grounded_key_sources=11, grounding_refs=13
- reuse_count：6
- skeleton_score：0.44
- key_source_coverage：0.23
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/base/ComponentAnnotation.java、dagger-compiler/main/java/dagger/internal/codegen/componentgenerator/ComponentGenerator.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeApp.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeLogger.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeMaker.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeShop.java、examples/maven/coffee/src/main/java/example/dagger/HeaterModule.java、examples/maven/coffee/src/main/java/example/dagger/PumpModule.java、examples/maven/coffee/src/main/java/example/dagger/Thermosiphon.java
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ComponentProcessor.java、ComponentAnnotation.java、ComponentGenerator.java、CoffeeApp.java、CoffeeLogger.java、CoffeeMaker.java、CoffeeShop.java、HeaterModule.java

### 核心概念/核心注解详解/@Inject 注解详解.md

- reference 标题：@Inject 注解详解
- 生成页：核心运行时/核心概念/Inject-注解详解.md（@Inject 注解详解）
- KnowledgeUnit：unit_id=unit-edcdcfe121ff, unit_type=module_doc, domain_id=domain-6226b0bb2dd4, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.06
- key_source_coverage：0.00
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codecgen/binding/InjectionAnnotations.java、dagger-compiler/main/java/dagger/internal/codecgen/binding/InjectionBinding.java、dagger-compiler/main/java/dagger/internal/codecgen/binding/MembersInjectionBinding.java、dagger-compiler/main/java/dagger/internal/codecgen/binding/MembersInjectorBinding.java、dagger-compiler/main/java/dagger/internal/codecgen/processingstep/InjectProcessingStep.java、dagger-compiler/main/java/dagger/internal/codecgen/writing/MembersInjectorGenerator.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：InjectionAnnotations.java、InjectionBinding.java、MembersInjectionBinding.java、MembersInjectorBinding.java、InjectProcessingStep.java、MembersInjectorGenerator.java

### 核心概念/核心注解详解/@Module 注解详解.md

- reference 标题：@Module 注解详解
- 生成页：核心运行时/核心概念.md（核心概念）
- KnowledgeUnit：unit_id=unit-aa6ae00aecab, unit_type=module_doc, domain_id=domain-6226b0bb2dd4, readiness=compose_ready, child_digests=3, planned_key_sources=8, grounded_key_sources=3, grounding_refs=5
- reuse_count：2
- skeleton_score：0.15
- key_source_coverage：0.17
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/processingstep/ModuleProcessingStep.java、dagger-compiler/main/java/dagger/internal/codegen/validation/ModuleValidator.java、dagger-compiler/main/java/dagger/internal/codegen/writing/ModuleGenerator.java、dagger-runtime/main/java/dagger/Binds.java、dagger-runtime/main/java/dagger/Provides.java、dagger-runtime/main/java/dagger/multibindings/ElementsIntoSet.java、dagger-runtime/main/java/dagger/multibindings/IntoSet.java、dagger-runtime/main/java/dagger/multibindings/Multibinds.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeApp.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeMaker.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ModuleProcessingStep.java、ModuleValidator.java、ModuleGenerator.java、Binds.java、Provides.java、ElementsIntoSet.java、IntoSet.java、Multibinds.java

### 核心概念/核心注解详解/@Provides 与 @Binds 注解详解.md

- reference 标题：@Provides 与 @Binds 注解详解
- 生成页：API-参考/Provides-与--Binds-注解详解.md（@Provides 与 @Binds 注解详解）
- KnowledgeUnit：unit_id=unit-d14dba3d1de2, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：3
- skeleton_score：0.50
- key_source_coverage：0.00
- missing_key_sources：dagger-android/main/java/dagger/android/AndroidInjector.java、dagger-compiler/main/java/dagger/internal/codecgen/DelegateComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/DelegateComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/validation/BindsInstanceElementValidator.java、dagger-compiler/main/java/dagger/internal/codegen/validation/BindsMethodValidator.java、dagger-compiler/main/java/dagger/internal/codegen/validation/ProvidesMethodValidator.java、dagger-runtime/main/java/dagger/Binds.java、dagger-runtime/main/java/dagger/BindsInstance.java、dagger-runtime/main/java/dagger/Provides.java
- 结论：图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：AndroidInjector.java、DelegateComponentProcessor.java、DelegateComponentProcessor.java、BindsInstanceElementValidator.java、BindsMethodValidator.java、ProvidesMethodValidator.java、Binds.java、BindsInstance.java

### 核心概念/核心注解详解/核心注解详解.md

- reference 标题：核心注解详解
- 生成页：API-参考/Provides-与--Binds-注解详解.md（@Provides 与 @Binds 注解详解）
- KnowledgeUnit：unit_id=unit-d14dba3d1de2, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：3
- skeleton_score：0.46
- key_source_coverage：0.15
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/processingstep/ComponentProcessingStep.java、dagger-runtime/main/java/dagger/Provides.java、dagger-runtime/main/java/dagger/internal/InjectedFieldSignature.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeApp.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeAppComponent.java、examples/maven/coffee/src/main/java/example/dagger/Heater.java、examples/maven/coffee/src/main/java/example/dagger/HeaterModule.java、examples/maven/coffee/src/main/java/example/dagger/Pump.java、examples/maven/coffee/src/main/java/example/dagger/SimpleModule.java、examples/maven/coffee/src/main/java/example/dagger/Thermosiphon.java
- 结论：图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ComponentProcessor.java、ComponentProcessingStep.java、Provides.java、InjectedFieldSignature.java、CoffeeApp.java、CoffeeAppComponent.java、Heater.java、HeaterModule.java

### 核心概念/组件与模块系统/模块组织与管理.md

- reference 标题：模块组织与管理
- 生成页：核心模块/dagger-kythe.md（dagger-kythe）
- KnowledgeUnit：unit_id=unit-adec100c2ce2, unit_type=module_doc, domain_id=domain-8984fdaabf39, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：4
- skeleton_score：0.19
- key_source_coverage：0.17
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/validation/ModuleValidator.java、dagger-compiler/main/java/dagger/internal/codegen/validation/ProvidesMethodValidator.java、dagger-compiler/main/java/dagger/internal/codegen/writing/ModuleProxies.java、dagger-runtime/main/java/dagger/Provides.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeApp.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ModuleValidator.java、ProvidesMethodValidator.java、ModuleProxies.java、Provides.java、CoffeeApp.java

### 核心概念/组件与模块系统/组件与模块系统.md

- reference 标题：组件与模块系统
- 生成页：核心模块/dagger-kythe.md（dagger-kythe）
- KnowledgeUnit：unit_id=unit-adec100c2ce2, unit_type=module_doc, domain_id=domain-8984fdaabf39, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：4
- skeleton_score：0.07
- key_source_coverage：0.30
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/BindingGraphValidationModule.java、dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/SubcomponentFactoryMethodValidator.java、dagger-compiler/main/java/dagger/internal/codegen/model/ComponentPath.java、dagger-compiler/main/java/dagger/internal/codegen/validation/ComponentHierarchyValidator.java、dagger-compiler/main/java/dagger/internal/codegen/writing/ComponentImplementation.java、java/dagger/example/atm/AccountModule.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ComponentProcessor.java、BindingGraphValidationModule.java、SubcomponentFactoryMethodValidator.java、ComponentPath.java、ComponentHierarchyValidator.java、ComponentImplementation.java、AccountModule.java

### 核心概念/组件与模块系统/组件关系设计/子组件机制.md

- reference 标题：子组件机制
- 生成页：核心模块/核心模块.md（核心模块）
- KnowledgeUnit：unit_id=unit-73b5821a1486, unit_type=domain_index, domain_id=domain-8984fdaabf39, readiness=compose_ready, child_digests=5, planned_key_sources=36, grounded_key_sources=11, grounding_refs=13
- reuse_count：6
- skeleton_score：0.47
- key_source_coverage：0.07
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/CompositeBindingGraphPlugin.java、dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/DependencyCycleValidator.java、dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/SubcomponentFactoryMethodValidator.java、dagger-compiler/main/java/dagger/internal/codegen/componentgenerator/CurrentImplementationSubcomponent.java、dagger-compiler/main/java/dagger/internal/codegen/componentgenerator/TopLevelImplementationComponent.java、javatests/artifacts/dagger-android-ksp/app/src/main/kotlin/dagger/android/ksp/SimpleActivity.kt、javatests/artifacts/dagger-android-ksp/app/src/main/kotlin/dagger/android/ksp/SimpleActivityComponent.kt、javatests/artifacts/dagger-android/simple/app/src/main/java/dagger/android/simple/SimpleActivity.java、javatests/artifacts/dagger-android/simple/app/src/main/java/dagger/android/simple/SimpleActivityComponent.java、javatests/artifacts/dagger/kotlin-app/kotlin-library/src/main/kotlin/library/MySubcomponent.kt、javatests/artifacts/dagger/transitive-annotation-app/library1/src/main/java/library1/MySubcomponentWithBuilder.java、javatests/artifacts/dagger/transitive-annotation-app/library1/src/main/java/library1/MySubcomponentWithFactory.java、javatests/dagger/android/support/functional/AllControllersAreDirectChildrenOfApplication.java、javatests/dagger/android/support/functional/ComponentStructureFollowsControllerStructureApplication.java、javatests/dagger/functional/builder/BuilderTest.java、javatests/dagger/functional/factory/SubcomponentFactoryTest.java、javatests/dagger/functional/kotlinsrc/builder/BuilderTest.kt、javatests/dagger/functional/kotlinsrc/factory/SubcomponentFactoryTest.kt、javatests/dagger/functional/kotlinsrc/subcomponent/UnresolvableChildComponent.kt、javatests/dagger/functional/kotlinsrc/subcomponent/module/SubcomponentFromModuleAndFactoryMethod.kt、javatests/dagger/functional/kotlinsrc/subcomponent/repeat/SubcomponentWithRepeatedModule.kt、javatests/dagger/functional/kotlinsrc/subcomponent/repeat/SubcomponentWithoutRepeatedModule.kt、javatests/dagger/functional/subcomponent/UnresolvableChildComponent.java、javatests/dagger/functional/subcomponent/module/SubcomponentFromModuleAndFactoryMethod.java、javatests/dagger/functional/subcomponent/repeat/SubcomponentWithRepeatedModule.java、javatests/dagger/functional/subcomponent/repeat/SubcomponentWithoutRepeatedModule.java、javatests/dagger/grpc/functional/server/CoffeeServerWithCallScopeService.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CompositeBindingGraphPlugin.java、DependencyCycleValidator.java、SubcomponentFactoryMethodValidator.java、CurrentImplementationSubcomponent.java、TopLevelImplementationComponent.java、SimpleActivity.kt、SimpleActivityComponent.kt、SimpleActivity.java

### 核心概念/组件与模块系统/组件关系设计/组件依赖关系.md

- reference 标题：组件依赖关系
- 生成页：核心模块/dagger-kythe.md（dagger-kythe）
- KnowledgeUnit：unit_id=unit-adec100c2ce2, unit_type=module_doc, domain_id=domain-8984fdaabf39, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：4
- skeleton_score：0.20
- key_source_coverage：0.08
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codecgen/writing/ComponentRequirementRequestRepresentation.java、dagger-compiler/main/java/dagger/internal/codegen/binding/ComponentDependencyBinding.java、dagger-compiler/main/java/dagger/internal/codegen/binding/ComponentDescriptor.java、dagger-compiler/main/java/dagger/internal/codegen/binding/ComponentRequirement.java、dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/DependencyCycleValidator.java、dagger-compiler/main/java/dagger/internal/codegen/model/BindingKind.java、dagger-compiler/main/java/dagger/internal/codegen/validation/ComponentValidator.java、dagger-compiler/main/java/dagger/internal/codegen/writing/ComponentRequirementRequestRepresentation.java、dagger-compiler/main/java/dagger/internal/codegen/writing/DependencyMethodProducerCreationExpression.java、dagger-compiler/main/java/dagger/internal/codegen/writing/DependencyMethodProviderCreationExpression.java、dagger-spi/main/java/dagger/spi/model/BindingKind.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ComponentRequirementRequestRepresentation.java、ComponentDependencyBinding.java、ComponentDescriptor.java、ComponentRequirement.java、DependencyCycleValidator.java、BindingKind.java、ComponentValidator.java、ComponentRequirementRequestRepresentation.java

### 核心概念/组件与模块系统/组件关系设计/组件关系设计.md

- reference 标题：组件关系设计
- 生成页：核心模块/dagger-kythe.md（dagger-kythe）
- KnowledgeUnit：unit_id=unit-adec100c2ce2, unit_type=module_doc, domain_id=domain-8984fdaabf39, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：4
- skeleton_score：0.23
- key_source_coverage：0.27
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codecgen/binding/ComponentDependencyBinding.java、dagger-compiler/main/java/dagger/internal/codecgen/binding/SubcomponentCreatorBinding.java、dagger-compiler/main/java/dagger/internal/codecgen/binding/SubcomponentDeclaration.java、dagger-compiler/main/java/dagger/internal/codecgen/bindinggraphvalidation/SubcomponentFactoryMethodValidator.java、dagger-compiler/main/java/dagger/internal/codecgen/componentgenerator/ComponentGenerator.java、dagger-compiler/main/java/dagger/internal/codecgen/validation/ComponentHierarchyValidator.java、dagger-compiler/main/java/dagger/internal/codecgen/writing/SubcomponentCreatorRequestRepresentation.java、dagger-producers/main/java/dagger/producers/ProductionSubcomponent.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ComponentDependencyBinding.java、SubcomponentCreatorBinding.java、SubcomponentDeclaration.java、SubcomponentFactoryMethodValidator.java、ComponentGenerator.java、ComponentHierarchyValidator.java、SubcomponentCreatorRequestRepresentation.java、ProductionSubcomponent.java

### 核心概念/组件与模块系统/组件基础概念.md

- reference 标题：组件基础概念
- 生成页：核心模块/核心模块.md（核心模块）
- KnowledgeUnit：unit_id=unit-73b5821a1486, unit_type=domain_index, domain_id=domain-8984fdaabf39, readiness=compose_ready, child_digests=5, planned_key_sources=36, grounded_key_sources=11, grounding_refs=13
- reuse_count：6
- skeleton_score：0.43
- key_source_coverage：0.17
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/binding/ComponentDescriptor.java、dagger-compiler/main/java/dagger/internal/codegen/componentgenerator/ComponentGenerator.java、dagger-runtime/main/java/dagger/MembersInjector.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeApp.java、javatests/artifacts/dagger/transitive-annotation-app/src/main/java/app/MyComponent.java
- 结论：内容比 reference 更展开；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ComponentDescriptor.java、ComponentGenerator.java、MembersInjector.java、CoffeeApp.java、MyComponent.java

### 核心概念/组件与模块系统/组件构建器与工厂.md

- reference 标题：组件构建器与工厂
- 生成页：核心模块/核心模块.md（核心模块）
- KnowledgeUnit：unit_id=unit-73b5821a1486, unit_type=domain_index, domain_id=domain-8984fdaabf39, readiness=compose_ready, child_digests=5, planned_key_sources=36, grounded_key_sources=11, grounding_refs=13
- reuse_count：6
- skeleton_score：0.40
- key_source_coverage：0.09
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/base/ComponentCreatorKind.java、dagger-compiler/main/java/dagger/internal/codegen/binding/ComponentCreatorDescriptor.java、dagger-compiler/main/java/dagger/internal/codegen/binding/ErrorMessages.java、dagger-compiler/main/java/dagger/internal/codegen/validation/ComponentCreatorValidator.java、dagger-runtime/main/java/dagger/BindsInstance.java、javatests/artifacts/dagger/transitive-annotation-app/library1/src/main/java/library1/MySubcomponentWithBuilder.java、javatests/artifacts/dagger/transitive-annotation-app/src/main/java/app/MyComponent.java、javatests/dagger/functional/builder/BuilderBindsInstanceParameterTest.java、javatests/dagger/functional/builder/BuilderTest.java、javatests/dagger/functional/producers/builder/ProductionComponentBuilderTest.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ComponentCreatorKind.java、ComponentCreatorDescriptor.java、ErrorMessages.java、ComponentCreatorValidator.java、BindsInstance.java、MySubcomponentWithBuilder.java、MyComponent.java、BuilderBindsInstanceParameterTest.java

### 核心概念/绑定与提供者模式.md

- reference 标题：绑定与提供者模式
- 生成页：核心运行时/核心概念/绑定与提供者模式.md（绑定与提供者模式）
- KnowledgeUnit：unit_id=unit-7f6a6a33a021, unit_type=module_doc, domain_id=domain-6226b0bb2dd4, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.07
- key_source_coverage：0.10
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/binding/ContributionBinding.java、dagger-compiler/main/java/dagger/internal/codegen/validation/BindsMethodValidator.java、dagger-compiler/main/java/dagger/internal/codegen/validation/ProvidesMethodValidator.java、dagger-runtime/main/java/dagger/Binds.java、dagger-runtime/main/java/dagger/BindsOptionalOf.java、dagger-runtime/main/java/dagger/Lazy.java、dagger-runtime/main/java/dagger/Provides.java、dagger-runtime/main/java/dagger/multibindings/IntoMap.java、dagger-runtime/main/java/dagger/multibindings/IntoSet.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ContributionBinding.java、BindsMethodValidator.java、ProvidesMethodValidator.java、Binds.java、BindsOptionalOf.java、Lazy.java、Provides.java、IntoMap.java

### 测试策略与最佳实践/Android测试.md

- reference 标题：Android测试
- 生成页：测试策略与最佳实践/Android测试.md（Android测试）
- KnowledgeUnit：unit_id=unit-157bd350f471, unit_type=test_doc, domain_id=domain-64363b4da66f, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.06
- key_source_coverage：0.44
- missing_key_sources：examples/bazel/javatests/example/hilt/CoffeeAppFakeHeaterTest.java、examples/bazel/javatests/example/hilt/CoffeeAppFakePumpTest.java、hilt-android-testing/main/java/dagger/hilt/android/testing/BindValue.java、hilt-android-testing/main/java/dagger/hilt/android/testing/HiltTestApplication.java、hilt-core/main/java/dagger/hilt/InstallIn.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CoffeeAppFakeHeaterTest.java、CoffeeAppFakePumpTest.java、BindValue.java、HiltTestApplication.java、InstallIn.java

### 测试策略与最佳实践/单元测试.md

- reference 标题：单元测试
- 生成页：测试策略与最佳实践/单元测试.md（单元测试）
- KnowledgeUnit：unit_id=unit-6b750af866ba, unit_type=test_doc, domain_id=domain-64363b4da66f, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.08
- key_source_coverage：0.00
- missing_key_sources：dagger-runtime/test/javatests/dagger/internal/DoubleCheckTest.java、dagger-runtime/test/javatests/dagger/internal/InstanceFactoryTest.java、dagger-runtime/test/javatests/dagger/internal/MapProviderFactoryTest.java、dagger-runtime/test/javatests/dagger/internal/SetFactoryTest.java、dagger-testing/main/java/dagger/model/testing/BindingGraphSubject.java、examples/bazel/javatests/example/hilt/CoffeeAppFakeHeaterTest.java、hilt-android-testing/main/java/dagger/hilt/testing/TestInstallIn.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：DoubleCheckTest.java、InstanceFactoryTest.java、MapProviderFactoryTest.java、SetFactoryTest.java、BindingGraphSubject.java、CoffeeAppFakeHeaterTest.java、TestInstallIn.java

### 测试策略与最佳实践/性能测试与监控.md

- reference 标题：性能测试与监控
- 生成页：测试策略与最佳实践/性能测试与监控.md（性能测试与监控）
- KnowledgeUnit：unit_id=unit-ba15e78f728f, unit_type=test_doc, domain_id=domain-64363b4da66f, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.11
- key_source_coverage：0.38
- missing_key_sources：dagger-producers/main/java/dagger/producers/monitoring/ProducerMonitor.java、dagger-producers/main/java/dagger/producers/monitoring/ProducerToken.java、dagger-producers/main/java/dagger/producers/monitoring/ProductionComponentMonitor.java、dagger-producers/main/java/dagger/producers/monitoring/ProductionComponentTimingRecorder.java、dagger-producers/main/java/dagger/producers/monitoring/TimingProductionComponentMonitor.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ProducerMonitor.java、ProducerToken.java、ProductionComponentMonitor.java、ProductionComponentTimingRecorder.java、TimingProductionComponentMonitor.java

### 测试策略与最佳实践/测试策略与最佳实践.md

- reference 标题：测试策略与最佳实践
- 生成页：测试策略与最佳实践/测试策略与最佳实践.md（测试策略与最佳实践）
- KnowledgeUnit：unit_id=unit-42f796a9a9ed, unit_type=test_doc, domain_id=domain-64363b4da66f, readiness=compose_ready, child_digests=4, planned_key_sources=12, grounded_key_sources=3, grounding_refs=5
- reuse_count：1
- skeleton_score：0.07
- key_source_coverage：0.17
- missing_key_sources：dagger-testing/main/java/dagger/model/testing/BindingGraphSubject.java、examples/bazel/javatests/example/hilt/CoffeeAppFakeHeaterTest.java、examples/bazel/javatests/example/hilt/CoffeeAppFakePumpTest.java、hilt-android-testing/main/java/dagger/hilt/android/testing/UninstallModules.java、hilt-android-testing/main/java/dagger/hilt/testing/TestInstallIn.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：BindingGraphSubject.java、CoffeeAppFakeHeaterTest.java、CoffeeAppFakePumpTest.java、UninstallModules.java、TestInstallIn.java

### 测试策略与最佳实践/测试自动化与CI_CD.md

- reference 标题：测试自动化与CI/CD
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略与最佳实践/集成测试.md

- reference 标题：集成测试
- 生成页：测试策略与最佳实践/集成测试.md（集成测试）
- KnowledgeUnit：unit_id=unit-36776209056e, unit_type=test_doc, domain_id=domain-64363b4da66f, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.06
- key_source_coverage：0.13
- missing_key_sources：examples/bazel/javatests/example/hilt/CoffeeAppFakeHeaterTest.java、hilt-android-testing/main/java/dagger/hilt/android/internal/testing/TestComponentData.java、hilt-android-testing/main/java/dagger/hilt/android/internal/testing/TestInjector.java、hilt-android-testing/main/java/dagger/hilt/android/testing/BindValue.java、hilt-android-testing/main/java/dagger/hilt/android/testing/CustomTestApplication.java、hilt-android-testing/main/java/dagger/hilt/android/testing/UninstallModules.java、hilt-android-testing/main/java/dagger/hilt/testing/TestInstallIn.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CoffeeAppFakeHeaterTest.java、TestComponentData.java、TestInjector.java、BindValue.java、CustomTestApplication.java、UninstallModules.java、TestInstallIn.java

### 示例与教程/Android应用示例.md

- reference 标题：Android应用示例
- 生成页：API-参考/hilt-core.md（API：hilt-core）
- KnowledgeUnit：unit_id=unit-ce9d5b292b4c, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：2
- skeleton_score：0.53
- key_source_coverage：0.50
- missing_key_sources：README.md、examples/bazel/java/example/hilt/CoffeeApp.java、examples/bazel/javatests/example/hilt/CoffeeAppFakeHeaterTest.java、examples/bazel/javatests/example/hilt/CoffeeAppFakePumpTest.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：README.md、CoffeeApp.java、CoffeeAppFakeHeaterTest.java、CoffeeAppFakePumpTest.java

### 示例与教程/基础示例.md

- reference 标题：基础示例
- 生成页：示例与教程/基础示例.md（基础示例）
- KnowledgeUnit：unit_id=unit-dd88e735d50f, unit_type=example_doc, domain_id=domain-be9fa08103e5, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.13
- key_source_coverage：0.50
- missing_key_sources：examples/maven/coffee/pom.xml、examples/maven/coffee/src/main/java/example/dagger/ElectricHeater.java、examples/maven/coffee/src/main/java/example/dagger/HeaterModule.java、examples/maven/coffee/src/main/java/example/dagger/PumpModule.java、examples/maven/coffee/src/main/java/example/dagger/Thermosiphon.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：pom.xml、ElectricHeater.java、HeaterModule.java、PumpModule.java、Thermosiphon.java

### 示例与教程/实践教程.md

- reference 标题：实践教程
- 生成页：API-参考/Provides-与--Binds-注解详解.md（@Provides 与 @Binds 注解详解）
- KnowledgeUnit：unit_id=unit-d14dba3d1de2, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：3
- skeleton_score：0.43
- key_source_coverage：0.18
- missing_key_sources：README.md、dagger-runtime/main/java/dagger/Binds.java、dagger-runtime/main/java/dagger/Lazy.java、dagger-runtime/main/java/dagger/Provides.java、dagger-runtime/main/java/dagger/Reusable.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeApp.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeLogger.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeMaker.java、examples/maven/coffee/src/main/java/example/dagger/ElectricHeater.java、examples/maven/coffee/src/main/java/example/dagger/Heater.java、examples/maven/coffee/src/main/java/example/dagger/HeaterModule.java、examples/maven/coffee/src/main/java/example/dagger/Pump.java、examples/maven/coffee/src/main/java/example/dagger/PumpModule.java、examples/maven/coffee/src/main/java/example/dagger/Thermosiphon.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：README.md、Binds.java、Lazy.java、Provides.java、Reusable.java、CoffeeApp.java、CoffeeLogger.java、CoffeeMaker.java

### 示例与教程/示例与教程.md

- reference 标题：示例与教程
- 生成页：示例与教程/示例与教程.md（示例与教程）
- KnowledgeUnit：unit_id=unit-897c95498831, unit_type=example_doc, domain_id=domain-be9fa08103e5, readiness=compose_ready, child_digests=1, planned_key_sources=8, grounded_key_sources=3, grounding_refs=5
- reuse_count：1
- skeleton_score：0.08
- key_source_coverage：0.29
- missing_key_sources：README.md、dagger-android/main/java/dagger/android/AndroidInjection.java、dagger-android/main/java/dagger/android/DaggerApplication.java、dagger-producers/main/java/dagger/producers/Producer.java、dagger-producers/main/java/dagger/producers/Production.java、examples/maven/coffee/src/main/java/example/dagger/HeaterModule.java、examples/maven/coffee/src/main/java/example/dagger/PumpModule.java、examples/maven/coffee/src/main/java/example/dagger/Thermosiphon.java、hilt-core/main/java/dagger/hilt/DefineComponent.java、hilt-core/main/java/dagger/hilt/EntryPoint.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：README.md、AndroidInjection.java、DaggerApplication.java、Producer.java、Production.java、HeaterModule.java、PumpModule.java、Thermosiphon.java

### 示例与教程/高级应用示例.md

- reference 标题：高级应用示例
- 生成页：API-参考/异步处理与生产者.md（异步处理与生产者）
- KnowledgeUnit：unit_id=unit-067191dc7b9e, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：2
- skeleton_score：0.67
- key_source_coverage：0.29
- missing_key_sources：CONTRIBUTING.md、README.md、dagger-runtime/main/java/dagger/Binds.java、dagger-runtime/main/java/dagger/Lazy.java、dagger-runtime/main/java/dagger/Provides.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeApp.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeLogger.java、examples/maven/coffee/src/main/java/example/dagger/CoffeeMaker.java、examples/maven/coffee/src/main/java/example/dagger/HeaterModule.java、examples/maven/coffee/src/main/java/example/dagger/PumpModule.java、hilt-core/main/java/dagger/hilt/DefineComponent.java、hilt-core/main/java/dagger/hilt/InstallIn.java
- 结论：图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CONTRIBUTING.md、README.md、Binds.java、Lazy.java、Provides.java、CoffeeApp.java、CoffeeLogger.java、CoffeeMaker.java

### 编译时处理机制/代码生成策略.md

- reference 标题：代码生成策略
- 生成页：编译工具链/编译时处理机制/代码生成策略.md（代码生成策略）
- KnowledgeUnit：unit_id=unit-ff7e3fffea18, unit_type=module_doc, domain_id=domain-7f3e824e1e4c, readiness=compose_ready, child_digests=0, planned_key_sources=10, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.06
- key_source_coverage：0.22
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/componentgenerator/ComponentGenerator.java、dagger-compiler/main/java/dagger/internal/codegen/writing/ComponentCreatorImplementation.java、dagger-compiler/main/java/dagger/internal/codegen/writing/ComponentImplementation.java、dagger-compiler/main/java/dagger/internal/codegen/writing/GeneratedImplementation.java、dagger-compiler/main/java/dagger/internal/codegen/writing/MembersInjectorGenerator.java、dagger-compiler/main/java/dagger/internal/codegen/writing/ModuleGenerator.java、dagger-compiler/main/java/dagger/internal/codegen/writing/SwitchingProviders.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ComponentGenerator.java、ComponentCreatorImplementation.java、ComponentImplementation.java、GeneratedImplementation.java、MembersInjectorGenerator.java、ModuleGenerator.java、SwitchingProviders.java

### 编译时处理机制/依赖图构建.md

- reference 标题：依赖图构建
- 生成页：编译工具链/编译时处理机制/依赖图构建.md（依赖图构建）
- KnowledgeUnit：unit_id=unit-2cb836808948, unit_type=module_doc, domain_id=domain-7f3e824e1e4c, readiness=compose_ready, child_digests=0, planned_key_sources=10, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.17
- key_source_coverage：0.15
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codecgen/binding/ComponentNodeImpl.java、dagger-compiler/main/java/dagger/internal/codecgen/binding/DependencyEdgeImpl.java、dagger-compiler/main/java/dagger/internal/codecgen/bindinggraphvalidation/CompositeBindingGraphPlugin.java、dagger-compiler/main/java/dagger/internal/codecgen/bindinggraphvalidation/DependencyCycleValidator.java、dagger-compiler/main/java/dagger/internal/codecgen/bindinggraphvalidation/DuplicateBindingsValidator.java、dagger-compiler/main/java/dagger/internal/codecgen/bindinggraphvalidation/IncompatiblyScopedBindingsValidator.java、dagger-compiler/main/java/dagger/internal/codecgen/bindinggraphvalidation/MissingBindingValidator.java、dagger-compiler/main/java/dagger/internal/codecgen/bindinggraphvalidation/SubcomponentFactoryMethodValidator.java、dagger-compiler/main/java/dagger/internal/codecgen/model/Binding.java、dagger-compiler/main/java/dagger/internal/codecgen/model/DependencyRequest.java、dagger-compiler/main/java/dagger/internal/codecgen/model/Key.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ComponentNodeImpl.java、DependencyEdgeImpl.java、CompositeBindingGraphPlugin.java、DependencyCycleValidator.java、DuplicateBindingsValidator.java、IncompatiblyScopedBindingsValidator.java、MissingBindingValidator.java、SubcomponentFactoryMethodValidator.java

### 编译时处理机制/增量编译优化.md

- reference 标题：增量编译优化
- 生成页：编译工具链/编译时处理机制/增量编译优化.md（增量编译优化）
- KnowledgeUnit：unit_id=unit-3d0325e3e95c, unit_type=module_doc, domain_id=domain-7f3e824e1e4c, readiness=compose_ready, child_digests=0, planned_key_sources=10, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.08
- key_source_coverage：0.14
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/DelegateComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/KspComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/compileroption/ProcessingEnvironmentCompilerOptions.java、dagger-compiler/main/java/dagger/internal/codegen/validation/ComponentValidator.java、dagger-compiler/main/java/dagger/internal/codegen/validation/InjectValidator.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ComponentProcessor.java、DelegateComponentProcessor.java、KspComponentProcessor.java、ProcessingEnvironmentCompilerOptions.java、ComponentValidator.java、InjectValidator.java

### 编译时处理机制/注解处理基础.md

- reference 标题：注解处理基础
- 生成页：编译工具链/编译时处理机制/注解处理基础.md（注解处理基础）
- KnowledgeUnit：unit_id=unit-3e055844873d, unit_type=module_doc, domain_id=domain-7f3e824e1e4c, readiness=compose_ready, child_digests=0, planned_key_sources=10, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.07
- key_source_coverage：0.11
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/DelegateComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/compileroption/ProcessingEnvironmentCompilerOptions.java、dagger-compiler/main/java/dagger/internal/codegen/processingstep/ComponentProcessingStep.java、dagger-compiler/main/java/dagger/internal/codegen/processingstep/InjectProcessingStep.java、dagger-compiler/main/java/dagger/internal/codegen/processingstep/MapKeyProcessingStep.java、dagger-compiler/main/java/dagger/internal/codegen/processingstep/ModuleProcessingStep.java、dagger-compiler/main/java/dagger/internal/codegen/processingstep/TypeCheckingProcessingStep.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ComponentProcessor.java、DelegateComponentProcessor.java、ProcessingEnvironmentCompilerOptions.java、ComponentProcessingStep.java、InjectProcessingStep.java、MapKeyProcessingStep.java、ModuleProcessingStep.java、TypeCheckingProcessingStep.java

### 编译时处理机制/编译时处理机制.md

- reference 标题：编译时处理机制
- 生成页：编译工具链/编译时处理机制.md（编译时处理机制）
- KnowledgeUnit：unit_id=unit-9471ea7609c1, unit_type=module_doc, domain_id=domain-7f3e824e1e4c, readiness=compose_ready, child_digests=5, planned_key_sources=10, grounded_key_sources=3, grounding_refs=5
- reuse_count：2
- skeleton_score：0.07
- key_source_coverage：0.13
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/DelegateComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/KspComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/ProcessingEnvironmentModule.java、dagger-compiler/main/java/dagger/internal/codegen/binding/BindingGraphFactory.java、dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/BindingGraphValidationModule.java、dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/CompositeBindingGraphPlugin.java、dagger-compiler/main/java/dagger/internal/codegen/compileroption/CompilerOptions.java、dagger-compiler/main/java/dagger/internal/codegen/compileroption/ProcessingEnvironmentCompilerOptions.java、dagger-compiler/main/java/dagger/internal/codegen/processingstep/ComponentProcessingStep.java、dagger-compiler/main/java/dagger/internal/codegen/processingstep/InjectProcessingStep.java、dagger-compiler/main/java/dagger/internal/codegen/processingstep/ModuleProcessingStep.java、dagger-compiler/main/java/dagger/internal/codegen/validation/ComponentValidator.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ComponentProcessor.java、DelegateComponentProcessor.java、KspComponentProcessor.java、ProcessingEnvironmentModule.java、BindingGraphFactory.java、BindingGraphValidationModule.java、CompositeBindingGraphPlugin.java、CompilerOptions.java

### 编译时处理机制/验证与错误处理.md

- reference 标题：验证与错误处理
- 生成页：编译工具链/编译时处理机制/验证与错误处理.md（验证与错误处理）
- KnowledgeUnit：unit_id=unit-7aaeb3ab2258, unit_type=module_doc, domain_id=domain-7f3e824e1e4c, readiness=compose_ready, child_digests=0, planned_key_sources=10, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.13
- key_source_coverage：0.30
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codec/binding/ErrorMessages.java、dagger-compiler/main/java/dagger/internal/codec/model/BindingGraphPlugin.java、dagger-compiler/main/java/dagger/internal/codec/validation/BindingGraphValidator.java、dagger-compiler/main/java/dagger/internal/codec/validation/ComponentValidator.java、dagger-compiler/main/java/dagger/internal/codec/validation/DiagnosticReporterFactory.java、dagger-compiler/main/java/dagger/internal/codec/validation/ModuleValidator.java、dagger-compiler/main/java/dagger/internal/codec/validation/ValidationBindingGraphPlugin.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ErrorMessages.java、BindingGraphPlugin.java、BindingGraphValidator.java、ComponentValidator.java、DiagnosticReporterFactory.java、ModuleValidator.java、ValidationBindingGraphPlugin.java

### 贡献指南.md

- reference 标题：贡献指南
- 生成页：无
- 问题：缺少对应生成页面

### 项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- KnowledgeUnit：unit_id=unit-e50a8961f419, unit_type=overview, domain_id=system, readiness=compose_ready, child_digests=1, planned_key_sources=40, grounded_key_sources=15, grounding_refs=13
- reuse_count：1
- skeleton_score：0.46
- key_source_coverage：0.38
- missing_key_sources：CONTRIBUTING.md、dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/DelegateComponentProcessor.java、dagger-runtime/main/java/dagger/Component.java、dagger-runtime/main/java/dagger/Lazy.java、gradle.properties、hilt-core/main/java/dagger/hilt/DefineComponent.java、settings.gradle.kts
- 结论：内容比 reference 更展开；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CONTRIBUTING.md、ComponentProcessor.java、DelegateComponentProcessor.java、Component.java、Lazy.java、gradle.properties、DefineComponent.java、settings.gradle.kts

### 高级特性与扩展/SPI扩展机制.md

- reference 标题：SPI扩展机制
- 生成页：编译工具链/SPI扩展机制.md（SPI扩展机制）
- KnowledgeUnit：unit_id=unit-0793a2423091, unit_type=module_doc, domain_id=domain-7f3e824e1e4c, readiness=compose_ready, child_digests=0, planned_key_sources=10, grounded_key_sources=3, grounding_refs=4
- reuse_count：2
- skeleton_score：0.19
- key_source_coverage：0.21
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codecgen/bindinggraphvalidation/CompositeBindingGraphPlugin.java、dagger-compiler/main/java/dagger/internal/codecgen/validation/DiagnosticReporterFactory.java、dagger-compiler/main/java/dagger/internal/codecgen/validation/ValidationBindingGraphPlugin.java、dagger-compiler/main/java/dagger/internal/codecgen/validation/ValidationBindingGraphPlugins.java、dagger-spi/main/java/dagger/spi/DiagnosticReporter.java、dagger-spi/main/java/dagger/spi/model/BindingGraph.java、dagger-spi/main/java/dagger/spi/model/ChildFactoryMethodEdge.java、dagger-spi/main/java/dagger/spi/model/ComponentNode.java、dagger-spi/main/java/dagger/spi/model/DependencyEdge.java、dagger-spi/main/java/dagger/spi/model/MaybeBinding.java、examples/java/dagger/example/spi/BindingGraphVisualizer.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：CompositeBindingGraphPlugin.java、DiagnosticReporterFactory.java、ValidationBindingGraphPlugin.java、ValidationBindingGraphPlugins.java、DiagnosticReporter.java、BindingGraph.java、ChildFactoryMethodEdge.java、ComponentNode.java

### 高级特性与扩展/多值绑定高级用法.md

- reference 标题：多值绑定高级用法
- 生成页：核心运行时/高级特性与扩展/多值绑定高级用法.md（多值绑定高级用法）
- KnowledgeUnit：unit_id=unit-126669654221, unit_type=module_doc, domain_id=domain-6226b0bb2dd4, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.06
- key_source_coverage：0.20
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/binding/MultibindingDeclaration.java、dagger-compiler/main/java/dagger/internal/codegen/writing/MapFactoryCreationExpression.java、dagger-compiler/main/java/dagger/internal/codegen/writing/SetFactoryCreationExpression.java、dagger-runtime/main/java/dagger/internal/MapProviderFactory.java、dagger-runtime/main/java/dagger/internal/SetBuilder.java、dagger-runtime/main/java/dagger/multibindings/Multibinds.java、dagger-runtime/test/javatests/dagger/internal/MapProviderFactoryTest.java、dagger-runtime/test/javatests/dagger/internal/SetBuilderTest.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：MultibindingDeclaration.java、MapFactoryCreationExpression.java、SetFactoryCreationExpression.java、MapProviderFactory.java、SetBuilder.java、Multibinds.java、MapProviderFactoryTest.java、SetBuilderTest.java

### 高级特性与扩展/延迟初始化与线程安全.md

- reference 标题：延迟初始化与线程安全
- 生成页：核心运行时/高级特性与扩展/延迟初始化与线程安全.md（延迟初始化与线程安全）
- KnowledgeUnit：unit_id=unit-1b67356703b5, unit_type=module_doc, domain_id=domain-6226b0bb2dd4, readiness=compose_ready, child_digests=0, planned_key_sources=8, grounded_key_sources=3, grounding_refs=4
- reuse_count：1
- skeleton_score：0.08
- key_source_coverage：0.57
- missing_key_sources：dagger-runtime/main/java/dagger/Component.java、dagger-runtime/test/javatests/dagger/internal/DoubleCheckTest.java、dagger-runtime/test/javatests/dagger/internal/SingleCheckTest.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：Component.java、DoubleCheckTest.java、SingleCheckTest.java

### 高级特性与扩展/性能监控与调试.md

- reference 标题：性能监控与调试
- 生成页：API-参考/性能监控与调试.md（性能监控与调试）
- KnowledgeUnit：unit_id=unit-ee7be44894bf, unit_type=api_doc, domain_id=domain-94ece2bc9556, readiness=compose_ready, child_digests=0, planned_key_sources=12, grounded_key_sources=4, grounding_refs=12
- reuse_count：1
- skeleton_score：0.64
- key_source_coverage：0.27
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/base/ClearableCache.java、dagger-compiler/main/java/dagger/internal/codegen/compileroption/CompilerOptions.java、dagger-compiler/main/java/dagger/internal/codegen/validation/Validation.java、dagger-producers/main/java/dagger/producers/monitoring/ProducerMonitor.java、dagger-producers/main/java/dagger/producers/monitoring/ProducerToken.java、dagger-producers/main/java/dagger/producers/monitoring/ProductionComponentMonitor.java、dagger-producers/main/java/dagger/producers/monitoring/ProductionComponentTimingRecorder.java、dagger-producers/main/java/dagger/producers/monitoring/TimingProductionComponentMonitor.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ClearableCache.java、CompilerOptions.java、Validation.java、ProducerMonitor.java、ProducerToken.java、ProductionComponentMonitor.java、ProductionComponentTimingRecorder.java、TimingProductionComponentMonitor.java

### 高级特性与扩展/自定义扩展开发.md

- reference 标题：自定义扩展开发
- 生成页：编译工具链/SPI扩展机制.md（SPI扩展机制）
- KnowledgeUnit：unit_id=unit-0793a2423091, unit_type=module_doc, domain_id=domain-7f3e824e1e4c, readiness=compose_ready, child_digests=0, planned_key_sources=10, grounded_key_sources=3, grounding_refs=4
- reuse_count：2
- skeleton_score：0.10
- key_source_coverage：0.23
- missing_key_sources：dagger-android-processor/main/java/dagger/android/processor/DelegateAndroidProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/DelegateComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/base/SourceFileGenerator.java、dagger-compiler/main/java/dagger/internal/codegen/processingstep/AssistedFactoryProcessingStep.java、dagger-compiler/main/java/dagger/internal/codegen/processingstep/InjectProcessingStep.java、dagger-compiler/main/java/dagger/internal/codegen/processingstep/ProcessingStepsModule.java、dagger-compiler/main/java/dagger/internal/codegen/validation/ValidationBindingGraphPlugins.java、hilt-compiler/main/java/dagger/hilt/android/processor/HiltProcessor.kt、hilt-compiler/main/java/dagger/hilt/processor/HiltProcessor.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：DelegateAndroidProcessor.java、ComponentProcessor.java、DelegateComponentProcessor.java、SourceFileGenerator.java、AssistedFactoryProcessingStep.java、InjectProcessingStep.java、ProcessingStepsModule.java、ValidationBindingGraphPlugins.java

### 高级特性与扩展/高级特性与扩展.md

- reference 标题：高级特性与扩展
- 生成页：核心运行时/高级特性与扩展.md（高级特性与扩展）
- KnowledgeUnit：unit_id=unit-a6b2c5f6a900, unit_type=module_doc, domain_id=domain-6226b0bb2dd4, readiness=compose_ready, child_digests=2, planned_key_sources=8, grounded_key_sources=3, grounding_refs=5
- reuse_count：2
- skeleton_score：0.07
- key_source_coverage：0.42
- missing_key_sources：dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java、dagger-compiler/main/java/dagger/internal/codegen/ServiceLoaders.java、dagger-compiler/main/java/dagger/internal/codegen/bindinggraphvalidation/CompositeBindingGraphPlugin.java、dagger-compiler/main/java/dagger/internal/codegen/validation/ExternalBindingGraphPlugins.java、dagger-producers/main/java/dagger/producers/monitoring/TimingProducerMonitor.java、dagger-runtime/main/java/dagger/BindsOptionalOf.java、dagger-spi/main/java/dagger/spi/BindingGraphPlugin.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；关键文件覆盖不足；缺少关键文件提及：ComponentProcessor.java、ServiceLoaders.java、CompositeBindingGraphPlugin.java、ExternalBindingGraphPlugins.java、TimingProducerMonitor.java、BindsOptionalOf.java、BindingGraphPlugin.java

