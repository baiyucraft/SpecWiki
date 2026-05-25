---
name: unispec-continue
description: 根据当前 stage 调度到应继续使用的 UniSpec skill。当用户想继续当前 change 或不确定下一步时使用。
metadata:
  author: unispec
  generatedBy: "0.1.0"
---

继续一个 UniSpec change：读取当前状态，判断当前流程是否完成，并调度到当前或下一个 `unispec-*` skill。它是跨阶段调度入口，不是 CLI 命令，也不替代专业阶段 skill。

覆盖阶段：

```text
跨阶段调度，不映射到单个 stage
```

**重要：本 Skill 不直接修改 `meta.yaml.stage`，不创建阶段 artifact，不更新 `.wiki`，不归档。artifact 产出、wiki 沉淀和 stage 推进由被调度的目标 skill 负责。**

---

**输入**

```text
用户指定的 change-id，或当前对话可推断的 change-id
.spec/changes/<change-id>/meta.yaml
.spec/changes/<change-id>/ 中的 artifacts
unispec status --json
unispec validate <change-id>
```

**步骤**

1. **选择 change**

如果用户指定 change-id，使用该 change。否则：

- 从当前对话推断 change-id。
- 如果只有一个 active change，可以使用它。
- 如果存在多个 active changes 且无法判断，读取 `.spec/changes/` 并让用户选择。

不要处理已经位于 `.spec/archive/` 的 change。没有 active change 时停止并说明。

2. **读取状态**

先执行：

```bash
unispec status --json
```

从输出中定位目标 change，读取：

- `metadata.stage`
- `metadata.deliveryShape`
- `metadata.multiChange`
- `artifacts`
- `blockingIssues`

再执行：

```bash
unispec validate <change-id>
```

`unispec status --json` 和 `unispec validate` 是主要判断依据；文件存在性只是辅助核验，不得绕过 metadata consistency、archive marker、report evidence 或 blockingIssues。

3. **判断当前应使用的 skill**

按当前状态选择目标 skill：

| 当前状态 | 判断 | 目标 skill |
| --- | --- | --- |
| `exploration` parent | `split.md`、parent `meta.yaml` 或 child stub 未完成 | `unispec-explore` |
| `exploration` parent | split 和 child stubs 已完成 | 选择下一个未完成 child，使用 `unispec-propose` |
| `exploration` child | child 仍缺少 `proposal.md` | `unispec-propose` |
| `exploration` standalone single-change stub | 已有 `meta.yaml` / `research/`，仍缺少 `proposal.md` | `unispec-propose` |
| `proposal` | `proposal.md` 缺失、为空或 status/validate 有 blocking issues | `unispec-propose` |
| `proposal` | proposal 已完成 | `unispec-design` |
| `delivery` | `deliveryShape`、parent / child 边界或 child proposal 仍需收尾 | `unispec-propose` |
| `delivery` | deliveryShape 与 proposal 已完成 | `unispec-design` |
| `design` | `design.md` 未完成 | `unispec-design` |
| `design` | design 已完成 | `unispec-plan` |
| `cases` | `system-tests.md` 或 `tasks.md` 未完成 | `unispec-plan` |
| `cases` | planning artifacts 已完成 | `unispec-plan` |
| `tasks` | `implementation-ready: true` 缺失或非 true | `unispec-plan` |
| `tasks` | `implementation-ready: true` 已确认 | `unispec-apply` |
| `implementation` | tasks、checklist、实现或本地验证仍未完成 | `unispec-apply` |
| `implementation` | implementation 已完成 | `unispec-review` |
| `review` | full/pass 的 `review-report.md` 或 `test-report.md` 证据缺失 | `unispec-review` |
| `review` | full/pass 报告证据齐备，且 validate 无 blocking issues | `unispec-archive` |
| `verification` | full/pass 的 `review-report.md` 或 `test-report.md` 证据缺失 | `unispec-review` |
| `verification` | full/pass 报告证据齐备，且 validate 无 blocking issues | `unispec-archive` |
| `archive` | 已进入归档收尾 | `unispec-archive` |

如果当前 stage 与 artifacts 明显不一致，例如 stage 已进入 `design` 但 `proposal.md` 缺失，停止并报告 blockingIssues；不要自行回退或修补 stage。

4. **调度目标 skill**

调度前说明：

```text
change-id:
current-stage:
status-summary:
target-skill:
reason:
blockingIssues:
- ...
```

如果当前流程未完成，继续使用当前阶段对应的 skill，并列出缺失 artifact、未完成 task、未通过报告证据或 blockingIssues。

如果当前流程已完成，使用下一阶段对应的 skill。不要先改 `meta.yaml.stage`；目标 skill 在产出 artifact 后负责推进 stage。

5. **执行目标 skill**

如果当前运行环境支持显式调用 Skill，切换到或调用 `target-skill` 并继续处理同一个 change。

如果当前运行环境不能显式调用 Skill：

- 读取 `.agents/skills/<target-skill>/SKILL.md`。
- 按该 skill 的流程继续执行。
- 在最终输出中明确本次实际按哪个 skill 执行。

如果目标 skill 是 `unispec-archive`，只进入归档 skill 的检查和执行流程；本 Skill 不移动目录，也不直接写 `.wiki`。只有切换并按 `unispec-archive` 执行后，才继承 archive 的 wiki 沉淀权限。

6. **完成后复查状态**

目标 skill 执行完成或暂停后，重新运行：

```bash
unispec status --json
```

汇报当前 change 的最新 stage、blockingIssues 和下一步建议。

**输出**

最终汇报字段：

```text
continue-result: routed | paused | no-active-change
change-id:
current-stage:
target-skill:
reason:
actions-taken:
- ...
blockingIssues:
- ...
next-step:
- ...
```

**约束**

- 一次最多调度一个目标 skill，不跨多个流程动作。
- 不直接修改 `meta.yaml.stage`。
- 不直接更新 `.wiki`，不把 `.wiki` 作为路由判断的必读输入。
- 不创建或改写 `proposal.md`、`design.md`、`system-tests.md`、`tasks.md`、`review-report.md`、`test-report.md`，除非已经切换并按目标 skill 执行。
- 不在 blockingIssues 未解决时推进流程。
- 不用文件存在性绕过 `unispec status --json`、`unispec validate` 或报告证据。
- 不替 `unispec-apply` 完成任务，不替 `unispec-review` 签发报告，不替 `unispec-archive` 归档。
- 不凭主观判断把 review 或 verification 标记为 pass。
- 不归档、不移动目录、不删除历史产物。
