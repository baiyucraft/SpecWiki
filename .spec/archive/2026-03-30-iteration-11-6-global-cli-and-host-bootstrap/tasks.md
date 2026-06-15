> Scope Revision:
> 11.6 当前的真实目标不是“补几句文档说明”，而是把 `spec-wiki` 从当前过渡态 `agents/spec-wiki + src/bootstrap/**`，收口到正式结构 `packages/spec-wiki + src/agents/** + src/orchestration/** + src/runtime/**`。本轮对外发布面仍继续保持 `v0.1.0 index-only`。

## 1. 文档真相收口

- [x] 1.1 更新 `README.md`，明确区分当前实际源码位置 `agents/spec-wiki` 与目标结构 `packages/spec-wiki`，避免把目标目录误写成仓库现状。
- [x] 1.2 重写 11.6 的 `proposal / design / tasks`，将 change 从“假完成状态”恢复为真实迁移计划，并明确 `agents / orchestration / runtime` 三层目标边界。
- [x] 1.3 最小同步 11.6 specs，对齐正式源码包来源与三层边界约束，避免实现继续围绕旧 `bootstrap` 主轴发散。

## 2. 目录与工作区迁移

- [x] 2.1 将正式源码包从 `agents/spec-wiki` 迁移到 `packages/spec-wiki`，并保证仓库内只保留一个 `spec-wiki` 正式源码真相。
- [x] 2.2 更新 `pnpm-workspace.yaml`、根级脚本和相关路径解析，使 workspace、build、test、publish 都以 `packages/spec-wiki` 为主包来源。
- [x] 2.3 更新 staging / publish 相关脚本与辅助模块，确保 `dist/npm/spec-wiki` 的产物来源切换到新的正式源码目录。

## 3. 包内结构迁移

- [x] 3.1 将当前宿主差异实现按宿主拆分到 `src/agents/codex`、`src/agents/claude`、`src/agents/codebuddy` 与 `src/agents/shared`。
- [x] 3.2 将 `spec-wiki init` 的宿主选择、资产规划、统一写入、ownership 和报告逻辑迁移到 `src/orchestration/init/**`。
- [x] 3.3 保持 `spec-wiki wiki <action>` 的 binary resolve、invoke、JSON/NDJSON/bridge 解析逻辑收敛在 `src/runtime/**`。
- [x] 3.4 删除或降级旧 `src/bootstrap/**`，确保它不再作为正式结构主轴存在。
- [x] 3.5 调整顶层 `cli.ts` 路由，使其只承担参数解析和命令分发，不继续承载完整 init 编排细节。

## 4. 发布边界与文案校准

- [x] 4.1 复核 CLI 文案、宿主模板与 README，确保 `v0.1.0` 继续只正式保证 index-only 的 `init / update / query`，不伪装成完整 knowledge/page runtime。
- [x] 4.2 复核 `dist/npm/spec-wiki` 的包内资产，确认 CLI 入口、runtime forwarding 和宿主模板资产都来自迁移后的正式源码目录。

## 5. 测试与验收

- [x] 5.1 更新 `spec-wiki` 包级测试，覆盖迁移后的目录与三层边界，不再依赖旧 `bootstrap` 主轴路径。
- [x] 5.2 更新根级 staging / e2e / 工作区测试，验证主包来源、dist 产物与命令面都已切到 `packages/spec-wiki`。
- [x] 5.3 执行相关 JS/Rust/index-only 测试，确认本轮迁移未破坏 `init / update / query` 的索引层可用性。
- [x] 5.4 单独执行一轮 `.wiki/02-开发指南/00-代码注释规范.md` 合规检查，确认迁移过程中新增或调整代码的注释风格与粒度符合要求。
