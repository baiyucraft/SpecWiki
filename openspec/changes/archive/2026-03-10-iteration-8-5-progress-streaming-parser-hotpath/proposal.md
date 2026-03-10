## Why

迭代 8 已经把 symbol graph 主链接进 `init / update / rebuild / query`，但当前用户体感仍然明显落后于能力本身。真实代码里，`crates/wiki-core/src/transport/dto.rs` 与 `crates/wiki-core/src/transport/json_rpc.rs` 仍是“一次请求，只返回一次最终 JSON”的最小协议，`agents/codebuddy/src/runtime/invokeCore.ts` 也会把 `stdout` 全量缓存到子进程退出后才解析，因此 `init / update / rebuild` 在大仓库上天然表现为长时间黑盒等待。与此同时，`crates/wiki-core/src/repo/symbols/pipeline.rs` 对同一 parse unit 仍会分别为 definitions 与 raw captures 重复构建 syntax tree 和 query，`crates/wiki-core/src/workflows/update.rs` 在增量路径里也仍会把已持久化的 `symbols / edges` 整体读回再做全图摘要重算，导致“解析更快”的收益没有被用户明显感知到。

从上游真实实现看，这两类问题都应该在事实层与宿主边界之间尽早收口，而不是拖到 LLM 或消费层之后。GitNexus 的 `pipeline.ts` / `parsing-processor.ts` 用分阶段 ingestion 和 worker 复用把解析热点控制在 parsing 层；deepwiki-open 的 `websocket_wiki.py` 虽然不适合直接照搬，但它证明了长流程任务必须有持续可见的进度输出；当前 spec-wiki 自己的 `init.rs` / `update.rs` / `rebuild.rs` 也已经天然具备可枚举的阶段边界。基于这些真实代码，迭代 8.5 最合理的目标不是继续扩功能，而是先补齐“运行可感知”和“热路径真正变快”。

## What Changes

- 为 `wiki-core --json` 新增阶段化 progress 输出能力：`init / update / rebuild` 在真实 workflow 阶段之间输出结构化进度事件，而不是只在结束时返回最终结果。
- 为 CodeBuddy Agent 的 IPC 调用链新增 progress 消费与透传能力，但继续保持 thin Agent 边界；Agent 只负责流式读取、协议校验和宿主可消费的返回，不在 TS 层重做 workflow 逻辑。
- 升级 `status` / workflow 报告模型，使进度事件至少能表达 `action`、`phase`、`processed`、`total`、`message`、`elapsed_ms` 等用户可感知字段。
- 优化 symbol parsing 热路径：同一 parse unit 的 definitions 与 raw captures 必须复用同一份 tree-sitter parse/query 结果，而不是重复 parse/compile query；在保持 deterministic 输出的前提下，为后续并行化保留清晰的单文件工作单元边界。
- 优化增量 update 热路径：对局部源码变化，系统应优先按文件或受影响集合读取和合并 symbol/edge snapshot，避免为了少量 dirty file 无条件回读全量 `symbols / edges`。
- 扩展验证链路：新增 progress 协议、Agent 流式 IPC、parser 复用不改变 symbol 语义、增量 update 热路径不回退成全量读取的测试。
- 明确非目标：graph 呈现增强、页面信息密度提升、TOON/RAG/混合搜索仍归后续迭代，不并入 8.5。

## Capabilities

### New Capabilities
- `workflow-progress-streaming`: 定义 `wiki-core` 长流程 workflow 的结构化进度事件、最终结果事件以及 Agent 侧的流式消费契约。

### Modified Capabilities
- `codebuddy-agent-integration`: Agent 从“等待最终 JSON”升级为“流式读取 progress/result 事件并透传给宿主”，但仍必须保持 thin Agent 边界。
- `repo-wiki-workflow`: `init / update / rebuild` 除了完成既有产物生成外，还必须按稳定阶段输出进度，并在增量路径中避免不必要的全量 symbol/edge 回读。
- `symbol-parsing`: `parse_symbols` 的执行模型需要增加 parse/query 复用和稳定工作单元约束，确保热路径优化不改变 symbol 输出语义。
- `sqlite-cache-storage`: SQLite 需要支持增量 workflow 所需的文件级 symbol/edge 读取与合并，而不是只提供全量枚举接口。
- `workflow-verification`: 验证必须覆盖 progress 事件流、Agent IPC 流式消费、parser 热路径复用一致性和增量 update 的局部读取行为。

## Impact

- 主要影响 `crates/wiki-core/src/transport/{dto,json_rpc,cli}.rs`、`crates/wiki-core/src/main.rs`、`crates/wiki-core/src/workflows/{init,update,rebuild,status}.rs`、`crates/wiki-core/src/repo/symbols/pipeline.rs`、`crates/wiki-core/src/storage/sqlite_store.rs`，以及 `agents/codebuddy/src/runtime/{invokeCore,parseResult}.ts`。
- 长流程 JSON IPC 会从“单个最终对象”升级为“progress/result 事件流”，需要同步调整 Agent、脚本和测试的消费方式，但不改变六个工具的业务外壳契约。
- 需要新增 workflow/transport/performance 相关测试，并更新根级生命周期脚本，使其既能验证最终结果，也能验证长流程期间的事件输出与阶段计数。
