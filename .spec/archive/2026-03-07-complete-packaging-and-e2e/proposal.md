## Why

当前仓库已经具备 Rust core、CodeBuddy adapter、基础扫描/生成/查询能力，但交付链路仍未闭合：npm 平台包暂时只覆盖最小样板，缺少稳定的二进制分发约定；同时还没有端到端验证和可操作文档，无法证明“安装后可运行、更新后可验证、发布时可复用”。现在补齐这两部分，可以把仓库从“架构样机”推进到“可验证交付物”。

## What Changes

- 完善 npm 打包与发布脚本，使主包与平台包的 staging 结果可重复生成，并从真实包信息推导清单而不是写死常量。
- 明确平台二进制分发约定，支持按当前平台生成对应包结构，并为后续多平台扩展保留稳定入口。
- 增加端到端测试，覆盖初始化 Wiki、更新、查询以及适配层调用 Rust core 的整条链路。
- 补充 README 与开发/发布说明，明确本地开发、打包验证、`.wiki/` 产物和发布流程。

## Capabilities

### New Capabilities
- `adapter-distribution`: 定义 CodeBuddy Wiki 主包与平台包的 staging、清单生成和二进制分发行为。
- `workflow-verification`: 定义从适配层调用 core 到生成 `.wiki/` 产物的端到端验证行为，以及对应的开发文档要求。

### Modified Capabilities

无。

## Impact

- 受影响代码：`scripts/`、`packages/codebuddy/`、`tests/`、`README.md`
- 受影响系统：npm 打包产物布局、平台二进制定位、Node 测试与端到端验证链路
- 依赖与工具：继续使用现有 Node/Rust 工具链，不引入新的运行时宿主
