---
title: UniSpec 开发规范
description: UniSpec change artifact 与阶段推进规范
updated: 2026-04-28
owner: unispec
---

# UniSpec 开发规范

> UniSpec managed baseline
>
> 本页由 UniSpec 内置模板管理。`unispec init --force` / `unispec update --force` 会将本页恢复为当前版本内置模板。
> 项目特有流程规则不要长期写入本页，应新增项目专属页面并从本页回链。

UniSpec 使用 `.spec/changes/<change-id>/` 保存一个 change 的生命周期 artifact。每个阶段只生成或维护本阶段负责的文件。Skill 可以引用本页判断阶段边界、artifact 归属和 parent / child change 的基本规则。

## 基本流程

```text
explore
  -> propose
  -> design
  -> plan
  -> apply
  -> review
  -> archive
```

对应产物推进：

```text
split.md / meta.yaml
  -> proposal.md
  -> design.md
  -> system-tests.md + tasks.md (+ unit-tests.md when TDD)
  -> 代码实现 + tasks.md 勾选
  -> review-report.md + test-report.md
  -> .spec/archive/YYYY-MM-DD-<change-id>/
```

如果需求是 `single-change`，可以从 `propose` 直接创建一个独立 change。如果先经过 `explore` 且需要沉淀 research，可以创建 standalone exploration stub 保存 `meta.yaml` 和 `research/`，再进入 `propose` 补齐 `proposal.md`。如果需求需要拆分，先用 `explore` 创建 parent `split.md` 和 child stub，再逐个 child 进入 `propose -> design -> plan -> apply -> review -> archive`。

## Artifact

| 文件 | 用途 |
| --- | --- |
| split.md | multi-change parent 的拆分方案 |
| proposal.md | 问题、目标、非目标、成功标准和影响范围 |
| design.md | 技术方案、模块边界、接口、流程变化和风险 |
| system-tests.md | ST-* 系统测试用例 |
| tasks.md | 可执行、可验证、可勾选的实现任务 |
| unit-tests.md | TDD 模式下的单元测试用例蓝图；非 runtime required artifact |
| review-report.md | 整个 change 的最终 review 报告 |
| test-report.md | 整个 change 的测试 / verification 报告 |
| meta.yaml | stage、deliveryShape、multiChange 和 artifact 状态 |

## 阶段边界

- `explore`：探索需求、调研现状、判断 `deliveryShape`。如果是 `multi-change`，创建 parent `split.md` / `meta.yaml`，并创建 child stub `meta.yaml`；如果是 `single-change` 且需要落盘 research，可创建 standalone exploration stub；不写 `proposal.md`。
- `propose`：为一个具体 `single-change`、child change 或 standalone exploration stub 创建 `proposal.md`，明确问题、目标、非目标、成功标准、影响范围和风险；已有 explore research 只是输入证据，必须判断是否仍需补充 proposal 调研；不写设计方案。
- `design`：根据已确认的 `proposal.md` 创建 `design.md`，说明方案、涉及模块、接口与数据结构、流程变化、验证思路和风险；不写任务，不写代码。
- `plan`：根据 `design.md` 创建 `system-tests.md` 和 `tasks.md`，确定实现模式；当实现模式为 `tdd` 时额外创建 `unit-tests.md`；normal 模式不创建或更新 `unit-tests.md`；不执行实现。
- `apply`：按 `tasks.md` 的实现模式执行任务。TDD 模式按 Red / Green / Refactor 执行并读取 `unit-tests.md`；normal 模式参考结构化实现清单，允许先实现代码，完成大 task 前补齐单元测试或替代局部验证；不改变 proposal 或 design 的方向，不主动承担 archive-time wiki 沉淀或目录化拆分。
- `review`：对整个 change 做整体 review 和 verification，创建或更新 `review-report.md` 与 `test-report.md`；不直接归档，不更新 `.wiki`，只报告 wiki-sync issues。
- `archive`：检查两个报告证据，执行 `unispec archive <change-id>`，将 change 移动到 `.spec/archive/YYYY-MM-DD-<change-id>/`，并检查是否需要沉淀 `.wiki` 或在用户确认后整理长页面。

