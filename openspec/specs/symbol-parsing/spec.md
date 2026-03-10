## MODIFIED Requirements

### Requirement: 系统必须在扫描后执行独立的 tree-sitter 符号解析阶段
系统 MUST 在 `scan_repo` 产出 `ScanReport` 后、符号关系解析前执行独立的 `parse_symbols` 阶段。`parse_symbols` MUST 基于文件的稳定语言标签分发到对应 parser，并统一抽取 `definition`、`import`、`call`、`heritage` 四类 capture。对于迭代 8，系统 MUST 不仅把 definition capture 提升为 `SymbolNode`，还 MUST 为后续 `resolve_symbol_graph` 提供结构化 raw capture；这些 raw capture 至少必须包含解析所需的文件路径、行号、名称/路径文本，以及可定位 owner/source symbol 的元信息。raw capture 本身 MAY unresolved，但不得因为本轮尚未完成关系解析就被 parser 丢弃。

#### Scenario: init 为 supported source 同时产出 definitions 与 raw captures
- **WHEN** 用户对包含受支持源码语言的仓库执行 `init`
- **THEN** 系统 MUST 在扫描之后执行 `parse_symbols`
- **THEN** 系统 MUST 为每个成功解析的源码文件产出 definition capture 对应的 `SymbolNode`
- **THEN** 系统 MUST 同时产出供后续 resolution 消费的 structured raw `import / call / heritage` captures

#### Scenario: 单文件解析失败不阻断 workflow
- **WHEN** `parse_symbols` 遇到某个源码文件存在 tree-sitter parse error、query mismatch 或编码问题
- **THEN** 系统 MUST 仅把该文件视为无有效 symbol 输出并记录诊断
- **THEN** 系统 MUST 继续执行后续符号关系解析、模块树、页面规划和写盘流程

### Requirement: 系统必须提供 12 种核心语言的 parser registry
系统 MUST 通过统一的 parser registry 支持至少以下 12 种核心语言的符号解析：TypeScript、JavaScript、Python、Java、C、Go、C++、C#、Rust、PHP、Kotlin、Swift。对于 React 这类由 JSX/TSX 直接落到底层 grammar 的包装语言，系统 MUST 通过底层 JavaScript / TypeScript parser 完成符号解析。对于 Vue、Svelte 这类单文件组件，系统 MUST 在本迭代抽取 `<script>` / `<script setup>` 片段并委托到底层 JavaScript / TypeScript parser，再把 definitions 和 raw captures 的行号映射回原始组件文件；系统不得继续把这些包装语言长期停留在 fail-soft 空结果。

#### Scenario: parser registry 覆盖核心语言
- **WHEN** `parse_symbols` 收到上述 12 种核心语言中的任意一种源码文件
- **THEN** 系统 MUST 能为该语言找到明确的 parser 和 query 定义
- **THEN** 系统不得把该文件误判为“无解析器可用”

#### Scenario: React JSX/TSX 委托到底层 parser
- **WHEN** 仓库中出现 `.jsx` 或 `.tsx` React 文件
- **THEN** 系统 MUST 把这些文件委托到底层 JavaScript 或 TypeScript parser
- **THEN** 系统 MUST 仍然为其中的定义类符号和 raw relation captures 产出解析结果

#### Scenario: Vue 与 Svelte 通过脚本块委托解析
- **WHEN** 仓库中出现包含 `<script>` 或 `<script setup>` 的 Vue / Svelte 文件
- **THEN** 系统 MUST 提取脚本块并委托到底层 JavaScript 或 TypeScript parser
- **THEN** 系统 MUST 把解析出的 definitions 和 raw relation captures 映射回原始组件文件路径与原始行号
