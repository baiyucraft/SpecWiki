# Cold/Warm 与 Session Trace 说明

## 1. 口径

9.2 把 `init / rebuild` 的 runtime 清理与 LLM cache 生命周期拆开后，项目集与 reference 报告需要明确区分两类运行：

- `cold run`
  - `llm.cache_mode = clear`
  - 代表首轮真实 provider 成本和最坏耗时
- `warm run`
  - `llm.cache_mode = preserve`
  - 代表反复调试、二次对比和增量验证时的真实体验

`refresh` 不属于上面两类对照口径：

- `refresh`
  - 保留既有 cache 数据，但本轮强制跳过读缓存
  - 更适合单独做“重新请求 provider 但不丢历史缓存”的诊断

## 2. 当前脚本口径

- [run-test-projects.mjs](/E:/project/!byAI/spec-wiki/scripts/run-test-projects.mjs)
  - 已支持按 run mode 输出 `cold / warm`
  - 会实时打印 workflow progress 和 `llm_usage`
  - 汇总里带 `page_research` 请求数
- [collect-reference-project-reports.mjs](/E:/project/!byAI/spec-wiki/scripts/collect-reference-project-reports.mjs)
  - 已迁到 9.2 change 目录
  - 会在 init 过程中实时打印 usage
  - 单项目报告和 summary 会带 `page_research / page_enrichment / total_tokens`

## 3. Session Trace 应怎么看

9.2 的 research session 相关 trace 重点看这些事件：

- `llm_provider_request`
  - 看 provider body 是否真的带了顶层 `tools / tool_choice / response_format`
- `llm_provider_chat_completion`
  - 看 provider 是直接回 final，还是先回 `tool_calls`
- `llm_research_session_turn`
  - 看 session 最近一轮前的压缩状态、已有 `tool_artifact_refs`
- `llm_research_session_final`
  - 看最终回填的 `PageResearchResult`、recent turns 和 tool artifact refs
- `llm_budget_trim`
  - 看本轮请求是否触发了 phase budget 裁剪，以及裁剪前后的估算 token
- `llm_budget_rejected`
  - 看当前 workflow 是否已经把 `uncertainty` 或 `enrichment/research` 预算打满

## 4. 普通模式下的 usage

9.2 不再要求必须进 debug trace 才能看成本。普通 progress stream 已实时输出：

- `request_count`
- `input_tokens`
- `output_tokens`
- `total_tokens`
- `by_prompt_type`
- `by_provider_model`

因此：

- 调试 provider/session 协议时看 trace
- 跑项目集、看成本热点时直接看普通 progress 和脚本汇总

## 5. 这轮实现边界

当前 9.2 已经落下的边界：

- `file_purpose` 已支持批量 + provider 有限并行
- `top_level_promotion`、`dependency_edge` 已进入同类型批量 contract
- `module/topic` 页的 provider research session 已支持 `native_tools -> emulated_tools -> no_tools`
- `response_format` 默认顶层发送，prompt 内继续保留 schema
- learned capability 会写入 `~/.spec-wiki/state.yaml`

当前仍未在本轮收满的点：

- lifecycle 脚本还没有完整迁到与项目集脚本同等级的实时 usage/冷暖口径
- 19 项目 provider 全量跑数与 reference 新报告还需要单独执行一次
