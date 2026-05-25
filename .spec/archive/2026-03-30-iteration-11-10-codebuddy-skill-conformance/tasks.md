## 1. UniSpec

- [x] 1.1 为 `codebuddy-agent-integration` 补充 CodeBuddy skill 合规与 `query thin consumption` 的正式要求
- [x] 1.2 为 `global-cli-bootstrap` 补充 CodeBuddy hooks 只能做入口提示与边界提醒的要求

## 2. CodeBuddy 资产

- [x] 2.1 删除 CodeBuddy shared skill，隐藏 `sync` / `rebuild` skills，清理旧受管残留，并把共享边界回收到 hooks 与 action skills
- [x] 2.2 重写 `wiki-query` 等 action skill，明确 `query` 只消费 runtime 稳定结构化字段
- [x] 2.3 收紧 CodeBuddy hook 文案，避免宿主侧二次组织 Wiki 业务语义
- [x] 2.4 CodeBuddy v0.1.0 只暴露 `init/status/query/update`，不再生成 `sync/rebuild` skills

## 3. 测试与注释

- [x] 3.1 补充和更新测试，验证 skill frontmatter、`wiki-query` 消费边界与 hook 边界
- [x] 3.2 检查本轮新增和修改注释，确保符合 `COMMENTING.md`




