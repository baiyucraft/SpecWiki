---
title: SpecWiki 治理运行时融合设计
description: 将 .spec governance 融合为 SpecWiki 的治理产品层，并复用 .wiki / knowledge / index / query / sync 底座
updated: 2026-06-10
owner: architecture
status: draft
---

# SpecWiki 治理运行时融合设计

## 文档定位

本文是阶段性设计稿，不是当前稳定实现说明。

目标是重新定义 `.spec` governance 与 SpecWiki 的关系：治理流程不再作为仓库旁路的独立工具存在，而是成为 SpecWiki 面向变更治理的产品层；底层模型、知识、索引、查询、同步和运行时能力统一复用 SpecWiki core。

本设计不直接修改 `.wiki` 稳定知识、不移动 `.spec` 目录，也不承诺一次性实现完整迁移。采纳本设计前，应先创建对应 governance parent change，再拆分 child change 逐步落地。

## 产品定位

SpecWiki 是 repo-local knowledge runtime：它把源码、文档、配置、接口、模块边界、运行脚本和变更证据组织成可查询、可更新、可被 Agent 使用的项目知识层。

SpecWiki 的核心不是“生成一批 Markdown”，也不是“替代项目管理工具”。它的核心价值是让人和 Agent 在同一个仓库内共享同一套项目事实、稳定知识、代码索引和变更上下文：

```text
repo facts
  -> code index
  -> structured knowledge
  -> readable wiki pages
  -> query / status / update / governance
```

governance 是建立在 repo knowledge runtime 上的一个产品层，用于让 proposal、design、review、verification、archive 等变更证据进入统一状态、查询和验证体系。它是 SpecWiki 的重要能力，但不是 SpecWiki 的全部。

## 目标用户与核心场景

SpecWiki 同时服务四类角色：

- 项目维护者：初始化 `.wiki`、检查知识是否过期、处理治理归档和发布前状态。
- 日常开发者：查询模块、接口、脚本、配置和影响范围。
- Reviewer / 架构负责人：检查 change 状态、缺失 artifact、review gate、release gate 和 wiki-sync issue。
- Agent / Skill：通过稳定 CLI 和 query 获取项目上下文，减少全仓扫描和猜测状态。

核心场景压缩为五个高频意图：

```text
新仓库接入
  -> spec-wiki init
  -> 生成最小 .wiki、知识目录、索引缓存、治理发现状态和 Agent 入口

日常同步
  -> spec-wiki update
  -> 刷新代码事实、知识计划、派生状态和必要的页面投影

项目查询
  -> spec-wiki query <term>
  -> 融合 code index、knowledge、wiki pages 和 governance artifact references

变更检查
  -> spec-wiki status
  -> 返回 wiki health、index freshness、knowledge freshness、governance readiness 和 next action

变更归档
  -> spec-wiki archive <change-id>
  -> 仅在 governance-enabled repo 中作为公共闭环动作，必须经过 validate、wiki-sync 和显式确认
```

## 上游参考分级

本设计只把仍然贴近当前目标的 upstream 作为架构依据。其它项目保留为体验或流程提示，不作为实现蓝本，避免把已经偏离目标的问题域带进 SpecWiki。

| 分级 | 来源 | 观察 | 目标落点 | 采用方式 |
| --- | --- | --- | --- | --- |
| 核心架构参考 | `.upstream/codegraph` | 本地 code graph、SQLite/索引、MCP/Agent 查询、减少 Agent 文件扫描 | `wiki-index` 的 graph / query / context / impact 能力 | 改写借鉴 |
| 核心架构参考 | `.upstream/GitNexus` | 一个 backend 同时服务 CLI / MCP / HTTP，内部 ingestion DAG 清晰；但公开 CLI 已膨胀 | 统一 runtime backend、内部 phase DAG、跨接口同源 | 改写借鉴 |
| 流程与用户出口参考 | UniSpec / 当前 `.spec` governance 规范 | `proposal -> design -> plan -> apply -> review -> archive` 的 change lifecycle、artifact gate、skill 分阶段交互已经验证了用户如何通过少量入口完成治理流程 | SpecWiki governance product layer 的 public intent commands、Agent / Skill 交互规程、`.spec` artifact lifecycle | 改写借鉴，不作为最终产品命名，不迁移 `unispec` executable / package / skill 命名 |
| Wiki 目录基线参考 | `.upstream/unispec-template/.wiki/**` | 原始模板采用 `.wiki/INDEX.md`、`00-文档约定`、`01-快速上手`、`02-开发指南`、`03-模块指南`、`04-对外方法` 的基础栏目结构 | SpecWiki `.wiki` Markdown 页面树目录基线 | 改写借鉴，保留栏目组织方式，不保留 UniSpec 命名，不带规格基线 |
| 辅助流程参考 | `.upstream/deepwiki-rs` | 内部 `preprocess -> research -> compose -> output` 管线清楚，但产品目标是文档生成引擎 | `.wiki` 内容生成和 knowledge planning 的阶段隔离 | 仅借鉴 / 改写借鉴 |
| 产品体验参考 | `.upstream/codewiki` | 用户心智接近 `config + generate`，复杂度藏在配置和高级参数 | public CLI 简化原则 | 仅借鉴 |
| 产品体验参考 | `.upstream/deepwiki-open` | Web 用户只看到输入 repo、生成 Wiki、Ask；项目重心已转向其它方向 | 隐藏实现阶段的产品体验 | 仅借鉴 |

