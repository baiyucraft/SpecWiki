---
name: unispec-explore
description: 探索需求、澄清问题并判断 deliveryShape。当用户想先讨论想法、调研问题或拆分多 change 时使用。
metadata:
  author: unispec
  generatedBy: "0.1.0"
---

覆盖 `exploration`：先澄清需求、调研现状并判断 `deliveryShape`；若需要 `multi-change`，创建 parent change 的 `split.md` / `meta.yaml`，并为每个 child 创建 stub 目录和 `meta.yaml`，为后续逐个 child change 的 `unispec-propose` 提供输入。若判断为 `single-change` 且需要沉淀 research，可创建 standalone exploration stub，只保存 `meta.yaml` 和 `research/`，不写 `proposal.md`。

覆盖阶段：

```text
exploration
```

**重要：本 Skill 只负责探索、切分、parent split、child stub 和必要的 standalone exploration stub，不写 proposal.md，不进入 design，不写实现代码。**

---

**输入**

```text
用户提出的需求、问题或探索方向
.wiki/ 中的项目规范和长期知识
.spec/changes/ 中已有的 active changes
.spec/archive/ 中可能相关的历史 changes
相关代码、测试、配置和文档
```

**步骤**

1. **判断输入是否足够**

确认用户是在探索问题、拆分范围或判断交付形态。若连问题背景、目标用户、期望结果或关键约束都无法判断，先问一个会阻塞探索的问题。

只问必要问题。可以合理推断的细节写入 `split.md` 的风险、未知项或后续确认项，不要用长问题列表阻塞推进。

2. **确定 parent change-id 并检查冲突**

为探索对象确定 parent change-id，使用 kebab-case。parent id 表达整体问题或交付主题；子 change id 默认使用 `<parent-change-id>-<child-topic>`。

检查 `.spec/changes/` 和 `.spec/archive/` 中是否已有同名或相邻 change。若 parent 目录已存在，询问是继续更新该 split，还是换一个 parent id。

3. **进行探索并调研现状**

当需求模糊、跨模块、存在多种切分方式，或用户仍在探索问题时，进行边界澄清和只读调研。它只服务于 `exploration` 阶段的 `deliveryShape` 判断与 parent / child 边界，不进入 proposal、设计或实现。

优先使用当前 tool 或运行环境提供的只读子代理并行调研。主 agent 默认拆出至少 2 个互不依赖的只读调研问题；极小且边界明确的需求可压缩为最小调研，但要说明无需展开的理由。

按需读取，不要无差别扫描全仓库。常见调研方向：

- `.wiki/INDEX.md` 与 `.wiki/` 中相关项目规范；如果 `.wiki/INDEX.md` 仍是初始化任务页，只把它视为 wiki 尚未初始化的信号。
- `.spec/changes/` 中已有 active changes。
- `.spec/archive/` 中可能相关的历史 changes。
- 相关代码、测试、配置、README。
- 项目内既有模式、命名约定和流程约束。

子代理只负责 read-only 调研，输出发现、证据路径、风险、未知项，以及对 scope 和 `deliveryShape` 的影响。主 agent 负责汇总、复核和取舍，只保留会影响切分和 proposal 边界的结论。

如果调研结论会影响当前 split / deliveryShape / 后续 proposal 边界，或需要汇总多条证据 / 子代理结论，可以写入辅助调研目录：

```text
.spec/changes/<change-id>/research/<topic>.md
```

`research/` 是辅助调研材料，不属于 required artifacts，不写入 `meta.yaml.artifacts`，不作为 `unispec validate` 通过标准。报告文件名根据实际主题使用 kebab-case，正文标题使用中文。multi-change parent 的 research 写入 parent change 目录，并必须在 `split.md` 的 `## 参考资料` 中引用相对路径；single-change exploration research 写入 standalone exploration stub，后续由 `unispec-propose` 在 `proposal.md` 的 `## 参考资料` 中引用。创建 research 报告后，必须在最终输出中列出报告路径。

创建 research 报告前，先读取本 Skill 的 `references/research-template.md`，按模板填写阶段、服务边界、证据、结论和对当前 artifact 的影响。

允许做：

