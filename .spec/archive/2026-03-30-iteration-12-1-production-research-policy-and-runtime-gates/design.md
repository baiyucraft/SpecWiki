## Context

当前仓库已经完成四包边界重整，并且上一条 `iteration-12-0 / parent-unit-research-contract` 已经把高层 parent unit 拉回 `KnowledgeUnit -> UnitResearch -> Compose` 主线；但正式 runtime 仍然没有收成 `v0.2.0` 所要求的 production policy。具体表现是：

- `crates/wiki-runtime/src/workflows/research_provider.rs` 仍然把 `StructuralResearchProvider` 作为正式 workflow 可达路径；
- `crates/wiki-runtime/src/workflows/{init,update,rebuild,status}.rs` 仍然没有把 provider 缺失、provider 失败和 runtime blocker 投影成正式 knowledge runtime 状态；
- `crates/wiki-runtime/src/domain/change_set.rs` 与 `crates/wiki-runtime/src/workflows/update.rs` 仍然是 page-first 增量心智，因此本轮不应提前跳去做 knowledge-first update；
- `.wiki/.knowledge/**` 仍未形成正式文件落盘主链，当前 knowledge 相关正式对象主要还在 SQLite 中，因此也不应在本轮把 C 一起打包。

这轮设计只回答一个问题：如何把正式 workflow 的 `Research -> Compose` 成功语义，从“可以退回 structural baseline 的过渡态”，收成“provider-backed research 才算正式成功”的稳定 contract。

设计参考遵循本仓库边界，而不是照搬上游实现：

- `tmp/upstream/deepwiki-rs`
  - 参考其 workflow 分阶段执行与中断恢复思路，不照搬其产品目录结构；
- `tmp/upstream/codewiki/codewiki`
  - 参考其 leaf-first / parent-consume-child 的生成纪律，不照搬其页面语义；
- `tmp/upstream/deepwiki-open`
  - 仅作为 query / session / 消费层参考，不作为本轮 core 主链模板；
- `tmp/upstream/GitNexus`
  - 仅作为 host / repo understanding 辅助参考。

资深 reviewer 结论已经明确：这条 change 可以做，但前提是边界必须锁死在 `Production Research Policy + Runtime Gates`，不能顺手把 `.wiki/.knowledge/**`、knowledge-first update 或 release 验收混进来。

## Goals / Non-Goals

**Goals:**

- 让正式 `init / update / rebuild` 默认必须走 provider-backed research。
- 把 `StructuralResearchProvider` 收敛到测试、fixture、显式开发模式。
- 让 provider 缺失、provider 不可用、provider research 失败都变成显式失败，并留下 checkpoint / gate / blocker 诊断。
- 让 `status`、runtime summary 和 gate 摘要能表达 `runtime_incomplete / blocker / needs_update` 这类 knowledge runtime 状态。
- 让 `storybook` 的专项验证直接检查 provider 路径、失败诊断和 runtime readiness，而不是只看页面表象。

**Non-Goals:**

- 不在本轮实现 `.wiki/.knowledge/**` 的正式文件落盘。
- 不在本轮把 `update` 改写为 knowledge-first 主线。
- 不在本轮收口 query route 的最终 `index -> knowledge -> page fallback` 发布语义。
- 不在本轮扩展宿主 skill、query payload 或更深一层的 bridge/session 能力。
- 不在本轮追求 `v0.2.0` 全部 release 门槛一次达成。

## Decisions

### 决策 1：正式 workflow 与开发 workflow 显式分轨

正式模式下，`init / update / rebuild` 的 research provider 选择必须只有两类结果：

- `provider_backed`
- `failed_with_blocker`

`StructuralResearchProvider` 只能出现在：

- Rust 测试
- fixture
- 显式开发模式

理由：

- 现行 spec 已经把“正式 runtime 不得默认 structural fallback”写成硬约束；
- 如果继续把 structural baseline 视为正式成功态，后面的 `.wiki/.knowledge/**` 落盘只会固化错误语义。

备选方案：

- 保留当前 `provider_backed + structural fallback` 双轨正式成功语义
  - 否决：会让 `v0.2.0` 的 knowledge runtime 发布面继续失真。

### 决策 2：provider 选择仍在 workflow 入口统一决策，但结果必须进入 gate / checkpoint 语义

不新建第二套 provider 选择层，而是在现有 workflow 入口选择逻辑上收紧输出：

- 选择结果必须区分正式模式与显式开发模式；
- 失败原因必须可投影为 checkpoint、unit gate blocker 与 runtime summary；
- `init / update / rebuild` 共用同一套选择语义。

