## MODIFIED Requirements

### Requirement: 系统必须提供端到端验证适配层到 Wiki 产物的主链路
系统 MUST 提供自动化测试，验证适配层调用 Rust core 后能够在目标仓库完成 Wiki 初始化、检测更新并执行查询，且生成的 `.wiki/` 产物与返回结果符合预期。针对迭代 1 收口，还 MUST 固定包含 `E:\\project\\aLocal` 的 `init` 产物快照步骤，并将快照写入 `tmp/` 下用于与参考样例对照。

#### Scenario: 初始化并查询 Wiki
- **WHEN** 端到端测试在临时 Git 仓库中执行 `init` 后再执行 `query`
- **THEN** 测试必须观察到 `.wiki/` 目录和 `wiki.metadata.json` 已生成，且查询结果包含匹配页面

#### Scenario: 修改源码后执行更新
- **WHEN** 端到端测试在初始化后修改目标仓库中的源码文件并执行 `update`
- **THEN** 测试必须得到非空的更新结果，证明当前公开更新链路可工作

#### Scenario: 对照 `aLocal` 参考产物
- **WHEN** 开发者为迭代 1 相关 change 设计 tasks 或执行 baseline 验证
- **THEN** 系统必须能够对 `E:\\project\\aLocal` 运行 `init`
- **THEN** 开发者必须将生成的 `.wiki` 快照写入 `tmp/` 下的对比目录
- **THEN** 开发者必须将该快照与 `tmp/reference-zh` 做结构和内容信号对照，并据此回调实现或测试
