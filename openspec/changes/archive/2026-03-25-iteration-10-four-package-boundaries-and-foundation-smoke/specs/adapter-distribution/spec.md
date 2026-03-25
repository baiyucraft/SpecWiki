## MODIFIED Requirements

### Requirement: 平台包名称和二进制复制目标必须可由当前平台推导
系统 MUST 根据当前运行平台生成稳定的平台包目录名，并将对应的 Rust runtime 二进制复制到该平台包目录中，使主包能够通过 `optionalDependencies` 指向正确的平台包。

#### Scenario: 在当前平台生成 staging 产物
- **WHEN** 开发者在任意受支持平台执行 staging 流程
- **THEN** 系统必须生成与当前平台匹配的平台包目录和 manifest，并复制对应的 `wiki-runtime` 二进制文件

#### Scenario: 主包引用平台包
- **WHEN** staging 流程完成
- **THEN** 主包 manifest 中的 `optionalDependencies` 必须包含当前平台包名和与主包一致的版本
