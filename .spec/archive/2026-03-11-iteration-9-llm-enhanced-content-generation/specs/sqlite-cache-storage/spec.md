## ADDED Requirements

### Requirement: SQLite 必须以可版本化的键模型承载 LLM 缓存
系统 MUST 使用 `.wiki/.cache/wiki-cache.db` 中的 `llm_cache` 表承载 LLM 辅助判断和页面增强结果。每条缓存记录 MUST 至少绑定 `prompt_type`、规范化输入哈希、prompt 版本、模型标识、响应内容和 TTL 语义；读取缓存时，系统 MUST 同时校验输入哈希、prompt 版本和模型标识，而不是只按单一 key 复用旧结果。缓存命中时，系统 MUST 直接复用结果并跳过实际 provider 调用。

#### Scenario: 相同输入命中缓存并跳过真实调用
- **WHEN** 某个 Uncertainty Gate 或页面增强请求的输入哈希、prompt 版本和模型标识与已有缓存完全一致
- **THEN** 系统 MUST 直接复用 `llm_cache` 中的结果
- **THEN** 本轮 workflow 不得再次向 Agent 发起对应的真实 LLM 请求

#### Scenario: prompt 版本或模型变化导致缓存失效
- **WHEN** 某个请求的输入事实未变，但 prompt 版本或模型标识发生变化
- **THEN** 系统 MUST 视为缓存未命中
- **THEN** 系统 MUST 生成新的缓存记录，而不是继续复用旧响应

#### Scenario: update 只失效受影响输入的缓存
- **WHEN** `update` 只影响部分页面、部分模块或少量模糊判断点
- **THEN** 系统 MUST 只对这些受影响输入对应的 LLM 缓存执行重算或替换
- **THEN** 与本轮变化无关的 LLM 缓存不得被整体清空
