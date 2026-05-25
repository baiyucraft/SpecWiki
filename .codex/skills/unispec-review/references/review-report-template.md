# Review Report 模板

用于 `unispec-review` 生成或更新 `.spec/changes/<change-id>/review-report.md`。

Review Report 是整个 change 的最终审查报告，用于记录实现是否符合 proposal、design、system-tests 和 tasks。它不同于 `tasks.md` 中每个大 task 的局部质量检查；本文件给出整个 change 的 review 结论。

## 使用规则

- `review-report.md` 必须包含 YAML frontmatter。
- frontmatter 必须包含 `review-result` 和 `scope`。
- `review-result` 取值为 `pass`、`fail` 或 `partial`；`scope` 取值为 `full` 或 `partial`。
- `review-result: pass` 只能在无 blocking issues、实现与 artifacts 一致且剩余风险可接受时写入。
- 只有 `review-result: pass` 且 `scope: full` 可作为归档证据。
- fail / partial 必须记录原因、影响范围和回退路径。
- 没有内容的小节写 `无`，不要保留占位文本。

## 固定结构

~~~md
---
review-result: pass
scope: full
---

# <change-id> 审查报告

## 审查结论

- review-result: pass
- scope: full
- 结论摘要：<一句话说明整体审查结果>

## 审查概览

| 项目 | 内容 |
| --- | --- |
| change-id | <change-id> |
| 审查类型 | full / partial |
| 审查对象 | <整体 change 或指定范围> |
| 问题总数 | <阻塞 + 非阻塞 + artifact 同步问题数量> |

## 审查范围

- <整体 change | partial 范围>

## 采用的审查规范

| 规范 | 选择依据 | 覆盖范围 |
| --- | --- | --- |
| references/review-standard.md | always | <整体 change> |
| <references/review-standard.*.md> | <用户指定 / artifacts / wiki / 配置文件 / 文件后缀> | <覆盖文件或模块> |

## 部分范围

- <仅当 scope: partial 时填写审查边界、未覆盖内容和原因；scope: full 时写“无”>

## Artifact 一致性

- proposal.md：<符合 / 不符合 / 不适用 + 说明>
- design.md：<符合 / 不符合 / 不适用 + 说明>
- system-tests.md：<符合 / 不符合 / 不适用 + 说明>
- tasks.md：<符合 / 不符合 / 不适用 + 说明>

## 问题统计

| 类型 | 数量 | 说明 |
| --- | --- | --- |
| 阻塞问题 | <数量> | <无则写 0> |
| 非阻塞问题 | <数量> | <无则写 0> |
| Artifact 同步问题 | <数量> | <无则写 0> |
| Wiki 同步问题 | <数量> | <无则写 0> |
| 证据缺口 | <数量> | <无则写 0> |

## 阻塞问题

- <阻止 verification 或归档的问题；没有则写“无”>

## 非阻塞问题

- <不阻止归档但需要记录的问题；没有则写“无”>

## Artifact 同步问题

- <需要回到 proposal / design / plan / apply 同步的问题；没有则写“无”>

## Wiki 同步问题

- <实现已改变长期知识、公开契约、模块职责或流程规则但尚未沉淀到 .wiki 的 wiki-sync issues；没有则写“无”>

## 证据缺口

- <缺少 diff、相关文件、测试证据或 artifact 事实的问题；没有则写“无”>

## 剩余风险

- <已接受的风险；没有则写“无”>

## 下一步

- <回到哪个阶段，或继续生成 test-report.md 后使用 unispec-archive>
~~~

## 写作边界

- 只记录整个 change 的最终 review，不记录大 task 内的局部检查细节。
- 阻塞问题必须说明阻塞原因、影响范围和建议回到哪个阶段。
- Artifact 同步问题记录 artifacts 与实现不一致的问题，不直接篡改 artifacts 来掩盖偏差。
- Wiki 同步问题只报告需要 archive 沉淀的长期知识缺口，不直接更新 `.wiki`。
- 采用的审查规范只记录本次采用的 standards、选择依据和覆盖范围，不复制 standards 正文。
- 不写测试命令明细；测试和 verification 证据写入 `test-report.md`。
- 归档就绪由 CLI 根据 stage、required artifacts 和两个报告 frontmatter 计算，不在报告中手写归档结论字段。
