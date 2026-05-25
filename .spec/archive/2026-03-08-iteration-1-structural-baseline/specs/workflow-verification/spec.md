## MODIFIED Requirements

### Requirement: 系统必须提供端到端验证适配层到 Wiki 产物的主链路
系统 MUST 提供自动化测试，验证适配层调用 Rust core 后能够在具有递归模块结构的中型本地代码目录中完成 Wiki 初始化、导出 metadata/cache 并执行结构优先查询，且生成的 `.wiki/` 产物与返回结果符合 baseline 预期。

#### Scenario: 在层级化仓库夹具中初始化并查询 Wiki
- **WHEN** 端到端或集成测试在具有多级模块结构的本地代码目录中执行 `init` 后再执行 `query`
- **THEN** 测试必须观察到 `.wiki/` 目录、`wiki.metadata.json` 和 `.wiki/.cache/` 已生成
- **THEN** 测试必须观察到总览页、架构页和至少一类模块页已落盘
- **THEN** 查询结果必须包含页面、模块或源码等结构化命中对象

#### Scenario: 修改源码后执行更新
- **WHEN** 端到端测试在初始化后修改目标仓库中的源码文件并执行 `update`
- **THEN** 测试必须得到非空的更新结果，证明当前公开更新链路可工作
