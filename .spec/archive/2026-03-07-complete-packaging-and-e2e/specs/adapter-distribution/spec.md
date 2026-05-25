## ADDED Requirements

### Requirement: 主包清单必须从真实适配层包元数据生成
系统 MUST 从 `packages/codebuddy/package.json` 读取主包名称、版本和可执行入口信息来生成 staging 后的主包 manifest，而不是在 staging 脚本中维护独立常量副本。

#### Scenario: 生成主包清单
- **WHEN** 开发者执行平台包 staging 流程
- **THEN** `dist/npm/<main-package>/package.json` 必须使用适配层包中的名称、版本和 bin 配置生成

#### Scenario: 更新版本后重新 staging
- **WHEN** 适配层包版本发生变化并重新执行 staging
- **THEN** 新生成的主包 manifest 必须反映最新版本，而不需要手动修改 staging 脚本常量

### Requirement: 平台包名称和二进制复制目标必须可由当前平台推导
系统 MUST 根据当前运行平台生成稳定的平台包目录名，并将对应的 Rust core 二进制复制到该平台包目录中，使主包能够通过 `optionalDependencies` 指向正确的平台包。

#### Scenario: 在当前平台生成 staging 产物
- **WHEN** 开发者在任意受支持平台执行 staging 流程
- **THEN** 系统必须生成与当前平台匹配的平台包目录和 manifest，并复制对应的 core 二进制文件

#### Scenario: 主包引用平台包
- **WHEN** staging 流程完成
- **THEN** 主包 manifest 中的 `optionalDependencies` 必须包含当前平台包名和与主包一致的版本
