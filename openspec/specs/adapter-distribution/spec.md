# adapter-distribution Specification

## Purpose
定义 `spec-wiki v0.2.0` 当前真实发布的单包分发合同。这个规范只描述当前 Windows x64 单包 staging/publish 形态，不再沿用旧的 optionalDependencies 平台包方案。

## Requirements
### Requirement: 主包清单必须从真实全局 CLI 包元数据生成
系统 MUST 从正式发布的全局 CLI 包元数据读取主包名称、版本和可执行入口信息来生成 staging 后的主包 manifest，而不是在 staging 脚本中维护独立常量副本。

#### Scenario: 生成主包清单
- **WHEN** 开发者执行 staging 流程
- **THEN** `dist/spec-wiki/package.json` MUST 使用 `packages/spec-wiki/package.json` 中的名称、版本和 `bin` 配置生成

### Requirement: 当前正式 staging 必须收敛为单个 Windows x64 主包并内置 runtime
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

### Requirement: staged package 必须提供可追溯的 publish 证据
系统 MUST 让 `dist/spec-wiki/` 提供可追溯到当前源码真相的 staged package 证据。当前版本的 package evidence MUST 至少覆盖 staged `package.json`、staged README、CLI/help 与主包版本之间的一致性检查，并通过 `npm publish --dry-run` 或等价方式证明当前 staged package 具备发布可行性。系统 MUST NOT 复用旧 staging、旧 release 文案或历史产物残留作为当前版本发布证据。

#### Scenario: staged package 通过 dry-run publish 验证
- **WHEN** 开发者为当前版本执行 staged package 验证
- **THEN** 系统 MUST 能对 `dist/spec-wiki/` 执行 `npm publish --dry-run` 或等价验证
- **THEN** 该验证 MUST 直接针对当前 staged package，而不是仓库根目录或旧产物目录

#### Scenario: staged truth source 一致性可追溯
- **WHEN** 当前版本 staging 流程完成
- **THEN** staged `package.json`、staged README 与 CLI/help MUST 能证明来自当前源码真相
- **THEN** 系统 MUST NOT 继续保留与当前版本冲突的旧 release 文案或旧动作列表
