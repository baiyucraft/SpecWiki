# make-tdd-default-remove-readiness-gate

## 问题

当前 Lite 的任务模板虽然包含 `implementation-mode: tdd`，但没有形成明确的模式选择交互；同时 `implementation-ready: true` 被 `wiki-plan`、`wiki-continue` 和 `wiki-apply` 当作进入实现前的额外确认门。结果是 TDD 选择不清晰，而用户已经授权实现后仍需要一次重复的 readiness 修改。

## 目标

- 将 TDD 作为可选实现模式；`wiki-plan` 在生成 tasks 前必须询问用户选择 `tdd` 或 `direct`，除非当前请求已经明确指定。
- 删除 `implementation-ready` 作为额外人工确认步骤；tasks 完成后即可由 `wiki-continue` 路由到 `wiki-apply`。
- tasks 使用稳定 `implementation-mode: tdd|direct` 记录选择；TDD 模式要求 Red/Green/Refactor，direct 模式要求 Implement/Verify/Refactor。
- 保留 strict validate、用户授权范围、安全、失败回退、scope drift 和 review/archive 门禁。
- 同步中英文 8 个 Skill、tasks 模板、Wiki、README 和自动化测试。

## 非目标

- 不绕过 proposal/design/plan 阶段，不允许 parent 直接实现。
- 不删除用户授权检查、路径安全、失败回滚、测试、review 或 archive 门禁。
- 不改变 CLI、`.spec` artifact 文件名、stage 名、JSON 字段或历史 archive。

## 成功标准

- `wiki-plan` 在模式未明确时询问 `tdd` 或 `direct`，已明确时不重复询问。
- 新 tasks 模板记录选定的 `implementation-mode: tdd|direct`，不再要求或生成 `implementation-ready` 字段。
- `wiki-continue` 在 tasks 完整且 implementation mode 有效时直接路由 `wiki-apply`。
- `wiki-apply` 前置条件不再检查 `implementation-ready`，但仍要求用户已授权、tasks 完整和 strict validate。
- 中英文 package assets 与当前 `.agents/skills/wiki-*` 同步，且不存在相互矛盾的旧门禁说明。
- 自动化测试覆盖默认 TDD、tasks→apply 路由、无 readiness 字段仍可 apply、未授权仍暂停。
- package/root tests、lint、两层 typecheck、build、pack、Wiki validate、diff check 全通过。

## 影响范围

- `assets/skills/{zh,en}/wiki-plan|wiki-continue|wiki-apply/**`
- 当前 `.agents/skills/wiki-*`、tasks templates、README 和 Wiki workflow/design 文档
- workflow content contract 与 CLI/Skill routing tests

## 交付形态

single-change

这是一个统一的流程合同变更，必须同时更新路由、计划模板、apply 前置和中英文资产才能避免半旧半新的行为。

## 风险

- 删除 readiness 字段可能误放行未授权实现，必须保留明确的用户授权和 scope 检查。
- 模式未询问或未记录会造成 apply 行为不确定，必须在 plan 阶段失败关闭。
- 历史 archive 中的旧字段是不可变证据，不应批量改写。

## 参考资料

- 当前 Lite `wiki-plan`、`wiki-continue`、`wiki-apply` Skill 与 tasks template；目标落点为同一组双语 package-owned assets；采用方式：直接改写 Lite 工作流合同。
