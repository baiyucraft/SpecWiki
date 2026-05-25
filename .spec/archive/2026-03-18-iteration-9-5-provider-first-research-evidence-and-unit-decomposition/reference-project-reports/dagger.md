# dagger Reference 对比报告

生成页面：107 页
reference 页面：65 页
命中对比：65 页
缺失对比：0 页
总体对齐率：100%
运行模式：warm (cache_mode=preserve)
LLM usage：requests=0, total_tokens=0, page_research=0, page_enrichment=0

## 95% 验收口径

- overall_match_rate：65/65 = 100%
- missing pages：0
- collapsed pages：0
- low-fidelity matched pages：65
- extra generated pages：63
- provider-backed page research requests：0
- budget stopped pages：0
- stalled pages：0
- invalid output pages：0
- provider failed pages：1

## 覆盖统计

- 专题页覆盖：generated 7 / reference 7（repo-archetype=0）
- evidence 落页：generated 105 / reference 65
- citation 密度：generated 83.64 / reference 79.69
- 图表达覆盖：generated 107 / reference 65
- 主章节骨架短板：61 页
- 英文 raw docs 命名残留：matched 0 / extra 0
- page research 请求：0
- page enrichment 请求：0
- stop reason 分布：completed(93)、provider_error(1)
- research session 聚合：turns=0, tool_calls=0, delta_section=375, delta_evidence=140, delta_diagram=83, child_digest=0
- 已规划专题类型：专题页(7)
- 高频缺失专题：无

## Decomposition 命中

- generated：runtime(60)、compiler-pipeline(68)、api-surface(31)、config-surface(21)、docs-guide(106)、testing(67)、example-tutorial(47)、troubleshooting(15)、integration-platform(83)
- reference：runtime(32)、compiler-pipeline(47)、api-surface(22)、config-surface(8)、docs-guide(65)、testing(30)、example-tutorial(47)、troubleshooting(65)、integration-platform(37)
- 高频缺口：troubleshooting(57)、compiler-pipeline(14)、example-tutorial(14)、api-surface(10)、runtime(6)、testing(4)

## 项目结论

- 单页章节拆分比 reference 粗，主题混杂在同一页里
- docs-backed 页面主章节骨架仍偏离 reference，存在英文原始 docs heading 或泛化模板回退
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 编译工具链/dagger-spi.md 被 2 个 reference 页面共享映射
- 平台绑定-Android/dagger-android.md 被 3 个 reference 页面共享映射
- 核心模块/dagger-producers.md 被 2 个 reference 页面共享映射
- 平台绑定-Android/平台绑定-Android.md 被 4 个 reference 页面共享映射
- 核心模块/examples.md 被 9 个 reference 页面共享映射
- 概念指南/README.md 被 2 个 reference 页面共享映射
- 配置参考/配置参考.md 被 2 个 reference 页面共享映射
- 核心模块/核心概念/Inject-注解详解.md 被 2 个 reference 页面共享映射
- 核心模块/dagger-lint.md 被 2 个 reference 页面共享映射
- 核心模块/核心模块.md 被 2 个 reference 页面共享映射
- 项目概述.md 被 2 个 reference 页面共享映射

## 额外生成页面

