## MODIFIED Requirements

### Requirement: 系统必须支持 repo 级 steering 配置文件
系统 MUST 支持从 `.wiki/wiki.steering.yaml` 读取 steering 配置。配置文件 MUST 使用 `version` 字段标识 schema 版本，并支持嵌套的 `scan` 配置块。`scan` 块 MUST 至少支持 `ignore` 和 `include` 两个字段：`scan.ignore` 用于追加忽略路径，`scan.include` 用于显式恢复原本会被忽略的路径。当配置文件不存在时，系统 MUST 使用合理默认值正常运行，不得阻塞 pipeline 或输出 warning。

#### Scenario: 读取存在的 steering 配置
- **WHEN** `.wiki/wiki.steering.yaml` 存在且格式合法
- **THEN** 系统 MUST 解析配置并在 scanner 与 planner 阶段消费
- **THEN** 配置中的 `scan.ignore` 和 `scan.include` 字段 MUST 按 schema 定义生效

#### Scenario: steering 配置不存在
- **WHEN** `.wiki/wiki.steering.yaml` 不存在
- **THEN** 系统 MUST 使用默认值（空忽略列表、空 include 列表、无提升/降级、默认合并阈值）
- **THEN** pipeline MUST 正常完成，行为与未引入 steering 时一致

#### Scenario: steering 配置格式非法
- **WHEN** `.wiki/wiki.steering.yaml` 存在但格式不合法（YAML 语法错误或字段类型不匹配）
- **THEN** 系统 MUST 输出明确的解析错误信息
- **THEN** 系统 MUST 回退到默认值继续运行，不得中断 pipeline

## REMOVED Requirements

### Requirement: steering 配置必须支持全局和按语言两层忽略路径
**Reason**: `.wiki/06-设计文档/00-总体设计.md` 已将扫描侧 steering 结构收敛到 `scan.ignore` / `scan.include`，旧的 `ignore.global` 与 `ignore.<language>` 结构不再是长期事实模型。
**Migration**: 现有仓库应把旧的 `ignore.global` / `ignore.<language>` 配置迁移为 `scan.ignore`，需要强制纳入扫描的路径迁移为 `scan.include`。兼容读取仅作为过渡行为，不构成长期 contract。

## ADDED Requirements

### Requirement: steering 配置必须支持 scan.ignore 与 scan.include
系统 MUST 允许用户通过 steering 配置的 `scan.ignore` 与 `scan.include` 声明额外的扫描边界。`scan.ignore` MUST 追加到 scanner 内置忽略规则之上；`scan.include` MUST 作为白名单覆盖同轮扫描中的忽略结果。两者 MUST 在扫描阶段生效，并被 `init`、`update`、`sync`、`rebuild` 统一消费。

#### Scenario: 配置 scan.ignore
- **WHEN** steering 配置中声明 `scan.ignore: [\"docs/**\", \"examples/**\"]`
- **THEN** scanner MUST 在内置排除规则之外额外排除匹配这些 glob 的文件和目录
- **THEN** 被排除的文件不得出现在 `ScanReport.files` 中

#### Scenario: 配置 scan.include 恢复被忽略路径
- **WHEN** 某路径同时命中内置忽略规则或 `scan.ignore`
- **AND** steering 配置中声明 `scan.include` 包含该路径
- **THEN** scanner MUST 恢复该路径下的文件参与扫描
- **THEN** 这些文件 MUST 正常出现在 `ScanReport.files` 中

#### Scenario: init 与 update 共享同一 steering 扫描边界
- **WHEN** 用户修改 `scan.ignore` 或 `scan.include` 后执行 `init`、`update`、`sync` 或 `rebuild`
- **THEN** 系统 MUST 在该次 workflow 中按新配置重新执行扫描边界判断
- **THEN** 新旧扫描边界差异 MUST 能反映到后续的模块树、页面规划和 change_set 结果中
