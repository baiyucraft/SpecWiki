## MODIFIED Requirements

### Requirement: 系统必须提供端到端验证适配层到 Wiki 产物的主链路
系统 MUST 提供自动化测试，验证适配层调用 Rust core 后能够在目标仓库完成 Wiki 初始化、增量更新并执行查询，且生成的 `.wiki/` 产物与返回结果符合预期。针对迭代 4 收口，验证 MUST 额外覆盖 scanner 噪声过滤质量（fixture 排除、嵌套仓库排除、单文件模块抑制、非代码目录排除）、module kind 分类准确性和关键源码选择信号的正确性。每轮与迭代 4 相关的 tasks 设计、实现或测试时，还 MUST 对测试项目集执行 `init` 分析，至少固定包含 `E:\\project\\aLocal` 和当前仓库 `E:\\project\\!byAI\\spec-wiki`，并在存在 reference 时对照 `.wiki/*.md` 与 `wiki.metadata.json`。

#### Scenario: 初始化并查询 Wiki
- **WHEN** 端到端测试在临时 Git 仓库中执行 `init` 后再执行 `query`
- **THEN** 测试必须观察到 `.wiki/` 目录和 `wiki.metadata.json` 已生成，且查询结果包含匹配页面

#### Scenario: 修改单个源码后执行增量更新
- **WHEN** 端到端或集成测试在初始化后只修改一个与单一模块相关的源码文件并执行 `update`
- **THEN** 测试必须观察到 `updated_pages` 只包含受影响页面
- **THEN** 测试必须观察到未受影响页面未被无条件重写

#### Scenario: 新增或删除源码后执行更新
- **WHEN** 测试在初始化后新增或删除源码文件并执行 `update`
- **THEN** 测试必须观察到模块树和页面规划被正确更新
- **THEN** 测试必须观察到 metadata 中的页面集合与最终 `.wiki/` 落盘结果一致

#### Scenario: cache 缺失后执行状态检查和重建
- **WHEN** 测试删除关键 cache 文件后执行 `status`、`update` 或 `rebuild`
- **THEN** 测试必须观察到系统把运行时提升为 `needs_rebuild` 或走 full rebuild
- **THEN** 测试不得观察到系统在半残 cache 上继续声称 runtime 为 `fresh`

#### Scenario: 对照测试项目集与参考产物
- **WHEN** 开发者为迭代 4 相关 change 设计 tasks、执行实现或做回归验证
- **THEN** 系统必须能够对测试项目集中的目标仓库执行 `init`
- **THEN** 开发者必须固定分析 `E:\\project\\aLocal` 和 `E:\\project\\!byAI\\spec-wiki` 的产物
- **THEN** 如果目标仓库存在 reference 参考，则必须将生成结果与 reference 的页面结构和 metadata 字段做对照

#### Scenario: 验证 fixture 和嵌套仓库被正确排除
- **WHEN** 对当前仓库 `E:\\project\\!byAI\\spec-wiki` 执行 `init`
- **THEN** `crates/wiki-core/tests/fixtures/` 下的测试仓库不得被提升为正式模块
- **THEN** 生成的模块页中不得包含以 fixture 目录命名的页面

#### Scenario: 验证单文件配置不被提升为模块
- **WHEN** 对当前仓库执行 `init`
- **THEN** `eslint.config.mjs`、`vitest.config.mjs` 等单文件配置不得各自生成独立模块页

#### Scenario: 验证非代码目录不被提升为模块
- **WHEN** 对当前仓库执行 `init`
- **THEN** `.spec` 目录不得被提升为模块并生成模块页

#### Scenario: 验证 module kind 分类准确性
- **WHEN** 对当前仓库执行 `init`
- **THEN** `wiki-core` 模块的 kind 不得为 `infrastructure`，应为 `library` 或更准确的类型
- **THEN** `codebuddy` 模块的 kind 不得为 `frontend-app`，应为 `module` 或更准确的类型

#### Scenario: 验证关键源码选择不被 fixture 淹没
- **WHEN** 对当前仓库执行 `init`
- **THEN** `wiki-core` 模块页的关键源码列表中必须包含 `src/domain/` 或 `src/workflows/` 下的文件
- **THEN** 关键源码列表中不得全部来自 `tests/fixtures/`
