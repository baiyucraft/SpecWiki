---
implementation-ready: true
---

# close-specwiki-3-0-design-baseline-host-trigger-contract 任务计划

## 任务总览

任务按四个可独立验收的能力块拆分：共享 decision kernel、corpus 与 host capability matrix、Codex-first skill projection 与 CodeBuddy parser、可执行 hook 与全量 drift gate。用户已明确授权自动 TDD 实现、review、verification 与归档，因此规划产物完成后直接进入 implementation。

## 实现模式

tdd

先写失败单元测试并确认失败，再写最小实现，通过后重构。每个大 task 的 focused tests、质量检查和注释检查完成后才勾选 CheckList。

## 1. 共享 trigger taxonomy 与确定性 evaluator

- [x] 1.1 Red: UT-001 编写 TriggerDecision invariant 与 action policy 的失败测试，并确认失败原因符合预期
- [x] 1.2 Green: UT-001 创建 `triggerContract.ts` 的闭集类型、policy 和 validator，使测试通过
- [x] 1.3 Refactor: UT-001 复用 `PublicWikiAction`、补齐导出契约注释并保持测试通过
- [x] 1.4 Red: UT-002 编写 normalization、precedence、中英文语义与近碰撞失败测试，并确认失败原因符合预期
- [x] 1.5 Green: UT-002 实现纯 `evaluateHostTrigger` 与稳定 evidence，使测试通过
- [x] 1.6 Refactor: UT-002 数据驱动整理 rules/reducer，确认 evaluator 无 I/O、无 host id、无 raw prompt 持久化

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 2. 版本化 corpus 与 Host compatibility/capability matrix

- [x] 2.1 Red: UT-003 编写 corpus schema/version 与三宿主 semantic conformance 失败测试，并确认失败原因符合预期
- [x] 2.2 Green: UT-003 创建 `triggerCorpus.ts` 与 `host-trigger-corpus.v1.json`，使测试通过
- [x] 2.3 Refactor: UT-003 分离 semantic/adapter cases、contractVersion/revision 并保持测试通过
- [x] 2.4 Red: UT-004 编写 Codex 唯一 reference、三宿主 capabilities 与 asset validator 失败测试，并确认失败原因符合预期
- [x] 2.5 Green: UT-004 扩展 `HostDefinition` 并在 asset build 返回前执行 validator，使测试通过
- [x] 2.6 Refactor: UT-004 保留最小 host switch、集中 capability invariant 和导出注释并保持测试通过

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 3. Codex-first guidance 与 CodeBuddy 结构化 adapter

- [x] 3.1 Red: UT-005 编写 Codex reference、compatible guidance parity、canonical query fields 和 explicit-only 失败测试
- [x] 3.2 Green: UT-005 让共享 action renderer 消费 trigger policy，先满足 Codex reference 后生成 compatible skills
- [x] 3.3 Refactor: UT-005 去除重复 trigger 文案 authority，保持三宿主 renderer tests 通过
- [x] 3.4 Red: UT-006 编写 CodeBuddy `user_prompt` 单字段 parser、delivery 与 malformed fail-closed 失败测试
- [x] 3.5 Green: UT-006 创建 `codebuddy/triggerAdapter.ts`，实现结构化 parser 与 context-only delivery
- [x] 3.6 Refactor: UT-006 移除多字段猜测、session identity 和 raw JSON substring 路径并保持测试通过

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 4. CodeBuddy executable hook、bootstrap 与 drift gate

- [x] 4.1 Red: UT-007 编写生成 `.mjs` 的 adapter corpus 子进程失败测试，并确认 raw substring 误触发
- [x] 4.2 Green: UT-007 从共享 rules/policy 生成自包含 hook，使 executable corpus 与库 delivery 对拍通过
- [x] 4.3 Refactor: UT-007 分离 SessionStart orientation 与 UserPromptSubmit decision，确保唯一 JSON 输出且不调用 CLI
- [x] 4.4 Red: UT-008 编写 bootstrap/capability/query-session cross-layer drift 失败测试
- [x] 4.5 Green: UT-008 接入 workspace contract、更新 bootstrap assertions 与受管覆盖，使测试通过
- [x] 4.6 Refactor: UT-008 运行 package/workspace lint/build/distribution 回归，清理测试 helper 并同步任务证据

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 | 2. compatibility/capability matrix | 2.4-2.6 / UT-004 |
| ST-002 | 1. trigger evaluator | 1.1-1.6 / UT-001、UT-002 |
| ST-003 | 2. corpus | 2.1-2.3 / UT-003 |
| ST-004 | 3. Codex-first guidance | 3.1-3.3 / UT-005 |
| ST-005 | 3、4. CodeBuddy adapter/hook | 3.4-3.6、4.1-4.3 / UT-006、UT-007 |
| ST-006 | 1、3、4. no execution | 1.4-1.6、3.4-3.6、4.1-4.3 / UT-002、UT-006、UT-007 |
| ST-007 | 4. drift gate | 4.4-4.6 / UT-008 |
| ST-008 | 2、4. validator/full regression | 2.4-2.6、4.4-4.6 / UT-004、UT-008 |

## 执行顺序

- 先完成 1，得到共享对象语言与 evaluator。
- 再完成 2，固定 corpus 和 machine-readable host matrix。
- 然后完成 3，先 Codex reference、再 Claude/CodeBuddy compatible projection。
- 最后完成 4，验证真实生成 hook、bootstrap、workspace drift 与全量回归。
- 全部 task 完成后进入 `unispec-review`；full/pass 证据齐备后进入 `unispec-archive`。

## 暂缓事项

- 完整 HostAdapter detect/context/assets 重构。
- production research bridge、多轮 agent-session bridge 与 durable provider session。
- 外部宿主模型开放域 skill-selection 准确率测试。
