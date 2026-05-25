## 1. Runtime Contract

- [x] 1.1 对照 `tmp/reference-zh/meta/repowiki-metadata.json` 收敛第一阶段最小 metadata 字段集，并更新 `crates/wiki-core/src/domain/metadata.rs`
- [x] 1.2 调整 `storage/metadata_store` 与 `storage/cache_store`，明确 `.wiki/wiki.metadata.json` 和 `.wiki/.cache/` 的分层写入约束
- [x] 1.3 调整 `generation` 与 `storage/wiki_fs`，确保 `init`/`rebuild` 至少生成一个正式 Wiki 页面并稳定写入 `.wiki/`

## 2. Workflow Semantics

- [x] 2.1 收敛 `init` 行为：校验 Git 仓库、生成页面、写 metadata、建立 fresh 状态
- [x] 2.2 收敛 `status` 行为：区分 `fresh`、`stale`、`missing` 和需要重建的最小状态
- [x] 2.3 收敛 `update` 行为：在 stale 或 missing 时执行第一阶段允许的全量刷新，并使 runtime 回到 fresh
- [x] 2.4 收敛 `query` 行为：返回结构化结果，而不是仅返回非结构化文本或进程失败
- [x] 2.5 收敛 `sync` 与 `rebuild` 行为：同步外部 Wiki 修改、强制全量重建 runtime

## 3. CodeBuddy Agent Integration

- [x] 3.1 检查并收敛 `agents/codebuddy/src/runtime/*` 的 Windows binary 解析与 core 调用路径
- [x] 3.2 检查并收敛 `agents/codebuddy/src/tools/*`，保证 `wikiInit/wikiStatus/wikiUpdate/wikiQuery/wikiSync/wikiRebuild` 与 core action 一一对应
- [x] 3.3 收敛 Agent 返回值和错误透传，避免在 TS 层承载 Wiki 业务逻辑或静默改写状态

## 4. Verification

- [x] 4.1 为 `wiki-core` 增加覆盖 runtime contract 与六个工作流最小语义的测试
- [x] 4.2 为 CodeBuddy Agent 增加覆盖工具映射、binary 调用和错误透传的测试
- [x] 4.3 增加或更新 Windows 下的端到端验证，覆盖 `init -> status -> update -> query -> sync -> rebuild`
- [x] 4.4 更新仓库文档，说明第一阶段仅支持 Windows 和 CodeBuddy Agent，以及 `.wiki/` 运行产物约束
