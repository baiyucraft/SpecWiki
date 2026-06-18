# refactor-specwiki-around-contract-closure-page-tree-contract 设计方案

## 方案概述

本 change 只收口“页面树真相”这一层，并且必须把页面路径规划纳入合同。目标是让 SpecWiki 的正式可见页面树统一落在 `.wiki/INDEX.md`、栏目 `INDEX.md`、`NN-主题.md` 这条线上。`.wiki/pages/**` 不属于新版 SpecWiki runtime surface；系统不迁移、不清理、不诊断，也不把它作为 query / restore / update 的输入。

实现上，路径生成唯一入口必须是 `KnowledgeUnit.relative_path`。`PlannedPage`、`WikiState`、`wiki.metadata.json`、SQLite `wiki_pages / wiki_pages_fts` 和 query fallback 只消费这条链产出的正式路径，不再各自修正路径。runtime 的主链只处理正式页面树，runtime surface 外的文件自然被忽略。

### 需求背景引用

- 业务现状：当前代码与文档里同时存在 `.wiki/pages/**` 和新的可见页面树口径。
- 驱动因素：后续 truth / restore / projection / query / governance / CLI 变更都需要单一页面树前提。
- 痛点 / 机会：如果不先收口页面树，metadata、query、update 和 rebuild 仍可能重新接受非正式路径。

### 方案目标

- 将可见页面树确认为唯一正式 runtime 目标。
- 让 `.wiki/pages/**` 退出 runtime surface，不再承担任何产品语义。
- 让 metadata、state、SQLite FTS、query fallback、restore 都只消费正式页面树。

### 方案范围

- 覆盖范围：runtime 页面写入、查询兜底、metadata 导出、cache 恢复、文档与测试口径。
- 边界说明：不实现 truth kind、两级 restore、projection ownership 重构、query DTO 完整化或 code graph 重构。
- 设计边界：本 change 不做旧目录清理、迁移、诊断，也不新增旧目录专用 CLI 或状态字段。

### 核心设计思路

引入“正式页面树 predicate”作为内部合同：planner 必须生成正式可见页面树路径；runtime 只把符合 predicate 的页面纳入 state、metadata、SQLite FTS、query fallback 和 restore。`.wiki/pages/**` 和 `.wiki/.knowledge/**`、`.wiki/.cache/**` 一样，属于 runtime 主页面树之外的目录。

正式页面 predicate 固定为：

- `.wiki/INDEX.md`
- `.wiki/<栏目路径>/INDEX.md`
- `.wiki/<栏目路径>/NN-主题.md`

其中 `<栏目路径>` 可以是一层或多层目录；`NN-主题.md` 的 `NN` 是稳定排序前缀。以下路径明确排除在正式页面树之外：`.wiki/pages/**`、`.wiki/.knowledge/**`、`.wiki/.cache/**`、`.wiki/wiki.metadata.json` 和其它 runtime 隐藏产物。

## 架构分析

### 现有架构概述

| 架构层级 | 组件 / 模块 | 说明 |
| --- | --- | --- |
| 规划层 | `crates/wiki-knowledge/src/planning.rs`、`crates/wiki-knowledge/src/projection.rs` | 由 `KnowledgeUnit.relative_path` 规划 `PlannedPage.relative_path` |
| 写入层 | `crates/wiki-runtime/src/workflows/init.rs` / `update.rs` / `rebuild.rs` | 直接按 `PlannedPage.relative_path` 写页面与 metadata |
| 状态层 | `crates/wiki-runtime/src/domain/state.rs`、`metadata_mapper.rs` | `WikiState.pages` 与 `wiki.metadata.json` 绑定页面真相 |
| 查询层 | `crates/wiki-runtime/src/workflows/query.rs` | 既读知识摘要，也会对 `state.pages` 做 Markdown fallback |
| 文件层 | `crates/wiki-runtime/src/storage/wiki_fs.rs` | 负责页面路径解析、落盘与清理 |

### 方案与现有架构的关系

| 维度 | 说明 |
| --- | --- |
| 复用模块 | `PlannedPage`、`WikiState`、`WikiMetadata`、SQLite 状态表、现有 init/update/rebuild/query 主链 |
| 新增组件 | 正式页面树 predicate、可见页面树过滤视图 |
| 改造模块 | `wiki-knowledge` 路径规划、`wiki_fs`、`init/update/rebuild/query/state/metadata_mapper/knowledge_artifacts/change_set/sync` |
| 技术栈 | 不变，继续用现有 Rust runtime 与 SQLite cache |

### 依赖关系

