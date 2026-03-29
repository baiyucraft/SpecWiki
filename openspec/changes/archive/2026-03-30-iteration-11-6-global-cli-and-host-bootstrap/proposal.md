## Why

`iteration-11-6-global-cli-and-host-bootstrap` 当前存在一个明显失真：对外入口虽然已经收敛为全局 `spec-wiki`，但仓库中的真实源码位置、README 表述和 OpenSpec 任务状态并没有保持一致。

当前现状：

- `spec-wiki` 的实际源码包仍位于 `agents/spec-wiki`
- `pnpm-workspace.yaml` 当前仍只收录 `agents/*`，并不存在 `packages/spec-wiki`
- 包内仍保留 `src/bootstrap/**` 这样的过渡实现表述
- 11.6 的 `tasks.md` 却把“迁到 `packages/spec-wiki`”和“三层收敛完成”写成了已完成状态

目标结构：

- 正式源码包应迁移到 `packages/spec-wiki`
- 包内结构应明确拆成：
  - `src/agents/**`
  - `src/orchestration/**`
  - `src/runtime/**`
- `agents` 是体系概念，不再等于 npm 包所在目录

因此，11.6 不能继续维持“假完成”状态，而应回到一个真实可执行的迁移 change：先同步文档真相，再推动目录、工作区、脚本和包内结构一起收口。

## What Changes

- 同步 `README.md`，明确区分：
  - 当前实际源码位置仍在 `agents/spec-wiki`
  - 目标结构后续迁移到 `packages/spec-wiki`
- 重写 11.6 的 `proposal / design / tasks`，去掉把目标结构误写成已落地事实的表述。
- 将 11.6 的任务状态改回真实进度：
  - 文档已完成的项可以完成
  - 尚未真实落地的目录迁移、workspace 更新、结构拆分必须保持未完成
- 把 11.6 的目标结构统一收敛为：
  - `packages/spec-wiki`
  - `src/agents/**`
  - `src/orchestration/**`
  - `src/runtime/**`
- 最小同步相关 specs，补上两类约束：
  - 正式发布包源码来源必须收敛到 `packages/spec-wiki`
  - 宿主差异、init 编排、runtime forwarding 三层不得重新回混到单一 `bootstrap` 主轴
- 保持 `v0.1.0` 的正式发布面不变，继续只正式保证 index-only 的 `init / update / query`

## Capabilities

### Modified Capabilities

- `global-cli-bootstrap`
  - 从“已有全局 CLI 能跑”收敛为“全局 CLI 的正式源码包、目录边界和 init/runtime 分层都必须真实落地”
- `adapter-distribution`
  - 从“主包已经切到 spec-wiki”收敛为“主包身份已经成立，但源码目录与 staging 来源还要继续迁到 `packages/spec-wiki`”

### No New User Capability

- 本次不新增用户侧功能
- 本次主要修正文档真相，并把 11.6 重新定义成真实迁移计划

## Impact

- `README.md` 将不再把目标结构误写成仓库现状
- 11.6 的 OpenSpec 文档与任务状态将恢复为真实可执行的迁移状态
- 后续 apply 11.6 时，将以 `packages/spec-wiki + agents / orchestration / runtime` 作为明确目标，而不是继续围绕 `agents/spec-wiki + bootstrap` 打补丁
- 不改变当前 `v0.1.0 index-only` 的 runtime 发布合同
