## MODIFIED Requirements

### Requirement: 主包清单必须从真实适配层包元数据生成
系统 MUST 从新的全局 CLI 包元数据读取主包名称、版本和可执行入口信息来生成 staging 后的主包 manifest，而不是继续从 `agents/codebuddy/package.json` 或 staging 脚本常量维护这些信息。该主包元数据来源 MUST 是正式发布的 `spec-wiki` CLI 包；在 11.6 迁移完成后，该正式源码来源 MUST 收敛到 `packages/spec-wiki/package.json`，而不得继续停留在过渡目录。

#### Scenario: 生成全局 CLI 主包清单
- **WHEN** 开发者执行平台包 staging 流程
- **THEN** `dist/npm/spec-wiki/package.json` 必须从全局 CLI 包的真实 manifest 生成
- **THEN** 生成结果 MUST 反映全局 `spec-wiki` 的名称、版本和 `bin` 入口

#### Scenario: 更新全局 CLI 版本后重新 staging
- **WHEN** 全局 CLI 包版本发生变化并重新执行 staging
- **THEN** 新生成的主包 manifest 必须反映最新版本
- **THEN** staging 脚本 MUST 不需要再手动维护另一份主包版本常量

#### Scenario: 正式主包源码来源收敛到 packages 目录
- **WHEN** 开发者完成 11.6 的目录迁移并重新执行 staging
- **THEN** `dist/npm/spec-wiki/package.json` 的元数据来源 MUST 是 `packages/spec-wiki/package.json`
- **THEN** staging 流程 MUST 不得继续从 `agents/spec-wiki` 或其他过渡目录读取正式发布包身份

### Requirement: 平台包名称和二进制复制目标必须可由当前平台推导
系统 MUST 根据当前运行平台生成稳定的平台包目录名，并将对应的 Rust runtime 二进制复制到该平台包目录中，使全局 `spec-wiki` 主包能够通过 `optionalDependencies` 指向正确的平台包。

#### Scenario: 在当前平台生成全局 CLI staging 产物
- **WHEN** 开发者在任意当前受支持平台执行 staging 流程
- **THEN** 系统必须生成与当前平台匹配的 `spec-wiki-<platform>` 平台包目录和 manifest
- **THEN** 系统必须复制对应的 `wiki-runtime` 二进制文件到该平台包目录中

#### Scenario: 全局主包引用平台包
- **WHEN** staging 流程完成
- **THEN** 全局 `spec-wiki` 主包 manifest 中的 `optionalDependencies` 必须包含当前平台包名
- **THEN** 该平台包版本必须与全局主包版本保持一致

## ADDED Requirements

### Requirement: 全局 CLI 主包 staging 必须包含 bootstrap 所需发布资产
系统 MUST 在全局 `spec-wiki` 主包 staging 结果中包含 CLI 可执行入口、运行时调用逻辑以及宿主 bootstrap 模板资产，使已发布主包在安装后即可执行 `spec-wiki init` 与 `spec-wiki wiki <action>`，而不依赖工作区源码目录。

#### Scenario: staging 结果包含 CLI 与模板资产
- **WHEN** 开发者完成全局 `spec-wiki` staging
- **THEN** 主包目录 MUST 包含 `spec-wiki` 的 bin 入口和构建后的 CLI bundle
- **THEN** 主包目录 MUST 包含宿主 bootstrap 所需的模板资产

#### Scenario: 已发布主包不依赖工作区脚本目录
- **WHEN** 用户通过 npm 安装已发布的 `spec-wiki` 主包
- **THEN** 已安装主包 MUST 能独立完成 `spec-wiki init`
- **THEN** 主包 MUST 不依赖工作区中的 `scripts/*.mjs` 或未发布源码目录才能生成宿主资产

### Requirement: 工作区编排 manifest 必须与正式发布包身份解耦
系统 MUST 确保工作区根 manifest 不会与正式发布的 `spec-wiki` 主包共享同一个 package identity。工作区编排壳与正式发布包 MUST 有且仅有一个 `spec-wiki` 真相来源，以避免 workspace 包名冲突或发布身份混淆。

#### Scenario: 新增正式发布包时消除重名冲突
- **WHEN** 仓库新增正式发布的 `spec-wiki` 全局 CLI 包
- **THEN** 根 private workspace manifest MUST 不再占用同一个公开包名
- **THEN** 工作区中 MUST 只有新的正式发布包作为 `spec-wiki` 的主包真相来源
