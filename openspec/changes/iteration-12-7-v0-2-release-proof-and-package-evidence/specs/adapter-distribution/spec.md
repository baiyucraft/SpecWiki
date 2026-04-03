## ADDED Requirements

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
