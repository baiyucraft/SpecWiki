# 阶段材料索引

`.docs/` 保存阶段性设计稿、调研记录、路线说明、发布缺口分析和质量门禁分析。这里的内容不是当前稳定项目知识的 SSOT；采纳后应先进入 `.spec` change artifact，完成实现和验收后再提炼到 `.wiki/`。

| 分类 | 目录 | 内容 |
| --- | --- | --- |
| 设计草案 | [design](./design/) | 尚未完全实现或仍在推衍的架构设计 |
| 调研记录 | [research](./research/) | 外部理念、参考材料和分析记录 |
| 路线规划 | [roadmap](./roadmap/) | 实施路线、program roadmap 和历史路径 |
| 发布材料 | [release](./release/) | 版本缺口、发布收口和 release drift 分析 |
| 质量分析 | [quality](./quality/) | 质量门禁、验收规则和专项分析 |

## 当前文件

| 文件 | 类型 | 处理方式 |
| --- | --- | --- |
| [design/specwiki-code-graph-index-design.md](./design/specwiki-code-graph-index-design.md) | SpecWiki code graph / index 专项设计草案 | 采纳前先进入 `.spec` change artifact |
| [design/knowledge-to-wiki-projection-contract.md](./design/knowledge-to-wiki-projection-contract.md) | knowledge 到 Wiki 页面投影、metadata 绑定和 `sync` 回写边界 | 采纳前先进入 `.spec` change artifact |
| [design/governance-runtime-integration.md](./design/governance-runtime-integration.md) | `.spec` governance 融合为 SpecWiki 治理产品层的设计草案 | 采纳前先创建 parent governance change 并拆分 child |
| [design/specwiki-cli-unification.md](./design/specwiki-cli-unification.md) | SpecWiki CLI 一级命令统一设计草案 | 采纳前先进入 `.spec` change artifact |
| [design/specwiki-contract-closure.md](./design/specwiki-contract-closure.md) | 新版 SpecWiki 页面树、truth 分层、恢复、query、governance 隔离和产品体验收口合同 | 作为后续实现拆分 change 的上位草案 |
| [research/karpathy-llm-wiki-analysis.md](./research/karpathy-llm-wiki-analysis.md) | 外部理念分析 | 只提炼理念和不采用边界 |
| [roadmap/implementation-roadmap.md](./roadmap/implementation-roadmap.md) | 3.0 实施路线 | 保留为阶段路线参考 |
| [roadmap/knowledge-system-completeness-roadmap.md](./roadmap/knowledge-system-completeness-roadmap.md) | knowledge system roadmap | 以对应 `.spec/changes/**` 为 authoritative source |
| [release/v0-2-0.md](./release/v0-2-0.md) | v0.2.0 版本说明 | 保留为历史发布面记录 |
| [release/v0-2-0-knowledge-runtime-gaps.md](./release/v0-2-0-knowledge-runtime-gaps.md) | v0.2.0 缺口分析 | 保留为历史判断依据 |
| [release/v0-2-0-release-gap-closure.md](./release/v0-2-0-release-gap-closure.md) | 发布收口过程 | 不全文沉淀到 Wiki |
| [quality/knowledge-quality-gates-acceptance.md](./quality/knowledge-quality-gates-acceptance.md) | 质量门禁验收分析 | 稳定 gate 原则可提炼到 `.wiki` |
