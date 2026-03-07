# CodeBuddy Repo Wiki Design

## Goal

构建一个供 CodeBuddy 使用的 Repo Wiki 插件，模仿 Qoder Repo Wiki 的核心功能：

- 在任意 Git 仓库中手动初始化 Wiki
- 将 Wiki 产物写入目标项目的 `.wiki/`
- 在后续会话中自动检测代码变化并增量更新 Wiki
- 允许 agent 在回答项目结构、模块职责、依赖关系、核心流程等问题前读取和更新 Wiki

第一版约束：

- 只支持中文
- 只优先实现 CodeBuddy 注入方式
- 只做功能等价，不追求 Qoder 的格式或索引字段完全一致
- `.wiki/` 和 `.wiki/wiki.metadata.json` 上库
- `.wiki/.cache/` 不上库

## Non-Goals

- 不实现多语言 Wiki
- 不实现 Web UI
- 不做云端索引服务
- 不复刻 Qoder/Lingma 的产品外壳或私有元数据字段
- 不要求用户安装 Rust 工具链

## Reference Inputs

本设计参考以下本地样本与官方文档：

- `参考/zh/content/`
- `参考/zh/meta/repowiki-metadata.json`
- Qoder Repo Wiki 文档
- 灵码 Repo Wiki 文档

参考样本说明了两件事：

1. Wiki 正式产物是树状 Markdown 内容
2. 索引文件负责描述页面、层级关系、更新水位和回溯信息

但这些样本只作为形态参考。第一版会根据不同语言、不同项目类型的实际结构动态生成 Wiki 页面，而不是硬编码生成固定目录树。

## Architecture

整体架构拆成两层：

### 1. Rust Core

源码路径：`crates/wiki-core`

负责所有核心工具动作和仓库分析逻辑：

- `init`
- `status`
- `update`
- `query`
- `sync`
- `rebuild`

其职责包括：

- 仓库扫描
- Git 和工作区变更检测
- 文件摘要与缓存
- 模块/依赖/流程抽取
- Wiki 页面规划与 Markdown 生成
- 元数据维护
- Markdown 与索引同步

Rust core 同时提供：

- 可执行入口：供 Node adapter 通过子进程调用
- 可复用库入口：便于后续接入其他 CLI / IDE

### 2. CodeBuddy Adapter

源码路径：`packages/codebuddy`

职责明确限制为适配层：

- 向 CodeBuddy 注入工具动作
- 解析调用参数
- 定位本机 Rust 二进制
- 调用 Rust core 并处理 JSON 结果
- 设计会话开始检查与按需更新的触发方式

适配层不负责仓库分析和 Wiki 业务规则。

## Source Repository Layout

源码仓库采用精简结构，只保留长期维护所需目录：

```text
codebuddy-wiki/
├─ Cargo.toml
├─ package.json
├─ pnpm-workspace.yaml
├─ README.md
├─ docs/
│  └─ plans/
├─ crates/
│  └─ wiki-core/
├─ packages/
│  └─ codebuddy/
├─ scripts/
└─ 参考/
```

不在源码仓库中维护平台发布包目录。平台 npm 子包在发布时由脚本临时生成。

## Target Wiki Layout

插件运行在目标项目时，产物写入目标仓库的 `.wiki/`：

```text
.wiki/
├─ 项目概述.md
├─ wiki.metadata.json
├─ 系统架构/
├─ 核心功能模块/
├─ 前端开发指南/
├─ 后端开发指南/
├─ API接口文档/
├─ 数据库设计/
├─ 部署与运维/
├─ 开发规范/
├─ 扩展与定制/
└─ .cache/
```

说明：

- `.wiki/*.md` 是正式 Wiki 页面，允许提交到 Git
- `.wiki/wiki.metadata.json` 是正式索引，允许提交到 Git
- `.wiki/.cache/` 存放摘要、图谱快照、脏页状态等运行时中间产物，不提交到 Git

页面集合是动态的：

- `项目概述.md` 固定生成
- 其余页面根据项目特征探测器决定是否生成

## Metadata Contract

`wiki.metadata.json` 的目标是“功能兼容”，而不是 Qoder 字段复刻。

第一版建议结构：

```json
{
  "schema_version": "1",
  "language": "zh",
  "repo_root": "/abs/path/to/repo",
  "branch": "main",
  "generated_at": "2026-03-07T00:00:00Z",
  "last_indexed_commit": "abc123",
  "wiki_items": [],
  "relations": [],
  "source_files": [],
  "dirty_state": {
    "status": "fresh",
    "dirty_pages": [],
    "dirty_sources": []
  }
}
```

核心字段职责：

- `wiki_items`
  - 每个页面或节点的 `id/title/path/type/parent_id/source_files/content_hash`
- `relations`
  - 记录 `PARENT_CHILD`、`DEPENDS_ON`、`CALLS`、`IMPLEMENTS` 等关系
- `source_files`
  - 记录每个源码文件的相对路径、摘要、上次分析时间、关联页面
- `dirty_state`
  - 记录当前是否过期、哪些源文件变化、哪些页面需要重建

## Core Workflows

### `init`

- 验证当前目录是 Git 仓库
- 扫描仓库结构、文件类型、符号、依赖和关键入口
- 在 `.wiki/.cache/` 建立缓存和中间图谱
- 生成页面规划
- 渲染 Markdown 页面
- 写入 `wiki.metadata.json`

### `status`

用于会话开始时的轻量检查：

- `HEAD` 是否变化
- 工作区是否存在未索引变更
- `.wiki/` 正式文件是否缺失
- `.wiki/*.md` 是否存在用户手工修改

返回：

