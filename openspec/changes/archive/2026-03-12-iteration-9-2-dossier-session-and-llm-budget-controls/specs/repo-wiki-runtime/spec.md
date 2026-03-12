## ADDED Requirements

### Requirement: runtime 必须持久化 dossier、child rollup 与 session 摘要缓存
系统 MUST 在现有 runtime/state/cache 主链内持久化 dossier、child rollup 和 research session 摘要缓存，而不是新增 `.wiki/` 之外的 sidecar 层。相关 identity/hash MUST 可被 `update`、`rebuild` 和 cache 命中逻辑复用。

#### Scenario: dossier 与 child rollup 进入现有 cache/state
- **WHEN** workflow 完成某个页面的 dossier 组装或 child rollup 计算
- **THEN** 系统 MUST 把对应 identity、input hash 和必要摘要写入现有 runtime/cache 主链
- **THEN** 后续 `update` MUST 能基于这些缓存判断是否需要重建父页

#### Scenario: session 摘要缓存复用现有 runtime contract
- **WHEN** research session 生成 `session_summary`、`recent_turns` 或 `tool_artifact_refs`
- **THEN** 系统 MUST 在现有 cache/state contract 内持久化可复用摘要
- **THEN** 系统不得为此新增独立正式 runtime 目录

### Requirement: LLM cache 生命周期必须与 runtime 清理解耦
系统 MUST 让 LLM cache 生命周期独立于普通 runtime 清理。`init`、`rebuild` 默认不得隐式清空 LLM cache；当用户显式要求 cold-start 或 cache mode 为 `clear`/`refresh` 时，系统才 MAY 清空或失效对应缓存。

#### Scenario: runtime 清理不隐式删除 LLM cache
- **WHEN** 系统为 `init` 或 `rebuild` 清理旧 runtime 产物
- **THEN** 普通模式下 LLM cache MUST 保持可复用
- **THEN** 同仓库重复运行不得因为 runtime 清理而总是冷启动

#### Scenario: cache mode 控制 cache 失效方式
- **WHEN** 当前 workflow 显式设置 `cache_mode = clear` 或 `refresh`
- **THEN** 系统 MUST 按配置清空或强制刷新相应缓存
- **THEN** 失效行为 MUST 能被 trace、progress 或 summary 识别
