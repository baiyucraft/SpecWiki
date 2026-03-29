# spec-wiki v0.1.0 版本说明

## 发布定位

`v0.1.0` 是当前阶段的收敛版发布，不追求完整的 Repo Wiki 生命周期能力，而是先把可稳定承诺的 `index-only` 主链发布出来。

这一版的目标不是“完整生成并维护 `.wiki/` 的所有知识层产物”，而是先稳定提供：

- `facts / index substrate`
- `index-first query`
- 统一的 `spec-wiki` CLI 与宿主 bootstrap

当前仍以 [DESIGN-3.0.md](E:/project/!byAI/spec-wiki/DESIGN-3.0.md)、[DESIGN-RUNTIME.md](E:/project/!byAI/spec-wiki/DESIGN-RUNTIME.md) 和 [DESIGN-AGENTS.md](E:/project/!byAI/spec-wiki/DESIGN-AGENTS.md) 作为长期目标边界；本说明只描述 `v0.1.0` 当前真实发布面。

## 正式支持范围

`v0.1.0` 正式保证的是 `index-only` 的：

- `spec-wiki wiki init`
- `spec-wiki wiki update`
- `spec-wiki wiki query`

这里的“正式保证”含义是：

- `init / update` 当前稳定落到 facts/index 持久化，不把完整 knowledge/page runtime 当作本版承诺
- `query` 当前稳定保证的是 `index-first` 结构化结果
- `.wiki/.knowledge/`、`.wiki/pages/`、`wiki.metadata.json` 仍属于目标 runtime 分层，但不是 `v0.1.0` 的发布 blocker

`spec-wiki wiki status` 在本版可以作为检查入口使用，帮助宿主判断 repo 当前是否 ready、stale 或需要下一步动作；但它不意味着 `v0.1.0` 已把完整 knowledge/page runtime 升级为正式支持面。

## 当前实现可见字段（非稳定合同）

`v0.1.0` 的 `query` 当前真实实现里，除稳定字段外，还可能返回这些实现可见字段：

- `matched_modules`
- `matched_sources`
- `matched_relations`
- `matched_symbols`
- `matched_symbol_edges`
- `matches`

说明：

- 这些字段反映的是当前 runtime 的实现状态，不等同于 `v0.1.0` 的正式稳定合同
- 宿主可以做面向人的薄转述，但不得基于这些字段建立新的宿主状态机、业务分支或固定协议
- 当前 `matched_processes` 与 `matched_communities` 仍是预留字段，不应被当成已正式可用能力
- 当前 `wiki-index` 内部虽然已有 `entrypoint / callers / callees / impact slice` 等 query substrate，但它们不等于 `v0.1.0` 已正式对外承诺的 query 结果

当前宿主可稳定依赖的 `query` 字段仍然只有：

- `term`
- `runtime_state`
- `query_mode`
- `query_trust`
- `recommended_action`
- `matched_pages`
- `provenance_summary`

## 当前未纳入 `v0.1.0` query 正式承诺的能力

以下能力可以作为后续设计和实现方向，但当前不属于 `v0.1.0` 的正式 query 保证：

- 把 `process / community` 作为一等命中结果稳定返回
- 在外部 `term` 之外公开显式 `query intent / control` 面
- 把 semantic search / embedding search 当成当前版本的发布前提
- 以宿主侧规则扩写 runtime 的 query 语义、结果层次或状态机

## 暂不属于公开 CLI 支持范围的能力

以下能力当前不属于 `v0.1.0` 的公开 CLI 支持范围：

- `spec-wiki wiki sync`
- `spec-wiki wiki rebuild`
- 宿主侧自行重建 Wiki 状态机、页面语义或 knowledge/page projection
- 把 query 结果重新包装成一套新的宿主业务协议

说明：

- 当前公开 CLI 与三类宿主都不暴露 `sync / rebuild`
- 旧资产清理列表与内部 runtime 仍可能提到 `sync / rebuild`，那不代表它们属于 `v0.1.0` 的公开支持面

## 宿主限定

