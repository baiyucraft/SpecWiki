## ADDED Requirements

### Requirement: 系统必须在扫描后执行独立的 tree-sitter 符号解析阶段
系统 MUST 在 `scan_repo` 产出 `ScanReport` 后、模块树构建前执行独立的 `parse_symbols` 阶段。`parse_symbols` MUST 基于文件的稳定语言标签分发到对应 parser，并统一抽取 `definition`、`import`、`call`、`heritage` 四类 capture。对于当前迭代，系统 MUST 至少把 definition capture 提升为 `SymbolNode`；其余 capture MAY 保留为 workflow 内存中的原始结果，供后续迭代继续解析，但不得因此阻断本轮 workflow。

#### Scenario: init 解析受支持源码
- **WHEN** 用户对包含受支持源码语言的仓库执行 `init`
- **THEN** 系统 MUST 在扫描之后执行 `parse_symbols`
- **THEN** 系统 MUST 为每个成功解析的源码文件产出 definition capture 对应的 `SymbolNode`
- **THEN** 每个 `SymbolNode` MUST 至少包含稳定 `symbol_id`、`name`、`label`、`file_path`、`start_line`、`end_line`、`is_exported` 和 `language`

#### Scenario: 单文件解析失败不阻断 workflow
- **WHEN** `parse_symbols` 遇到某个源码文件存在 tree-sitter parse error、query mismatch 或编码问题
- **THEN** 系统 MUST 仅把该文件视为无有效 symbol 输出并记录诊断
- **THEN** 系统 MUST 继续执行后续模块树、页面规划和写盘流程

### Requirement: 系统必须提供 12 种核心语言的 parser registry
系统 MUST 通过统一的 parser registry 支持至少以下 12 种核心语言的符号解析：TypeScript、JavaScript、Python、Java、C、Go、C++、C#、Rust、PHP、Kotlin、Swift。对于 React 这类由 JSX/TSX 直接落到底层 grammar 的包装语言，系统 MUST 通过底层 JavaScript / TypeScript parser 完成符号解析。对于当前仍需额外脚本块抽取的 Vue、Svelte，系统 MAY 在本迭代保持 fail-soft，并把包装层委托逻辑明确延后到迭代 8。

#### Scenario: parser registry 覆盖核心语言
- **WHEN** `parse_symbols` 收到上述 12 种核心语言中的任意一种源码文件
- **THEN** 系统 MUST 能为该语言找到明确的 parser 和 query 定义
- **THEN** 系统不得把该文件误判为“无解析器可用”

#### Scenario: React JSX/TSX 委托到底层 parser
- **WHEN** 仓库中出现 `.jsx` 或 `.tsx` React 文件
- **THEN** 系统 MUST 把这些文件委托到底层 JavaScript 或 TypeScript parser
- **THEN** 系统 MUST 仍然为其中的定义类符号产出 `SymbolNode`

#### Scenario: Vue 与 Svelte 暂未接入时 fail-soft
- **WHEN** 仓库中出现 Vue 或 Svelte 文件，但本轮尚未启用其脚本块委托解析
- **THEN** 系统 MAY 对这些文件返回空 symbol 结果
- **THEN** 系统 MUST 不得因为这些文件未解析出 symbol 而阻断 workflow

### Requirement: 系统必须维护按文件和按名称的双索引 SymbolTable
系统 MUST 为当前 workflow 内的解析结果构建双索引 `SymbolTable`：一个索引按 `file_path -> symbol_name -> symbol_ids` 组织，另一个索引按 `symbol_name -> symbol_ids` 组织。双索引 MUST 允许同名符号保留多个候选，而不是把同名定义折叠成单个节点。

#### Scenario: 同文件内按名称查找
- **WHEN** 某个文件内存在多个已解析符号
- **THEN** 系统 MUST 能通过文件路径和名称稳定地取回该文件下的候选 `symbol_id`
- **THEN** 同一文件中的不同 label 或不同起始行定义不得相互覆盖

#### Scenario: 跨文件同名符号共存
- **WHEN** 不同文件中存在同名函数、类或方法
- **THEN** 全局名称索引 MUST 同时保留这些候选
- **THEN** 系统不得因为名称重复而丢失后写入的符号

### Requirement: 系统必须按语言规则判断符号导出可见性
系统 MUST 在 symbol parsing 阶段按语言语义判断 `SymbolNode.is_exported`，至少覆盖 JavaScript/TypeScript 的 `export`、Rust 的 `pub`、Go 的大写可见性、Java/C#/PHP/Kotlin/Swift 的可见性修饰符，以及 Python 对非 `_` 前缀顶层定义的默认公开规则。

#### Scenario: Rust 与 Go 的导出判断
- **WHEN** `parse_symbols` 分别解析 Rust 和 Go 文件
- **THEN** Rust 中带 `pub` 的定义 MUST 被标记为 exported
- **THEN** Go 中首字母大写的顶层定义 MUST 被标记为 exported

#### Scenario: JavaScript 与 Python 的导出判断
- **WHEN** `parse_symbols` 分别解析 JavaScript/TypeScript 和 Python 文件
- **THEN** JavaScript/TypeScript 中显式 `export` 的定义 MUST 被标记为 exported
- **THEN** Python 中以下划线开头的顶层定义不得被默认标记为 exported

### Requirement: 系统必须按字节预算分批执行符号解析
系统 MUST 为 symbol parsing 设定稳定的字节预算批次，并按确定性顺序对源码文件分批解析，以控制大型仓库的内存峰值。预算控制只影响批次切分，不得改变最终 symbol 集合或符号 ID。

#### Scenario: 大型仓库按批解析
- **WHEN** 仓库中的待解析源码总量超过单批预算
- **THEN** 系统 MUST 把文件切分为多个解析批次
- **THEN** 每个批次内文件处理顺序 MUST 保持 deterministic
- **THEN** 多批执行后的 symbol 结果 MUST 与单批语义一致
