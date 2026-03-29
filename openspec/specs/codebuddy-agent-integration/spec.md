# codebuddy-agent-integration Specification

## Purpose
定义 `spec-wiki v0.1.0` 中 CodeBuddy 宿主接入层的真实公开合同。这个规范只覆盖 CodeBuddy 侧的 bootstrap 资产、公开 action skills 和 runtime 桥接边界，不把旧版六工具或单宿主阶段性方案继续当作正式发布合同。

## Requirements
### Requirement: CodeBuddy 必须作为当前三宿主之一接入，而不是唯一宿主
系统 MUST 将 CodeBuddy 视为 `v0.1.0` 当前支持的宿主之一，并与 Claude、Codex 一起共享同一套公开 workflow 收敛边界。CodeBuddy 集成当前只承诺 Windows x64 打包运行时；该规范 MUST NOT 再把“只有 CodeBuddy、没有其他宿主”描述为当前版本事实。

#### Scenario: 通过 bootstrap 生成 CodeBuddy 资产
- **WHEN** 用户执行 `spec-wiki init --tool codebuddy`
- **THEN** 系统 MUST 为 CodeBuddy 写入受管宿主资产
- **THEN** 该资产边界 MUST 与当前版本其他宿主的公开 workflow 合同一致

### Requirement: CodeBuddy 当前公开 action skills 必须只暴露 `wiki-init`、`wiki-status`、`wiki-update`、`wiki-query`
CodeBuddy 当前版本 MUST 只公开 `wiki-init`、`wiki-status`、`wiki-update`、`wiki-query` 四个 action skills。`wiki-sync` 与 `wiki-rebuild` MAY 作为内部历史残留、测试对象或后续迭代能力存在，但 MUST NOT 被当前版本公开暴露为正式 CodeBuddy action skills。

#### Scenario: 生成 CodeBuddy action skills
- **WHEN** 系统为当前版本输出 CodeBuddy skills
- **THEN** 公开 action skill 列表 MUST 只包含 `wiki-init`、`wiki-status`、`wiki-update`、`wiki-query`
- **THEN** 生成结果不得把 `wiki-sync` 或 `wiki-rebuild` 当成当前版本正式入口

### Requirement: CodeBuddy 必须保持 thin host boundary
CodeBuddy MUST 通过本地进程调用 `wiki-runtime`，并只负责参数收集、binary 定位、事件/结果解析、错误透传和必要上下文注入。CodeBuddy MUST NOT 在宿主层重建 Wiki 状态机、页面语义或 knowledge/page projection。

#### Scenario: 调用 runtime
- **WHEN** 任一 CodeBuddy Wiki skill 被调用
- **THEN** CodeBuddy MUST 将请求转换为 runtime 可识别的命令并调用本地 binary
- **THEN** CodeBuddy MUST 直接薄消费 runtime 返回值，而不是在宿主层重写 Wiki 业务语义

#### Scenario: runtime 返回错误
- **WHEN** `wiki-runtime` 返回失败响应
- **THEN** CodeBuddy MUST 向宿主返回明确错误
- **THEN** 宿主不得静默改写当前 Wiki 业务状态

### Requirement: CodeBuddy 对长流程只桥接 `init` 与 `update` 的流式事件
CodeBuddy 在调用当前版本正式公开的长流程 action 时，MUST 直接消费 `progress / result / error` 事件流，并在不扩展业务语义的前提下转交给宿主。当前版本该要求只正式覆盖 `init` 与 `update`；`status` 与 `query` 继续按短流程 JSON 处理即可。

#### Scenario: 宿主订阅长流程事件
- **WHEN** CodeBuddy 调用 `wiki-init` 或 `wiki-update`，且宿主提供事件消费桥接
- **THEN** CodeBuddy MUST 按到达顺序透传 `progress` 事件
- **THEN** CodeBuddy MUST 继续返回最终 `result` 或 `error`

### Requirement: CodeBuddy 可选桥接 LLM 请求，但不得改变当前公开 workflow 合同
CodeBuddy 在长流程中 MAY 桥接 `llm_request` 事件，但这种桥接只负责协议与 provider 调用，不负责改变当前版本的公开 workflow 合同。无论桥接是否可用，宿主都 MUST 不把 `rebuild`、完整 knowledge/page 生成或宿主侧二次状态机解释成 `v0.1.0` 正式支持面。

#### Scenario: LLM bridge 不可用时显式回退
- **WHEN** CodeBuddy 收到 `llm_request`，但当前宿主或 provider 不可用
- **THEN** CodeBuddy MUST 明确回写不可用或空响应，让 runtime 自行决定后续回退路径
- **THEN** CodeBuddy MUST 继续保持 thin host boundary