- `fresh`
- `stale`
- `needs_rebuild`
- `missing`

### `update`

增量更新流程：

1. 识别代码变更文件集合
2. 从 `source_files` 和 `relations` 回溯受影响页面
3. 局部重建页面及必要的父级摘要页
4. 更新元数据中的摘要与脏页状态

### `query`

供 agent 检索结构化上下文：

- 输入用户问题或主题
- 匹配相关页面、关系、源文件
- 输出结构化结果而不是整页 Markdown

### `sync`

当用户直接修改 `.wiki/*.md` 时：

- 重新解析页面结构
- 更新索引中的摘要、章节哈希和关系
- 保留用户编辑内容

### `rebuild`

在以下场景触发：

- 索引损坏
- schema 升级
- 页面缺失严重
- 用户显式要求全量重建

## Update Strategy

第一版性能目标不依赖“换 Rust 自动变快”，而依赖增量架构：

- 文件级摘要缓存
- Git/worktree 变更检测
- 源文件到页面的映射
- 页面级脏区重建

自动触发采用双触发模型：

1. 会话开始先执行 `status`
2. 若状态为 `stale`，则执行 `update`
3. 当 agent 准备回答仓库结构、模块职责、流程、依赖问题前，再次检查 `status`
4. 仅在相关页面已脏时执行 `update`

## Page Generation Rules

页面按四类生成：

### 1. 总览页

固定生成：

- `项目概述.md`

内容包括：

- 项目简介
- 目录结构
- 核心组件
- 数据流/调用流概览
- 关键依赖
- 主要入口

### 2. 架构页

当项目存在明显分层、服务边界或基础设施配置时生成，例如：

- `系统架构/整体架构设计.md`
- `系统架构/数据流设计.md`

### 3. 模块页

按目录边界、导入关系、接口暴露和业务聚类生成，例如：

- `核心功能模块/<模块名>/<模块名>.md`

### 4. 专题页

根据仓库特征生成，例如：

- Web 项目偏向 `前端开发指南/`、`API接口文档/`
- 后端服务偏向 `数据库设计/`、`部署与运维/`
- 工具库项目偏向 `核心功能模块/`、`扩展与定制/`

## Managed Markdown Strategy

为兼顾自动生成和用户手改，页面采用“分节受管控”的方式。

原则：

- 页面是普通 Markdown
- 自动生成章节使用隐藏 HTML 注释锚点标记
- 更新时按章节块替换，而不是整页覆盖
- 非受控内容原样保留

这样可以满足：

- 页面可读
- 用户可直接修改 `.wiki/*.md`
- 自动更新不会无差别覆盖整页

## CodeBuddy Integration

第一版适配的工具动作：

- `wiki_init`
- `wiki_status`
- `wiki_update`
- `wiki_query`
- `wiki_sync`
- `wiki_rebuild`

集成行为：

- 会话开始调用 `wiki_status`
- 如果过期则按需 `wiki_update`
- 在 agent 需要项目知识前调用 `wiki_query`
- 回答时优先使用 Wiki 页面和索引关系作为上下文

## Packaging And Distribution

源码仓库只保留：

- `crates/wiki-core`
- `packages/codebuddy`

发布模型采用：

- 用户安装一个 npm 包
- npm 包内的 Node adapter 调用预编译 Rust 二进制
- 平台二进制 npm 子包在发布时由脚本临时生成并发布
- 用户不需要安装 Rust

推荐参考 `esbuild` 式分发策略：

- 主包：`codebuddy-wiki`
- 平台包：`codebuddy-wiki-<platform>`
- 平台包通过 `optionalDependencies` 参与安装

Node 与 Rust 的交互方式在第一版使用：

- `child_process.spawn`
- `stdin/stdout` JSON 请求响应

这是第一版最稳的接口，不引入 N-API 的额外复杂度。

## Error Handling

第一版明确的降级规则：

- 非 Git 仓库：拒绝初始化
- 仓库过大：允许先生成核心页面并标记 `partial`
- 无法识别技术栈：仍生成 `项目概述.md` 和基础模块页
- 元数据损坏：要求 `rebuild`
- 用户手改冲突：保留用户块并在元数据中标记冲突

## Testing Scope

第一版至少覆盖：

- `init` 能生成 `.wiki/` 与 `wiki.metadata.json`
- 单文件修改只触发受影响页面更新
- 手改 Markdown 后 `sync` 能刷新索引
- 删除和重命名源码文件后能清理孤儿关系
- 不同类型仓库生成不同页面集合
- CodeBuddy adapter 能正确调用 Rust core 并处理失败场景

## Risks

- 不同语言和框架的结构差异很大，模块聚类规则可能需要逐步收敛
- 第一版如果缺少稳定的解析器，依赖关系和流程分析会先偏启发式
- Markdown 分节同步必须设计稳定，否则容易误伤用户改动
- npm 平台包发布脚本需要一次性设计好，避免后续维护成本过高

## Open Decisions Resolved

已确认的关键决策：

- 使用 Rust 实现 core
- 使用 CodeBuddy adapter 作为首个宿主接入
- 源码仓库仅保留 `crates/wiki-core` 与 `packages/codebuddy`
- 平台 npm 包通过发布脚本临时生成
- Wiki 产物写入目标仓库 `.wiki/`
- `.wiki/.cache/` 不上库
- 第一版仅支持中文

## Implementation Handoff

下一步进入 implementation plan，按以下顺序执行：

1. 建立精简 monorepo 骨架
2. 实现 Rust core 的命令和数据模型
3. 实现扫描、索引、生成、增量更新
4. 实现 CodeBuddy adapter
5. 实现 npm 发布与跨平台二进制分发