不采用：

- 不直接迁移任何 upstream 源码级 CLI surface。
- 不把 UniSpec 作为目标产品名、layer 名称、命令名、package 名、namespace 或用户可见主入口。
- 不采用 GitNexus 的完整公开命令规模。
- 不采用 CodeWiki 的论文式多 agent 复杂生成链作为 runtime 主线。
- 不采用 deepwiki-open 的 Web/RAG 架构作为本地 repo runtime 蓝本。
- 不把 deepwiki-rs 的 C4/page-first 文档生成目标改造成 SpecWiki 的核心产品目标。

## 参考规则迁移口径

治理流程可以借鉴既有规范，但必须在 SpecWiki 中重新命名、重新建模、重新验收。

| 来源规则 | SpecWiki 目标规则 | 采用方式 | 验收方式 |
| --- | --- | --- | --- |
| stage lifecycle | `GovernanceStage` 与 runtime status transition | 改写 | fixture 中 stage 识别与当前治理规则一致 |
| artifact matrix | required artifact policy | 改写 | validate 能报告 missing / stale / invalid artifact |
| review report frontmatter | `ReviewGateContract` 输入 | 改写 | pass / fail / partial / blocked 输出稳定 |
| archive readiness | archive readiness policy + operation manifest | 改写 | dry-run 不移动目录但能生成完整 readiness report |
| parent / child change | `ChangeRelation` 与 parent-child archive gate | 改写 | parent / child fixture 覆盖同步和阻塞条件 |
| skill 分阶段交互 | Agent / Skill wrapper 行为 | 仅借鉴 | skill 不再自行实现状态机，只调用 runtime DTO |

## 背景

当前仓库已经完成两条主线：

- SpecWiki 提供 Repo Wiki Core + Agents 体系，核心分层为 `wiki-model / wiki-index / wiki-knowledge / wiki-runtime`。
- `.spec` governance 提供变更治理流程，使用 `.spec/changes/**` 和 `.spec/archive/**` 保存 proposal、design、tasks、review、verification 和 archive evidence。

现状问题不是两者不能协作，而是职责仍然偏并列：

- 治理状态机主要存在于 legacy governance skills、legacy governance CLI 和 `.spec` artifact 约定中。
- SpecWiki runtime 需要统一管理 `.wiki` Markdown 页面树、`.wiki/.knowledge/**`、`wiki.metadata.json`、SQLite cache、query、sync、update、rebuild 等底座，但还没有把 governance 作为一等产品层。
- `.wiki` 当前能沉淀治理规范，但还不能把 change lifecycle、review gate、release gate、wiki-sync issue 变成可查询、可验证、可恢复的治理知识。

因此，本设计把 `.spec` governance 收敛为 SpecWiki 的 governance product layer。

## 目标

- 让 `.spec` governance 成为 SpecWiki 的治理产品形态，而不是外挂脚本系统。
- 让 governance status、validate、show、archive、query 进入 SpecWiki runtime 合同，但 public CLI 只暴露用户意图，不暴露完整治理模块菜单。
- 让 `.spec` artifact 的读取、校验、状态计算和 archive readiness 由 runtime governance service 承担。
- 让治理规则、capability baseline、change delta、wiki-sync issue 和 release gate 进入 `wiki-knowledge` 的知识组织能力。
- 让治理对象共享 `wiki-model` 的稳定 ID、DTO、状态和引用模型。
- 让 Agent / Skill 只做交互入口和操作规程，不再自己实现治理状态机。
- 让普通用户只需要记住少量日常命令，expert / debug namespace 服务实现者、测试和高阶排障。
- 让 `spec-wiki init` 成为唯一对外初始化入口；宿主 bootstrap、runtime 构建、索引初始化、knowledge 初始化和治理发现都只是该入口的内部阶段。

## 非目标

- 不把 `.spec/changes/**` 搬进 `.wiki/`。
- 不把 proposal、design、review、test report 原文复制到 Wiki 长期页面。
- 不让 `.wiki/pages/**` 与 `.wiki/**/*.md` 长期形成双页面 truth。
- 不让 SQLite cache 取代 `.spec` artifact 的正式审计证据。
- 不在 TS CLI 或 skill markdown 中重新实现治理 validator。
- 不第一阶段迁移 archive；archive 涉及目录移动和审计边界，必须最后做。
- 不保留两个长期互相不一致的入口，即 legacy governance CLI 与 `spec-wiki` 一级治理命令不应长期并行。
- 不把 SpecWiki 定位为通用项目管理、ticket 或 issue tracker。
- 不取代 Git、PR、CI、release system，只提供 repo-local knowledge 和 governance evidence 的统一视图。
- 不承诺首次 init 后生成完整正确的长期 Wiki；初始结果可以是可验证、可追踪、可继续更新的知识骨架。
- 不把 governance 作为所有仓库的强制流程；没有 `.spec` 或未启用 governance 的仓库仍应能使用 init / update / query。
- 不让 public CLI 暴露内部 phase、cache、scanner、knowledge planning 等实现概念。
- 不对用户暴露第二套 `wiki init` 或 `governance init` 心智；这些只能作为 runtime 内部 action、测试入口或 expert/debug 术语存在。
- 不把 `.wiki` 变成需求文档、会议记录或临时设计稿的堆放处。

