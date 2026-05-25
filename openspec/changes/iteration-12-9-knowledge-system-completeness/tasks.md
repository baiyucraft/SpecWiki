## 1. Scope Freeze

- [x] 1.1 明确 `iteration-12-9-knowledge-system-completeness` 是 umbrella / roadmap change，不直接承诺 runtime 代码落地。
- [x] 1.2 将“完善 knowledge system”收敛为 7 个 capability，并写清非目标：不回到 page-first、不做治理平台空话、不做多 repo 编排、不做宿主 UI 扩张。
- [x] 1.3 复核 capability 命名、owner crate 与依赖顺序，确保与 `DESIGN-3.0.md`、`DESIGN-RUNTIME.md`、`SCENE-1.md`、`SCENE-2.md` 一致。

## 2. Spec Impact Matrix

- [x] 2.1 为 7 个 capability 列出现有 specs 修改矩阵，明确每个 capability 将修改哪些既有 specs。
- [x] 2.2 为缺失的 formal contract 列出必须新增的 specs，至少覆盖 `derived research`、`projection readiness`、`governance conflict`、`query routing`、`answer assembly` 与 `quality gates`。
- [x] 2.3 为每个 capability 标注 formal object、truth source、workflow consumers、artifact layer 与 failure / degraded / recovery contract。

## 3. Capability Change Proposals

- [x] 3.1 生成 `Declared Lifecycle Completeness` 子 change。
- [x] 3.2 生成 `Derived Research Contract Completeness` 子 change。
- [x] 3.3 生成 `Projection / Readiness / Recovery Completeness` 子 change。
- [x] 3.4 生成 `Governance Conflict Artifacts` 子 change。
- [x] 3.5 生成 `Query Route Completeness` 子 change。
- [x] 3.6 生成 `Answer Assembly Contract` 子 change。
- [x] 3.7 生成 `Engineering Hardening / Quality Gates` 子 change。

## 4. Acceptance Matrix

- [x] 4.1 为每个 capability 定义 `formal object / schema` 验收。
- [x] 4.2 为每个 capability 定义 `workflow consumption` 与 `artifact / recovery` 验收。
- [x] 4.3 定义 `storybook + dagger` 的专项验收与报告要求。
- [x] 4.4 定义全量 `19` 项目的批量验收入口与质量指标。
- [x] 4.5 单独检查 [COMMENTING.md](/E:/project/!byAI/spec-wiki/COMMENTING.md) 与 OpenSpec 产物边界一致性，避免后续子 change 把注释和 contract 一起漂移。
