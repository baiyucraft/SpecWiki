## MODIFIED Requirements

### Requirement: 系统必须生成稳定的模块树
系统 MUST 基于归一化后的仓库扫描结果生成稳定的递归 `ModuleTree`，用于表达模块边界、父子层级、模块类型和模块包含的关键源码集合，而不是只生成根模块与一层子模块。系统 MUST 在模块提升阶段抑制单文件模块，不允许只含单个文件且无子模块的候选节点被提升为独立模块。系统 MUST 在模块提升评分中对 test / fixture 路径下的文件施加降权，避免测试产物主导模块发现。

#### Scenario: 从仓库结构生成模块树
- **WHEN** 系统完成仓库扫描并进入层级拆分阶段
- **THEN** 系统必须生成包含根模块和递归子模块的模块树
- **THEN** 每个模块必须具有稳定标识、名称、根路径、模块类型和关联源码集合
- **THEN** 每个非叶子模块必须能够导出稳定的 `child_ids`

#### Scenario: 重复扫描同一仓库
- **WHEN** 同一仓库在源码结构未发生影响模块边界的变化时被重复扫描
- **THEN** 系统必须保持模块发现顺序和父子关系稳定
- **THEN** 同一模块必须保持稳定标识

#### Scenario: 抑制单文件模块
- **WHEN** hierarchy 层发现一个候选模块根路径下只包含单个文件且没有子模块
- **THEN** 系统 MUST 不将该候选节点提升为独立模块
- **THEN** 该文件 MUST 被归入其最近的父模块

#### Scenario: 抑制非代码目录提升为模块
- **WHEN** hierarchy 层发现一个候选模块根路径对应的目录不包含任何源码文件（kind 为 `"source"` 的文件），只包含文档、配置或产物文件
- **THEN** 系统 MUST 不将该候选节点提升为独立模块

#### Scenario: test / fixture 文件不主导模块提升评分
- **WHEN** hierarchy 层对候选模块根路径进行提升评分，且该路径下的文件大部分带有 `"test-file"` 降权标记
- **THEN** 系统 MUST 在评分中对这些文件施加降权
- **THEN** 评分结果不得因 test / fixture 文件数量多而将该路径提升为高优先级模块

## ADDED Requirements

### Requirement: module kind 分类必须综合多维信号
系统 MUST 基于 manifest 类型、目录结构模式、入口文件、模块标签和文件组成的综合信号判断 module kind，而不是仅依赖文件扩展名。分类结果 MUST 覆盖以下 kind 值：`application`、`library`、`backend-service`、`frontend-app`、`infrastructure`、`cli-tool`、`workspace-member`、`module-group`、`module`。

#### Scenario: Rust 核心引擎被正确分类
- **WHEN** 一个模块包含 `Cargo.toml` manifest、`src/` 目录下有大量 `.rs` 源码文件、且 `Cargo.toml` 中声明了 `[lib]` section
- **THEN** 系统 MUST 将其 kind 分类为 `library`，而不是 `infrastructure`

#### Scenario: Agent 接入层被正确分类
- **WHEN** 一个模块包含 `package.json` manifest、`src/` 目录下有 `.ts` 源码文件、且模块标签包含 `"frontend"` 但目录名或 manifest 中包含 `agent` / `adapter` / `plugin` 关键词
- **THEN** 系统 MUST 将其 kind 分类为 `module`（或更精确的类型），而不是 `frontend-app`

#### Scenario: CLI 工具被正确分类
- **WHEN** 一个模块包含 `main` 入口文件（如 `src/main.rs`、`cmd/main.go`）且 manifest 中声明了 `[[bin]]` 或 `bin` 字段
- **THEN** 系统 MUST 将其 kind 分类为 `cli-tool` 或 `application`

#### Scenario: 基础设施模块被正确分类
- **WHEN** 一个模块主要包含 `Dockerfile`、`docker-compose.yml`、`nginx.conf`、shell 脚本等基础设施文件，且不包含应用源码
- **THEN** 系统 MUST 将其 kind 分类为 `infrastructure`

### Requirement: 关键源码选择必须优先反映核心实现
系统 MUST 在关键源码选择（`key_source_score`）中确保核心实现目录（如 `src/`、`lib/`、`core/`）下的文件优先于 test / fixture / docs 路径下的文件。test / fixture 路径下的文件 MUST 受到显式惩罚，确保关键源码列表反映模块的核心实现而非测试产物。

#### Scenario: 核心源码优先于 fixture 文件
- **WHEN** 一个模块同时包含 `src/domain/*.rs`（核心实现）和 `tests/fixtures/**/*.rs`（测试 fixture），且 fixture 文件数量远多于核心实现文件
- **THEN** 系统 MUST 在关键源码列表中优先选择 `src/domain/` 下的文件
- **THEN** `tests/fixtures/` 下的文件不得出现在关键源码列表的前 8 位

#### Scenario: test 文件不淹没核心源码
- **WHEN** 一个模块的 `tests/` 目录下有大量测试文件，而 `src/` 目录下有少量但关键的实现文件
- **THEN** 系统 MUST 确保 `src/` 下的实现文件在关键源码列表中排名高于 `tests/` 下的测试文件
