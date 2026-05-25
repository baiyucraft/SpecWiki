## ADDED Requirements

### Requirement: 系统必须提供端到端验证适配层到 Wiki 产物的主链路
系统 MUST 提供自动化测试，验证适配层调用 Rust core 后能够在目标仓库完成 Wiki 初始化、检测更新并执行查询，且生成的 `.wiki/` 产物与返回结果符合预期。

#### Scenario: 初始化并查询 Wiki
- **WHEN** 端到端测试在临时 Git 仓库中执行 `init` 后再执行 `query`
- **THEN** 测试必须观察到 `.wiki/` 目录和 `wiki.metadata.json` 已生成，且查询结果包含匹配页面

#### Scenario: 修改源码后执行更新
- **WHEN** 端到端测试在初始化后修改目标仓库中的源码文件并执行 `update`
- **THEN** 测试必须得到非空的更新结果，证明当前公开更新链路可工作

### Requirement: 仓库必须提供可执行的开发与发布前说明
系统 MUST 在仓库文档中说明本地开发、测试、二进制准备、staging 验证和 `.wiki/` 产物约定，使开发者无需阅读设计文档即可完成基本验证。

#### Scenario: 新开发者阅读 README
- **WHEN** 开发者只参考仓库 README
- **THEN** 其必须能够找到本地测试入口、打包脚本入口、`.wiki/` 目录说明和发布前检查步骤

#### Scenario: 验证文档与脚本一致
- **WHEN** 开发者按照 README 中列出的命令执行本地验证
- **THEN** 文档引用的脚本和测试入口必须与仓库中的实际文件和命令保持一致
