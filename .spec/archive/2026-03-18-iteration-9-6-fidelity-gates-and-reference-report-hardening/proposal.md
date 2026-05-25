## Why

`9.5` 已经把 `storybook + dagger` 的专项报告做到了 `overall_match_rate = 100%`，但真实产物仍然暴露出严重失真：`storybook` 当前报告同时给出 `low-fidelity = 176`、`extra generated = 165`，并且同一生成页被多个 reference 页面复用；`dagger` 的当前测试 runtime 甚至只剩 `.cache/wiki-cache.db` 而没有最终 Markdown 页面。现在不先把“什么叫接近 reference 的可读页面”固化成稳定、可复现、可定位的工程指标，后续 `9.7-9.9` 的 planner / research / compose 收敛都会继续被错误验收口径污染。

## What Changes

- 把 `9.6` 的范围严格限定为 `Fidelity Gates & Reference Report Hardening`：只升级专项报告、验证脚本和验收口径，不修改 `wiki-core` 的 planner / research / compose 主链。
- 升级 `scripts/collect-reference-project-reports.mjs`，把最终 `reference -> generated` 匹配对保留为正式分析输入，而不是只在中间选择阶段短暂存在。
- 新增 `many-to-one reuse` 正式一级指标，至少包含 `reuse_count(page)`、`reuse_pages`、`severe_reuse_pages`、`reuse_overage`，修复当前“reuse 严重但 collapsed = 0”的失真。
- 新增 `skeleton fidelity` 正式一级指标：从最终 `.wiki/*.md` 与 reference Markdown 提取并标准化 H2/H3 heading，忽略 `目录`、`附录`、`章节结构图` 与空标题 cite preamble，计算 `skeleton_score`。
- 新增 `key source coverage` 正式一级指标：优先提取 `file://` citation 链接，再 fallback 到文件提及，比较 generated/reference 的关键文件集合重合率。
- 保留 `overall_match_rate`，但将其降级为一级门槛，不再单独作为完成判据。
- 在项目报告中新增正式定位块：`reuse reverse index top offenders`、`skeleton lowest pages`、`key-source lowest pages`、`symptom -> metric -> offending pages -> contract hypothesis` gap ledger。
- 把专项报告同时输出为“可读 Markdown 报告 + 稳定结构化数据”，让后续 `9.7-9.9` 可以直接消费 9.6 的诊断结果，而不是重新从文本里猜。
- 明确 9.6 的正式基线必须来自 fresh run 生成的原子快照；当前 `tmp/test/*` 目录只可用于现状诊断，不能直接充当专项验收基线。
- 为 heading normalize、ignore list、key-source extraction、reuse/collapse 分类补脚本级测试，并增加同项目 warm report 重跑稳定性检查，确保指标抖动可控。
- 专项验收范围继续固定为 `storybook + dagger`，并明确报告必须回答四个问题：页数是否接近 reference、是否存在 coarse page reuse、docs-backed 页面是否具备 reference 式骨架、正文是否真正覆盖关键文件。
- **BREAKING**：废弃当前把 `collapsed pages` 仅绑定到窄化 note 模式的分类方式；旧的“只有 topic 被折叠进非 topic 页才算 collapsed”口径不再作为正式验收定义。
- **BREAKING**：`runtime_incomplete` 项目不再继续计算 `overall_match_rate` 或进入 fidelity 汇总；其专项 gate 直接失败，并单独输出不完整原因。

## Capabilities

### New Capabilities
- `reference-fidelity-reporting`: 定义 reference 对比报告的正式指标、结构化输出、诊断定位块与稳定性校验。

### Modified Capabilities
- `workflow-verification`: 将 `storybook + dagger` 的专项验收从单纯 `overall_match_rate` 升级为包含 reuse、skeleton fidelity、key source coverage 与 warm report stability 的正式门槛。

## Impact

- 影响 `scripts/collect-reference-project-reports.mjs`、相关脚本测试，以及 `changes/**/reference-project-reports/*.md` 的生成格式。
- 影响 `.wiki/05-规格基线/capabilities/workflow-verification/spec.md` 的专项验收定义，并新增 reference fidelity 报告能力 spec。
- 影响后续 `9.7-9.9` 的收敛入口，因为 planner / research / compose 将直接依赖 9.6 固化后的 gap ledger 和 fidelity gates。
