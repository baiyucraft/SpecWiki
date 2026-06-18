## Context

当前仓库的长流程体感问题来自两条真实链路，而不是单点实现瑕疵。

- 协议链路是单次收口的：`crates/wiki-core/src/main.rs` 在 `--json` 模式下先一次性读完 stdin，再由 `transport/json_rpc.rs -> transport/cli.rs` 同步执行 workflow，最后只 `println!` 一个 `CoreResponse`。对应的 `agents/codebuddy/src/runtime/invokeCore.ts` 也会把 `stdout` 全量拼接，直到子进程退出后才调用 `parseResult()`。这意味着 `init / update / rebuild` 即使内部有明确阶段，也无法被用户感知。
- parser 热路径仍有重复工作：`crates/wiki-core/src/repo/symbols/pipeline.rs` 里的 `parse_unit_symbols()` 与 `parse_unit_raw_captures()` 会对同一 `ParseUnit` 再次 `parse_tree()` 和 `Query::new()`。`CHUNK_BYTE_BUDGET` 当前只约束顺序处理的分块边界，并没有换来更高吞吐。
- 增量 update 的 graph 热路径仍偏重：`crates/wiki-core/src/workflows/update.rs` 虽然只重解析 `graph_refresh_sources`，但之后仍会通过 `sqlite_store::list_symbols()` / `list_edges()` 把全量持久化 rows 读回，再拼出全量 snapshot 做 graph summary。

上游源码给了两个直接启发：

- GitNexus 的 `src/core/ingestion/pipeline.ts` 把进度回调视为 pipeline 的一等输入，并把扫描、结构分析、分块解析、关系解析拆成稳定阶段；其 `parsing-processor.ts` 还把 parse worker 的输出设计为可复用的 extracted records，而不是为每个后续步骤重复 parse。
- deepwiki-open 的 `api/websocket_wiki.py` 说明长耗时任务必须以流式事件持续输出，哪怕最终消费形态和本仓库不同；这类能力应放在 transport / orchestration 边界，而不是塞进 query 或页面层。

同时必须遵守当前仓库边界：

- core 仍然是 deterministic facts engine，不能为了“显示进度”把 workflow 逻辑移到 Agent。
- CodeBuddy Agent 仍是 thin Agent，最多增加流式收发与宿主桥接，不能在 TS 层重建工作流状态机。
- “graph 呈现更强、页面内容更密、TOON/RAG/混合搜索”继续归后续迭代，不混入 8.5。

## Goals / Non-Goals

**Goals:**

- 为 `init / update / rebuild` 提供结构化的 progress 事件流，让长流程不再只有最终结果。
- 把长流程 JSON IPC 收口到统一的 NDJSON 事件流，避免 core、Agent 和脚本长期维护两套协议。
- 消除 symbol parsing 中对同一 parse unit 的重复 `parse_tree / Query::new()`，并在保持 deterministic 输出的前提下为有限并行解析建立工作单元边界。
- 为 parser worker 引入按语言复用的 `Parser / Query` 缓存，避免大仓库在每个文件上重复构造同一套 tree-sitter 工件。
- 为增量 update 增加文件级 symbol/edge 读取能力，让小范围变更时优先走局部 snapshot 合并，而不是无条件全量回读。
- 为 `render_pages` 引入“预渲染并行、写盘串行”的两段式执行，优先压缩大仓库页面生成阶段的串行耗时。

**Non-Goals:**

- 不在 8.5 重写 graph analysis 为真正的组件级增量算法；community/process/cycle 仍可在必要时基于全量 snapshot 重算。
- 不在 8.5 扩展 query 返回面、页面模板或 workflow 页面形态；“更强效果”继续留给迭代 9/10 及之后的消费层增强。
- 不引入 MCP、WebSocket、独立 daemon 或额外持久化后端；SQLite 仍是唯一状态后端，`wiki-core --json` 仍是本地子进程协议入口。

**Deferred to Later Iterations:**

