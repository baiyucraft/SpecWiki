# bat Reference 对比报告

生成页面：24 页
reference 页面：79 页
命中对比：47 页
缺失对比：32 页

## 覆盖统计

- 专题页覆盖：generated 10 / reference 5
- evidence 落页：generated 16 / reference 79
- 图表达覆盖：generated 18 / reference 79
- 已规划专题类型：流程主题(8)、专题页(2)
- 高频缺失专题：无

## 项目结论

- reference 文档树明显更细，当前 planner 仍以总览页和少量模块页为主
- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- evidence block 已进入页面，但覆盖率和密度仍低于 reference
- Mermaid/结构图表达仍然不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 核心模块/src.md 被 32 个 reference 页面共享映射
- 专题/module-capability/module-dc9cc1e5ef17-library-src能力：扩展机制.md 被 3 个 reference 页面共享映射
- 工作流与部署.md 被 7 个 reference 页面共享映射
- 系统架构.md 被 2 个 reference 页面共享映射
- 专题/process/process-main-flow-流程主题：main-flow.md 被 2 个 reference 页面共享映射

## 额外生成页面

- 专题/module-capability/module-f27a20dbbb7c-plugin-assets能力：扩展机制.md (assets能力：扩展机制, 34 行)
- 专题/process/process-build_static_mappings-flow-流程主题：build_static_mappings-flow.md (流程主题：build_static_mappings flow, 54 行)
- 专题/process/process-from_str-flow-流程主题：from_str-flow.md (流程主题：from_str flow, 39 行)
- 专题/process/process-gen_man_and_comp-flow-流程主题：gen_man_and_comp-flow.md (流程主题：gen_man_and_comp flow, 29 行)
- 专题/process/process-get_acknowledgements-flow-流程主题：get_acknowledgements-flow.md (流程主题：get_acknowledgements flow, 49 行)
- 专题/process/process-run-flow-流程主题：run-flow.md (流程主题：run flow, 111 行)
- 专题/process/process-syntax_for_file_name-flow-流程主题：syntax_for_file_name-flow.md (流程主题：syntax_for_file_name flow, 85 行)
- 专题/process/process-syntax_for_file_os-flow-流程主题：syntax_for_file_os-flow.md (流程主题：syntax_for_file_os flow, 97 行)
- 核心模块/assets.md (模块：assets, 57 行)
- 核心模块/build.md (模块：build, 39 行)
- 核心模块/tests.md (模块：tests, 50 行)
- 核心模块/tests/syntax-tests.md (模块：syntax-tests, 55 行)
- 核心模块/tests/syntax-tests/highlighted.md (模块：highlighted, 52 行)
- 核心模块/tests/syntax-tests/highlighted/Requirements.txt.md (模块：Requirements.txt, 31 行)
- 核心模块/tests/syntax-tests/highlighted/nginx.md (模块：nginx, 31 行)
- 核心模块/tests/syntax-tests/source.md (模块：source, 55 行)
- 核心模块/tests/syntax-tests/source/Requirements.txt.md (模块：Requirements.txt, 31 行)
- 核心模块/tests/syntax-tests/source/nginx.md (模块：nginx, 26 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API 参考文档/API 参考文档.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考文档/内部模块 API.md | 核心模块/src.md | 359/57 | 152/11 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、config.rs、error.rs、output.rs、printer.rs |
| API 参考文档/库 API 接口.md | 核心模块/src.md | 333/57 | 134/11 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、advanced.rs、simple.rs、config.rs、pretty_printer.rs、printer.rs、test_pretty_printer.rs |
| API 参考文档/数据结构.md | 核心模块/src.md | 411/57 | 166/11 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、inputs.rs、list_syntaxes_and_themes.rs、simple.rs、config.rs、pretty_printer.rs |
| API 参考文档/配置 API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考文档/错误处理.md | 核心模块/src.md | 300/57 | 79/11 | 19/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、clap_app.rs、diff.rs、error.rs、lessopen.rs、line_range.rs、output.rs、metadata.yaml |
| 命令行接口参考/命令行接口参考.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 命令行接口参考/基本选项.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 命令行接口参考/实用工具命令.md | 核心模块/src.md | 319/57 | 229/11 | 17/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：build_assets.rs、clap_app.rs、directories.rs、cache.rs |
| 命令行接口参考/配置管理选项.md | 核心模块/src.md | 398/57 | 145/11 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.conf、clap_app.rs、config.rs、directories.rs、readme.md、theme.rs、bat-tabs.conf、bat-theme.conf |
| 命令行接口参考/高级功能选项/Git 集成功能.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 命令行接口参考/高级功能选项/主题定制系统.md | 专题/module-capability/module-dc9cc1e5ef17-library-src能力：扩展机制.md | 315/36 | 107/12 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、list_syntaxes_and_themes.rs、clap_app.rs、theme.rs、bat-theme.conf |
| 命令行接口参考/高级功能选项/分页器控制.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 命令行接口参考/高级功能选项/文本格式化选项.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 命令行接口参考/高级功能选项/行号显示控制.md | 核心模块/src.md | 245/57 | 56/11 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：clap_app.rs、printer.rs、style.rs、integration_tests.rs |
| 命令行接口参考/高级功能选项/语法映射配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 命令行接口参考/高级功能选项/高级功能选项.md | 核心模块/src.md | 360/57 | 123/11 | 18/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、config.rs、diff.rs、line_range.rs、pager.rs、printer.rs、style.rs |
| 安装与配置/Windows 特定配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安装与配置/安装与配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安装与配置/安装方法.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安装与配置/环境变量.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安装与配置/配置文件.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 工具集成与工作流/Shell 环境集成.md | 工作流与部署.md | 259/93 | 96/12 | 15/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、app.rs、completions.rs、main.rs、config.rs、pager.rs |
| 工具集成与工作流/工作流自动化.md | 工作流与部署.md | 255/93 | 107/12 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、release-checklist.md、advanced.rs、simple.rs、app.rs、main.rs、find-slow-to-highlight-files.py |
| 工具集成与工作流/工具集成与工作流.md | 工作流与部署.md | 268/93 | 120/12 | 20/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、readme-zh.md、app.rs、clap_app.rs、completions.rs、main.rs、config.rs |
| 工具集成与工作流/第三方工具集成/Git 版本控制集成.md | 工作流与部署.md | 225/93 | 108/12 | 16/1 | 5/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、app.rs、clap_app.rs、config.rs、diff.rs、50-diff.toml、50-git.toml |
| 工具集成与工作流/第三方工具集成/fzf 预览器集成.md | 核心模块/src.md | 345/57 | 203/11 | 14/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、config.rs、line_range.rs、bat-theme.conf、bat.conf、basic.rs、large-file.js、output.rs |
| 工具集成与工作流/第三方工具集成/ripgrep 搜索集成.md | 工作流与部署.md | 266/93 | 107/12 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、alternatives.md、config.rs、main.rs、lib.rs、style.rs、syntax_mapping.rs |
| 工具集成与工作流/第三方工具集成/其他工具集成.md | 核心模块/src.md | 257/57 | 86/11 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、alternatives.md、assets.md、advanced.rs、list_syntaxes_and_themes.rs、config.rs、50-bat.toml |
| 工具集成与工作流/第三方工具集成/手册页查看集成.md | 工作流与部署.md | 212/93 | 62/12 | 0/1 | 3/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、50-bat.toml |
| 工具集成与工作流/第三方工具集成/第三方工具集成.md | 工作流与部署.md | 267/93 | 72/12 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、alternatives.md、assets.md、main.rs、config.rs、less.rs、lessopen.rs |
| 工具集成与工作流/第三方工具集成/管道操作集成.md | 核心模块/src.md | 267/57 | 103/11 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、sh.rustup.rs、config.rs、output.rs、pager.rs、paging.rs、terminal.rs |
| 开发者指南/代码结构说明/代码结构说明.md | 核心模块/src.md | 354/57 | 176/11 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、config.rs、diff.rs、error.rs、line_range.rs、output.rs、printer.rs、style.rs |
| 开发者指南/代码结构说明/扩展点设计.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/代码结构说明/数据流架构.md | 系统架构.md | 375/70 | 194/23 | 0/1 | 9/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、assets.rs、config.rs、controller.rs、error.rs、input.rs、line_range.rs、output.rs |
| 开发者指南/代码结构说明/核心模块详解/Assets 资源模块.md | 核心模块/src.md | 403/57 | 155/11 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、list_syntaxes_and_themes.rs、build_assets.rs、acknowledgements.rs、lazy_theme_set.rs、serialized_syntax_set.rs、syntax_mapping.rs、theme.rs |
| 开发者指南/代码结构说明/核心模块详解/Config 配置模块.md | 核心模块/src.md | 556/57 | 373/11 | 20/1 | 11/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：clap_app.rs、config.rs、directories.rs、theme.rs、bat-theme.conf、bat.conf、integration_tests.rs、system_wide_config.rs |
| 开发者指南/代码结构说明/核心模块详解/Controller 控制器模块.md | 核心模块/src.md | 571/57 | 439/11 | 24/1 | 16/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.rs、diff.rs、line_range.rs、output.rs、paging.rs、printer.rs |
| 开发者指南/代码结构说明/核心模块详解/Input 输入模块.md | 核心模块/src.md | 317/57 | 139/11 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、inputs.rs、error.rs、lessopen.rs、preprocessor.rs |
| 开发者指南/代码结构说明/核心模块详解/Output 输出模块.md | 核心模块/src.md | 325/57 | 128/11 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、error.rs、output.rs、pager.rs、preprocessor.rs、printer.rs、terminal.rs、vscreen.rs |
| 开发者指南/代码结构说明/核心模块详解/Printer 打印器模块.md | 核心模块/src.md | 416/57 | 183/11 | 20/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：simple.rs、output.rs、preprocessor.rs、printer.rs、style.rs、terminal.rs、vscreen.rs |
| 开发者指南/代码结构说明/核心模块详解/核心模块详解.md | 核心模块/src.md | 479/57 | 232/11 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、config.rs、output.rs、printer.rs |
| 开发者指南/代码结构说明/模块组织原则.md | 核心模块/src.md | 374/57 | 278/11 | 15/1 | 10/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、config.rs、output.rs、pretty_printer.rs、style.rs、mod.rs、metadata.yaml |
| 开发者指南/代码结构说明/设计模式应用.md | 核心模块/src.md | 375/57 | 142/11 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、config.rs、preprocessor.rs、pretty_printer.rs、printer.rs、style.rs、syntax_mapping.rs |
| 开发者指南/开发环境搭建.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/开发者指南.md | 核心模块/src.md | 334/57 | 103/11 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cicd.yml、contributing.md、cargo.toml、readme.md、rustfmt.toml、config.rs、output.rs、pretty_printer.rs |
| 开发者指南/测试策略与实践/单元测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/测试策略与实践/回归测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/测试策略与实践/基准测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/测试策略与实践/快照测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/测试策略与实践/测试策略与实践.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/测试策略与实践/集成测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/调试与故障排除.md | 核心模块/src.md | 327/57 | 140/11 | 20/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、cargo.toml、config.rs、error.rs、macros.rs、integration_tests.rs、find-slow-to-highlight-files.py |
| 开发者指南/贡献流程与规范.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 快速开始.md | 核心模块/src.md | 194/57 | 68/11 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、simple.rs、clap_app.rs、config.rs、bat-theme.conf、bat.conf |
| 故障排除与常见问题.md | 核心模块/src.md | 300/57 | 99/11 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、cargo.toml、readme.md、config.rs、error.rs、printer.rs |
| 核心功能详解/Git 集成功能.md | 核心模块/src.md | 257/57 | 103/11 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、clap_app.rs、config.rs、diff.rs、line_range.rs、50-git.toml、ignored_suffixes.rs |
| 核心功能详解/主题与样式系统.md | 专题/module-capability/module-dc9cc1e5ef17-library-src能力：扩展机制.md | 447/36 | 191/12 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：theme_preview.rs、lazy_theme_set.rs、config.rs、printer.rs、style.rs、terminal.rs、theme.rs、vscreen.rs |
| 核心功能详解/分页与输出控制.md | 核心模块/src.md | 374/57 | 154/11 | 20/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：clap_app.rs、config.rs、less.rs、nonprintable_notation.rs、output.rs、pager.rs、paging.rs、terminal.rs |
| 核心功能详解/核心功能详解.md | 核心模块/src.md | 323/57 | 136/11 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、config.rs、diff.rs、output.rs、pager.rs、preprocessor.rs、pretty_printer.rs、printer.rs |
| 核心功能详解/语法高亮系统/主题系统集成.md | 专题/process/process-main-flow-流程主题：main-flow.md | 400/97 | 154/72 | 0/1 | 8/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：theme_preview.rs、list_syntaxes_and_themes.rs、assets.rs、build_assets.rs、lazy_theme_set.rs、terminal.rs、theme.rs |
| 核心功能详解/语法高亮系统/性能优化策略.md | 核心模块/src.md | 279/57 | 97/11 | 14/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、lazy_theme_set.rs、serialized_syntax_set.rs、config.rs、pretty_printer.rs、jquery.js |
| 核心功能详解/语法高亮系统/语法文件管理.md | 核心模块/src.md | 357/57 | 119/11 | 17/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、build_assets.rs、lazy_theme_set.rs、serialized_syntax_set.rs、syntax_mapping.rs、builtin.rs、metadata.yaml |
| 核心功能详解/语法高亮系统/语法高亮系统.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心功能详解/语法高亮系统/语言识别机制.md | 专题/module-capability/module-dc9cc1e5ef17-library-src能力：扩展机制.md | 270/36 | 109/12 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：syntax_mapping.rs、builtin.rs、50-bat.toml、50-cpp.toml、50-diff.toml、50-git.toml、50-json.toml、50-markdown.toml |
| 测试与质量保证/单元测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试与质量保证/回归测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试与质量保证/快照测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试与质量保证/性能基准测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试与质量保证/测试与质量保证.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试与质量保证/质量保证流程.md | 系统架构.md | 334/70 | 128/23 | 0/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：audit.toml、.codecov.yml、dependabot.yml、cicd.yml、contributing.md、cargo.toml、security.md、github-actions.rs |
| 测试与质量保证/集成测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 自定义与扩展/主题开发指南.md | 专题/process/process-main-flow-流程主题：main-flow.md | 383/97 | 144/72 | 22/1 | 7/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、theme_preview.rs、assets.rs、build_assets.rs、lazy_theme_set.rs、theme.rs、integration_tests.rs |
| 自定义与扩展/自定义与扩展.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 自定义与扩展/语法扩展开发.md | 核心模块/src.md | 244/57 | 102/11 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、syntax_mapping.rs、builtin.rs、50-bat.toml |
| 自定义与扩展/资源管理与打包.md | 核心模块/src.md | 336/57 | 134/11 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、build_assets.rs、acknowledgements.rs、lazy_theme_set.rs、serialized_syntax_set.rs、metadata.yaml |
| 自定义与扩展/配置系统定制.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 贡献指南.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 项目概述.md | 项目概述.md | 224/80 | 78/17 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、cargo.toml、readme.md、alternatives.md、config.rs、controller.rs、input.rs、output.rs |

## 逐文件详情

### API 参考文档/API 参考文档.md

- reference 标题：API 参考文档
- 生成页：无
- 问题：缺少对应生成页面

### API 参考文档/内部模块 API.md

- reference 标题：内部模块 API
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：208
- 页面类型：module / module
- 行数：359 / 57
- 段落行数：152 / 11
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：assets.rs、controller.rs、input.rs、lib.rs
- reference 关键文件未覆盖：cargo.toml、config.rs、error.rs、output.rs、printer.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、config.rs、error.rs、output.rs、printer.rs

### API 参考文档/库 API 接口.md

- reference 标题：库 API 接口
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：106
- 页面类型：other / module
- 行数：333 / 57
- 段落行数：134 / 11
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：controller.rs、input.rs、lib.rs
- reference 关键文件未覆盖：cargo.toml、advanced.rs、simple.rs、config.rs、pretty_printer.rs、printer.rs、test_pretty_printer.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、advanced.rs、simple.rs、config.rs、pretty_printer.rs、printer.rs、test_pretty_printer.rs

### API 参考文档/数据结构.md

- reference 标题：数据结构
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：74
- 页面类型：other / module
- 行数：411 / 57
- 段落行数：166 / 11
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：input.rs、lib.rs
- reference 关键文件未覆盖：cargo.toml、inputs.rs、list_syntaxes_and_themes.rs、simple.rs、config.rs、pretty_printer.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、inputs.rs、list_syntaxes_and_themes.rs、simple.rs、config.rs、pretty_printer.rs

### API 参考文档/配置 API.md

- reference 标题：配置 API
- 生成页：无
- 问题：缺少对应生成页面

### API 参考文档/错误处理.md

- reference 标题：错误处理
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：180
- 页面类型：other / module
- 行数：300 / 57
- 段落行数：79 / 11
- Evidence：19 / 1
- Mermaid：4 / 1
- 文件提及重合：assets.rs、app.rs、main.rs、controller.rs、input.rs、lib.rs
- reference 关键文件未覆盖：cargo.toml、clap_app.rs、diff.rs、error.rs、lessopen.rs、line_range.rs、output.rs、metadata.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、clap_app.rs、diff.rs、error.rs、lessopen.rs、line_range.rs、output.rs、metadata.yaml

### 命令行接口参考/命令行接口参考.md

- reference 标题：命令行接口参考
- 生成页：无
- 问题：缺少对应生成页面

### 命令行接口参考/基本选项.md

- reference 标题：基本选项
- 生成页：无
- 问题：缺少对应生成页面

### 命令行接口参考/实用工具命令.md

- reference 标题：实用工具命令
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：106
- 页面类型：other / module
- 行数：319 / 57
- 段落行数：229 / 11
- Evidence：17 / 1
- Mermaid：9 / 1
- 文件提及重合：assets.rs、assets_metadata.rs、main.rs、app.rs
- reference 关键文件未覆盖：build_assets.rs、clap_app.rs、directories.rs、cache.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：build_assets.rs、clap_app.rs、directories.rs、cache.rs

### 命令行接口参考/配置管理选项.md

- reference 标题：配置管理选项
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：98
- 页面类型：other / module
- 行数：398 / 57
- 段落行数：145 / 11
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：assets.rs、assets_metadata.rs、app.rs
- reference 关键文件未覆盖：.conf、clap_app.rs、config.rs、directories.rs、readme.md、theme.rs、bat-tabs.conf、bat-theme.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.conf、clap_app.rs、config.rs、directories.rs、readme.md、theme.rs、bat-tabs.conf、bat-theme.conf

### 命令行接口参考/高级功能选项/Git 集成功能.md

- reference 标题：Git 集成功能
- 生成页：无
- 问题：缺少对应生成页面

### 命令行接口参考/高级功能选项/主题定制系统.md

- reference 标题：主题定制系统
- 生成页：专题/module-capability/module-dc9cc1e5ef17-library-src能力：扩展机制.md（src能力：扩展机制）
- 匹配分数：116
- 页面类型：topic / topic
- 行数：315 / 36
- 段落行数：107 / 12
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：app.rs
- reference 关键文件未覆盖：readme.md、list_syntaxes_and_themes.rs、clap_app.rs、theme.rs、bat-theme.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、list_syntaxes_and_themes.rs、clap_app.rs、theme.rs、bat-theme.conf

### 命令行接口参考/高级功能选项/分页器控制.md

- reference 标题：分页器控制
- 生成页：无
- 问题：缺少对应生成页面

### 命令行接口参考/高级功能选项/文本格式化选项.md

- reference 标题：文本格式化选项
- 生成页：无
- 问题：缺少对应生成页面

### 命令行接口参考/高级功能选项/行号显示控制.md

- reference 标题：行号显示控制
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：62
- 页面类型：other / module
- 行数：245 / 57
- 段落行数：56 / 11
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：app.rs、decorations.rs
- reference 关键文件未覆盖：clap_app.rs、printer.rs、style.rs、integration_tests.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：clap_app.rs、printer.rs、style.rs、integration_tests.rs

### 命令行接口参考/高级功能选项/语法映射配置.md

- reference 标题：语法映射配置
- 生成页：无
- 问题：缺少对应生成页面

### 命令行接口参考/高级功能选项/高级功能选项.md

- reference 标题：高级功能选项
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：94
- 页面类型：other / module
- 行数：360 / 57
- 段落行数：123 / 11
- Evidence：18 / 1
- Mermaid：8 / 1
- 文件提及重合：main.rs、controller.rs、decorations.rs
- reference 关键文件未覆盖：cargo.toml、readme.md、config.rs、diff.rs、line_range.rs、pager.rs、printer.rs、style.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、config.rs、diff.rs、line_range.rs、pager.rs、printer.rs、style.rs

### 安装与配置/Windows 特定配置.md

- reference 标题：Windows 特定配置
- 生成页：无
- 问题：缺少对应生成页面

### 安装与配置/安装与配置.md

- reference 标题：安装与配置
- 生成页：无
- 问题：缺少对应生成页面

### 安装与配置/安装方法.md

- reference 标题：安装方法
- 生成页：无
- 问题：缺少对应生成页面

### 安装与配置/环境变量.md

- reference 标题：环境变量
- 生成页：无
- 问题：缺少对应生成页面

### 安装与配置/配置文件.md

- reference 标题：配置文件
- 生成页：无
- 问题：缺少对应生成页面

### 工具集成与工作流/Shell 环境集成.md

- reference 标题：Shell 环境集成
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：92
- 页面类型：workflow / workflow
- 行数：259 / 93
- 段落行数：96 / 12
- Evidence：15 / 1
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：cargo.toml、readme.md、app.rs、completions.rs、main.rs、config.rs、pager.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、app.rs、completions.rs、main.rs、config.rs、pager.rs

### 工具集成与工作流/工作流自动化.md

- reference 标题：工作流自动化
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：104
- 页面类型：workflow / workflow
- 行数：255 / 93
- 段落行数：107 / 12
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：cargo.toml、readme.md、release-checklist.md、advanced.rs、simple.rs、app.rs、main.rs、find-slow-to-highlight-files.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、release-checklist.md、advanced.rs、simple.rs、app.rs、main.rs、find-slow-to-highlight-files.py

### 工具集成与工作流/工具集成与工作流.md

- reference 标题：工具集成与工作流
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：100
- 页面类型：workflow / workflow
- 行数：268 / 93
- 段落行数：120 / 12
- Evidence：20 / 1
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：cargo.toml、readme.md、readme-zh.md、app.rs、clap_app.rs、completions.rs、main.rs、config.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、readme-zh.md、app.rs、clap_app.rs、completions.rs、main.rs、config.rs

### 工具集成与工作流/第三方工具集成/Git 版本控制集成.md

- reference 标题：Git 版本控制集成
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：98
- 页面类型：workflow / workflow
- 行数：225 / 93
- 段落行数：108 / 12
- Evidence：16 / 1
- Mermaid：5 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：cargo.toml、readme.md、app.rs、clap_app.rs、config.rs、diff.rs、50-diff.toml、50-git.toml
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、app.rs、clap_app.rs、config.rs、diff.rs、50-diff.toml、50-git.toml

### 工具集成与工作流/第三方工具集成/fzf 预览器集成.md

- reference 标题：fzf 预览器集成
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：132
- 页面类型：workflow / module
- 行数：345 / 57
- 段落行数：203 / 11
- Evidence：14 / 1
- Mermaid：6 / 1
- 文件提及重合：main.rs、assets.rs、controller.rs、input.rs、app.rs
- reference 关键文件未覆盖：readme.md、config.rs、line_range.rs、bat-theme.conf、bat.conf、basic.rs、large-file.js、output.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、config.rs、line_range.rs、bat-theme.conf、bat.conf、basic.rs、large-file.js、output.rs

### 工具集成与工作流/第三方工具集成/ripgrep 搜索集成.md

- reference 标题：ripgrep 搜索集成
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：100
- 页面类型：workflow / workflow
- 行数：266 / 93
- 段落行数：107 / 12
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：cargo.toml、readme.md、alternatives.md、config.rs、main.rs、lib.rs、style.rs、syntax_mapping.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、alternatives.md、config.rs、main.rs、lib.rs、style.rs、syntax_mapping.rs

### 工具集成与工作流/第三方工具集成/其他工具集成.md

- reference 标题：其他工具集成
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：122
- 页面类型：workflow / module
- 行数：257 / 57
- 段落行数：86 / 11
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：main.rs、lib.rs、assets.rs、controller.rs
- reference 关键文件未覆盖：cargo.toml、readme.md、alternatives.md、assets.md、advanced.rs、list_syntaxes_and_themes.rs、config.rs、50-bat.toml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、alternatives.md、assets.md、advanced.rs、list_syntaxes_and_themes.rs、config.rs、50-bat.toml

### 工具集成与工作流/第三方工具集成/手册页查看集成.md

- reference 标题：手册页查看集成
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：84
- 页面类型：workflow / workflow
- 行数：212 / 93
- 段落行数：62 / 12
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、50-bat.toml
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、50-bat.toml

### 工具集成与工作流/第三方工具集成/第三方工具集成.md

- reference 标题：第三方工具集成
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：88
- 页面类型：workflow / workflow
- 行数：267 / 93
- 段落行数：72 / 12
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：cargo.toml、readme.md、alternatives.md、assets.md、main.rs、config.rs、less.rs、lessopen.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、alternatives.md、assets.md、main.rs、config.rs、less.rs、lessopen.rs

### 工具集成与工作流/第三方工具集成/管道操作集成.md

- reference 标题：管道操作集成
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：126
- 页面类型：workflow / module
- 行数：267 / 57
- 段落行数：103 / 11
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：app.rs、main.rs、controller.rs、input.rs
- reference 关键文件未覆盖：cargo.toml、readme.md、sh.rustup.rs、config.rs、output.rs、pager.rs、paging.rs、terminal.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、sh.rustup.rs、config.rs、output.rs、pager.rs、paging.rs、terminal.rs

### 开发者指南/代码结构说明/代码结构说明.md

- reference 标题：代码结构说明
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：166
- 页面类型：other / module
- 行数：354 / 57
- 段落行数：176 / 11
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：assets.rs、main.rs、controller.rs、decorations.rs、input.rs、lib.rs
- reference 关键文件未覆盖：cargo.toml、config.rs、diff.rs、error.rs、line_range.rs、output.rs、printer.rs、style.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、config.rs、diff.rs、error.rs、line_range.rs、output.rs、printer.rs、style.rs

### 开发者指南/代码结构说明/扩展点设计.md

- reference 标题：扩展点设计
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/代码结构说明/数据流架构.md

- reference 标题：数据流架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：128
- 页面类型：architecture / architecture
- 行数：375 / 70
- 段落行数：194 / 23
- Evidence：0 / 1
- Mermaid：9 / 2
- 文件提及重合：lib.rs
- reference 关键文件未覆盖：cargo.toml、assets.rs、config.rs、controller.rs、error.rs、input.rs、line_range.rs、output.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、assets.rs、config.rs、controller.rs、error.rs、input.rs、line_range.rs、output.rs

### 开发者指南/代码结构说明/核心模块详解/Assets 资源模块.md

- reference 标题：Assets 资源模块
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：148
- 页面类型：module / module
- 行数：403 / 57
- 段落行数：155 / 11
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：assets.rs、assets_metadata.rs
- reference 关键文件未覆盖：cargo.toml、list_syntaxes_and_themes.rs、build_assets.rs、acknowledgements.rs、lazy_theme_set.rs、serialized_syntax_set.rs、syntax_mapping.rs、theme.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、list_syntaxes_and_themes.rs、build_assets.rs、acknowledgements.rs、lazy_theme_set.rs、serialized_syntax_set.rs、syntax_mapping.rs、theme.rs

### 开发者指南/代码结构说明/核心模块详解/Config 配置模块.md

- reference 标题：Config 配置模块
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：122
- 页面类型：module / module
- 行数：556 / 57
- 段落行数：373 / 11
- Evidence：20 / 1
- Mermaid：11 / 1
- 文件提及重合：app.rs
- reference 关键文件未覆盖：clap_app.rs、config.rs、directories.rs、theme.rs、bat-theme.conf、bat.conf、integration_tests.rs、system_wide_config.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：clap_app.rs、config.rs、directories.rs、theme.rs、bat-theme.conf、bat.conf、integration_tests.rs、system_wide_config.rs

### 开发者指南/代码结构说明/核心模块详解/Controller 控制器模块.md

- reference 标题：Controller 控制器模块
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：154
- 页面类型：module / module
- 行数：571 / 57
- 段落行数：439 / 11
- Evidence：24 / 1
- Mermaid：16 / 1
- 文件提及重合：main.rs、controller.rs、input.rs、lib.rs
- reference 关键文件未覆盖：config.rs、diff.rs、line_range.rs、output.rs、paging.rs、printer.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.rs、diff.rs、line_range.rs、output.rs、paging.rs、printer.rs

### 开发者指南/代码结构说明/核心模块详解/Input 输入模块.md

- reference 标题：Input 输入模块
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：118
- 页面类型：module / module
- 行数：317 / 57
- 段落行数：139 / 11
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：input.rs
- reference 关键文件未覆盖：cargo.toml、inputs.rs、error.rs、lessopen.rs、preprocessor.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、inputs.rs、error.rs、lessopen.rs、preprocessor.rs

### 开发者指南/代码结构说明/核心模块详解/Output 输出模块.md

- reference 标题：Output 输出模块
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：120
- 页面类型：module / module
- 行数：325 / 57
- 段落行数：128 / 11
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：controller.rs
- reference 关键文件未覆盖：cargo.toml、error.rs、output.rs、pager.rs、preprocessor.rs、printer.rs、terminal.rs、vscreen.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、error.rs、output.rs、pager.rs、preprocessor.rs、printer.rs、terminal.rs、vscreen.rs

### 开发者指南/代码结构说明/核心模块详解/Printer 打印器模块.md

- reference 标题：Printer 打印器模块
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：152
- 页面类型：module / module
- 行数：416 / 57
- 段落行数：183 / 11
- Evidence：20 / 1
- Mermaid：8 / 1
- 文件提及重合：controller.rs、decorations.rs
- reference 关键文件未覆盖：simple.rs、output.rs、preprocessor.rs、printer.rs、style.rs、terminal.rs、vscreen.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：simple.rs、output.rs、preprocessor.rs、printer.rs、style.rs、terminal.rs、vscreen.rs

### 开发者指南/代码结构说明/核心模块详解/核心模块详解.md

- reference 标题：核心模块详解
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：236
- 页面类型：module / module
- 行数：479 / 57
- 段落行数：232 / 11
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：assets.rs、main.rs、controller.rs、input.rs、lib.rs
- reference 关键文件未覆盖：cargo.toml、config.rs、output.rs、printer.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、config.rs、output.rs、printer.rs

### 开发者指南/代码结构说明/模块组织原则.md

- reference 标题：模块组织原则
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：224
- 页面类型：module / module
- 行数：374 / 57
- 段落行数：278 / 11
- Evidence：15 / 1
- Mermaid：10 / 1
- 文件提及重合：assets.rs、main.rs、controller.rs、input.rs、lib.rs
- reference 关键文件未覆盖：cargo.toml、config.rs、output.rs、pretty_printer.rs、style.rs、mod.rs、metadata.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、config.rs、output.rs、pretty_printer.rs、style.rs、mod.rs、metadata.yaml

### 开发者指南/代码结构说明/设计模式应用.md

- reference 标题：设计模式应用
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：154
- 页面类型：other / module
- 行数：375 / 57
- 段落行数：142 / 11
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：assets.rs、main.rs、controller.rs、decorations.rs、lib.rs
- reference 关键文件未覆盖：cargo.toml、config.rs、preprocessor.rs、pretty_printer.rs、printer.rs、style.rs、syntax_mapping.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、config.rs、preprocessor.rs、pretty_printer.rs、printer.rs、style.rs、syntax_mapping.rs

### 开发者指南/开发环境搭建.md

- reference 标题：开发环境搭建
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/开发者指南.md

- reference 标题：开发者指南
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：130
- 页面类型：other / module
- 行数：334 / 57
- 段落行数：103 / 11
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：main.rs、controller.rs、input.rs、lib.rs
- reference 关键文件未覆盖：cicd.yml、contributing.md、cargo.toml、readme.md、rustfmt.toml、config.rs、output.rs、pretty_printer.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cicd.yml、contributing.md、cargo.toml、readme.md、rustfmt.toml、config.rs、output.rs、pretty_printer.rs

### 开发者指南/测试策略与实践/单元测试.md

- reference 标题：单元测试
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/测试策略与实践/回归测试.md

- reference 标题：回归测试
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/测试策略与实践/基准测试.md

- reference 标题：基准测试
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/测试策略与实践/快照测试.md

- reference 标题：快照测试
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/测试策略与实践/测试策略与实践.md

- reference 标题：测试策略与实践
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/测试策略与实践/集成测试.md

- reference 标题：集成测试
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/调试与故障排除.md

- reference 标题：调试与故障排除
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：154
- 页面类型：other / module
- 行数：327 / 57
- 段落行数：140 / 11
- Evidence：20 / 1
- Mermaid：8 / 1
- 文件提及重合：assets.rs、app.rs、main.rs、controller.rs、lib.rs
- reference 关键文件未覆盖：contributing.md、cargo.toml、config.rs、error.rs、macros.rs、integration_tests.rs、find-slow-to-highlight-files.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、cargo.toml、config.rs、error.rs、macros.rs、integration_tests.rs、find-slow-to-highlight-files.py

### 开发者指南/贡献流程与规范.md

- reference 标题：贡献流程与规范
- 生成页：无
- 问题：缺少对应生成页面

### 快速开始.md

- reference 标题：快速开始
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：64
- 页面类型：other / module
- 行数：194 / 57
- 段落行数：68 / 11
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：app.rs、main.rs
- reference 关键文件未覆盖：cargo.toml、readme.md、simple.rs、clap_app.rs、config.rs、bat-theme.conf、bat.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、simple.rs、clap_app.rs、config.rs、bat-theme.conf、bat.conf

### 故障排除与常见问题.md

- reference 标题：故障排除与常见问题
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：126
- 页面类型：other / module
- 行数：300 / 57
- 段落行数：99 / 11
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：app.rs、main.rs、lib.rs、controller.rs
- reference 关键文件未覆盖：contributing.md、cargo.toml、readme.md、config.rs、error.rs、printer.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、cargo.toml、readme.md、config.rs、error.rs、printer.rs

### 核心功能详解/Git 集成功能.md

- reference 标题：Git 集成功能
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：68
- 页面类型：other / module
- 行数：257 / 57
- 段落行数：103 / 11
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：controller.rs、decorations.rs
- reference 关键文件未覆盖：cargo.toml、clap_app.rs、config.rs、diff.rs、line_range.rs、50-git.toml、ignored_suffixes.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、clap_app.rs、config.rs、diff.rs、line_range.rs、50-git.toml、ignored_suffixes.rs

### 核心功能详解/主题与样式系统.md

- reference 标题：主题与样式系统
- 生成页：专题/module-capability/module-dc9cc1e5ef17-library-src能力：扩展机制.md（src能力：扩展机制）
- 匹配分数：146
- 页面类型：topic / topic
- 行数：447 / 36
- 段落行数：191 / 12
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：assets.rs、decorations.rs
- reference 关键文件未覆盖：theme_preview.rs、lazy_theme_set.rs、config.rs、printer.rs、style.rs、terminal.rs、theme.rs、vscreen.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：theme_preview.rs、lazy_theme_set.rs、config.rs、printer.rs、style.rs、terminal.rs、theme.rs、vscreen.rs

### 核心功能详解/分页与输出控制.md

- reference 标题：分页与输出控制
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：126
- 页面类型：other / module
- 行数：374 / 57
- 段落行数：154 / 11
- Evidence：20 / 1
- Mermaid：8 / 1
- 文件提及重合：app.rs、main.rs、controller.rs、input.rs
- reference 关键文件未覆盖：clap_app.rs、config.rs、less.rs、nonprintable_notation.rs、output.rs、pager.rs、paging.rs、terminal.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：clap_app.rs、config.rs、less.rs、nonprintable_notation.rs、output.rs、pager.rs、paging.rs、terminal.rs

### 核心功能详解/核心功能详解.md

- reference 标题：核心功能详解
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：128
- 页面类型：other / module
- 行数：323 / 57
- 段落行数：136 / 11
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：assets.rs、main.rs、controller.rs、lib.rs
- reference 关键文件未覆盖：cargo.toml、config.rs、diff.rs、output.rs、pager.rs、preprocessor.rs、pretty_printer.rs、printer.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、config.rs、diff.rs、output.rs、pager.rs、preprocessor.rs、pretty_printer.rs、printer.rs

### 核心功能详解/语法高亮系统/主题系统集成.md

- reference 标题：主题系统集成
- 生成页：专题/process/process-main-flow-流程主题：main-flow.md（流程主题：main flow）
- 匹配分数：122
- 页面类型：topic / topic
- 行数：400 / 97
- 段落行数：154 / 72
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：main.rs
- reference 关键文件未覆盖：theme_preview.rs、list_syntaxes_and_themes.rs、assets.rs、build_assets.rs、lazy_theme_set.rs、terminal.rs、theme.rs
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：theme_preview.rs、list_syntaxes_and_themes.rs、assets.rs、build_assets.rs、lazy_theme_set.rs、terminal.rs、theme.rs

### 核心功能详解/语法高亮系统/性能优化策略.md

- reference 标题：性能优化策略
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：122
- 页面类型：other / module
- 行数：279 / 57
- 段落行数：97 / 11
- Evidence：14 / 1
- Mermaid：5 / 1
- 文件提及重合：assets.rs、main.rs、controller.rs、input.rs
- reference 关键文件未覆盖：cargo.toml、lazy_theme_set.rs、serialized_syntax_set.rs、config.rs、pretty_printer.rs、jquery.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、lazy_theme_set.rs、serialized_syntax_set.rs、config.rs、pretty_printer.rs、jquery.js

### 核心功能详解/语法高亮系统/语法文件管理.md

- reference 标题：语法文件管理
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：96
- 页面类型：other / module
- 行数：357 / 57
- 段落行数：119 / 11
- Evidence：17 / 1
- Mermaid：6 / 1
- 文件提及重合：assets.rs、assets_metadata.rs、main.rs
- reference 关键文件未覆盖：cargo.toml、build_assets.rs、lazy_theme_set.rs、serialized_syntax_set.rs、syntax_mapping.rs、builtin.rs、metadata.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、build_assets.rs、lazy_theme_set.rs、serialized_syntax_set.rs、syntax_mapping.rs、builtin.rs、metadata.yaml

### 核心功能详解/语法高亮系统/语法高亮系统.md

- reference 标题：语法高亮系统
- 生成页：无
- 问题：缺少对应生成页面

### 核心功能详解/语法高亮系统/语言识别机制.md

- reference 标题：语言识别机制
- 生成页：专题/module-capability/module-dc9cc1e5ef17-library-src能力：扩展机制.md（src能力：扩展机制）
- 匹配分数：118
- 页面类型：topic / topic
- 行数：270 / 36
- 段落行数：109 / 12
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：assets.rs
- reference 关键文件未覆盖：syntax_mapping.rs、builtin.rs、50-bat.toml、50-cpp.toml、50-diff.toml、50-git.toml、50-json.toml、50-markdown.toml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：syntax_mapping.rs、builtin.rs、50-bat.toml、50-cpp.toml、50-diff.toml、50-git.toml、50-json.toml、50-markdown.toml

### 测试与质量保证/单元测试.md

- reference 标题：单元测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试与质量保证/回归测试.md

- reference 标题：回归测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试与质量保证/快照测试.md

- reference 标题：快照测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试与质量保证/性能基准测试.md

- reference 标题：性能基准测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试与质量保证/测试与质量保证.md

- reference 标题：测试与质量保证
- 生成页：无
- 问题：缺少对应生成页面

### 测试与质量保证/质量保证流程.md

- reference 标题：质量保证流程
- 生成页：系统架构.md（系统架构）
- 匹配分数：70
- 页面类型：other / architecture
- 行数：334 / 70
- 段落行数：128 / 23
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：main.rs、lib.rs
- reference 关键文件未覆盖：audit.toml、.codecov.yml、dependabot.yml、cicd.yml、contributing.md、cargo.toml、security.md、github-actions.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：audit.toml、.codecov.yml、dependabot.yml、cicd.yml、contributing.md、cargo.toml、security.md、github-actions.rs

### 测试与质量保证/集成测试.md

- reference 标题：集成测试
- 生成页：无
- 问题：缺少对应生成页面

### 自定义与扩展/主题开发指南.md

- reference 标题：主题开发指南
- 生成页：专题/process/process-main-flow-流程主题：main-flow.md（流程主题：main flow）
- 匹配分数：126
- 页面类型：topic / topic
- 行数：383 / 97
- 段落行数：144 / 72
- Evidence：22 / 1
- Mermaid：7 / 1
- 文件提及重合：main.rs
- reference 关键文件未覆盖：readme.md、theme_preview.rs、assets.rs、build_assets.rs、lazy_theme_set.rs、theme.rs、integration_tests.rs
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、theme_preview.rs、assets.rs、build_assets.rs、lazy_theme_set.rs、theme.rs、integration_tests.rs

### 自定义与扩展/自定义与扩展.md

- reference 标题：自定义与扩展
- 生成页：无
- 问题：缺少对应生成页面

### 自定义与扩展/语法扩展开发.md

- reference 标题：语法扩展开发
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：124
- 页面类型：other / module
- 行数：244 / 57
- 段落行数：102 / 11
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：assets.rs、lib.rs、main.rs、controller.rs
- reference 关键文件未覆盖：cargo.toml、readme.md、syntax_mapping.rs、builtin.rs、50-bat.toml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、readme.md、syntax_mapping.rs、builtin.rs、50-bat.toml

### 自定义与扩展/资源管理与打包.md

- reference 标题：资源管理与打包
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：98
- 页面类型：other / module
- 行数：336 / 57
- 段落行数：134 / 11
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：main.rs、assets.rs、assets_metadata.rs
- reference 关键文件未覆盖：cargo.toml、build_assets.rs、acknowledgements.rs、lazy_theme_set.rs、serialized_syntax_set.rs、metadata.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、build_assets.rs、acknowledgements.rs、lazy_theme_set.rs、serialized_syntax_set.rs、metadata.yaml

### 自定义与扩展/配置系统定制.md

- reference 标题：配置系统定制
- 生成页：无
- 问题：缺少对应生成页面

### 贡献指南.md

- reference 标题：贡献指南
- 生成页：无
- 问题：缺少对应生成页面

### 项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：634
- 页面类型：overview / overview
- 行数：224 / 80
- 段落行数：78 / 17
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：main.rs、lib.rs
- reference 关键文件未覆盖：contributing.md、cargo.toml、readme.md、alternatives.md、config.rs、controller.rs、input.rs、output.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、cargo.toml、readme.md、alternatives.md、config.rs、controller.rs、input.rs、output.rs

