## 1. Release Evidence Closure
- [x] 1.1 基于现有 `build-dist.mjs`、`publish-packages.mjs` 与 distribution 测试，补 staged package 的一致性与可追溯检查
- [x] 1.2 增加 `dist/spec-wiki/` 级别的 `npm publish --dry-run` 或等价验证，并记录结果
- [x] 1.3 将 staged `package.json`、README、CLI/help 与当前源码版本/公开动作的一致性纳入自动化验证

## 2. Gate Alignment
- [x] 2.1 收口 `packages/spec-wiki` 自动化测试、`storybook` primary gate、`chi + zustand` smoke 与 `.wiki/02-开发指南/00-代码注释规范.md` 检查为正式 release evidence
- [x] 2.2 明确 `dagger` 与 19 项目全量结果在本轮仅作为观察项或 baseline guard，不重新提升为强制 release blocker

## 3. Reporting
- [x] 3.1 输出可复用的 release evidence 记录，明确 staged package proof、gate 结果与 dry-run 结果
- [x] 3.2 在变更记录中写明本轮只做 evidence / packaging gap closure，不修改 public surface 或 runtime contract