- scanner 的目录遍历/指纹并发，明确归到 `迭代 9` 的吞吐补强。
- `community / process / cycle` 的子阶段并行化，明确归到 `迭代 9` 的 graph 吞吐补强。
- 宿主侧正式 progress UI / 消费面抛光，明确归到 `迭代 10` 的 Agent 消费层迭代处理。

## Decisions

### 1. 长流程直接采用 NDJSON 事件流，而不是继续保留单响应协议

决策：

- 对 `init / update / rebuild` 这类长流程 action，`wiki-core --json` 直接按行输出 NDJSON 事件流：
  - `progress`：阶段事件
  - `result`：最终成功结果，负载仍是现有 `CoreResponse`
  - `error`：最终失败结果
- `status / query / sync` 仍可保持单次最终响应，因为它们不属于长流程感知问题的目标面。

原因：

- 用户已明确不要求为旧调用方保留兼容路径，8.5 的目标是先把长流程可观测性和宿主消费面打通。
- NDJSON 仍保持了“stdin 一条命令，stdout 一个文本流”的本地子进程边界，不需要把协议升级成 socket 或双向 RPC。

备选方案：

- 方案 A：把 progress 写到 stderr。
  - 否决原因：stderr 当前承载的是错误文本；混入结构化进度后，Agent 与脚本很难可靠区分日志、进度和真正失败。
- 方案 B：单独新增 `status --watch` 轮询。
  - 否决原因：会把实时进度变成轮询协议，既增加调用复杂度，也无法精确映射 workflow 内部阶段。

### 2. 在 transport 层引入 `ProgressSink`，workflow 只上报事实，不直接打印

决策：

- 新增 transport/workflow 共用的轻量进度接口，例如 `ProgressSink` 或 `WorkflowReporter`。
- `run_init / run_update / run_rebuild` 在真实阶段边界调用 `report_progress(...)`，但不直接接触 stdout。
- transport 把这些事件编码为 NDJSON；`status / query / sync` 之外不再单独维护长流程的旧单响应路径。

原因：

- `init.rs`、`update.rs`、`rebuild.rs` 已经有明确阶段边界，适合上报进度；但如果让 workflow 直接 `println!`，会把业务层与 transport 层耦合死。
- 进度事件未来也可以被测试、CLI 或其他 host 复用，不应该只为 CodeBuddy 特判。

备选方案：

- 方案 A：在 workflow 内直接写 stdout。
  - 否决原因：破坏分层，测试也难以拦截和断言。
- 方案 B：仅扩展 `StatusReport`。
  - 否决原因：`status` 是执行后的静态视图，不是 workflow 进行中的事件流。

### 3. 进度模型采用“稳定阶段 + 可选计数 + elapsed”，并按 action 维持一致命名

决策：

- 为长流程定义稳定 phase 名称，至少覆盖：
  - `init`: `scan`、`parse_symbols`、`resolve_symbol_graph`、`analyze_symbol_graph`、`build_module_tree`、`build_contexts`、`plan_pages`、`render_pages`、`write_state`、`write_metadata`
  - `update`: `plan_changes`、`parse_symbols`、`resolve_symbol_graph`、`analyze_symbol_graph`、`build_module_tree`、`build_contexts`、`plan_pages`、`render_pages`、`write_state`、`write_metadata`
  - `rebuild`: 同 `init`，但保留 `clear_runtime` 或等价阶段
- 每个事件携带 `action`、`phase`、`message`、`elapsed_ms`，以及尽可能提供的 `processed / total / unit`。
- 对无法精确计算总数的阶段允许 `total` 为空，但文件解析和页面渲染类阶段必须提供计数。

原因：

- 当前 `init.rs` / `update.rs` 的阶段天然存在，缺的是协议表达，不是算法推导。
- 稳定 phase 名称可以直接被 Agent、脚本和测试复用；否则“进度感”会变成不可断言的字符串日志。

备选方案：

- 方案 A：只输出百分比。
  - 否决原因：难以解释当前在做什么，也不利于定位卡在扫描、解析还是写盘。
- 方案 B：只输出 message。
  - 否决原因：宿主无法稳定聚合或渲染，也不利于自动化验证。

