## ADDED Requirements

### Requirement: 系统必须把 raw symbol captures 解析为稳定的符号关系边
系统 MUST 在 `parse_symbols` 之后执行独立的 symbol resolution 阶段，把 raw `import / call / heritage` captures 解析为稳定的 `IMPORTS`、`CALLS`、`EXTENDS` 和 `IMPLEMENTS` edges。每条 edge MUST 至少包含稳定 `id`、`source_id`、`target_id`、`edge_type`、`confidence` 和 `reason`，并写入 SQLite 的 `edges` 表。对于无法确定目标的 raw capture，系统 MUST 保留 diagnostics 或 unresolved 状态，但不得写入伪造的目标边。

#### Scenario: init 写入解析后的符号关系边
- **WHEN** 用户执行 `init` 且仓库中存在可解析的 import、call 或 heritage 关系
- **THEN** 系统 MUST 把这些关系解析成 `edges` 表中的稳定 rows
- **THEN** 每条 edge MUST 包含关系类型、置信度和来源说明

#### Scenario: unresolved capture 不得生成伪造边
- **WHEN** 某个 raw capture 无法唯一解析出目标 symbol
- **THEN** 系统 MUST 不为该 capture 生成指向错误 symbol 的 edge row
- **THEN** 系统 MUST 继续解析同文件和同批次的其他 captures

### Requirement: 系统必须提供可复用的导入解析上下文与语言专用 resolver
系统 MUST 为一轮 workflow 内的 symbol resolution 预先构建可复用的导入解析上下文，至少包含归一化文件路径列表、`SuffixIndex`、resolve cache 和语言专用配置。导入解析 MUST 至少覆盖 TypeScript/JavaScript 的 path alias 与相对路径、Rust 的 `crate:: / super:: / self::` 模块路径、Java/Kotlin 的 package/wildcard import、Go 的 module path、PHP 的 PSR-4 namespace 映射以及 Swift 的 target/module import。

#### Scenario: TypeScript path alias 与 Rust module path 解析
- **WHEN** 仓库中的 TypeScript 文件使用 tsconfig path alias，或 Rust 文件使用 `crate::`、`super::`、`self::`
- **THEN** 系统 MUST 能把这些 import 路径解析到仓库内的目标文件或目标 symbol
- **THEN** 相同的 import 路径在同一轮 workflow 中 MUST 复用解析缓存

#### Scenario: JVM、Go、PHP 与 Swift 导入解析
- **WHEN** 仓库中的 Java/Kotlin、Go、PHP 或 Swift 文件使用其语言常见的 import/module 语法
- **THEN** 系统 MUST 按对应语言规则解析这些导入
- **THEN** 对于可解析到多个目标文件的导入，系统 MUST 生成覆盖这些目标的稳定 `IMPORTS` edges

### Requirement: 系统必须按分层置信度解析调用关系并过滤 built-ins
系统 MUST 基于 `SymbolTable`、import resolution 结果和语言内置噪声名单解析调用关系。系统 MUST 至少区分 `same-file`、`import-resolved` 和 `fuzzy-global` 三类调用命中来源，并为不同来源赋予不同的置信度和 `reason`。系统 MUST 过滤语言标准库、集合操作、日志函数、框架 hooks 与其他已知 built-in/noise 名称，避免这些调用污染 CALLS 图。

#### Scenario: 同文件、导入命中与全局兜底的调用解析
- **WHEN** 某个调用在当前文件内、已导入目标文件内或全局名称索引中存在候选目标
- **THEN** 系统 MUST 按既定优先级解析该调用并产出对应的 `CALLS` edge
- **THEN** 该 edge MUST 在 `reason` 中标记命中来源

#### Scenario: built-in 与噪声调用被过滤
- **WHEN** 调用目标属于语言 built-in、常见日志函数、集合方法或框架 hook 噪声名单
- **THEN** 系统 MUST 不为该调用生成 `CALLS` edge
- **THEN** 这些过滤不得阻断同文件其他调用关系的解析

### Requirement: `update` 必须按文件刷新 edges 并清理陈旧关系
系统 MUST 在 `update` 中按受影响文件增量刷新 `edges` 表，而不是无条件重写全量边。受影响集合 MUST 至少覆盖新增、修改、删除的源码文件，以及通过已知 `IMPORTS` 关系直接依赖这些文件的一跳反向依赖文件。删除源码或解析失败时，系统 MUST 删除这些文件曾经产出的旧 edge rows，避免 query 命中陈旧关系。

#### Scenario: 修改源码后刷新受影响文件与一跳依赖文件的 edges
- **WHEN** 某个源码文件被修改并执行 `update`
- **THEN** 系统 MUST 重解析该文件的 symbol graph
- **THEN** 系统 MUST 重新解析直接依赖该文件的一跳文件，并刷新这些文件对应的 edge rows

#### Scenario: 删除源码或解析失败后清理旧 edge rows
- **WHEN** 某个已索引源码文件被删除，或重新解析时失败
- **THEN** 系统 MUST 删除该文件此前产出的旧 edge rows
- **THEN** 后续 query 不得继续命中这些已失效的关系

