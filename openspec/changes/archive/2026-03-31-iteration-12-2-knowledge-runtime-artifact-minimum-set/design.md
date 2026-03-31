## Context

当前仓库已经完成四包拆分，并在 `12.1` 把正式 workflow 收口到 provider-backed research 与 runtime gate / blocker 语义；但 knowledge runtime 仍然没有形成 `v0.2.0` 所要求的正式 Git-tracked 产物闭环。具体表现是：

- `knowledge_domains`、`knowledge_units`、`page_digests`、`research_cache` 等关键对象目前主要停留在 SQLite 与本地 `.wiki/.cache/**`
- `.wiki/.knowledge/**` 还没有最小正式对象集，因此 `DESIGN-RUNTIME.md` 里定义的 `.knowledge + pages + metadata + cache` 分层仍然只停留在概念上
- [SCENE-1.md](E:/project/!byAI/spec-wiki/SCENE-1.md) 的协作恢复场景还不成立：B 用户拉取 A 用户提交的 `.wiki` 后，不能仅凭 `.wiki/.knowledge/** + pages + metadata` 恢复本地 runtime
- `repo-wiki-workflow` 的公开合同仍然是 `v0.1.0 index-only`，因此本轮不能把这次收口误写成 release surface 已升级

这轮设计只回答一个问题：如何把现有 knowledge runtime 的最小正式对象，从“只在 SQLite / `.cache` 中可见的工作态”收成“可上库、可恢复、可审计的 `.wiki/.knowledge/**` 正式产物”，并保证 restore 的语义是“恢复本地 runtime”，而不是“重新跑一遍 full init”。

设计参考遵循当前仓库边界，而不是照搬上游：

- `tmp/upstream/deepwiki-rs`
  - 参考 staged workflow 与恢复链路的组织方式，不照搬目录结构
- `tmp/upstream/codewiki/codewiki`
  - 参考 leaf-first / parent-consume-child 的摘要纪律，不照搬页面语义
- `tmp/upstream/deepwiki-open`
  - 仅作为 query / session / 消费层参考，不作为本轮 core artifact 设计模板
- `tmp/upstream/GitNexus`
  - 仅作为宿主 / repo understanding 辅助参考，不作为 `.knowledge` 正式真相设计依据

资深 reviewer 已审核通过这条 change，但明确要求把两条红线写死：

- `.wiki/.knowledge/**` 中不同对象必须区分 formal identity、formal summary 与 recovery anchor，不能混层
- restore 的完成标准必须是“恢复本地 `.cache` 与 runtime 可消费状态”，不能偷换成“重新生成知识本体”

## Goals / Non-Goals

**Goals:**

- 为 `.wiki/.knowledge/**` 定义最小正式产物集，并明确每类对象的 truth kind
- 将最小正式产物集划分为三类：
  - formal identity objects：`knowledge_domains`、`knowledge_units`、`knowledge_tree`
  - formal derived summaries：parent / unit research 摘要
  - projection / recovery anchors：`page_digests`、runtime gates / readiness、最小恢复锚点
- 让正式 workflow 在完成 planning / research / compose / assemble 后，把上述最小对象写入 `.wiki/.knowledge/**`
- 让 runtime 能在缺失或丢弃 `.wiki/.cache/**` 时，仅凭 `.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json` 恢复本地 cache 与可消费状态
- 明确 SQLite `knowledge_store` 回到 working cache / rebuild target，而不是 knowledge formal truth
- 以 `storybook` 为优先样本验证 `.knowledge` 落盘、cold restore、恢复后 `status / query` 与 blocker/readiness 语义

**Non-Goals:**

- 不在本轮把 `update` 主线改成 knowledge-first
- 不在本轮引入 `ChangeSet -> affected knowledge scope -> refresh derived knowledge -> refresh projections` 的正式 contract
- 不在本轮收口最终 `index -> knowledge -> page fallback` 的 release 语义
- 不在本轮引入 declared knowledge 的 authoring / sync / lifecycle
- 不在本轮重写 compose 语义本身，例如 leaf-first、parent rollup、section plan 或 citation policy
- 不在本轮把 `repo-wiki-workflow` 的公开 `v0.1.0` surface 改写成“已经正式承诺 knowledge runtime”

