# refactor-specwiki-around-contract-closure 拆分方案

## 拆分原则

本 parent change 承载新版 SpecWiki 重构 program 的边界、顺序和依赖，不直接实现代码，也不替代各 child 的 proposal / design / tasks。

所有 child 都必须是可独立验收的 `single-change`，不嵌套在 parent 目录下。实现依据以以下设计草案为上位输入：

- `.docs/design/specwiki-contract-closure.md`
- `.docs/design/specwiki-code-graph-index-design.md`
- `.docs/design/knowledge-to-wiki-projection-contract.md`
- `.docs/design/governance-runtime-integration.md`
- `.docs/design/specwiki-cli-unification.md`

## Children

### 1. refactor-specwiki-around-contract-closure-page-tree-contract

- 目标: 统一新版用户可见 Wiki 页面树，停止把 `.wiki/pages/**` 作为 runtime 目标目录；旧目录不属于新版 runtime surface，不迁移、不清理、不诊断。
- 依赖: 无。
- 关键验收: `init / update / rebuild` 不再写 `.wiki/pages/**`；metadata、state、SQLite FTS、query fallback 和 restore 只消费正式页面树。
- 归档状态: [x] archived

### 2. refactor-specwiki-around-contract-closure-truth-restore-snapshot

- 目标: 落实 truth kind、两级 restore、committed snapshot manifest 和 readiness 状态。
- 依赖: refactor-specwiki-around-contract-closure-page-tree-contract。
- 关键验收: `.cache` 缺失不会伪装 graph ready；metadata 只保存 snapshot pointer 和 binding index；Level 1 restore 支持 diagnostic mode。
- 归档状态: [x] archived

### 3. refactor-specwiki-around-contract-closure-projection-writeback-boundaries

- 目标: 拆清 `wiki-knowledge` 的 PagePlan / SectionPlan / declared contract 与 `wiki-runtime` 的 render / merge / write / metadata 绑定职责。
- 依赖: refactor-specwiki-around-contract-closure-truth-restore-snapshot。
- 关键验收: declared authoring writeback 按 runtime parse、knowledge validate、runtime commit 执行；derived drift 不写回 derived knowledge。
- 归档状态: [x] archived

### 4. refactor-specwiki-around-contract-closure-query-route-readiness

- 目标: 统一 query route tags、query result DTO、readiness、trust 和人类可解释输出。
- 依赖: refactor-specwiki-around-contract-closure-truth-restore-snapshot。
- 关键验收: 输出区分 `index_symbol_hit / index_path_hit / index_graph_hit / knowledge_* / governance_* / projection_ref / fallback`。
- 归档状态：[x] archived

### 5. refactor-specwiki-around-contract-closure-code-graph-index

- 目标: 按 code graph/index 设计重构 `wiki-index` 的 graph schema、raw captures、phase DAG、GraphStore / IndexQueryStore 和 index query adapter。
- 依赖: refactor-specwiki-around-contract-closure-truth-restore-snapshot、refactor-specwiki-around-contract-closure-query-route-readiness。
- 关键验收: raw imports / calls / heritage 持久化；`SymbolNode` 扩展；`.spec` 不进入 code graph facts。
- 归档状态：[x] archived

### 6. refactor-specwiki-around-contract-closure-cli-product-surface

- 目标: 收窄默认 CLI 心智，落实 `init / status / query / update` 主路径、landing state、安全模式和人类可解释输出。
- 依赖: refactor-specwiki-around-contract-closure-query-route-readiness、refactor-specwiki-around-contract-closure-projection-writeback-boundaries。
- 关键验收: 默认 help 只突出主路径；advanced / governance 命令通过场景或 `--help --all` 暴露；破坏性动作需要 dry-run / manifest / confirm。
- 归档状态: [ ] pending

### 7. refactor-specwiki-around-contract-closure-governance-isolation

- 目标: 将 `.spec` evidence、artifact reference index、governance derived knowledge 与 core wiki runtime 隔离。
- 依赖: refactor-specwiki-around-contract-closure-query-route-readiness、refactor-specwiki-around-contract-closure-code-graph-index。
- 关键验收: `.spec` changes 只刷新 governance index / summary；`governance_readiness: not_enabled` 不阻断普通 `query / update`。
- 归档状态: [ ] pending

### 8. refactor-specwiki-around-contract-closure-archive-dry-run-manifest

- 目标: 实现 archive dry-run、readiness report、operation manifest 和失败恢复提示，不把 Wiki 更新混入 archive 事务。
- 依赖: refactor-specwiki-around-contract-closure-governance-isolation、refactor-specwiki-around-contract-closure-cli-product-surface。
- 关键验收: archive 默认先 validate；dry-run 不移动目录；apply 只覆盖 `.spec` move + manifest + parent / child meta；Wiki 更新只输出 issue / refs。
- 归档状态: [ ] pending

## ASCII dependency view

```text
page-tree
  -> truth-restore-snapshot
       -> projection-writeback
       -> query-route-readiness
            -> code-graph-index
            -> cli-product-surface
            -> governance-isolation
                 -> archive-dry-run-manifest

projection-writeback
  -> cli-product-surface

code-graph-index
  -> governance-isolation
```