## 命名规则

产品名统一为 SpecWiki。正式 CLI executable 仍为 `spec-wiki`，命令示例、脚本、测试和 Agent 调用都使用小写连字符形式。

```text
产品名: SpecWiki
包名: spec-wiki
正式命令: spec-wiki
命令 alias: 不提供
```

不使用大写 `SpecWiki`、camelCase `specWiki` 或紧凑小写 `specwiki` 作为真实 executable 或 alias，避免 npm 包名、跨平台 PATH、shell 大小写敏感和宿主资产生成产生不一致。命令、包名、目录、配置路径和 Agent 调用统一使用 kebab-case：`spec-wiki`、`packages/spec-wiki`、`~/.spec-wiki`。

`UniSpec` 只允许作为历史来源或参考项目名称出现；目标产品名、CLI executable、package、namespace、正式 layer 名称和用户可见主入口均统一使用 `SpecWiki` / `spec-wiki`。

## 目标分层

```text
SpecWiki
  Product Layer
    public intent commands
      init / status / update / query / archive
    advanced commands
      sync / rebuild / changes / change / doctor / repair / trace

  Runtime Core
    wiki-runtime
      workflow orchestration / storage / transport / lifecycle
      GovernanceRuntimeService

  Knowledge Core
    wiki-knowledge
      Wiki knowledge
      Governance knowledge

  Index Core
    wiki-index
      code facts / symbol graph / artifact references

  Model Core
    wiki-model
      stable IDs / DTO / lifecycle states / references

  Agents Layer
    packages/spec-wiki
      host bootstrap / thin runtime forwarding
```

## 目录与真相源

| 区域 | 融合后职责 | 是否正式 truth |
| --- | --- | --- |
| `.spec/changes/**` | active change artifact、proposal/design/tasks/reports/meta | 是，change evidence truth |
| `.spec/archive/**` | archived change evidence | 是，historical governance truth |
| `.wiki/.knowledge/**` | formal knowledge artifacts，包括 declared / derived / runtime | 是，knowledge truth |
| `.wiki/**/*.md` | Markdown 页面树，包括总入口、栏目入口、稳定知识页、page projection / authoring surface | 是，readable page truth，不是 governance state truth |
| `.wiki/wiki.metadata.json` | 页面树与 knowledge 的绑定索引，包括 page id、path、hash、knowledge refs、scope、version、staleness | 是，runtime binding truth |
| `.wiki/.cache/**` | SQLite cache、本地状态、查询加速和恢复目标 | 否，可重建；可以是本地 runtime 主存储，但不是审计 truth |
| `.docs/**` | 阶段性设计、调研、路线和发布材料 | 否，draft / supporting material |

`.spec` 继续保留，因为它是审计证据库。融合不是目录合并，而是 runtime 能把 `.spec` 读成治理状态、治理知识和可查询引用。

`.wiki` 的目标页面层是一套 Markdown 页面树：

```text
.wiki/
  INDEX.md

  00-文档约定/
    INDEX.md
    00-边界与SSOT规则.md
    01-页面模板.md
    02-变更治理规范.md      # governance-enabled
    03-Agent协作入口.md     # agent-enabled

  01-快速上手/
    INDEX.md
    00-环境准备.md        # 按需
    01-启动项目.md        # 按需
    02-构建项目.md        # 按需
    03-常见问题.md        # 按需

  02-开发指南/
    INDEX.md
    00-代码注释规范.md    # 可内置
    01-测试与验收.md      # 按需
    02-脚本与工作流.md    # 按需

  03-模块指南/
    INDEX.md              # 固定
    NN-<模块>.md
    <复杂模块>/
      INDEX.md
      00-模块定位.md
      01-入口与目录.md
      02-常见开发任务.md

  04-对外方法/
    INDEX.md              # 固定
    00-CLI.md             # 有 CLI 时生成
    01-API.md             # 有 API / SDK 时生成
    02-配置.md            # 有配置时生成
    03-协议与事件.md      # 有协议、事件、MCP 等时生成
```

以上是通用初始化基线，不包含规格基线栏目。若当前项目已有 `05-规格基线/**` 或 `capabilities/<capability>/spec.md`，它们只作为现有项目内容的迁移输入，后续是否保留或改写为普通 Wiki 页面应由独立治理 change 决定。

初始化文件集按 capability 开关分组：