- 探索 problem space，复述并校准真实问题。
- 挑战假设，识别隐藏前提和范围膨胀。
- 对需求做 reframe，区分业务目标、工程目标和操作性约束。
- 用 ASCII 图澄清范围、依赖、流程或方案对比。
- 比较可能的 scope 边界和 `deliveryShape`。
- 提出 risks、unknowns、spikes 或后续 design 阶段要处理的问题。

ASCII 图只用于当前沟通中的理解，不作为正式设计图写入 `split.md`：

```text
用户问题
  -> 真实目标
  -> 影响范围
  -> parent split 边界
  -> child proposal 边界
  -> deferred-to-design
```

停止条件：

- 已能判断 `problem`、`goals`、`non-goals`、`success-criteria`、`impact-scope`。
- 已能判断 `deliveryShape` 是 `single-change` 还是 `multi-change`。
- 若为 `multi-change`，已能列出 child change、顺序、依赖和验收边界。
- 剩余不确定性可以进入 `unknowns`、`risks` 或 `deferred-to-design`，不会阻塞 split。

4. **判断 deliveryShape**

如果是 `single-change`，不要创建 parent split。默认只输出推荐 change-id 和理由，让用户进入 `unispec-propose`。

只有当 explore 阶段已经形成需要沉淀的 research 时，才允许创建 standalone exploration stub。该 stub 用于给 research 提供稳定落点，不表示 proposal 已完成，也不能跳过 `unispec-propose` 的定向 proposal 调研。

single-change research stub 的写文件方案必须先让用户确认：

```text
change-id: <kebab-case>
deliveryShape: single-change
用途: standalone exploration stub，仅保存 explore research，proposal 尚未生成
将创建:
- .spec/changes/<change-id>/meta.yaml
- .spec/changes/<change-id>/research/<topic>.md
不会创建:
- .spec/changes/<change-id>/proposal.md
```

如果是 `multi-change`，继续形成 split 方案。multi-change 的判断条件包括多个可独立验收目标、明显不同的业务域 / 模块边界 / 团队边界 / 交付阶段、长依赖链，或需要隔离风险。

5. **形成 split 方案并等待确认**

在写 multi-change 文件前先向用户给出简短方案：

```text
parent-change-id: <kebab-case>
deliveryShape: multi-change
问题概述:
整体目标:
非目标:
影响范围:
拆分原则:
children:
- id: <parent-change-id>-<child-topic>
  order: 1
  dependsOn: []
  goal:
  acceptance:
风险 / 未知项:
将创建:
- .spec/changes/<parent-change-id>/split.md
- .spec/changes/<parent-change-id>/meta.yaml
- .spec/changes/<child-change-id>/meta.yaml
```

等待用户确认后再创建目录和写文件。若用户要求调整，先更新方案。

6. **创建 parent 和 child stub 目录**

直接创建目录：

```text
.spec/changes/<parent-change-id>/
.spec/changes/<child-change-id>/
```

child 目录是 stub，只写 `meta.yaml`，不写 `proposal.md`。这表示 child 已由 parent split 规划出来，但尚未完成 proposal 阶段。

7. **创建 standalone exploration stub（仅 single-change research 需要）**

如果 `deliveryShape` 是 `single-change` 且本阶段需要落盘 research，确认后创建：

```text
.spec/changes/<change-id>/
.spec/changes/<change-id>/research/
```

`meta.yaml` 必须至少包含：

```yaml
id: <change-id>
stage: exploration
deliveryShape: single-change
artifacts:
  proposal:
    status: missing
  metadata:
    status: present
```

standalone exploration stub 不包含 `multiChange`，不创建 `split.md`，不创建 `proposal.md`。后续必须使用 `unispec-propose` 复用该目录、读取已有 research、补齐 proposal 调研并生成 `proposal.md`。

8. **创建 split.md**

`split.md` 必须使用以下结构：

```markdown
# <parent-change-id> 拆分方案

## 问题概述

<整体问题、背景和为什么需要拆分>

## 整体目标

- <整体目标>

## 非目标

- <明确不做什么>

## 交付形态

multi-change

<判断理由>

## 子 change

### 1. <child-change-id>

- 目标：<独立目标>
- 验收边界：<可独立验证的结果>
- 依赖：<无 | child-change-id>
- 归档状态：[ ] pending
- 后续阶段：使用 `unispec-propose` 创建该 child 的 `proposal.md`

## 顺序与依赖

- <执行顺序和 dependsOn 关系>

## 风险与未知项

- <风险或未知项>

## 后续执行

- <下一步应该先创建哪个 child proposal>

## 参考资料

- <如有 research 报告，引用相对路径；没有则写“无”>
```

