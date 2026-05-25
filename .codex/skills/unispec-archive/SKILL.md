---
name: unispec-archive
description: 归档已完成的 UniSpec change 并检查 .wiki 沉淀。当用户通过 review 与 verification 后需要归档时使用。
metadata:
  author: unispec
  generatedBy: "0.1.0"
---

归档已完成的 UniSpec change：先做归档前最终核对，再执行 `unispec archive <change-id>` 移动 change，并检查 `.wiki` 沉淀是否已完成或需要后续处理。CLI 归档动作不保证自动写入 wiki。

覆盖阶段：

```text
archive
```

**重要：本 Skill 只负责归档前检查、执行归档和提示 / 协助知识沉淀，不重新 review，不重新实现，不手动把 `meta.yaml.stage` 改成 `archive`。**

---

**输入**

```text
普通 change / child change:
- .spec/changes/<change-id>/proposal.md
- .spec/changes/<change-id>/design.md
- .spec/changes/<change-id>/system-tests.md
- .spec/changes/<change-id>/tasks.md
- .spec/changes/<change-id>/review-report.md
- .spec/changes/<change-id>/test-report.md
- .spec/changes/<change-id>/meta.yaml

parent change:
- .spec/changes/<change-id>/split.md
- .spec/changes/<change-id>/meta.yaml

.wiki/ 现有文档和相关索引
```

当普通 change / child change 已完成 implementation、最终 full review 和 full verification，或 parent change 的所有 children 已实际归档，需要移动到 `.spec/archive/` 并收尾知识沉淀检查时使用。parent archive 是 children 归档状态聚合检查，不生成 parent 自身 `review-report.md` 或 `test-report.md`。

**步骤**

1. **选择 change**

如果用户指定 change-id，使用该 change。否则：

- 从当前对话推断 change-id。
- 如果只有一个 active change，可以使用它。
- 如果存在多个 active changes 且无法判断，读取 `.spec/changes/` 并让用户选择。

不要归档已经位于 `.spec/archive/` 中的 change。

2. **确认归档前置条件**

普通 change / child change 确认以下文件存在且非空：

```text
.spec/changes/<change-id>/proposal.md
.spec/changes/<change-id>/design.md
.spec/changes/<change-id>/system-tests.md
.spec/changes/<change-id>/tasks.md
.spec/changes/<change-id>/review-report.md
.spec/changes/<change-id>/test-report.md
.spec/changes/<change-id>/meta.yaml
```

如果 `multiChange.role` 为 `parent`，只要求：

```text
.spec/changes/<change-id>/split.md
.spec/changes/<change-id>/meta.yaml
```

确认 `meta.yaml` 可解析，并检查：

- `meta.yaml.id` 与 change-id 一致。
- `meta.yaml.deliveryShape` 未丢失或改变。
- 普通 change / child change 的 `meta.yaml.stage` 为 `verification`。
- parent change 的每个 child 均已实际归档；active child 即使 full/pass 或 archive-ready，也不能替代已归档。

如果普通 change / child change 的 stage 不是 `verification`，暂停并建议先回到 `unispec-review` 完成最终 review 和 verification。

parent change 不要求自身 `stage: verification`，也不做自身 final review 或 verification；它的归档就绪只来自 children 已实际归档的文件系统事实和 parent 侧归档标记。

3. **核对报告证据**

普通 change / child change 必须确认：

- `review-report.md` frontmatter 为 `review-result: pass` 且 `scope: full`。
- `test-report.md` frontmatter 为 `verification-result: pass` 且 `scope: full`。
- `unispec validate <change-id>` 无 blocking issues。
- tasks 中所有小 task 已完成。
- 每个大 task 的 `### CheckList` 已完成且有证据支撑。
- system-tests 有通过结果或明确验证记录。

上述证据只适用于普通 change / child change。

parent change 不做代码审查或 verification。parent 归档条件是：每个 child 都已位于 `.spec/archive/YYYY-MM-DD-<child-id>/`，且 parent `meta.yaml` / `split.md` 中的 child 归档标记一致。任何 child 仍在 `.spec/changes/<child-id>/`，即使处于 `stage: verification` 且报告 full/pass，也都阻止 parent archive。

如果普通 change / child change 无法确认报告证据通过，暂停并建议回到 `unispec-review`。如果 parent 无法确认所有 child 状态，暂停并先处理未完成 child。不要用主观判断替代缺失证据。