- API-参考/API-参考.md (API 参考, 168 行)
- API-参考/dagger.md (API：dagger, 168 行)
- API-参考/java.md (API：java, 213 行)
- API-参考/类型定义参考.md (类型定义参考, 214 行)
- API-参考/类型定义参考/API类型定义.md (API类型定义, 174 行)
- API-参考/类型定义参考/工具类型定义.md (工具类型定义, 208 行)
- API-参考/类型定义参考/框架类型定义.md (框架类型定义, 174 行)
- 平台绑定-Android/dagger-android-support.md (dagger-android-support, 131 行)
- 平台绑定-Android/dagger-lint-android.md (dagger-lint-android, 40 行)
- 开发工具/tools.md (tools, 132 行)
- 开发工具/开发工具.md (开发工具, 138 行)
- 构建系统/buildSrc.md (buildSrc, 147 行)
- 构建系统/构建系统.md (构建系统, 138 行)
- 核心模块/dagger-grpc-server.md (dagger-grpc-server, 126 行)
- 核心模块/dagger-kythe.md (dagger-kythe, 112 行)
- 核心模块/dagger-testing.md (dagger-testing, 117 行)
- 核心模块/dagger.md (dagger, 180 行)
- 核心模块/gradle.md (gradle, 92 行)
- 核心模块/hilt-android-testing.md (hilt-android-testing, 139 行)
- 核心模块/hilt-testing.md (hilt-testing, 46 行)
- 核心模块/java.md (java, 174 行)
- 核心模块/javatests.md (javatests, 154 行)
- 核心模块/util.md (util, 132 行)
- 核心模块/高级特性与扩展.md (高级特性与扩展, 348 行)
- 核心运行时/dagger-runtime.md (dagger-runtime, 149 行)
- 核心运行时/核心概念/Inject-注解详解.md (@Inject 注解详解, 172 行)
- 核心运行时/核心概念/绑定与提供者模式.md (绑定与提供者模式, 149 行)
- 核心运行时/核心运行时.md (核心运行时, 153 行)
- 框架集成-Hilt/hilt-android.md (hilt-android, 167 行)
- 框架集成-Hilt/hilt-compiler.md (hilt-compiler, 171 行)
- 框架集成-Hilt/hilt-core.md (hilt-core, 170 行)
- 框架集成-Hilt/框架集成-Hilt.md (框架集成：Hilt, 156 行)
- 概念指南/CONTRIBUTING.md (CONTRIBUTING, 147 行)
- 概念指南/buildSrc.md (BuildSrc, 170 行)
- 概念指南/dagger-grpc-server.md (Dagger Grpc Server, 171 行)
- 概念指南/hilt-core/main/java/dagger/hilt.md (Hilt, 170 行)
- 概念指南/概念指南.md (概念指南, 129 行)
- 测试基础设施/dagger-testing.md (测试：dagger-testing, 120 行)
- 测试基础设施/hilt-android-testing.md (测试：hilt-android-testing, 139 行)
- 测试基础设施/hilt-testing.md (测试：hilt-testing, 110 行)
- 测试基础设施/javatests.md (测试：javatests, 154 行)
- 测试基础设施/测试基础设施.md (测试基础设施, 159 行)
- 编译工具链/dagger-android-proguard-processor.md (dagger-android-proguard-processor, 110 行)
- 编译工具链/dagger-compiler.md (dagger-compiler, 147 行)
- 编译工具链/dagger-grpc-server-annotations.md (dagger-grpc-server-annotations, 112 行)
- 编译工具链/dagger-grpc-server-processor.md (dagger-grpc-server-processor, 127 行)
- 配置参考/AndroidManifest-Xml.md (AndroidManifest Xml, 106 行)
- 配置参考/Bazelignore.md (Bazelignore, 106 行)
- 配置参考/Bazelrc.md (Bazelrc, 103 行)
- 配置参考/Bazelversion.md (Bazelversion, 106 行)
- 配置参考/Binary-Artifacts.md (Binary Artifacts, 106 行)
- 配置参考/Build-Gradle-Kts.md (Build Gradle Kts, 103 行)
- 配置参考/Build-Gradle.md (Build Gradle, 95 行)
- 配置参考/Gitignore.md (Gitignore, 95 行)
- 配置参考/Gradle-Properties.md (Gradle Properties, 103 行)
- 配置参考/Gradle-Wrapper-Properties.md (Gradle Wrapper Properties, 106 行)
- 配置参考/HiltProcessingEnvConfigs-Java.md (HiltProcessingEnvConfigs Java, 103 行)
- 配置参考/Libs-Versions.md (Libs Versions, 95 行)
- 配置参考/Maven-Install.md (Maven Install, 95 行)
- 配置参考/Pom-Xml.md (Pom Xml, 95 行)
- 配置参考/Settings-Gradle-Kts.md (Settings Gradle Kts, 106 行)
- 配置参考/Settings-Gradle.md (Settings Gradle, 106 行)
- 配置参考/Wiki-Dev.md (Wiki Dev, 97 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API参考文档/API参考文档.md | 编译工具链/dagger-spi.md | 594/173 | 239/53 | 0/12 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：androidinjector.java、hasandroidinjector.java、cancellationpolicy.java、producer.java、producermodule.java、produces.java、production.java、productioncomponent.java |
| API参考文档/Android API.md | 平台绑定-Android/dagger-android.md | 398/138 | 168/52 | 21/5 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：readme.md、androidinjectordescriptor.java、contributesandroidinjectorprocessingstep.java、androidsupportinjection.java、androidsupportinjectionmodule.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java |
| API参考文档/Hilt API.md | API-参考/Hilt-API.md | 356/174 | 120/88 | 0/5 | 8/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java |
| API参考文档/异步处理API.md | 核心模块/dagger-producers.md | 367/159 | 163/46 | 0/10 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：provisiondependencyonproducerbindingvalidator.java、dependencymethodproducercreationexpression.java、producercreationexpression.java、cancellationpolicy.java、produced.java、producer.java、producermodule.java、produces.java |
| API参考文档/编译时API.md | 编译工具链/dagger-spi.md | 407/173 | 164/53 | 19/12 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentprocessor.java、componentdescriptor.java、bindinggraph.java、componentpath.java、dependencyrequest.java、bindinggraphplugin.java、diagnosticreporter.java、bindinggraphplugin.ini |
| API参考文档/运行时API.md | API-参考/运行时API.md | 388/174 | 102/88 | 0/5 | 3/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：binds.java、bindsinstance.java、component.java、lazy.java、mapkey.java、membersinjector.java、module.java、provides.java |
| Android集成开发/@ContributesAndroidInjector注解.md | 编译工具链/dagger-android-processor.md | 324/166 | 122/46 | 16/12 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：xtypenames.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、androidinjectiontest.java |
| Android集成开发/Activity注入.md | 平台绑定-Android/dagger-android.md | 302/138 | 117/52 | 17/5 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java、simpleactivity.java |
| Android集成开发/Android生命周期管理.md | 平台绑定-Android/dagger-android.md | 383/138 | 153/52 | 21/5 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java、dispatchingandroidinjectortest.java |
| Android集成开发/Android集成开发.md | 平台绑定-Android/平台绑定-Android.md | 321/153 | 158/34 | 0/5 | 9/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、androidinjectionkey.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java、coffeeapp.java |
| Android集成开发/Fragment注入.md | 平台绑定-Android/平台绑定-Android.md | 318/153 | 128/34 | 0/5 | 5/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：androidsupportinjection.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java |
| Android集成开发/Service注入.md | 平台绑定-Android/平台绑定-Android.md | 275/153 | 103/34 | 0/5 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java |
| Android集成开发/其他组件注入.md | 平台绑定-Android/平台绑定-Android.md | 323/153 | 150/34 | 0/5 | 8/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：contributesandroidinjector.java、daggerbroadcastreceiver.java、daggercontentprovider.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java |
| Hilt框架详解/@HiltAndroidApp与注解使用.md | 框架集成-Hilt/Hilt框架详解/HiltAndroidApp与注解使用.md | 315/247 | 90/50 | 0/17 | 4/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：coffeeapp.java、androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、generatesrootinput.java、installin.java |
| Hilt框架详解/Hilt核心概念与架构.md | 系统架构.md | 295/166 | 127/41 | 17/4 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、readme.md、singletoncomponent.java、definecomponentnoparent.java |
| Hilt框架详解/Hilt框架详解.md | 框架集成-Hilt/Hilt框架详解.md | 328/317 | 134/57 | 0/22 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、generatesrootinput.java、installin.java、generatedcomponent.java、generatedcomponentmanager.java |
| Hilt框架详解/模块安装与依赖管理.md | 框架集成-Hilt/Hilt框架详解/模块安装与依赖管理.md | 289/260 | 124/50 | 18/18 | 8/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：duplicatebindingsvalidator.java、moduleprocessingstep.java、modulevalidator.java、activitycomponent.java、definecomponent.java、installin.java、definecomponentnoparent.java |
| Hilt框架详解/测试支持与模拟.md | 框架集成-Hilt/Hilt框架详解/测试支持与模拟.md | 306/247 | 141/50 | 19/17 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：coffeeappfakeheatertest.java、earlysingletoncomponentcreator.java、markthatrulesranrule.java、testapplicationcomponentmanager.java、testcomponentdatasupplier.java、bindvalue.java、customtestapplication.java、hiltandroidrule.java |
| Hilt框架详解/组件树结构与生命周期.md | 框架集成-Hilt/Hilt框架详解/组件树结构与生命周期.md | 284/273 | 124/50 | 13/19 | 8/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：definecomponent.java、entrypoint.java、installin.java |
| Hilt框架详解/编译时处理与代码生成.md | 框架集成-Hilt/Hilt框架详解/编译时处理与代码生成.md | 296/260 | 134/50 | 0/18 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：androidentrypointprocessingstep.java、androidentrypointprocessor.java、bindvalueprocessingstep.java、bindvalueprocessor.java、baseprocessingstep.java、javacbaseprocessingstepprocessor.java、aggregateddepsprocessingstep.java、aggregateddepsprocessor.java |
| 异步处理与生产者.md | API-参考/异步处理与生产者.md | 296/174 | 131/88 | 0/5 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：cancellationpolicy.java、produced.java、producer.java、producermodule.java、produces.java、productioncomponent.java、productionscope.java、productionsubcomponent.java |
| 快速开始.md | 核心模块/examples.md | 290/146 | 135/46 | 0/9 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：readme.md、electricheater.java、heater.java、pump.java |
| 故障排除与调试.md | 概念指南/README.md | 317/214 | 107/63 | 0/13 | 7/1 | 图表少于 reference；缺少关键文件提及：validationreport.java、errormessages.java、dependencycyclevalidator.java、duplicatebindingsvalidator.java、missingbindingvalidator.java、diagnosticreporter.java |
| 构建与部署.md | 配置参考/配置参考.md | 339/175 | 136/44 | 20/5 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：ci.yml、contributing.md、daggerconventionplugin.kt |
| 核心概念/作用域与生命周期管理.md | 核心运行时/核心概念.md | 327/185 | 131/69 | 0/6 | 9/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：incompatiblyscopedbindingsvalidator.java、scope.java、component.java、reusable.java、subcomponent.java |
| 核心概念/依赖注入基础理论.md | 核心模块/examples.md | 323/146 | 138/46 | 19/9 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：binds.java、component.java、lazy.java、membersinjector.java、module.java、provides.java、doublecheck.java |
| 核心概念/核心概念.md | 核心模块/核心概念.md | 326/309 | 128/127 | 17/14 | 8/2 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：scopes.java、componentdescriptor.java、scope.java、binds.java、bindsinstance.java、component.java、module.java、provides.java |
| 核心概念/核心注解详解/@Component 注解详解.md | 核心模块/examples.md | 211/146 | 88/46 | 0/9 | 4/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentprocessor.java、componentannotation.java、componentgenerator.java、component.java、module.java、subcomponent.java、coffeeshop.java |
| 核心概念/核心注解详解/@Inject 注解详解.md | 核心模块/核心概念/Inject-注解详解.md | 308/292 | 145/121 | 19/16 | 9/2 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：injectionannotations.java、injectionbinding.java、membersinjectionbinding.java、membersinjectorbinding.java、injectprocessingstep.java、membersinjectorgenerator.java |
| 核心概念/核心注解详解/@Module 注解详解.md | 核心模块/高级特性与扩展/多值绑定高级用法.md | 236/324 | 105/119 | 0/17 | 6/2 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：moduleprocessingstep.java、modulevalidator.java、modulegenerator.java、binds.java、module.java、provides.java、elementsintoset.java、intomap.java |
| 核心概念/核心注解详解/@Provides 与 @Binds 注解详解.md | API-参考/Provides-与--Binds-注解详解.md | 274/174 | 103/88 | 13/5 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：androidinjector.java、delegatecomponentprocessor.java、bindsinstanceelementvalidator.java、bindsmethodvalidator.java、providesmethodvalidator.java、binds.java、bindsinstance.java、provides.java |
| 核心概念/核心注解详解/核心注解详解.md | 核心模块/examples.md | 262/146 | 98/46 | 0/9 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentprocessor.java、componentprocessingstep.java、component.java、module.java、provides.java、injectedfieldsignature.java、coffeeappcomponent.java、heater.java |
| 核心概念/组件与模块系统/模块组织与管理.md | 核心模块/examples.md | 317/146 | 117/46 | 0/9 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：modulevalidator.java、providesmethodvalidator.java、moduleproxies.java、module.java、provides.java |
| 核心概念/组件与模块系统/组件与模块系统.md | 核心模块/dagger-lint.md | 331/144 | 139/78 | 0/6 | 9/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentprocessor.java、bindinggraphvalidationmodule.java、subcomponentfactorymethodvalidator.java、componentpath.java、componenthierarchyvalidator.java、componentimplementation.java、component.java、module.java |
| 核心概念/组件与模块系统/组件关系设计/子组件机制.md | 核心模块/核心概念/Inject-注解详解.md | 340/292 | 144/121 | 23/16 | 10/2 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：compositebindinggraphplugin.java、dependencycyclevalidator.java、subcomponentfactorymethodvalidator.java、currentimplementationsubcomponent.java、toplevelimplementationcomponent.java、subcomponent.java、simpleactivity.kt、simpleactivitycomponent.kt |
| 核心概念/组件与模块系统/组件关系设计/组件依赖关系.md | 核心模块/dagger-producers.md | 266/159 | 112/46 | 0/10 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentrequirementrequestrepresentation.java、componentdependencybinding.java、componentdescriptor.java、componentrequirement.java、dependencycyclevalidator.java、bindingkind.java、componentvalidator.java、dependencymethodproducercreationexpression.java |
| 核心概念/组件与模块系统/组件关系设计/组件关系设计.md | 核心模块/dagger-lint.md | 266/144 | 103/78 | 0/6 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentdependencybinding.java、subcomponentcreatorbinding.java、subcomponentdeclaration.java、subcomponentfactorymethodvalidator.java、componentgenerator.java、componenthierarchyvalidator.java、subcomponentcreatorrequestrepresentation.java、productionsubcomponent.java |
| 核心概念/组件与模块系统/组件基础概念.md | 核心模块/核心模块.md | 231/175 | 90/44 | 0/5 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：component.java、componentdescriptor.java、componentgenerator.java、membersinjector.java、mycomponent.java |
| 核心概念/组件与模块系统/组件构建器与工厂.md | 核心模块/核心模块.md | 358/175 | 111/44 | 15/5 | 5/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：bindsinstance.java、builderbindsinstanceparametertest.java、buildertest.java、component.java、componentcreatordescriptor.java、componentcreatorkind.java、componentcreatorvalidator.java、errormessages.java |
| 核心概念/绑定与提供者模式.md | 核心模块/核心概念/绑定与提供者模式.md | 246/273 | 99/121 | 0/13 | 5/2 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：binds.java、bindsmethodvalidator.java、bindsoptionalof.java、contributionbinding.java、intomap.java、intoset.java、lazy.java、module.java |
| 测试策略与最佳实践/Android测试.md | 核心模块/高级特性与扩展/延迟初始化与线程安全.md | 248/270 | 98/121 | 0/16 | 5/2 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：coffeeappfakeheatertest.java、coffeeappfakepumptest.java、bindvalue.java、customtestapplication.java、hiltandroidtest.java、testinstallin.java、installin.java |
| 测试策略与最佳实践/单元测试.md | 测试基础设施/测试策略与最佳实践/单元测试.md | 254/168 | 108/82 | 15/5 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：doublechecktest.java、instancefactorytest.java、mapproviderfactorytest.java、setfactorytest.java、bindinggraphsubject.java、coffeeappfakeheatertest.java、testinstallin.java |
| 测试策略与最佳实践/性能测试与监控.md | 测试基础设施/测试策略与最佳实践/性能测试与监控.md | 317/168 | 125/82 | 0/5 | 3/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：producermonitor.java、producertimingrecorder.java、producertoken.java、productioncomponentmonitor.java、productioncomponenttimingrecorder.java、timingproducermonitor.java、timingproductioncomponentmonitor.java、timingrecorders.java |
| 测试策略与最佳实践/测试策略与最佳实践.md | 测试基础设施/测试策略与最佳实践.md | 253/208 | 103/88 | 15/6 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：bindinggraphsubject.java、coffeeappfakeheatertest.java、coffeeappfakepumptest.java、hiltandroidtest.java、uninstallmodules.java、testinstallin.java |
| 测试策略与最佳实践/测试自动化与CI_CD.md | 配置参考/配置参考.md | 230/175 | 91/44 | 0/5 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：readme.md |
| 测试策略与最佳实践/集成测试.md | 测试基础设施/测试策略与最佳实践/集成测试.md | 293/202 | 111/76 | 19/12 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：coffeeappfakeheatertest.java、testinjector.java、bindvalue.java、customtestapplication.java、hiltandroidtest.java、uninstallmodules.java、testinstallin.java |
| 示例与教程/Android应用示例.md | 项目概述.md | 258/223 | 100/51 | 0/6 | 6/1 | 图表少于 reference；缺少关键文件提及：coffeeappfakeheatertest.java、coffeeappfakepumptest.java、definecomponent.java、entrypoint.java、entrypoints.java、installin.java |
| 示例与教程/基础示例.md | 核心模块/examples.md | 262/146 | 119/46 | 0/9 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：electricheater.java、heater.java、pump.java |
| 示例与教程/实践教程.md | 核心模块/examples.md | 350/146 | 159/46 | 0/9 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：readme.md、binds.java、component.java、lazy.java、module.java、provides.java、reusable.java、subcomponent.java |
| 示例与教程/示例与教程.md | 核心模块/examples.md | 300/146 | 147/46 | 0/9 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：readme.md、androidinjection.java、daggerapplication.java、producer.java、production.java、heater.java、pump.java、definecomponent.java |
| 示例与教程/高级应用示例.md | 核心模块/examples.md | 319/146 | 146/46 | 0/9 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：contributing.md、readme.md、producer.java、productioncomponent.java、binds.java、component.java、lazy.java、module.java |
| 编译时处理机制/代码生成策略.md | 编译工具链/编译时处理机制/代码生成策略.md | 361/163 | 159/38 | 0/15 | 9/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：sourcefilegenerator.java、componentgenerator.java、componentcreatorimplementation.java、componentimplementation.java、factorygenerator.java、generatedimplementation.java、membersinjectorgenerator.java、modulegenerator.java |
| 编译时处理机制/依赖图构建.md | 编译工具链/编译时处理机制/依赖图构建.md | 335/228 | 164/38 | 0/17 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：bindinggraphfactory.java、componentnodeimpl.java、dependencyedgeimpl.java、compositebindinggraphplugin.java、dependencycyclevalidator.java、duplicatebindingsvalidator.java、incompatiblyscopedbindingsvalidator.java、missingbindingvalidator.java |
| 编译时处理机制/增量编译优化.md | 编译工具链/编译时处理机制/增量编译优化.md | 268/201 | 122/38 | 0/16 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、kspcomponentprocessor.java、clearablecache.java、processingenvironmentcompileroptions.java、componentvalidator.java、injectvalidator.java |
| 编译时处理机制/注解处理基础.md | 编译工具链/编译时处理机制/注解处理基础.md | 296/247 | 135/38 | 0/18 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、processingenvironmentcompileroptions.java、componentprocessingstep.java、injectprocessingstep.java、mapkeyprocessingstep.java、moduleprocessingstep.java、processingstepsmodule.java |
| 编译时处理机制/编译时处理机制.md | 编译工具链/编译时处理机制.md | 327/306 | 135/45 | 0/22 | 5/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、kspcomponentprocessor.java、processingenvironmentmodule.java、validationreport.java、bindinggraphfactory.java、bindinggraphvalidationmodule.java、compositebindinggraphplugin.java |
| 编译时处理机制/验证与错误处理.md | 编译工具链/编译时处理机制/验证与错误处理.md | 399/201 | 180/38 | 0/18 | 9/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：validationreport.java、errormessages.java、bindinggraphplugin.java、diagnosticreporter.java、bindinggraphvalidator.java、componentvalidator.java、diagnosticreporterfactory.java、modulevalidator.java |
| 贡献指南.md | 概念指南/README.md | 251/214 | 71/63 | 0/13 | 3/1 | 图表少于 reference；缺少关键文件提及：build.gradle.kt、settings.gradle.kt、libs.versions.toml |
| 项目概述.md | 项目概述.md | 244/223 | 114/51 | 0/6 | 7/1 | 图表少于 reference；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、component.java、lazy.java、module.java、subcomponent.java、definecomponent.java、settings.gradle.kt |
| 高级特性与扩展/SPI扩展机制.md | 编译工具链/SPI扩展机制.md | 393/241 | 185/38 | 0/18 | 8/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：serviceloaders.java、compositebindinggraphplugin.java、diagnosticreporterfactory.java、externalbindinggraphplugins.java、validationbindinggraphplugin.java、validationbindinggraphplugins.java、bindinggraphplugin.java、diagnosticreporter.java |
| 高级特性与扩展/多值绑定高级用法.md | 核心运行时/高级特性与扩展/多值绑定高级用法.md | 311/172 | 133/59 | 21/10 | 9/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：intomap.java、intoset.java、mapfactorycreationexpression.java、mapproviderfactory.java、mapproviderfactorytest.java、multibindingdeclaration.java、multibinds.java、setbuildertest.java |
| 高级特性与扩展/延迟初始化与线程安全.md | 核心运行时/高级特性与扩展/延迟初始化与线程安全.md | 283/172 | 134/59 | 0/10 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：component.java、lazy.java、module.java、doublechecktest.java、singlechecktest.java |
| 高级特性与扩展/性能监控与调试.md | API-参考/性能监控与调试.md | 312/174 | 151/88 | 16/5 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：clearablecache.java、compileroptions.java、validation.java、producermonitor.java、producertimingrecorder.java、producertoken.java、productioncomponentmonitor.java、productioncomponenttimingrecorder.java |
| 高级特性与扩展/自定义扩展开发.md | 编译工具链/编译工具链.md | 352/168 | 164/44 | 24/5 | 9/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：delegateandroidprocessor.java、delegatecomponentprocessor.java、sourcefilegenerator.java、assistedfactoryprocessingstep.java、injectprocessingstep.java、processingstepsmodule.java、validationbindinggraphplugins.java、bindinggraphplugin.java |
| 高级特性与扩展/高级特性与扩展.md | 核心运行时/高级特性与扩展.md | 377/184 | 146/69 | 0/6 | 8/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：bindinggraphplugin.java、bindsoptionalof.java、componentprocessor.java、compositebindinggraphplugin.java、externalbindinggraphplugins.java、intomap.java、intoset.java、lazy.java |

## 逐文件详情

### API参考文档/API参考文档.md

- reference 标题：API参考文档
- 生成页：编译工具链/dagger-spi.md（dagger-spi）
- 匹配分数：216
- 页面类型：other / other
- 行数：594 / 173
- 段落行数：239 / 53
- Evidence：0 / 12
- Mermaid：9 / 1
- 文件提及重合：binding.java、key.java、requestkind.java、scope.java
- reference 关键文件未覆盖：androidinjector.java、hasandroidinjector.java、cancellationpolicy.java、producer.java、producermodule.java、produces.java、production.java、productioncomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：androidinjector.java、hasandroidinjector.java、cancellationpolicy.java、producer.java、producermodule.java、produces.java、production.java、productioncomponent.java

### API参考文档/Android API.md

- reference 标题：Android API
- 生成页：平台绑定-Android/dagger-android.md（dagger-android）
- 匹配分数：222
- 页面类型：other / other
- 行数：398 / 138
- 段落行数：168 / 52
- Evidence：21 / 5
- Mermaid：8 / 1
- 文件提及重合：androidinjection.java、androidinjectionmodule.java、androidinjector.java、daggeractivity.java、daggerapplication.java、daggerfragment.java、daggerservice.java
- reference 关键文件未覆盖：readme.md、androidinjectordescriptor.java、contributesandroidinjectorprocessingstep.java、androidsupportinjection.java、androidsupportinjectionmodule.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：readme.md、androidinjectordescriptor.java、contributesandroidinjectorprocessingstep.java、androidsupportinjection.java、androidsupportinjectionmodule.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java

### API参考文档/Hilt API.md

- reference 标题：Hilt API
- 生成页：API-参考/Hilt-API.md（Hilt API）
- 匹配分数：266
- 页面类型：other / other
- 行数：356 / 174
- 段落行数：120 / 88
- Evidence：0 / 5
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java

### API参考文档/异步处理API.md

- reference 标题：异步处理API
- 生成页：核心模块/dagger-producers.md（dagger-producers）
- 匹配分数：82
- 页面类型：other / module
- 行数：367 / 159
- 段落行数：163 / 46
- Evidence：0 / 10
- Mermaid：6 / 1
- 文件提及重合：abstractproducer.java、mapproducer.java
- reference 关键文件未覆盖：provisiondependencyonproducerbindingvalidator.java、dependencymethodproducercreationexpression.java、producercreationexpression.java、cancellationpolicy.java、produced.java、producer.java、producermodule.java、produces.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：provisiondependencyonproducerbindingvalidator.java、dependencymethodproducercreationexpression.java、producercreationexpression.java、cancellationpolicy.java、produced.java、producer.java、producermodule.java、produces.java

### API参考文档/编译时API.md

- reference 标题：编译时API
- 生成页：编译工具链/dagger-spi.md（dagger-spi）
- 匹配分数：144
- 页面类型：other / other
- 行数：407 / 173
- 段落行数：164 / 53
- Evidence：19 / 12
- Mermaid：7 / 1
- 文件提及重合：binding.java、key.java、requestkind.java、scope.java
- reference 关键文件未覆盖：componentprocessor.java、componentdescriptor.java、bindinggraph.java、componentpath.java、dependencyrequest.java、bindinggraphplugin.java、diagnosticreporter.java、bindinggraphplugin.ini
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentprocessor.java、componentdescriptor.java、bindinggraph.java、componentpath.java、dependencyrequest.java、bindinggraphplugin.java、diagnosticreporter.java、bindinggraphplugin.ini

### API参考文档/运行时API.md

- reference 标题：运行时API
- 生成页：API-参考/运行时API.md（运行时API）
- 匹配分数：252
- 页面类型：other / other
- 行数：388 / 174
- 段落行数：102 / 88
- Evidence：0 / 5
- Mermaid：3 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：binds.java、bindsinstance.java、component.java、lazy.java、mapkey.java、membersinjector.java、module.java、provides.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：binds.java、bindsinstance.java、component.java、lazy.java、mapkey.java、membersinjector.java、module.java、provides.java

### Android集成开发/@ContributesAndroidInjector注解.md

- reference 标题：@ContributesAndroidInjector注解
- 生成页：编译工具链/dagger-android-processor.md（dagger-android-processor）
- 匹配分数：170
- 页面类型：other / other
- 行数：324 / 166
- 段落行数：122 / 46
- Evidence：16 / 12
- Mermaid：7 / 1
- 文件提及重合：androidinjectordescriptor.java、androidprocessor.java、baseprocessingstep.java、contributesandroidinjectorprocessingstep.java、delegateandroidprocessor.java
- reference 关键文件未覆盖：xtypenames.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、androidinjectiontest.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：xtypenames.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、androidinjectiontest.java

### Android集成开发/Activity注入.md

- reference 标题：Activity注入
- 生成页：平台绑定-Android/dagger-android.md（dagger-android）
- 匹配分数：144
- 页面类型：other / other
- 行数：302 / 138
- 段落行数：117 / 52
- Evidence：17 / 5
- Mermaid：6 / 1
- 文件提及重合：androidinjection.java、androidinjectionmodule.java、androidinjector.java、daggeractivity.java
- reference 关键文件未覆盖：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java、simpleactivity.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java、simpleactivity.java

### Android集成开发/Android生命周期管理.md

- reference 标题：Android生命周期管理
- 生成页：平台绑定-Android/dagger-android.md（dagger-android）
- 匹配分数：196
- 页面类型：other / other
- 行数：383 / 138
- 段落行数：153 / 52
- Evidence：21 / 5
- Mermaid：8 / 1
- 文件提及重合：androidinjection.java、androidinjectionmodule.java、androidinjector.java、daggeractivity.java、daggerapplication.java、daggerfragment.java
- reference 关键文件未覆盖：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java、dispatchingandroidinjectortest.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java、dispatchingandroidinjectortest.java

### Android集成开发/Android集成开发.md

- reference 标题：Android集成开发
- 生成页：平台绑定-Android/平台绑定-Android.md（平台绑定：Android）
- 匹配分数：144
- 页面类型：other / other
- 行数：321 / 153
- 段落行数：158 / 34
- Evidence：0 / 5
- Mermaid：9 / 1
- 文件提及重合：daggerfragment.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java
- reference 关键文件未覆盖：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、androidinjectionkey.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java、coffeeapp.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、daggerappcompatactivity.java、androidinjectionkey.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java、coffeeapp.java

### Android集成开发/Fragment注入.md

- reference 标题：Fragment注入
- 生成页：平台绑定-Android/平台绑定-Android.md（平台绑定：Android）
- 匹配分数：162
- 页面类型：other / other
- 行数：318 / 153
- 段落行数：128 / 34
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：daggerfragment.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java
- reference 关键文件未覆盖：androidsupportinjection.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：androidsupportinjection.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java

### Android集成开发/Service注入.md

- reference 标题：Service注入
- 生成页：平台绑定-Android/平台绑定-Android.md（平台绑定：Android）
- 匹配分数：114
- 页面类型：other / other
- 行数：275 / 153
- 段落行数：103 / 34
- Evidence：0 / 5
- Mermaid：7 / 1
- 文件提及重合：androidinjection.java、androidinjector.java、daggerservice.java
- reference 关键文件未覆盖：contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java

### Android集成开发/其他组件注入.md

- reference 标题：其他组件注入
- 生成页：平台绑定-Android/平台绑定-Android.md（平台绑定：Android）
- 匹配分数：144
- 页面类型：other / other
- 行数：323 / 153
- 段落行数：150 / 34
- Evidence：0 / 5
- Mermaid：8 / 1
- 文件提及重合：androidinjection.java、androidinjectionmodule.java、androidinjector.java、daggerintentservice.java
- reference 关键文件未覆盖：contributesandroidinjector.java、daggerbroadcastreceiver.java、daggercontentprovider.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：contributesandroidinjector.java、daggerbroadcastreceiver.java、daggercontentprovider.java、dispatchingandroidinjector.java、hasandroidinjector.java、androidinjectiontest.java

### Hilt框架详解/@HiltAndroidApp与注解使用.md

- reference 标题：@HiltAndroidApp与注解使用
- 生成页：框架集成-Hilt/Hilt框架详解/HiltAndroidApp与注解使用.md（@HiltAndroidApp与注解使用）
- 匹配分数：322
- 页面类型：other / other
- 行数：315 / 247
- 段落行数：90 / 50
- Evidence：0 / 17
- Mermaid：4 / 1
- 文件提及重合：entrypoints.java、package-info.java
- reference 关键文件未覆盖：coffeeapp.java、androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、generatesrootinput.java、installin.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：coffeeapp.java、androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、generatesrootinput.java、installin.java

### Hilt框架详解/Hilt核心概念与架构.md

- reference 标题：Hilt核心概念与架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：100
- 页面类型：architecture / architecture
- 行数：295 / 166
- 段落行数：127 / 41
- Evidence：17 / 4
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、readme.md、singletoncomponent.java、definecomponentnoparent.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、readme.md、singletoncomponent.java、definecomponentnoparent.java

### Hilt框架详解/Hilt框架详解.md

- reference 标题：Hilt框架详解
- 生成页：框架集成-Hilt/Hilt框架详解.md（Hilt框架详解）
- 匹配分数：320
- 页面类型：other / other
- 行数：328 / 317
- 段落行数：134 / 57
- Evidence：0 / 22
- Mermaid：7 / 1
- 文件提及重合：entrypoints.java、singletoncomponent.java
- reference 关键文件未覆盖：androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、generatesrootinput.java、installin.java、generatedcomponent.java、generatedcomponentmanager.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：androidentrypoint.java、hiltandroidapp.java、definecomponent.java、entrypoint.java、generatesrootinput.java、installin.java、generatedcomponent.java、generatedcomponentmanager.java

### Hilt框架详解/模块安装与依赖管理.md

- reference 标题：模块安装与依赖管理
- 生成页：框架集成-Hilt/Hilt框架详解/模块安装与依赖管理.md（模块安装与依赖管理）
- 匹配分数：368
- 页面类型：module / module
- 行数：289 / 260
- 段落行数：124 / 50
- Evidence：18 / 18
- Mermaid：8 / 1
- 文件提及重合：singletoncomponent.java
- reference 关键文件未覆盖：duplicatebindingsvalidator.java、moduleprocessingstep.java、modulevalidator.java、activitycomponent.java、definecomponent.java、installin.java、definecomponentnoparent.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：duplicatebindingsvalidator.java、moduleprocessingstep.java、modulevalidator.java、activitycomponent.java、definecomponent.java、installin.java、definecomponentnoparent.java

### Hilt框架详解/测试支持与模拟.md

- reference 标题：测试支持与模拟
- 生成页：框架集成-Hilt/Hilt框架详解/测试支持与模拟.md（测试支持与模拟）
- 匹配分数：254
- 页面类型：other / other
- 行数：306 / 247
- 段落行数：141 / 50
- Evidence：19 / 17
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：coffeeappfakeheatertest.java、earlysingletoncomponentcreator.java、markthatrulesranrule.java、testapplicationcomponentmanager.java、testcomponentdatasupplier.java、bindvalue.java、customtestapplication.java、hiltandroidrule.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：coffeeappfakeheatertest.java、earlysingletoncomponentcreator.java、markthatrulesranrule.java、testapplicationcomponentmanager.java、testcomponentdatasupplier.java、bindvalue.java、customtestapplication.java、hiltandroidrule.java

### Hilt框架详解/组件树结构与生命周期.md

- reference 标题：组件树结构与生命周期
- 生成页：框架集成-Hilt/Hilt框架详解/组件树结构与生命周期.md（组件树结构与生命周期）
- 匹配分数：292
- 页面类型：other / other
- 行数：284 / 273
- 段落行数：124 / 50
- Evidence：13 / 19
- Mermaid：8 / 1
- 文件提及重合：singletoncomponent.java
- reference 关键文件未覆盖：definecomponent.java、entrypoint.java、installin.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：definecomponent.java、entrypoint.java、installin.java

### Hilt框架详解/编译时处理与代码生成.md

- reference 标题：编译时处理与代码生成
- 生成页：框架集成-Hilt/Hilt框架详解/编译时处理与代码生成.md（编译时处理与代码生成）
- 匹配分数：256
- 页面类型：other / other
- 行数：296 / 260
- 段落行数：134 / 50
- Evidence：0 / 18
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：androidentrypointprocessingstep.java、androidentrypointprocessor.java、bindvalueprocessingstep.java、bindvalueprocessor.java、baseprocessingstep.java、javacbaseprocessingstepprocessor.java、aggregateddepsprocessingstep.java、aggregateddepsprocessor.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：androidentrypointprocessingstep.java、androidentrypointprocessor.java、bindvalueprocessingstep.java、bindvalueprocessor.java、baseprocessingstep.java、javacbaseprocessingstepprocessor.java、aggregateddepsprocessingstep.java、aggregateddepsprocessor.java

### 异步处理与生产者.md

- reference 标题：异步处理与生产者
- 生成页：API-参考/异步处理与生产者.md（异步处理与生产者）
- 匹配分数：262
- 页面类型：other / other
- 行数：296 / 174
- 段落行数：131 / 88
- Evidence：0 / 5
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：cancellationpolicy.java、produced.java、producer.java、producermodule.java、produces.java、productioncomponent.java、productionscope.java、productionsubcomponent.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：cancellationpolicy.java、produced.java、producer.java、producermodule.java、produces.java、productioncomponent.java、productionscope.java、productionsubcomponent.java

### 快速开始.md

- reference 标题：快速开始
- 生成页：核心模块/examples.md（examples）
- 匹配分数：192
- 页面类型：other / module
- 行数：290 / 146
- 段落行数：135 / 46
- Evidence：0 / 9
- Mermaid：6 / 1
- 文件提及重合：coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java、thermosiphon.java
- reference 关键文件未覆盖：readme.md、electricheater.java、heater.java、pump.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：readme.md、electricheater.java、heater.java、pump.java

### 故障排除与调试.md

- reference 标题：故障排除与调试
- 生成页：概念指南/README.md（README）
- 匹配分数：116
- 页面类型：other / other
- 行数：317 / 214
- 段落行数：107 / 63
- Evidence：0 / 13
- Mermaid：7 / 1
- 文件提及重合：contributing.md、readme.md
- reference 关键文件未覆盖：validationreport.java、errormessages.java、dependencycyclevalidator.java、duplicatebindingsvalidator.java、missingbindingvalidator.java、diagnosticreporter.java
- 结论：图表少于 reference；缺少关键文件提及：validationreport.java、errormessages.java、dependencycyclevalidator.java、duplicatebindingsvalidator.java、missingbindingvalidator.java、diagnosticreporter.java

### 构建与部署.md

- reference 标题：构建与部署
- 生成页：配置参考/配置参考.md（配置参考）
- 匹配分数：118
- 页面类型：workflow / other
- 行数：339 / 175
- 段落行数：136 / 44
- Evidence：20 / 5
- Mermaid：7 / 1
- 文件提及重合：build.gradle.kt、libs.versions.toml、settings.gradle.kt
- reference 关键文件未覆盖：ci.yml、contributing.md、daggerconventionplugin.kt
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：ci.yml、contributing.md、daggerconventionplugin.kt

### 核心概念/作用域与生命周期管理.md

- reference 标题：作用域与生命周期管理
- 生成页：核心运行时/核心概念.md（核心概念）
- 匹配分数：84
- 页面类型：other / other
- 行数：327 / 185
- 段落行数：131 / 69
- Evidence：0 / 6
- Mermaid：9 / 1
- 文件提及重合：doublecheck.java、singlecheck.java
- reference 关键文件未覆盖：incompatiblyscopedbindingsvalidator.java、scope.java、component.java、reusable.java、subcomponent.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：incompatiblyscopedbindingsvalidator.java、scope.java、component.java、reusable.java、subcomponent.java

### 核心概念/依赖注入基础理论.md

- reference 标题：依赖注入基础理论
- 生成页：核心模块/examples.md（examples）
- 匹配分数：252
- 页面类型：other / module
- 行数：323 / 146
- 段落行数：138 / 46
- Evidence：19 / 9
- Mermaid：7 / 1
- 文件提及重合：coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java
- reference 关键文件未覆盖：binds.java、component.java、lazy.java、membersinjector.java、module.java、provides.java、doublecheck.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：binds.java、component.java、lazy.java、membersinjector.java、module.java、provides.java、doublecheck.java

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：核心模块/核心概念.md（核心概念）
- 匹配分数：292
- 页面类型：other / module
- 行数：326 / 309
- 段落行数：128 / 127
- Evidence：17 / 14
- Mermaid：8 / 2
- 文件提及重合：coffeeapp.java
- reference 关键文件未覆盖：scopes.java、componentdescriptor.java、scope.java、binds.java、bindsinstance.java、component.java、module.java、provides.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：scopes.java、componentdescriptor.java、scope.java、binds.java、bindsinstance.java、component.java、module.java、provides.java

### 核心概念/核心注解详解/@Component 注解详解.md

- reference 标题：@Component 注解详解
- 生成页：核心模块/examples.md（examples）
- 匹配分数：266
- 页面类型：module / module
- 行数：211 / 146
- 段落行数：88 / 46
- Evidence：0 / 9
- Mermaid：4 / 1
- 文件提及重合：coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java、thermosiphon.java
- reference 关键文件未覆盖：componentprocessor.java、componentannotation.java、componentgenerator.java、component.java、module.java、subcomponent.java、coffeeshop.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentprocessor.java、componentannotation.java、componentgenerator.java、component.java、module.java、subcomponent.java、coffeeshop.java

### 核心概念/核心注解详解/@Inject 注解详解.md

- reference 标题：@Inject 注解详解
- 生成页：核心模块/核心概念/Inject-注解详解.md（@Inject 注解详解）
- 匹配分数：254
- 页面类型：other / module
- 行数：308 / 292
- 段落行数：145 / 121
- Evidence：19 / 16
- Mermaid：9 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：injectionannotations.java、injectionbinding.java、membersinjectionbinding.java、membersinjectorbinding.java、injectprocessingstep.java、membersinjectorgenerator.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：injectionannotations.java、injectionbinding.java、membersinjectionbinding.java、membersinjectorbinding.java、injectprocessingstep.java、membersinjectorgenerator.java

### 核心概念/核心注解详解/@Module 注解详解.md

- reference 标题：@Module 注解详解
- 生成页：核心模块/高级特性与扩展/多值绑定高级用法.md（多值绑定高级用法）
- 匹配分数：76
- 页面类型：other / module
- 行数：236 / 324
- 段落行数：105 / 119
- Evidence：0 / 17
- Mermaid：6 / 2
- 文件提及重合：coffeeapp.java、coffeemaker.java
- reference 关键文件未覆盖：moduleprocessingstep.java、modulevalidator.java、modulegenerator.java、binds.java、module.java、provides.java、elementsintoset.java、intomap.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：moduleprocessingstep.java、modulevalidator.java、modulegenerator.java、binds.java、module.java、provides.java、elementsintoset.java、intomap.java

### 核心概念/核心注解详解/@Provides 与 @Binds 注解详解.md

- reference 标题：@Provides 与 @Binds 注解详解
- 生成页：API-参考/Provides-与--Binds-注解详解.md（@Provides 与 @Binds 注解详解）
- 匹配分数：264
- 页面类型：other / other
- 行数：274 / 174
- 段落行数：103 / 88
- Evidence：13 / 5
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：androidinjector.java、delegatecomponentprocessor.java、bindsinstanceelementvalidator.java、bindsmethodvalidator.java、providesmethodvalidator.java、binds.java、bindsinstance.java、provides.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：androidinjector.java、delegatecomponentprocessor.java、bindsinstanceelementvalidator.java、bindsmethodvalidator.java、providesmethodvalidator.java、binds.java、bindsinstance.java、provides.java

### 核心概念/核心注解详解/核心注解详解.md

- reference 标题：核心注解详解
- 生成页：核心模块/examples.md（examples）
- 匹配分数：154
- 页面类型：other / module
- 行数：262 / 146
- 段落行数：98 / 46
- Evidence：0 / 9
- Mermaid：7 / 1
- 文件提及重合：coffeeapp.java、heatermodule.java、thermosiphon.java
- reference 关键文件未覆盖：componentprocessor.java、componentprocessingstep.java、component.java、module.java、provides.java、injectedfieldsignature.java、coffeeappcomponent.java、heater.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentprocessor.java、componentprocessingstep.java、component.java、module.java、provides.java、injectedfieldsignature.java、coffeeappcomponent.java、heater.java

### 核心概念/组件与模块系统/模块组织与管理.md

- reference 标题：模块组织与管理
- 生成页：核心模块/examples.md（examples）
- 匹配分数：144
- 页面类型：module / module
- 行数：317 / 146
- 段落行数：117 / 46
- Evidence：0 / 9
- Mermaid：7 / 1
- 文件提及重合：coffeeapp.java
- reference 关键文件未覆盖：modulevalidator.java、providesmethodvalidator.java、moduleproxies.java、module.java、provides.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：modulevalidator.java、providesmethodvalidator.java、moduleproxies.java、module.java、provides.java

### 核心概念/组件与模块系统/组件与模块系统.md

- reference 标题：组件与模块系统
- 生成页：核心模块/dagger-lint.md（dagger-lint）
- 匹配分数：100
- 页面类型：module / module
- 行数：331 / 144
- 段落行数：139 / 78
- Evidence：0 / 6
- Mermaid：9 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：componentprocessor.java、bindinggraphvalidationmodule.java、subcomponentfactorymethodvalidator.java、componentpath.java、componenthierarchyvalidator.java、componentimplementation.java、component.java、module.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentprocessor.java、bindinggraphvalidationmodule.java、subcomponentfactorymethodvalidator.java、componentpath.java、componenthierarchyvalidator.java、componentimplementation.java、component.java、module.java

### 核心概念/组件与模块系统/组件关系设计/子组件机制.md

- reference 标题：子组件机制
- 生成页：核心模块/核心概念/Inject-注解详解.md（@Inject 注解详解）
- 匹配分数：134
- 页面类型：module / module
- 行数：340 / 292
- 段落行数：144 / 121
- Evidence：23 / 16
- Mermaid：10 / 2
- 文件提及重合：usercommandsrouter.java
- reference 关键文件未覆盖：compositebindinggraphplugin.java、dependencycyclevalidator.java、subcomponentfactorymethodvalidator.java、currentimplementationsubcomponent.java、toplevelimplementationcomponent.java、subcomponent.java、simpleactivity.kt、simpleactivitycomponent.kt
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：compositebindinggraphplugin.java、dependencycyclevalidator.java、subcomponentfactorymethodvalidator.java、currentimplementationsubcomponent.java、toplevelimplementationcomponent.java、subcomponent.java、simpleactivity.kt、simpleactivitycomponent.kt

### 核心概念/组件与模块系统/组件关系设计/组件依赖关系.md

- reference 标题：组件依赖关系
- 生成页：核心模块/dagger-producers.md（dagger-producers）
- 匹配分数：98
- 页面类型：module / module
- 行数：266 / 159
- 段落行数：112 / 46
- Evidence：0 / 10
- Mermaid：5 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：componentrequirementrequestrepresentation.java、componentdependencybinding.java、componentdescriptor.java、componentrequirement.java、dependencycyclevalidator.java、bindingkind.java、componentvalidator.java、dependencymethodproducercreationexpression.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentrequirementrequestrepresentation.java、componentdependencybinding.java、componentdescriptor.java、componentrequirement.java、dependencycyclevalidator.java、bindingkind.java、componentvalidator.java、dependencymethodproducercreationexpression.java

### 核心概念/组件与模块系统/组件关系设计/组件关系设计.md

- reference 标题：组件关系设计
- 生成页：核心模块/dagger-lint.md（dagger-lint）
- 匹配分数：98
- 页面类型：module / module
- 行数：266 / 144
- 段落行数：103 / 78
- Evidence：0 / 6
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：componentdependencybinding.java、subcomponentcreatorbinding.java、subcomponentdeclaration.java、subcomponentfactorymethodvalidator.java、componentgenerator.java、componenthierarchyvalidator.java、subcomponentcreatorrequestrepresentation.java、productionsubcomponent.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentdependencybinding.java、subcomponentcreatorbinding.java、subcomponentdeclaration.java、subcomponentfactorymethodvalidator.java、componentgenerator.java、componenthierarchyvalidator.java、subcomponentcreatorrequestrepresentation.java、productionsubcomponent.java

### 核心概念/组件与模块系统/组件基础概念.md

- reference 标题：组件基础概念
- 生成页：核心模块/核心模块.md（核心模块）
- 匹配分数：122
- 页面类型：module / module
- 行数：231 / 175
- 段落行数：90 / 44
- Evidence：0 / 5
- Mermaid：7 / 1
- 文件提及重合：coffeeapp.java
- reference 关键文件未覆盖：component.java、componentdescriptor.java、componentgenerator.java、membersinjector.java、mycomponent.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：component.java、componentdescriptor.java、componentgenerator.java、membersinjector.java、mycomponent.java

### 核心概念/组件与模块系统/组件构建器与工厂.md

- reference 标题：组件构建器与工厂
- 生成页：核心模块/核心模块.md（核心模块）
- 匹配分数：106
- 页面类型：module / module
- 行数：358 / 175
- 段落行数：111 / 44
- Evidence：15 / 5
- Mermaid：5 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：bindsinstance.java、builderbindsinstanceparametertest.java、buildertest.java、component.java、componentcreatordescriptor.java、componentcreatorkind.java、componentcreatorvalidator.java、errormessages.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：bindsinstance.java、builderbindsinstanceparametertest.java、buildertest.java、component.java、componentcreatordescriptor.java、componentcreatorkind.java、componentcreatorvalidator.java、errormessages.java

### 核心概念/绑定与提供者模式.md

- reference 标题：绑定与提供者模式
- 生成页：核心模块/核心概念/绑定与提供者模式.md（绑定与提供者模式）
- 匹配分数：256
- 页面类型：other / module
- 行数：246 / 273
- 段落行数：99 / 121
- Evidence：0 / 13
- Mermaid：5 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：binds.java、bindsmethodvalidator.java、bindsoptionalof.java、contributionbinding.java、intomap.java、intoset.java、lazy.java、module.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：binds.java、bindsmethodvalidator.java、bindsoptionalof.java、contributionbinding.java、intomap.java、intoset.java、lazy.java、module.java

### 测试策略与最佳实践/Android测试.md

- reference 标题：Android测试
- 生成页：核心模块/高级特性与扩展/延迟初始化与线程安全.md（延迟初始化与线程安全）
- 匹配分数：82
- 页面类型：other / module
- 行数：248 / 270
- 段落行数：98 / 121
- Evidence：0 / 16
- Mermaid：5 / 2
- 文件提及重合：hiltandroidrule.java、hilttestapplication.java
- reference 关键文件未覆盖：coffeeappfakeheatertest.java、coffeeappfakepumptest.java、bindvalue.java、customtestapplication.java、hiltandroidtest.java、testinstallin.java、installin.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：coffeeappfakeheatertest.java、coffeeappfakepumptest.java、bindvalue.java、customtestapplication.java、hiltandroidtest.java、testinstallin.java、installin.java

### 测试策略与最佳实践/单元测试.md

- reference 标题：单元测试
- 生成页：测试基础设施/测试策略与最佳实践/单元测试.md（单元测试）
- 匹配分数：258
- 页面类型：other / other
- 行数：254 / 168
- 段落行数：108 / 82
- Evidence：15 / 5
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：doublechecktest.java、instancefactorytest.java、mapproviderfactorytest.java、setfactorytest.java、bindinggraphsubject.java、coffeeappfakeheatertest.java、testinstallin.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：doublechecktest.java、instancefactorytest.java、mapproviderfactorytest.java、setfactorytest.java、bindinggraphsubject.java、coffeeappfakeheatertest.java、testinstallin.java

### 测试策略与最佳实践/性能测试与监控.md

- reference 标题：性能测试与监控
- 生成页：测试基础设施/测试策略与最佳实践/性能测试与监控.md（性能测试与监控）
- 匹配分数：258
- 页面类型：other / other
- 行数：317 / 168
- 段落行数：125 / 82
- Evidence：0 / 5
- Mermaid：3 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：producermonitor.java、producertimingrecorder.java、producertoken.java、productioncomponentmonitor.java、productioncomponenttimingrecorder.java、timingproducermonitor.java、timingproductioncomponentmonitor.java、timingrecorders.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：producermonitor.java、producertimingrecorder.java、producertoken.java、productioncomponentmonitor.java、productioncomponenttimingrecorder.java、timingproducermonitor.java、timingproductioncomponentmonitor.java、timingrecorders.java

### 测试策略与最佳实践/测试策略与最佳实践.md

- reference 标题：测试策略与最佳实践
- 生成页：测试基础设施/测试策略与最佳实践.md（测试策略与最佳实践）
- 匹配分数：262
- 页面类型：other / other
- 行数：253 / 208
- 段落行数：103 / 88
- Evidence：15 / 6
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：bindinggraphsubject.java、coffeeappfakeheatertest.java、coffeeappfakepumptest.java、hiltandroidtest.java、uninstallmodules.java、testinstallin.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：bindinggraphsubject.java、coffeeappfakeheatertest.java、coffeeappfakepumptest.java、hiltandroidtest.java、uninstallmodules.java、testinstallin.java

### 测试策略与最佳实践/测试自动化与CI_CD.md

- reference 标题：测试自动化与CI/CD
- 生成页：配置参考/配置参考.md（配置参考）
- 匹配分数：88
- 页面类型：other / other
- 行数：230 / 175
- 段落行数：91 / 44
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：build.gradle.kt、settings.gradle.kt
- reference 关键文件未覆盖：readme.md
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：readme.md

### 测试策略与最佳实践/集成测试.md

- reference 标题：集成测试
- 生成页：测试基础设施/测试策略与最佳实践/集成测试.md（集成测试）
- 匹配分数：278
- 页面类型：other / other
- 行数：293 / 202
- 段落行数：111 / 76
- Evidence：19 / 12
- Mermaid：7 / 1
- 文件提及重合：testcomponentdata.java
- reference 关键文件未覆盖：coffeeappfakeheatertest.java、testinjector.java、bindvalue.java、customtestapplication.java、hiltandroidtest.java、uninstallmodules.java、testinstallin.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：coffeeappfakeheatertest.java、testinjector.java、bindvalue.java、customtestapplication.java、hiltandroidtest.java、uninstallmodules.java、testinstallin.java

### 示例与教程/Android应用示例.md

- reference 标题：Android应用示例
- 生成页：项目概述.md（项目概述）
- 匹配分数：74
- 页面类型：other / overview
- 行数：258 / 223
- 段落行数：100 / 51
- Evidence：0 / 6
- Mermaid：6 / 1
- 文件提及重合：readme.md、coffeeapp.java
- reference 关键文件未覆盖：coffeeappfakeheatertest.java、coffeeappfakepumptest.java、definecomponent.java、entrypoint.java、entrypoints.java、installin.java
- 结论：图表少于 reference；缺少关键文件提及：coffeeappfakeheatertest.java、coffeeappfakepumptest.java、definecomponent.java、entrypoint.java、entrypoints.java、installin.java

### 示例与教程/基础示例.md

- reference 标题：基础示例
- 生成页：核心模块/examples.md（examples）
- 匹配分数：188
- 页面类型：other / module
- 行数：262 / 146
- 段落行数：119 / 46
- Evidence：0 / 9
- Mermaid：6 / 1
- 文件提及重合：coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java、thermosiphon.java
- reference 关键文件未覆盖：electricheater.java、heater.java、pump.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：electricheater.java、heater.java、pump.java

### 示例与教程/实践教程.md

- reference 标题：实践教程
- 生成页：核心模块/examples.md（examples）
- 匹配分数：192
- 页面类型：other / module
- 行数：350 / 146
- 段落行数：159 / 46
- Evidence：0 / 9
- Mermaid：7 / 1
- 文件提及重合：coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java、thermosiphon.java
- reference 关键文件未覆盖：readme.md、binds.java、component.java、lazy.java、module.java、provides.java、reusable.java、subcomponent.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：readme.md、binds.java、component.java、lazy.java、module.java、provides.java、reusable.java、subcomponent.java

### 示例与教程/示例与教程.md

- reference 标题：示例与教程
- 生成页：核心模块/examples.md（examples）
- 匹配分数：160
- 页面类型：other / module
- 行数：300 / 146
- 段落行数：147 / 46
- Evidence：0 / 9
- Mermaid：7 / 1
- 文件提及重合：coffeeapp.java、coffeemaker.java、heatermodule.java、pumpmodule.java、thermosiphon.java
- reference 关键文件未覆盖：readme.md、androidinjection.java、daggerapplication.java、producer.java、production.java、heater.java、pump.java、definecomponent.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：readme.md、androidinjection.java、daggerapplication.java、producer.java、production.java、heater.java、pump.java、definecomponent.java

### 示例与教程/高级应用示例.md

- reference 标题：高级应用示例
- 生成页：核心模块/examples.md（examples）
- 匹配分数：164
- 页面类型：other / module
- 行数：319 / 146
- 段落行数：146 / 46
- Evidence：0 / 9
- Mermaid：6 / 1
- 文件提及重合：coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java
- reference 关键文件未覆盖：contributing.md、readme.md、producer.java、productioncomponent.java、binds.java、component.java、lazy.java、module.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：contributing.md、readme.md、producer.java、productioncomponent.java、binds.java、component.java、lazy.java、module.java

### 编译时处理机制/代码生成策略.md

- reference 标题：代码生成策略
- 生成页：编译工具链/编译时处理机制/代码生成策略.md（代码生成策略）
- 匹配分数：334
- 页面类型：topic / topic
- 行数：361 / 163
- 段落行数：159 / 38
- Evidence：0 / 15
- Mermaid：9 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：sourcefilegenerator.java、componentgenerator.java、componentcreatorimplementation.java、componentimplementation.java、factorygenerator.java、generatedimplementation.java、membersinjectorgenerator.java、modulegenerator.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：sourcefilegenerator.java、componentgenerator.java、componentcreatorimplementation.java、componentimplementation.java、factorygenerator.java、generatedimplementation.java、membersinjectorgenerator.java、modulegenerator.java

### 编译时处理机制/依赖图构建.md

- reference 标题：依赖图构建
- 生成页：编译工具链/编译时处理机制/依赖图构建.md（依赖图构建）
- 匹配分数：346
- 页面类型：topic / topic
- 行数：335 / 228
- 段落行数：164 / 38
- Evidence：0 / 17
- Mermaid：6 / 1
- 文件提及重合：binding.java、key.java
- reference 关键文件未覆盖：bindinggraphfactory.java、componentnodeimpl.java、dependencyedgeimpl.java、compositebindinggraphplugin.java、dependencycyclevalidator.java、duplicatebindingsvalidator.java、incompatiblyscopedbindingsvalidator.java、missingbindingvalidator.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：bindinggraphfactory.java、componentnodeimpl.java、dependencyedgeimpl.java、compositebindinggraphplugin.java、dependencycyclevalidator.java、duplicatebindingsvalidator.java、incompatiblyscopedbindingsvalidator.java、missingbindingvalidator.java

### 编译时处理机制/增量编译优化.md

- reference 标题：增量编译优化
- 生成页：编译工具链/编译时处理机制/增量编译优化.md（增量编译优化）
- 匹配分数：334
- 页面类型：topic / topic
- 行数：268 / 201
- 段落行数：122 / 38
- Evidence：0 / 16
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：componentprocessor.java、delegatecomponentprocessor.java、kspcomponentprocessor.java、clearablecache.java、processingenvironmentcompileroptions.java、componentvalidator.java、injectvalidator.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、kspcomponentprocessor.java、clearablecache.java、processingenvironmentcompileroptions.java、componentvalidator.java、injectvalidator.java

### 编译时处理机制/注解处理基础.md

- reference 标题：注解处理基础
- 生成页：编译工具链/编译时处理机制/注解处理基础.md（注解处理基础）
- 匹配分数：336
- 页面类型：topic / topic
- 行数：296 / 247
- 段落行数：135 / 38
- Evidence：0 / 18
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：componentprocessor.java、delegatecomponentprocessor.java、processingenvironmentcompileroptions.java、componentprocessingstep.java、injectprocessingstep.java、mapkeyprocessingstep.java、moduleprocessingstep.java、processingstepsmodule.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、processingenvironmentcompileroptions.java、componentprocessingstep.java、injectprocessingstep.java、mapkeyprocessingstep.java、moduleprocessingstep.java、processingstepsmodule.java

### 编译时处理机制/编译时处理机制.md

- reference 标题：编译时处理机制
- 生成页：编译工具链/编译时处理机制.md（编译时处理机制）
- 匹配分数：340
- 页面类型：topic / topic
- 行数：327 / 306
- 段落行数：135 / 45
- Evidence：0 / 22
- Mermaid：5 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：componentprocessor.java、delegatecomponentprocessor.java、kspcomponentprocessor.java、processingenvironmentmodule.java、validationreport.java、bindinggraphfactory.java、bindinggraphvalidationmodule.java、compositebindinggraphplugin.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、kspcomponentprocessor.java、processingenvironmentmodule.java、validationreport.java、bindinggraphfactory.java、bindinggraphvalidationmodule.java、compositebindinggraphplugin.java

### 编译时处理机制/验证与错误处理.md

- reference 标题：验证与错误处理
- 生成页：编译工具链/编译时处理机制/验证与错误处理.md（验证与错误处理）
- 匹配分数：340
- 页面类型：topic / topic
- 行数：399 / 201
- 段落行数：180 / 38
- Evidence：0 / 18
- Mermaid：9 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：validationreport.java、errormessages.java、bindinggraphplugin.java、diagnosticreporter.java、bindinggraphvalidator.java、componentvalidator.java、diagnosticreporterfactory.java、modulevalidator.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：validationreport.java、errormessages.java、bindinggraphplugin.java、diagnosticreporter.java、bindinggraphvalidator.java、componentvalidator.java、diagnosticreporterfactory.java、modulevalidator.java

### 贡献指南.md

- reference 标题：贡献指南
- 生成页：概念指南/README.md（README）
- 匹配分数：118
- 页面类型：other / other
- 行数：251 / 214
- 段落行数：71 / 63
- Evidence：0 / 13
- Mermaid：3 / 1
- 文件提及重合：contributing.md、readme.md
- reference 关键文件未覆盖：build.gradle.kt、settings.gradle.kt、libs.versions.toml
- 结论：图表少于 reference；缺少关键文件提及：build.gradle.kt、settings.gradle.kt、libs.versions.toml

### 项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：666
- 页面类型：overview / overview
- 行数：244 / 223
- 段落行数：114 / 51
- Evidence：0 / 6
- Mermaid：7 / 1
- 文件提及重合：contributing.md、readme.md
- reference 关键文件未覆盖：componentprocessor.java、delegatecomponentprocessor.java、component.java、lazy.java、module.java、subcomponent.java、definecomponent.java、settings.gradle.kt
- 结论：图表少于 reference；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、component.java、lazy.java、module.java、subcomponent.java、definecomponent.java、settings.gradle.kt

### 高级特性与扩展/SPI扩展机制.md

- reference 标题：SPI扩展机制
- 生成页：编译工具链/SPI扩展机制.md（SPI扩展机制）
- 匹配分数：338
- 页面类型：topic / topic
- 行数：393 / 241
- 段落行数：185 / 38
- Evidence：0 / 18
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：serviceloaders.java、compositebindinggraphplugin.java、diagnosticreporterfactory.java、externalbindinggraphplugins.java、validationbindinggraphplugin.java、validationbindinggraphplugins.java、bindinggraphplugin.java、diagnosticreporter.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：serviceloaders.java、compositebindinggraphplugin.java、diagnosticreporterfactory.java、externalbindinggraphplugins.java、validationbindinggraphplugin.java、validationbindinggraphplugins.java、bindinggraphplugin.java、diagnosticreporter.java

### 高级特性与扩展/多值绑定高级用法.md

- reference 标题：多值绑定高级用法
- 生成页：核心运行时/高级特性与扩展/多值绑定高级用法.md（多值绑定高级用法）
- 匹配分数：266
- 页面类型：other / other
- 行数：311 / 172
- 段落行数：133 / 59
- Evidence：21 / 10
- Mermaid：9 / 1
- 文件提及重合：setbuilder.java
- reference 关键文件未覆盖：intomap.java、intoset.java、mapfactorycreationexpression.java、mapproviderfactory.java、mapproviderfactorytest.java、multibindingdeclaration.java、multibinds.java、setbuildertest.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：intomap.java、intoset.java、mapfactorycreationexpression.java、mapproviderfactory.java、mapproviderfactorytest.java、multibindingdeclaration.java、multibinds.java、setbuildertest.java

### 高级特性与扩展/延迟初始化与线程安全.md

- reference 标题：延迟初始化与线程安全
- 生成页：核心运行时/高级特性与扩展/延迟初始化与线程安全.md（延迟初始化与线程安全）
- 匹配分数：312
- 页面类型：other / other
- 行数：283 / 172
- 段落行数：134 / 59
- Evidence：0 / 10
- Mermaid：6 / 1
- 文件提及重合：doublecheck.java、singlecheck.java
- reference 关键文件未覆盖：component.java、lazy.java、module.java、doublechecktest.java、singlechecktest.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：component.java、lazy.java、module.java、doublechecktest.java、singlechecktest.java

### 高级特性与扩展/性能监控与调试.md

- reference 标题：性能监控与调试
- 生成页：API-参考/性能监控与调试.md（性能监控与调试）
- 匹配分数：268
- 页面类型：other / other
- 行数：312 / 174
- 段落行数：151 / 88
- Evidence：16 / 5
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：clearablecache.java、compileroptions.java、validation.java、producermonitor.java、producertimingrecorder.java、producertoken.java、productioncomponentmonitor.java、productioncomponenttimingrecorder.java
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：clearablecache.java、compileroptions.java、validation.java、producermonitor.java、producertimingrecorder.java、producertoken.java、productioncomponentmonitor.java、productioncomponenttimingrecorder.java

### 高级特性与扩展/自定义扩展开发.md

- reference 标题：自定义扩展开发
- 生成页：编译工具链/编译工具链.md（编译工具链）
- 匹配分数：100
- 页面类型：other / other
- 行数：352 / 168
- 段落行数：164 / 44
- Evidence：24 / 5
- Mermaid：9 / 1
- 文件提及重合：androidprocessor.java、componentprocessor.java、serviceloaders.java
- reference 关键文件未覆盖：delegateandroidprocessor.java、delegatecomponentprocessor.java、sourcefilegenerator.java、assistedfactoryprocessingstep.java、injectprocessingstep.java、processingstepsmodule.java、validationbindinggraphplugins.java、bindinggraphplugin.java
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：delegateandroidprocessor.java、delegatecomponentprocessor.java、sourcefilegenerator.java、assistedfactoryprocessingstep.java、injectprocessingstep.java、processingstepsmodule.java、validationbindinggraphplugins.java、bindinggraphplugin.java

### 高级特性与扩展/高级特性与扩展.md

- reference 标题：高级特性与扩展
- 生成页：核心运行时/高级特性与扩展.md（高级特性与扩展）
- 匹配分数：282
- 页面类型：other / other
- 行数：377 / 184
- 段落行数：146 / 69
- Evidence：0 / 6
- Mermaid：8 / 1
- 文件提及重合：doublecheck.java、singlecheck.java
- reference 关键文件未覆盖：bindinggraphplugin.java、bindsoptionalof.java、componentprocessor.java、compositebindinggraphplugin.java、externalbindinggraphplugins.java、intomap.java、intoset.java、lazy.java
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：bindinggraphplugin.java、bindsoptionalof.java、componentprocessor.java、compositebindinggraphplugin.java、externalbindinggraphplugins.java、intomap.java、intoset.java、lazy.java