理由：

- 当前 `select_runtime_research_provider()` 已经是统一入口，继续沿用改造成本最低；
- provider 选择如果不进入生命周期对象，就无法在 `status`、恢复和验证里稳定复用。

备选方案：

- 每个 workflow 自己判断 provider 可用性
  - 否决：会制造 contract 漂移，和现有 spec 冲突。

### 决策 3：provider 失败时保留双层诊断，不允许“假成功”

provider 相关失败必须同时留下两层信息：

- workflow 级：`pipeline_checkpoint`
- unit / runtime 级：`unit_runtime_gates` 与 runtime summary blocker

并且外部语义要区分：

- `needs_update`
  - facts 或源码变脏，需要刷新；
- `runtime_incomplete`
  - facts 已就绪，但 research / compose 尚未完成；
- `blocker`
  - provider 缺失、provider 不可用、provider research 失败等硬阻塞。

理由：

- 仅有 checkpoint 不足以回答当前仓库是否还能继续消费；
- 仅有 gate 也不足以回答“这次 workflow 为什么失败”。

备选方案：

- 只保留 checkpoint，不扩充 status / gate 投影
  - 否决：宿主和验证脚本仍然无法稳定区分“不完整”和“被阻塞”。

### 决策 4：本轮只收“生产 research policy”，不同时重写 compose / update / artifact truth

这轮只改正式成功语义，不改变以下 truth owner：

- `.wiki/pages/**` 仍然是 projection truth；
- SQLite 中的 `research_cache / page_digests / page_drafts` 仍然是 knowledge cache / compose artifact；
- `ChangeSet -> AffectedSet` 仍然维持当前 page-first 结构。

理由：

- 这是 B 的最小闭环；
- 若同时进入 `.wiki/.knowledge/**` 或 knowledge-first update，会把 B、C、D 三轮搅在一起，验证面失控。

备选方案：

- 在去掉 structural fallback 的同时直接引入 `.wiki/.knowledge/**` 正式产物
  - 否决：会把还未稳定的运行时状态固化成 Git-tracked 正式对象。

### 决策 5：专项验收以 `storybook` 的 provider/gate 行为为主，不再以页面表面对齐率驱动

本轮验收重点改为：

- 是否真实走了 provider-backed research；
- provider 不可用或失败时是否留下 checkpoint / gate / blocker；
- `status` 是否能稳定表达 `runtime_incomplete / blocker / needs_update`；
- `storybook` 上的 readiness / blocker 是否可诊断。

不把本轮作为前置门槛的内容：

- `.wiki/.knowledge/**` 正式落盘结果；
- knowledge-first update 增量行为；
- 全量 19 项目最终回归。

理由：

- 只有先确认生产 research policy 成立，后续 C/D/E 才有稳定输入。

## Risks / Trade-offs

- [正式模式去掉 structural fallback 后，短期失败率会上升] → 通过显式开发模式保留调试路径，并把失败转为可诊断 blocker，而不是假成功。
- [`status` 新增 knowledge runtime 状态后，旧的 `index_only` 心智会被打破] → 本轮明确以 `v0.2.0` 主线为准，不再为旧过渡语义保留兼容层。
- [provider 失败链路若只补 workflow 级错误，仍然难以恢复] → 强制 checkpoint 与 gate/blocker 同步落盘。
- [`storybook` 专项仍然偏重，测试时间会增长] → 仍限定单样本范围，不提前恢复 19 项目全量作为前置。

## Migration Plan

1. 收紧 workflow 入口的 provider 选择逻辑，显式区分正式模式与开发模式。
2. 调整 `init / update / rebuild` 的正式失败语义，禁止无 provider 时退回 structural success。
3. 将 provider 缺失、provider 不可用和 provider research 失败写入 checkpoint 与 runtime gate/blocker。
4. 收口 `status`、runtime summary 与对外状态投影。
5. 为 `storybook` 补专项验证与报告口径，并把 `dagger` 记录为后续 change 的观察项。
6. 本轮结束后，再进入 C：`.wiki/.knowledge/**` 最小正式产物落盘。

## Open Questions

- `status` 对外是否直接输出 `runtime_incomplete / blocker / needs_update`，还是保留更细的内部枚举后再做投影。
- provider 不可用与 provider research 返回非法结构，是否需要拆成不同 blocker code，供宿主做更细粒度动作建议。
- 显式开发模式是否只由 `wiki.dev.yaml` / steering mode 触发，还是允许命令行再提供单独开关。