4. **确认 child parent 同步策略**

当归档的是 child change 时，不要在运行 CLI 前手动修改 parent。当前 CLI 应在 `unispec archive <child-change-id>` 成功后同步 active parent：

```yaml
multiChange:
  children:
    - id: <child-change-id>
      order: 1
      dependsOn: []
      archiveStatus: archived
      archivedAt: "<ISO-8601 UTC>"
      archivedTo: ".spec/archive/YYYY-MM-DD-<child-change-id>"
```

同时将 parent `split.md` 中对应 child 小节的归档状态更新为：

```markdown
- 归档状态：[x] archived
```

归档后校验要求：

- 不使用 `status: archived`，避免和 artifact status 混淆。
- `archivedTo` 使用 repo-relative path，不写绝对路径。
- 如果 parent 不存在、parent 已归档、parent meta 与 child meta 不一致，CLI 应失败或本 Skill 必须暂停并报告，不静默跳过。
- parent archive 判断不能只信 `archiveStatus`，必须以 `.spec/archive/YYYY-MM-DD-<child-id>/` 和 parent 侧归档标记为准；active child 的报告证据不能替代归档事实。

5. **检查归档目标冲突**

检查归档目标：

```text
.spec/archive/YYYY-MM-DD-<change-id>/
```

如果目标目录已存在，暂停并报告冲突。不要覆盖、合并或删除已有归档。

6. **检查 .wiki 沉淀计划**

读取 `.wiki/INDEX.md`、相关栏目索引和与当前 change 直接相关的页面，判断是否需要沉淀：

- 设计决策、架构约束或长期维护规则。
- 实际实现与设计文档之间的最终取舍。
- 新增或变更的开发规范、流程规则、CLI 行为或公开契约。
- 需要更新的栏目索引或对外方法说明。
- 已经阻碍导航、维护或精确链接的长页面整理或目录化拆分。

沉淀内容分四类：

- 必须沉淀：新增或改变的 CLI / API / 配置契约、stage / artifact / skill 触发规则、长期开发规范、公开使用方式。
- 可沉淀：稳定架构决策、模块职责变化、长期排障经验、迁移后的维护注意事项。
- 不沉淀：临时调研过程、一次性验证输出、未确认假设、完整 proposal / design / report 原文。
- 只保留在 archive：change 执行历史、review 明细、测试日志摘要、任务勾选历史。

区分三类结果：

```text
wiki-updates-made: 本次已实际写入或更新的 wiki 内容
wiki-updates-required: 仍需后续补充、迁移或由用户确认的 wiki 内容
wiki-updates-not-needed: 本次无需更新 wiki 的原因
```

`unispec archive <change-id>` 不会自动写入 `.wiki`，也不会自动完成 wiki 页面整理或目录化拆分。如果 wiki 沉淀是后续维护必须依赖的，或用户明确要求先沉淀，先完成或等待确认再归档；如果只是后续知识整理，可以在归档后继续处理，但必须输出 `wiki-updates-required`。不得修改 UniSpec managed baseline 页面来写项目特有长期规则，不得把 proposal / design / review-report / test-report 原文搬进 wiki。

长页面整理只作为维护 guidance，不是 CLI validate 或 archive 门禁。判断信号包括：页面约 250-300 行以上、H2 / H3 过多、同一页面承载多个独立稳定主题、维护者经常只引用某一节，或页面已经阻碍导航、维护和精确链接。若用户明确确认整理或拆分，可以协助：

- 保持单一主题页面为单页，只整理标题和入口。
- 将多个稳定主题的长页面升级为 `<主题>/INDEX.md + 01-<子主题>.md` 结构。
- 为每个新增目录提供 `INDEX.md`。
- 迁移或重算 frontmatter：子页可继承 `owner`、`package`、`audience`、`version_scope`，但 `source_of_truth` 必须按子页事实来源重新判断，`last_reviewed` 更新为整理日期。
- 让导航 `INDEX.md` 只做范围说明、阅读路径、子页索引和事实来源链接，不复制源码、规格、配置或 report 正文。
- 更新父目录 `INDEX.md`、栏目 `INDEX.md`、必要的 `.wiki/INDEX.md`、其它 wiki 反向链接，以及生成 Skill / reference / 测试中的旧链接。

