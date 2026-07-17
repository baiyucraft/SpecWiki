# close-specwiki-3-0-design-baseline-host-trigger-contract 系统测试用例

## 用例总览

本组用例覆盖 Codex-first compatibility role、共享三态 trigger taxonomy、显式/语义 action policy、跨宿主 corpus、真实资产 capability matrix、CodeBuddy 结构化 hook、Runtime query 薄消费和 trigger/bridge/provider-session 正交边界。验收对象是项目拥有的 TypeScript 实现、生成资产和可执行 hook；不承诺外部宿主模型的黑盒 skill-selection 准确率。

## 系统测试用例

### ST-001 Codex 是唯一 reference host

- 关联成功标准: 单一 compatibility authority；新增宿主必须声明 capabilities。
- 覆盖设计点: `HostCompatibilityRole`、Codex-first registry、capability/asset validator。
- 前置条件: 构建当前三宿主 registry 与 bootstrap assets。
- 操作 / 触发: 枚举全部宿主并校验 role、capabilities 和实际资产。
- 期望结果: 恰好一个 reference 且为 Codex；Claude、CodeBuddy 为 compatible；声明与 skills/hooks/settings 实际集合一致。
- 验证方式: package Vitest registry/asset tests 与 bootstrap integration tests。

### ST-002 共享 trigger evaluator 产生确定性三态决策

- 关联成功标准: 单一 taxonomy、decision/action/reason/evidence authority；negative/ambiguous 不执行。
- 覆盖设计点: `TriggerDecision`、action policy、normalization、reducer precedence 和 invariants。
- 前置条件: 加载 shared trigger contract v1。
- 操作 / 触发: 输入显式 action、语义 query/status、opt-out、冲突、缺 term、关键词近碰撞和 invalid input。
- 期望结果: 输出稳定 `should_trigger / should_not_trigger / ambiguous`；target/reason/evidence 满足闭集与 invariant；mutation action 仅显式请求可触发。
- 验证方式: table-driven Vitest 和 invariant tests。

### ST-003 中英文 corpus 对三宿主保持语义一致

- 关联成功标准: 版本化 corpus 覆盖中英文三态、近碰撞、上下文不足和 malformed input。
- 覆盖设计点: corpus schema/version/revision、semanticCases/adapterCases 分区。
- 前置条件: `host-trigger-corpus.v1.json` 可被严格解析。
- 操作 / 触发: 对每个 semantic case 运行 shared evaluator，并投影到 Codex、Claude、CodeBuddy。
- 期望结果: 三宿主 decision、targetAction、reason 一致；普通 case 不依赖宿主机制；malformed adapter case fail closed。
- 验证方式: corpus conformance Vitest。

### ST-004 三宿主 skills 服从 Codex reference 与 Runtime query authority

- 关联成功标准: 三宿主从同一 taxonomy/template 派生；query 只消费 canonical fields。
- 覆盖设计点: Codex reference skill projection、Claude/CodeBuddy compatible renderer、query/status guidance、mutation explicit-only guardrails。
- 前置条件: 为六个公开 action 构建三宿主 assets。
- 操作 / 触发: 检查路径、frontmatter、When To Use、guardrails 与 query field markers。
- 期望结果: Codex 为 reference；兼容 skills 共享 trigger policy；包含 `readiness/query_mode/query_trust/recommended_action/governance/route_groups/answer`，不含旧字段或 richer 私参。
- 验证方式: renderer/asset snapshot assertions 与 bootstrap tests。

### ST-005 CodeBuddy UserPromptSubmit 使用真实结构化包络

- 关联成功标准: CodeBuddy hook 不再拥有 raw substring authority；malformed input fail closed；SessionStart 分离。
- 覆盖设计点: `user_prompt` 单字段 parser、decision delivery、SessionStart orientation、生成 `.mjs` 对拍。
- 前置条件: 使用 CodeBuddy 官方资料固定的 v1 event fixture。
- 操作 / 触发: 以 Node 子进程执行生成 hook，输入合法 should/should-not/ambiguous events、缺字段和 malformed JSON。
- 期望结果: 只解析 `user_prompt`；合法请求按 shared decision 注入 action/clarification context；negative/invalid 无 action context；SessionStart 只注入 orientation；进程输出唯一 JSON 且不阻断。
- 验证方式: executable hook Vitest + corpus adapter cases。

### ST-006 任何 trigger decision 都不直接执行 CLI

