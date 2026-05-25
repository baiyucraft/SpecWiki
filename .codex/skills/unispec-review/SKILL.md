---
name: unispec-review
description: 执行最终 review 与 verification 并生成正式报告。当用户完成实现并需要审查和验证时使用。
metadata:
  author: unispec
  generatedBy: "0.1.0"
---

一次性覆盖 `review` 阶段的最终审查和 `verification` 阶段的验证确认：两个项目级 agent 先生成 draft 报告，`unispec-review` 再校验 draft、签发正式 `review-report.md` / `test-report.md`，并在 full/pass 时推进 `stage: verification`。

```text
unispec-code-reviewer -> review-report.draft.md
unispec-test-verifier -> test-report.draft.md
unispec-review -> review-report.md + test-report.md
```

**职责边界**

- 本 Skill 负责流程编排、draft 校验、正式报告签发、draft 清理和 stage 推进。
- 两个 project agent 是 `.agents/agents/*.md` role protocol files，由本 Skill 显式调用；它们不参与 Skill discovery，不写正式报告、不写正式 YAML frontmatter、不推进 stage。
- 本 Skill 不直接修改代码，不执行归档，不更新 `.wiki`，不生成旧 `review.md`，不手写归档结论字段。
- 整体 review 覆盖整个 change，不等同于 `tasks.md` 中每个大 task 的局部质量检查。

---

**输入**

```text
.spec/changes/<change-id>/proposal.md
.spec/changes/<change-id>/design.md
.spec/changes/<change-id>/system-tests.md
.spec/changes/<change-id>/tasks.md
.spec/changes/<change-id>/meta.yaml
.spec/changes/<change-id>/review-report.md（已有时读取并更新）
.spec/changes/<change-id>/test-report.md（已有时读取并更新）
.spec/changes/<change-id>/review-report.draft.md（本次 review 中间产物）
.spec/changes/<change-id>/test-report.draft.md（本次 verification 中间产物）
.agents/agents/unispec-code-reviewer.md（存在时读取）
.agents/agents/unispec-test-verifier.md（存在时读取）
.agents/skills/unispec-review/references/review-standard.md
.agents/skills/unispec-review/references/review-standard.*.md（按项目事实选择）
.agents/skills/unispec-review/references/review-report-template.md
.agents/skills/unispec-review/references/test-report-template.md
代码 diff 或实现证据
测试结果和验证记录
.spec/config.yaml（读取顶层 playwright 和 imageAnalysis；缺失按 false）
必要时读取 research/ 中被引用的报告
```

partial review 只在用户明确要求时执行。partial 必须写入 `scope: partial`，`test-report.md` 写 `verification-result: skipped` 和原因，不推进 stage，也不是归档证据。如果当前 change 已处于 `stage: verification`，写入 partial / skipped 后会失去 archive readiness；是否回退 stage 必须由用户确认。

**步骤**

1. **选择 change 并确认可 review**

如果用户指定 change-id，使用该 change。否则：

- 从当前对话推断 change-id。
- 如果只有一个 active change，可以使用它。
- 如果存在多个 active changes 且无法判断，读取 `.spec/changes/` 并让用户选择。

确认以下文件存在且非空：

```text
.spec/changes/<change-id>/proposal.md
.spec/changes/<change-id>/design.md
.spec/changes/<change-id>/system-tests.md
.spec/changes/<change-id>/tasks.md
.spec/changes/<change-id>/meta.yaml
```

确认 `meta.yaml` 可解析，并保留已有 `id`、`deliveryShape`、artifact 状态和未知字段。如果 `multiChange.role` 为 `parent`，停止；parent 的归档由 `unispec-archive` 在 children 完成后处理，不做整体代码审查。

如果 `tasks.md` 中仍有未完成小 task，默认暂停并建议回到 `unispec-apply`。只有用户明确要求 partial review 时，才继续审查指定范围。

2. **准备上下文**

读取 proposal、design、system-tests、tasks、meta、既有 reports、代码 diff / 实现证据、测试结果、验证记录、相关 research、两个 agent 定义、review standards 列表和两个 report template。

没有足够实现证据时暂停；不要在缺少 diff、改动摘要或相关文件的情况下强行 review。

3. **清理旧 draft 并生成两个 agent draft**

每次开始本轮 review 前，先删除固定 draft 路径中的旧文件，避免误读历史初稿：

```text
.spec/changes/<change-id>/review-report.draft.md
.spec/changes/<change-id>/test-report.draft.md
```

只删除以上固定文件名，不扫描目录，不删除其它用户文件。

调度 `unispec-code-reviewer` 生成 `review-report.draft.md`：