### 4. symbol parsing 改为“单 parse unit 一次 parse/query，多提取阶段复用”，并在 chunk 内支持有限并行

决策：

- 在 `repo/symbols/pipeline.rs` 中为每个 `ParseUnit` 生成一次中间工件，例如 `ParsedUnitArtifacts`：
  - `source_bytes`
  - `Tree`
  - 编译后的 `Query`
  - `capture_names`
- definitions、imports、calls、heritage 都从同一份工件提取，避免双重 `parse_tree()` 和 `Query::new()`。
- 对全量/大批量解析，按既有 `CHUNK_BYTE_BUDGET` 切块，但 chunk 内文件解析改为有限并行；每个文件独立返回 `ParsedFileSymbols`，chunk 结束后按文件路径排序合并，保证最终输出 deterministic。
- 每个 parse worker 维护按语言键控的 `Parser / Query` 缓存；同一 worker 内重复处理 JavaScript/TypeScript 等语言文件时，必须复用已编译 query 和已设置 language 的 parser，而不是为每个文件重新构造。

原因：

- 当前最明确、风险最低的性能损耗就是重复 parse/query；先消掉这块，收益确定且不改变语义。
- 只在 chunk 内并行，能继续沿用现有内存预算，不会像“整仓无限并行”那样把 tree-sitter 内存峰值放大到不可控。
- `storybook` 这类文件数远大于页面数的仓库实测显示，单纯提高 worker 数会让 `Parser / Query` 重建成本被放大；先做 worker 级缓存，收益比继续加线程更稳定。

备选方案：

- 方案 A：只缓存 `Query`，不复用 parse tree。
  - 否决原因：仍然会为 raw captures 再 parse 一次 tree，收益不够。
- 方案 B：整仓库级线程池解析，不保留 chunk 边界。
  - 否决原因：会冲掉当前 20MB 分块设计，放大峰值内存和调试复杂度。
- 方案 C：继续顺序执行，只做代码清理。
  - 否决原因：用户对“更快解析”的体感不会明显改善。

### 5. `render_pages` 采用“预渲染并行、写盘串行”，不在 8.5 并发化最终状态写入

决策：

- `init / rebuild` 的页面生成拆成两段：
  - 第一段并行执行 `build_page_context -> compute_page_input_hash -> render_page_bundle`
  - 第二段串行执行 `write_page -> write_page_context_cache -> write_page_generation_cache -> 组装 PageBuildResult`
- 并行阶段只产生内存中的 page artifacts，不直接写 `.wiki`、cache 或 SQLite。
- 页面 artifact 合并顺序继续按 planner 输出顺序收口，保证最终页面顺序、状态和 progress 计数 deterministic。

原因：

- `storybook` 这类大仓库实测中，`render_pages` 与 `parse_symbols` 耗时相当，已经成为第二个明确热点。
- 当前真正需要串行的是写盘和状态收口，不是每页上下文计算与 Markdown 拼装本身；把并行边界卡在 artifact 生成层，复杂度和收益更平衡。

备选方案：

- 方案 A：页面最终写盘也一起并行。
  - 否决原因：会把 Windows 文件句柄、managed section merge 顺序和 cache 一致性一起复杂化。
- 方案 B：继续把页面生成全部留到迭代 9。
  - 否决原因：当前 `init` 真实瓶颈已经明确落在 `render_pages`，继续延期会让 8.5 的性能目标不完整。

### 6. update 热路径采用“文件级读取优先 + 阈值回退”，不在 8.5 重写全图增量分析

决策：

- 为 `sqlite_store` 增加按文件路径读取 symbol/edge 的接口，以及按受影响路径读取一跳 graph frontier 的辅助查询。
- `update` 在 `affected_set.graph_refresh_sources` 很小且 frontier 可控时，优先用 scoped read + changed snapshot 合并构造工作集。
- 当受影响集合过大、关键索引缺失或 graph frontier 膨胀超过阈值时，允许显式回退到现有全量读取路径，但该回退必须被记录为阶段事件或诊断。

原因：

