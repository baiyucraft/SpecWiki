# Research 报告模板

用于 `unispec-explore`、`unispec-propose`、`unispec-design` 生成 `.spec/changes/<change-id>/research/<topic>.md`。

Research 是阶段性调研材料，用来记录 evidence、context、trade-off、risks、unknowns 和子代理调研结论。只要调研结论会影响当前阶段 artifact，或对话上下文不足以稳定承载调研信息，就可以生成 research 报告。它只服务当前阶段 artifact，不属于 required artifacts，不写入 `meta.yaml.artifacts`，也不作为 `unispec validate` 的通过标准。

## 使用规则

- 文件名按主题使用 kebab-case，例如 `research/current-cli-flow.md`。
- 正文标题使用中文。
- 只记录对当前阶段 artifact 有复用价值的证据、判断和结论。
- 创建后必须在对应 artifact 的 `## 参考资料` 中引用相对路径。
- explore 阶段判断为 `single-change` 且需要落盘 research 时，可先创建 standalone exploration stub；后续 `unispec-propose` 必须读取该 research，并判断是否还需要补充 proposal 阶段调研。
- 不适用的小节可以删除。
- 不写实现代码，不承载正式设计、任务计划或归档结论。
- 不写 `TBD`、`TODO`、空泛占位或无法追溯来源的结论。

## 调研原则

- **Grounded**：优先基于真实代码、文档、配置、历史 artifact 或用户输入，不凭空假设。
- **Bounded**：调研服务当前阶段目标，足以支撑 artifact 后停止，不无差别扫描全仓库。
- **Evidence first**：结论需要能追溯到文件、命令输出、用户确认或子代理报告。
- **Decision oriented**：记录调研如何影响 split / proposal / design，而不是堆积资料摘要。
- **No implementation**：research 阶段不写业务代码、不修改实现文件。

## 生成条件

满足任一条件即可生成 research 报告：

- 用户要求“调研一下”“研究一下”“查一下现状”或要求形成报告。
- 需求边界还不完全清楚，但已有阶段性结论需要沉淀。
- 需要比较多个方案、多个模块、多个影响点或多个约束。
- 调研包含多条证据、多个子代理结论或跨多轮对话，继续只保留在对话上下文中可能导致记忆缺失、压缩丢失或后续误引用。
- 后续 proposal、design、plan 或 review 可能需要引用同一份证据；直接写入正式 artifact 又会让正文变冗长。

以下情况不要生成 research 报告：

- 只是一两句话能说清的小事实核对。
- 没有可追溯证据。
- 只是临时命令输出、一次性验证日志或无长期复用价值的原始记录。
- 会把未确认假设包装成结论。
- 会替代 proposal、design、system-tests 或 tasks 的正式内容。

## 阶段边界

| 阶段 | 服务边界 | 可影响 | 禁止承载 | 引用位置 |
| --- | --- | --- | --- | --- |
| explore | split / deliveryShape / proposal 边界输入 | child 边界、顺序、依赖、风险、standalone change 边界 | proposal 正文、架构设计、接口设计、实现计划 | `split.md` 或后续 `proposal.md` |
| propose | proposal | problem、goals、non-goals、success criteria、impact scope | 拆分方案、架构设计、接口设计、实现计划 | `proposal.md` |
| design | design | 模块、接口、数据结构、流程、兼容性、验证方向 | 改变 proposal 目标、非目标或成功标准 | `design.md` |

## 停止条件

满足下列条件后停止调研，进入当前阶段 artifact 编写：

- 已能回答当前阶段的核心问题。
- 已识别关键 evidence、risks、unknowns。
- 已能判断哪些内容写入当前 artifact，哪些 deferred 到后续阶段。
- 剩余不确定性不会阻塞当前 artifact，可以写入 risks / unknowns / deferred items。

## 模板

~~~md
# <调研主题>

## 调研目的

- 阶段：<explore | propose | design>
- 关联 change：<change-id>
- 服务边界：<split | proposal | design>
- 要回答的问题：<问题>
- 停止条件：<本次调研达到什么程度即可停止>

## 结论摘要

- <结论>
- <结论>

## 已读取资料

| 路径 / 来源 | 目的 | 关键发现 |
| --- | --- | --- |
| <path> | <为什么读取> | <发现> |

## 子代理调研

| 子代理 / 角色 | 调研主题 | 结论 | 主会话复核 |
| --- | --- | --- | --- |
| <专责调研 agent / 通用 agent> | <topic> | <summary> | <accepted / adjusted / rejected + reason> |

> 未使用子代理时删除本节。

## 关键发现

### <发现标题>

- 证据：<path 或用户确认>
- 说明：<说明>
- 影响：<对当前阶段 artifact 的影响>

## 方案与取舍

| 选项 | 优点 | 缺点 | 结论 |
| --- | --- | --- | --- |
| <option> | <pros> | <cons> | <采用 / 不采用 / deferred> |

> 没有多方案比较时删除本节。

## 风险与未知项

| 项目 | 影响 | 处理方式 |
| --- | --- | --- |
| <risk / unknown> | <impact> | <写入当前 artifact / deferred-to-design / deferred-to-plan / 需要用户确认> |

## 对当前 artifact 的影响

- 应写入：<split.md | proposal.md | design.md>
- 影响内容：
  - <具体内容>
- 后续阶段处理：
  - <deferred-to-proposal | deferred-to-design | deferred-to-plan | 无>

## 未采纳内容

- <调研中发现但不进入当前 artifact 的内容及原因>
~~~
