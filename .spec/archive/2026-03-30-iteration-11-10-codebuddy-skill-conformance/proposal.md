## Why

当前 `spec-wiki init --tool codebuddy` 已经能生成 `.codebuddy/skills/*`、hooks 和 settings，但使用体验还有两个明显偏差：

- 生成出的 CodeBuddy skill 仍偏“命令说明书”，没有把 skill 的触发边界、工作流和 thin host boundary 说清楚。
- `wiki-query` 与相关 hook 文案没有明确要求宿主直接消费 runtime 的稳定结构化字段，仍给“再解释一层 query 结果”留了口子。

这轮只收口 CodeBuddy：把生成资产修正为更符合 skill 形态的工作流说明，并把 `query thin consumption` 固化到 UniSpec 和测试里。

## What Changes

- 为 CodeBuddy 增补正式约束：
  - generated skills 必须是 skill，而不是把 command 文案搬进 `SKILL.md`
  - `wiki-query` 必须直接消费 runtime 的稳定结构化字段
  - hooks 只能提示入口和边界，不能引导宿主重建 Wiki 业务语义
- 删除 CodeBuddy shared skill，隐藏 `sync` / `rebuild` skills，清理旧受管残留，并把共享规则改由 hooks 与 action skill 文案承载
- CodeBuddy 在 v0.1.0 只暴露 `init/status/query/update` 四个 action skills，不暴露 `sync/rebuild`
- 补强测试，验证生成内容是否符合上述边界

## Scope

- 只改 `CodeBuddy`
- 不改 `Codex / Claude`
- 不改 `wiki-runtime` 协议




