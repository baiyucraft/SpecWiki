## ADDED Requirements

### Requirement: page_id 必须锚定到模块 root_path 而不是模块名或发现顺序
系统 MUST 为 module 类型页面生成锚定到模块 `root_paths[0]`（归一化后的相对路径）的稳定 `page_id`。overview 和 architecture 页面的 `page_id` MUST 使用固定种子。`page_id` 不得依赖模块名、模块发现顺序或模块 kind 分类结果。

#### Scenario: 模块名变化但 root_path 不变时 page_id 稳定
- **WHEN** 模块的 kind 分类或显示名称因为启发式规则调整而变化，但模块的 root_path 保持不变
- **THEN** 该模块对应页面的 `page_id` MUST 保持不变

#### Scenario: 增删少量源文件后核心页面 page_id 不变
- **WHEN** 仓库中增加或删除少量源文件，但不影响核心模块的 root_path
- **THEN** 所有核心模块页面的 `page_id` MUST 保持不变
- **THEN** overview 和 architecture 页面的 `page_id` MUST 保持不变

#### Scenario: overview 和 architecture 页面使用固定种子
- **WHEN** 系统生成 overview 或 architecture 页面
- **THEN** 这两个页面的 `page_id` MUST 始终使用固定种子生成
- **THEN** 无论模块树如何变化，这两个页面的 `page_id` MUST 保持绝对稳定

### Requirement: page_path 必须锚定到模块 root_path 的归一化形式
系统 MUST 为 module 类型页面生成锚定到模块 `root_paths[0]` 归一化形式的稳定 `page_path`（`.wiki/` 下的文件路径）。`page_path` 不得依赖模块名或发现顺序。路径中不得包含 Windows 非法字符。

#### Scenario: 模块名变化但 root_path 不变时 page_path 稳定
- **WHEN** 模块的显示名称变化但 root_path 不变
- **THEN** 该模块对应页面的 `page_path` MUST 保持不变

#### Scenario: 嵌套模块的 page_path 反映层级结构
- **WHEN** 模块位于多级嵌套路径（如 `packages/domain/auth`）
- **THEN** 该模块的 `page_path` MUST 反映其在模块树中的层级位置
- **THEN** `page_path` 中不得包含 Windows 非法路径字符

### Requirement: 页面父子关系必须按模块树层级分配
系统 MUST 按模块树的层级关系自动分配页面父子关系，而不是将所有模块页都平铺在 overview 页下面。顶层模块页 MUST 以 overview 页为父页面。嵌套模块页的父页面 MUST 是其在模块树中的父模块对应的页面。如果父模块因合并策略没有独立页面，MUST 向上查找最近的有独立页面的祖先模块。

#### Scenario: 顶层模块页的父页面是 overview
- **WHEN** 系统为顶层模块生成页面
- **THEN** 该页面的 `parent_id` MUST 指向 overview 页面

#### Scenario: 嵌套模块页的父页面是父模块页
- **WHEN** 系统为嵌套模块生成页面，且其父模块有独立页面
- **THEN** 该页面的 `parent_id` MUST 指向父模块的页面

#### Scenario: 父模块被合并时向上查找祖先
- **WHEN** 嵌套模块的直接父模块因合并策略没有独立页面
- **THEN** 该页面的 `parent_id` MUST 指向最近的有独立页面的祖先模块页面
- **THEN** 如果所有祖先模块都没有独立页面，`parent_id` MUST 指向 overview 页面

### Requirement: 小模块必须按合并策略合并到父模块页面
系统 MUST 基于模块"页面权重"评分决定是否为模块生成独立页面。页面权重 MUST 基于源码文件数量、是否有子模块、是否是 workspace 成员、是否有入口文件等因素计算。权重低于合并阈值的模块 MUST 不生成独立页面，其内容 MUST 合并到父模块页面中。合并后的模块 MUST 仍保留在 `WikiState.modules` 中。

#### Scenario: 低权重模块被合并
- **WHEN** 某模块的源文件数量 ≤ 合并阈值且没有子模块
- **THEN** 该模块 MUST 不生成独立页面
- **THEN** 该模块的内容 MUST 作为父模块页面的子模块概述 section 出现
- **THEN** 该模块 MUST 仍保留在 `WikiState.modules` 中

#### Scenario: 高权重模块保留独立页面
- **WHEN** 某模块的源文件数量 > 合并阈值或拥有子模块
- **THEN** 该模块 MUST 生成独立页面

#### Scenario: steering 配置覆盖合并决策
- **WHEN** steering 配置中 promote 了一个低权重模块
- **THEN** 该模块 MUST 生成独立页面，忽略合并阈值
- **WHEN** steering 配置中 demote 了一个高权重模块
- **THEN** 该模块 MUST 被合并到父模块页面，忽略合并阈值
