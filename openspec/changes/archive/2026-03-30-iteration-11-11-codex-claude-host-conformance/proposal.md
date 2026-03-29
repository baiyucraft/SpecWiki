## Why

当前 CodeBuddy 的宿主资产已经收敛到较清晰的 `v0.1.0` 边界：

- 只暴露正式可用的 action
- `query` 明确薄消费稳定结构化字段
- 宿主不重建 Wiki 状态机或页面语义
- action 文档骨架已经稳定在 `Purpose / Inputs / Steps / Output / Action Notes / Guardrails`

但 Claude 和 Codex 还停留在不同于 OpenSpec 自身习惯的宿主形态：

- 仍分别使用 command 和 prompt，而不是 skill
- 旧的 command / prompt 清理范围也不完整
- 宿主形式和 CodeBuddy 继续分叉

这会导致三个宿主的规范和使用体验继续分叉。

## What Changes

- 为 Claude / Codex 增补与当前 CodeBuddy 一致的宿主边界：
  - `v0.1.0` 只显式暴露 `init / status / query / update`
  - `query` 必须薄消费稳定结构化字段
  - 宿主不得重建 Wiki 状态机、页面语义或 knowledge/page projection
- 删除 Claude 的 command 生成与 Codex 的 prompt 生成，统一改为 repo 内 skill
- 把旧的 Claude command、Codex prompt 与旧 shared skill 目录都纳入清理
- 更新宿主正文与分发模板，使三类宿主都以 skill 形态表达当前边界

## Scope

- 改 `Claude / Codex` 宿主资产
- 修正 `global-cli-bootstrap` 的宿主真实落点表述
- 不改 `wiki-runtime` 协议
- 不扩大 `v0.1.0` 正式支持范围
