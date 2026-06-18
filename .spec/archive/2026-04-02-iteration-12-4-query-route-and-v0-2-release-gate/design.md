## Context

当前仓库经过 `12.1 / 12.2 / 12.3` 已经具备了 `v0.2.0 knowledge runtime` 的主链基础：

- `12.1` 把正式 workflow 收口到 provider-backed research，并禁止默认 structural fallback 伪装成功
- `12.2` 把 `.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json + .wiki/.cache/**` 的最小正式产物和 cold restore 闭环扶正
- `12.3` 把 `update` 收成 `ChangeSet -> affected knowledge scope -> refresh derived knowledge -> refresh impacted projections`

但仓库对外公开合同仍然停在 `v0.1.0 index-only`：

- [README.md](E:/project/!byAI/spec-wiki/README.md) 仍明确把 `v0.1.0` 定位成 `index-only runtime`
- [.wiki/05-规格基线/capabilities/repo-wiki-workflow/spec.md](E:/project/!byAI/spec-wiki/.wiki/05-规格基线/capabilities/repo-wiki-workflow/spec.md) 仍把 `init / update` 成功语义定义成“facts/index 可查询即可”
- [crates/wiki-runtime/src/workflows/release_scope.rs](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/workflows/release_scope.rs) 仍保留 `SPEC_WIKI_V0_1_INDEX_ONLY`、`index_only` 成功态和对外状态投影短路

这意味着内部已经成立的 knowledge runtime 主线，仍然被旧 release scope 压成“内部实现细节”，而不是正式发布承诺。

本轮设计只收口 `12.4 = Query Route + v0.2.0 Release Gate`，不扩成新的 lifecycle 大迭代。参考实现边界遵循当前仓库设计文档：

- `tmp/upstream/GitNexus`
  - `symbol-table.ts` 展示了 file/global/callable/field 多索引 facts substrate，适合作为 `index-first` query 底座参考
  - `pipeline.ts` 在 call resolution 前做 wildcard import synthesis、receiver seeding，再并行处理 calls / heritage，说明 query substrate 应先建立厚 facts/graph 底座，再谈更高层消费
- `tmp/upstream/deepwiki-open`
  - `websocket_wiki.py` 展示了 query/session 侧如何围绕 retriever、conversation history 与用户问题组织消费层
  - 但它仍然是 consumption / session 参考，不是本仓库 core 生成主链模板

对 `12.4` 而言，这些参考只支持一个结论：`query route` 应该建立在已有的 index/knowledge/runtime 分层之上，而不是再开一套宿主或 session 语义。

## Goals / Non-Goals

**Goals:**

- 把公开 workflow 合同从 `v0.1.0 index-only` 切换到 `v0.2.0 knowledge runtime first-class release`
- 删除 `index_only` 公开成功态与 `SPEC_WIKI_V0_1_INDEX_ONLY` 短路的正式地位
- 在保持 `term-only` 外部输入稳定的前提下，正式收口 `index -> knowledge -> page fallback` 的 query route
- 明确 `status / query / recommended_action / provenance_summary` 在 `v0.2.0` 下的对外解释边界，其中 `provenance_summary` 固定使用 `index_hit / knowledge_hit / page_fallback` route tags
- 为 `storybook`、`chi + zustand` 与注释合规检查定义统一 release gate，并把 `dagger` 与完整项目集 baseline guard 降为后续参考项
- 让 `README`、命令帮助、宿主消费预期与 runtime 真正公开合同一致

**Non-Goals:**

- 不在本轮把 `sync / rebuild` 升级成新的公开 workflow surface
- 不在本轮引入新的外部结构化 query payload、intent 参数或 session contract
- 不在本轮进入 declared knowledge 的 authoring / conflict / lifecycle
- 不把 `.wiki/06-设计文档/04-扩展场景.md` 中更完整的 query routing、权限分级或长期知识生命周期一起打包进来
- 不把 `12.4` 扩成 `13` 阶段的完整 runtime lifecycle 收口；例如更细的恢复状态机、同步治理和宿主高级 bridge 仍留给后续
- 不为了追 release gate 去新增针对 `storybook` 或 `dagger` 的专有 planner / renderer 分支

## Decisions

### 决策 1：`12.4` 定位为“发布合同切换”，不是新的 pipeline 能力迭代

`12.1 / 12.2 / 12.3` 已经回答了“research policy 是什么”“formal artifacts 是什么”“update 主线是什么”；`12.4` 需要回答的是“什么时候这些内部能力才算正式发布面”。因此本轮主问题不是继续补 planning/research/compose 细节，而是：