- 当前 `list_symbols()` / `list_edges()` 的全表回读在小变更下成本过高，但把 community/process/cycle 全部改成组件级增量分析又超出 8.5 合理范围。
- “局部优先 + 阈值回退”可以先把最常见的小改动场景做快，同时保留大仓库的安全回退路径。

备选方案：

- 方案 A：维持全量读取。
  - 否决原因：无法兑现 8.5 对热路径优化的目标。
- 方案 B：一次性把 graph summary、community、process 全部改成完全增量化。
  - 否决原因：范围已经接近 9 之前再开一个大型图状态重构，不适合 8.5。

### 7. CodeBuddy Agent 只增加流式消费能力，不改变六个工具的业务接口

决策：

- `agents/codebuddy/src/runtime/invokeCore.ts` 改为逐行消费 NDJSON，并在内部识别 `progress/result/error`。
- `invokeCore()` 仍返回最终 `CoreResponse`；新增的 progress 只通过可选回调或可注入 observer 暴露，避免把现有工具函数签名全部打散。
- `parseResult.ts` 保留“最小最终协议校验”的职责；流式事件校验应由新的事件解析器承担。

原因：

- 当前仓库没有稳定的宿主级 progress API 抽象；先让 Agent 具备消费/转发能力，比直接改六个工具的返回面更稳妥。
- 这样既能给 CodeBuddy 接入留口，也不会让 `wikiInit/wikiUpdate/...` 的业务契约在 8.5 被迫重写。

备选方案：

- 方案 A：让每个工具直接返回 `{ progress, result }` 复合对象。
  - 否决原因：会破坏现有工具测试和宿主集成习惯。
- 方案 B：Agent 忽略 progress，只在 core 侧输出。
  - 否决原因：无法真正改善用户感知。

## Risks / Trade-offs

- [调用方迁移成本] → 长流程统一切到 NDJSON；同步改造 Agent、脚本和测试，避免两套协议长期并存。
- [并行解析导致结果抖动] → 文件级结果在合并前统一按 `file_path/start_line/symbol_id` 排序；worker 数量固定且只影响吞吐，不影响最终顺序。
- [progress 事件过密导致输出噪声过大] → 只在阶段切换和有意义的文件/页面计数增量时发事件，不做逐行源码级日志。
- [局部读取优化收益不稳定] → 采用阈值回退策略；对大范围变更仍允许全量路径，但要显式暴露回退原因。
- [Agent 宿主侧暂时没有统一 progress UI] → 先把 core 与 Agent 的流式能力打通；宿主如果暂时没有进度面板，也可复用回调输出简洁文本提示。

## Migration Plan

1. 先扩展 transport 协议和事件模型，并把长流程收口到 NDJSON 事件流。
2. 在 `init / update / rebuild` 中接入 reporter，补齐阶段边界与计数字段。
3. 同步改造 Agent `invokeCore()` 的逐行解析，并让脚本/测试统一按事件流消费。
4. 重构 symbol parsing 热路径，先完成 parse/query 复用，再引入 chunk 内有限并行。
5. 为 SQLite 增加 scoped read API，并在 `update` 中引入“局部优先 + 阈值回退”。
6. 补齐 transport/workflow/parser/update 的测试与生命周期脚本验证。

回滚策略：

- 若 NDJSON 流式协议出现宿主接入问题，可先在宿主桥接层降级为“忽略 progress、只消费终态事件”，而不是回退到旧协议。
- 若 scoped read 或并行解析出现稳定性问题，可保留 parse/query 复用，同时回退到顺序解析或全量读取路径，不影响对外结果正确性。

## Open Questions

- CodeBuddy 宿主当前是否已有正式的 progress 展示接口，还是先通过内部 observer/日志桥接；这会影响 Agent 默认如何消费 progress 事件，但不影响 core 事件模型本身。
- `update` 的 scoped read 阈值如何设定最合理：按 dirty file 数、frontier edge 数，还是按估算 row 数；需要在 19 项目集上跑数据后定。
- chunk 内并行解析是否直接引入 `rayon`，还是先用标准库线程池/受控 worker；需要结合 Windows 打包与测试稳定性评估。
