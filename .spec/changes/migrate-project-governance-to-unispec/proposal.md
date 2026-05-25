# migrate-project-governance-to-unispec

## 问题

当前项目的变更治理仍由 legacy spec system 承载，目录、脚本、测试、扫描过滤、Agent Skill 和历史文档存在多处旧治理入口。继续保留双重对象语言会让后续知识运行时设计、质量门禁和协作流程难以收敛。

本 change 是迁移 program 的控制面，目标是把治理体系统一到 UniSpec。它不把本阶段视为迁移完成，也不在 proposal 阶段切换任何运行入口。

## 目标

- 建立 `.spec/` 作为 UniSpec 治理控制面。
- 明确 legacy spec system 到 UniSpec 的分阶段迁移边界。
- 将后续结构迁移、artifact 转换、脚本测试切换、Skill 发现验证和旧术语清理纳入同一个 program。
- 让最终仓库只保留 UniSpec / `.spec` / change artifact 语义。

## 非目标

- 本 proposal 阶段不切换 CLI、脚本、测试、runtime scanner 或 Agent Skill 的执行入口。
- 本 proposal 阶段不声称全量迁移已经完成。
- 本 proposal 阶段不引入旧治理兼容层。
- 本迁移不修改产品包名 `spec-wiki`。

## 成功标准

- `.spec/config.yaml` 存在，并采用 UniSpec 基线配置。
- `.spec/changes/migrate-project-governance-to-unispec/meta.yaml` 存在且声明 migration control metadata。
- 本 proposal 明确迁移 phases、双轨边界、最终退出条件和验证命令。
- 最终态门槛为全仓不再出现 legacy spec system 的旧品牌词或旧目录名。
- UniSpec status / validate、项目 lint / test / cargo test 能作为后续验收入口。

## 影响范围

- Governance artifacts：active changes、archive、长期 capability baseline。
- Scripts：reference report、debug trace、批量测试报告路径。
- Runtime：scanner noise filter、generation context filter。
- Tests：路径断言、噪声过滤断言、snapshot 读取路径。
- Agent skills：UniSpec skills、reviewer agents、宿主发现 wrapper。
- Docs：AGENTS、设计文档、阶段性文档、历史 archive 正文。
- Upstream refs：仓库内不保留旧治理系统参考目录。

## 交付形态

multi-change

这是迁移 program 的 parent / umbrella change。后续 child change 按以下阶段推进：

```text
Phase 1: structure migration
  -> .spec/.agents/.wiki 基线落地
  -> legacy governance artifacts 移入 .spec

Phase 2: artifact normalization
  -> active changes 补 meta.yaml / system-tests.md
  -> archived changes 统一路径和术语
  -> capability baseline 迁入 .wiki

Phase 3: tooling switch
  -> scripts/tests/runtime scanner/lint 改为 .spec
  -> reference reports 写入 .spec

Phase 4: agent migration
  -> UniSpec skills / agents 生效
  -> 必要时提供 Codex wrapper

Phase 5: final cleanup
  -> 移除旧参考目录
  -> 全仓旧术语清零
  -> 运行完整验证
```

## 风险

- 历史 archive 体量较大，术语清理可能误改引用语义。
- `.wiki` 同时承担项目长期知识和当前产品知识运行时语义，需要避免覆盖已有内容。
- Agent Skill 发现机制在不同宿主间不同，可能需要 `.agents` 与宿主 wrapper 并存。
- 常驻 capability baseline 没有 UniSpec 同名目录，迁移到 `.wiki` 时必须保留可追溯性。

## 未知项

- 当前宿主是否直接发现 `.agents/skills`，需要在 agent migration 阶段验证。
- 已归档 change 是否需要补齐 UniSpec review / verification 报告，只能在 artifact normalization 阶段逐类判定。

## 参考资料

- `E:\project\!byAI\unispec-demo`
- `.spec/config.yaml`