| 依赖项 | 类型 | 用途 | 来源 / 文档 | 备注 |
| --- | --- | --- | --- | --- |
| `.wiki/INDEX.md` / 栏目 `INDEX.md` / `NN-主题.md` | 文档契约 | 可见页面树目标 | `.wiki/00-文档约定/01-页面模板.md`、`.wiki/INDEX.md` | 正式目标 |
| `KnowledgeUnit.relative_path` | 规划源头 | 唯一页面路径生成入口 | `crates/wiki-knowledge/src/planning.rs` | 必须产出正式页面树 |
| `PlannedPage.relative_path` | 规划结果 | 页面路径输入 | `crates/wiki-knowledge/src/projection.rs` | 只透传正式路径 |
| `WikiState.pages` | runtime state | 页面状态集合 | `crates/wiki-runtime/src/domain/state.rs` | 只保存正式页面 |
| `WikiMetadata` | 外部导出 | metadata 绑定索引 | `crates/wiki-runtime/src/domain/metadata_mapper.rs` | 只导出正式页面 |
| `query` fallback | 查询路径 | Markdown 兜底 | `crates/wiki-runtime/src/workflows/query.rs` | 默认只看正式页面 |

## 功能设计

### 功能模块划分

| 模块名称 | 功能描述 | 优先级 | 依赖模块 | 对应 proposal 内容 |
| --- | --- | --- | --- | --- |
| 页面路径规划 | 让 `KnowledgeUnit.relative_path` 直接产出正式页面树路径 | P0 | `wiki-knowledge` planning / projection | 统一新版用户可见页面树 |
| 页面树 predicate | 判断某个页面路径是否属于正式可见页面树 | P0 | `wiki_fs`、`PlannedPage` | 统一新版用户可见页面树 |
| 写入路由 | 让 init / update / rebuild 只写正式页面树 | P0 | `wiki_fs`、`page_render` | 停止写 `.wiki/pages/**` |
| 查询过滤 | 默认 query 只从正式页面树做 Markdown fallback | P0 | `query`、`state`、`metadata` | 默认 query 不读取旧目录 |
| 元数据恢复 | 从 metadata / cache 恢复时只接受正式页面树 | P0 | `state_store`、`knowledge_artifacts` | 为后续 truth / restore 提供稳定前提 |
| 同步过滤 | sync 仅同步正式页面树 | P1 | `sync`、`wiki_fs` | 避免非正式路径进入主链 |

### 功能详细设计

#### 页面树 predicate

- 功能说明：在 runtime 内部引入正式页面树判断，不再默认把“能读到的 `.wiki/*.md`”都视为正式页面。
- 前置条件处理：先判断路径是否匹配 `.wiki/INDEX.md`、`.wiki/<栏目路径>/INDEX.md` 或 `.wiki/<栏目路径>/NN-主题.md`。
- 业务逻辑实现：只有正式页面参与主链；不符合 predicate 的路径不进入 state、metadata、FTS、query fallback 或 restore。
- 输出结果生成：对外只输出正式页面路径。
- 异常场景处理：若 planner 产出非正式路径，按新版实现 bug 阻断本轮 workflow。

#### 页面路径规划

- 功能说明：`KnowledgeUnit.relative_path` 是路径生成唯一入口，必须直接生成正式页面树路径。
- 前置条件处理：规划阶段根据页面类型和栏目路径生成 `.wiki/INDEX.md`、栏目 `INDEX.md` 或 `NN-主题.md`。
- 业务逻辑实现：`PlannedPage.id = stable_id("page", relative_path)` 仍沿用现有规则，因此路径变化会导致 page_id 变化。
- 输出结果生成：`PlannedPage.relative_path` 不再出现 `pages/` 前缀，也不输出旧的根级散页结构。
- 异常场景处理：若规划结果不是正式 predicate，阻断本轮 workflow，而不是在 runtime 下游静默改名。

#### 写入路由

- 功能说明：`init / update / rebuild` 的页面落盘目标统一走正式可见页面树。
- 前置条件处理：在写入前用 predicate 校验 `PlannedPage.relative_path`。
- 业务逻辑实现：正式路径写入 `.wiki/` 下的目标文件；非正式路径不写盘，也不进入页面刷新集合。
- 输出结果生成：`generated_pages / updated_pages` 只统计正式页面。
- 异常场景处理：若 planner 产出非正式目标，直接视为设计违规并阻断本轮输出。

#### 查询过滤

- 功能说明：默认 query 只消费正式页面，不把 runtime surface 外的 Markdown 作为正式答案来源。
- 前置条件处理：从 `WikiState` 与 metadata 生成正式页面索引时只保留正式页面。
- 业务逻辑实现：知识摘要、索引命中、graph 命中保持现有逻辑；Markdown fallback 仅用于正式页面。
- 输出结果生成：默认结果只返回正式页面。
- 异常场景处理：如果没有正式页面命中，按普通 query miss 处理，不为旧目录提供特殊结果。

