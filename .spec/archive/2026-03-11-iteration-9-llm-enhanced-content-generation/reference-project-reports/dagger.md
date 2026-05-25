# dagger Reference 对比报告

生成页面：28 页
reference 页面：65 页
命中对比：29 页
缺失对比：36 页

## 项目结论

- reference 文档树明显更细，当前 planner 仍以总览页和少量模块页为主
- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- Mermaid/结构图表达仍然不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 核心模块/dagger-runtime.md 被 14 个 reference 页面共享映射
- 核心模块/dagger-android-support.md 被 4 个 reference 页面共享映射
- 核心模块/dagger-producers.md 被 2 个 reference 页面共享映射
- 核心模块/dagger-android-processor.md 被 2 个 reference 页面共享映射
- 核心模块/hilt-core.md 被 3 个 reference 页面共享映射

## 额外生成页面

- 核心模块/buildSrc.md (模块：buildSrc, 32 行)
- 核心模块/dagger-android-proguard-processor.md (模块：dagger-android-proguard-processor, 35 行)
- 核心模块/dagger-android.md (模块：dagger-android, 38 行)
- 核心模块/dagger-compiler.md (模块：dagger-compiler, 38 行)
- 核心模块/dagger-grpc-server-annotations.md (模块：dagger-grpc-server-annotations, 40 行)
- 核心模块/dagger-grpc-server-processor.md (模块：dagger-grpc-server-processor, 41 行)
- 核心模块/dagger-grpc-server.md (模块：dagger-grpc-server, 43 行)
- 核心模块/dagger-kythe.md (模块：dagger-kythe, 36 行)
- 核心模块/dagger-lint-android.md (模块：dagger-lint-android, 34 行)
- 核心模块/dagger-lint.md (模块：dagger-lint, 53 行)
- 核心模块/dagger-spi.md (模块：dagger-spi, 48 行)
- 核心模块/dagger-testing.md (模块：dagger-testing, 37 行)
- 核心模块/gradle.md (模块：gradle, 34 行)
- 核心模块/hilt-android-testing.md (模块：hilt-android-testing, 44 行)
- 核心模块/hilt-android.md (模块：hilt-android, 47 行)
- 核心模块/hilt-compiler.md (模块：hilt-compiler, 47 行)
- 核心模块/hilt-testing.md (模块：hilt-testing, 37 行)
- 核心模块/java.md (模块：java, 65 行)
- 核心模块/util.md (模块：util, 39 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | --- |
| API参考文档/API参考文档.md | 核心模块/dagger-runtime.md | 594/50 | 239/3 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidinjector.java、hasandroidinjector.java、cancellationpolicy.java、producer.java、producermodule.java、produces.java、production.java、productioncomponent.java |
| API参考文档/Android API.md | 核心模块/dagger-android-support.md | 398/38 | 168/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、androidinjectordescriptor.java、contributesandroidinjectorprocessingstep.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggeractivity.java |
| API参考文档/Hilt API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考文档/异步处理API.md | 核心模块/dagger-producers.md | 367/43 | 163/3 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：provisiondependencyonproducerbindingvalidator.java、dependencymethodproducercreationexpression.java、producercreationexpression.java、cancellationpolicy.java、productioncomponent.java、productionsubcomponent.java、abstractproducer.java、dependencymethodproducer.java |
| API参考文档/编译时API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考文档/运行时API.md | 核心模块/dagger-runtime.md | 388/50 | 102/3 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bindsinstance.java、membersinjector.java、subcomponent.java、assisted.java、assistedfactory.java、intomap.java、intoset.java |
| Android集成开发/@ContributesAndroidInjector注解.md | 核心模块/dagger-android-processor.md | 324/38 | 122/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：baseprocessingstep.java、xtypenames.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、androidinjectiontest.java |
| Android集成开发/Activity注入.md | 缺失 | - | - | - | 缺少对应生成页面 |
| Android集成开发/Android生命周期管理.md | 核心模块/dagger-android-support.md | 383/38 | 153/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggeractivity.java、dispatchingandroidinjector.java、hasandroidinjector.java |
| Android集成开发/Android集成开发.md | 核心模块/dagger-android-support.md | 321/38 | 158/6 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、androidinjection.java、androidinjectionkey.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java |
| Android集成开发/Fragment注入.md | 核心模块/dagger-android-support.md | 318/38 | 128/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java |
| Android集成开发/Service注入.md | 缺失 | - | - | - | 缺少对应生成页面 |
| Android集成开发/其他组件注入.md | 缺失 | - | - | - | 缺少对应生成页面 |
| Hilt框架详解/@HiltAndroidApp与注解使用.md | 核心模块/hilt-core.md | 315/46 | 90/3 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：coffeeapp.java、androidentrypoint.java、hiltandroidapp.java、generatesrootinput.java |
| Hilt框架详解/Hilt核心概念与架构.md | 系统架构.md | 295/72 | 127/8 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、readme.md、singletoncomponent.java、definecomponentnoparent.java |
| Hilt框架详解/Hilt框架详解.md | 核心模块/hilt-core.md | 328/46 | 134/3 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidentrypoint.java、hiltandroidapp.java、generatesrootinput.java、generatedcomponent.java、generatedcomponentmanager.java |
| Hilt框架详解/模块安装与依赖管理.md | 核心模块/hilt-core.md | 289/46 | 124/3 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：duplicatebindingsvalidator.java、moduleprocessingstep.java、modulevalidator.java、activitycomponent.java、definecomponentnoparent.java |
| Hilt框架详解/测试支持与模拟.md | 缺失 | - | - | - | 缺少对应生成页面 |
| Hilt框架详解/组件树结构与生命周期.md | 缺失 | - | - | - | 缺少对应生成页面 |
| Hilt框架详解/编译时处理与代码生成.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 异步处理与生产者.md | 核心模块/dagger-producers.md | 296/43 | 131/3 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cancellationpolicy.java、productioncomponent.java、productionsubcomponent.java、abstractproducer.java、dependencymethodproducer.java、producermonitor.java、productioncomponentmonitor.java、producercancellationtest.java |
| 快速开始.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 故障排除与调试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 构建与部署.md | 工作流与部署.md | 339/47 | 136/14 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：ci.yml、contributing.md、build.gradle.kt、daggerconventionplugin.kt、libs.versions.toml、settings.gradle.kt |
| 核心概念/作用域与生命周期管理.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 核心概念/依赖注入基础理论.md | 核心模块/dagger-runtime.md | 323/50 | 138/3 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：membersinjector.java、doublecheck.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java |
| 核心概念/核心概念.md | 核心模块/dagger-runtime.md | 326/50 | 128/3 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：scopes.java、componentdescriptor.java、scope.java、bindsinstance.java、coffeeapp.java、heatermodule.java、pumpmodule.java、thermosiphon.java |
| 核心概念/核心注解详解/@Component 注解详解.md | 核心模块/dagger-runtime.md | 211/50 | 88/3 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、componentannotation.java、componentgenerator.java、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、coffeeshop.java |
| 核心概念/核心注解详解/@Inject 注解详解.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 核心概念/核心注解详解/@Module 注解详解.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 核心概念/核心注解详解/@Provides 与 @Binds 注解详解.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 核心概念/核心注解详解/核心注解详解.md | 核心模块/dagger-runtime.md | 262/50 | 98/3 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、componentprocessingstep.java、injectedfieldsignature.java、coffeeapp.java、coffeeappcomponent.java、heater.java、heatermodule.java、pump.java |
| 核心概念/组件与模块系统/模块组织与管理.md | 核心模块/dagger-runtime.md | 317/50 | 117/3 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：modulevalidator.java、providesmethodvalidator.java、moduleproxies.java、coffeeapp.java |
| 核心概念/组件与模块系统/组件与模块系统.md | 核心模块/dagger-runtime.md | 331/50 | 139/3 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、bindinggraphvalidationmodule.java、subcomponentfactorymethodvalidator.java、componentpath.java、componenthierarchyvalidator.java、componentimplementation.java、subcomponent.java、accountmodule.java |
| 核心概念/组件与模块系统/组件关系设计/子组件机制.md | 核心模块/javatests.md | 340/52 | 144/3 | 10/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compositebindinggraphplugin.java、dependencycyclevalidator.java、subcomponentfactorymethodvalidator.java、currentimplementationsubcomponent.java、toplevelimplementationcomponent.java、subcomponent.java、usercommandsrouter.java、simpleactivity.kt |
| 核心概念/组件与模块系统/组件关系设计/组件依赖关系.md | 核心模块/dagger-runtime.md | 266/50 | 112/3 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentrequirementrequestrepresentation.java、componentdependencybinding.java、componentdescriptor.java、componentrequirement.java、dependencycyclevalidator.java、bindingkind.java、componentvalidator.java、dependencymethodproducercreationexpression.java |
| 核心概念/组件与模块系统/组件关系设计/组件关系设计.md | 核心模块/dagger-runtime.md | 266/50 | 103/3 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentdependencybinding.java、subcomponentcreatorbinding.java、subcomponentdeclaration.java、subcomponentfactorymethodvalidator.java、componentgenerator.java、componenthierarchyvalidator.java、subcomponentcreatorrequestrepresentation.java、productionsubcomponent.java |
| 核心概念/组件与模块系统/组件基础概念.md | 核心模块/dagger-runtime.md | 231/50 | 90/3 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：coffeeapp.java、componentdescriptor.java、componentgenerator.java、membersinjector.java、mycomponent.java |
| 核心概念/组件与模块系统/组件构建器与工厂.md | 核心模块/dagger-runtime.md | 358/50 | 111/3 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bindsinstance.java、builderbindsinstanceparametertest.java、buildertest.java、componentcreatordescriptor.java、componentcreatorkind.java、componentcreatorvalidator.java、errormessages.java、mycomponent.java |
| 核心概念/绑定与提供者模式.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/Android测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/单元测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/性能测试与监控.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/测试策略与最佳实践.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/测试自动化与CI_CD.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略与最佳实践/集成测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 示例与教程/Android应用示例.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 示例与教程/基础示例.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 示例与教程/实践教程.md | 核心模块/dagger-runtime.md | 350/50 | 159/3 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、electricheater.java、heater.java、heatermodule.java |
| 示例与教程/示例与教程.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 示例与教程/高级应用示例.md | 核心模块/dagger-runtime.md | 319/50 | 146/3 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、producer.java、productioncomponent.java、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java |
| 编译时处理机制/代码生成策略.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 编译时处理机制/依赖图构建.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 编译时处理机制/增量编译优化.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 编译时处理机制/注解处理基础.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 编译时处理机制/编译时处理机制.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 编译时处理机制/验证与错误处理.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 贡献指南.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 项目概述.md | 项目概述.md | 244/68 | 114/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、componentprocessor.java、delegatecomponentprocessor.java、component.java、lazy.java、module.java、subcomponent.java |
| 高级特性与扩展/SPI扩展机制.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 高级特性与扩展/多值绑定高级用法.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 高级特性与扩展/延迟初始化与线程安全.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 高级特性与扩展/性能监控与调试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 高级特性与扩展/自定义扩展开发.md | 核心模块/dagger-android-processor.md | 352/38 | 164/7 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、serviceloaders.java、sourcefilegenerator.java、assistedfactoryprocessingstep.java、injectprocessingstep.java、processingstepsmodule.java、validationbindinggraphplugins.java |
| 高级特性与扩展/高级特性与扩展.md | 缺失 | - | - | - | 缺少对应生成页面 |

## 逐文件详情

### API参考文档/API参考文档.md

- reference 标题：API参考文档
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：212
- 行数：594 / 50
- 段落行数：239 / 3
- Mermaid：9 / 0
- 文件提及重合：binds.java、component.java、lazy.java、mapkey.java、module.java、provides.java、reusable.java
- reference 关键文件未覆盖：androidinjector.java、hasandroidinjector.java、cancellationpolicy.java、producer.java、producermodule.java、produces.java、production.java、productioncomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidinjector.java、hasandroidinjector.java、cancellationpolicy.java、producer.java、producermodule.java、produces.java、production.java、productioncomponent.java

### API参考文档/Android API.md

- reference 标题：Android API
- 生成页：核心模块/dagger-android-support.md（模块：dagger-android-support）
- 匹配分数：136
- 行数：398 / 38
- 段落行数：168 / 6
- Mermaid：8 / 0
- 文件提及重合：androidsupportinjection.java、androidsupportinjectionmodule.java、daggerapplication.java、daggerfragment.java
- reference 关键文件未覆盖：readme.md、androidinjectordescriptor.java、contributesandroidinjectorprocessingstep.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggeractivity.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、androidinjectordescriptor.java、contributesandroidinjectorprocessingstep.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggeractivity.java

### API参考文档/Hilt API.md

- reference 标题：Hilt API
- 生成页：无
- 问题：缺少对应生成页面

### API参考文档/异步处理API.md

- reference 标题：异步处理API
- 生成页：核心模块/dagger-producers.md（模块：dagger-producers）
- 匹配分数：62
- 行数：367 / 43
- 段落行数：163 / 3
- Mermaid：6 / 0
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
- 匹配分数：78
- 行数：388 / 50
- 段落行数：102 / 3
- Mermaid：3 / 0
- 文件提及重合：binds.java、component.java、lazy.java、mapkey.java、module.java、provides.java、reusable.java
- reference 关键文件未覆盖：bindsinstance.java、membersinjector.java、subcomponent.java、assisted.java、assistedfactory.java、intomap.java、intoset.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bindsinstance.java、membersinjector.java、subcomponent.java、assisted.java、assistedfactory.java、intomap.java、intoset.java

### Android集成开发/@ContributesAndroidInjector注解.md

- reference 标题：@ContributesAndroidInjector注解
- 生成页：核心模块/dagger-android-processor.md（模块：dagger-android-processor）
- 匹配分数：132
- 行数：324 / 38
- 段落行数：122 / 7
- Mermaid：7 / 0
- 文件提及重合：androidinjectordescriptor.java、androidprocessor.java、contributesandroidinjectorprocessingstep.java、delegateandroidprocessor.java
- reference 关键文件未覆盖：baseprocessingstep.java、xtypenames.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、androidinjectiontest.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：baseprocessingstep.java、xtypenames.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、androidinjectiontest.java

### Android集成开发/Activity注入.md

- reference 标题：Activity注入
- 生成页：无
- 问题：缺少对应生成页面

### Android集成开发/Android生命周期管理.md

- reference 标题：Android生命周期管理
- 生成页：核心模块/dagger-android-support.md（模块：dagger-android-support）
- 匹配分数：108
- 行数：383 / 38
- 段落行数：153 / 6
- Mermaid：8 / 0
- 文件提及重合：daggerappcompatactivity.java、daggerapplication.java、daggerfragment.java
- reference 关键文件未覆盖：contributesandroidinjectorprocessingstep.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggeractivity.java、dispatchingandroidinjector.java、hasandroidinjector.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、daggeractivity.java、dispatchingandroidinjector.java、hasandroidinjector.java

### Android集成开发/Android集成开发.md

- reference 标题：Android集成开发
- 生成页：核心模块/dagger-android-support.md（模块：dagger-android-support）
- 匹配分数：80
- 行数：321 / 38
- 段落行数：158 / 6
- Mermaid：9 / 0
- 文件提及重合：daggerappcompatactivity.java、daggerfragment.java
- reference 关键文件未覆盖：contributesandroidinjectorprocessingstep.java、androidinjection.java、androidinjectionkey.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributesandroidinjectorprocessingstep.java、androidinjection.java、androidinjectionkey.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java

### Android集成开发/Fragment注入.md

- reference 标题：Fragment注入
- 生成页：核心模块/dagger-android-support.md（模块：dagger-android-support）
- 匹配分数：74
- 行数：318 / 38
- 段落行数：128 / 6
- Mermaid：5 / 0
- 文件提及重合：androidsupportinjection.java、daggerfragment.java
- reference 关键文件未覆盖：androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidinjection.java、androidinjectionmodule.java、androidinjector.java、contributesandroidinjector.java、dispatchingandroidinjector.java、hasandroidinjector.java

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
- 匹配分数：62
- 行数：315 / 46
- 段落行数：90 / 3
- Mermaid：4 / 0
- 文件提及重合：definecomponent.java、entrypoint.java、entrypoints.java、installin.java、package-info.java
- reference 关键文件未覆盖：coffeeapp.java、androidentrypoint.java、hiltandroidapp.java、generatesrootinput.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：coffeeapp.java、androidentrypoint.java、hiltandroidapp.java、generatesrootinput.java

### Hilt框架详解/Hilt核心概念与架构.md

- reference 标题：Hilt核心概念与架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：92
- 行数：295 / 72
- 段落行数：127 / 8
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、readme.md、singletoncomponent.java、definecomponentnoparent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：definecomponent.java、entrypoint.java、entrypoints.java、generatesrootinput.java、installin.java、readme.md、singletoncomponent.java、definecomponentnoparent.java

### Hilt框架详解/Hilt框架详解.md

- reference 标题：Hilt框架详解
- 生成页：核心模块/hilt-core.md（模块：hilt-core）
- 匹配分数：62
- 行数：328 / 46
- 段落行数：134 / 3
- Mermaid：7 / 0
- 文件提及重合：definecomponent.java、entrypoint.java、entrypoints.java、installin.java、singletoncomponent.java
- reference 关键文件未覆盖：androidentrypoint.java、hiltandroidapp.java、generatesrootinput.java、generatedcomponent.java、generatedcomponentmanager.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：androidentrypoint.java、hiltandroidapp.java、generatesrootinput.java、generatedcomponent.java、generatedcomponentmanager.java

### Hilt框架详解/模块安装与依赖管理.md

- reference 标题：模块安装与依赖管理
- 生成页：核心模块/hilt-core.md（模块：hilt-core）
- 匹配分数：120
- 行数：289 / 46
- 段落行数：124 / 3
- Mermaid：8 / 0
- 文件提及重合：definecomponent.java、installin.java、singletoncomponent.java
- reference 关键文件未覆盖：duplicatebindingsvalidator.java、moduleprocessingstep.java、modulevalidator.java、activitycomponent.java、definecomponentnoparent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：duplicatebindingsvalidator.java、moduleprocessingstep.java、modulevalidator.java、activitycomponent.java、definecomponentnoparent.java

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
- 匹配分数：60
- 行数：296 / 43
- 段落行数：131 / 3
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
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：90
- 行数：339 / 47
- 段落行数：136 / 14
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：ci.yml、contributing.md、build.gradle.kt、daggerconventionplugin.kt、libs.versions.toml、settings.gradle.kt
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：ci.yml、contributing.md、build.gradle.kt、daggerconventionplugin.kt、libs.versions.toml、settings.gradle.kt

### 核心概念/作用域与生命周期管理.md

- reference 标题：作用域与生命周期管理
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/依赖注入基础理论.md

- reference 标题：依赖注入基础理论
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：148
- 行数：323 / 50
- 段落行数：138 / 3
- Mermaid：7 / 0
- 文件提及重合：binds.java、component.java、lazy.java、module.java、provides.java
- reference 关键文件未覆盖：membersinjector.java、doublecheck.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：membersinjector.java、doublecheck.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、heatermodule.java、pumpmodule.java

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：60
- 行数：326 / 50
- 段落行数：128 / 3
- Mermaid：8 / 0
- 文件提及重合：binds.java、component.java、module.java、provides.java、reusable.java
- reference 关键文件未覆盖：scopes.java、componentdescriptor.java、scope.java、bindsinstance.java、coffeeapp.java、heatermodule.java、pumpmodule.java、thermosiphon.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：scopes.java、componentdescriptor.java、scope.java、bindsinstance.java、coffeeapp.java、heatermodule.java、pumpmodule.java、thermosiphon.java

### 核心概念/核心注解详解/@Component 注解详解.md

- reference 标题：@Component 注解详解
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：104
- 行数：211 / 50
- 段落行数：88 / 3
- Mermaid：4 / 0
- 文件提及重合：component.java、module.java
- reference 关键文件未覆盖：componentprocessor.java、componentannotation.java、componentgenerator.java、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、coffeeshop.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、componentannotation.java、componentgenerator.java、subcomponent.java、coffeeapp.java、coffeelogger.java、coffeemaker.java、coffeeshop.java

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
- 匹配分数：90
- 行数：262 / 50
- 段落行数：98 / 3
- Mermaid：7 / 0
- 文件提及重合：component.java、module.java、provides.java
- reference 关键文件未覆盖：componentprocessor.java、componentprocessingstep.java、injectedfieldsignature.java、coffeeapp.java、coffeeappcomponent.java、heater.java、heatermodule.java、pump.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、componentprocessingstep.java、injectedfieldsignature.java、coffeeapp.java、coffeeappcomponent.java、heater.java、heatermodule.java、pump.java

### 核心概念/组件与模块系统/模块组织与管理.md

- reference 标题：模块组织与管理
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：142
- 行数：317 / 50
- 段落行数：117 / 3
- Mermaid：7 / 0
- 文件提及重合：module.java、provides.java
- reference 关键文件未覆盖：modulevalidator.java、providesmethodvalidator.java、moduleproxies.java、coffeeapp.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：modulevalidator.java、providesmethodvalidator.java、moduleproxies.java、coffeeapp.java

### 核心概念/组件与模块系统/组件与模块系统.md

- reference 标题：组件与模块系统
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：106
- 行数：331 / 50
- 段落行数：139 / 3
- Mermaid：9 / 0
- 文件提及重合：component.java、module.java
- reference 关键文件未覆盖：componentprocessor.java、bindinggraphvalidationmodule.java、subcomponentfactorymethodvalidator.java、componentpath.java、componenthierarchyvalidator.java、componentimplementation.java、subcomponent.java、accountmodule.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、bindinggraphvalidationmodule.java、subcomponentfactorymethodvalidator.java、componentpath.java、componenthierarchyvalidator.java、componentimplementation.java、subcomponent.java、accountmodule.java

### 核心概念/组件与模块系统/组件关系设计/子组件机制.md

- reference 标题：子组件机制
- 生成页：核心模块/javatests.md（模块：javatests）
- 匹配分数：124
- 行数：340 / 52
- 段落行数：144 / 3
- Mermaid：10 / 0
- 文件提及重合：allcontrollersaredirectchildrenofapplication.java、componentstructurefollowscontrollerstructureapplication.java、coffeeserverwithcallscopeservice.java
- reference 关键文件未覆盖：compositebindinggraphplugin.java、dependencycyclevalidator.java、subcomponentfactorymethodvalidator.java、currentimplementationsubcomponent.java、toplevelimplementationcomponent.java、subcomponent.java、usercommandsrouter.java、simpleactivity.kt
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compositebindinggraphplugin.java、dependencycyclevalidator.java、subcomponentfactorymethodvalidator.java、currentimplementationsubcomponent.java、toplevelimplementationcomponent.java、subcomponent.java、usercommandsrouter.java、simpleactivity.kt

### 核心概念/组件与模块系统/组件关系设计/组件依赖关系.md

- reference 标题：组件依赖关系
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：96
- 行数：266 / 50
- 段落行数：112 / 3
- Mermaid：5 / 0
- 文件提及重合：component.java
- reference 关键文件未覆盖：componentrequirementrequestrepresentation.java、componentdependencybinding.java、componentdescriptor.java、componentrequirement.java、dependencycyclevalidator.java、bindingkind.java、componentvalidator.java、dependencymethodproducercreationexpression.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentrequirementrequestrepresentation.java、componentdependencybinding.java、componentdescriptor.java、componentrequirement.java、dependencycyclevalidator.java、bindingkind.java、componentvalidator.java、dependencymethodproducercreationexpression.java

### 核心概念/组件与模块系统/组件关系设计/组件关系设计.md

- reference 标题：组件关系设计
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：106
- 行数：266 / 50
- 段落行数：103 / 3
- Mermaid：6 / 0
- 文件提及重合：component.java、module.java
- reference 关键文件未覆盖：componentdependencybinding.java、subcomponentcreatorbinding.java、subcomponentdeclaration.java、subcomponentfactorymethodvalidator.java、componentgenerator.java、componenthierarchyvalidator.java、subcomponentcreatorrequestrepresentation.java、productionsubcomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentdependencybinding.java、subcomponentcreatorbinding.java、subcomponentdeclaration.java、subcomponentfactorymethodvalidator.java、componentgenerator.java、componenthierarchyvalidator.java、subcomponentcreatorrequestrepresentation.java、productionsubcomponent.java

### 核心概念/组件与模块系统/组件基础概念.md

- reference 标题：组件基础概念
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：100
- 行数：231 / 50
- 段落行数：90 / 3
- Mermaid：7 / 0
- 文件提及重合：component.java
- reference 关键文件未覆盖：coffeeapp.java、componentdescriptor.java、componentgenerator.java、membersinjector.java、mycomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：coffeeapp.java、componentdescriptor.java、componentgenerator.java、membersinjector.java、mycomponent.java

### 核心概念/组件与模块系统/组件构建器与工厂.md

- reference 标题：组件构建器与工厂
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：120
- 行数：358 / 50
- 段落行数：111 / 3
- Mermaid：5 / 0
- 文件提及重合：component.java
- reference 关键文件未覆盖：bindsinstance.java、builderbindsinstanceparametertest.java、buildertest.java、componentcreatordescriptor.java、componentcreatorkind.java、componentcreatorvalidator.java、errormessages.java、mycomponent.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bindsinstance.java、builderbindsinstanceparametertest.java、buildertest.java、componentcreatordescriptor.java、componentcreatorkind.java、componentcreatorvalidator.java、errormessages.java、mycomponent.java

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
- 匹配分数：68
- 行数：350 / 50
- 段落行数：159 / 3
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
- 生成页：核心模块/dagger-runtime.md（模块：dagger-runtime）
- 匹配分数：66
- 行数：319 / 50
- 段落行数：146 / 3
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
- 匹配分数：596
- 行数：244 / 68
- 段落行数：114 / 6
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
- 生成页：核心模块/dagger-android-processor.md（模块：dagger-android-processor）
- 匹配分数：78
- 行数：352 / 38
- 段落行数：164 / 7
- Mermaid：9 / 0
- 文件提及重合：androidprocessor.java、delegateandroidprocessor.java
- reference 关键文件未覆盖：componentprocessor.java、delegatecomponentprocessor.java、serviceloaders.java、sourcefilegenerator.java、assistedfactoryprocessingstep.java、injectprocessingstep.java、processingstepsmodule.java、validationbindinggraphplugins.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：componentprocessor.java、delegatecomponentprocessor.java、serviceloaders.java、sourcefilegenerator.java、assistedfactoryprocessingstep.java、injectprocessingstep.java、processingstepsmodule.java、validationbindinggraphplugins.java

### 高级特性与扩展/高级特性与扩展.md

- reference 标题：高级特性与扩展
- 生成页：无
- 问题：缺少对应生成页面

