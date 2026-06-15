## 1. UniSpec 与模板边界

- [x] 1.1 更新本次 change 的 specs，明确共享 workflow template 层、CodeBuddy skill+hook 主链、Claude/Codex 模板复用边界
- [x] 1.2 复核注释与命名边界，确保宿主模板层、renderer 层和 orchestration 层职责清晰

## 2. 共享 workflow semantics 层

- [x] 2.1 新增宿主无关的 workflow semantics types/source，承载 action 元数据、shared runtime rules 和可复用 steps/guardrails
- [x] 2.2 让 Claude/Codex 宿主入口资产改为消费共享 workflow semantics 层，而不是继续直接依赖独立宿主 markdown 模板

## 3. CodeBuddy 主链切换

- [x] 3.1 新增 CodeBuddy 四个 action skill、hook script 与 settings patch 的 renderer / asset builder
- [x] 3.2 调整 `spec-wiki init --tool codebuddy` 的写盘逻辑，不再生成 `.codebuddy/commands/wiki/*.md`，改为生成 skills/hooks/settings
- [x] 3.3 确保 CodeBuddy hooks 只做 context/guardrail，不承担 action dispatch 或 bridge/session 编排
- [x] 3.4 定义 `.codebuddy/settings.json` 的 ownership / merge 规则，并保证重复执行时只刷新 `spec-wiki` 管理的条目

## 4. 测试与验收

- [x] 4.1 更新包级测试，覆盖 CodeBuddy 四个 action skill、hooks/settings 与 Claude/Codex 模板复用路径
- [x] 4.2 验证重复执行 `spec-wiki init --tool codebuddy` 时只刷新 `spec-wiki` 管理的 skills/hooks/settings，不覆盖无关宿主文件与用户自有 settings 项
- [x] 4.3 单独执行一轮 `.wiki/02-开发指南/00-代码注释规范.md` 合规检查，确认新增模板/renderer 注释符合规范