过滤点至少覆盖 `load_or_rebuild_state()` 从 metadata 重建出来的 state、SQLite `wiki_pages_fts` 的页面索引、`collect_page_fallback_matches()` 的 Markdown fallback，以及 knowledge match 中对 `projection_ref` 的页面路径回填。

#### 元数据恢复

- 功能说明：`wiki.metadata.json` 只保存正式页面树的绑定索引。
- 前置条件处理：加载 metadata 或重建 state 时只接收正式页面 path。
- 业务逻辑实现：正式页面参与恢复与一致性校验；非正式路径不参与恢复。
- 输出结果生成：恢复后的可消费 runtime 只依赖正式页面树。
- 异常场景处理：若 metadata 中没有可用的正式页面，按普通 missing / incomplete runtime 处理，不提供旧目录特殊分支。

### 处理流程

```text
planned_page.relative_path
  -> 正式页面树 predicate
  -> 正式页面
       -> init / update / rebuild 写盘
       -> metadata 导出
       -> SQLite FTS
       -> query 默认消费
  -> 非正式路径
       -> 不进入 runtime page surface
```

### 业务规则实现

| 规则 | 实现方式 | 对应 proposal 内容 |
| --- | --- | --- |
| 正式页面树唯一 | 所有主流程只接受正式可见页面路径 | 统一新版用户可见 Wiki 页面树 |
| 旧目录无产品语义 | `.wiki/pages/**` 不参与写入、查询、恢复、状态诊断或清理 | 不考虑历史兼容 |
| metadata 不双真相 | `wiki.metadata.json` 只导出正式页面绑定 | 避免旧路径和新路径并存为正式 truth |

### 异常处理设计

| 异常场景 | 异常类型 | 处理策略 | 用户提示 / 系统行为 |
| --- | --- | --- | --- |
| planner 产出非正式路径 | 设计违规 | 直接阻断该轮流程 | 报告路径不符合页面树契约 |
| metadata 没有正式页面 | runtime 不完整 | 按普通 missing / incomplete 处理 | 建议重新 init / rebuild |
| query 无正式页面命中 | 查询未命中 | 不读取 runtime surface 外目录 | 返回普通空结果或 fallback 状态 |

## 数据设计

本 change 不引入新的持久化 schema。页面路径语义从“写了什么字符串”变成“是否属于正式页面树”，但落盘数据仍沿用现有 `WikiState.pages`、`WikiMetadata.wiki_items`、SQLite `wiki_pages.path` 和页面 Markdown 文件本身。

### 数据模型设计

| 字段 / 实体 | 类型 | 是否必填 | 业务含义 | 数据约束 | 对应 proposal 内容 |
| --- | --- | --- | --- | --- | --- |
| `page.path` | string | 是 | 页面磁盘路径 | 正式页面必须落在可见页面树 | 收口页面树 |
| `WikiMetadata.wiki_items[].path` | string | 是 | metadata 绑定路径 | 只导出正式页面路径 | 不把非正式路径当正式 truth |
| `wiki_pages.path` | string | 是 | SQLite 页面索引路径 | 只保存正式页面路径的运行时视图 | query / restore 一致性 |

### 数据流向设计

`PlannedPage.relative_path` 进入正式页面树 predicate 后，正式路径才会流向 `write_page`、`WikiState`、`WikiMetadata` 和 SQLite 索引；非正式路径不进入默认 query 与正式 metadata。

路径变化是破坏性重建：因为 page id 由 `stable_id("page", relative_path)` 派生，迁到新页面树会改变 page id、cache key、source/page map、FTS rows 和 affected page diff。本 change 不承诺无缝增量保留旧 page id，验收应以 rebuild / metadata regeneration 为准。

### 存储方案

| 数据类型 | 存储介质 | 存储位置 | 索引 / 查询策略 | 保留策略 |
| --- | --- | --- | --- | --- |
| 正式页面 Markdown | 文件 | `.wiki/**`（不含 runtime 隐藏目录） | 按现有页面路径读取 | 长期保留 |
| metadata | 文件 | `.wiki/wiki.metadata.json` | 由 `MetadataMapper` 导出 | 只保留正式页面绑定 |
| runtime state | SQLite / cache | `.wiki/.cache/wiki-cache.db` | 继续按现有索引读取 | 可重建 |

### 数据迁移方案

无数据迁移方案。本 change 不迁移旧目录，不清理旧文件，也不提供旧目录导入路径。

## 接口设计

### 接口概览

