# 阶段材料索引

`.docs/` 保存阶段性设计稿、调研记录、路线说明、发布缺口分析和质量门禁分析。这里的内容不是当前稳定项目知识的 SSOT；采纳后应先进入 `.spec` change artifact，完成实现和验收后再提炼到 `.wiki/`。

| 分类 | 目录 | 内容 |
| --- | --- | --- |
| 设计草案 | [design](./design/) | 活跃草案或已迁移设计的历史入口；稳定合同以 `.wiki/` 为准 |
| 调研记录 | [research](./research/) | 外部理念、参考材料和分析记录 |
| 路线规划 | [roadmap](./roadmap/) | 实施路线、program roadmap 和历史路径 |
| 发布材料 | [release](./release/) | 版本缺口、发布收口和 release drift 分析 |
| 质量分析 | [quality](./quality/) | 质量门禁、验收规则和专项分析 |

## 当前文件

| 文件 | 类型 | 处理方式 |
| --- | --- | --- |
| [design/specwiki-code-graph-index-design.md](./design/specwiki-code-graph-index-design.md) | 已迁移的 code graph/index 设计入口 | 稳定合同见 wiki-index 模块页与 capability；原稿随 contract-closure parent 归档 |
| [design/knowledge-to-wiki-projection-contract.md](./design/knowledge-to-wiki-projection-contract.md) | 已迁移的 projection/writeback 设计入口 | 稳定合同见 Runtime 设计与 projection capability；原稿随 parent 归档 |
| [design/governance-runtime-integration.md](./design/governance-runtime-integration.md) | 已迁移的 governance runtime 设计入口 | 稳定合同见 Runtime 设计、runtime 模块页和 CLI 页；原稿随 parent 归档 |
| [design/specwiki-cli-unification.md](./design/specwiki-cli-unification.md) | 已迁移的一级 CLI 设计入口 | 稳定合同见对外方法/CLI；原稿随 parent 归档 |
| [design/specwiki-contract-closure.md](./design/specwiki-contract-closure.md) | 已完成的 contract-closure program 入口 | 8 个 child 已归档；稳定合同进入 `.wiki`，原稿随 parent 归档 |
| [research/karpathy-llm-wiki-analysis.md](./research/karpathy-llm-wiki-analysis.md) | 外部理念分析 | 只提炼理念和不采用边界 |
| [roadmap/implementation-roadmap.md](./roadmap/implementation-roadmap.md) | 3.0 实施路线 | 保留为阶段路线参考 |
| [roadmap/knowledge-system-completeness-roadmap.md](./roadmap/knowledge-system-completeness-roadmap.md) | knowledge system 历史路线镜像 | 当前 authoritative source 已切换到 contract-closure 主线 |
| [release/v0-2-0.md](./release/v0-2-0.md) | v0.2.0 版本说明 | 保留为历史发布面记录 |
| [release/v0-2-0-knowledge-runtime-gaps.md](./release/v0-2-0-knowledge-runtime-gaps.md) | v0.2.0 缺口分析 | 保留为历史判断依据 |
| [release/v0-2-0-release-gap-closure.md](./release/v0-2-0-release-gap-closure.md) | 发布收口过程 | 不全文沉淀到 Wiki |
| [quality/knowledge-quality-gates-acceptance.md](./quality/knowledge-quality-gates-acceptance.md) | 质量门禁验收分析 | 稳定 gate 原则可提炼到 `.wiki` |