```text
artifacts: proposal / design / system-tests / tasks / meta
scope: full | partial
diff: code diff / changed files / related files
tests: test results and verification evidence
standardsDirectory: .agents/skills/unispec-review/references/
availableStandards: review-standard*.md
reportTemplate: .agents/skills/unispec-review/references/review-report-template.md
```

调度或 fallback 执行 test verifier agent，生成 `test-report.draft.md`：

```text
artifacts: proposal / design / system-tests / tasks / meta
scope: full | partial
reviewDraft: .spec/changes/<change-id>/review-report.draft.md
tests: unit test outputs, framework reports, coverage reports, system test evidence, manual verification evidence
playwrightPolicy: .spec/config.yaml top-level playwright boolean value, missing-as-false, invalid-as-false with configuration issue
browserInteractionEvidence: configured playwright, verification mode, Playwright availability when enabled, entry URL, browser / viewport, actions, assertion points, evidence paths, manual verification steps, fallback reason
imageAnalysisPolicy: .spec/config.yaml top-level imageAnalysis boolean value, missing-as-false, invalid-as-false with configuration issue
evidenceDirectory: optional .spec/changes/<change-id>/evidence/ for attachments referenced by test-report.md
reportTemplate: .agents/skills/unispec-review/references/test-report-template.md
```

如果宿主不支持调度独立 agent，当前执行者必须按同一 role protocol、同一输入和对应 report template 本地 fallback，并写入同名 draft。fallback 不降低最终 review 和 verification 要求，最终汇报要说明未使用独立 agent。

两个 draft 必须按对应 template 的正文章节和字段语义提供可映射内容，但不得写正式 YAML frontmatter，不得签发最终 `review-result` / `verification-result`。

4. **校验任务和 draft**

检查：

- 所有小 task 是否已从 `- [ ]` 勾选为 `- [x]`。
- 每个大 task 的 `### CheckList` 是否完成。
- checkbox 是否有代码、测试或验证证据支撑。
- lint、typecheck、static analysis、formatter check、compiler check 或语言 / 框架自带检查是否已运行，或是否记录了无法运行原因。
- tasks 是否仍能追溯到 design 和 system-tests。
- `review-report.draft.md` 是否可映射到 `review-report-template.md` 的正文结构，且包含 `采用的审查规范`、阻塞问题、非阻塞问题、Artifact 同步问题、Wiki 同步问题和证据缺口。
- `test-report.draft.md` 是否可映射到 `test-report-template.md` 的正文结构，且包含验证建议、执行信息、测试结果汇总、单元测试报告、系统测试报告、浏览器交互验证 evidence、测试命令结果、系统测试用例覆盖、成功标准覆盖、失败项、未验证项和证据缺口。
- 浏览器交互验证 evidence 是否记录 configured playwright、verification mode、`playwright: true` 时的 host Playwright availability、入口 URL、浏览器 / viewport、动作摘要、断言点、证据路径、configured imageAnalysis、actual image analysis 和 fallback reason。
- 当顶层 `playwright` 缺失、无效或为 `false` 时，draft 是否避免规划或执行 Playwright，且没有因为缺少 Playwright evidence 判风险；涉及 UI / browser 的 `ST-*` 是否记录手工验证步骤、人工断言、替代证据、证据路径或 fallback reason。
- 当顶层 `playwright: true` 时，关键 UI / browser 用例没有 Playwright 且缺少充分替代证据，才记录 evidence gap。
- 当顶层 `imageAnalysis` 缺失、无效或为 `false` 时，draft 是否避免读取、解释、描述或比较截图 / 图片内容，且没有把“截图看起来正确”作为 pass 证据。
- 当顶层 `imageAnalysis: true` 时，draft 是否仍包含覆盖相关 `ST-*` / 成功标准的非图片断言或替代证据。
- 是否存在实现已改变长期知识、公开契约、模块职责或流程规则但尚未沉淀到 `.wiki` 的 wiki-sync issues。

发现 task 误勾、证据不足、draft 缺字段、draft 写了正式 frontmatter 或无法映射模板时，不签发正式报告，保留 draft 并要求补齐。

5. **签发正式报告**

读取两个 draft 和两个 template，由本 Skill 签发正式报告：

- `review-report.md` 使用 `review-report-template.md`，由 reviewer draft 映射审查概览、采用的审查规范、问题统计、阻塞问题、非阻塞问题、Artifact 同步问题、证据缺口和剩余风险。
- `test-report.md` 使用 `test-report-template.md`，由 test verifier draft 映射执行信息、测试结果汇总、单元测试报告、系统测试报告、浏览器交互验证 evidence、测试命令结果、系统测试用例覆盖、成功标准覆盖、失败项、未验证项和证据缺口。
- 正式 YAML frontmatter 只由本 Skill 写入，必须是单一合法值，不得写枚举串。

