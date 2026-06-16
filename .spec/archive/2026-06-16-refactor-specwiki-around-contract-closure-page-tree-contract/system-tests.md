# refactor-specwiki-around-contract-closure-page-tree-contract 系统测试用例

## 用例总览

本用例集覆盖页面树合同收口的系统级验收：`init / update / rebuild / restore / query` 只生成和消费正式可见页面树，metadata、state 与 SQLite FTS 只记录正式页面路径，`.wiki/pages/**` 不再具有新版 runtime 产品语义。验证重点是用户可见页面结构、运行时索引边界和文档 / fixture 口径一致性。

## 系统测试用例

### ST-001 init 只生成正式可见页面树

- 关联成功标准: `init / update / rebuild` 的 runtime 写入目标不再包含 `.wiki/pages/**`；文档、测试和 fixture 不再把 `.wiki/pages/**` 写作新版目标目录。
- 覆盖设计点: 正式页面树 predicate；`KnowledgeUnit.relative_path -> PlannedPage.relative_path` 作为唯一页面路径入口；根入口、栏目入口和叶子页路径合同。
- 前置条件: 使用最小 fixture 仓库，清空 `.wiki/` runtime 输出后执行初始化流程。
- 操作 / 触发: 运行当前 CLI 的 init 流程，并检查 `.wiki/` 输出。
- 期望结果: 生成 `.wiki/INDEX.md`、至少一个 `.wiki/<栏目路径>/INDEX.md` 和至少一个 `.wiki/<栏目路径>/NN-主题.md`；不生成 `.wiki/pages/**`；不把旧根级散页作为正式目标。
- 验证方式: 文件系统断言加 runtime 输出断言，检查 generated pages / metadata 中的路径均满足正式页面树 predicate。

### ST-002 metadata、state 与 SQLite FTS 只记录正式页面

- 关联成功标准: metadata、state、SQLite `wiki_pages / wiki_pages_fts` 只记录正式页面树；`restore / rebuild` 不从 `.wiki/pages/**` 重建正式 metadata 或 runtime state。
- 覆盖设计点: `WikiState.pages`、`wiki.metadata.json`、SQLite page rows 与 FTS rows 共用正式页面树过滤视图。
- 前置条件: 已完成 init 或 rebuild，并存在可读取的 `.wiki/wiki.metadata.json` 与 `.wiki/.cache/wiki-cache.db`。
- 操作 / 触发: 读取 metadata、runtime state 和 SQLite `wiki_pages / wiki_pages_fts`。
- 期望结果: 所有 page path 均为 `.wiki/INDEX.md`、`.wiki/<栏目路径>/INDEX.md` 或 `.wiki/<栏目路径>/NN-主题.md`；不包含 `.wiki/pages/**`、`.wiki/.knowledge/**`、`.wiki/.cache/**`、`.wiki/wiki.metadata.json` 或其它 runtime 隐藏产物。
- 验证方式: 结构化读取 metadata 和 SQLite 表，使用正式页面树 predicate 做全量断言。

### ST-003 query 默认只返回正式页面结果

- 关联成功标准: 默认 `query` 不读取 `.wiki/pages/**`，不返回旧目录内容作为正式结果。
- 覆盖设计点: query route、knowledge match projection ref、SQLite FTS 和 Markdown fallback 只消费正式页面。
- 前置条件: 初始化后的仓库包含正式页面；额外手动放置 `.wiki/pages/ignored.md`，其中包含唯一短语。
- 操作 / 触发: 对正式页面内容中的关键词执行 query；再对 `.wiki/pages/ignored.md` 中的唯一短语执行 query。
- 期望结果: 正式关键词查询可返回正式页面路径；旧目录唯一短语不以正式页面结果返回；query 输出中的 page refs 均满足正式页面树 predicate。
- 验证方式: CLI query 输出或 machine-readable query 结果断言，同时检查 fallback refs 不读取 runtime surface 外目录。

### ST-004 update / rebuild / restore 不消费 runtime surface 外目录

