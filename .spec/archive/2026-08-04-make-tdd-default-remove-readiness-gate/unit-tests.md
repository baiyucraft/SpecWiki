# make-tdd-default-remove-readiness-gate TDD 单元测试

## UT-01 计划模式选择合同

- Test：`scripts/tests/workflow-contract.test.ts`
- Modify：`packages/spec-wiki-lite/assets/skills/{zh,en}/wiki-plan/SKILL.md`
- 映射：ST-01 / ST-02 / 成功标准 1
- Red：旧正文没有“询问 tdd/direct、已有明确选择时不重复询问”的合同，并仍包含 readiness 门禁
- Green：plan Skill 明确一次性询问可选模式并把合法值写入 tasks
- Refactor：统一中英文术语、稳定字段和暂停条件

## UT-02 tasks 模板合同

- Test：`scripts/tests/workflow-contract.test.ts`
- Modify：`packages/spec-wiki-lite/assets/skills/{zh,en}/wiki-plan/references/tasks-template.md`
- 映射：ST-01 / ST-02 / ST-04 / 安全边界
- Red：模板仍声明 `implementation-ready`，且没有 direct 模式说明
- Green：模板只保留 `implementation-mode: tdd|direct` 并为两种模式提供证据要求
- Refactor：保持 YAML/frontmatter 机器字段稳定、正文双语化

## UT-03 continue 路由合同

- Test：`scripts/tests/workflow-contract.test.ts`
- Modify：`packages/spec-wiki-lite/assets/skills/{zh,en}/wiki-continue/SKILL.md`
- 映射：ST-03 / ST-04 / 成功标准 3
- Red：旧路由按 readiness true/false 决定 plan/apply
- Green：合法 mode 直接 apply，缺失/非法 mode 回 plan
- Refactor：保留 parent/child、依赖和 strict validate 门禁

## UT-04 apply 前置与模式行为

- Test：`scripts/tests/workflow-contract.test.ts`
- Modify：`packages/spec-wiki-lite/assets/skills/{zh,en}/wiki-apply/SKILL.md`
- 映射：ST-05 / ST-06 / 成功标准 4
- Red：旧 apply 要求 readiness true
- Green：apply 要求当前请求授权、完整 artifacts、合法 mode 和 strict validate；tdd 要 Red，direct 不要 Red
- Refactor：统一失败回退、scope drift、path safety 和质量门禁文字

## UT-05 repo-local 同步与 current-surface 扫描

- Test：`scripts/tests/workflow-contract.test.ts`
- Modify：`.agents/skills/wiki-*`、README、Wiki workflow 文档
- 映射：ST-07 / 成功标准 5
- Red：当前 repo-local Skills 与 package assets 仍含旧 readiness 合同
- Green：build/update 后两者一致，current surface 无旧 token
- Refactor：历史 `.spec/archive/**` 明确排除在扫描范围外

## 覆盖边界

- 不测试仅属于实现细节且不影响 observable workflow contract 的内容。
- 路由和授权以稳定文本/metadata 合同测试为主，CLI 与 strict validate 作为聚合证据。
- 历史 archive 只读，不因本 change 批量改写。
