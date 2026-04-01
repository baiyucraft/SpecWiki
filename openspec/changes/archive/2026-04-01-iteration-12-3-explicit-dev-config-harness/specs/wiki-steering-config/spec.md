## MODIFIED Requirements

### Requirement: 系统必须支持 repo 级 steering 配置文件
系统 MUST 支持从 `.wiki/config.yaml` 读取 repo 级 steering 配置。配置文件 MUST 使用 `version` 字段标识 schema 版本，并支持嵌套的 `scan` 配置块。`scan` 块 MUST 至少支持 `ignore` 和 `include` 两个字段：`scan.ignore` 用于追加忽略路径，`scan.include` 用于显式恢复原本会被忽略的路径。当配置文件不存在时，系统 MUST 使用合理默认值正常运行，不得阻塞 pipeline 或输出 warning。

#### Scenario: 默认配置优先级按 user < repo 叠加
- **WHEN** `~/.spec-wiki/config.yaml` 和 `.wiki/config.yaml` 同时存在，且未开启显式开发模式
- **THEN** 系统 MUST 先读取用户级默认配置，再叠加 repo 级共享配置
- **THEN** 最终生效值 MUST 满足 `~/.spec-wiki/config.yaml < .wiki/config.yaml`

#### Scenario: 显式开发模式下再叠加 dev 配置
- **WHEN** `~/.spec-wiki/config.yaml`、`.wiki/config.yaml` 和 `wiki.dev.yaml` 同时存在，且显式开发模式已开启
- **THEN** 系统 MUST 先读取用户级默认配置，再叠加 repo 级共享配置，最后叠加 `wiki.dev.yaml`
- **THEN** 最终生效值 MUST 满足 `~/.spec-wiki/config.yaml < .wiki/config.yaml < wiki.dev.yaml`

#### Scenario: init 默认生成用户配置模板
- **WHEN** 用户首次执行 `init` 且 `~/.spec-wiki/config.yaml` 不存在
- **THEN** 系统 MUST 自动创建 `~/.spec-wiki/config.yaml`
- **THEN** 该模板 MUST 保持最小且合法，不得写入会改变默认运行语义的重默认值

#### Scenario: init 对用户配置做严格语法校验
- **WHEN** 用户执行 `init` 且 `~/.spec-wiki/config.yaml` 存在但 YAML 非法
- **THEN** 系统 MUST 直接返回解析错误并终止 `init`
- **THEN** 该严格校验只要求覆盖 `init`，其它 workflow MAY 保持容错加载

### Requirement: 本地 dev 配置必须允许覆盖 provider 直连参数
系统 MUST 允许通过 repo 根的 `wiki.dev.yaml` 为本地开发环境覆盖 LLM provider 直连参数。provider 配置 MUST 采用 `llm.providers.<provider>.models.<model>` 的两级结构，顶层 `llm.model` MUST 使用 `provider/model` 选择具体模型。只有在显式开发模式开启时，`wiki.dev.yaml` 的加载优先级才 MUST 高于 `.wiki/config.yaml` 中对应的 `llm` 字段；未开启时系统 MUST 忽略该文件，即使文件存在。它只服务本地 dev 调试，不得被写入 runtime 页面、metadata 或 `.wiki/.cache` 之外的正式产物。`wiki.dev.yaml` 不得放在 `.wiki/` 目录下，以避免被 init/rebuild 的 runtime 清理删除。

#### Scenario: 未开启开发模式时忽略 dev 配置文件
- **WHEN** repo 根存在 `wiki.dev.yaml`，但未开启显式开发模式
- **THEN** 系统 MUST 忽略该文件
- **THEN** workflow MUST 只使用 `~/.spec-wiki/config.yaml`、`.wiki/config.yaml` 与默认值