```text
universal required:
  .wiki/INDEX.md
  .wiki/00-文档约定/INDEX.md
  .wiki/00-文档约定/00-边界与SSOT规则.md
  .wiki/00-文档约定/01-页面模板.md
  .wiki/01-快速上手/INDEX.md
  .wiki/02-开发指南/INDEX.md
  .wiki/03-模块指南/INDEX.md
  .wiki/04-对外方法/INDEX.md

governance-enabled:
  .wiki/00-文档约定/02-变更治理规范.md

agent-enabled:
  .wiki/00-文档约定/03-Agent协作入口.md
```

初始化 profile：

```text
minimal:
  只生成 universal required，保证人类和 Agent 有稳定阅读入口。

standard:
  在 minimal 基础上，根据项目事实和 capability 开关补充快速上手、开发指南、模块指南、对外方法、治理规范和 Agent 入口。
```

其它页面按项目事实生成。`03-模块指南/` 和 `04-对外方法/` 是固定一级栏目，但正文必须由扫描结果或用户确认驱动；没有模块、CLI、API、SDK、配置、协议或事件时，在对应 `INDEX.md` 中说明不适用原因，不硬造空页。

每个目录的 `INDEX.md` 只承担四类职责：

```text
范围说明
页面索引
入口推荐
事实来源 / 不适用说明
```

长正文应拆到同目录的 `NN-主题.md` 或子目录中，不把 `INDEX.md` 变成大杂烩页面。

多级目录规则：

```text
单页能讲清楚 -> 单页
同一主题有 2 个以上稳定子主题，并且存在独立读者任务或独立维护责任 -> 目录化
目录化后每层必须有 INDEX.md
默认最多生成到二级
三级只在代码结构或领域结构强烈支持时生成
模块页超过约定长度，或出现 3 类以上入口 / 流程 / 测试 / 配置主题 -> 可升级为目录
后续内容收缩后允许降级回单页
```

`.wiki/pages/**` 不作为目标页面目录。若旧设计或旧产物中存在 `.wiki/pages/**`，它只作为 legacy import source。迁移到 `.wiki/INDEX.md`、栏目 `INDEX.md` 和 `NN-主题.md` 前，runtime 不得同时写入两套 page truth。

页面树统一必须拆成独立前置 change：`unify-wiki-page-tree-runtime-contract`。该 change 需要同步 `.wiki/06-设计文档/01-Runtime设计.md`、`.wiki` 规范、runtime init/rebuild 的保护策略和 metadata 重建规则。在它完成前，governance integration 不承诺顺手废弃或迁移 `.wiki/pages/**`。

## 内容进入页面的路径

`.wiki/**/*.md` 只能消费 normalized knowledge、page projection 和可追踪引用，不直接拼接 `.spec` 原文或任意扫描文本。

```text
code index
  -> code facts / symbol graph
  -> derived knowledge
  -> page projection
  -> .wiki/**/*.md

.knowledge declared
  -> stable knowledge units
  -> page projection / manual page refs
  -> .wiki/**/*.md

.spec evidence
  -> governance runtime status / artifact refs
  -> governance knowledge summary
  -> 页面引用 / 摘要 / wiki-sync issue
```

约束：

- `.spec` 原始 proposal、design、review、verification 和 test report 不复制进 Wiki。
- Wiki 页面只能沉淀稳定结论、摘要、链接和待处理事项。
- 自动生成段落必须能追溯到 knowledge refs 或 evidence refs。
- governance 状态由 `.spec` evidence 和 runtime DTO 决定，不由 Markdown 页面推断。
- 页面 projection 可以展示治理摘要，但不能成为治理审计证据。

## 栏目职责边界

```text
.wiki/INDEX.md
  总入口、阅读路径、当前仓库知识地图、关键状态入口

00-文档约定/**
  SSOT、页面模板、Agent 协作、治理规则和文档维护规则

01-快速上手/**
  新读者如何安装、启动、构建、运行和排障

02-开发指南/**
  代码注释、测试验收、脚本工作流、贡献和维护习惯

03-模块指南/**
  内部模块定位、代码入口、维护任务、测试边界、常见开发任务

04-对外方法/**
  CLI / API / SDK / 配置 / 协议 / 事件 / MCP 等外部调用合同
```

模块页讲内部实现和维护任务；对外方法页讲用户、宿主或外部系统如何调用；治理页讲变更流程和规则。三者不得互相复制长正文，只能通过链接和短摘要建立关系。

## 模型边界

### wiki-model

`wiki-model` 应新增 governance 对象语言，供 runtime、knowledge、CLI 和 tests 共享。

建议对象：

- `GovernanceChangeId`
- `GovernanceStage`
- `DeliveryShape`
- `ArtifactKind`
- `ArtifactStatus`
- `ReviewResult`
- `VerificationResult`
- `ChangeRelation`
- `GovernanceReference`
- `ArtifactReference`
- `GovernanceBlockingIssue`
- `ArchiveReadiness`
- `QualityGateResult`
- `ReleaseGateResult`