- 删除 `v0.1.0 index-only` 的临时公开承诺
- 把已经成立的 knowledge runtime 变成真实 release contract
- 用统一 gate 证明这个 contract 可发布

否则继续做 `12.3.x` 式 contract 小修补，只会让 `v0.2.0` 永远停在“内部已实现、外部未承诺”的灰区。

备选方案：

- 继续拆 `12.3.x` follow-up 小 change
  - 否决：会继续回避 release 真相，且让 `README/spec/workflow` 长期与实现分裂
- 直接跳到 `13` 的完整 lifecycle 收口
  - 否决：范围过大，会把 query route、sync/rebuild、恢复状态机和宿主消费一起卷进来，边界立即漂移

### 决策 2：删除双轨 release scope，公开成功语义只能有一条

本轮采用单一公开成功语义：

- `init / update` 成功 = 最小正式 knowledge runtime 已形成
- facts/index 仍是 query substrate 和降级诊断基础
- facts/index 单独可用，不再等于正式初始化成功

这意味着 `release_scope.rs` 里的：

- `SPEC_WIKI_V0_1_INDEX_ONLY`
- `index_only`
- “facts_ready + missing => index_only” 投影

都不能继续保留为正式发布路径。可以有迁移期实现细节，但不能再成为 spec、README、帮助文本或宿主合同的一部分。

备选方案：

- 保留双轨：既支持 `v0.1 index-only`，又支持 `v0.2 knowledge runtime`
  - 否决：会把测试、文档、宿主消费和状态解释长期一分为二；用户永远不知道自己依赖的是哪条真相线

### 决策 3：外部 query 输入保持稳定，但内部 route 必须升级为三层链路

本轮不升级外部 query 输入；仍保持：

- `term-only`
- runtime 内部可继续映射为 `wiki-index::query` 的 `auto` 请求

但 `run_query` 的正式 route 要从“index-first + page fallback”收口为：

```text
index
-> knowledge
-> page fallback
```

含义是：

- 先用 `wiki-index` 解决 symbol/source/module/entrypoint/graph 级命中
- 当 facts 命中不足以回答问题，或需要更稳定的知识摘要/投影锚点时，才读取 formal knowledge artifacts
- 只有前两层都不足时，才退回页面命中

并且三层 provenance 必须显式区分，禁止 page fallback 再伪装成 facts/index 命中。本轮固定通过 `provenance_summary` 暴露稳定 route tags：

- `index_hit`
- `knowledge_hit`
- `page_fallback`

这样做既保持了 `GitNexus` 式的厚 facts substrate，也符合当前仓库 `.wiki/06-设计文档/01-Runtime设计.md` 里的 `symbol -> graph -> declared knowledge -> derived knowledge -> page` 方向，只是本轮先把 `derived knowledge` 和 page fallback 扶正，不提前把 declared lifecycle 一起做掉。

备选方案：

- 现在就引入外部结构化 query payload
  - 否决：这会把 12.4 扩成新的 transport/host 设计，不是本轮目标
- 继续只保留 `index-first + page fallback`
  - 否决：这会让 `.wiki/.knowledge/**` 永远只是落盘产物，无法成为 query route 的正式消费层
- 新增独立的稳定 `matched_knowledge` 外部字段
  - 暂不采纳：会扩大 `v0.2.0` 的 transport 面；本轮先把 knowledge observability 收进稳定 `provenance_summary` tags

### 决策 4：`runtime_incomplete` 可以作为诊断态存在，但不能再充当发布成功态

本轮不要求系统在所有样本上都永远只剩 `fresh`；诊断态仍然允许存在。但公开合同必须区分：

- `release-success`
  - 已形成最小正式 knowledge runtime，可进入正式消费
- `diagnostic-state`
  - `blocker / runtime_incomplete / needs_update / stale` 等状态，用来说明为什么还没达到正式成功

也就是说，`runtime_incomplete` 可以继续是状态机的一部分，但它不再等价于 “`init / update` 已按公开合同成功”。

备选方案：

- 把所有 `runtime_incomplete` 一律抹成 `blocker`
  - 否决：会损失现有诊断粒度
- 继续把 `runtime_incomplete` 当成可公开承诺的成功路径
  - 否决：这正是 `v0.1` 语义残留，会直接稀释 `v0.2` 发布门槛

### 决策 5：release gate 采用“两层强制门禁 + 一层参考项”，避免只看专项样本或只看大盘数字

本轮 release gate 分两层强制门禁，外加一层非阻塞参考：

1. `storybook`
   - primary gate
   - 证明 docs-heavy 代表样本可以完成 `v0.2` 公开合同
