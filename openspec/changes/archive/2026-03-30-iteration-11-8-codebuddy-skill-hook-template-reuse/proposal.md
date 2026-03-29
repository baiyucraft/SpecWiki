## Why

当前 `spec-wiki init` 的宿主模板生成仍然以“每个宿主各自持有命令/prompt 文案”作为主轴。这个结构对 CodeBuddy 新主链不够用，因为 CodeBuddy 现在要把显式入口切到 skills，并补 hooks；如果继续沿用当前按宿主散落的 markdown 模板，后续 Claude/Codex 也很难复用同一套 workflow 语义。

## What Changes

- 将宿主资产生成的单一真相来源提升为共享 workflow template 层，而不是继续直接从宿主 markdown 模板拼接
- 将 CodeBuddy bootstrap 主链从 `.codebuddy/commands/wiki/*.md` 切换为显式 action skills、以及项目级 hooks/settings
- 让 Claude/Codex 与 CodeBuddy 复用同一共享 workflow semantics 层，并把宿主入口统一收敛到 repo 内 skill
- 明确 CodeBuddy 仍保持 thin host boundary，不在 hooks 或 skills 中重写 Wiki 业务语义

## Capabilities

- **Modified Capabilities**
  - `global-cli-bootstrap`
  - `codebuddy-agent-integration`

## Impact

- `packages/spec-wiki/src/agents/**`
- `packages/spec-wiki/src/orchestration/init/**`
- `packages/spec-wiki/assets/**`
- `packages/spec-wiki/src/bootstrap.test.ts`
- 可能涉及 `cli` 文案与初始化报告输出