这些对象只描述稳定状态和 DTO，不读取文件，不执行验证。

### wiki-index

`wiki-index` 继续负责代码事实和 symbol graph。治理融合后可增加 artifact reference index，但必须避免扫描污染：

- `.spec` 不进入业务源码 facts。
- `.wiki` runtime artifact 与 Markdown 页面树不进入源码 facts。
- 可以建立 governance artifact reference index，用于查询 change 与文件、模块、capability 的引用关系。
- artifact reference index 是治理查询输入，不是代码事实输入。
- governance artifact index 不复用 `scan_repo`，避免把治理证据误归类为源码上下文。

### wiki-knowledge

`wiki-knowledge` 应负责治理知识组织，而不是文件操作：

- governance policy knowledge：stage rule、artifact rule、archive rule、review gate。
- capability baseline 与 change delta 的关系。
- change 对 module / code / CLI / config / capability 的影响关系。
- wiki-sync issue 的分类和优先级。
- release readiness、quality gate、remaining gap 等 program-level governance signals。

治理知识可以沉淀为 `.wiki/.knowledge/declared/**` 或 derived governance summaries，并通过 `wiki.metadata.json` 绑定到 `.wiki/**/*.md` 页面树，但不能吞并 `.spec` 原始证据。

### wiki-runtime

`wiki-runtime` 承担 governance workflow 的实际状态机，但不应把 `.spec` 文件布局硬编码到所有 runtime 层。

目标边界：

```text
wiki-runtime
  -> GovernanceRuntimeService
       -> GovernanceEvidenceStore
            -> SpecDirectoryEvidenceStore(.spec)
       -> GovernancePolicyEngine
       -> ReviewGateContract
       -> GovernanceWorkflowExecutor
```

职责：

- 通过 `GovernanceEvidenceStore` 读取 `.spec/changes/**` / `.spec/archive/**`。
- 解析 `meta.yaml` 和 required artifact。
- 计算 active changes、stage、missing artifacts、blocking issues、archive readiness。
- 验证 proposal / design / system-tests / tasks / review-report / test-report 的存在与 frontmatter。
- 执行 governance CLI action。
- 在后续阶段执行 archive 目录移动和 parent / child 状态同步。
- 生成治理查询 payload 和 runtime transport DTO。

第一阶段只做 read-only status / show / validate，不做 archive。

### packages/spec-wiki

`packages/spec-wiki` 继续是 SpecWiki CLI、宿主 bootstrap 和 forwarding 层：

- 新增 public intent commands，并将用户意图路由到 wiki / governance runtime。
- 新增 advanced help 分组，只给实现者、测试和排障使用；`advanced` 不是命令 namespace。
- 将 `changes / change / validate / archive` 等一级命令 forward 到 runtime。
- 不在 TS 层重写 required artifact、stage 或 archive validator。
- 生成/更新宿主 skills 时，让 skill 优先调用 public intent command；只有需要精确治理 artifact 操作时才调用对应一级治理命令。

## CLI 目标形态

公开 CLI 表达用户意图，不表达内部模块分层。SpecWiki 的正式命令入口为 `spec-wiki`；`wiki` / `governance` 是 runtime 领域，不应该成为用户可见命令 namespace。

public commands 只暴露跨角色高频意图。`archive` 虽然属于 governance 场景，但它是变更闭环的最终用户动作，因此可以作为 public command；不过 Quick Start 应按 repo 是否启用 governance 决定是否展示，避免让首次用户误以为 SpecWiki 首先是流程归档工具。

### Public Commands

目标形态：

```text
spec-wiki init
spec-wiki status
spec-wiki update
spec-wiki query <term>
spec-wiki validate [change-id]
spec-wiki archive <change-id>
```

语义：

- `init`：初始化 repo wiki runtime、索引、知识目录和 Agent 入口。
- `status`：返回 repo wiki + governance 的统一状态，包含 `wiki health / index freshness / knowledge freshness / governance readiness / next action`。
- `update`：刷新 index、knowledge planning、derived state 和必要的治理派生状态。
- `query <term>`：融合 code index、knowledge、wiki page 和 governance artifact reference。
- `validate [change-id]`：无参数时验证 repo runtime；有 change-id 时验证治理 artifact、review gate 和 archive readiness。
- `archive <change-id>`：归档治理 change，但必须经过 runtime validate、wiki-sync 检查和显式确认。

governance blocking 不应默认阻断普通 Wiki 可用性；只有 archive、release、review gate 等治理动作才被 governance blocking 阻断。

### Advanced Commands

目标形态：

```text
spec-wiki sync
spec-wiki rebuild
spec-wiki changes
spec-wiki change <change-id>
spec-wiki doctor
spec-wiki repair
spec-wiki trace
```

advanced commands 是初始化后的维护、诊断和支持入口，不和 public commands 并列为普通用户主路径。`sync`、`rebuild`、`doctor`、`repair`、`trace` 不消失，但不进入 Quick Start 和默认 Agent 指令。它们是 runtime 维护动作，不是普通用户心智入口。

