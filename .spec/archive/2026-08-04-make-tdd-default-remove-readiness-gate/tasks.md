# make-tdd-default-remove-readiness-gate 实现任务

implementation-mode: tdd

## Red

- [x] R1 更新 workflow contract 测试，覆盖 tdd/direct 选择、一次性询问、合法 mode 路由、readiness 移除、授权保留（UT-01~UT-05；ST-01~ST-07）。证据：`pnpm exec vitest run scripts/tests/workflow-contract.test.ts` 首次运行 5 项中 4 项失败。
- [x] R2 运行 focused tests，记录因旧 Skills/templates 仍含 readiness 且缺少 direct 规则而产生的 Red 失败证据。证据：首次 focused run 明确报告模板 `implementation-ready`、固定 `implementation-mode: tdd` 与 repo-local 旧 Skill 不符合目标合同。

## Green

- [x] G1 改写中英文 `wiki-plan` 与 tasks templates：在 mode 未明确时询问 `tdd|direct`，记录 `implementation-mode`，删除 readiness 字段和确认步骤（UT-01/02）。证据：package assets 与模板扫描通过。
- [x] G2 改写中英文 `wiki-continue`：tasks 完整且 mode 合法直接进入 `wiki-apply`；缺失/非法 mode fail-closed 回 `wiki-plan`（UT-03）。证据：双语路由表和 mode 合同测试通过。
- [x] G3 改写中英文 `wiki-apply`：删除 readiness 前置；保留用户授权、strict validate、scope/path/data safety；tdd 要求 Red，direct 不要求 Red（UT-04）。证据：apply 合同测试通过。
- [x] G4 同步当前 `.agents/skills/wiki-*`、README 和 Wiki workflow/design 文档，current surface 排除历史 archive（UT-05）。证据：`spec-wiki-lite update --json` 更新 4 个登记文件，文档扫描通过。
- [x] G5 运行 focused tests，记录 Green 证据。证据：`pnpm exec vitest run scripts/tests/workflow-contract.test.ts` 5/5 passed。

## Refactor 与验证

- [x] F1 统一中英文术语、错误/暂停条件、mode schema 和证据格式，移除重复矛盾说明。
- [x] F2 运行 package/root tests、lint、两层 typecheck、build、pack evidence、Wiki strict validate、`git diff --check`。证据：package 71 passed/1 skipped、root 13 passed、lint/typecheck/build/pack/status/strict validate/diff check 均通过。
- [x] F3 生成 full/pass review 与 verification，使用 Lite CLI 归档本 change。证据：`review-report.md` 与 `test-report.md` 均为 full/pass。

## 成功标准映射

| 成功标准 | ST/UT | Task | 命令/证据 |
| --- | --- | --- | --- |
| plan 询问并记录 tdd/direct | ST-01/02, UT-01/02 | G1 | `pnpm test`, `.spec/changes/.../tasks.md` |
| 合法 mode 直接 apply，缺失/非法回 plan | ST-03/04, UT-03 | G2 | workflow contract test、Skill 文本 |
| readiness 删除但授权和安全门禁保留 | ST-05/06, UT-04 | G3 | workflow contract test、strict validate |
| 双语 package/repo-local/current docs 一致 | ST-07, UT-05 | G4/F1 | build + update + scan + Wiki validate |
| 全量质量门禁与归档通过 | ST-07 | F2/F3 | test/lint/typecheck/build/pack/archive |