- 关联成功标准: `should_not_trigger`、`ambiguous`、invalid 与 mutation implicit 请求不调用 CLI。
- 覆盖设计点: evaluator 无 I/O、delivery 闭集、hook context-only 输出。
- 前置条件: 加载 shared evaluator 和生成 hook。
- 操作 / 触发: 对全部 corpus case 运行库 evaluator与生成脚本，并扫描 hook asset。
- 期望结果: evaluator/hook 不启动子进程、不调用 `spec-wiki`；输出只包含 decision 或 `{continue, additionalContext?}`。
- 验证方式: unit tests、generated script assertions 和静态 contract test。

### ST-007 Trigger、bridge 与 provider session 保持正交

- 关联成功标准: 三层不复用 identity/persistence；provider durable state forbidden。
- 覆盖设计点: trigger DTO/corpus/context 禁止 session/bridge fields；Runtime query authority 不被 trigger 覆盖。
- 前置条件: 构建源码、fixtures、generated assets 和稳定 Wiki capability。
- 操作 / 触发: 对对象语言、corpus、hook output、assets 与相关文档运行 drift guard。
- 期望结果: trigger 面不含 `session_id/session_summary/recent_turns/tool_artifact_refs`；不把基础 bridge transport误报为 production research bridge。
- 验证方式: workspace contract test + reliability/query contract regression tests。

### ST-008 Bootstrap、发布与全量回归无漂移

- 关联成功标准: capability/taxonomy/assets/Runtime fields drift gate；新增宿主必须通过同一 corpus。
- 覆盖设计点: production asset validator、受管覆盖、Codex-first distribution snapshot。
- 前置条件: package/build 工具链可用。
- 操作 / 触发: 运行 package tests、workspace contract tests、lint、build、distribution、UniSpec validate。
- 期望结果: 三宿主 bootstrap 幂等；旧 CodeBuddy raw hook被覆盖；全部门禁通过，无历史 archive 修改。
- 验证方式: 自动化命令与最终 test/review report。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| 单一 trigger taxonomy / decision authority | ST-002、ST-003 | package table/corpus tests |
| 机器可消费 compatibility/capability matrix | ST-001、ST-008 | registry/asset/bootstrap tests |
| 三宿主同 corpus 语义一致 | ST-003、ST-004 | conformance tests |
| 中英文三态、近碰撞、malformed 覆盖 | ST-002、ST-003、ST-005 | fixture-driven tests |
| negative/ambiguous/mutation implicit 不执行 | ST-002、ST-005、ST-006 | reducer/hook tests |
| query canonical fields 与 thin consumption | ST-004、ST-007 | renderer/workspace contract tests |
| Runtime degraded/error 不被宿主改写 | ST-004、ST-007 | query/reliability regression |
| CodeBuddy parser/hook fail closed | ST-005、ST-008 | executable hook/bootstrap tests |
| trigger/bridge/provider session 正交 | ST-007 | static drift guard |
| 新宿主/capability/assets drift gate | ST-001、ST-003、ST-008 | registry/corpus/full regression |

## 边界与异常

- 非 string、空白、未知 event、malformed JSON 一律 fail closed。
- opt-out + positive、多显式 action、query/status 同时命中均为 ambiguous，不自动执行。
- 英文按词/短语边界，中文按请求语义短语；裸 `wiki/module/status` 不直接触发。
- CodeBuddy 只接受官方 v1 `user_prompt`，不猜测 `prompt/userPrompt` fallback。
- Codex/Claude 无项目可控 prompt hook，不伪造 mechanism parity。

## 验证数据与环境

- Node.js、pnpm、Vitest 与当前 Windows x64 package 工具链。
- `packages/spec-wiki/tests/fixtures/host-trigger-corpus.v1.json`。
- CodeBuddy 官方 `UserPromptSubmit.user_prompt` v1 fixture；测试不读取真实用户 transcript。
- 临时目录中的生成 skills/hooks/settings 与 Node 子进程。

## 未覆盖项

- 不验证 Codex、Claude 或 CodeBuddy 外部模型实际自动选 skill 的开放域准确率；只验证项目拥有的 contract projection 和可观测 hook。
- 不启用 production research bridge 或多轮 agent-session bridge。

## 参考资料

- [设计方案](./design.md)
- [宿主触发合同技术设计调研](./research/host-trigger-technical-design.md)
- `../../../.wiki/05-规格基线/capabilities/host-trigger-contract/spec.md`
- `../../../.wiki/06-设计文档/06-Runtime查询合同.md`