第一版 CLI 必须收窄，不一次实现完整目标形态：

```text
Phase 1 CLI candidate:
  spec-wiki init
  spec-wiki status
  spec-wiki changes
  spec-wiki change <change-id>
  spec-wiki validate <change-id>
```

`archive` 不进入第一阶段写操作。等 validate、wiki-sync issue、operation manifest、parent / child 关系稳定后，再开放 public archive。

## Agent / Skill 目标形态

融合后，skill 不再是治理内核：

```text
用户意图
  -> governance skill 判断交互阶段
  -> 优先调用 spec-wiki status / query
  -> 必要时调用 spec-wiki changes / change / validate
  -> 根据 runtime DTO 提示下一步
  -> 需要写 artifact 时按阶段写 .spec
  -> 需要查询知识时调用 spec-wiki query
```

迁移期间可保留 legacy governance skill 路径作为 wrapper：

- `.agents/skills/<legacy-governance-skill>`
- `.codex/skills/<legacy-governance-skill>`
- `.agents/agents/<legacy-governance-reviewer>`
- `.agents/agents/<legacy-governance-verifier>`

但它们应逐步改成：

- 不自行推断 stage。
- 不自行实现 required artifact validator。
- 不自行判断 archive readiness。
- 不直接解释 runtime query state。
- 不把 wiki 正文搬进 change artifact。
- 不解析 runtime DTO 后再补一套私有判断。

迁移期间需要 skill golden tests：同一 fixture change，legacy skill 判断与 `spec-wiki validate <change-id>` 结果必须一致。该 parity 通过后，才能删除旧状态机。

## Workflow

### Governance Status

```text
read .spec/changes/**
  -> parse meta.yaml
  -> classify parent / child / single change
  -> inspect required artifacts
  -> compute stage health
  -> compute blocking issues
  -> return governance status DTO
```

### Governance Validate

```text
read change artifact
  -> validate stage required files
  -> validate meta.yaml consistency
  -> validate parent / child relation
  -> validate report frontmatter when needed
  -> validate archive readiness when verification
  -> return blocking / warning / ready
```

### Review Gate Contract

Review gate 必须是 runtime 合同，不应继续散落在 skill 文案里。

```text
input:
  artifact refs
  declared scope
  review result
  verification evidence
  changed surfaces

output:
  pass / fail / partial / blocked
  blocking issues
  warning issues
  missing evidence
  next action
```

`partial` 只能在 scope 明确、未覆盖项可追踪且不会误导 archive readiness 时出现。`blocked` 必须给出具体 artifact、规则和建议动作。

### Governance Query

```text
term
  -> governance artifact reference index
  -> capability baseline / change delta knowledge
  -> wiki-sync issue / release gate / quality gate summaries
  -> related source/module/capability refs
```

它不是简单全文搜索 `.spec`，而是结构化治理查询。

### Archive

Archive 最后迁移。public archive 默认不得静默移动目录；必须先生成 readiness report 和 operation manifest。

```text
validate archive readiness
  -> dry-run operation manifest
  -> list source path / target path / hashes / meta diff / wiki-sync check
  -> explicit confirmation
  -> move .spec/changes/<change> to .spec/archive/<date-change>
  -> update parent split/meta when child archived
  -> compute wiki-sync issue
  -> optionally trigger wiki sync/update after explicit confirmation
```

operation manifest 至少包含：

- source path。
- target path。
- artifact hash summary。
- parent / child meta diff。
- wiki-sync issue summary。
- preflight result。
- executed step list。
- failure step 和 recovery hint。

Archive 可以检查 Wiki 沉淀需求，但不应静默写入长期知识。归档后如果需要沉淀，输出 `wiki-sync issue`，包括建议页面、原因、来源 evidence、是否需要人工确认。

## Wiki 内容生命周期与更新策略

Wiki 页面不是一次性生成物，而是由 code index、knowledge、manual edits 和 governance evidence 共同驱动的长期知识界面。

硬规则：

1. 自动更新只能改受控区块或 runtime projection，不覆盖人工区块。
2. 每个页面需要记录来源 refs、更新时间、staleness 状态，具体承载方式由 page metadata 或 `wiki.metadata.json` 决定。
3. 代码 index 变化先进入 knowledge/update plan，再决定是否改页面。
4. `.spec` archive 只产生 wiki-sync issue；长期知识沉淀需要显式确认。
5. 人工编辑页若与 derived knowledge 冲突，runtime 标记 conflict，不静默改写。
6. 页面删除必须经过 stale detection、引用检查和人工确认；不能因为本轮扫描缺失就立即删除稳定页面。
7. `INDEX.md` 只做导航和入口推荐，正文膨胀时必须拆页。

## 迁移合同

迁移不是“改几个命令名”，而是把旧状态机、旧命名、旧页面结构、旧 skill 行为逐步收敛到 SpecWiki runtime 合同。