## Stage 与 required artifacts

| stage | required artifacts |
| --- | --- |
| `exploration` parent | `split.md`、`meta.yaml` |
| `exploration` child stub | `meta.yaml` |
| `exploration` standalone single-change stub | `meta.yaml` |
| `proposal` | `proposal.md`、`meta.yaml` |
| `delivery` | `proposal.md`、`meta.yaml`；`deliveryShape` 是 metadata 字段 |
| `design` | `proposal.md`、`design.md`、`meta.yaml` |
| `cases` | `proposal.md`、`design.md`、`system-tests.md`、`meta.yaml` |
| `tasks` / `implementation` | `proposal.md`、`design.md`、`system-tests.md`、`tasks.md`、`meta.yaml` |
| `review` | `proposal.md`、`design.md`、`system-tests.md`、`tasks.md`、`meta.yaml` |
| `verification` / archive 前 | `proposal.md`、`design.md`、`system-tests.md`、`tasks.md`、`review-report.md`、`test-report.md`、`meta.yaml` |

`unit-tests.md` 只在 TDD 模式下由 `unispec-plan` 生成，用于记录测试先行的 `UT-*` 单元测试用例蓝图：目标行为、Test / Modify / Reference 文件、具体测试代码蓝图、测试数据 / fixture / mock 边界、运行命令、预期 Red 失败、Green 通过条件和 Refactor 守卫。它不是 runtime required artifact，不写入 `meta.yaml.artifacts`，也不替代最终 `test-report.md`。normal 模式不创建或更新 `unit-tests.md`；如历史残留文件已存在，plan 阶段仅在最终汇报中标记为 ignored stale planning file，删除必须先获得用户确认。

`unit-tests.md` 中的测试代码只是蓝图，plan 阶段不写入真实测试文件；真实测试文件由 `unispec-apply` 在 Red task 执行时写入。如果 plan 阶段无法写出具体测试代码、精确命令或预期 Red 失败原因，应暂停并补充 design / plan，而不是生成空壳。Red / Green / Refactor 执行步骤属于 `tasks.md` 和 `unispec-apply`，不写入 `unit-tests.md`。

`review-report.md` 和 `test-report.md` 是 runtime artifacts，不只是对话记录。它们必须以 YAML frontmatter 开头，并写入 `scope`。`review-result` 取值为 `pass` / `fail` / `partial`，`verification-result` 取值为 `pass` / `fail` / `skipped`，`scope` 取值为 `full` / `partial`。归档只接受 `scope: full`：

```yaml
---
review-result: pass
scope: full
---
```

```yaml
---
verification-result: pass
scope: full
---
```

归档前普通 change / child change 必须满足 `review-result: pass`、`verification-result: pass`，且两个报告均为 `scope: full`。parent change 不要求自身进入 `stage: verification`，也不生成 parent 自身 review / test 报告；parent 归档只检查所有 children 是否已实际归档。active child 即使具备 full/pass 报告证据，也不能替代已归档事实。

## deliveryShape

- `single-change`：解决一个核心问题，可形成完整验收闭环，可在一次设计、任务拆分、实现、review 和 verification 中完成。
- `multi-change`：包含多个独立问题、需要分阶段验收、存在长依赖链，或需要隔离风险。

不要把多个不可独立验收的问题塞进一个 change。也不要只按技术层切分 change；每个 child change 都应有独立目标、独立验收和可隔离风险。

## multi-change

- parent change 保存 `split.md` 和 `meta.yaml`，只承载拆分方案，不写 `proposal.md`。
- child change 是 `.spec/changes/` 下的一级目录，不嵌套在 parent 目录下。
- child id 默认使用 `<parent-change-id>-<child-topic>`。
- child stub 初始为 `stage: exploration`、`deliveryShape: single-change`、`multiChange.role: child`，并记录 `parent`、`order`、`dependsOn`。
- child 归档后，active parent 的 `meta.yaml` 和 `split.md` 必须同步归档状态。
- parent 归档前，所有 child 必须已实际归档；active child 的 `review-report.md` 与 `test-report.md` 证据不能替代归档事实。

