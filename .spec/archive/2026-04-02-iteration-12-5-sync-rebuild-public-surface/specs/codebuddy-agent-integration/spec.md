## MODIFIED Requirements

### Requirement: CodeBuddy 当前公开 action skills 必须只暴露 `wiki-init`、`wiki-status`、`wiki-update`、`wiki-query`
CodeBuddy 当前版本 MUST 公开 `wiki-init`、`wiki-status`、`wiki-update`、`wiki-query`、`wiki-sync`、`wiki-rebuild` 六个 action skills。`wiki-sync` 与 `wiki-rebuild` 既然已被纳入公开 workflow surface，就不得继续被描述为内部历史残留、测试对象或未来迭代入口。

#### Scenario: 生成 CodeBuddy action skills
- **WHEN** 系统为当前版本输出 CodeBuddy skills
- **THEN** 公开 action skill 列表 MUST 包含 `wiki-sync` 与 `wiki-rebuild`
- **THEN** 生成结果不得继续把两者标记为非正式入口

### Requirement: CodeBuddy 对长流程只桥接 `init` 与 `update` 的流式事件
CodeBuddy 在调用当前版本正式公开的长流程 action 时，MUST 直接消费 `progress / result / error` 事件流，并在不扩展业务语义的前提下转交给宿主。当前版本该要求正式覆盖 `init`、`update` 与 `rebuild`；`status`、`query` 与 `sync` 继续按短流程 JSON 处理即可。

#### Scenario: 宿主订阅长流程事件
- **WHEN** CodeBuddy 调用 `wiki-rebuild`，且宿主提供事件消费桥接
- **THEN** CodeBuddy MUST 按到达顺序透传 `progress` 事件
- **THEN** CodeBuddy MUST 继续返回最终 `result` 或 `error`
