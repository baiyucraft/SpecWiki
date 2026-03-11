## ADDED Requirements

### Requirement: steering 配置必须支持 LLM 增强控制项与页面提示
系统 MUST 允许用户通过 `.wiki/wiki.steering.yaml` 的 `llm` 配置块控制 LLM 增强行为。`llm` 配置块 MUST 至少支持是否启用内容增强、是否启用 Uncertainty Gate、单次 workflow 的最大真实调用次数，以及与图生成相关的开关或模式。系统还 MUST 允许通过 `pages.hints` 为不同页面类型追加提示语，并把这些提示作为页面增强输入的一部分。

#### Scenario: steering 显式关闭 LLM 增强
- **WHEN** steering 配置中声明关闭 LLM 内容增强或 Uncertainty Gate
- **THEN** 系统 MUST 跳过对应 LLM 阶段
- **THEN** workflow MUST 回退到纯 deterministic 行为，而不是尝试隐式调用 LLM

#### Scenario: steering 限制单次 workflow 的真实调用次数
- **WHEN** steering 配置中声明了最大真实调用次数
- **THEN** 系统 MUST 在达到该上限后停止发起新的真实 LLM 请求
- **THEN** 后续待处理项 MUST 回退到 deterministic 结果或缓存结果

#### Scenario: steering 配置 provider 直连并行度
- **WHEN** steering 或 `wiki.dev.yaml` 中声明了 `llm.parallel_requests`
- **THEN** 系统 MUST 仅把该值用作 provider 直连路径下的同层页面增强并行上限
- **THEN** 当值缺失、非法或小于 `1` 时，系统 MUST 回退到安全默认值而不是创建无上限并发

#### Scenario: pages.hints 参与页面增强输入
- **WHEN** steering 配置中为 `overview`、`architecture`、`module` 或 `workflow` 页面声明了提示项
- **THEN** 系统 MUST 把这些提示作为对应页面增强输入的一部分
- **THEN** 这些提示不得直接绕过事实层或改写页面身份

### Requirement: 本地 dev 配置必须允许覆盖 provider 直连参数
系统 MUST 允许通过 repo 根的 `wiki.dev.yaml` 为本地开发环境覆盖 LLM provider 直连参数。provider 配置 MUST 采用 `llm.providers.<provider>.models.<model>` 的两级结构，顶层 `llm.model` MUST 使用 `provider/model` 选择具体模型。`wiki.dev.yaml` 的加载优先级 MUST 高于 `.wiki/wiki.steering.yaml` 中对应的 `llm` 字段，但它只服务本地 dev 调试，不得被写入 runtime 页面、metadata 或 `.wiki/.cache` 之外的正式产物。`wiki.dev.yaml` 不得放在 `.wiki/` 目录下，以避免被 init/rebuild 的 runtime 清理删除。

#### Scenario: 本地 dev 配置覆盖共享 steering 的 provider 字段
- **WHEN** `.wiki/wiki.steering.yaml` 未声明 provider 直连参数，但 repo 根存在 `wiki.dev.yaml`
- **THEN** 系统 MUST 从 `wiki.dev.yaml` 读取并覆盖对应的 `llm.providers.*` 与 `llm.model` 配置
- **THEN** workflow MUST 使用覆盖后的本地 provider 配置参与 LLM 选择逻辑

#### Scenario: dev 配置缺失时保持共享 steering 行为
- **WHEN** repo 根不存在 `wiki.dev.yaml`
- **THEN** 系统 MUST 只使用 `.wiki/wiki.steering.yaml` 和默认值
- **THEN** workflow 行为不得因为缺少 dev 文件而报错

#### Scenario: 顶层 model 使用 provider/model 选择具体 provider 模型
- **WHEN** steering 或 `wiki.dev.yaml` 中声明 `llm.model = "proxy/gpt-5-mini"`
- **THEN** 系统 MUST 先解析出 `provider = proxy`、`model = gpt-5-mini`
- **THEN** 只有当 `llm.providers.proxy.models.gpt-5-mini` 存在时，core 才 MAY 直连该 provider
