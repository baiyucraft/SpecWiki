## ADDED Requirements

### Requirement: 系统必须支持 repo 级 steering 配置文件
系统 MUST 支持从 `.wiki/wiki.steering.yaml` 读取 steering 配置。配置文件 MUST 使用 `version` 字段标识 schema 版本。当配置文件不存在时，系统 MUST 使用合理默认值正常运行，不得阻塞 pipeline 或输出 warning。

#### Scenario: 读取存在的 steering 配置
- **WHEN** `.wiki/wiki.steering.yaml` 存在且格式合法
- **THEN** 系统 MUST 解析配置并在 planner 阶段消费
- **THEN** 配置中的所有字段 MUST 按 schema 定义生效

#### Scenario: steering 配置不存在
- **WHEN** `.wiki/wiki.steering.yaml` 不存在
- **THEN** 系统 MUST 使用默认值（空忽略列表、无提升/降级、默认合并阈值）
- **THEN** pipeline MUST 正常完成，行为与未引入 steering 时一致

#### Scenario: steering 配置格式非法
- **WHEN** `.wiki/wiki.steering.yaml` 存在但格式不合法（YAML 语法错误或字段类型不匹配）
- **THEN** 系统 MUST 输出明确的解析错误信息
- **THEN** 系统 MUST 回退到默认值继续运行，不得中断 pipeline

### Requirement: steering 配置必须支持全局和按语言两层忽略路径
系统 MUST 允许用户通过 steering 配置的 `ignore` 字段声明额外的忽略路径 glob 模式。`ignore` 字段 MUST 支持两层结构：`ignore.global`（所有仓库生效）和 `ignore.<language>`（仅对 scanner 检测到的主语言匹配时生效）。这些模式 MUST 追加到 scanner 内置排除规则之上，在扫描阶段生效。

#### Scenario: 配置全局忽略路径
- **WHEN** steering 配置中声明 `ignore.global: ["docs/**", "examples/**"]`
- **THEN** scanner MUST 在内置排除规则之外额外排除匹配这些 glob 的文件和目录
- **THEN** 被排除的文件不得出现在 `RepoFacts.files` 中

#### Scenario: 配置按语言忽略路径
- **WHEN** steering 配置中声明 `ignore.rust: ["benches/**"]`
- **AND** scanner 检测到仓库主语言为 Rust
- **THEN** scanner MUST 额外排除匹配 `benches/**` 的文件和目录
- **THEN** 被排除的文件不得出现在 `RepoFacts.files` 中

#### Scenario: 按语言忽略路径不匹配当前仓库语言
- **WHEN** steering 配置中声明 `ignore.rust: ["benches/**"]`
- **AND** scanner 检测到仓库主语言不是 Rust
- **THEN** 该语言忽略规则 MUST 不生效，`benches/**` 下的文件正常扫描

#### Scenario: 全局和按语言忽略路径同时存在
- **WHEN** steering 配置中同时声明 `ignore.global` 和 `ignore.<language>`
- **THEN** scanner MUST 合并两层忽略路径（取并集），全部追加到内置排除规则之上

#### Scenario: 忽略路径为空
- **WHEN** steering 配置中 `ignore` 为空对象或未声明
- **THEN** scanner MUST 只使用内置排除规则，行为不变

### Requirement: scanner 必须内置 per-language 默认忽略规则
系统 MUST 在 scanner 中内置常见语言的默认忽略路径，无需用户配置即可排除语言特有的产物目录。内置规则 MUST 根据 scanner 检测到的仓库主语言自动激活。steering 配置中的用户忽略路径 MUST 追加到内置规则之上（不覆盖）。

#### Scenario: Rust 仓库内置忽略
- **WHEN** scanner 检测到仓库主语言为 Rust
- **THEN** scanner MUST 自动排除 `target/` 目录

#### Scenario: JavaScript/TypeScript 仓库内置忽略
- **WHEN** scanner 检测到仓库主语言为 JavaScript 或 TypeScript
- **THEN** scanner MUST 自动排除 `node_modules/`、`dist/`、`.next/`、`.nuxt/` 等常见产物目录

#### Scenario: Python 仓库内置忽略
- **WHEN** scanner 检测到仓库主语言为 Python
- **THEN** scanner MUST 自动排除 `__pycache__/`、`.venv/`、`*.egg-info/` 等常见产物目录

#### Scenario: Java 仓库内置忽略
- **WHEN** scanner 检测到仓库主语言为 Java
- **THEN** scanner MUST 自动排除 `build/`、`.gradle/`、`target/`（Maven）等常见产物目录

#### Scenario: Go 仓库内置忽略
- **WHEN** scanner 检测到仓库主语言为 Go
- **THEN** scanner MUST 自动排除 `vendor/`（如果存在 `go.sum`）等常见产物目录

### Requirement: steering 配置必须支持模块提升和降级
系统 MUST 允许用户通过 steering 配置的 `modules.promote` 和 `modules.demote` 字段声明模块提升和降级。提升的模块 MUST 强制生成独立页面（即使低于合并阈值）。降级的模块 MUST 被合并到父模块页面（即使高于合并阈值）。

#### Scenario: 提升低权重模块
- **WHEN** steering 配置中声明 `modules.promote` 包含某个模块路径
- **THEN** 该模块 MUST 生成独立页面，即使其页面权重低于合并阈值

#### Scenario: 降级高权重模块
- **WHEN** steering 配置中声明 `modules.demote` 包含某个模块路径
- **THEN** 该模块 MUST 被合并到父模块页面，即使其页面权重高于合并阈值
- **THEN** 该模块的内容 MUST 作为父模块页面的子模块概述 section 出现

#### Scenario: 提升/降级路径不匹配任何模块
- **WHEN** steering 配置中声明的提升或降级路径不匹配当前模块树中的任何模块
- **THEN** 系统 MUST 忽略该条目，不得报错或中断

### Requirement: steering 配置必须支持小模块合并阈值
系统 MUST 允许用户通过 steering 配置的 `merge_threshold` 字段调整小模块合并阈值。阈值 MUST 表示"源文件数量 ≤ 该值且无子模块的模块被合并到父模块页面"。

#### Scenario: 自定义合并阈值
- **WHEN** steering 配置中声明 `merge_threshold: 5`
- **THEN** planner MUST 使用 5 作为合并阈值
- **THEN** 源文件数量 ≤ 5 且无子模块的模块 MUST 被合并到父模块页面

#### Scenario: 合并阈值为 0
- **WHEN** steering 配置中声明 `merge_threshold: 0`
- **THEN** planner MUST 不合并任何模块，所有模块都生成独立页面

#### Scenario: 未声明合并阈值
- **WHEN** steering 配置中未声明 `merge_threshold`
- **THEN** planner MUST 使用默认阈值（3）

### Requirement: steering 配置必须支持页面优先级调整
系统 MUST 允许用户通过 steering 配置的 `pages.priority` 字段为特定路径下的模块页面设置优先级提升。优先级提升 MUST 影响页面在 metadata 中的排序。

#### Scenario: 提升特定路径的页面优先级
- **WHEN** steering 配置中声明 `pages.priority` 包含某个路径和 boost 值
- **THEN** 该路径对应的模块页面 MUST 在 metadata 导出时获得更高的排序优先级

#### Scenario: 未声明优先级调整
- **WHEN** steering 配置中未声明 `pages.priority`
- **THEN** 页面排序 MUST 使用 planner 的默认优先级规则
