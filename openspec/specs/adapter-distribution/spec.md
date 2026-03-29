# adapter-distribution Specification

## Purpose
定义 `spec-wiki v0.1.0` 当前真实发布的单包分发合同。这个规范只描述当前 Windows x64 单包 staging/publish 形态，不再沿用旧的 optionalDependencies 平台包方案。

## Requirements
### Requirement: 主包清单必须从真实全局 CLI 包元数据生成
系统 MUST 从正式发布的全局 CLI 包元数据读取主包名称、版本和可执行入口信息来生成 staging 后的主包 manifest，而不是在 staging 脚本中维护独立常量副本。

#### Scenario: 生成主包清单
- **WHEN** 开发者执行 staging 流程
- **THEN** `dist/spec-wiki/package.json` MUST 使用 `packages/spec-wiki/package.json` 中的名称、版本和 `bin` 配置生成

### Requirement: `v0.1.0` staging 必须收敛为单个 Windows x64 主包并内置 runtime
系统 MUST 将 `v0.1.0` 的 staging 产物收敛为单个 `spec-wiki` 主包，并在主包内直接携带 Windows x64 runtime 二进制。staged manifest MUST 显式声明 `os=["win32"]` 与 `cpu=["x64"]`，而不是继续依赖 optionalDependencies 平台包分发。

#### Scenario: 生成当前版本 staging 产物
- **WHEN** 开发者执行当前版本 staging 流程
- **THEN** 系统 MUST 生成 `dist/spec-wiki/` 作为唯一 publish 目录
- **THEN** 系统 MUST 将 runtime 复制到 `dist/spec-wiki/lib/x64-win32/wiki-runtime.exe`
- **THEN** staged manifest MUST 显式写出 Windows x64 安装边界

#### Scenario: 当前版本主包不得继续声明平台包 optionalDependencies
- **WHEN** staging 流程完成
- **THEN** `dist/spec-wiki/package.json` MUST NOT 继续包含平台包 `optionalDependencies`
- **THEN** 当前发布说明 MUST 与这一单包分发形态保持一致