## Decisions

### 决策 1：`.wiki/.knowledge/**` 采用“identity / summary / anchor”三层最小 taxonomy

`.wiki/.knowledge/**` 只落最小正式对象，不把所有工作态和缓存都塞进去。对象按 truth kind 分三类：

- formal identity objects
  - `knowledge_domains`
  - `knowledge_units`
  - `knowledge_tree`
- formal derived summaries
  - parent / unit research 摘要
- projection / recovery anchors
  - `page_digests`
  - `unit_runtime_gates` 的正式摘要
  - 与当前 snapshot 对齐所需的最小恢复锚点

建议目录遵循 [DESIGN-RUNTIME.md](E:/project/!byAI/spec-wiki/DESIGN-RUNTIME.md) 的分层：

- `.wiki/.knowledge/derived/**`
  - identity objects 与 research summaries
- `.wiki/.knowledge/runtime/**`
  - page projection / readiness / restore 所需 anchors

本轮只收 `derived/` 与 `runtime/` 两层最小正式对象集；`declared/` 目录 MAY 保留为空占位，但 MUST NOT 在本轮被定型为正式 lifecycle contract。

理由：

- 这能把“知识本体”“摘要结果”“恢复辅助对象”拆开，避免把 `page_digests` 或 gate 摘要误当成知识主真相
- 这与 `.knowledge / pages / metadata / cache` 的职责边界一致

备选方案：

- 继续把所有对象只放在 SQLite `knowledge_store`
  - 否决：无法形成 Git-tracked 正式产物，也无法满足协作恢复场景
- 把所有 knowledge 相关对象一股脑写入 `.wiki/.knowledge/runtime/**`
  - 否决：会把 identity、summary 和 recovery anchor 混成一层，后续 declared knowledge 生命周期也会失控

### 决策 2：restore 只允许“恢复本地 runtime”，不允许“重新生成知识”

本轮定义的 restore 入口只做两件事：

- 读取 `.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json`
- 重建本地 `.wiki/.cache/**` 与 runtime 可消费状态

restore 明确不允许执行以下行为作为“成功恢复”的前提：

- 重新扫描源码构建完整 facts snapshot
- 重新跑 planning / research / compose / assemble
- 因 `.cache` 缺失就自动退回全量 `init`

如果 restore 发现正式产物与当前仓库代码不一致，可以：

- 恢复本地 `.cache`
- 标记 `stale`、`needs_update` 或 blocker
- 引导后续显式 `update`

但不得把“先 restore，再偷偷 regenerate”伪装成 restore 成功。
restore 成功的语义也 MUST 只是“本地 cache 已恢复、`status/query` 可消费且 blocker/readiness 可诊断”，而不是“完整 wiki runtime 已 ready”。

理由：

- 这条边界直接来自 [SCENE-1.md](E:/project/!byAI/spec-wiki/SCENE-1.md)
- 如果 restore 退化成 regenerate，`.cache` 仍然是隐性主存储，`v0.2.0` C 段就没有真正成立

备选方案：

- `.cache` 不存在时直接触发一次完整 `init`
  - 否决：违背协作恢复场景，成本和语义都不对

### 决策 3：SQLite `knowledge_store` 保留为 working cache，但必须可由 `.knowledge` 重建

`knowledge_store` 中现有的 `knowledge_domains`、`knowledge_units`、`page_digests`、`research_cache` 等对象在本轮后仍然存在，但其定位收紧为：

- workflow 运行期的 working cache
- query / status / restore 的本地加速层
- 正式 `.knowledge` 产物的 rebuild target

正式 truth 改为：

- `.wiki/.knowledge/**`
- `.wiki/pages/**`
- `wiki.metadata.json`

SQLite 不再是 knowledge formal truth；但 `.knowledge/**` 也不替代 `.cache` 的本地派生职责。
与之对应，`sqlite-cache-storage` 合同必须同步更新，明确 knowledge 相关表是 mirror / cache / rebuild target。