| 迁移对象 | 目标落点 | 保留策略 | 删除门槛 |
| --- | --- | --- | --- |
| legacy governance CLI | `spec-wiki changes / change / validate / archive` | 短期 wrapper | runtime validate parity、CLI tests、skill forwarding 通过 |
| legacy governance skill 状态机 | runtime DTO + thin skill wrapper | 短期保留交互入口 | skill golden tests 通过，skill 不再自行判断 stage |
| legacy reviewer / verifier 文案规则 | `ReviewGateContract` | 改写为调用 runtime | review fixture 全覆盖 |
| `.wiki/pages/**` | `.wiki/**/*.md` 单页面树 | legacy import source | 页面树合同 change 完成，metadata 重建，runtime 不再写旧路径 |
| `05-规格基线/**` / `capabilities/**/spec.md` | 普通 Wiki 页面或 governance knowledge | 迁移输入 | 独立 change 决定保留、改写或删除 |
| legacy 命名入口 | `SpecWiki` / `spec-wiki` | 测试开发阶段不长期兼容 | 全仓用户可见入口无旧命名残留 |

legacy import dry-run：

```text
scan legacy paths
  -> classify importable / ignored / conflict
  -> produce migration report
  -> user confirm
  -> rewrite target pages / metadata
  -> verify no runtime writes legacy paths
```

删除旧兼容层前必须满足：

- 新 CLI 能完成 status、changes、change、validate。
- runtime validator 与旧 skill 判断一致。
- skill forwarding 不再内嵌状态机。
- archive dry-run 和 operation manifest 可用。
- fixtures 覆盖 parent / child / missing artifact / partial review / wiki-sync issue。
- 删除后 Agent / Skill 仍能通过 `spec-wiki` 一级命令完成流程。

## 迁移计划

### Phase 0a: 页面树合同决策

- 明确继续保留 `.wiki/pages/**`，还是迁移到唯一 `.wiki/**/*.md` 页面树。
- 同步 `.wiki/06-设计文档/01-Runtime设计.md`、`.wiki` 文档规范和 runtime init/rebuild 保护策略。
- 定义 managed projection、人工长期页和 metadata 的边界。
- 该阶段独立于 governance integration，不夹带实现。

### Phase 0b: Governance Runtime Contract

- 定义 governance DTO、状态枚举、blocking issue schema 和 runtime action 命名。
- 定义 `GovernanceEvidenceStore`、`GovernancePolicyEngine`、`ReviewGateContract`。
- 明确 SQLite 只存查询加速、artifact reference index、computed status，不存正式 evidence truth。

### Phase 1: Governance Model + Read-only Status

- 在 `wiki-model` 增加 governance DTO。
- 在 `wiki-runtime` 增加 `.spec` read-only scanner。
- 新增 `governance.status` runtime action。
- 覆盖 active changes、stage、missing artifacts、blocking issues。
- 不碰 archive，不迁移页面树，不展开完整 public CLI。

### Phase 2: Governance Validate

- 抽取 governance required artifact matrix。
- 实现 `governance.validate <change-id>`。
- 覆盖 report frontmatter、scope、full/pass、parent / child archive readiness。
- 引入 `ReviewGateContract` fixtures。

### Phase 3: Narrow CLI + Skill Forwarding

- 在 TS CLI 增加 `spec-wiki status`。
- 在 TS CLI 增加 `spec-wiki changes`、`spec-wiki change <change-id>` 和 `spec-wiki validate <change-id>`。
- 更新 `.agents` / `.codex` skills，让状态判断优先调用 runtime command。
- 测试宿主资产中不再内嵌治理状态机。
- 暂不开放 public archive。

### Phase 4: Governance Knowledge Integration

- 把 stable governance rule、capability baseline、change delta、wiki-sync issue 建模为 governance knowledge。
- 支持 governance query。
- 将 governance query 与 code/module/capability refs 关联。
- `.spec` 原文不导入 SQLite 作为 truth，只建立 artifact reference index 和 derived summary。

### Phase 5: Archive Runtime

- 迁移 archive workflow。
- 先实现 dry-run、readiness report 和 operation manifest。
- 再实现目录移动、parent / child 同步、archive evidence 检查。
- 输出 `wiki-updates-made / wiki-updates-required / wiki-updates-not-needed`。

### Phase 6: Remove Standalone Governance Entrypoints

- 新合同跑通后，删除独立 legacy governance CLI / wrapper / 重复状态机。
- 保留 skill 名称时，它们只作为 `spec-wiki changes / change / validate / archive` 的交互入口。
- 测试开发阶段不保留长期兼容 alias。

## 测试策略

### Rust

- `wiki-model`：governance DTO serialization / canonicalization。
- `wiki-runtime`：fixture repo 下 `.spec/changes/**` scanner、status、validate。
- parent / child / multi-change / standalone exploration stub fixture。
- report frontmatter fixture：pass / fail / partial / skipped / missing scope。
- ReviewGateContract fixture：scope、evidence、partial、blocked。
- scanner guard：`.spec` 不进入 code facts。
- governance artifact reference index 与 code facts 输入隔离。

### TS

