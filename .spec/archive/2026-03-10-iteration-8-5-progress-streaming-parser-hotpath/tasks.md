## 1. 协议与 Agent 进度流

- [x] 1.1 扩展 `CoreCommand` / transport 事件模型，为长流程定义 `progress/result/error` NDJSON 事件流，并让 Agent/脚本统一按新协议消费
- [x] 1.2 在 `main.rs`、`transport/json_rpc.rs`、`transport/cli.rs` 中接入 progress sink / writer，让 `init`、`update`、`rebuild` 统一输出事件流
- [x] 1.3 重构 `agents/codebuddy/src/runtime/invokeCore.ts` 与结果解析逻辑，支持逐行消费 progress 事件并继续返回最终结果
- [x] 1.4 为 core transport 和 Agent IPC 增加流式协议测试，覆盖成功、失败和终态唯一性

## 2. Workflow 阶段化进度

- [x] 2.1 为 `init` 主链补齐稳定 phase 上报，至少覆盖 `scan`、`parse_symbols`、`resolve_symbol_graph`、`analyze_symbol_graph`、`build_module_tree`、`build_contexts`、`plan_pages`、`render_pages`、`write_state`、`write_metadata`
- [x] 2.2 为 `update` 和 `rebuild` 补齐同口径 phase 上报，并把 `update` 回退到 `init/rebuild` 的真实执行路径也暴露为可观测进度
- [x] 2.3 为文件解析、页面渲染等有明确工作量的阶段补齐 `processed / total / elapsed_ms` 计数逻辑
- [x] 2.4 增加 workflow 级测试，验证阶段顺序稳定、终态事件唯一且终态后不再发 progress

## 3. Symbol Parsing 热路径

- [x] 3.1 重构 `repo/symbols/pipeline.rs`，为同一 `ParseUnit` 复用 syntax tree、compiled query 和 capture metadata，不再为 definitions/raw captures 重复 parse
- [x] 3.2 在保留 `CHUNK_BYTE_BUDGET` 的前提下，为 chunk 内文件解析引入有限并行执行和确定性合并
- [x] 3.3 补充 symbol parsing 测试，覆盖 definitions/raw captures 语义不变、并行度变化不影响 symbol ID/排序、单文件失败隔离
- [x] 3.4 为 chunk worker 引入按语言复用的 `Parser / Query` 缓存，避免大仓库在每个文件上重复构造 tree-sitter 工件

## 3.5 页面渲染热路径

- [x] 3.5.1 改造 `init / rebuild` 页面生成链，先并行预构建 page context / input hash / rendered bundle，再串行写盘与缓存
- [x] 3.5.2 补充 workflow 级或集成测试，验证页面预渲染并行不改变输出顺序、page hash 和 managed section 行为

## 4. 增量读取与 Update 热路径

- [x] 4.1 在 `sqlite_store` 中新增按文件路径集合读取 `symbols` / `edges` 及必要 graph frontier 的接口
- [x] 4.2 改造 `workflows/update.rs`，让小范围 graph 变化优先走局部 symbol/edge 工作集合并，并在超阈值时显式回退
- [x] 4.3 补充 update / storage 测试，验证小范围变更不会固定触发全量 `symbols / edges` 枚举，且回退路径可观测

## 5. 文档、注释与验证

- [x] 5.1 实现收口后复核 `DESIGN-ITER.md` / `DESIGN.md` 与最终落地范围保持一致，避免 8.5 与后续迭代的边界再次漂移
- [x] 5.2 按 `COMMENTING.md` 逐项检查本轮新增或重构代码的注释，确保 transport、workflow 和 parser 热路径代码的注释粒度与风格符合仓库规范
- [x] 5.3 运行 `cargo test -p wiki-core`、`pnpm test`，并补跑根级流式协议相关测试
- [x] 5.4 运行 `node scripts/run-test-projects.mjs` 对 19 个测试项目执行 `init` 分析，确认进度流与解析热路径优化没有引入回归
- [x] 5.5 运行 `node scripts/test-wiki-lifecycle.mjs` 完成全生命周期验证，重点检查 `init / update / rebuild` 的进度事件与最终状态一致性
- [x] 5.6 针对 `storybook` 复测 `init` 阶段耗时，确认 parser cache 与页面预渲染并行带来的真实收益
