# dagger Reference 对比报告

生成页面：49 页
reference 页面：65 页
命中对比：31 页
缺失对比：34 页
运行模式：cold (cache_mode=clear)
LLM usage：requests=32, total_tokens=125802, page_research=16, page_enrichment=0

## 覆盖统计

- 专题页覆盖：generated 21 / reference 7
- evidence 落页：generated 48 / reference 65
- 图表达覆盖：generated 42 / reference 65
- page research 请求：16
- page enrichment 请求：0
- 已规划专题类型：专题页(13)、流程主题(8)
- 高频缺失专题：无

## 项目结论

- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- evidence block 已进入页面，但覆盖率和密度仍低于 reference
- Mermaid/结构图表达仍然不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 核心模块/dagger-runtime.md 被 14 个 reference 页面共享映射
- 核心模块/dagger-producers.md 被 2 个 reference 页面共享映射
- 核心模块/hilt-core.md 被 3 个 reference 页面共享映射
- 专题/module-capability/module-76f4cd583a47-plugin-buildSrc能力：扩展机制.md 被 2 个 reference 页面共享映射
- 专题/module-capability/module-4157350ffac0-plugin-java能力：扩展机制.md 被 4 个 reference 页面共享映射

## 额外生成页面

- 专题/module-capability/module-433a4f9c971f-model-hilt-android能力：领域建模.md (hilt-android能力：领域建模, 46 行)
- 专题/module-capability/module-43739af0b616-service-dagger-grpc-server-processor能力：服务协作.md (dagger-grpc-server-processor能力：服务协作, 36 行)
- 专题/module-capability/module-4393e1eb6826-library-dagger-android能力：扩展机制.md (dagger-android能力：扩展机制, 42 行)
- 专题/module-capability/module-5c4b09a603cd-model-javatests能力：领域建模.md (javatests能力：领域建模, 58 行)
- 专题/module-capability/module-61993fb5083d-handler-dagger-android-processor能力：处理流程.md (dagger-android-processor能力：处理流程, 40 行)
- 专题/module-capability/module-8d5ac3555d39-service-dagger-grpc-server-annotations能力：服务协作.md (dagger-grpc-server-annotations能力：服务协作, 34 行)
- 专题/module-capability/module-a873a55f20b4-library-dagger-android-support能力：扩展机制.md (dagger-android-support能力：扩展机制, 40 行)
- 专题/module-capability/module-abe43725d4fe-model-dagger-spi能力：领域建模.md (dagger-spi能力：领域建模, 49 行)
- 专题/module-capability/module-ba4d8ae330d4-model-hilt-compiler能力：领域建模.md (hilt-compiler能力：领域建模, 47 行)
- 专题/module-capability/module-cd9fb4d8e220-plugin-dagger-android-proguard-processor能力：扩展机制.md (dagger-android-proguard-processor能力：扩展机制, 33 行)
- 专题/module-capability/module-cf82c270785f-component-hilt-core能力：界面组成.md (hilt-core能力：界面组成, 43 行)
- 专题/process/process-assertnostartednodes-flow-流程主题：assertNoStartedNodes-flow.md (流程主题：assertNoStartedNodes flow, 31 行)
- 专题/process/process-assertnotstarted-flow-流程主题：assertNotStarted-flow.md (流程主题：assertNotStarted flow, 45 行)
- 专题/process/process-assertstarted-flow-流程主题：assertStarted-flow.md (流程主题：assertStarted flow, 43 行)
- 专题/process/process-getsavedstatehandle-flow-流程主题：getSavedStateHandle-flow.md (流程主题：getSavedStateHandle flow, 31 行)
- 专题/process/process-handleinput-flow-流程主题：handleInput-flow.md (流程主题：handleInput flow, 21 行)
- 专题/process/process-run-flow-流程主题：run-flow.md (流程主题：run flow, 21 行)
- 专题/process/process-testdelaycomponentreadyafterstart_fails-flow-流程主题：testDelayComponentReadyAfterStart_fails-flow.md (流程主题：testDelayComponentReadyAfterStart_fails flow, 21 行)
- 核心模块/buildSrc.md (模块：buildSrc, 103 行)
- 核心模块/dagger-android-processor.md (模块：dagger-android-processor, 77 行)
- 核心模块/dagger-android-proguard-processor.md (模块：dagger-android-proguard-processor, 55 行)
- 核心模块/dagger-android-support.md (模块：dagger-android-support, 72 行)
- 核心模块/dagger-compiler.md (模块：dagger-compiler, 91 行)
- 核心模块/dagger-grpc-server-annotations.md (模块：dagger-grpc-server-annotations, 66 行)
- 核心模块/dagger-grpc-server-processor.md (模块：dagger-grpc-server-processor, 68 行)
- 核心模块/dagger-grpc-server.md (模块：dagger-grpc-server, 74 行)
- 核心模块/dagger-kythe.md (模块：dagger-kythe, 57 行)
- 核心模块/dagger-lint-android.md (模块：dagger-lint-android, 52 行)
- 核心模块/dagger-lint.md (模块：dagger-lint, 105 行)
- 核心模块/dagger-spi.md (模块：dagger-spi, 88 行)
- 核心模块/dagger-testing.md (模块：dagger-testing, 59 行)
- 核心模块/gradle.md (模块：gradle, 43 行)
- 核心模块/hilt-android-testing.md (模块：hilt-android-testing, 76 行)
- 核心模块/hilt-android.md (模块：hilt-android, 84 行)
- 核心模块/hilt-compiler.md (模块：hilt-compiler, 85 行)
- 核心模块/hilt-testing.md (模块：hilt-testing, 61 行)
- 核心模块/java.md (模块：java, 132 行)
- 核心模块/util.md (模块：util, 54 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API参考文档/API参考文档.md | 核心模块/dagger-runtime.md | 594/94 | 239/34 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidinjector.java、hasandroidinjector.java、cancellationpolicy.java、producer.java、producermodule.java、produces.java、production.java、productioncomponent.java |
| API参考文档/Android API.md | 核心模块/dagger-android.md | 398/77 | 168/22 | 21/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、androidinjectordescriptor.java、contributesandroidinjectorprocessingstep.java、androidsupportinjection.java、androidsupportinjectionmodule.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java |
| API参考文档/Hilt API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考文档/异步处理API.md | 核心模块/dagger-producers.md | 367/74 | 163/21 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：provisiondependencyonproducerbindingvalidator.java、dependencymethodproducercreationexpression.java、producercreationexpression.java、cancellationpolicy.java、productioncomponent.java、productionsubcomponent.java、abstractproducer.java、dependencymethodproducer.java |
| API参考文档/编译时API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考文档/运行时API.md | 核心模块/dagger-runtime.md | 388/94 | 102/34 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bindsinstance.java、membersinjector.java、subcomponent.java、assisted.java、assistedfactory.java、intomap.java、intoset.java |
| Android集成开发/@ContributesAndroidInjector注解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| Android集成开发/Activity注入.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| Android集成开发/Android生命周期管理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| Android集成开发/Android集成开发.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| Android集成开发/Fragment注入.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| Android集成开发/Service注入.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| Android集成开发/其他组件注入.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| Hilt框架详解/@HiltAndroidApp与注解使用.md | 核心模块/hilt-core.md | 315/81 | 90/24 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：coffeeapp.java、androidentrypoint.java、hiltandroidapp.java、generatesrootinput.java |
| Hilt框架详解/Hilt核心概念与架构.md | 系统架构.md | 295/277 | 127/122 | 17/1 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、readme.md、singletoncomponent.java、definecomponentnoparent.java |
| Hilt框架详解/Hilt框架详解.md | 核心模块/hilt-core.md | 328/81 | 134/24 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidentrypoint.java、hiltandroidapp.java、generatesrootinput.java、generatedcomponent.java、generatedcomponentmanager.java |
| Hilt框架详解/模块安装与依赖管理.md | 核心模块/hilt-core.md | 289/81 | 124/24 | 18/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：duplicatebindingsvalidator.java、moduleprocessingstep.java、modulevalidator.java、activitycomponent.java、definecomponentnoparent.java |
| Hilt框架详解/测试支持与模拟.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| Hilt框架详解/组件树结构与生命周期.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| Hilt框架详解/编译时处理与代码生成.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 异步处理与生产者.md | 核心模块/dagger-producers.md | 296/74 | 131/21 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cancellationpolicy.java、productioncomponent.java、productionsubcomponent.java、abstractproducer.java、dependencymethodproducer.java、producermonitor.java、productioncomponentmonitor.java、producercancellationtest.java |
| 快速开始.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 故障排除与调试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 构建与部署.md | 工作流与部署.md | 339/501 | 136/3 | 20/0 | 7/0 | 内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：ci.yml、contributing.md、build.gradle.kt、daggerconventionplugin.kt、libs.versions.toml、settings.gradle.kt |
| 核心概念/作用域与生命周期管理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/依赖注入基础理论.md | 核心模块/dagger-runtime.md | 323/94 | 138/34 | 19/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：membersinjector.java、doublecheck.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java |
| 核心概念/核心概念.md | 核心模块/dagger-runtime.md | 326/94 | 128/34 | 17/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：scopes.java、componentdescriptor.java、scope.java、bindsinstance.java、coffeeapp.java、heatermodule.java、pumpmodule.java、thermosiphon.java |
| 核心概念/核心注解详解/@Component 注解详解.md | 核心模块/dagger-runtime.md | 211/94 | 88/34 | 0/1 | 4/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、componentannotation.java、componentgenerator.java、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、coffeeshop.java |
| 核心概念/核心注解详解/@Inject 注解详解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/核心注解详解/@Module 注解详解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/核心注解详解/@Provides 与 @Binds 注解详解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/核心注解详解/核心注解详解.md | 核心模块/dagger-runtime.md | 262/94 | 98/34 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、componentprocessingstep.java、injectedfieldsignature.java、coffeeapp.java、coffeeappcomponent.java、heater.java、heatermodule.java、pump.java |
| 核心概念/组件与模块系统/模块组织与管理.md | 核心模块/dagger-runtime.md | 317/94 | 117/34 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：modulevalidator.java、providesmethodvalidator.java、moduleproxies.java、coffeeapp.java |
| 核心概念/组件与模块系统/组件与模块系统.md | 核心模块/dagger-runtime.md | 331/94 | 139/34 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、bindinggraphvalidationmodule.java、subcomponentfactorymethodvalidator.java、componentpath.java、componenthierarchyvalidator.java、componentimplementation.java、subcomponent.java、accountmodule.java |
| 核心概念/组件与模块系统/组件关系设计/子组件机制.md | 核心模块/javatests.md | 340/101 | 144/38 | 23/1 | 10/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：compositebindinggraphplugin.java、dependencycyclevalidator.java、subcomponentfactorymethodvalidator.java、currentimplementationsubcomponent.java、toplevelimplementationcomponent.java、subcomponent.java、usercommandsrouter.java、simpleactivity.kt |
| 核心概念/组件与模块系统/组件关系设计/组件依赖关系.md | 核心模块/dagger-runtime.md | 266/94 | 112/34 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentrequirementrequestrepresentation.java、componentdependencybinding.java、componentdescriptor.java、componentrequirement.java、dependencycyclevalidator.java、bindingkind.java、componentvalidator.java、dependencymethodproducercreationexpression.java |
| 核心概念/组件与模块系统/组件关系设计/组件关系设计.md | 核心模块/dagger-runtime.md | 266/94 | 103/34 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentdependencybinding.java、subcomponentcreatorbinding.java、subcomponentdeclaration.java、subcomponentfactorymethodvalidator.java、componentgenerator.java、componenthierarchyvalidator.java、subcomponentcreatorrequestrepresentation.java、productionsubcomponent.java |
| 核心概念/组件与模块系统/组件基础概念.md | 核心模块/dagger-runtime.md | 231/94 | 90/34 | 0/1 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：coffeeapp.java、componentdescriptor.java、componentgenerator.java、membersinjector.java、mycomponent.java |
| 核心概念/组件与模块系统/组件构建器与工厂.md | 核心模块/dagger-runtime.md | 358/94 | 111/34 | 15/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：bindsinstance.java、builderbindsinstanceparametertest.java、buildertest.java、componentcreatordescriptor.java、componentcreatorkind.java、componentcreatorvalidator.java、errormessages.java、mycomponent.java |
| 核心概念/绑定与提供者模式.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/Android测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/单元测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/性能测试与监控.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/测试策略与最佳实践.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/测试自动化与CI_CD.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/集成测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例与教程/Android应用示例.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例与教程/基础示例.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例与教程/实践教程.md | 核心模块/dagger-runtime.md | 350/94 | 159/34 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、electricheater.java、heater.java、heatermodule.java |
| 示例与教程/示例与教程.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例与教程/高级应用示例.md | 核心模块/dagger-runtime.md | 319/94 | 146/34 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、producer.java、productioncomponent.java、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java |
| 编译时处理机制/代码生成策略.md | 专题/module-capability/module-76f4cd583a47-plugin-buildSrc能力：扩展机制.md | 361/65 | 159/44 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：sourcefilegenerator.java、componentgenerator.java、componentcreatorimplementation.java、componentimplementation.java、factorygenerator.java、generatedimplementation.java、membersinjectorgenerator.java、modulegenerator.java |
| 编译时处理机制/依赖图构建.md | 专题/module-capability/module-b542d0916206-model-dagger-compiler能力：领域建模.md | 335/51 | 164/30 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bindinggraphfactory.java、componentnodeimpl.java、dependencyedgeimpl.java、compositebindinggraphplugin.java、dependencycyclevalidator.java、duplicatebindingsvalidator.java、incompatiblyscopedbindingsvalidator.java、missingbindingvalidator.java |
| 编译时处理机制/增量编译优化.md | 专题/module-capability/module-4157350ffac0-plugin-java能力：扩展机制.md | 268/76 | 122/55 | 0/1 | 7/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、kspcomponentprocessor.java、clearablecache.java、processingenvironmentcompileroptions.java、componentvalidator.java、injectvalidator.java |
| 编译时处理机制/注解处理基础.md | 专题/module-capability/module-4157350ffac0-plugin-java能力：扩展机制.md | 296/76 | 135/55 | 0/1 | 6/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、processingenvironmentcompileroptions.java、componentprocessingstep.java、injectprocessingstep.java、mapkeyprocessingstep.java、moduleprocessingstep.java、processingstepsmodule.java |
| 编译时处理机制/编译时处理机制.md | 专题/module-capability/module-4157350ffac0-plugin-java能力：扩展机制.md | 327/76 | 135/55 | 0/1 | 5/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、kspcomponentprocessor.java、processingenvironmentmodule.java、validationreport.java、bindinggraphfactory.java、bindinggraphvalidationmodule.java、compositebindinggraphplugin.java |
| 编译时处理机制/验证与错误处理.md | 专题/module-capability/module-76f4cd583a47-plugin-buildSrc能力：扩展机制.md | 399/65 | 180/44 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：validationreport.java、errormessages.java、bindinggraphplugin.java、diagnosticreporter.java、bindinggraphvalidator.java、componentvalidator.java、diagnosticreporterfactory.java、modulevalidator.java |
| 贡献指南.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 项目概述.md | 项目概述.md | 244/77 | 114/8 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、componentprocessor.java、delegatecomponentprocessor.java、component.java、lazy.java、module.java、subcomponent.java |
| 高级特性与扩展/SPI扩展机制.md | 专题/module-capability/module-4157350ffac0-plugin-java能力：扩展机制.md | 393/76 | 185/55 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：serviceloaders.java、compositebindinggraphplugin.java、diagnosticreporterfactory.java、externalbindinggraphplugins.java、validationbindinggraphplugin.java、validationbindinggraphplugins.java、bindinggraphplugin.java、diagnosticreporter.java |
| 高级特性与扩展/多值绑定高级用法.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级特性与扩展/延迟初始化与线程安全.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级特性与扩展/性能监控与调试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级特性与扩展/自定义扩展开发.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级特性与扩展/高级特性与扩展.md | 缺失 | - | - | - | - | 缺少对应生成页面 |

## 逐文件详情

### API参考文档/API参考文档.md

- reference 标题：API参考文档
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：216
- 页面类型：other / module
- 行数：594 / 94
- 段落行数：239 / 34
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：binds.java、component.java、lazy.java、mapkey.java、module.java、provides.java、reusable.java
- reference 关键文件未覆盖：androidinjector.java、hasandroidinjector.java、cancellationpolicy.java、producer.java、producermodule.java、produces.java、production.java、productioncomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidinjector.java、hasandroidinjector.java、cancellationpolicy.java、producer.java、producermodule.java、produces.java、production.java、productioncomponent.java

### API参考文档/Android API.md

- reference 标题：Android API
- 生成页：核心模块/dagger-android.md（模块：dagger-android）
- 匹配分数：64
- 页面类型：other / module
- 行数：398 / 77
- 段落行数：168 / 22
- Evidence：21 / 1
- Mermaid：8 / 1
- 文件提及重合：androidinjection.java、daggeractivity.java、daggerapplication.java、daggerfragment.java、daggerservice.java
- reference 关键文件未覆盖：readme.md、androidinjectordescriptor.java、contributesandroidinjectorprocessingstep.java、androidsupportinjection.java、androidsupportinjectionmodule.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、androidinjectordescriptor.java、contributesandroidinjectorprocessingstep.java、androidsupportinjection.java、androidsupportinjectionmodule.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java

### API参考文档/Hilt API.md

- reference 标题：Hilt API
- 生成页：无
- 问题：缺少对应生成页面

### API参考文档/异步处理API.md

- reference 标题：异步处理API
- 生成页：核心模块/dagger-producers.md（模块：dagger-producers）
- 匹配分数：66
- 页面类型：other / module
- 行数：367 / 74
- 段落行数：163 / 21
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：produced.java、producer.java、producermodule.java、produces.java、productionscope.java
- reference 关键文件未覆盖：provisiondependencyonproducerbindingvalidator.java、dependencymethodproducercreationexpression.java、producercreationexpression.java、cancellationpolicy.java、productioncomponent.java、productionsubcomponent.java、abstractproducer.java、dependencymethodproducer.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：provisiondependencyonproducerbindingvalidator.java、dependencymethodproducercreationexpression.java、producercreationexpression.java、cancellationpolicy.java、productioncomponent.java、productionsubcomponent.java、abstractproducer.java、dependencymethodproducer.java

### API参考文档/编译时API.md

- reference 标题：编译时API
- 生成页：无
- 问题：缺少对应生成页面

### API参考文档/运行时API.md

- reference 标题：运行时API
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：82
- 页面类型：other / module
- 行数：388 / 94
- 段落行数：102 / 34
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：binds.java、component.java、lazy.java、mapkey.java、module.java、provides.java、reusable.java
- reference 关键文件未覆盖：bindsinstance.java、membersinjector.java、subcomponent.java、assisted.java、assistedfactory.java、intomap.java、intoset.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bindsinstance.java、membersinjector.java、subcomponent.java、assisted.java、assistedfactory.java、intomap.java、intoset.java

### Android集成开发/@ContributesAndroidInjector注解.md

- reference 标题：@ContributesAndroidInjector注解
- 生成页：无
- 问题：缺少对应生成页面

### Android集成开发/Activity注入.md

- reference 标题：Activity注入
- 生成页：无
- 问题：缺少对应生成页面

### Android集成开发/Android生命周期管理.md

- reference 标题：Android生命周期管理
- 生成页：无
- 问题：缺少对应生成页面

### Android集成开发/Android集成开发.md

- reference 标题：Android集成开发
- 生成页：无
- 问题：缺少对应生成页面

### Android集成开发/Fragment注入.md

- reference 标题：Fragment注入
- 生成页：无
- 问题：缺少对应生成页面

### Android集成开发/Service注入.md

- reference 标题：Service注入
- 生成页：无
- 问题：缺少对应生成页面

### Android集成开发/其他组件注入.md

- reference 标题：其他组件注入
- 生成页：无
- 问题：缺少对应生成页面

### Hilt框架详解/@HiltAndroidApp与注解使用.md

- reference 标题：@HiltAndroidApp与注解使用
- 生成页：核心模块/hilt-core.md（模块：hilt-core）
- 匹配分数：66
- 页面类型：other / module
- 行数：315 / 81
- 段落行数：90 / 24
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：definecomponent.java、entrypoint.java、entrypoints.java、installin.java、package-info.java
- reference 关键文件未覆盖：coffeeapp.java、androidentrypoint.java、hiltandroidapp.java、generatesrootinput.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：coffeeapp.java、androidentrypoint.java、hiltandroidapp.java、generatesrootinput.java

### Hilt框架详解/Hilt核心概念与架构.md

- reference 标题：Hilt核心概念与架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：102
- 页面类型：architecture / architecture
- 行数：295 / 277
- 段落行数：127 / 122
- Evidence：17 / 1
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、readme.md、singletoncomponent.java、definecomponentnoparent.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、readme.md、singletoncomponent.java、definecomponentnoparent.java

### Hilt框架详解/Hilt框架详解.md

- reference 标题：Hilt框架详解
- 生成页：核心模块/hilt-core.md（模块：hilt-core）
- 匹配分数：66
- 页面类型：other / module
- 行数：328 / 81
- 段落行数：134 / 24
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：definecomponent.java、entrypoint.java、entrypoints.java、installin.java、singletoncomponent.java
- reference 关键文件未覆盖：androidentrypoint.java、hiltandroidapp.java、generatesrootinput.java、generatedcomponent.java、generatedcomponentmanager.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidentrypoint.java、hiltandroidapp.java、generatesrootinput.java、generatedcomponent.java、generatedcomponentmanager.java

### Hilt框架详解/模块安装与依赖管理.md

- reference 标题：模块安装与依赖管理
- 生成页：核心模块/hilt-core.md（模块：hilt-core）
- 匹配分数：124
- 页面类型：module / module
- 行数：289 / 81
- 段落行数：124 / 24
- Evidence：18 / 1
- Mermaid：8 / 1
- 文件提及重合：definecomponent.java、installin.java、singletoncomponent.java
- reference 关键文件未覆盖：duplicatebindingsvalidator.java、moduleprocessingstep.java、modulevalidator.java、activitycomponent.java、definecomponentnoparent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：duplicatebindingsvalidator.java、moduleprocessingstep.java、modulevalidator.java、activitycomponent.java、definecomponentnoparent.java

### Hilt框架详解/测试支持与模拟.md

- reference 标题：测试支持与模拟
- 生成页：无
- 问题：缺少对应生成页面

### Hilt框架详解/组件树结构与生命周期.md

- reference 标题：组件树结构与生命周期
- 生成页：无
- 问题：缺少对应生成页面

### Hilt框架详解/编译时处理与代码生成.md

- reference 标题：编译时处理与代码生成
- 生成页：无
- 问题：缺少对应生成页面

### 异步处理与生产者.md

- reference 标题：异步处理与生产者
- 生成页：核心模块/dagger-producers.md（模块：dagger-producers）
- 匹配分数：64
- 页面类型：other / module
- 行数：296 / 74
- 段落行数：131 / 21
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：produced.java、producer.java、producermodule.java、produces.java、productionscope.java
- reference 关键文件未覆盖：cancellationpolicy.java、productioncomponent.java、productionsubcomponent.java、abstractproducer.java、dependencymethodproducer.java、producermonitor.java、productioncomponentmonitor.java、producercancellationtest.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cancellationpolicy.java、productioncomponent.java、productionsubcomponent.java、abstractproducer.java、dependencymethodproducer.java、producermonitor.java、productioncomponentmonitor.java、producercancellationtest.java

### 快速开始.md

- reference 标题：快速开始
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除与调试.md

- reference 标题：故障排除与调试
- 生成页：无
- 问题：缺少对应生成页面

### 构建与部署.md

- reference 标题：构建与部署
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：98
- 页面类型：workflow / workflow
- 行数：339 / 501
- 段落行数：136 / 3
- Evidence：20 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：ci.yml、contributing.md、build.gradle.kt、daggerconventionplugin.kt、libs.versions.toml、settings.gradle.kt
- 结论：内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：ci.yml、contributing.md、build.gradle.kt、daggerconventionplugin.kt、libs.versions.toml、settings.gradle.kt

### 核心概念/作用域与生命周期管理.md

- reference 标题：作用域与生命周期管理
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/依赖注入基础理论.md

- reference 标题：依赖注入基础理论
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：152
- 页面类型：other / module
- 行数：323 / 94
- 段落行数：138 / 34
- Evidence：19 / 1
- Mermaid：7 / 1
- 文件提及重合：binds.java、component.java、lazy.java、module.java、provides.java
- reference 关键文件未覆盖：membersinjector.java、doublecheck.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：membersinjector.java、doublecheck.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：64
- 页面类型：other / module
- 行数：326 / 94
- 段落行数：128 / 34
- Evidence：17 / 1
- Mermaid：8 / 1
- 文件提及重合：binds.java、component.java、module.java、provides.java、reusable.java
- reference 关键文件未覆盖：scopes.java、componentdescriptor.java、scope.java、bindsinstance.java、coffeeapp.java、heatermodule.java、pumpmodule.java、thermosiphon.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：scopes.java、componentdescriptor.java、scope.java、bindsinstance.java、coffeeapp.java、heatermodule.java、pumpmodule.java、thermosiphon.java

### 核心概念/核心注解详解/@Component 注解详解.md

- reference 标题：@Component 注解详解
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：108
- 页面类型：module / module
- 行数：211 / 94
- 段落行数：88 / 34
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：component.java、module.java
- reference 关键文件未覆盖：componentprocessor.java、componentannotation.java、componentgenerator.java、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、coffeeshop.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、componentannotation.java、componentgenerator.java、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、coffeeshop.java

### 核心概念/核心注解详解/@Inject 注解详解.md

- reference 标题：@Inject 注解详解
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/核心注解详解/@Module 注解详解.md

- reference 标题：@Module 注解详解
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/核心注解详解/@Provides 与 @Binds 注解详解.md

- reference 标题：@Provides 与 @Binds 注解详解
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/核心注解详解/核心注解详解.md

- reference 标题：核心注解详解
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：94
- 页面类型：other / module
- 行数：262 / 94
- 段落行数：98 / 34
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：component.java、module.java、provides.java
- reference 关键文件未覆盖：componentprocessor.java、componentprocessingstep.java、injectedfieldsignature.java、coffeeapp.java、coffeeappcomponent.java、heater.java、heatermodule.java、pump.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、componentprocessingstep.java、injectedfieldsignature.java、coffeeapp.java、coffeeappcomponent.java、heater.java、heatermodule.java、pump.java

### 核心概念/组件与模块系统/模块组织与管理.md

- reference 标题：模块组织与管理
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：146
- 页面类型：module / module
- 行数：317 / 94
- 段落行数：117 / 34
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：module.java、provides.java
- reference 关键文件未覆盖：modulevalidator.java、providesmethodvalidator.java、moduleproxies.java、coffeeapp.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：modulevalidator.java、providesmethodvalidator.java、moduleproxies.java、coffeeapp.java

### 核心概念/组件与模块系统/组件与模块系统.md

- reference 标题：组件与模块系统
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：110
- 页面类型：module / module
- 行数：331 / 94
- 段落行数：139 / 34
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：component.java、module.java
- reference 关键文件未覆盖：componentprocessor.java、bindinggraphvalidationmodule.java、subcomponentfactorymethodvalidator.java、componentpath.java、componenthierarchyvalidator.java、componentimplementation.java、subcomponent.java、accountmodule.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、bindinggraphvalidationmodule.java、subcomponentfactorymethodvalidator.java、componentpath.java、componenthierarchyvalidator.java、componentimplementation.java、subcomponent.java、accountmodule.java

### 核心概念/组件与模块系统/组件关系设计/子组件机制.md

- reference 标题：子组件机制
- 生成页：核心模块/javatests.md（模块：javatests）
- 匹配分数：128
- 页面类型：module / module
- 行数：340 / 101
- 段落行数：144 / 38
- Evidence：23 / 1
- Mermaid：10 / 1
- 文件提及重合：allcontrollersaredirectchildrenofapplication.java、componentstructurefollowscontrollerstructureapplication.java、coffeeserverwithcallscopeservice.java
- reference 关键文件未覆盖：compositebindinggraphplugin.java、dependencycyclevalidator.java、subcomponentfactorymethodvalidator.java、currentimplementationsubcomponent.java、toplevelimplementationcomponent.java、subcomponent.java、usercommandsrouter.java、simpleactivity.kt
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：compositebindinggraphplugin.java、dependencycyclevalidator.java、subcomponentfactorymethodvalidator.java、currentimplementationsubcomponent.java、toplevelimplementationcomponent.java、subcomponent.java、usercommandsrouter.java、simpleactivity.kt

### 核心概念/组件与模块系统/组件关系设计/组件依赖关系.md

- reference 标题：组件依赖关系
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：100
- 页面类型：module / module
- 行数：266 / 94
- 段落行数：112 / 34
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：component.java
- reference 关键文件未覆盖：componentrequirementrequestrepresentation.java、componentdependencybinding.java、componentdescriptor.java、componentrequirement.java、dependencycyclevalidator.java、bindingkind.java、componentvalidator.java、dependencymethodproducercreationexpression.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentrequirementrequestrepresentation.java、componentdependencybinding.java、componentdescriptor.java、componentrequirement.java、dependencycyclevalidator.java、bindingkind.java、componentvalidator.java、dependencymethodproducercreationexpression.java

### 核心概念/组件与模块系统/组件关系设计/组件关系设计.md

- reference 标题：组件关系设计
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：110
- 页面类型：module / module
- 行数：266 / 94
- 段落行数：103 / 34
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：component.java、module.java
- reference 关键文件未覆盖：componentdependencybinding.java、subcomponentcreatorbinding.java、subcomponentdeclaration.java、subcomponentfactorymethodvalidator.java、componentgenerator.java、componenthierarchyvalidator.java、subcomponentcreatorrequestrepresentation.java、productionsubcomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentdependencybinding.java、subcomponentcreatorbinding.java、subcomponentdeclaration.java、subcomponentfactorymethodvalidator.java、componentgenerator.java、componenthierarchyvalidator.java、subcomponentcreatorrequestrepresentation.java、productionsubcomponent.java

### 核心概念/组件与模块系统/组件基础概念.md

- reference 标题：组件基础概念
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：104
- 页面类型：module / module
- 行数：231 / 94
- 段落行数：90 / 34
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：component.java
- reference 关键文件未覆盖：coffeeapp.java、componentdescriptor.java、componentgenerator.java、membersinjector.java、mycomponent.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：coffeeapp.java、componentdescriptor.java、componentgenerator.java、membersinjector.java、mycomponent.java

### 核心概念/组件与模块系统/组件构建器与工厂.md

- reference 标题：组件构建器与工厂
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：124
- 页面类型：module / module
- 行数：358 / 94
- 段落行数：111 / 34
- Evidence：15 / 1
- Mermaid：5 / 1
- 文件提及重合：component.java
- reference 关键文件未覆盖：bindsinstance.java、builderbindsinstanceparametertest.java、buildertest.java、componentcreatordescriptor.java、componentcreatorkind.java、componentcreatorvalidator.java、errormessages.java、mycomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：bindsinstance.java、builderbindsinstanceparametertest.java、buildertest.java、componentcreatordescriptor.java、componentcreatorkind.java、componentcreatorvalidator.java、errormessages.java、mycomponent.java

### 核心概念/绑定与提供者模式.md

- reference 标题：绑定与提供者模式
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略与最佳实践/Android测试.md

- reference 标题：Android测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略与最佳实践/单元测试.md

- reference 标题：单元测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略与最佳实践/性能测试与监控.md

- reference 标题：性能测试与监控
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略与最佳实践/测试策略与最佳实践.md

- reference 标题：测试策略与最佳实践
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略与最佳实践/测试自动化与CI_CD.md

- reference 标题：测试自动化与CI/CD
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略与最佳实践/集成测试.md

- reference 标题：集成测试
- 生成页：无
- 问题：缺少对应生成页面

### 示例与教程/Android应用示例.md

- reference 标题：Android应用示例
- 生成页：无
- 问题：缺少对应生成页面

### 示例与教程/基础示例.md

- reference 标题：基础示例
- 生成页：无
- 问题：缺少对应生成页面

### 示例与教程/实践教程.md

- reference 标题：实践教程
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：72
- 页面类型：other / module
- 行数：350 / 94
- 段落行数：159 / 34
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：binds.java、component.java、lazy.java、module.java、provides.java、reusable.java
- reference 关键文件未覆盖：readme.md、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、electricheater.java、heater.java、heatermodule.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、electricheater.java、heater.java、heatermodule.java

### 示例与教程/示例与教程.md

- reference 标题：示例与教程
- 生成页：无
- 问题：缺少对应生成页面

### 示例与教程/高级应用示例.md

- reference 标题：高级应用示例
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：70
- 页面类型：other / module
- 行数：319 / 94
- 段落行数：146 / 34
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：binds.java、component.java、lazy.java、module.java、provides.java
- reference 关键文件未覆盖：contributing.md、readme.md、producer.java、productioncomponent.java、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、producer.java、productioncomponent.java、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java

### 编译时处理机制/代码生成策略.md

- reference 标题：代码生成策略
- 生成页：专题/module-capability/module-76f4cd583a47-plugin-buildSrc能力：扩展机制.md（buildSrc能力：扩展机制）
- 匹配分数：94
- 页面类型：topic / topic
- 行数：361 / 65
- 段落行数：159 / 44
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：sourcefilegenerator.java、componentgenerator.java、componentcreatorimplementation.java、componentimplementation.java、factorygenerator.java、generatedimplementation.java、membersinjectorgenerator.java、modulegenerator.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：sourcefilegenerator.java、componentgenerator.java、componentcreatorimplementation.java、componentimplementation.java、factorygenerator.java、generatedimplementation.java、membersinjectorgenerator.java、modulegenerator.java

### 编译时处理机制/依赖图构建.md

- reference 标题：依赖图构建
- 生成页：专题/module-capability/module-b542d0916206-model-dagger-compiler能力：领域建模.md（dagger-compiler能力：领域建模）
- 匹配分数：106
- 页面类型：topic / topic
- 行数：335 / 51
- 段落行数：164 / 30
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：binding.java、key.java
- reference 关键文件未覆盖：bindinggraphfactory.java、componentnodeimpl.java、dependencyedgeimpl.java、compositebindinggraphplugin.java、dependencycyclevalidator.java、duplicatebindingsvalidator.java、incompatiblyscopedbindingsvalidator.java、missingbindingvalidator.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bindinggraphfactory.java、componentnodeimpl.java、dependencyedgeimpl.java、compositebindinggraphplugin.java、dependencycyclevalidator.java、duplicatebindingsvalidator.java、incompatiblyscopedbindingsvalidator.java、missingbindingvalidator.java

### 编译时处理机制/增量编译优化.md

- reference 标题：增量编译优化
- 生成页：专题/module-capability/module-4157350ffac0-plugin-java能力：扩展机制.md（java能力：扩展机制）
- 匹配分数：94
- 页面类型：topic / topic
- 行数：268 / 76
- 段落行数：122 / 55
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：componentprocessor.java、delegatecomponentprocessor.java、kspcomponentprocessor.java、clearablecache.java、processingenvironmentcompileroptions.java、componentvalidator.java、injectvalidator.java
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、kspcomponentprocessor.java、clearablecache.java、processingenvironmentcompileroptions.java、componentvalidator.java、injectvalidator.java

### 编译时处理机制/注解处理基础.md

- reference 标题：注解处理基础
- 生成页：专题/module-capability/module-4157350ffac0-plugin-java能力：扩展机制.md（java能力：扩展机制）
- 匹配分数：94
- 页面类型：topic / topic
- 行数：296 / 76
- 段落行数：135 / 55
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：componentprocessor.java、delegatecomponentprocessor.java、processingenvironmentcompileroptions.java、componentprocessingstep.java、injectprocessingstep.java、mapkeyprocessingstep.java、moduleprocessingstep.java、processingstepsmodule.java
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、processingenvironmentcompileroptions.java、componentprocessingstep.java、injectprocessingstep.java、mapkeyprocessingstep.java、moduleprocessingstep.java、processingstepsmodule.java

### 编译时处理机制/编译时处理机制.md

- reference 标题：编译时处理机制
- 生成页：专题/module-capability/module-4157350ffac0-plugin-java能力：扩展机制.md（java能力：扩展机制）
- 匹配分数：92
- 页面类型：topic / topic
- 行数：327 / 76
- 段落行数：135 / 55
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：componentprocessor.java、delegatecomponentprocessor.java、kspcomponentprocessor.java、processingenvironmentmodule.java、validationreport.java、bindinggraphfactory.java、bindinggraphvalidationmodule.java、compositebindinggraphplugin.java
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、kspcomponentprocessor.java、processingenvironmentmodule.java、validationreport.java、bindinggraphfactory.java、bindinggraphvalidationmodule.java、compositebindinggraphplugin.java

### 编译时处理机制/验证与错误处理.md

- reference 标题：验证与错误处理
- 生成页：专题/module-capability/module-76f4cd583a47-plugin-buildSrc能力：扩展机制.md（buildSrc能力：扩展机制）
- 匹配分数：92
- 页面类型：topic / topic
- 行数：399 / 65
- 段落行数：180 / 44
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：validationreport.java、errormessages.java、bindinggraphplugin.java、diagnosticreporter.java、bindinggraphvalidator.java、componentvalidator.java、diagnosticreporterfactory.java、modulevalidator.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：validationreport.java、errormessages.java、bindinggraphplugin.java、diagnosticreporter.java、bindinggraphvalidator.java、componentvalidator.java、diagnosticreporterfactory.java、modulevalidator.java

### 贡献指南.md

- reference 标题：贡献指南
- 生成页：无
- 问题：缺少对应生成页面

### 项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：596
- 页面类型：overview / overview
- 行数：244 / 77
- 段落行数：114 / 8
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：contributing.md、readme.md、componentprocessor.java、delegatecomponentprocessor.java、component.java、lazy.java、module.java、subcomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、componentprocessor.java、delegatecomponentprocessor.java、component.java、lazy.java、module.java、subcomponent.java

### 高级特性与扩展/SPI扩展机制.md

- reference 标题：SPI扩展机制
- 生成页：专题/module-capability/module-4157350ffac0-plugin-java能力：扩展机制.md（java能力：扩展机制）
- 匹配分数：94
- 页面类型：topic / topic
- 行数：393 / 76
- 段落行数：185 / 55
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：serviceloaders.java、compositebindinggraphplugin.java、diagnosticreporterfactory.java、externalbindinggraphplugins.java、validationbindinggraphplugin.java、validationbindinggraphplugins.java、bindinggraphplugin.java、diagnosticreporter.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：serviceloaders.java、compositebindinggraphplugin.java、diagnosticreporterfactory.java、externalbindinggraphplugins.java、validationbindinggraphplugin.java、validationbindinggraphplugins.java、bindinggraphplugin.java、diagnosticreporter.java

### 高级特性与扩展/多值绑定高级用法.md

- reference 标题：多值绑定高级用法
- 生成页：无
- 问题：缺少对应生成页面

### 高级特性与扩展/延迟初始化与线程安全.md

- reference 标题：延迟初始化与线程安全
- 生成页：无
- 问题：缺少对应生成页面

### 高级特性与扩展/性能监控与调试.md

- reference 标题：性能监控与调试
- 生成页：无
- 问题：缺少对应生成页面

### 高级特性与扩展/自定义扩展开发.md

- reference 标题：自定义扩展开发
- 生成页：无
- 问题：缺少对应生成页面

### 高级特性与扩展/高级特性与扩展.md

- reference 标题：高级特性与扩展
- 生成页：无
- 问题：缺少对应生成页面