## standalone exploration stub

- 仅当 `unispec-explore` 判断为 `single-change` 且已有 research 需要沉淀时创建。
- 只保存 `meta.yaml` 和可选 `research/`，不保存 `split.md` 或 `proposal.md`。
- `meta.yaml` 使用 `stage: exploration`、`deliveryShape: single-change`，不包含 `multiChange`，并记录 `artifacts.proposal.status: missing`。
- 下一步必须进入 `unispec-propose`。propose 需要读取已有 research，判断是否足以支撑 proposal；不足时继续做定向 proposal 调研。

child 归档状态在 parent `split.md` 中使用 checkbox 表达：

```markdown
- 归档状态：[ ] pending
```

归档后更新为：

```markdown
- 归档状态：[x] archived
```

## Wiki 与 Skill 联动规则

| Skill | 读取 wiki | 写入 wiki |
| --- | --- | --- |
| `unispec-explore` | 可读取 `.wiki/INDEX.md` 和相关规范，用于判断范围和 deliveryShape | 不写 wiki |
| `unispec-propose` | 可读取与当前 proposal 边界相关的 wiki 事实 | 不写 wiki |
| `unispec-design` | 可读取与当前设计相关的稳定结论和 source-of-truth 链接 | 不写 wiki，不复制 wiki 正文进 `design.md` |
| `unispec-plan` | 可读取 `.wiki/INDEX.md`、流程 / 开发指南、相关模块或能力文档 | 不写 wiki，只提炼影响测试和任务规划的事实 |
| `unispec-apply` | 默认不读 wiki；只在实现任务需要核对既有文档事实时读取相关页面 | 除非 tasks 明确要求且用户确认，否则不更新 `.wiki`；长期知识沉淀和目录化拆分交给 `unispec-archive` |
| `unispec-review` | 可读取被 artifacts 或改动直接引用的 wiki facts | 不写 wiki，只报告 wiki-sync issues |
| `unispec-continue` | 不把 wiki 作为路由必读输入 | 不写 wiki，只调度目标 Skill |
| `unispec-archive` | 读取 `.wiki/INDEX.md`、栏目索引和相关页面，检查沉淀需求和长文整理需求 | 仅在用户确认下协助沉淀长期知识、整理长页面或目录化拆分 |

archive 的 wiki 沉淀分四类：

- 必须沉淀：CLI / API / 配置契约、stage / artifact / Skill 触发规则、长期开发规范、公开使用方式。
- 可沉淀：稳定架构决策、模块职责变化、长期排障经验、迁移后的维护注意事项。
- 不沉淀：临时调研过程、一次性验证输出、未确认假设、完整 proposal / design / report 原文。
- 只保留在 archive：change 执行历史、review 明细、测试日志摘要、任务勾选历史。

`unispec archive <change-id>` 不保证自动写 wiki。`unispec-archive` 输出必须区分 `wiki-updates-made`、`wiki-updates-required` 和 `wiki-updates-not-needed`，不得把待处理项写成已完成。

长文 wiki 整理同样遵守该边界：`unispec-review` 只能报告 wiki-sync issues，`unispec-continue` 不直接写 wiki，`unispec-explore` / `unispec-propose` / `unispec-design` / `unispec-plan` 不为清理长页面而全量扫描 `.wiki`，也不把 wiki 正文搬入 change artifacts。`unispec-archive` 只有在用户明确确认后，才可协助更新 wiki 页面、栏目索引、反向链接或把长页面升级为 `<主题>/INDEX.md + 01-*.md` 的目录结构；未确认或未完成的拆分必须写入 `wiki-updates-required`。

长文整理不是 CLI 门禁。约 250-300 行、标题层级过多、覆盖多个稳定主题或经常被局部引用只是整理信号；只有多主题页面已经阻碍导航、维护或精确链接时，才建议目录化。`INDEX.md` 只做导航、摘要和链接，不能替代正式规格目录、`src/`、`bin/`、测试或配置作为事实来源。
