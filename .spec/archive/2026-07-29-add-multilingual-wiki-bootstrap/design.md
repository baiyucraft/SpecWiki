# add-multilingual-wiki-bootstrap 设计方案

## 方案概述

SpecWiki Lite 使用 `.wiki/config.yaml` 选择 Wiki 语言。配置只决定正式 Wiki 的目录名、文件名和正文；CLI、JSON、`.spec`、Skill 标识保持英文。`init` 默认写入中文配置，也可通过 `--language en` 创建英文项目；`update` 只读取配置。

双语模板采用相同的信息架构和 ownership。根 `INDEX.md` 是带稳定 marker 的一次性初始化任务页，静态结构可健康，但 `ProjectStatusReport.ready` 在任务页未被替换前为 false。

## 配置设计

### Schema

```yaml
version: 1
wiki:
  language: zh
```

- `WikiLanguage` 闭集为 `zh | en`，默认 `zh`。
- 配置文件不存在时使用默认中文；同步成功后创建默认配置。
- 配置存在时必须是 YAML object，`version` 必须严格等于 `1`，`wiki.language` 必须属于闭集。
- `init --language` 是目标语言 override。配置已存在时通过 YAML document API 只更新 `wiki.language`，保留未知字段和注释；迁移失败时不写配置。
- `update` 不接受 language 参数，只读取配置。
- `.wiki/config.yaml` 不进入 Markdown inspector，也不写入 `PROJECT_ASSETS` ownership；它是用户配置，`--force` 不覆盖。

### CLI

```text
spec-wiki-lite init [path] [--host codex] [--language zh|en] [--force]
```

`ParsedCommand` 增加 optional language。该参数只允许用于 `init`；缺值或非法值返回 usage `64`。`runBootstrapInit` 接收 language 并传给资产同步。

## 模板与 Ownership

package 模板：

```text
assets/wiki/
  zh/**
  en/**
assets/migrations/wiki-en-v0/**
```

两种语言各登记十个当前 Wiki 文件：

```text
INDEX.md                                      scaffold
00-*/INDEX.md                                 scaffold
00-*/00-*SSOT*.md                             managed
00-*/01-*page-template*.md                    managed
00-*/02-*SpecWiki-Lite-workflow*.md           managed
01-*/INDEX.md                                 scaffold
02-*/INDEX.md                                 scaffold
02-*/00-*code-commenting*.md                  scaffold
03-*/INDEX.md                                 scaffold
04-*/INDEX.md                                 scaffold
```

Skills 八个文件保持 `skill` ownership。Registry 提供：

- `PROJECT_SKILL_NAMES`
- `projectAssetsForLanguage(language)`
- 当前语言资产及对应另一语言资产的稳定映射
- 旧 `e830627` 英文 scaffold registry，只用于迁移识别，不参与普通同步

普通同步规则不变：缺失即创建；scaffold 永不覆盖；managed 仅 `--force` 覆盖；skill 始终同步；未登记用户文件保留。

## 初始化任务页

中英文根模板都包含：

```html
<!-- spec-wiki-lite:bootstrap-pending -->
```

任务页要求 Codex：

1. 判断项目定位、技术栈和运行方式是否足够明确。
2. 按需读取 README、manifest、构建/测试/部署配置、源码入口和既有文档。
3. 用项目事实填充五个栏目，不适用的栏目说明原因。
4. 按需新增快速上手、模块和对外方法页面。
5. 将根任务页整体替换为正式首页，并移除 marker。

正式首页至少包含一级目录、SSOT、按任务导航、按模块/包导航和文档约定。模板正文借鉴 UniSpec 的任务分层，但全部按 SpecWiki Lite 产品边界重新撰写。

## Status 与 Skill 路由

`WikiInspectionReport` 增加：

```ts
language: "zh" | "en";
bootstrapPending: boolean;
```

- `inspectWiki` 读取并严格校验配置；缺失时返回默认 `zh`。
- `bootstrapPending` 只由根 `INDEX.md` 的稳定 marker 判定。
- `wiki.ready` 继续只表达 Markdown 静态健康。
- `ProjectStatusReport.ready` 额外要求 `bootstrapPending === false`。
- `status --json` 复用该结构，不新增另一份 locale 输出。
- `wiki-continue` 在 active changes 为零且 bootstrap pending 时路由 `wiki-explore`；有 active change 时继续按 stage 路由；多 active change仍暂停选择。

## 语言迁移

### 来源识别

同步前根据配置和登记路径识别当前资产来源：

- 配置存在且对应语言路径完整：普通同语言同步。
- 配置语言与磁盘另一语言登记资产不一致：执行另一语言到目标语言迁移。
- 配置缺失且存在旧 `e830627` 英文路径：来源为 `legacy-en-v0`，目标为默认 `zh`。
- 同时存在互相冲突的中英文登记资产：迁移失败并列出路径。

### 预检

对每个来源资产：

- scaffold 只有内容与对应 package 模板逐字一致时可删除/替换；否则冲突。
- managed 内容一致时可迁移；内容不一致仅在 `--force` 时可迁移。
- root `INDEX.md` 与语言专属页面遵循相同规则。
- 未登记文件不参与迁移，也不删除或翻译。
- 目标不存在或内容与目标模板一致时可写；目标存在不同内容时冲突。
- 所有源、目标和配置路径先经过 lexical + realpath containment。

任何冲突都在第一次写入前失败。执行阶段记录所有受影响文件的原内容/缺失状态；写入或删除失败时反向恢复，避免命令造成部分迁移。成功后清理空的旧语言目录，再写入配置 override 或默认配置。

### 旧版升级

`assets/migrations/wiki-en-v0/**` 保存提交 `e830627` 的 11 个英文 scaffold 原文，仅用于内容比对。旧页面未修改时迁移到新的中文五栏目；任何修改都 fail closed。用户若希望保留旧英文，先创建：

```yaml
version: 1
wiki:
  language: en
```

随后 update 安装新的英文信息架构。若旧页面仍等于 migration baseline，则完成受控迁移；若旧页面已修改，则把旧路径视为用户内容保留，不再删除或覆盖，同时创建不冲突的当前英文资产。

## Wiki 与配置落点

当前 SpecWiki Lite 仓库增加 `.wiki/config.yaml`，值为 `zh`。长期 Wiki 更新：

- 总体设计与 Lite 内核：配置、locale registry、bootstrap readiness 和迁移。
- CLI / 配置合同：`--language`、默认中文和 `.wiki/config.yaml`。
- assets/wiki 模块与 capability：双语 ownership、失败关闭迁移。
- 快速上手、README、发布合同：中文默认与英文显式示例。

历史 `.spec/archive/**` 保持只读。

## 参考边界

- 来源：本机 `E:/project/!byAI/UniSpec`，package `@uni-sw/unispec@0.1.0`，验证命令 `unispec init project --tools codex`。
- 目标落点：Lite 根 bootstrap 任务页、五栏目信息架构、status readiness 和 `wiki-continue` 路由。
- 采用方式：行为结构改写；不复制 UniSpec 源码、模板正文、产品名、tool 选择或其 `.spec/config.yaml`。
- 原 SpecWiki `origin/main` 的 `.wiki/config.yaml` 只作为配置职责参考；不迁移 scan/LLM/runtime schema。

## 回滚

- 普通同步沿用单文件原子写。
- 迁移在内存中完成全量预检，并对已触碰文件保留 rollback snapshot。
- rollback 不修改未登记页面。
- archive 前验证中文当前仓库、中文临时项目、英文临时项目和旧英文迁移 fixture。
