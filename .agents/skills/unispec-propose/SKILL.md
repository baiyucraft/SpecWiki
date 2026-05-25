---
name: unispec-propose
description: 为具体 UniSpec change 生成 proposal.md 与 meta.yaml。当用户已明确要创建单个 change 时使用。
metadata:
  author: unispec
  generatedBy: "0.1.0"
---

覆盖 `proposal` 和 `delivery`：为一个具体、可独立验收的 change 创建 `proposal.md` 和 `meta.yaml`，并在 metadata 中记录 `deliveryShape`，为 `design` 阶段提供输入。

覆盖阶段：

```text
proposal
delivery
```

**重要：本 Skill 只负责一个具体 change 的 proposal，不负责 multi-change 拆分。若仍需拆分或判断交付形态，先使用 `unispec-explore`。**

---

**输入**

```text
用户提出的需求或问题
.spec/changes/<child-change-id>/meta.yaml（当从 multi-change child stub 创建 proposal 时）
.spec/changes/<change-id>/meta.yaml（当从 standalone exploration stub 创建 proposal 时）
.spec/changes/<change-id>/research/ 下已有 explore research（如有）
.spec/changes/<parent-change-id>/split.md
.spec/changes/<parent-change-id>/meta.yaml
相关代码、测试、配置和文档
.wiki/ 中的项目规范和长期知识
.spec/changes/ 中已有的 active changes
.spec/archive/ 中可能相关的历史 changes
```

如果用户还在开放讨论、没有准备创建具体 change，或需求明显需要拆分，停止并建议先进入 `unispec-explore`。

**步骤**

1. **判断输入是否足够**

如果用户没有说明要解决的问题、目标用户、期望结果或约束，先提问，不要创建 change。

只问会阻塞 proposal 判断的问题。一次只问一个关键问题，优先确认：

- 要解决的 problem 是什么。
- 目标用户、使用者或受影响系统是谁。
- 期望结果和 success criteria 是什么。
- 已知约束、截止条件、兼容要求或非目标是什么。

可以合理推断的细节直接写入 `unknowns` 或 `risks`，不要用大量问题阻塞推进。不要把澄清做成审讯式问题列表。

2. **确定 change-id 并检查冲突**

从用户需求推导 kebab-case 名称，例如：

```text
"添加登录审计日志" -> add-login-audit-log
"修复归档校验误报" -> fix-archive-validation-false-positive
```

规则：

- 使用小写字母、数字和连字符。
- 表达要交付的业务或工程结果，不用泛泛的 `update-system`。
- 如果来自 parent `split.md`，child id 必须使用 parent `meta.yaml.multiChange.children` 中已声明的 id，并与 child stub 目录一致。
- 读取 `.spec/changes/`，检查是否已有同名或相邻 active change。
- 读取 `.spec/archive/`，检查是否已有相关历史 change 可参考。
- 如果 `.spec/changes/<change-id>/` 已存在且是 child stub，继续使用该 stub 补齐 `proposal.md`。
- 如果 `.spec/changes/<change-id>/` 已存在且是 standalone exploration stub，继续使用该 stub 补齐 `proposal.md`。
- 如果 `.spec/changes/<change-id>/` 已存在但不是目标 child stub，询问是继续更新该 change，还是换一个名称。
- 不允许在本 Skill 中自行重命名 child；需要改名时，回到 `unispec-explore` 修改 parent split。

3. **读取上下文并做定向调研**

读取与当前 change 直接相关的上下文。若来自 parent split，先读取 parent `split.md`、parent `meta.yaml` 和 child stub `meta.yaml`，确认该 child 的目标、验收边界、顺序和依赖。若来自 standalone exploration stub，先读取该 stub 的 `meta.yaml` 和已有 `research/` 报告。

本步骤不是整体探索。它只服务当前一个具体 change 的 proposal，不重新判断整体交付形态，不重新拆分 child，不扩大 parent scope。

本步骤只补齐当前 change 的 problem、goals、non-goals、success criteria 和 impact scope。若发现需要重新判断 `deliveryShape`、拆分 child、调整 child 边界、修改 child id / order / dependsOn，立即停止并回到 `unispec-explore` 更新 `split.md` 和 child stub。

已有 explore research 只是输入证据，不是跳过 proposal 调研的理由。必须检查它是否足以支撑当前 proposal 所需的 `problem`、`goals`、`non-goals`、`success criteria` 和 `impact scope`；足够时复用并在 `proposal.md` 引用，不足时继续做定向 proposal 调研，必要时新增 `research/` 报告。

