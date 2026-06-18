## Context

完成 `12-6` 之后，仓库会在合同、README 与版本叙事层面收齐到 `v0.2.0`。但这还不足以证明“当前 staged package 确实来自当前源码真相，并且具备可发布证据”。当前仓库已经有：

- `scripts/build-dist.mjs`
- `scripts/publish-packages.mjs`
- `scripts/tests/distribution.test.ts`
- `workflow-verification` 的现有 gate 约束

因此本轮不应重新设计 packaging，而应围绕现有脚本与 gate 收口一套可复用的发布证据链。

## Goals / Non-Goals

**Goals:**
- 证明 `dist/spec-wiki/` 是从当前源码真相构建出的 staged package
- 为 `v0.2.0` 收口 package-level 证据：staged manifest、README、CLI/help、一致性检查与 publish dry-run
- 将 `storybook` primary gate、`chi + zustand` smoke、`packages/spec-wiki` 测试与 `.wiki/02-开发指南/00-代码注释规范.md` 检查收口为正式 release evidence
- 让 distribution / verification spec 明确这些证据是当前版本发布前置条件

**Non-Goals:**
- 不再修改 public workflow surface
- 不再修改 runtime capability 或知识主链语义
- 不重新设计 multi-platform packaging
- 不把运营 runbook、公告或外部渠道同步当成 capability change

## Decisions

### 1. 证据链直接建立在现有脚本之上
本轮优先复用 `build-dist.mjs`、`publish-packages.mjs` 与 `distribution.test.ts`，只补齐缺失的 evidence checks，而不是再开一套新的 packaging 编排。

### 2. release gate 继承现有规范，不自创新 gate
本轮显式继承当前已存在的 gate：

- `storybook` primary gate
- `chi + zustand` smoke
- `packages/spec-wiki` 自动化测试
- `.wiki/02-开发指南/00-代码注释规范.md` 检查

发布证据只负责把这些 gate 收口到 release 语境下，不重新发明另一套门禁。

### 3. staged package proof 至少包含 dry-run 与 truth-source consistency
本轮 package evidence 至少要证明：

- staged `package.json` 与源码主包版本一致
- staged README 与当前 release 口径一致
- staged CLI/help 与当前 public surface 一致
- `npm publish --dry-run` 或等价动作可在 staged package 上通过

### 4. 纯 GA runbook 保留在 `.docs`
如果某些动作只属于“发布操作步骤”而不改变 capability 或 contract，例如最终 checklist、外部发布步骤、公告模板，它们只进入 `.docs`，不进入 UniSpec capability change。

## Risks / Trade-offs

- [风险] 证据链过宽，重新滑回“大而全 packaging 设计”
  → Mitigation：所有任务都绑定现有脚本和 staged package 证明，不新增抽象层

- [风险] gate 报告与 publish dry-run 分散在多个位置，难以复用
  → Mitigation：要求在本轮输出统一的 release evidence 记录

- [风险] staged package 看起来对齐，但 help/README/version 仍可能存在细节差异
  → Mitigation：把 README/help/manifest 一致性当成正式验证项，而不是人工目检项