正式 `test-report.md` 是 verification evidence 的 SSOT。被报告采用且需要随 change 保留的 Playwright 附件可以放在 `.spec/changes/<change-id>/evidence/`；该目录只是 optional attachment directory，不是 required artifact，不写入 `meta.yaml.artifacts`。临时调试截图、失败探索输出和未采用 trace 放入 `.tmp/` 或清理。

如果 review 存在 blocking issues，`review-report.md` 写 `review-result: fail` 或 `partial`，`test-report.md` 可写 `verification-result: skipped` 并说明原因；不推进 stage。测试缺失或失败默认是 blocking issue，只有有明确不可运行原因和替代验证证据时才可降级。

6. **删除 draft 并判断结果**

只有正式 `review-report.md` 和 `test-report.md` 都成功写入后，才删除：

```text
.spec/changes/<change-id>/review-report.draft.md
.spec/changes/<change-id>/test-report.draft.md
```

删除 draft 与报告结论无关：pass / fail / partial / skipped 只要正式报告成功写入，都删除 draft。若正式报告写入失败、流程中断或无法确认写入结果，保留 draft 供诊断和恢复。

Draft 不是 runtime artifacts，不写入 `meta.yaml.artifacts`，不进入 validate / archive 门禁，不作为 show 证据。

正式报告 frontmatter 示例：

```yaml
# review-report.md
review-result: pass
scope: full

# test-report.md
verification-result: pass
scope: full
```

7. **通过后推进 stage**

只有同时满足以下条件时，才可将 `meta.yaml.stage` 更新为 `verification`：

- `review-report.md` frontmatter 为 `review-result: pass`。
- `test-report.md` frontmatter 为 `verification-result: pass`。
- 两个报告的 frontmatter 均为 `scope: full`。
- tasks 全部完成。
- system-tests 有通过结果或明确验证证据。
- design / tasks / code 一致。
- proposal 的 success criteria 已被验证覆盖。

`stage: verification` 表示已完成最终 review 和 verification，可以进入 archive；不表示已归档。归档就绪由 CLI 根据 stage、required artifacts 和两个报告 frontmatter 计算。

**输出**

最终汇报字段：

```text
change-id:
review-result: pass | fail | partial
verification-result: pass | fail | skipped
scope: full | partial
blocking issues:
- ...
non-blocking issues:
- ...
artifact-sync issues:
- ...
wiki-sync issues:
- ...
residual risks:
- ...
验证命令和结果:
- ...
更新文件:
- review-report.md
- test-report.md
- meta.yaml（仅当 stage 更新为 verification）
下一步:
- ...
```

**产物更新指南**

- 本 Skill 一般不修改代码。
- 本 Skill 可以删除本轮固定 draft 文件，但只在正式报告成功写入后删除。
- 不直接修改 `proposal.md`、`design.md`、`system-tests.md` 或 `tasks.md` 来掩盖实现偏差；需要同步时输出 artifact-sync issues，并建议回到对应阶段。
- 不直接更新 `.wiki`；需要沉淀长期知识或整理长页面时输出 wiki-sync issues，并交给 `unispec-archive` 检查或由用户确认处理。
- 每次最终 review 都要创建或更新 `review-report.md` 与 `test-report.md`；进入 `review` / `verification` / `archive` 阶段后会被 validate 和 archive 检查。
- `review-report.draft.md` 和 `test-report.draft.md` 是本次运行的中间产物，不进入 validate / archive 门禁。
- 只有 review + verification 均通过时，才可将 `meta.yaml.stage` 更新为 `verification`。
- 更新 `meta.yaml` 时必须保留已有 `id`、`deliveryShape`、`createdAt`、已有 artifact 状态和未知字段。runtime 归档证据以文件系统状态和两个报告 frontmatter 为准。
- 不设置 `stage: archive`；归档只由 `unispec-archive` 处理。

**暂停条件**

遇到以下情况必须暂停：

- 必需 artifacts 缺失、为空或 `meta.yaml` 不可解析。
- tasks 未完成，且用户没有明确要求 partial review。
- 缺少代码 diff、改动摘要、测试结果或其他实现证据。
- 测试失败，且无法在 review 范围内判断是否可接受。
- 发现需要改变 proposal 成功标准或 design 方向。
- 发现 artifact-sync issues 影响 verification 判断。
- 用户尚未完成必要的业务验收确认。

**约束**

- 不直接修改代码。
- 不更新 `.wiki`。
- 不跳过测试结果。
- 不生成旧单文件审查产物。
- 不手写归档结论字段。
- 不把 partial review 当作归档证据。
- 不把未验证内容标记为已通过。
- 不在 blocking issues 未解决时推进 stage 或归档。
- 不替用户做最终业务验收。
- 不自行归档。
