## 1. Public Surface Contract
- [x] 1.1 更新 `repo-wiki-workflow` 与 `codebuddy-agent-integration` 的 UniSpec 合同，正式公开 `sync / rebuild`
- [x] 1.2 更新 CLI/help、README 或等价公开入口，移除“`sync/rebuild` 非公开”的表述
- [x] 1.3 更新宿主 bootstrap / action skill 资产，使 `wiki-sync`、`wiki-rebuild` 成为正式公开入口

## 2. Surface Alignment
- [x] 2.1 对齐 `sync` 与 `rebuild` 的公开命名、帮助文本和宿主入口，不改现有 runtime 语义
- [x] 2.2 对齐 `status/query` 与公开入口的说明，明确已有 `recommended_action = sync | rebuild` 现在可以被直接执行，但不修改推荐逻辑
- [x] 2.3 对涉及代码与脚本执行一次 `.wiki/02-开发指南/00-代码注释规范.md` 合规检查

## 3. Verification
- [x] 3.1 补 CLI / host / public-surface 自动化测试，验证 `sync/rebuild` 已正式公开
- [x] 3.2 补代表性脚本验证，确认 `storybook` 至少覆盖 `init -> status -> sync -> query/update -> rebuild`
- [x] 3.3 在变更记录中写明本轮不包含 19 项目全量回归，仅将其保留为后续 baseline guard



