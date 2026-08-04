# align-direct-tasks-with-unispec-template

## 问题

当前 Lite 的 `tasks-template.md` 只有全局 Red/Green/Refactor 区块。`direct` 虽然不要求 Red 证据，但没有 UniSpec 风格的能力块、大 task、小 task、局部 checklist 和用例映射结构，难以把直接实现拆成可执行、可验证的任务。

## 目标

- 将 `direct` 模式 tasks 模板改为 UniSpec 任务模板中适用于 Lite 的结构化清单模式。
- 保留 `implementation-mode: direct`、稳定英文机器字段和 Lite 的安全/质量门禁。
- 让每个大 task 包含自己的 `### CheckList`，并提供完整的 ST 到 task 映射、执行顺序和暂缓事项。
- 中英文模板正文同步增强；当前 repo-local Skills 通过 `update` 同步。
- 记录 UniSpec 本机来源、目标落点和“结构改写”采用方式。

## 非目标

- 不引入 UniSpec runtime、`normal` 字段、多宿主或 readiness 门禁。
- 不改 CLI、metadata schema、历史 archive 或 `.spec` artifact 文件名。
- 不把 Lite 的 TDD 模式改成结构化 direct 模式；TDD 仍保留 Red → Green → Refactor 证据要求。

## 成功标准

- 双语 tasks 模板包含任务总览、实现模式、编号大 task、每个大 task 的 checklist、用例到任务映射、执行顺序和暂缓事项。
- 模板明确 `direct` 是结构化实现清单：小 task 以动词开头，指向具体模块/文件/接口/配置/数据流，并在大 task 完成前补齐单元测试或替代局部验证。
- 模板仍支持 `tdd`，不残留 `implementation-ready` 或 UniSpec 可执行命令。
- workflow contract 测试覆盖上述双语内容，package/root tests、lint、typecheck、build、pack、Wiki validate 和 diff check 通过。

## 影响范围

- `packages/spec-wiki-lite/assets/skills/{zh,en}/wiki-plan/references/tasks-template.md`
- 双语 `wiki-plan/SKILL.md`、repo-local `.agents/skills/wiki-plan/**`
- `scripts/tests/workflow-contract.test.ts`

## 参考资料

- source：`E:/project/!byAI/UniSpec/src/core/assets/skill-templates/references/tasks.ts`
- target：Lite 双语 `wiki-plan` tasks template 与 Codex repo-local Skill
- adoption：改写 UniSpec 的任务总览/大 task/checklist/映射结构，不复制其 readiness、`normal` 命名或实现命令。