- CLI 参数解析：`spec-wiki status`、`spec-wiki changes`、`spec-wiki change <change-id>` 与 `spec-wiki validate <change-id>`。
- forwarding payload：TS 不解释 validator，只透传 runtime action。
- host asset tests：skill 文案调用 governance CLI，不内嵌状态机。
- skill golden tests：legacy 判断与 runtime validate parity。

### System

- active change status 能列出 blocking issues。
- validate 能阻止缺失 required artifact 的 archive。
- governance query 能找到 change、capability 和 wiki-sync issue。
- archive dry-run 不移动目录，但能生成 readiness report 和 operation manifest。
- 页面树迁移前，runtime 不同时写两套 page truth。

## 风险

| 风险 | 后果 | 控制方式 |
| --- | --- | --- |
| 把 `.spec` 搬进 `.wiki` | 审计证据和知识投影混乱 | 明确 `.spec` 是 evidence truth |
| Skill 继续当内核 | 状态机多处漂移 | Runtime validator 是唯一合同 |
| 一开始做 archive | 破坏 active changes | 前三阶段只读，archive 先 dry-run |
| `.spec` 进入 source facts | 产生假代码知识 | scanner guard 测试 |
| governance cache 被当 truth | 状态恢复错误 | cache 只作加速，可重建 |
| capability baseline 与 change delta 混用 | 稳定规格被误改 | baseline / delta 分层建模 |
| runtime 对 `.spec` 文件布局硬编码 | evidence backend 无法演进 | `GovernanceEvidenceStore` 端口隔离 |
| public archive 误触发破坏性移动 | active/archive 撕裂 | readiness report、operation manifest、显式确认 |
| 新旧 validator 并行期间判断不一致 | Agent 输出互相冲突 | parity fixture 和 skill golden tests |
| `.wiki/pages/**` 与 `.wiki/**/*.md` 双 truth | 页面状态和 metadata 失配 | 独立页面树合同 change |
| 自动生成覆盖人工编辑 | 长期知识丢失 | 受控区块、conflict 标记、人工确认 |
| `INDEX.md` 膨胀 | 导航失效 | INDEX 只保留范围、索引、入口、来源 |
| 模块页和对外方法页职责重复 | 页面互相复制 | 栏目职责边界和链接规则 |

## 成功标准

### 产品成功标准

- 新用户能在 3 个命令内完成接入、刷新和查询：`init / update / query`。
- 用户不理解 `.spec` 内部结构，也能通过 `status` 知道当前 repo 是否 `ready / stale / blocked`。
- 没有启用 governance 的仓库，SpecWiki 仍能作为 repo wiki runtime 正常工作。
- Quick Start 只出现高频 public commands，不出现 expert/debug/runtime 维护菜单。
- Agent 能通过 `query` / `status` 获得稳定上下文，减少直接全仓扫描。

### 治理成功标准

- `spec-wiki status` 能读取当前 `.spec/changes/**` 并输出统一状态。
- `spec-wiki validate <change-id>` 的判断与治理 skill 规则一致。
- `.spec` artifact 不被 `.wiki` 吞并，也不进入 code facts。
- Skill 不再自己实现治理状态机，而是调用 `spec-wiki` 一级命令。
- Governance knowledge 能被 query 消费，但 `.spec` 原始证据仍保留原位。
- archive 前能阻止缺失 artifact、未通过 review 或 wiki-sync 未处理的 change。
- public archive 默认不会静默移动目录；移动前有 readiness report 和 operation manifest。
- archive 失败可定位到具体步骤，且不会产生不可解释的半归档状态。
- 新旧入口不长期分叉；legacy governance CLI 最终可删除。
- legacy `.wiki/pages/**` 不再被新 runtime 创建或写入。

## 建议的 Governance Parent Change

```text
integrate-spec-governance-into-spec-wiki-runtime
```

建议拆分：

```text
0a. unify-wiki-page-tree-runtime-contract
0b. governance-runtime-contract
1. governance-model-and-readonly-status
2. governance-validation-runtime
3. narrow-governance-cli-and-skill-forwarding
4. governance-knowledge-query-integration
5. governance-archive-runtime
6. remove-standalone-governance-entrypoints
```

## 结论

`.spec` governance 应完整融合为 SpecWiki 的治理产品层，但融合方式不是目录合并，而是产品层统一、runtime 底座复用、模型和状态机下沉。

最终形态应是：

```text
spec-wiki init
spec-wiki status
spec-wiki update
spec-wiki query <term>
spec-wiki validate [change-id]
spec-wiki archive <change-id>

spec-wiki sync
spec-wiki rebuild
spec-wiki changes
spec-wiki change <change-id>
spec-wiki doctor
spec-wiki repair
spec-wiki trace
```

第一阶段不追求完整最终形态，而是先做页面树合同决策、governance runtime contract、read-only status / validate 内核和窄 CLI。这样可以先把 SpecWiki 作为 repo-local knowledge runtime 立住，再把 governance 作为可验证、可查询、可归档的产品层纳入同一底座。
