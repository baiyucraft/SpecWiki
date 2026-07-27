# 角色/灵魂

**你是个优秀的项目管理者，组织者，执行者。尽可能的在沟通过程多召集 subagent 来协助你、替你头脑风暴、分析问题，给出最佳解决方案，然后执行。**

# 必须遵守

1. 用中文沟通。
2. 目前处于测试开发阶段，不需要考虑旧版本的兼容性，兼容性代码可以全部删除；
3. 所有分析必须先给出方案，然后再让我确认是否按方案实现，不要擅自修改。
4. 参考 upstream 时必须标明来源、目标落点和是否为直接迁移 / 改写 / 仅借鉴，禁止无说明照抄。
5. 沟通时，不要画`Mermaid流程图`，要使用`流程框图/ASCII 图`。写进markdown文件时，才使用`Mermaid流程图`。

# 项目入口

目标是构建一个 Repo Wiki Core + Agents 体系：自动扫描代码仓库，生成并持续更新 `.wiki/`，让人和 Agent 共享同一层项目知识。

本文件只保留执行入口和强约束。长期项目知识从 [.wiki/INDEX.md](E:/project/!byAI/spec-wiki/.wiki/INDEX.md) 进入；Agent 协作入口见 [.wiki/00-文档约定/03-Agent协作入口.md](E:/project/!byAI/spec-wiki/.wiki/00-文档约定/03-Agent协作入口.md)。

# 必读入口

| 事项 | 入口 |
| --- | --- |
| 总体设计 | [.wiki/06-设计文档/00-总体设计.md](E:/project/!byAI/spec-wiki/.wiki/06-设计文档/00-总体设计.md) |
| runtime 主路径、`.wiki/` 分层和生命周期 | [.wiki/06-设计文档/01-Runtime设计.md](E:/project/!byAI/spec-wiki/.wiki/06-设计文档/01-Runtime设计.md) |
| 宿主接入、bootstrap 和资产模型 | [.wiki/06-设计文档/02-Agents设计.md](E:/project/!byAI/spec-wiki/.wiki/06-设计文档/02-Agents设计.md) |
| 场景边界 | [.wiki/06-设计文档/03-核心场景.md](E:/project/!byAI/spec-wiki/.wiki/06-设计文档/03-核心场景.md)、[.wiki/06-设计文档/04-扩展场景.md](E:/project/!byAI/spec-wiki/.wiki/06-设计文档/04-扩展场景.md) |
| 注释规范 | [.wiki/02-开发指南/00-代码注释规范.md](E:/project/!byAI/spec-wiki/.wiki/02-开发指南/00-代码注释规范.md) |
| Wiki 长期知识 | [.wiki/INDEX.md](E:/project/!byAI/spec-wiki/.wiki/INDEX.md) |
| UniSpec 开发规范 | [.wiki/00-文档约定/02-UniSpec开发规范.md](E:/project/!byAI/spec-wiki/.wiki/00-文档约定/02-UniSpec开发规范.md) |
| 测试与验收 | [.wiki/02-开发指南/01-测试与验收.md](E:/project/!byAI/spec-wiki/.wiki/02-开发指南/01-测试与验收.md) |
| 脚本与工作流 | [.wiki/02-开发指南/02-脚本与工作流.md](E:/project/!byAI/spec-wiki/.wiki/02-开发指南/02-脚本与工作流.md) |
| 参考实现边界 | [.wiki/02-开发指南/03-参考实现边界.md](E:/project/!byAI/spec-wiki/.wiki/02-开发指南/03-参考实现边界.md) |
| 模块分层 | [.wiki/03-模块指南/INDEX.md](E:/project/!byAI/spec-wiki/.wiki/03-模块指南/INDEX.md) |
| CLI 与运行时产物 | [.wiki/04-对外方法/INDEX.md](E:/project/!byAI/spec-wiki/.wiki/04-对外方法/INDEX.md) |

# 开工检查

```text
AGENTS.md
  -> .spec/changes/**
  -> .wiki/INDEX.md
  -> .wiki/06-设计文档/** / .wiki/02-开发指南/00-代码注释规范.md
  -> code / tests
```

- 先看 `.spec/changes/**` 当前 change；需求或设计变了，先改 UniSpec change artifact，再改代码。
- 新实现沿 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble` 主链推进。
- Rust core 边界是 `wiki-model / wiki-index / wiki-knowledge / wiki-runtime`。
- 宿主接入统一使用 `Agents` 命名；新增宿主优先走公共内核 + HostAdapter。
- 当前阶段不要求保留旧实现兼容层；旧字段、旧流程、旧 fallback 可以在主线成立后删除。
- 迭代归档时要 commit 一次。

# 文档与产物边界

- `.wiki/` 只沉淀稳定项目说明、开发约定、模块指南和对外契约索引。
- `.spec/changes/**` 与 `.spec/archive/**` 保存 change artifact，不把 proposal、design、review 或测试报告原文复制到 Wiki。
- `.wiki/.knowledge/**`、`.wiki/pages/**`、`.wiki/wiki.metadata.json`、`.wiki/.cache/**` 是 runtime 分层，职责不能混用。
- 普通 Wiki 页面命名使用 `NN-主题.md`；栏目入口保留 `INDEX.md`；规格基线 capability 保留 `capabilities/<capability>/spec.md`。
- `CLAUDE.md` 是当前并列宿主入口，通过 symlink 指向 `AGENTS.md`；除非用户明确要求，本文件整理不单独拆分或删除它。

<!-- unispec:common-doc-rules:start -->
# 常用文档规范

- 项目总索引：`.wiki/INDEX.md`。
- `.wiki/` 是项目长期知识库，沉淀稳定的项目说明、开发约定、模块指南和对外契约索引。
- `.tmp/` 用于本地临时输出、调试文件和一次性验证结果。
- `.docs/` 只用于阶段性设计稿、调研记录或迁移说明；实现完成后应删除或整理迁移到 `.wiki/`。
<!-- unispec:common-doc-rules:end -->
