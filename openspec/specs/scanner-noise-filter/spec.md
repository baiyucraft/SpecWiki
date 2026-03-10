## ADDED Requirements

### Requirement: Scanner 必须排除嵌套仓库目录
系统 MUST 在目录遍历阶段检测嵌套仓库（子目录包含 `.git` 目录，或同时包含 manifest 文件与源码目录的组合），并将其整体排除在模块发现之外。嵌套仓库内的文件不得出现在 `RepoFacts.files` 中，也不得参与模块树构建。

#### Scenario: 排除含 .git 的嵌套仓库
- **WHEN** scanner 遍历到一个子目录，该子目录内部包含 `.git` 目录
- **THEN** scanner MUST 跳过该子目录及其所有后代
- **THEN** 该子目录下的文件不得出现在 `RepoFacts.files` 中

#### Scenario: 排除含 manifest + src 组合的嵌套仓库
- **WHEN** scanner 遍历到一个子目录，该子目录同时包含独立 manifest 文件（如 `Cargo.toml` + `src/`、`package.json` + `src/` 或 `node_modules/`）且该子目录不属于当前仓库的 workspace 成员
- **THEN** scanner MUST 将其识别为嵌套仓库并跳过
- **THEN** 该子目录下的文件不得参与模块树构建

#### Scenario: 不误排除 workspace 成员
- **WHEN** scanner 遍历到一个子目录，该子目录包含 manifest 文件但属于当前仓库的 workspace 成员（如 Cargo workspace 的 member、pnpm workspace 的 package）
- **THEN** scanner MUST 保留该子目录，不将其视为嵌套仓库

### Requirement: Scanner 必须排除 fixture 和测试数据目录
系统 MUST 识别并排除 fixture、测试数据和示例数据目录，这些目录的内容不应参与模块发现和页面生成。排除规则 MUST 覆盖常见命名模式。

#### Scenario: 排除 fixture 目录
- **WHEN** scanner 遍历到名为 `fixtures`、`__fixtures__`、`test-data`、`testdata`、`test_data`、`mock-data`、`mocks`、`__mocks__` 的目录，或位于 `tests/fixtures/`、`test/fixtures/`、`spec/fixtures/` 路径下的目录
- **THEN** scanner MUST 跳过该目录及其所有后代
- **THEN** 该目录下的文件不得出现在 `RepoFacts.files` 中

#### Scenario: 不误排除顶层 test 目录中的真实测试源码
- **WHEN** scanner 遍历到 `tests/` 或 `test/` 目录，但该目录下包含真实测试源码文件（非 fixture 子目录）
- **THEN** scanner MUST 保留测试源码文件，只排除 fixture 子目录

### Requirement: Scanner 必须排除非代码产物目录
系统 MUST 排除已知的非代码产物目录，这些目录不包含参与模块树构建的源码。排除列表 MUST 可通过后续 steering 配置扩展。

#### Scenario: 排除已知非代码产物目录
- **WHEN** scanner 遍历到名为 `openspec`、`.github`、`.gitlab`、`.circleci`、`.husky`、`coverage`、`.nyc_output`、`vendor`（非 Go 仓库）的目录
- **THEN** scanner MUST 跳过该目录及其所有后代

#### Scenario: 保留 Go vendor 目录
- **WHEN** scanner 遍历到 `vendor/` 目录，且当前仓库的 tech_hints 表明这是一个 Go 仓库（存在 `go.mod`）
- **THEN** scanner MUST 保留该目录，不将其排除

### Requirement: Scanner 必须对 test 路径文件标记降权信号
系统 MUST 对位于 test / spec / fixture 相关路径下的文件标记降权信号，使其在后续关键源码选择和模块提升评分中被适当降权，而不是与核心实现源码同等对待。

#### Scenario: 标记 test 路径文件
- **WHEN** scanner 处理位于 `tests/`、`test/`、`spec/`、`__tests__/`、`__test__/` 路径下的源码文件
- **THEN** scanner MUST 在该文件的 tags 中添加降权标记（如 `"test-file"`）
- **THEN** 该标记 MUST 能被 hierarchy 和 generation 层消费