| 接口名称 | 接口类型 | 方向 / 方法 | 所属模块 | 对应功能 / 集成需求 |
| --- | --- | --- | --- | --- |
| 页面树 predicate | 内部函数 | 读 / 校验 | `storage/wiki_fs.rs` 或相邻 domain 模块 | 分类正式页面路径 |
| query 正式页过滤 | 内部逻辑 | query 流程 | `workflows/query.rs` | 默认只读正式页面 |

### 接口详细定义

#### 页面树 predicate

- 接口路径 / 命令 / 事件名：内部路径判断函数
- 接口描述：判断页面路径是否为正式页面树成员。
- 所属模块：`crates/wiki-runtime/src/storage/wiki_fs.rs` 及其相邻 domain 辅助模块
- 请求 / 输入参数：页面路径字符串。
- 响应 / 输出结构：布尔结果或正式路径分类。
- 错误码 / 异常定义：planner 产出的非正式路径视为实现错误。

## 非功能性设计

### 可靠性设计

| 可靠性要求 | 需求描述 | 实现方案 |
| --- | --- | --- |
| 单一 truth | 默认结果只能来自正式页面树 | 在 query / metadata / restore 之前先做路径 predicate |
| 无兼容分支 | 旧目录不能重新形成产品入口 | 不设计旧目录 status、query、restore 或 migrate 分支 |

### 可维护性设计

| 设计维度 | 实现方案 |
| --- | --- |
| 日志规范 | 只记录正式页面树规划和写入错误 |
| 配置管理 | 不新增公开配置项，先用内部契约收口 |
| 版本兼容 | 不保留旧兼容写入层，不设计旧目录迁移 |

### 兼容性设计

当前处于测试开发阶段，不考虑历史兼容性。旧目录不进入新版 runtime surface。

## 资源评估

无新增资源要求。页面路径 predicate 只是现有状态与文件路径上的规则判断，不引入新数据库或额外索引。

## 风险与对策

### 技术风险

| 风险描述 | 影响程度 | 发生概率 | 应对策略 | 预留方案 |
| --- | --- | --- | --- | --- |
| metadata / state 继续接受非正式路径 | 高 | 中 | 在路径生成、metadata 导出、state rebuild 和 query fallback 前统一 predicate | 直接修复路径生成入口 |
| query fallback 误读 runtime surface 外 Markdown | 高 | 中 | 默认只读正式页面树 | 无例外路由 |
| page_id 因路径变化全量改变 | 高 | 高 | 明确这是破坏性重建，验收基于新版 metadata | 不做旧 ID 兼容 |

### 业务风险

| 风险描述 | 影响程度 | 发生概率 | 应对策略 | 利益相关者沟通 |
| --- | --- | --- | --- | --- |
| 旧测试/fixture 仍依赖 `.wiki/pages/**` | 中 | 高 | 统一改为新版正式页面树断言 | 在设计与测试说明中同步 |
| 旧文档描述与新页面树不一致 | 中 | 高 | 同步修正文档引用 | 在 change 范围内更新稳定文档 |

## 设计决策

- `.wiki/pages/**` 不属于新版 runtime surface，不迁移、不清理、不诊断、不查询。
- 正式 page predicate 只允许 `.wiki/INDEX.md`、`.wiki/<栏目路径>/INDEX.md`、`.wiki/<栏目路径>/NN-主题.md`，并排除 `.wiki/pages/**`、`.wiki/.knowledge/**`、`.wiki/.cache/**`。
- `KnowledgeUnit.relative_path` 是路径生成唯一入口；下游不得各自修路径。
- 本 change 不新增持久化 schema。
- 默认 query、metadata、update、rebuild 和 restore 都必须以正式可见页面树为准。
- 由于 page id 由 relative path 派生，本 change 接受 page_id 破坏性变化，要求通过 rebuild / metadata regeneration 收敛。
- 正式页面树支持多级栏目，但每级只允许 `INDEX.md` 和 `NN-主题.md` 两类页面。

## 待确认问题

无。

## 参考资料

- `.spec/changes/refactor-specwiki-around-contract-closure-page-tree-contract/proposal.md`
- `.spec/changes/refactor-specwiki-around-contract-closure/split.md`
- `.docs/design/specwiki-contract-closure.md`
- `.docs/design/governance-runtime-integration.md`
- `.docs/design/knowledge-to-wiki-projection-contract.md`
- `crates/wiki-knowledge/src/projection.rs`
- `crates/wiki-runtime/src/workflows/init.rs`
- `crates/wiki-runtime/src/workflows/update.rs`
- `crates/wiki-runtime/src/workflows/rebuild.rs`
- `crates/wiki-runtime/src/workflows/query.rs`
- `crates/wiki-runtime/src/storage/knowledge_artifacts.rs`
- `crates/wiki-runtime/src/domain/metadata_mapper.rs`