未获得用户确认、链接迁移未完成、frontmatter / SSOT 尚未核对，或目录化拆分只完成一部分时，必须写入 `wiki-updates-required`，不得写成 `wiki-updates-made` 或 `wiki-updates-not-needed`。

7. **执行归档**

执行：

```bash
unispec archive <change-id>
```

当前 CLI 会把：

```text
.spec/changes/<change-id>/
```

移动到：

```text
.spec/archive/YYYY-MM-DD-<change-id>/
```

本 Skill 不在归档前手动修改 `meta.yaml.stage` 为 `archive`。如果未来需要归档后记录 `stage: archive`，应由 CLI 统一处理。

8. **归档后校验**

归档命令成功后检查：

- `.spec/changes/<change-id>/` 不存在。
- `.spec/archive/YYYY-MM-DD-<change-id>/` 存在。
- 普通 change / child change 保留 `proposal.md`、`design.md`、`system-tests.md`、`tasks.md`、`review-report.md`、`test-report.md`、`meta.yaml`。
- parent change 保留 `split.md`、`meta.yaml`，以及可能存在的 `research/`。
- child change 归档后，active parent 的 `meta.yaml` 和 `split.md` 已同步该 child 的归档状态。
- `wiki-updates-made` 与实际文件改动一致。
- `wiki-updates-required` 清楚列出后续动作。
- `wiki-updates-not-needed` 说明无需更新的原因；有待处理项时不得写成无需更新。

如果归档命令失败，停止并报告失败原因，不手动移动目录绕过 CLI。

**输出**

最终汇报字段：

```text
archive-result: success | fail
change-id:
archived-to:
artifacts-preserved:
- <普通 / child: proposal.md, design.md, system-tests.md, tasks.md, review-report.md, test-report.md, meta.yaml>
- <parent: split.md, meta.yaml>
wiki-updates-made:
- ...
wiki-updates-required:
- ...
wiki-updates-not-needed:
- ...
warnings:
- ...
next-step:
- ...
```

**产物更新指南**

- 归档前一般不修改 change artifacts。
- 不手动把 `meta.yaml.stage` 改成 `archive`；当前归档事实由目录位置表达。
- 如果归档前发现 artifacts 不准确，暂停并回到对应阶段修正，不在 archive 阶段悄悄改需求、设计、用例或任务。
- 可以按用户确认协助更新 `.wiki` 的长期页面、栏目索引或对外契约入口，但必须区分已完成更新、仍需后续处理的更新和无需更新的原因。
- 可以按用户确认协助整理长页面、更新反向链接，或把长页面目录化拆分为 `<主题>/INDEX.md + 01-*.md`。
- 不把未完成的 `.wiki` 沉淀汇报为已完成。
- 不把未确认或未完成的长页面整理、目录化拆分、frontmatter / SSOT 迁移、链接迁移汇报为已完成。
- 不修改 UniSpec managed baseline 页面来写项目特有长期规则，不把 archived change artifact 原文搬进 wiki。

**暂停条件**

遇到以下情况必须暂停：

- change-id 无法确定。
- 必需 artifacts 缺失、为空或 `meta.yaml` 不可解析。
- `meta.yaml.id` 与 change-id 不一致。
- 普通 change / child change 的 `meta.yaml.stage` 不是 `verification`。
- 普通 change / child change 的 `review-report.md` 或 `test-report.md` 缺失、frontmatter 无效或结果未通过。
- 普通 change / child change 的 tasks、system-tests、review 或 verification 仍有 blocking issues。
- parent change 的任一 child 未实际归档。
- `.spec/archive/YYYY-MM-DD-<change-id>/` 已存在。
- 必须先完成的 `.wiki` 沉淀仍未完成或需要用户确认。
- `unispec archive <change-id>` 执行失败。

**约束**

- 不归档有 blocking issues 的普通 change / child change。
- 不归档 verification 未通过的普通 change / child change。
- 不归档仍有 active child 的 parent change；报告证据不能替代 child 已归档事实。
- 不覆盖、合并或删除已有归档。
- 不删除 change 历史产物。
- 不把 `.docs` 临时文档当作长期沉淀。
- 不跳过 `.wiki` 更新检查。
- 不主动扫描全 `.wiki` 树来寻找无关长页面；只检查与当前 change 直接相关的 wiki 页面和索引。
- 不重新设计、重新实现或重新 review。
- 不自行声称最终业务验收已完成。
