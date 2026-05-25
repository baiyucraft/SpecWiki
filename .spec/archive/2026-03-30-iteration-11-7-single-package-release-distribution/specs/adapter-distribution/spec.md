## ADDED Requirements

### Requirement: 主包 staging 目录必须直接成为可发布单包
系统 MUST 将 `packages/spec-wiki` 的可发布产物收口到单一目录 `dist/spec-wiki`，并使该目录可直接执行 `npm publish`。

#### Scenario: 生成单包 staging 产物
- **WHEN** 开发者执行根级 build
- **THEN** 系统必须生成 `dist/spec-wiki/package.json`
- **AND** 该目录必须包含 `bin/**`、`dist/**`、`assets/**`
- **AND** 不再生成依赖平台包的 `optionalDependencies`

### Requirement: Windows runtime binary 必须随主包一起发布
系统 MUST 将当前 Windows runtime binary 复制到主包内的 `lib/x64-win32/wiki-runtime.exe`，供 CLI 在安装后直接解析和调用。

#### Scenario: 发布包包含 Windows runtime
- **WHEN** staging 流程完成
- **THEN** `dist/spec-wiki/lib/x64-win32/wiki-runtime.exe` 必须存在
- **AND** `resolveBinary()` 必须能够优先解析该路径

### Requirement: 发布构建必须使用 release profile
系统 MUST 使用 release profile 构建 `wiki-runtime` 发布 binary，而不是 debug profile。

#### Scenario: 根级 build 生成 runtime binary
- **WHEN** 开发者执行 `pnpm run build`
- **THEN** staging 脚本必须从 release profile 收集 `wiki-runtime` binary
