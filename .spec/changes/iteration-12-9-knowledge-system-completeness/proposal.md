## Why

当前仓库已经收稳 `minimal formal knowledge runtime`，但“knowledge 功能完善”仍然不是一个单一 capability，而是至少包含 formal object、lifecycle、governance、query / answer contract 和 engineering hardening 五类不同验收面。若继续把这些内容打包成一个直接实施的 change，最后只会得到范围失控、验收失真或重新滑回 page-first 的方案。

因此，这轮 `iteration-12-9` 不直接承诺“一次性补完完整 knowledge system”，而是先把后续工作冻结成一个 umbrella / roadmap change：明确 capability 切分、spec impact matrix、依赖顺序、owner crate 与验收矩阵，让后续子 change 能按 formal object 主线逐个落地。

## What Changes

- 创建一个 `knowledge system completeness` 的 umbrella change，明确它本身不直接承诺 runtime 代码实现，而是负责后续子 change 的 capability 分解与边界冻结。
- 将“完善 knowledge system”收敛为 7 个 capability：`declared lifecycle completeness`、`derived research contract completeness`、`projection / readiness / recovery completeness`、`governance conflict artifacts`、`query route completeness`、`answer assembly contract`、`engineering hardening / quality gates`。
- 为每个 capability 建立 spec impact matrix，明确哪些现有 specs 要修改、哪些新 specs 必须新增，以及它们分别落在哪个 crate / artifact layer。
- 为每个 capability 固定统一的设计检查项：formal object、truth source、workflow consumers、artifact layer、failure / degraded / recovery 语义。
- 为后续子 change 固定统一的验收矩阵，区分 formal contract 验收、workflow 验收、artifact / recovery 验收、`storybook + dagger` 样本验收与全量 `19` 项目批量验收。
- 明确非目标：本轮不直接实现 runtime 代码、不回到 page-first、不把治理平台空话写成 capability、不把 engineering hardening 与 formal contract change 混成一个实施包。

## Capabilities

### New Capabilities
- `knowledge-system-completeness-roadmap`: 定义 knowledge system 完整化的 capability 切分、依赖顺序、spec impact matrix 与统一验收矩阵。

### Modified Capabilities

无。

## Impact

- 直接影响 `.spec/changes/**` 的后续拆分方式与 proposal / design / specs / tasks 编写标准。
- 间接影响后续 `wiki-model`、`wiki-knowledge`、`wiki-runtime` 子 change 的 formal object 边界、workflow 语义与验收顺序。
- 影响 `storybook / dagger / 19 projects` 的后续验收组织方式，但本 umbrella change 本身不直接修改 runtime 代码。
