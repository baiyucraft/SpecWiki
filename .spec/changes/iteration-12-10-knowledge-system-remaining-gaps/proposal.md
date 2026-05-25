## Why

`iteration-12-9` 已经把 knowledge system completeness 的最小 formal contract 冻结成 7 个 capability，并完成了对应子 change 的第一轮收口；但 authoritative 文档仍然明确指出，当前系统只达到 `minimal formal knowledge runtime`，还没有成为“形式完整、工程完善的 knowledge system”。

因此这轮不应重复 `12-9` 的 formalization，而应单独创建一个 `post-12-9 remaining gaps` umbrella，把剩余缺口按“既有前置 + 新增 child”的方式组织起来，固定依赖顺序、非目标与 release 收口面，避免后续又把已冻结 capability 换名重做。

## What Changes

- 创建 `iteration-12-10-knowledge-system-remaining-gaps` umbrella change，用来冻结 `12-9` 之后的 remaining gaps program，而不是承诺“一轮补完完整 knowledge system”。
- 将 `formalize-declared-authoring-contract` 明确吸纳为 `12-10` 的既有前置 change，标注其负责 `DeclaredRecord / typed scope / supersede / authoring truth`，不重复创建同义子 change。
- 新增 `extend-governance-conflict-resolution-contract`，把 `12-9-4` 已 formalize 的 conflict artifact 扩展到 resolution / decision lifecycle，而不是重做 deterministic conflict artifact。
- 新增 `harden-answer-assembly-compose-runtime`，把 `12-9-6` 已 formalize 的 answer contract 推进到 runtime / pipeline hardening，而不是重做 answer contract formalization。
- 新增 `formalize-knowledge-governance-metrics-and-release-gates`，在 `12-9-7` 的 quality gates 之上补 knowledge-level metrics、program-level blocker 与 release evidence。
- 明确非目标：不回到 page-first，不宣称 knowledge system 已 closure，不重做 `12-9` 的 7 capability freeze，不把 dashboard / 多 repo 编排 / 宿主 UI 扩张混进这轮。

## Capabilities

### New Capabilities
- `knowledge-system-remaining-gaps-roadmap`: 定义 `post-12-9` remaining gaps 的 adopted prerequisite、child change 边界、依赖顺序与收口矩阵。

### Modified Capabilities

无。

## Impact

- 直接影响 `.spec/changes/**` 的下一轮 program 组织方式与后续 apply 顺序。
- 间接影响 `wiki-model`、`wiki-knowledge`、`wiki-runtime` 后续变更的边界：declared authoring、governance resolution、answer/compose runtime hardening、knowledge-level metrics 与 release gates。
- 为后续 `.docs`、验收报告、`storybook + dagger` 与 `19` 项目批量收口提供统一程序入口，但本 umbrella 本身不直接修改运行时代码。