9. **创建 multi-change meta.yaml**

parent `meta.yaml` 必须至少包含：

```yaml
id: <parent-change-id>
stage: exploration
deliveryShape: multi-change
multiChange:
  role: parent
  children:
    - id: <parent-change-id>-<child-topic>
      order: 1
      dependsOn: []
artifacts:
  split:
    status: present
  metadata:
    status: present
```

每个 child stub 的 `meta.yaml` 必须至少包含：

```yaml
id: <parent-change-id>-<child-topic>
stage: exploration
deliveryShape: single-change
multiChange:
  role: child
  parent: <parent-change-id>
  order: 1
  dependsOn: []
artifacts:
  proposal:
    status: missing
  metadata:
    status: present
```

10. **校验 exploration 阶段产物**

- `.spec/changes/<parent-change-id>/split.md` 存在且非空。
- `.spec/changes/<parent-change-id>/meta.yaml` 存在且非空。
- 每个 `.spec/changes/<child-change-id>/meta.yaml` 存在且非空。
- `meta.yaml.stage` 为 `exploration`。
- `meta.yaml.deliveryShape` 为 `multi-change`。
- `multiChange.role` 为 `parent`。
- `multiChange.children` 非空。
- 每个 child id 使用 `<parent-change-id>-` 前缀。
- 每个 child stub 的 `stage` 为 `exploration`、`deliveryShape` 为 `single-change`、`multiChange.role` 为 `child`。
- 每个 child stub 的 `artifacts.proposal.status` 为 `missing`。
- `split.md` 中每个 child 小节包含 `- 归档状态：[ ] pending`。
- `split.md` 中的 child 列表与 `meta.yaml.multiChange.children` 一致。
- 如果存在 `research/` 报告，`split.md` 的 `## 参考资料` 已引用相对路径。

single-change standalone exploration stub 检查：

- `.spec/changes/<change-id>/meta.yaml` 存在且非空。
- `.spec/changes/<change-id>/research/<topic>.md` 存在且非空。
- `meta.yaml.stage` 为 `exploration`。
- `meta.yaml.deliveryShape` 为 `single-change`。
- `meta.yaml.multiChange` 不存在。
- `meta.yaml.artifacts.proposal.status` 为 `missing`。
- 不存在 `split.md` 和 `proposal.md`。
- 最终输出明确下一步必须使用 `unispec-propose`，且 propose 仍需判断 proposal 调研是否足够。

可以运行 `unispec status` 或 `unispec show <parent-change-id> --artifact split` 观察状态。

**输出**

创建或更新：

```text
.spec/changes/<parent-change-id>/split.md
.spec/changes/<parent-change-id>/meta.yaml
.spec/changes/<child-change-id>/meta.yaml
```

不要生成：

```text
.spec/changes/<parent-change-id>/proposal.md
.spec/changes/<parent-change-id>/design.md
.spec/changes/<parent-change-id>/system-tests.md
.spec/changes/<parent-change-id>/tasks.md
.spec/changes/<child-change-id>/proposal.md
```

最终汇报字段：

- parent-change-id
- parent change 目录
- `split.md` 路径
- `meta.yaml` 路径
- research 报告路径（如有）
- standalone exploration stub 路径（如有）
- child change 列表、顺序和依赖
- child stub 目录列表
- risks / unknowns 摘要
- 下一步：使用 `unispec-propose` 为 standalone change 或第一个 child 创建 proposal

---

**约束**

- 不写 `proposal.md`。
- 不写设计或实现细节。
- 只创建 child stub 目录和 `meta.yaml`，不创建 child `proposal.md`。
- single-change 无 research 时不创建 change 目录；有 research 时只创建 standalone exploration stub，不创建 `proposal.md`。
- 不把多个不可独立验收的问题塞进一个 child。
- 不引用当前 CLI 中不存在的 `unispec new` 或 `unispec change` 命令。