- 关联成功标准: `update` 不刷新 `.wiki/pages/**`，也不把旧目录纳入 `AffectedProjectionScope`；`restore / rebuild` 不从 `.wiki/pages/**` 重建正式 metadata 或 runtime state；`status` 不为 `.wiki/pages/**` 提供专门状态、诊断字段或清理建议。
- 覆盖设计点: update affected scope、rebuild state restore、cache artifact restore、status machine-readable 输出均只围绕正式页面树。
- 前置条件: 初始化后的仓库额外存在 `.wiki/pages/ignored.md`；正式页面和 knowledge artifact 均可被 update / rebuild / restore 使用。
- 操作 / 触发: 分别运行 update、rebuild、restore 相关流程和 status。
- 期望结果: update 的 changed / updated pages 与 `AffectedProjectionScope` 不包含 `.wiki/pages/**`；rebuild / restore 后 metadata、state 和 SQLite 不包含旧目录；status 不产生旧目录专用状态、诊断或清理建议。
- 验证方式: machine-readable 输出、metadata、SQLite 与文件修改时间 / affected scope 断言；旧目录文件存在也不影响正式 runtime 结果。

### ST-005 文档、测试和 fixture 口径收敛

- 关联成功标准: 文档、测试和 fixture 不再把 `.wiki/pages/**` 写作新版目标目录。
- 覆盖设计点: `.wiki/INDEX.md`、栏目 `INDEX.md`、`NN-主题.md` 是唯一正式可见页面树；`.wiki/pages/**` 只能作为 runtime surface 外目录或历史上下文出现。
- 前置条件: 完成实现和测试迁移。
- 操作 / 触发: 搜索 `.docs/`、`.wiki/`、`.spec/changes/**`、测试、fixtures 和脚本中的页面树描述。
- 期望结果: 不再存在把 `.wiki/pages/**` 描述为新版 runtime page projection、目标目录、query 来源、restore 来源或 status 分支的文本；相关测试断言均改为正式页面树。
- 验证方式: `rg` 结果人工复核加关键测试快照 / fixture 断言。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| `init / update / rebuild` 的 runtime 写入目标不再包含 `.wiki/pages/**` | ST-001, ST-004 | 文件系统、workflow 输出、affected scope 断言 |
| metadata、state、SQLite `wiki_pages / wiki_pages_fts` 只记录正式页面树 | ST-002 | metadata / state / SQLite 全量路径断言 |
| 默认 `query` 不读取 `.wiki/pages/**`，不返回旧目录内容作为正式结果 | ST-003 | query machine-readable 输出与 fallback refs 断言 |
| `restore / rebuild` 不从 `.wiki/pages/**` 重建正式 metadata 或 runtime state | ST-002, ST-004 | restore / rebuild 后 metadata、state、SQLite 断言 |
| `update` 不刷新 `.wiki/pages/**`，也不把旧目录纳入 `AffectedProjectionScope` | ST-004 | update 输出、affected scope 与文件修改断言 |
| `status` 不为 `.wiki/pages/**` 提供专门状态、诊断字段或清理建议 | ST-004 | status machine-readable 输出断言 |
| 文档、测试和 fixture 不再把 `.wiki/pages/**` 写作新版目标目录 | ST-005 | 搜索与人工复核 |

## 边界与异常

- planner 产出非正式页面路径时，本轮 workflow 应被阻断，不能由下游静默改名。
- `.wiki/pages/**` 文件存在时，系统不迁移、不清理、不诊断，也不把它作为 query / restore / update 输入。
- 正式页面树支持多级栏目，但正式页面只允许栏目 `INDEX.md` 和 `NN-主题.md` 两类叶子规则。
- page id 由 relative path 派生，本 change 接受路径变化导致的破坏性重建，不规划旧 page id 兼容。

## 验证数据与环境

- 最小 fixture 仓库：包含可生成项目总入口、栏目入口和主题页的源码 / knowledge 输入。
- 干扰文件：`.wiki/pages/ignored.md`，包含唯一短语，用于证明 query / update / restore 不消费旧目录。
- Runtime 产物：`.wiki/wiki.metadata.json`、`.wiki/.cache/wiki-cache.db`、workflow machine-readable 输出。
- 配置：本 change 不涉及浏览器 UI，不规划 Playwright 验证。

## 未覆盖项

无。

## 参考资料

- `proposal.md`
- `design.md`
- `.spec/changes/refactor-specwiki-around-contract-closure/split.md`
- `.docs/design/specwiki-contract-closure.md`
- `.docs/design/governance-runtime-integration.md`
- `.docs/design/knowledge-to-wiki-projection-contract.md`
