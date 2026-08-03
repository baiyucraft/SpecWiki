# add-localized-skill-templates TDD 单元测试

## UT-01 语言化 registry inventory

- Test：`packages/spec-wiki-lite/src/core/assets/registry.test.ts`
- Modify：`src/core/assets/registry.ts`
- 覆盖 zh/en 各 24 个文件、8 个稳定 Skill 分组、reference 目标路径和 ownership。
- Red：当前 registry 只登记 8 个扁平 `SKILL.md`，没有 locale 或 references。

## UT-02 Skill 同步与保留

- Test：`src/core/assets/sync.test.ts`
- Modify：`src/core/assets/sync.ts`
- 覆盖 init/update、zh↔en、修改/缺失 Skill 修复、用户新增文件保留、无效配置零写入、rollback。
- Red：当前同步器不会切换 Skill locale，也不会校验完整 inventory。

## UT-03 Skill readiness

- Test：`src/core/status.test.ts`、`src/lite-red.test.ts`
- Modify：`src/core/status.ts`
- 覆盖每个 Skill 的全部登记文件内容一致性、旧版本/缺失/语言错误和恢复 ready。
- Red：当前 status 只检查 `SKILL.md` 是否存在。

## UT-04 内容合同与引用闭包

- Test：`scripts/tests/skill-assets.test.ts`
- Modify：双语 assets、文档扫描脚本。
- 覆盖 frontmatter、稳定 token、references closure、禁用词和工具中立 browser automation。
- Red：当前 assets 没有 references inventory 和双语内容合同。

## UT-05 分发 smoke

- Test：`scripts/tests/tarball-smoke.test.ts`
- Modify：pack evidence / smoke harness（如需）。
- 覆盖 staged tarball 的 zh/en init、双向 update、status 和完整 assets inventory。
- Red：当前 tarball 不含双语 Skill references。
