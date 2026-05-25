# dagger Reference 对比报告

生成页面：71 页
reference 页面：65 页
命中对比：34 页
缺失对比：31 页
运行模式：warm (cache_mode=preserve)
LLM usage：requests=16, total_tokens=56936, page_research=0, page_enrichment=0

## 覆盖统计

- 专题页覆盖：generated 0 / reference 7（repo-archetype=0）
- evidence 落页：generated 0 / reference 65
- citation 密度：generated 0 / reference 79.69
- 图表达覆盖：generated 0 / reference 65
- page research 请求：0
- page enrichment 请求：0
- 已规划专题类型：无
- 高频缺失专题：无

## 项目结论

- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 专题页覆盖仍明显不足，很多 reference 主题还没有被 planner 单独承载
- 生成页普遍缺少 reference 那种源码引用/出处层
- evidence block 已进入页面，但覆盖率和密度仍低于 reference
- Mermaid/结构图表达仍然不足
- facts-driven 图输入尚未稳定覆盖到代表性页面
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 核心运行时/dagger-runtime.md 被 4 个 reference 页面共享映射
- 核心模块/dagger.md 被 6 个 reference 页面共享映射

## 额外生成页面

- API-参考/API-参考.md (API 参考, 14 行)
- API-参考/dagger.md (API：dagger, 23 行)
- API-参考/java.md (API：java, 23 行)
- 平台绑定-Android/dagger-android-processor.md (dagger-android-processor, 32 行)
- 平台绑定-Android/dagger-android-proguard-processor.md (dagger-android-proguard-processor, 31 行)
- 平台绑定-Android/dagger-android-support.md (dagger-android-support, 31 行)
- 平台绑定-Android/dagger-android.md (dagger-android, 32 行)
- 平台绑定-Android/dagger-lint-android.md (dagger-lint-android, 31 行)
- 平台绑定-Android/hilt-android-testing.md (hilt-android-testing, 32 行)
- 平台绑定-Android/hilt-android.md (hilt-android, 32 行)
- 平台绑定-Android/平台绑定-Android.md (平台绑定：Android, 19 行)
- 插件生态/插件生态.md (插件生态, 9 行)
- 构建系统/buildSrc.md (buildSrc, 29 行)
- 构建系统/构建系统.md (构建系统, 13 行)
- 核心模块/dagger-grpc-server.md (dagger-grpc-server, 32 行)
- 核心模块/dagger-kythe.md (dagger-kythe, 31 行)
- 核心模块/dagger-lint.md (dagger-lint, 30 行)
- 核心模块/dagger-testing.md (dagger-testing, 29 行)
- 核心模块/gradle.md (gradle, 31 行)
- 核心模块/java.md (java, 32 行)
- 核心模块/util.md (util, 32 行)
- 核心运行时/hilt-core.md (hilt-core, 32 行)
- 核心运行时/核心运行时.md (核心运行时, 14 行)
- 框架集成-Hilt/hilt-android-testing.md (hilt-android-testing, 32 行)
- 框架集成-Hilt/hilt-android.md (hilt-android, 32 行)
- 框架集成-Hilt/hilt-compiler.md (hilt-compiler, 32 行)
- 框架集成-Hilt/hilt-core.md (hilt-core, 32 行)
- 框架集成-Hilt/hilt-testing.md (hilt-testing, 31 行)
- 框架集成-Hilt/框架集成-Hilt.md (框架集成：Hilt, 17 行)
- 概念指南/概念指南.md (概念指南, 32 行)
- 测试基础设施/dagger-testing.md (测试：dagger-testing, 23 行)
- 测试基础设施/hilt-android-testing.md (测试：hilt-android-testing, 20 行)
- 测试基础设施/hilt-testing.md (测试：hilt-testing, 21 行)
- 测试基础设施/javatests.md (测试：javatests, 20 行)
- 测试基础设施/测试基础设施.md (测试基础设施, 16 行)
- 系统架构.md (系统架构, 6 行)
- 编译工具链/dagger-android-processor.md (dagger-android-processor, 32 行)
- 编译工具链/dagger-android-proguard-processor.md (dagger-android-proguard-processor, 31 行)
- 编译工具链/dagger-compiler.md (dagger-compiler, 32 行)
- 编译工具链/dagger-grpc-server-annotations.md (dagger-grpc-server-annotations, 31 行)
- 编译工具链/dagger-grpc-server-processor.md (dagger-grpc-server-processor, 32 行)
- 编译工具链/dagger-spi.md (dagger-spi, 32 行)
- 编译工具链/hilt-compiler.md (hilt-compiler, 32 行)
- 编译工具链/编译工具链.md (编译工具链, 19 行)
- 配置参考/配置参考.md (配置参考, 13 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API参考文档/API参考文档.md | 概念指南/API参考文档.md | 594/9 | 239/2 | 0/0 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidinjector.java、hasandroidinjector.java、cancellationpolicy.java、producer.java、producermodule.java、produces.java、production.java、productioncomponent.java |
| API参考文档/Android API.md | 概念指南/Android-API.md | 398/9 | 168/2 | 21/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、androidinjectordescriptor.java、contributesandroidinjectorprocessingstep.java、androidsupportinjection.java、androidsupportinjectionmodule.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java |
| API参考文档/Hilt API.md | 概念指南/Hilt-API.md | 356/9 | 120/2 | 0/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java |
| API参考文档/异步处理API.md | 概念指南/异步处理API.md | 367/9 | 163/2 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：provisiondependencyonproducerbindingvalidator.java、dependencymethodproducercreationexpression.java、producercreationexpression.java、cancellationpolicy.java、produced.java、producer.java、producermodule.java、produces.java |
| API参考文档/编译时API.md | 概念指南/编译时API.md | 407/9 | 164/2 | 19/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、componentdescriptor.java、binding.java、bindinggraph.java、componentpath.java、dependencyrequest.java、key.java、requestkind.java |
| API参考文档/运行时API.md | 概念指南/运行时API.md | 388/9 | 102/2 | 0/0 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：binds.java、bindsinstance.java、component.java、lazy.java、mapkey.java、membersinjector.java、module.java、provides.java |
| Android集成开发/@ContributesAndroidInjector注解.md | 概念指南/ContributesAndroidInjector注解.md | 324/9 | 122/2 | 16/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：androidinjectordescriptor.java、androidprocessor.java、baseprocessingstep.java、contributesandroidinjectorprocessingstep.java、delegateandroidprocessor.java、xtypenames.java、androidinjector.java、contributesandroidinjector.java |
| Android集成开发/Activity注入.md | 概念指南/Activity注入.md | 302/9 | 117/2 | 17/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggeractivity.java、dispatchingandroidinjector.java |
| Android集成开发/Android生命周期管理.md | 概念指南/Android生命周期管理.md | 383/9 | 153/2 | 21/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggeractivity.java、daggerapplication.java |
| Android集成开发/Android集成开发.md | 概念指南/Android集成开发.md | 321/9 | 158/2 | 0/0 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、daggerfragment.java、androidinjection.java、androidinjectionkey.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java |
| Android集成开发/Fragment注入.md | 概念指南/Fragment注入.md | 318/9 | 128/2 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidsupportinjection.java、daggerfragment.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java |
| Android集成开发/Service注入.md | 概念指南/Service注入.md | 275/9 | 103/2 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidinjection.java、androidinjector.java、contributesandroidinjector.java、daggerservice.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java |
| Android集成开发/其他组件注入.md | 概念指南/其他组件注入.md | 323/9 | 150/2 | 0/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggerbroadcastreceiver.java、daggercontentprovider.java、daggerintentservice.java、dispatchingandroidinjector.java |
| Hilt框架详解/@HiltAndroidApp与注解使用.md | 概念指南/HiltAndroidApp与注解使用.md | 315/9 | 90/2 | 0/0 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：coffeeapp.java、androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java |
| Hilt框架详解/Hilt核心概念与架构.md | 概念指南/Hilt核心概念与架构.md | 295/9 | 127/2 | 17/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、readme.md、singletoncomponent.java、definecomponentnoparent.java |
| Hilt框架详解/Hilt框架详解.md | 概念指南/Hilt框架详解.md | 328/9 | 134/2 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、singletoncomponent.java |
| Hilt框架详解/模块安装与依赖管理.md | 概念指南/模块安装与依赖管理.md | 289/9 | 124/2 | 18/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：duplicatebindingsvalidator.java、moduleprocessingstep.java、modulevalidator.java、activitycomponent.java、definecomponent.java、installin.java、singletoncomponent.java、definecomponentnoparent.java |
| Hilt框架详解/测试支持与模拟.md | 概念指南/测试支持与模拟.md | 306/9 | 141/2 | 19/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：coffeeappfakeheatertest.java、earlysingletoncomponentcreator.java、markthatrulesranrule.java、testapplicationcomponentmanager.java、testcomponentdatasupplier.java、bindvalue.java、customtestapplication.java、hiltandroidrule.java |
| Hilt框架详解/组件树结构与生命周期.md | 概念指南/组件树结构与生命周期.md | 284/9 | 124/2 | 13/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：definecomponent.java、entrypoint.java、installin.java、singletoncomponent.java |
| Hilt框架详解/编译时处理与代码生成.md | 概念指南/编译时处理与代码生成.md | 296/9 | 134/2 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidentrypointprocessingstep.java、androidentrypointprocessor.java、bindvalueprocessingstep.java、bindvalueprocessor.java、baseprocessingstep.java、javacbaseprocessingstepprocessor.java、aggregateddepsprocessingstep.java、aggregateddepsprocessor.java |
| 异步处理与生产者.md | 核心模块/dagger-producers.md | 296/32 | 131/5 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cancellationpolicy.java、productioncomponent.java、productionsubcomponent.java、abstractproducer.java、dependencymethodproducer.java、producermonitor.java、productioncomponentmonitor.java、producercancellationtest.java |
| 快速开始.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 故障排除与调试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 构建与部署.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/作用域与生命周期管理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/依赖注入基础理论.md | 核心运行时/dagger-runtime.md | 323/32 | 138/5 | 19/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：membersinjector.java、doublecheck.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java |
| 核心概念/核心概念.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/核心注解详解/@Component 注解详解.md | 核心模块/dagger.md | 211/32 | 88/5 | 0/0 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、componentannotation.java、componentgenerator.java、component.java、module.java、subcomponent.java、coffeeapp.java、coffeelogger.java |
| 核心概念/核心注解详解/@Inject 注解详解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/核心注解详解/@Module 注解详解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/核心注解详解/@Provides 与 @Binds 注解详解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/核心注解详解/核心注解详解.md | 核心运行时/dagger-runtime.md | 262/32 | 98/5 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、componentprocessingstep.java、injectedfieldsignature.java、coffeeapp.java、coffeeappcomponent.java、heater.java、heatermodule.java、pump.java |
| 核心概念/组件与模块系统/模块组织与管理.md | 核心模块/dagger.md | 317/32 | 117/5 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：modulevalidator.java、providesmethodvalidator.java、moduleproxies.java、module.java、provides.java、coffeeapp.java |
| 核心概念/组件与模块系统/组件与模块系统.md | 核心模块/dagger.md | 331/32 | 139/5 | 0/0 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、bindinggraphvalidationmodule.java、subcomponentfactorymethodvalidator.java、componentpath.java、componenthierarchyvalidator.java、componentimplementation.java、component.java、module.java |
| 核心概念/组件与模块系统/组件关系设计/子组件机制.md | 核心模块/javatests.md | 340/32 | 144/5 | 23/0 | 10/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：compositebindinggraphplugin.java、dependencycyclevalidator.java、subcomponentfactorymethodvalidator.java、currentimplementationsubcomponent.java、toplevelimplementationcomponent.java、subcomponent.java、usercommandsrouter.java、simpleactivity.kt |
| 核心概念/组件与模块系统/组件关系设计/组件依赖关系.md | 核心模块/dagger.md | 266/32 | 112/5 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentrequirementrequestrepresentation.java、componentdependencybinding.java、componentdescriptor.java、componentrequirement.java、dependencycyclevalidator.java、bindingkind.java、componentvalidator.java、dependencymethodproducercreationexpression.java |
| 核心概念/组件与模块系统/组件关系设计/组件关系设计.md | 核心模块/dagger.md | 266/32 | 103/5 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentdependencybinding.java、subcomponentcreatorbinding.java、subcomponentdeclaration.java、subcomponentfactorymethodvalidator.java、componentgenerator.java、componenthierarchyvalidator.java、subcomponentcreatorrequestrepresentation.java、productionsubcomponent.java |
| 核心概念/组件与模块系统/组件基础概念.md | 核心模块/dagger.md | 231/32 | 90/5 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：coffeeapp.java、component.java、componentdescriptor.java、componentgenerator.java、membersinjector.java、mycomponent.java |
| 核心概念/组件与模块系统/组件构建器与工厂.md | 核心模块/核心模块.md | 358/22 | 111/2 | 15/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：bindsinstance.java、builderbindsinstanceparametertest.java、buildertest.java、component.java、componentcreatordescriptor.java、componentcreatorkind.java、componentcreatorvalidator.java、errormessages.java |
| 核心概念/绑定与提供者模式.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/Android测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/单元测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/性能测试与监控.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/测试策略与最佳实践.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/测试自动化与CI_CD.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/集成测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例与教程/Android应用示例.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例与教程/基础示例.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例与教程/实践教程.md | 核心运行时/dagger-runtime.md | 350/32 | 159/5 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、electricheater.java、heater.java、heatermodule.java |
| 示例与教程/示例与教程.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例与教程/高级应用示例.md | 核心运行时/dagger-runtime.md | 319/32 | 146/5 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、producer.java、productioncomponent.java、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java |
| 编译时处理机制/代码生成策略.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 编译时处理机制/依赖图构建.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 编译时处理机制/增量编译优化.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 编译时处理机制/注解处理基础.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 编译时处理机制/编译时处理机制.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 编译时处理机制/验证与错误处理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 贡献指南.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 项目概述.md | 项目概述.md | 244/27 | 114/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、componentprocessor.java、delegatecomponentprocessor.java、component.java、lazy.java、module.java、subcomponent.java |
| 高级特性与扩展/SPI扩展机制.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级特性与扩展/多值绑定高级用法.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级特性与扩展/延迟初始化与线程安全.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级特性与扩展/性能监控与调试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级特性与扩展/自定义扩展开发.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级特性与扩展/高级特性与扩展.md | 缺失 | - | - | - | - | 缺少对应生成页面 |

## 逐文件详情

### API参考文档/API参考文档.md

- reference 标题：API参考文档
- 生成页：概念指南/API参考文档.md（API参考文档）
- 匹配分数：242
- 页面类型：other / other
- 行数：594 / 9
- 段落行数：239 / 2
- Evidence：0 / 0
- Mermaid：9 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：androidinjector.java、hasandroidinjector.java、cancellationpolicy.java、producer.java、producermodule.java、produces.java、production.java、productioncomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidinjector.java、hasandroidinjector.java、cancellationpolicy.java、producer.java、producermodule.java、produces.java、production.java、productioncomponent.java

### API参考文档/Android API.md

- reference 标题：Android API
- 生成页：概念指南/Android-API.md（Android API）
- 匹配分数：244
- 页面类型：other / other
- 行数：398 / 9
- 段落行数：168 / 2
- Evidence：21 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、androidinjectordescriptor.java、contributesandroidinjectorprocessingstep.java、androidsupportinjection.java、androidsupportinjectionmodule.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、androidinjectordescriptor.java、contributesandroidinjectorprocessingstep.java、androidsupportinjection.java、androidsupportinjectionmodule.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java

### API参考文档/Hilt API.md

- reference 标题：Hilt API
- 生成页：概念指南/Hilt-API.md（Hilt API）
- 匹配分数：244
- 页面类型：other / other
- 行数：356 / 9
- 段落行数：120 / 2
- Evidence：0 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java

### API参考文档/异步处理API.md

- reference 标题：异步处理API
- 生成页：概念指南/异步处理API.md（异步处理API）
- 匹配分数：242
- 页面类型：other / other
- 行数：367 / 9
- 段落行数：163 / 2
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：provisiondependencyonproducerbindingvalidator.java、dependencymethodproducercreationexpression.java、producercreationexpression.java、cancellationpolicy.java、produced.java、producer.java、producermodule.java、produces.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：provisiondependencyonproducerbindingvalidator.java、dependencymethodproducercreationexpression.java、producercreationexpression.java、cancellationpolicy.java、produced.java、producer.java、producermodule.java、produces.java

### API参考文档/编译时API.md

- reference 标题：编译时API
- 生成页：概念指南/编译时API.md（编译时API）
- 匹配分数：242
- 页面类型：other / other
- 行数：407 / 9
- 段落行数：164 / 2
- Evidence：19 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：componentprocessor.java、componentdescriptor.java、binding.java、bindinggraph.java、componentpath.java、dependencyrequest.java、key.java、requestkind.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、componentdescriptor.java、binding.java、bindinggraph.java、componentpath.java、dependencyrequest.java、key.java、requestkind.java

### API参考文档/运行时API.md

- reference 标题：运行时API
- 生成页：概念指南/运行时API.md（运行时API）
- 匹配分数：240
- 页面类型：other / other
- 行数：388 / 9
- 段落行数：102 / 2
- Evidence：0 / 0
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：binds.java、bindsinstance.java、component.java、lazy.java、mapkey.java、membersinjector.java、module.java、provides.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：binds.java、bindsinstance.java、component.java、lazy.java、mapkey.java、membersinjector.java、module.java、provides.java

### Android集成开发/@ContributesAndroidInjector注解.md

- reference 标题：@ContributesAndroidInjector注解
- 生成页：概念指南/ContributesAndroidInjector注解.md（@ContributesAndroidInjector注解）
- 匹配分数：242
- 页面类型：other / other
- 行数：324 / 9
- 段落行数：122 / 2
- Evidence：16 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：androidinjectordescriptor.java、androidprocessor.java、baseprocessingstep.java、contributesandroidinjectorprocessingstep.java、delegateandroidprocessor.java、xtypenames.java、androidinjector.java、contributesandroidinjector.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：androidinjectordescriptor.java、androidprocessor.java、baseprocessingstep.java、contributesandroidinjectorprocessingstep.java、delegateandroidprocessor.java、xtypenames.java、androidinjector.java、contributesandroidinjector.java

### Android集成开发/Activity注入.md

- reference 标题：Activity注入
- 生成页：概念指南/Activity注入.md（Activity注入）
- 匹配分数：242
- 页面类型：other / other
- 行数：302 / 9
- 段落行数：117 / 2
- Evidence：17 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggeractivity.java、dispatchingandroidinjector.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggeractivity.java、dispatchingandroidinjector.java

### Android集成开发/Android生命周期管理.md

- reference 标题：Android生命周期管理
- 生成页：概念指南/Android生命周期管理.md（Android生命周期管理）
- 匹配分数：242
- 页面类型：other / other
- 行数：383 / 9
- 段落行数：153 / 2
- Evidence：21 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggeractivity.java、daggerapplication.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggeractivity.java、daggerapplication.java

### Android集成开发/Android集成开发.md

- reference 标题：Android集成开发
- 生成页：概念指南/Android集成开发.md（Android集成开发）
- 匹配分数：242
- 页面类型：other / other
- 行数：321 / 9
- 段落行数：158 / 2
- Evidence：0 / 0
- Mermaid：9 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、daggerfragment.java、androidinjection.java、androidinjectionkey.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、daggerfragment.java、androidinjection.java、androidinjectionkey.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java

### Android集成开发/Fragment注入.md

- reference 标题：Fragment注入
- 生成页：概念指南/Fragment注入.md（Fragment注入）
- 匹配分数：240
- 页面类型：other / other
- 行数：318 / 9
- 段落行数：128 / 2
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：androidsupportinjection.java、daggerfragment.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidsupportinjection.java、daggerfragment.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java

### Android集成开发/Service注入.md

- reference 标题：Service注入
- 生成页：概念指南/Service注入.md（Service注入）
- 匹配分数：242
- 页面类型：other / other
- 行数：275 / 9
- 段落行数：103 / 2
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：androidinjection.java、androidinjector.java、contributesandroidinjector.java、daggerservice.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidinjection.java、androidinjector.java、contributesandroidinjector.java、daggerservice.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java

### Android集成开发/其他组件注入.md

- reference 标题：其他组件注入
- 生成页：概念指南/其他组件注入.md（其他组件注入）
- 匹配分数：240
- 页面类型：other / other
- 行数：323 / 9
- 段落行数：150 / 2
- Evidence：0 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggerbroadcastreceiver.java、daggercontentprovider.java、daggerintentservice.java、dispatchingandroidinjector.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggerbroadcastreceiver.java、daggercontentprovider.java、daggerintentservice.java、dispatchingandroidinjector.java

### Hilt框架详解/@HiltAndroidApp与注解使用.md

- reference 标题：@HiltAndroidApp与注解使用
- 生成页：概念指南/HiltAndroidApp与注解使用.md（@HiltAndroidApp与注解使用）
- 匹配分数：242
- 页面类型：other / other
- 行数：315 / 9
- 段落行数：90 / 2
- Evidence：0 / 0
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：coffeeapp.java、androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：coffeeapp.java、androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java

### Hilt框架详解/Hilt核心概念与架构.md

- reference 标题：Hilt核心概念与架构
- 生成页：概念指南/Hilt核心概念与架构.md（Hilt核心概念与架构）
- 匹配分数：322
- 页面类型：architecture / architecture
- 行数：295 / 9
- 段落行数：127 / 2
- Evidence：17 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、readme.md、singletoncomponent.java、definecomponentnoparent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、readme.md、singletoncomponent.java、definecomponentnoparent.java

### Hilt框架详解/Hilt框架详解.md

- reference 标题：Hilt框架详解
- 生成页：概念指南/Hilt框架详解.md（Hilt框架详解）
- 匹配分数：242
- 页面类型：other / other
- 行数：328 / 9
- 段落行数：134 / 2
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、singletoncomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、singletoncomponent.java

### Hilt框架详解/模块安装与依赖管理.md

- reference 标题：模块安装与依赖管理
- 生成页：概念指南/模块安装与依赖管理.md（模块安装与依赖管理）
- 匹配分数：320
- 页面类型：module / module
- 行数：289 / 9
- 段落行数：124 / 2
- Evidence：18 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：duplicatebindingsvalidator.java、moduleprocessingstep.java、modulevalidator.java、activitycomponent.java、definecomponent.java、installin.java、singletoncomponent.java、definecomponentnoparent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：duplicatebindingsvalidator.java、moduleprocessingstep.java、modulevalidator.java、activitycomponent.java、definecomponent.java、installin.java、singletoncomponent.java、definecomponentnoparent.java

### Hilt框架详解/测试支持与模拟.md

- reference 标题：测试支持与模拟
- 生成页：概念指南/测试支持与模拟.md（测试支持与模拟）
- 匹配分数：240
- 页面类型：other / other
- 行数：306 / 9
- 段落行数：141 / 2
- Evidence：19 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：coffeeappfakeheatertest.java、earlysingletoncomponentcreator.java、markthatrulesranrule.java、testapplicationcomponentmanager.java、testcomponentdatasupplier.java、bindvalue.java、customtestapplication.java、hiltandroidrule.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：coffeeappfakeheatertest.java、earlysingletoncomponentcreator.java、markthatrulesranrule.java、testapplicationcomponentmanager.java、testcomponentdatasupplier.java、bindvalue.java、customtestapplication.java、hiltandroidrule.java

### Hilt框架详解/组件树结构与生命周期.md

- reference 标题：组件树结构与生命周期
- 生成页：概念指南/组件树结构与生命周期.md（组件树结构与生命周期）
- 匹配分数：240
- 页面类型：other / other
- 行数：284 / 9
- 段落行数：124 / 2
- Evidence：13 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：definecomponent.java、entrypoint.java、installin.java、singletoncomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：definecomponent.java、entrypoint.java、installin.java、singletoncomponent.java

### Hilt框架详解/编译时处理与代码生成.md

- reference 标题：编译时处理与代码生成
- 生成页：概念指南/编译时处理与代码生成.md（编译时处理与代码生成）
- 匹配分数：240
- 页面类型：other / other
- 行数：296 / 9
- 段落行数：134 / 2
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：androidentrypointprocessingstep.java、androidentrypointprocessor.java、bindvalueprocessingstep.java、bindvalueprocessor.java、baseprocessingstep.java、javacbaseprocessingstepprocessor.java、aggregateddepsprocessingstep.java、aggregateddepsprocessor.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidentrypointprocessingstep.java、androidentrypointprocessor.java、bindvalueprocessingstep.java、bindvalueprocessor.java、baseprocessingstep.java、javacbaseprocessingstepprocessor.java、aggregateddepsprocessingstep.java、aggregateddepsprocessor.java

### 异步处理与生产者.md

- reference 标题：异步处理与生产者
- 生成页：核心模块/dagger-producers.md（dagger-producers）
- 匹配分数：60
- 页面类型：other / module
- 行数：296 / 32
- 段落行数：131 / 5
- Evidence：0 / 0
- Mermaid：7 / 0
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
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/作用域与生命周期管理.md

- reference 标题：作用域与生命周期管理
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/依赖注入基础理论.md

- reference 标题：依赖注入基础理论
- 生成页：核心运行时/dagger-runtime.md（dagger-runtime）
- 匹配分数：146
- 页面类型：other / other
- 行数：323 / 32
- 段落行数：138 / 5
- Evidence：19 / 0
- Mermaid：7 / 0
- 文件提及重合：binds.java、component.java、lazy.java、module.java、provides.java
- reference 关键文件未覆盖：membersinjector.java、doublecheck.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：membersinjector.java、doublecheck.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/核心注解详解/@Component 注解详解.md

- reference 标题：@Component 注解详解
- 生成页：核心模块/dagger.md（dagger）
- 匹配分数：86
- 页面类型：module / module
- 行数：211 / 32
- 段落行数：88 / 5
- Evidence：0 / 0
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：componentprocessor.java、componentannotation.java、componentgenerator.java、component.java、module.java、subcomponent.java、coffeeapp.java、coffeelogger.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、componentannotation.java、componentgenerator.java、component.java、module.java、subcomponent.java、coffeeapp.java、coffeelogger.java

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
- 生成页：核心运行时/dagger-runtime.md（dagger-runtime）
- 匹配分数：90
- 页面类型：other / other
- 行数：262 / 32
- 段落行数：98 / 5
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：component.java、module.java、provides.java
- reference 关键文件未覆盖：componentprocessor.java、componentprocessingstep.java、injectedfieldsignature.java、coffeeapp.java、coffeeappcomponent.java、heater.java、heatermodule.java、pump.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、componentprocessingstep.java、injectedfieldsignature.java、coffeeapp.java、coffeeappcomponent.java、heater.java、heatermodule.java、pump.java

### 核心概念/组件与模块系统/模块组织与管理.md

- reference 标题：模块组织与管理
- 生成页：核心模块/dagger.md（dagger）
- 匹配分数：88
- 页面类型：module / module
- 行数：317 / 32
- 段落行数：117 / 5
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：modulevalidator.java、providesmethodvalidator.java、moduleproxies.java、module.java、provides.java、coffeeapp.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：modulevalidator.java、providesmethodvalidator.java、moduleproxies.java、module.java、provides.java、coffeeapp.java

### 核心概念/组件与模块系统/组件与模块系统.md

- reference 标题：组件与模块系统
- 生成页：核心模块/dagger.md（dagger）
- 匹配分数：88
- 页面类型：module / module
- 行数：331 / 32
- 段落行数：139 / 5
- Evidence：0 / 0
- Mermaid：9 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：componentprocessor.java、bindinggraphvalidationmodule.java、subcomponentfactorymethodvalidator.java、componentpath.java、componenthierarchyvalidator.java、componentimplementation.java、component.java、module.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、bindinggraphvalidationmodule.java、subcomponentfactorymethodvalidator.java、componentpath.java、componenthierarchyvalidator.java、componentimplementation.java、component.java、module.java

### 核心概念/组件与模块系统/组件关系设计/子组件机制.md

- reference 标题：子组件机制
- 生成页：核心模块/javatests.md（javatests）
- 匹配分数：118
- 页面类型：module / module
- 行数：340 / 32
- 段落行数：144 / 5
- Evidence：23 / 0
- Mermaid：10 / 0
- 文件提及重合：allcontrollersaredirectchildrenofapplication.java、componentstructurefollowscontrollerstructureapplication.java、coffeeserverwithcallscopeservice.java
- reference 关键文件未覆盖：compositebindinggraphplugin.java、dependencycyclevalidator.java、subcomponentfactorymethodvalidator.java、currentimplementationsubcomponent.java、toplevelimplementationcomponent.java、subcomponent.java、usercommandsrouter.java、simpleactivity.kt
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：compositebindinggraphplugin.java、dependencycyclevalidator.java、subcomponentfactorymethodvalidator.java、currentimplementationsubcomponent.java、toplevelimplementationcomponent.java、subcomponent.java、usercommandsrouter.java、simpleactivity.kt

### 核心概念/组件与模块系统/组件关系设计/组件依赖关系.md

- reference 标题：组件依赖关系
- 生成页：核心模块/dagger.md（dagger）
- 匹配分数：88
- 页面类型：module / module
- 行数：266 / 32
- 段落行数：112 / 5
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：componentrequirementrequestrepresentation.java、componentdependencybinding.java、componentdescriptor.java、componentrequirement.java、dependencycyclevalidator.java、bindingkind.java、componentvalidator.java、dependencymethodproducercreationexpression.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentrequirementrequestrepresentation.java、componentdependencybinding.java、componentdescriptor.java、componentrequirement.java、dependencycyclevalidator.java、bindingkind.java、componentvalidator.java、dependencymethodproducercreationexpression.java

### 核心概念/组件与模块系统/组件关系设计/组件关系设计.md

- reference 标题：组件关系设计
- 生成页：核心模块/dagger.md（dagger）
- 匹配分数：88
- 页面类型：module / module
- 行数：266 / 32
- 段落行数：103 / 5
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：componentdependencybinding.java、subcomponentcreatorbinding.java、subcomponentdeclaration.java、subcomponentfactorymethodvalidator.java、componentgenerator.java、componenthierarchyvalidator.java、subcomponentcreatorrequestrepresentation.java、productionsubcomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentdependencybinding.java、subcomponentcreatorbinding.java、subcomponentdeclaration.java、subcomponentfactorymethodvalidator.java、componentgenerator.java、componenthierarchyvalidator.java、subcomponentcreatorrequestrepresentation.java、productionsubcomponent.java

### 核心概念/组件与模块系统/组件基础概念.md

- reference 标题：组件基础概念
- 生成页：核心模块/dagger.md（dagger）
- 匹配分数：90
- 页面类型：module / module
- 行数：231 / 32
- 段落行数：90 / 5
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：coffeeapp.java、component.java、componentdescriptor.java、componentgenerator.java、membersinjector.java、mycomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：coffeeapp.java、component.java、componentdescriptor.java、componentgenerator.java、membersinjector.java、mycomponent.java

### 核心概念/组件与模块系统/组件构建器与工厂.md

- reference 标题：组件构建器与工厂
- 生成页：核心模块/核心模块.md（核心模块）
- 匹配分数：90
- 页面类型：module / module
- 行数：358 / 22
- 段落行数：111 / 2
- Evidence：15 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：bindsinstance.java、builderbindsinstanceparametertest.java、buildertest.java、component.java、componentcreatordescriptor.java、componentcreatorkind.java、componentcreatorvalidator.java、errormessages.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：bindsinstance.java、builderbindsinstanceparametertest.java、buildertest.java、component.java、componentcreatordescriptor.java、componentcreatorkind.java、componentcreatorvalidator.java、errormessages.java

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
- 生成页：核心运行时/dagger-runtime.md（dagger-runtime）
- 匹配分数：66
- 页面类型：other / other
- 行数：350 / 32
- 段落行数：159 / 5
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：binds.java、component.java、lazy.java、module.java、provides.java、reusable.java
- reference 关键文件未覆盖：readme.md、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、electricheater.java、heater.java、heatermodule.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、electricheater.java、heater.java、heatermodule.java

### 示例与教程/示例与教程.md

- reference 标题：示例与教程
- 生成页：无
- 问题：缺少对应生成页面

### 示例与教程/高级应用示例.md

- reference 标题：高级应用示例
- 生成页：核心运行时/dagger-runtime.md（dagger-runtime）
- 匹配分数：60
- 页面类型：other / other
- 行数：319 / 32
- 段落行数：146 / 5
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：binds.java、component.java、lazy.java、module.java、provides.java
- reference 关键文件未覆盖：contributing.md、readme.md、producer.java、productioncomponent.java、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、producer.java、productioncomponent.java、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java

### 编译时处理机制/代码生成策略.md

- reference 标题：代码生成策略
- 生成页：无
- 问题：缺少对应生成页面

### 编译时处理机制/依赖图构建.md

- reference 标题：依赖图构建
- 生成页：无
- 问题：缺少对应生成页面

### 编译时处理机制/增量编译优化.md

- reference 标题：增量编译优化
- 生成页：无
- 问题：缺少对应生成页面

### 编译时处理机制/注解处理基础.md

- reference 标题：注解处理基础
- 生成页：无
- 问题：缺少对应生成页面

### 编译时处理机制/编译时处理机制.md

- reference 标题：编译时处理机制
- 生成页：无
- 问题：缺少对应生成页面

### 编译时处理机制/验证与错误处理.md

- reference 标题：验证与错误处理
- 生成页：无
- 问题：缺少对应生成页面

### 贡献指南.md

- reference 标题：贡献指南
- 生成页：无
- 问题：缺少对应生成页面

### 项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：566
- 页面类型：overview / overview
- 行数：244 / 27
- 段落行数：114 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：contributing.md、readme.md、componentprocessor.java、delegatecomponentprocessor.java、component.java、lazy.java、module.java、subcomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、componentprocessor.java、delegatecomponentprocessor.java、component.java、lazy.java、module.java、subcomponent.java

### 高级特性与扩展/SPI扩展机制.md

- reference 标题：SPI扩展机制
- 生成页：无
- 问题：缺少对应生成页面

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