如果需要补充只读调研，可使用当前 tool 或运行环境提供的只读子代理并行调研；调研问题必须限定为当前 change 的 proposal 事实，例如现有行为、影响范围、约束、风险和验收依据。

常见调研方向：

- `.wiki/INDEX.md` 与 `.wiki/` 中相关项目规范；只引用影响当前 proposal 边界的稳定事实，不复制 wiki 正文。
- `.spec/changes/` 中已有 active changes。
- `.spec/archive/` 中可能相关的历史 changes。
- 相关代码、测试、配置、README。
- 项目内既有模式、命名约定和流程约束。

如果调研结论会影响当前 proposal，或需要汇总多条证据 / 子代理结论，可以写入辅助调研目录：

```text
.spec/changes/<change-id>/research/<topic>.md
```

`research/` 是辅助调研材料。创建前读取 `references/research-template.md`；创建后必须在 `proposal.md` 的 `## 参考资料` 中引用相对路径，并在最终输出中列出报告路径。

proposal 阶段的 research 报告不得承载拆分方案、架构设计、接口设计或实现计划；这些内容分别回到 `unispec-explore`、`unispec-design` 或 `unispec-plan`。

4. **判断 deliveryShape**

本 Skill 生成的具体 change 默认是 `single-change`：

- standalone change：`deliveryShape: single-change`。
- parent split 下的 child change：`deliveryShape: single-change`，并在 `multiChange` 中记录 parent、order 和 dependsOn。

如果当前输入实际是 `multi-change` parent，不要创建 proposal；回到 `unispec-explore` 创建或修订 `split.md`。

如果当前输入是 child stub，确认：

- child `stage` 为 `exploration`。
- child `deliveryShape` 为 `single-change`。
- child `multiChange.role` 为 `child`。
- child `multiChange.parent` 指向 parent change。
- child `order` 和 `dependsOn` 与 parent `meta.yaml.multiChange.children` 一致。

如果当前输入是 standalone exploration stub，确认：

- `stage` 为 `exploration`。
- `deliveryShape` 为 `single-change`。
- 不存在 `multiChange`。
- `artifacts.proposal.status` 为 `missing`。
- 不存在 `proposal.md`。
- 已有 `research/` 已被读取，并已判断是否需要补充 proposal 阶段调研。

5. **形成 proposal 方案并等待确认**

在写文件前，先向用户给出简短方案。简单 change 可以压缩表达；复杂 change 必须列出推荐边界和关键取舍：

```text
change-id: <kebab-case>
parent-change-id: <parent-change-id | 无>
deliveryShape: single-change
澄清结论:
问题概述:
目标:
非目标:
影响范围:
推荐边界:
备选边界:
deferred-to-design:
风险 / 未知项:
将创建:
- .spec/changes/<change-id>/proposal.md
- .spec/changes/<change-id>/meta.yaml
```

`deferred-to-design` 只写哪些问题留到 `design.md` 判断，不写具体实现方案、架构、接口或数据流。

等待用户确认后再创建目录和写文件。若用户要求调整，先更新方案。

6. **创建或复用 change 目录**

standalone change 直接创建目录：

```text
.spec/changes/<change-id>/
```

child change 必须复用 `unispec-explore` 已创建的 stub 目录：

```text
.spec/changes/<child-change-id>/
```

standalone exploration stub 必须复用 `unispec-explore` 已创建的目录：

```text
.spec/changes/<change-id>/
```

不要引用不存在的脚手架命令；当前应直接创建目录和文件。

7. **创建 proposal.md**

创建 `proposal.md` 前，先读取 `references/proposal-template.md`，按其中的固定结构生成 proposal。该 reference 只用于 proposal 写作，不要写入 `proposal.md` 的 `参考资料`。

`proposal.md` 必须保留以下二级标题：

```text
问题
目标
非目标
成功标准
影响范围
交付形态
风险
未知项
参考资料
```

`references/proposal-template.md` 中的三级标题和提示项按需使用；简单工程 change 不创建不适用的小节，不保留空表格、空标题或占位文本。

写作要求：

- 聚焦 `为什么做`、`做什么`、`如何验收`。
- 不写设计、实现、任务或排期细节。
- 如果关键事实不确定，明确标入 `unknowns`，不要伪造。
- 如果创建了 research 报告，必须在 `参考资料` 中引用；没有 research 报告时写 `无`。

