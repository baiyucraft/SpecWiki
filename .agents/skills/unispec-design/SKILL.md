---
name: unispec-design
description: 根据已确认的 proposal.md 生成 design.md。当用户确认 proposal 后需要设计实现方案时使用。
metadata:
  author: unispec
  generatedBy: "0.1.0"
---

根据已确认的 `proposal.md` 形成设计方案，生成可供用户审核的 `design.md`；用户确认后再进入 `unispec-plan`，为 `cases` 阶段和 `tasks` 阶段提供输入。

覆盖阶段：

```text
design
```

**重要：本 Skill 只负责设计方案和 design.md，不生成 system-tests.md / tasks.md，不写实现代码。**

---

**输入**

```text
.spec/changes/<change-id>/proposal.md
.spec/changes/<change-id>/meta.yaml
.wiki/INDEX.md
.wiki/ 中相关项目规范
相关代码、测试、配置和文档
```

当 `proposal.md` 已完成并经过用户确认，需要进入 `design` 阶段时使用。

如果 proposal 缺失必需信息，或目标、非目标、成功标准不清，停止并回到 `unispec-propose` 修正。若只是旧版标题格式但内容完整，先向用户说明并确认是否按当前中文标题规范继续。

**步骤**

1. **检查输入是否满足设计阶段**

确认以下文件存在且非空：

```text
.spec/changes/<change-id>/proposal.md
.spec/changes/<change-id>/meta.yaml
```

读取 `meta.yaml`，确认：

- `id` 与 change 目录一致。
- 可执行 change 必须为 `deliveryShape: single-change`。
- 如果 `multiChange.role` 为 `parent`，停止；parent 只承载 `split.md`，应先使用 `unispec-propose` 创建具体 child change。
- 如果 `multiChange.role` 存在，只能是 `child`。
- 如果当前 stage 仍为 `exploration`，说明 child stub 尚未生成 `proposal.md`，先回到 `unispec-propose`。
- 当前 change 尚未进入 `cases`、`tasks` 或后续实现阶段；如果已经进入后续阶段，先询问用户是修订设计还是继续后续流程。

`proposal.md` 应包含当前规范的中文标题：

```text
问题
目标
非目标
成功标准
影响范围
交付形态
风险
未知项
```

2. **读取 proposal 和阶段上下文**

从 `proposal.md` 中提取：

- 要解决的问题和背景。
- 目标、非目标和成功标准。
- 影响范围和交付形态。
- 风险、未知项，以及 `deferred-to-design` 中留到设计阶段判断的问题。
- `参考资料` 中引用的已有 research 报告。

按需读取：

```text
.wiki/INDEX.md
.wiki/ 中与当前 change 直接相关的项目规范；只引用稳定结论，不复制 wiki 正文
相关代码、测试、配置、README
.spec/changes/<change-id>/research/ 下已有调研报告
```

如果 proposal、research、wiki、用户输入或现有代码显示本 change 涉及前端界面、表单、列表、查询分页、弹窗、上传、数据展示或浏览器交互，必须读取 `references/frontend-interaction-standard.md`。读取后只把与当前 change 相关的交互约束、校验规则、反馈、布局适配和验证方向写入 `design.md`，不要复制整份规范。

设计阶段只回答“如何实现已确认的 proposal”。不得改变 proposal 的目标、非目标和成功标准；如果发现 proposal 需要修改，停止并回到 `unispec-propose`。

如果 `research/` 中已有需求、现状或 scope 结论，先复用已有报告；只在缺少实现层面的证据时补充技术调研。

3. **进行设计调研**

优先使用当前 tool 或运行环境提供的只读子代理并行调研。主 agent 默认拆出至少 2 个互不依赖的只读调研问题；如果当前环境没有可用子代理机制，必须说明原因并改为本地串行只读调研。

常见调研方向：

- 现有模块、入口、调用链和边界。
- 接口、数据结构、CLI / API / UI 触点。
- 测试现状、验证方式和已有测试工具。
- 兼容性、迁移、错误处理和回滚风险。
- 项目内既有模式、命名约定和流程约束。

子代理只负责 read-only 调研，输出发现、证据路径、风险、未知项，以及对设计边界的影响。主 agent 负责汇总、复核和取舍，只保留会影响 `design.md` 的结论。

如果调研结论会影响当前 design，或需要汇总多条证据 / 子代理结论，可以写入辅助调研目录：