当前公开 CLI 与三类宿主都只暴露 4 个入口：

- `init`
- `status`
- `query`
- `update`

当前公开 CLI 与三类宿主都不暴露：

- `sync`
- `rebuild`

其中：

- Claude 暴露 `.claude/skills/wiki-init/SKILL.md`、`wiki-status/SKILL.md`、`wiki-query/SKILL.md`、`wiki-update/SKILL.md`
- Codex 暴露 `.codex/skills/wiki-init/SKILL.md`、`wiki-status/SKILL.md`、`wiki-query/SKILL.md`、`wiki-update/SKILL.md`
- CodeBuddy 暴露 `wiki-init`、`wiki-status`、`wiki-query`、`wiki-update` 四个 action skills
- 三类宿主的显式入口正文统一收敛到 `Purpose / Inputs / Steps / Output / Action Notes / Guardrails`

## 宿主使用边界

三类宿主当前都遵循同一条边界：

- `wiki-query` 回答应直接薄消费 runtime 的稳定字段：
  - `query_mode`
  - `query_trust`
  - `recommended_action`
  - `matched_pages`
  - `provenance_summary`
- `status` 只是辅助检查入口，不把完整 knowledge/page runtime 当成 `v0.1.0` 正式承诺
- 宿主不允许重建 Wiki 状态机、页面语义或 knowledge/page projection
- Claude、Codex 与 CodeBuddy skills 都只负责参数收集、CLI 调用、结果转述与必要上下文

CodeBuddy 额外边界：

- hooks 只负责 `context / guardrails`
- hooks 不负责 `dispatch / orchestration`

如果之前生成过旧版宿主资产，重新执行：

```powershell
node dist/spec-wiki/bin/spec-wiki.js init --tool <host> --repo-root <repo>
```

会清理以下旧形态的受管残留，包括：

- `.claude/commands/wiki/init.md`
- `.claude/commands/wiki/status.md`
- `.claude/commands/wiki/query.md`
- `.claude/commands/wiki/update.md`
- `.claude/commands/wiki/sync.md`
- `.claude/commands/wiki/rebuild.md`
- `.claude/skills/spec-wiki/`
- `<CODEX_HOME>/prompts/wiki-init.md`
- `<CODEX_HOME>/prompts/wiki-status.md`
- `<CODEX_HOME>/prompts/wiki-query.md`
- `<CODEX_HOME>/prompts/wiki-update.md`
- `<CODEX_HOME>/prompts/wiki-sync.md`
- `<CODEX_HOME>/prompts/wiki-rebuild.md`
- `.codex/skills/spec-wiki/`
- `.codebuddy/skills/spec-wiki-runtime/`
- `.codebuddy/skills/wiki-sync/`
- `.codebuddy/skills/wiki-rebuild/`
## 当前平台与交付形态

`v0.1.0` 当前只提供 Windows x64 runtime 分发。

当前正式源码与产物位置：

- 源码包：[packages/spec-wiki](E:/project/!byAI/spec-wiki/packages/spec-wiki)
- 发布 staging 目录：[dist/spec-wiki](E:/project/!byAI/spec-wiki/dist/spec-wiki)
- Windows x64 runtime binary：[dist/spec-wiki/lib/x64-win32/wiki-runtime.exe](E:/project/!byAI/spec-wiki/dist/spec-wiki/lib/x64-win32/wiki-runtime.exe)

## 不包含在本说明中的内容

以下内容属于后续迭代目标，不应被用户或宿主当成 `v0.1.0` 已正式支持：

- 完整的 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble` 终态交付
- 完整 `.wiki/pages/**` 页面体系的稳定生成承诺
- 依赖宿主二次解释的高阶 Wiki 业务规则
- 为特定样本仓库定制的宿主分支逻辑

## 一句话总结

`spec-wiki v0.1.0` 当前是一个公开 CLI 与三类宿主都只支持 `init/status/query/update`、其中正式承诺仍收敛在 `index-only init / update / query`、并统一要求宿主薄消费 query 结果的收敛版发布。
