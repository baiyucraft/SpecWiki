## Context

11.6 建立的正式边界是：

- 顶层 bootstrap：`spec-wiki init`
- runtime forwarding：`spec-wiki wiki <action>`
- 包内分层：`agents / orchestration / runtime`

当前 CodeBuddy 的真实产物仍然是：

- 历史上曾使用 `.codebuddy/commands/wiki/*.md`
- `.codebuddy/skills/spec-wiki/SKILL.md`

而用户现在明确要求：

- CodeBuddy 主链改为 `skill + hook`
- 显式 skill 直接承担原来的显式 command 入口职责
- 模板源必须可复用，后续 Claude/Codex 也能消费

这意味着本轮不能只做一份新的 CodeBuddy markdown 模板，而要先补一层宿主无关的 workflow semantics source。

## Goals / Non-Goals

### Goals

- 为宿主资产生成建立共享 workflow semantics 层，作为单一真相来源
- 让 CodeBuddy 主链切到显式 action skills + hooks/settings
- 让 Claude/Codex 也消费同一份 workflow semantics，并最终收敛到 repo 内 skill 形态
- 保持宿主层 thin boundary，不把 runtime 业务规则搬进 skills/hooks

### Non-Goals

- Claude/Codex 的最终宿主入口形态已经在后续变更中收敛为 skill-only，这里不再单独讨论迁移策略
- 本轮不把 `bridge/session` 协作抽进 CodeBuddy hooks
- 本轮不扩大 `v0.1.0` 正式保证范围
- 本轮不发明新的跨宿主 hook 通用框架

## Decisions

### 决策 1：共享 workflow semantics 层成为宿主资产生成的单一真相

当前的 `renderCommandFromTemplate("commands/codebuddy.md", action)` 只适合单文件 markdown 生成，不适合同时喂给：

- CodeBuddy action skill
- CodeBuddy hook script
- CodeBuddy settings patch
- Claude/Codex action skill

因此本轮新增一层宿主无关的 workflow semantics source，至少承载：

- action 元数据
- 对应 CLI 调用
- action-specific notes
- shared runtime rules
- host-agnostic steps / output / guardrails 文本片段

宿主最终产物的 frontmatter、文件路径都由 renderer 决定，而不是写回共享语义层。CodeBuddy hook 生命周期和 `.codebuddy/settings.json` 的 merge 结构也不进入共享层。

### 决策 2：CodeBuddy 主链改为 action skills + hooks

CodeBuddy 的显式入口不再写入 `.codebuddy/commands/wiki/*.md`，而是改为：

- `.codebuddy/skills/wiki-init/SKILL.md`
- `.codebuddy/skills/wiki-status/SKILL.md`
- `.codebuddy/skills/wiki-update/SKILL.md`
- `.codebuddy/skills/wiki-query/SKILL.md`
- `.codebuddy/hooks/spec-wiki/*.mjs`
- `.codebuddy/settings.json`

其中：

- action skills 承担显式入口
- hooks 与 action skill guardrails 一起承担共同规则
- hooks 只负责上下文注入与轻护栏

### 决策 3：Claude/Codex 也消费共享语义层，并收敛到 repo 内 skill

为了保证模板复用性，本轮必须让 Claude/Codex 也消费共享 workflow semantics 层。

但为了控制范围：

- Claude 输出 `.claude/skills/wiki-*/SKILL.md`
- Codex 输出 `.codex/skills/wiki-*/SKILL.md`

也就是说，本轮复用的是“模板源”，并且最终宿主入口形式也已经收敛为 skill。

### 决策 4：CodeBuddy 显式 action skill 只覆盖四个正式宿主入口

为了与当前 `v0.1.0` 宿主暴露边界一致，本轮显式 action skill 只覆盖：`init`、`status`、`update`、`query`。

其中 `v0.1.0` 的正式发布边界仍然只保证 index-only 的 `init / update / query`；`status` 可以继续作为辅助检查入口，但不得被描述成完整 knowledge/page runtime 已正式支持。

### 决策 5：hooks 只做 context/guardrail，不承担宿主业务编排

本轮 hooks 的职责上限是：

- `SessionStart` 注入 repo/runtime 背景
- `UserPromptSubmit` 注入优先使用 `wiki-status/wiki-query` 的提示

hooks 不负责：

- 直接替用户 dispatch `spec-wiki wiki <action>`
- 编排多步状态机
- 实现 bridge/session
- 重写 runtime 错误恢复逻辑

### 决策 6：CodeBuddy settings 必须有明确 ownership / merge 模型

由于本轮要写 `.codebuddy/settings.json`，系统必须显式区分：

- `spec-wiki` 自己管理的 hook 配置
- 用户原有配置
- 其他工具已有配置

本轮 merge 原则应为：

- 只写入 `spec-wiki` 自己的命名空间项
- 重复执行时只刷新 `spec-wiki` 自己管理的条目
- 不覆盖不属于 `spec-wiki` 命名空间的现有配置

## Risks / Trade-offs

- [风险：共享模板层抽象过厚]
  - 如果把所有宿主差异都提前塞进共享模板对象，会让 renderer 失去边界，反而更难复用
- [风险：CodeBuddy 主链切换破坏现有 contract]
  - 如果动作面不完整，或文案继续暗示旧 `/wiki:*` commands 是主入口，用户会看到不一致 contract
- [风险：hooks 越界为宿主业务编排]
  - 一旦把 action dispatch 或 session 语义写进 hooks，thin host boundary 会被破坏
- [风险：settings merge 污染用户已有配置]
  - 如果没有 ownership / merge 模型，重复执行 init 会污染 `.codebuddy/settings.json`
- [风险：Claude/Codex 复用只停留在常量抽取]
  - 如果它们仍继续直接依赖独立 markdown 模板，本轮的“复用性”就只是表面

## Migration Plan

1. 更新 UniSpec proposal/design/tasks/specs，明确新的 CodeBuddy 主链与共享语义层
2. 新增共享 workflow semantics types/source/render helpers
3. 将 Claude/Codex action skill 生成切到共享语义层
4. 新增 CodeBuddy skill/hook/settings 资产生成链，并定义 settings merge ownership
5. 调整 `runInit` 的 CodeBuddy 分支与 managed file 类型
6. 更新测试，覆盖四个 action skills、settings merge 幂等性与旧宿主复用边界

## Open Questions

- CodeBuddy action skill 的最终命名是 `wiki-init` 还是其他更短形式；当前先按 `wiki-<action>` 约束
- `.codebuddy/settings.json` 的 merge 策略是否允许保留用户原有 hooks；当前倾向是只追加 `spec-wiki` 命名空间项