2. `chi + zustand`
   - 证明 contract 不是只在两大样本上成立
3. `dagger`
   - 下沉为后续参考项
   - 用于继续观察 runtime-heavy 样本，但不再属于本轮强制 release gate
4. 完整 `19` 项目 `init` baseline guard
   - 继续保留为广覆盖观察面
   - 但不再属于 `12.4` 本轮必须完成的 release gate

并单独要求一次 `.wiki/02-开发指南/00-代码注释规范.md` 合规检查，防止为了 release 收口而把注释规范再次丢掉。

备选方案：

- 只用 `storybook`
  - 已采纳：在用户明确要求尽快收口、且 `chi + zustand` smoke gate 已补足 release 广度验证的前提下，本轮 primary gate 缩到单样本；`dagger` 下沉为后续参考观察面，但不得伪装成已完成验证
- 直接把 `19` 项目全绿当唯一前置
  - 否决：当前阶段噪声过高，会把 release 合同问题和长尾项目问题混成一团
- 把 `19` 项目 baseline guard 继续当成本轮强制 gate
  - 否决：用户已明确这轮不需要为 `12.4` 收口再跑全量项目，继续保留只会拖慢本轮 contract 切版

### 决策 6：文档、帮助文本和宿主预期必须与 runtime contract 同 change 收口

本轮不接受“代码已经切到 `v0.2`，但 README/帮助文案以后再说”的做法。只要 release contract 变了，以下对象就必须一起收口：

- `README.md`
- runtime command contract 测试
- 宿主 bootstrap / skill 文案里对 `init / status / update / query` 的解释

否则就会继续出现“实现、spec、文档、宿主”四套真相并存的问题。

备选方案：

- 先改代码和 spec，文档之后补
  - 否决：这正是当前 `v0.1 index-only` 残留能活到现在的原因

### 决策 7：12.4 只收口既有公开合同，不扩成新的 surface 迁移

本轮允许改动的公开面只有：

- 既有 `init / status / update / query` 的成功语义
- 既有 `term-only` query 的 route / provenance / readiness 解释
- 配套 `README`、帮助文本、宿主文案与验证脚本

本轮明确不扩张：

- 新公开命令
- 新外部 query payload
- `sync / rebuild` 的 release surface
- 13 阶段的完整 lifecycle / state migration

这样做的目的，是把 12.4 锁死在“contract 切版”，而不是借 release 名义继续长出新的公共 API。

## Risks / Trade-offs

- [部分项目当前仍停在 `runtime_incomplete`，公开合同抬升后会暴露更多失败] → 用 `blocker / runtime_incomplete / recommended_action` 保留诊断粒度，同时把 release gate 和 baseline guard 分层，不让长尾项目直接绑死 primary gate
- [移除 `index_only` 可能影响现有宿主或脚本的状态判断] → 保持 `term-only` 输入和核心状态字段稳定，只迁移其语义；同时补 command contract 与脚本断言
- [query route 引入 knowledge 层后，容易长出新的外部 query schema] → 明确本轮只升级内部 route 和 provenance，不升级外部 payload
- [release gate 可能被误做成“为了通过样本”] → 在 design/spec/tasks 中持续强调禁止新增 storybook/dagger 专有 planner / renderer 分支
- [文档与代码同步改动范围较大] → 把所有对外语义变化集中在本轮统一收口，避免之后长期两套版本共存

## Migration Plan

1. 先修改 UniSpec contract，把 `repo-wiki-workflow`、`repo-wiki-runtime` 与 `workflow-verification` 收到同一条 `v0.2.0` 真相线上。
2. 再移除或下沉 `release_scope.rs` 中的 `v0.1 index-only` 对外短路，让 `init / update / status / query` 真正执行或暴露 `v0.2` 语义。
3. 对 query result shaping 做最小必要调整，使 `provenance_summary` 以 `index_hit / knowledge_hit / page_fallback` 区分三层 route，同时保持 `term-only` 输入和核心字段稳定。
4. 同步更新 `README`、帮助文本、宿主可见文案与命令合同测试。
5. 按 `storybook -> chi + zustand -> .wiki/02-开发指南/00-代码注释规范.md` 的顺序完成本轮 release gate 验证，并沉淀报告；`dagger` 与完整项目集 baseline guard 如需执行，单独作为后续参考记录。

## Open Questions

- `runtime_incomplete` 在 `v0.2.0` 下是否继续作为公开 `status` 状态值保留，还是只在详细诊断中保留；当前倾向是保留，但不再把它视作成功初始化
