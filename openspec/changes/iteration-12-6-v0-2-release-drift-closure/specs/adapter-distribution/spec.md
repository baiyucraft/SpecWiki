## RENAMED Requirements

### FROM: `v0.1.0` staging 必须收敛为单个 Windows x64 主包并内置 runtime
### TO: 当前正式 staging 必须收敛为单个 Windows x64 主包并内置 runtime

## MODIFIED Requirements

### Requirement: `v0.1.0` staging 必须收敛为单个 Windows x64 主包并内置 runtime
系统 MUST 将当前正式发布的 staging 产物收敛为单个 `spec-wiki` 主包，并在主包内直接携带 Windows x64 runtime 二进制。staged manifest MUST 显式声明 `os=["win32"]` 与 `cpu=["x64"]`，而不是继续依赖 optionalDependencies 平台包分发。当前 staged README、release 说明与主包版本信息 MUST 与这一单包分发形态保持一致，不得继续残留旧版本合同或旧 runtime 叙述。

#### Scenario: 生成当前版本 staging 产物
- **WHEN** 开发者执行当前版本 staging 流程
- **THEN** 系统 MUST 生成 `dist/spec-wiki/` 作为唯一 publish 目录
- **THEN** 系统 MUST 将 runtime 复制到 `dist/spec-wiki/lib/x64-win32/wiki-runtime.exe`
- **THEN** staged manifest MUST 显式写出 Windows x64 安装边界

#### Scenario: 当前版本主包不得继续声明平台包 optionalDependencies
- **WHEN** staging 流程完成
- **THEN** `dist/spec-wiki/package.json` MUST NOT 继续包含平台包 `optionalDependencies`
- **THEN** 当前发布说明 MUST 与这一单包分发形态保持一致

#### Scenario: staged README 与版本叙事保持一致
- **WHEN** 开发者完成当前版本 staging
- **THEN** `dist/spec-wiki/README.md` MUST 与当前正式 release 口径一致
- **THEN** staged README MUST NOT 继续描述旧版本的公开动作或旧 runtime contract
