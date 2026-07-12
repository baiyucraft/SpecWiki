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

### 6. refactor-specwiki-around-contract-closure-governance-isolation

- 目标: 建立与 code facts 隔离的只读治理内核，统一 `.spec` evidence、status、validate、ReviewGate、artifact refs、governance query refs 和独立 freshness/readiness。
- 依赖: refactor-specwiki-around-contract-closure-query-route-readiness、refactor-specwiki-around-contract-closure-code-graph-index。
- 关键验收: `.spec` 不进入 code facts；治理 evidence 以 `.spec` 为 truth；runtime 能输出 status/validate/query refs；治理 blocked 不阻断普通 `query / update`；validator parity 有 fixture；本 child 不移动目录、不执行 archive。
- 归档状态：[x] archived

### 7. refactor-specwiki-around-contract-closure-cli-product-surface

- 目标: 在治理共享合同稳定后收窄 CLI 产品面，落实统一 `init`、一级 command router、分层 help、landing state、人类可解释输出和宿主资产。
- 依赖: refactor-specwiki-around-contract-closure-query-route-readiness、refactor-specwiki-around-contract-closure-projection-writeback-boundaries、refactor-specwiki-around-contract-closure-governance-isolation。
- 关键验收: 默认 help 只突出 `init / status / query / update`；advanced / governance 命令只通过场景或完整 help 暴露；统一 init 明确 partial success、退出码和 next action；保留 JSON/NDJSON 机器协议；本 child 不实现 archive manifest 或目录移动。
- 归档状态：[x] archived

### 8. refactor-specwiki-around-contract-closure-archive-dry-run-manifest

- 目标: 实现 archive dry-run、readiness report、operation manifest 和失败恢复提示，不把 Wiki 更新混入 archive 事务。
- 依赖: refactor-specwiki-around-contract-closure-governance-isolation、refactor-specwiki-around-contract-closure-cli-product-surface。
- 关键验收: archive 默认先 validate；dry-run 不移动目录；manifest 具备版本、持久化路径和 precondition digest；apply 原子同步目录移动、parent `meta.yaml` 与 `split.md`；失败可重试/恢复；Wiki 只输出 issue / refs，archive 不调用 Wiki 写流程。
- 归档状态: [ ] pending

## ASCII dependency view

```text
page-tree
  -> truth-restore-snapshot
       -> projection-writeback
       -> query-route-readiness
            -> code-graph-index
                 -> governance-isolation
                      -> cli-product-surface
                           -> archive-dry-run-manifest

projection-writeback
  -> cli-product-surface
```
