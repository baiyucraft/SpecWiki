# cobra Reference 对比报告

生成页面：4 页
reference 页面：38 页
命中对比：31 页
缺失对比：7 页

## 项目结论

- reference 文档树明显更细，当前 planner 仍以总览页和少量模块页为主
- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- Mermaid/结构图表达仍然不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 项目概述.md 被 22 个 reference 页面共享映射
- 核心模块/cobra.md 被 9 个 reference 页面共享映射

## 额外生成页面

- 工作流与部署.md (工作流与部署, 27 行)
- 系统架构.md (系统架构, 44 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | --- |
| 命令系统/命令定义与配置.md | 项目概述.md | 338/38 | 152/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、bash_completions.go、man_docs.go、md_docs.go、user_guide.md |
| 命令系统/命令执行流程.md | 项目概述.md | 257/38 | 104/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、completions.go、flag_groups.go、user_guide.md |
| 命令系统/命令生命周期.md | 项目概述.md | 257/38 | 84/7 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flag_groups.go、user_guide.md、main.go |
| 命令系统/命令系统.md | 核心模块/cobra.md | 312/38 | 95/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、cmd_test.go、user_guide.md |
| 命令系统/子命令管理.md | 项目概述.md | 296/38 | 103/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flag_groups.go、user_guide.md |
| 命令系统/钩子函数详解.md | 项目概述.md | 261/38 | 93/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、user_guide.md |
| 开发者指南.md | 项目概述.md | 286/38 | 117/7 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：dependabot.yml、labeler.yml、.golangci.yml、conduct.md、contributing.md、readme.md、security.md、cmd_test.go |
| 快速开始.md | 项目概述.md | 284/38 | 133/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、cobra_test.go、completions.go、cmd_test.go、user_guide.md、go.yaml、main.go、root.go |
| 文档生成系统/Man 手册生成.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 文档生成系统/Markdown 文档生成.md | 项目概述.md | 292/38 | 88/7 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：md_docs.go、md_docs_test.go、util.go、md.md |
| 文档生成系统/REST API 文档生成.md | 项目概述.md | 255/38 | 79/7 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：man_docs.go、md_docs.go、rest_docs.go、rest_docs_test.go、util.go、yaml_docs.go、rest.md |
| 文档生成系统/YAML 配置文档生成.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 文档生成系统/文档生成工具集.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 文档生成系统/文档生成系统.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 标志系统/标志类型与定义.md | 项目概述.md | 222/38 | 70/7 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：flag_groups.go、user_guide.md |
| 标志系统/标志系统.md | 项目概述.md | 295/38 | 105/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：flag_groups.go、flag_groups_test.go、user_guide.md |
| 标志系统/标志组管理.md | 项目概述.md | 269/38 | 104/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：completions_test.go、flag_groups.go、flag_groups_test.go、user_guide.md |
| 标志系统/标志继承机制.md | 项目概述.md | 234/38 | 97/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：user_guide.md |
| 标志系统/标志验证与定制.md | 核心模块/cobra.md | 300/38 | 128/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bash_completions.go、completions.go、flag_groups.go、flag_groups_test.go、user_guide.md |
| 核心概念/参数基础概念.md | 项目概述.md | 356/38 | 266/7 | 10/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、args_test.go |
| 核心概念/命令基础概念.md | 核心模块/cobra.md | 237/38 | 98/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md |
| 核心概念/架构设计理念.md | 核心模块/cobra.md | 289/38 | 156/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、bash_completions.go、cobra_test.go、completions.go、man_docs.go、md_docs.go、rest_docs.go、yaml_docs.go |
| 核心概念/标志基础概念.md | 项目概述.md | 280/38 | 107/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：completions_test.go、flag_groups.go、user_guide.md |
| 核心概念/核心概念.md | 项目概述.md | 319/38 | 152/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、completions.go、cmd_test.go、flag_groups.go、user_guide.md |
| 核心概念/钩子函数机制.md | 核心模块/cobra.md | 211/38 | 85/7 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：user_guide.md |
| 测试指南.md | 项目概述.md | 391/38 | 142/7 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、active_help_test.go、args_test.go、bash_completions_test.go、cobra_test.go、completions_test.go、cmd_test.go、man_docs_test.go |
| 补全系统/Active Help 功能.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 补全系统/Shell 补全支持.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 补全系统/自定义补全函数.md | 项目概述.md | 323/38 | 90/7 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bash_completions.go、completions.go、completions_test.go、fish_completions.go、shell_completions.go、_index.md、bash.md、zsh.md |
| 补全系统/补全数据管理.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 补全系统/补全系统.md | 核心模块/cobra.md | 357/38 | 108/7 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bash_completions.go、completions.go、completions_test.go、fish_completions.go、powershell_completions.go、shell_completions.go、active_help.md、_index.md |
| 项目概述.md | 项目概述.md | 284/38 | 140/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、bash_completions.go、completions.go、man_docs.go、md_docs.go、fish_completions.go、powershell_completions.go、projects_using_cobra.md |
| 高级特性/平台集成.md | 核心模块/cobra.md | 229/38 | 84/7 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md |
| 高级特性/性能优化.md | 核心模块/cobra.md | 340/38 | 138/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cobra_test.go、completions.go |
| 高级特性/扩展机制.md | 项目概述.md | 235/38 | 78/7 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：util.go、user_guide.md |
| 高级特性/模板系统.md | 项目概述.md | 258/38 | 112/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cobra_test.go、user_guide.md |
| 高级特性/错误处理.md | 项目概述.md | 284/38 | 120/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：args_test.go |
| 高级特性/高级特性.md | 核心模块/cobra.md | 398/38 | 132/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、active_help_test.go、bash_completions.go、cobra_test.go、completions.go、completions_test.go、fish_completions.go、powershell_completions.go |

## 逐文件详情

### 命令系统/命令定义与配置.md

- reference 标题：命令定义与配置
- 生成页：项目概述.md（项目概述）
- 匹配分数：96
- 行数：338 / 38
- 段落行数：152 / 7
- Mermaid：7 / 0
- 文件提及重合：cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：readme.md、bash_completions.go、man_docs.go、md_docs.go、user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、bash_completions.go、man_docs.go、md_docs.go、user_guide.md

### 命令系统/命令执行流程.md

- reference 标题：命令执行流程
- 生成页：项目概述.md（项目概述）
- 匹配分数：122
- 行数：257 / 38
- 段落行数：104 / 7
- Mermaid：5 / 0
- 文件提及重合：args.go、cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：readme.md、completions.go、flag_groups.go、user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、completions.go、flag_groups.go、user_guide.md

### 命令系统/命令生命周期.md

- reference 标题：命令生命周期
- 生成页：项目概述.md（项目概述）
- 匹配分数：98
- 行数：257 / 38
- 段落行数：84 / 7
- Mermaid：3 / 0
- 文件提及重合：cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：readme.md、flag_groups.go、user_guide.md、main.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flag_groups.go、user_guide.md、main.go

### 命令系统/命令系统.md

- reference 标题：命令系统
- 生成页：核心模块/cobra.md（模块：cobra）
- 匹配分数：108
- 行数：312 / 38
- 段落行数：95 / 7
- Mermaid：5 / 0
- 文件提及重合：cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：readme.md、cmd_test.go、user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、cmd_test.go、user_guide.md

### 命令系统/子命令管理.md

- reference 标题：子命令管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：108
- 行数：296 / 38
- 段落行数：103 / 7
- Mermaid：7 / 0
- 文件提及重合：cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：readme.md、flag_groups.go、user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flag_groups.go、user_guide.md

### 命令系统/钩子函数详解.md

- reference 标题：钩子函数详解
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 行数：261 / 38
- 段落行数：93 / 7
- Mermaid：5 / 0
- 文件提及重合：cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：readme.md、user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、user_guide.md

### 开发者指南.md

- reference 标题：开发者指南
- 生成页：项目概述.md（项目概述）
- 匹配分数：74
- 行数：286 / 38
- 段落行数：117 / 7
- Mermaid：4 / 0
- 文件提及重合：cobra.go、command.go
- reference 关键文件未覆盖：dependabot.yml、labeler.yml、.golangci.yml、conduct.md、contributing.md、readme.md、security.md、cmd_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：dependabot.yml、labeler.yml、.golangci.yml、conduct.md、contributing.md、readme.md、security.md、cmd_test.go

### 快速开始.md

- reference 标题：快速开始
- 生成页：项目概述.md（项目概述）
- 匹配分数：102
- 行数：284 / 38
- 段落行数：133 / 7
- Mermaid：7 / 0
- 文件提及重合：args.go、cobra.go、command.go
- reference 关键文件未覆盖：readme.md、cobra_test.go、completions.go、cmd_test.go、user_guide.md、go.yaml、main.go、root.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、cobra_test.go、completions.go、cmd_test.go、user_guide.md、go.yaml、main.go、root.go

### 文档生成系统/Man 手册生成.md

- reference 标题：Man 手册生成
- 生成页：无
- 问题：缺少对应生成页面

### 文档生成系统/Markdown 文档生成.md

- reference 标题：Markdown 文档生成
- 生成页：项目概述.md（项目概述）
- 匹配分数：64
- 行数：292 / 38
- 段落行数：88 / 7
- Mermaid：3 / 0
- 文件提及重合：cobra.go、command.go
- reference 关键文件未覆盖：md_docs.go、md_docs_test.go、util.go、md.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：md_docs.go、md_docs_test.go、util.go、md.md

### 文档生成系统/REST API 文档生成.md

- reference 标题：REST API 文档生成
- 生成页：项目概述.md（项目概述）
- 匹配分数：64
- 行数：255 / 38
- 段落行数：79 / 7
- Mermaid：3 / 0
- 文件提及重合：cobra.go、command.go
- reference 关键文件未覆盖：man_docs.go、md_docs.go、rest_docs.go、rest_docs_test.go、util.go、yaml_docs.go、rest.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：man_docs.go、md_docs.go、rest_docs.go、rest_docs_test.go、util.go、yaml_docs.go、rest.md

### 文档生成系统/YAML 配置文档生成.md

- reference 标题：YAML 配置文档生成
- 生成页：无
- 问题：缺少对应生成页面

### 文档生成系统/文档生成工具集.md

- reference 标题：文档生成工具集
- 生成页：无
- 问题：缺少对应生成页面

### 文档生成系统/文档生成系统.md

- reference 标题：文档生成系统
- 生成页：无
- 问题：缺少对应生成页面

### 标志系统/标志类型与定义.md

- reference 标题：标志类型与定义
- 生成页：项目概述.md（项目概述）
- 匹配分数：64
- 行数：222 / 38
- 段落行数：70 / 7
- Mermaid：4 / 0
- 文件提及重合：cobra.go、command.go
- reference 关键文件未覆盖：flag_groups.go、user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：flag_groups.go、user_guide.md

### 标志系统/标志系统.md

- reference 标题：标志系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：100
- 行数：295 / 38
- 段落行数：105 / 7
- Mermaid：6 / 0
- 文件提及重合：cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：flag_groups.go、flag_groups_test.go、user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：flag_groups.go、flag_groups_test.go、user_guide.md

### 标志系统/标志组管理.md

- reference 标题：标志组管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：72
- 行数：269 / 38
- 段落行数：104 / 7
- Mermaid：6 / 0
- 文件提及重合：command.go、command_test.go
- reference 关键文件未覆盖：completions_test.go、flag_groups.go、flag_groups_test.go、user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：completions_test.go、flag_groups.go、flag_groups_test.go、user_guide.md

### 标志系统/标志继承机制.md

- reference 标题：标志继承机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：98
- 行数：234 / 38
- 段落行数：97 / 7
- Mermaid：6 / 0
- 文件提及重合：cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：user_guide.md

### 标志系统/标志验证与定制.md

- reference 标题：标志验证与定制
- 生成页：核心模块/cobra.md（模块：cobra）
- 匹配分数：90
- 行数：300 / 38
- 段落行数：128 / 7
- Mermaid：8 / 0
- 文件提及重合：cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：bash_completions.go、completions.go、flag_groups.go、flag_groups_test.go、user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bash_completions.go、completions.go、flag_groups.go、flag_groups_test.go、user_guide.md

### 核心概念/参数基础概念.md

- reference 标题：参数基础概念
- 生成页：项目概述.md（项目概述）
- 匹配分数：122
- 行数：356 / 38
- 段落行数：266 / 7
- Mermaid：10 / 0
- 文件提及重合：args.go、cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：readme.md、args_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、args_test.go

### 核心概念/命令基础概念.md

- reference 标题：命令基础概念
- 生成页：核心模块/cobra.md（模块：cobra）
- 匹配分数：100
- 行数：237 / 38
- 段落行数：98 / 7
- Mermaid：5 / 0
- 文件提及重合：cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：readme.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md

### 核心概念/架构设计理念.md

- reference 标题：架构设计理念
- 生成页：核心模块/cobra.md（模块：cobra）
- 匹配分数：152
- 行数：289 / 38
- 段落行数：156 / 7
- Mermaid：8 / 0
- 文件提及重合：active_help.go、args.go、cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：readme.md、bash_completions.go、cobra_test.go、completions.go、man_docs.go、md_docs.go、rest_docs.go、yaml_docs.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、bash_completions.go、cobra_test.go、completions.go、man_docs.go、md_docs.go、rest_docs.go、yaml_docs.go

### 核心概念/标志基础概念.md

- reference 标题：标志基础概念
- 生成页：项目概述.md（项目概述）
- 匹配分数：106
- 行数：280 / 38
- 段落行数：107 / 7
- Mermaid：5 / 0
- 文件提及重合：cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：completions_test.go、flag_groups.go、user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：completions_test.go、flag_groups.go、user_guide.md

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：项目概述.md（项目概述）
- 匹配分数：98
- 行数：319 / 38
- 段落行数：152 / 7
- Mermaid：8 / 0
- 文件提及重合：args.go、cobra.go、command.go
- reference 关键文件未覆盖：readme.md、completions.go、cmd_test.go、flag_groups.go、user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、completions.go、cmd_test.go、flag_groups.go、user_guide.md

### 核心概念/钩子函数机制.md

- reference 标题：钩子函数机制
- 生成页：核心模块/cobra.md（模块：cobra）
- 匹配分数：90
- 行数：211 / 38
- 段落行数：85 / 7
- Mermaid：4 / 0
- 文件提及重合：cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：user_guide.md

### 测试指南.md

- reference 标题：测试指南
- 生成页：项目概述.md（项目概述）
- 匹配分数：106
- 行数：391 / 38
- 段落行数：142 / 7
- Mermaid：9 / 0
- 文件提及重合：command_test.go、cobra.go、command.go
- reference 关键文件未覆盖：readme.md、active_help_test.go、args_test.go、bash_completions_test.go、cobra_test.go、completions_test.go、cmd_test.go、man_docs_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、active_help_test.go、args_test.go、bash_completions_test.go、cobra_test.go、completions_test.go、cmd_test.go、man_docs_test.go

### 补全系统/Active Help 功能.md

- reference 标题：Active Help 功能
- 生成页：无
- 问题：缺少对应生成页面

### 补全系统/Shell 补全支持.md

- reference 标题：Shell 补全支持
- 生成页：无
- 问题：缺少对应生成页面

### 补全系统/自定义补全函数.md

- reference 标题：自定义补全函数
- 生成页：项目概述.md（项目概述）
- 匹配分数：66
- 行数：323 / 38
- 段落行数：90 / 7
- Mermaid：3 / 0
- 文件提及重合：cobra.go、command.go
- reference 关键文件未覆盖：bash_completions.go、completions.go、completions_test.go、fish_completions.go、shell_completions.go、_index.md、bash.md、zsh.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bash_completions.go、completions.go、completions_test.go、fish_completions.go、shell_completions.go、_index.md、bash.md、zsh.md

### 补全系统/补全数据管理.md

- reference 标题：补全数据管理
- 生成页：无
- 问题：缺少对应生成页面

### 补全系统/补全系统.md

- reference 标题：补全系统
- 生成页：核心模块/cobra.md（模块：cobra）
- 匹配分数：62
- 行数：357 / 38
- 段落行数：108 / 7
- Mermaid：4 / 0
- 文件提及重合：active_help.go、command.go
- reference 关键文件未覆盖：bash_completions.go、completions.go、completions_test.go、fish_completions.go、powershell_completions.go、shell_completions.go、active_help.md、_index.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bash_completions.go、completions.go、completions_test.go、fish_completions.go、powershell_completions.go、shell_completions.go、active_help.md、_index.md

### 项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：630
- 行数：284 / 38
- 段落行数：140 / 7
- Mermaid：6 / 0
- 文件提及重合：cobra.go、command.go
- reference 关键文件未覆盖：readme.md、bash_completions.go、completions.go、man_docs.go、md_docs.go、fish_completions.go、powershell_completions.go、projects_using_cobra.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、bash_completions.go、completions.go、man_docs.go、md_docs.go、fish_completions.go、powershell_completions.go、projects_using_cobra.md

### 高级特性/平台集成.md

- reference 标题：平台集成
- 生成页：核心模块/cobra.md（模块：cobra）
- 匹配分数：152
- 行数：229 / 38
- 段落行数：84 / 7
- Mermaid：4 / 0
- 文件提及重合：active_help.go、cobra.go、command.go、command_notwin.go、command_win.go
- reference 关键文件未覆盖：readme.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md

### 高级特性/性能优化.md

- reference 标题：性能优化
- 生成页：核心模块/cobra.md（模块：cobra）
- 匹配分数：150
- 行数：340 / 38
- 段落行数：138 / 7
- Mermaid：7 / 0
- 文件提及重合：active_help.go、args.go、cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：cobra_test.go、completions.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cobra_test.go、completions.go

### 高级特性/扩展机制.md

- reference 标题：扩展机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 行数：235 / 38
- 段落行数：78 / 7
- Mermaid：4 / 0
- 文件提及重合：cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：util.go、user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：util.go、user_guide.md

### 高级特性/模板系统.md

- reference 标题：模板系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：68
- 行数：258 / 38
- 段落行数：112 / 7
- Mermaid：6 / 0
- 文件提及重合：cobra.go、command.go
- reference 关键文件未覆盖：cobra_test.go、user_guide.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cobra_test.go、user_guide.md

### 高级特性/错误处理.md

- reference 标题：错误处理
- 生成页：项目概述.md（项目概述）
- 匹配分数：102
- 行数：284 / 38
- 段落行数：120 / 7
- Mermaid：8 / 0
- 文件提及重合：cobra.go、command.go、command_test.go
- reference 关键文件未覆盖：args_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：args_test.go

### 高级特性/高级特性.md

- reference 标题：高级特性
- 生成页：核心模块/cobra.md（模块：cobra）
- 匹配分数：148
- 行数：398 / 38
- 段落行数：132 / 7
- Mermaid：7 / 0
- 文件提及重合：active_help.go、cobra.go、command.go、command_notwin.go、command_win.go
- reference 关键文件未覆盖：readme.md、active_help_test.go、bash_completions.go、cobra_test.go、completions.go、completions_test.go、fish_completions.go、powershell_completions.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、active_help_test.go、bash_completions.go、cobra_test.go、completions.go、completions_test.go、fish_completions.go、powershell_completions.go

