## Context

11.6 的问题已经不再是“是否要做全局 CLI + 宿主 bootstrap”，而是“文档真相和工程真相是否一致”。

当前产品侧结论已经基本成立：

- 对外统一入口是 `spec-wiki`
- `spec-wiki init` 负责宿主接入
- `spec-wiki wiki <action>` 负责转发给 `wiki-runtime`
- `v0.1.0` 继续只正式保证 index-only 的 `init / update / query`

但当前工程侧仍处于过渡态：

- `spec-wiki` 的真实源码包仍在 `agents/spec-wiki`
- `pnpm-workspace.yaml` 仍只收录 `agents/*`
- 包内实现仍保留 `src/bootstrap/**` 作为过渡主轴
- README 与 11.6 tasks 曾把 `packages/spec-wiki` 和三层分拆误写成已完成

这会导致两类问题：

1. OpenSpec 状态会错误地告诉后续执行者“11.6 已完成，可以归档”。
2. 实现层会继续围绕 `agents/spec-wiki + bootstrap` 打补丁，而不是朝 `packages/spec-wiki + agents / orchestration / runtime` 收口。

## Current State

当前仓库中的真实工程状态：

- 全局 CLI 包当前位于 `agents/spec-wiki`
- workspace 包模式当前是：
  - `crates/*`
  - `agents/*`
- staging 产物中已经存在 `dist/npm/spec-wiki`
- `src/runtime/**` 已经作为独立子层存在
- `src/bootstrap/**` 仍承担宿主检测、资产生成、宿主选择等职责

结论：

- “对外主包身份是 `spec-wiki`” 这件事已经成立
- “正式源码目录已经迁到 `packages/spec-wiki` 并拆成三层” 这件事还没有成立

## Target State

11.6 的目标收敛结构应明确为：

```text
packages/spec-wiki/src/
  cli.ts
  index.ts

  agents/
    codex/
    claude/
    codebuddy/
    shared/
    prompts/

  orchestration/
    init/
    shared/

  runtime/
```

对应边界：

- `agents/**`
  - 只表达宿主差异：检测、上下文、资产产出、宿主真实落点
- `orchestration/**`
  - 只表达 `spec-wiki init` 的编排：选择宿主、汇总资产、统一写入、ownership、报告
- `runtime/**`
  - 只表达 `spec-wiki wiki <action>` 到 `wiki-runtime` 的 forwarding

`src/bootstrap/**` 若继续存在，只能视为迁移中的临时过渡层，不能再被写成目标结构。

## Goals / Non-Goals

### Goals

- 让 README、OpenSpec 和真实目录迁移目标重新对齐
- 把正式源码包目标明确为 `packages/spec-wiki`
- 把 `spec-wiki` 包内结构明确为 `agents / orchestration / runtime` 三层
- 让 11.6 tasks 反映真实进度，而不是假完成
- 保持 `v0.1.0 index-only` 的正式发布边界不变

### Non-Goals

- 不在本次收口中扩大 runtime 正式承诺范围
- 不在本次收口中新增 `codex / claude / codebuddy` 之外的宿主
- 不在本次收口中把 `sync / rebuild` 升级成 `v0.1.0` 的正式发布能力
- 不在本次收口中改写 `wiki-runtime` 的 transport 合同

## Decisions

### 决策 1：正式主包真相与当前过渡位置必须同时写清

文档不能再只写一句“主包是 `packages/spec-wiki`”就结束，因为这会把目标结构冒充成仓库现状。

正确表达应同时包含两层：

- 当前实现位置：`agents/spec-wiki`
- 目标正式位置：`packages/spec-wiki`

README 与 OpenSpec 都必须按这个方式写。

### 决策 2：11.6 必须回到“迁移中 change”，不能继续保留 all_done 假象

只要仓库里还没有 `packages/spec-wiki`，且 workspace 还没有切到 `packages/*`，就不能把 11.6 继续写成已完成。

因此 11.6 的 tasks 需要重排为真实迁移任务，并按实际落地状态重新打勾。

### 决策 3：`agents / orchestration / runtime` 是必须被实现约束消费的三层

这三层不只是文档术语，而是实现边界：

- 宿主差异不得继续堆进公共 `bootstrap`
- `init` 编排不得回流到宿主 adapter
- runtime forwarding 不得与宿主资产生成重新混层

如果 specs 不把这类边界写成约束，后续实现仍可能回到单层结构。

### 决策 4：迁移顺序必须先校正文档真相，再迁目录和内部结构

正确顺序是：

1. README / proposal / design / tasks / specs 先校准真相
2. 目录迁移到 `packages/spec-wiki`
3. workspace、根脚本、staging 来源改到新路径
4. 包内从 `bootstrap` 拆到 `agents / orchestration / runtime`
5. 测试与文案重新验收

这样后续 apply 11.6 时，执行者有一致的真相来源。

## Migration Plan

### 1. 文档真相收口

- README 明确区分“当前实现位置”和“目标结构”
- proposal / design / tasks 不再把 `packages/spec-wiki` 写成已落地事实
- specs 最小补充正式源码包来源与三层边界约束

### 2. 目录与工作区迁移

- `agents/spec-wiki -> packages/spec-wiki`
- `pnpm-workspace.yaml` 从 `agents/*` 切到 `packages/*`
- 根脚本、构建脚本、发布脚本同步改到新路径

### 3. 包内结构迁移

- 宿主差异收敛到 `src/agents/**`
- `spec-wiki init` 主链收敛到 `src/orchestration/**`
- `wiki-runtime` forwarding 保持在 `src/runtime/**`
- 删除或降级旧 `src/bootstrap/**`

### 4. 验收

- 更新包级测试、根级 staging/e2e 测试
- 确认 `dist/npm/spec-wiki` 仍然是正式 staging 产物
- 确认 `v0.1.0` 仍然只正式保证 index-only 的 `init / update / query`
- 单独检查注释是否符合 `COMMENTING.md`

## Risks / Trade-offs

- [风险：只改文档，不改任务状态]
  - 结果是 OpenSpec 仍会把 change 标成已完成，继续误导 apply 与 archive
- [风险：只迁目录，不拆内部层次]
  - 结果只是把 `agents/spec-wiki` 改名成 `packages/spec-wiki`，但 `bootstrap` 杂糅结构仍然存在
- [风险：specs 不补结构约束]
  - 结果是 design 写得很漂亮，但实现仍可以继续把逻辑塞回单一主轴
- [风险：README 继续把目标结构当现状]
  - 新读者会直接被错误仓库结构误导

## Open Questions

- `templates/**` 最终是继续作为独立共享目录，还是再细分到宿主层附近；当前倾向是保留共享目录
- `agents/prompts/**` 最终是否保留在宿主层下；当前倾向是保留，因为它服务宿主选择而不是 runtime
- `src/bootstrap/**` 是直接删除，还是先保留兼容壳过渡一次；当前阶段不要求兼容旧版本，倾向是迁移后直接删除
