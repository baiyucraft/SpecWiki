## 1. Drift Matrix Closure
- [x] 1.1 基于 `.docs/v0-2-0-release-gap-closure.md` 的漂移矩阵，逐项核对 README、README-CN、release note、staged README、package version 与主 spec 的当前口径
- [x] 1.2 更新 `README.md`、`README-CN.md` 与 release 文档，使其统一描述 `v0.2.0 minimal formal knowledge runtime` 和 6 个公开 workflow
- [x] 1.3 对齐 `packages/spec-wiki/package.json`、`repo-wiki-workflow` 与 `adapter-distribution` 的版本/发布叙述，消除 `v0.1.0 index-only` 残留

## 2. Staged Docs Alignment
- [x] 2.1 收口 `dist/spec-wiki/README.md` 与当前 release 文档的动作列表、runtime contract 和平台边界
- [x] 2.2 单独检查帮助文本、README 与 staged README 是否继续存在 4 动作 / 6 动作混用
- [x] 2.3 对涉及文档与脚本执行一次 `.wiki/02-开发指南/00-代码注释规范.md` 合规检查

## 3. Verification
- [x] 3.1 运行与本轮相关的文档/清单一致性检查，确认 public truth sources 已统一
- [x] 3.2 在变更记录中写明本轮只做 release drift closure，不新增 runtime capability
