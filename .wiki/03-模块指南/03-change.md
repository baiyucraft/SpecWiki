---
title: change 模块
description: SpecWiki Lite 的 .spec stage、artifact、校验和归档合同
updated: 2026-07-28
owner: docs
---

# change 模块

## 职责

`core/change` 读取 `.spec/changes/**`，用同一份 stage 与 artifact registry 支撑 `status`、`show`、`validate` 和 `archive`。

## Stage 与 artifact

Stage 闭集为：

```text
exploration -> proposal -> delivery -> design -> cases -> tasks
-> implementation -> review -> verification -> archive
```

Artifact registry 包含：

| ID | 文件 |
| --- | --- |
| `split` | `split.md` |
| `proposal` | `proposal.md` |
| `design` | `design.md` |
| `cases` | `system-tests.md` |
| `tasks` | `tasks.md` |
| `unit-tests` | `unit-tests.md` |
| `review-report` | `review-report.md` |
| `test-report` | `test-report.md` |
| `metadata` | `meta.yaml` |

普通 change 的 required artifacts 随 stage 单向增加。Parent change 只要求 `split.md` 与 `meta.yaml`。

## 校验规则

- Change ID 必须是 canonical kebab-case。
- `meta.yaml` 必须是 YAML object，`id` 与目录名一致。
- `stage` 必须属于闭集，`deliveryShape` 必须是 `single-change` 或 `multi-change`。
- 当前 stage 的 required artifacts 必须存在；`--strict` 还要求这些文件非空。
- 普通 change 到达 `verification` 或 `archive` 时，review 报告必须是 `review-result: pass / scope: full`，测试报告必须是 `verification-result: pass / scope: full`。

## 归档规则

`archive` 会执行严格校验，并把 active change rename 到 `.spec/archive/YYYY-MM-DD-<change-id>`；目标已存在时拒绝覆盖。

- 普通 change 只能从 `verification` 或 `archive` stage 归档。
- Child 归档会同步 active parent 的 child archive evidence 和 `split.md` marker；同步失败时回滚。
- Parent 只有在全部 children 已离开 active tree、归档目标存在且 parent marker 一致时才能归档。

## 维护入口

- Artifact registry：`packages/spec-wiki-lite/src/core/change/artifacts.ts`
- Metadata：`packages/spec-wiki-lite/src/core/change/metadata.ts`
- 校验：`packages/spec-wiki-lite/src/core/change/validate.ts`
- 查看：`packages/spec-wiki-lite/src/core/change/show.ts`
- 归档：`packages/spec-wiki-lite/src/core/change/archive.ts`