```text
.spec/changes/<change-id>/research/<topic>.md
```

`research/` 是辅助调研材料。创建前读取 `references/research-template.md`；创建后必须在 `design.md` 的 `## 参考资料` 中引用相对路径，并在最终输出中列出报告路径。

4. **创建或更新 design.md 草稿**

根据 proposal、阶段上下文和设计调研结论，直接创建或更新用户可审核的设计文档：

```text
.spec/changes/<change-id>/design.md
```

`design.md` 是用户审核对象。创建前先读取 `references/design-template.md`，按其中的固定结构生成设计文档。涉及前端界面时，同时读取并遵守 `references/frontend-interaction-standard.md`。不要只停留在口头方案；不适用的三级提示不创建空标题、空表格或占位文本。

5. **更新 meta.yaml**

只有在 `design.md` 已创建或更新后，才能更新 `meta.yaml`。

更新要求：

- 保留已有 `id`、`deliveryShape`、`createdAt`、已有 artifact 状态和未知字段。
- 将 `stage` 更新为 `design`。
- 将 `artifacts.design.status` 标记为 `present`。
- 保留 `artifacts.proposal` 和 `artifacts.metadata` 的已有状态。
- 如果已有 `updatedAt` 惯例，按项目惯例更新；否则不要新增格式不明的字段。

当前没有可用 CLI 命令时，直接编辑 `meta.yaml`。

这表示 change 已进入 `design` 阶段，不表示 `design.md` 已被用户最终确认，也不表示可以自动进入 `cases` / `tasks`。

6. **校验 design 阶段产物**

完成后检查：

- `.spec/changes/<change-id>/design.md` 存在且非空。
- `design.md` 包含所有必需中文标题。
- 设计方案能追溯到 proposal 的目标和成功标准。
- `meta.yaml.stage` 为 `design`。
- `meta.yaml.deliveryShape` 未丢失或改变。
- 未生成 `system-tests.md` 或 `tasks.md`。

校验 design 阶段产物只检查文件、标题、proposal 可追溯性和 metadata；用户最终确认属于人工 gate，不作为当前文件存在性的校验条件。

可以运行 `unispec validate <change-id>` 校验 design 阶段必需产物；当前阶段只应要求 `proposal.md`、`design.md` 和 `meta.yaml`。validate 不能替代用户对 `design.md` 的人工审核。

7. **提醒用户审核 design.md 并迭代修改**

完成后请用户阅读 `design.md`。如果用户提出修改意见，只更新 `design.md` 和必要的 `meta.yaml` 时间戳 / 状态，不生成 `system-tests.md` 或 `tasks.md`，不写实现代码。

如果用户反馈会改变 proposal 的目标、非目标或成功标准，停止并建议回到 `unispec-propose` 更新 proposal。

只有用户明确确认 `design.md` 已完成后，才建议进入 `unispec-plan`。

**输出**

创建或更新：

```text
.spec/changes/<change-id>/design.md
.spec/changes/<change-id>/meta.yaml
```

不要生成：

```text
.spec/changes/<change-id>/system-tests.md
.spec/changes/<change-id>/tasks.md
```

最终汇报字段：
- change-id
- change 目录
- `design.md` 路径
- `meta.yaml` 路径
- research 报告路径（如有）
- risks / unknowns 摘要
- 请用户审核 `design.md`
- 下一步：用户确认 `design.md` 完成后，使用 `unispec-plan`

---

**产物生成指南**

- `design.md` 的具体结构和写作边界以 `references/design-template.md` 为准。
- 涉及前端界面时，前端交互、表单、查询分页、弹窗、上传、数据展示和分辨率适配约束以 `references/frontend-interaction-standard.md` 为准。
- `research/` 的具体结构和写作边界以 `references/research-template.md` 为准。
- `design.md` 必须能追溯到 proposal 的目标、成功标准、影响范围、风险或未知项。
- `验证思路` 只描述后续 `system-tests.md` 应覆盖的方向，不生成具体系统测试用例。

---

**约束**

- 不跳过 `proposal.md`。
- 不改变 proposal 的目标、非目标和成功标准。
- 不生成 `system-tests.md` 或 `tasks.md`。
- 不写实现代码。
- 不为了未来假设做过度设计。
- 用户未明确确认 `design.md` 完成前，不进入 `cases` 或 `tasks`。
