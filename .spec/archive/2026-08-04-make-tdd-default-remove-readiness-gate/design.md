# make-tdd-default-remove-readiness-gate 设计

## 方案概述

Lite workflow 的 `tasks` 阶段完成即表示计划可执行，不再使用额外的 `implementation-ready` 字段表达人工确认。TDD 是显式可选模式：`wiki-plan` 在生成 tasks 前询问用户选择 `tdd` 或 `direct`；如果用户已在当前请求明确选择，则直接采用而不重复询问。tasks 用 `implementation-mode` 记录选择。

## 路由合同

- `wiki-continue`：`tasks` 且 proposal/design/cases/tasks strict validate 通过、`implementation-mode` 为 `tdd|direct` 时直接路由 `wiki-apply`；不读取或判断 `implementation-ready`。模式缺失/非法时路由回 `wiki-plan`。
- `wiki-plan`：模式未明确时必须询问 `tdd` 或 `direct`；用户已明确时不重复询问。规划完成时 stage 为 tasks，并写入选定 mode；不要求用户再改 readiness 字段。
- `wiki-apply`：前置要求 tasks 完整、mode 有效、用户已经在当前请求授权实现、strict validate 无 planning blocker；不要求 readiness 字段。
- parent/child、依赖、scope drift、path safety、外部状态和 review/archive 门禁保持不变。

## 模板与状态

- zh/en `references/tasks-template.md`：提供 `implementation-mode: <tdd|direct>` 占位，删除 `implementation-ready` 和“设置为 true”的说明。
- package 与 repo-local Skill 正文移除 `implementation-ready` 的前置、路由和输出要求。
- `.spec` metadata 不新增替代字段；stage 仍使用 `tasks -> implementation -> review -> verification -> archive`。
- 旧 archive 中的 `implementation-ready` 只作为历史证据保留，扫描和文档 current surface 排除 archive。

## 授权安全边界

删除 readiness gate 不等于自动扩大授权。`wiki-apply` 仍须：

1. 当前用户请求明确授权实现；仅“查看/解释/规划”不触发 apply。
2. change 为 standalone/child，不是 parent；tasks、proposal、design、cases 完整。
3. strict validate 通过；新增风险或 scope drift 时回到前序 Skill。
4. 不执行未声明的发布、tag、merge、破坏性迁移或外部写入。

## 实现模式

- `tdd`：plan 生成 UT/ST 映射与 Red/Green/Refactor tasks；apply 必须先取得相关 Red 失败证据。
- `direct`：plan 仍生成 system tests 与验证映射，但 tasks 使用 Implement/Verify/Refactor；apply 不要求 Red 证据，仍必须完成全部测试与质量门禁。
- 模式选择只询问一次；已有明确用户选择或已有合法 tasks mode 时不得重复询问。

## 验证设计

- 单元/内容测试检查双语模板支持 `tdd|direct`、无 readiness token、Skill 引用闭包和稳定字段。
- routing test 验证合法 mode 的 tasks 直接选择 `wiki-apply`，缺失/非法 mode 返回 `wiki-plan`。
- apply contract test 验证无 readiness 字段的 tasks 能进入 apply，未授权仍返回 pause/不修改，并按 mode 选择执行步骤。
- 保留现有完整 package/root、tarball、Wiki、lint、typecheck、build 和 diff gates。

## 参考边界

- 来源：当前 Lite 的双语 `wiki-*` Skill、tasks 模板和 change metadata 合同。
- 目标落点：`assets/skills/{zh,en}`、`.agents/skills/wiki-*`、README/Wiki workflow 文档和测试。
- 采用方式：直接改写 Lite 流程合同；不改写历史 archive，不引入新的 runtime 或 CLI 字段。

## 回滚

- 所有资产通过 package-owned 同步器原子覆盖；失败恢复原内容。
- 如新合同造成 scope/授权误判，回退 Skill 路由和模板即可，CLI metadata schema 不变。
