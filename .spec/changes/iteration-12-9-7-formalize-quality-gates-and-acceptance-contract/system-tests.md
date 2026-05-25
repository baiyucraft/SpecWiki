# iteration-12-9-7-formalize-quality-gates-and-acceptance-contract 系统测试用例

## 用例总览

本文件是在治理体系迁移期间补齐的 UniSpec required artifact。该 change 已经处于 tasks 阶段，原有 proposal、design 和 tasks 从 legacy spec system 迁入；系统测试用例先按现有任务闭环建立最低可验证边界，后续继续推进该 change 时应按对应 design 细化 ST 覆盖。

## 系统测试用例

### ST-001 迁移后的 change artifact 结构可被 UniSpec 识别

- 关联成功标准: change 目录具备 proposal、design、system-tests、tasks 和 meta artifact。
- 覆盖设计点: UniSpec 使用 .spec/changes/<change-id>/ 作为 active change 控制面。
- 前置条件: 当前 change 已完成迁移并保留原有 proposal、design、tasks。
- 操作 / 触发: 运行 unispec status --json 或 unispec validate iteration-12-9-7-formalize-quality-gates-and-acceptance-contract。
- 期望结果: 当前 change 不因缺少 required artifact 被阻塞。
- 验证方式: UniSpec CLI 结构校验。

### ST-002 原有任务闭环继续可追踪

- 关联成功标准: 原有 tasks 仍可作为后续实现、review 和 verification 的输入。
- 覆盖设计点: 迁移不改变 change 的需求边界和任务语义。
- 前置条件: 读取当前 change 的 proposal、design 和 tasks。
- 操作 / 触发: 对照 tasks 中的任务项检查是否仍能追溯到 proposal / design。
- 期望结果: 迁移只改变治理路径与 artifact 命名，不丢失原任务内容。
- 验证方式: 人工审查加后续 change-specific 测试补充。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| UniSpec required artifact 完整 | ST-001 | unispec status --json / unispec validate iteration-12-9-7-formalize-quality-gates-and-acceptance-contract |
| 迁移后任务语义不丢失 | ST-002 | 人工审查 proposal、design、tasks 追溯关系 |

## 边界与异常

- 本文件只补齐治理迁移后的最低系统测试边界，不替代后续针对具体能力的详细测试设计。
- 如果后续继续实现该 change，必须先根据当前 design 细化系统测试用例。

## 验证数据与环境

- 当前仓库 .spec/changes/iteration-12-9-7-formalize-quality-gates-and-acceptance-contract/。
- UniSpec CLI。

## 未覆盖项

- 具体业务能力的完整验收用例尚未在本迁移中展开，后续推进各 change 时补齐。

## 参考资料

- proposal.md
- design.md
- tasks.md