理由：

- 当前代码已经有完整的 SQLite store trait 与表结构，直接删除会扩大实现面
- 把 SQLite 收回 rebuild target，可以在不破坏当前主链的前提下扶正 `.knowledge` 的正式地位

备选方案：

- 直接删除大部分 SQLite knowledge 表，只保留文件系统产物
  - 否决：本轮目标是 formal artifact 收口，不是重写本地工作态结构

### 决策 4：research-driven compose 的改动只做“结果沉淀”，不重写 compose 主链

这轮对 `research-driven-page-composition` 的要求只增加一层：

- 正式 workflow 产出的 provider-backed research / compose 结果，必须沉淀出最小正式 summary / anchor

本轮不改变：

- leaf-first / parent rollup 组织方式
- section plan 语义
- citation / evidence / diagram policy
- provider policy 与 runtime gate 主线

理由：

- 这些能力已在 9.x 与 12.1 收口过，再重写会把 C 段变成大杂烩
- reviewer 已明确要求禁止借此重写 compose contract

备选方案：

- 顺手把 page compose 结果、draft、citation plan 全量升级成 `.knowledge` 正式真相
  - 否决：超出“最小正式产物集”范围

### 决策 5：验证重点切到 `.knowledge` 落盘与 cold restore，不再以页面 fidelity 为主

本轮验收重点改为：

- `.wiki/.knowledge/**` 是否按最小 taxonomy 落盘
- `.wiki/.knowledge/** + pages + metadata` 是否足以恢复 `.cache`
- restore 后 `status` 是否正确表达 `ready / stale / needs_update / blocker`
- restore 后当前 `query` 入口是否能消费恢复出的 runtime，而不是强制要求 full init

`storybook` 作为主样本，`dagger` 仅保留观察位。reference fidelity 可以继续作为背景信息，但不是本轮主通过线。

理由：

- 这是 C 段真正的通过条件
- 如果继续把重点放在页面对齐率，会把 E 段 release 验收提前混进来

## Risks / Trade-offs

- [`.knowledge/**` 文件 contract 过细会过早锁死后续 declared knowledge 生命周期] → 本轮只定义最小 formal set，不提前设计 authoring / governance 层
- [restore 后 `query` 能力可能弱于 warm cache] → 允许 runtime 明确标记 `needs_update` 或恢复受限，但不得要求重新 full init 才能进入可消费状态
- [SQLite 与 `.knowledge` 双写期间存在一致性风险] → 用 metadata / recovery anchor 绑定统一 snapshot identity，并把 restore 建立在正式产物上
- [把过多对象升级为正式产物会导致 `.wiki` 噪声膨胀] → 严格只升级 identity / summary / anchor 最小集，工作态和草稿继续留在 `.cache`

## Migration Plan

1. 先定义 `.wiki/.knowledge/**` 的最小文件 contract 与 truth taxonomy。
2. 在正式 `init / update / rebuild` 后把最小正式对象从当前 runtime 输出到 `.knowledge/**`。
3. 补齐从 `.knowledge/** + pages + metadata` 重建 `.cache` 的 restore 路径。
4. 调整 `status` 与当前 `query` 对恢复态 cache 的消费，使其无需 full init 即可工作或给出明确 `needs_update / blocker`。
5. 用 `storybook` 跑专项验证，确认落盘、restore、状态投影与 blocker/readiness 语义成立。
6. 本轮结束后，再进入 D：knowledge-first update 主线收口。

## Open Questions

- `page_digests` 应归入 `.wiki/.knowledge/runtime/**` 还是 `.wiki/.knowledge/derived/**`；当前更倾向把它视为 projection / recovery anchor
- 恢复锚点是复用 `wiki.metadata.json` 中的 snapshot 信息，还是在 `.wiki/.knowledge/runtime/**` 单独保留一个最小 manifest；本轮优先倾向最小 manifest
- restore 后若当前代码与正式产物存在轻微偏差，`query` 应直接返回恢复态结果，还是先显式提示 `needs_update` 再决定是否降级返回
