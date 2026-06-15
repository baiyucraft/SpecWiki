## Why

即使 `v0.2.0` 的公开合同与文档口径已经收口，当前仓库仍缺少一套明确的 staged package、publish dry-run 与 release gate 证据，来证明发布产物确实来自当前源码真相，而不是旧 release 文案或旧 staging 残留。没有这层证据，`v0.2.0` 仍然只是“设计上可发布”，而不是“可证明地可发布”。

这轮 change 只补发布证据和 staged package 收口，不再修改 CLI 语义或扩 public surface。

## What Changes

- 为 `v0.2.0` 收口 staged package、distribution proof 与 publish dry-run 证据
- 对齐 staged `package.json`、README、CLI/help 与实际构建产物之间的一致性检查
- 将 `storybook` primary gate、`chi + zustand` smoke、`packages/spec-wiki` 测试与 `.wiki/02-开发指南/00-代码注释规范.md` 检查收口为正式 release gate 证据
- 明确当前 `dist/spec-wiki/` 必须从当前源码真相重新装配，而不是继续沿用旧 release 口径
- 本轮只做 evidence / packaging gap closure，不新增新的 capability 或 runtime contract

## Capabilities

### New Capabilities
- None

### Modified Capabilities
- `workflow-verification`: 增补 `v0.2.0` 发布前必须具备的 release gate 与 package evidence
- `adapter-distribution`: 增补 staged package、publish dry-run 与产物一致性证明要求

## Impact

- `scripts/build-dist.mjs`
- `scripts/publish-packages.mjs`
- `scripts/tests/distribution.test.ts`
- `dist/spec-wiki/**`
- `.wiki/05-规格基线/capabilities/workflow-verification/spec.md`
- `.wiki/05-规格基线/capabilities/adapter-distribution/spec.md`
- 发布证据文档与报告