8. **创建 meta.yaml**

standalone change 的 `meta.yaml` 必须至少包含：

```yaml
id: <change-id>
stage: proposal
deliveryShape: single-change
artifacts:
  proposal:
    status: present
  metadata:
    status: present
```

child change 读取并更新已有 stub `meta.yaml`。必须保留 parent 关联，并将 `stage` 从 `exploration` 改为 `proposal`：

```yaml
id: <parent-change-id>-<child-topic>
stage: proposal
deliveryShape: single-change
multiChange:
  role: child
  parent: <parent-change-id>
  order: 1
  dependsOn: []
artifacts:
  proposal:
    status: present
  metadata:
    status: present
```

当前 Skill 初始化 `meta.yaml` 时保留 `stage: proposal`；后续 stage 变更由后续 workflow / CLI 处理。

standalone exploration stub 读取并更新已有 `meta.yaml`。必须将 `stage` 从 `exploration` 改为 `proposal`，将 `artifacts.proposal.status` 改为 `present`，保留已有 `id`、`deliveryShape`、`artifacts.metadata` 和未知字段，不新增 `multiChange`。

更新 child stub 时必须保留已有 `id`、`deliveryShape`、`multiChange.parent`、`order`、`dependsOn` 和未知字段。不得为了写 proposal 改 child id；需要改名时回到 `unispec-explore` 修订 parent split 和 child stub。

`deliveryShape` 在 `meta.yaml` 中使用 camelCase；`proposal.md` 中使用中文标题 `交付形态`。

9. **校验 proposal 阶段产物和 deliveryShape 字段**

完成后检查：

- `.spec/changes/<change-id>/proposal.md` 存在且非空。
- `.spec/changes/<change-id>/meta.yaml` 存在且非空。
- `meta.yaml` 字段能被当前 UniSpec metadata schema 读取。
- `proposal.md` 包含所有必需中文标题。
- `meta.yaml` 的 `deliveryShape` 与 `proposal.md` 的 `交付形态` 内容一致。
- 如果是 child change，`meta.yaml.stage` 已从 `exploration` 更新为 `proposal`，且 `meta.yaml.multiChange.parent`、`order`、`dependsOn` 与 parent `split.md` 一致。
- 如果是 standalone exploration stub，`meta.yaml.stage` 已从 `exploration` 更新为 `proposal`，且没有新增 `multiChange`。
- 如果存在 `research/` 报告，`proposal.md` 的 `参考资料` 已引用相对路径。

可以运行 `unispec status` 或 `unispec show <change-id>` 观察状态。

可以运行 `unispec validate <change-id>` 校验 proposal 阶段必需产物；此时 `design.md`、`system-tests.md`、`tasks.md` 预期尚未生成，不应被视为阻塞项。

**输出**

创建或更新：

```text
.spec/changes/<change-id>/proposal.md
.spec/changes/<change-id>/meta.yaml
```

不要生成：

```text
.spec/changes/<change-id>/design.md
.spec/changes/<change-id>/system-tests.md
.spec/changes/<change-id>/tasks.md
```

最终汇报字段：
- change-id
- change 目录
- `proposal.md` 路径
- `meta.yaml` 路径
- `deliveryShape`
- research 报告路径（如有）
- parent-change-id（如有）
- risks / unknowns 摘要
- 下一步：使用 `unispec-design`

---

**产物生成指南**

- `proposal.md` 的具体结构和写作边界以 `references/proposal-template.md` 为准。
- `meta.yaml` 中使用 `deliveryShape`；`proposal.md` 中使用中文标题 `交付形态`。
- 本 Skill 生成的 `deliveryShape` 应为 `single-change`；multi-change parent 由 `unispec-explore` 管理。
- `research/` 的具体结构和写作边界以 `references/research-template.md` 为准。

---

**约束**

- 不写实现代码。
- 不生成 `design.md`、`system-tests.md`、`tasks.md`。
- 不把多个不可独立验收的问题塞进一个 change。
- 复杂需求必须先用 `unispec-explore` 拆分为多个 child change。
- 不引用当前 CLI 中不存在的 `unispec new` 或 `unispec change` 命令。
- 如果用户要求直接实现，提醒先完成 proposal、design、system-tests、tasks 流程。
